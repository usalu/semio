# 📓️ P5 — the wgpu tree target (design §6.4) and the reconcile size law (design §7)

Packet P5 of ticket 26/09/16/ARTIFACT-TREE-VIRTUALISED-STREAMING. Normative input:
📓️design-virtualised-tree.md §6.4 and §7, 📓️audit-paging-plumbing-and-schema.md §6,
📓️audit-surface-budgets.md §3, plus the extra scope handed over from 📓️p1-contract.md §5.1.

---

## 1. Headline numbers (measured, not inherited)

| Quantity | Value | How |
|---|---|---|
| `size_of::<FlatPresentedNode>()` | **6 520 B** | printed by the new law, `cargo test -p semio-framework-ui-runtime` |
| Per-node host cost, 4 × 128-row window | **40 262 – 49 269 B** | same law, per page (`FlatPresentedNode` + that node's semantic bytes) |
| Peak surface bytes, 4 sections × full window | **4 187 905 B of 8 388 608 B (49.9 %)** | same law |
| Whole 4 × 128 = 512-row document | **7 pages, 23 238 711 B total** | same law |
| `size_of::<ui_contract::Component>()` | **3 096 B** (was 2 568) | P1 §5.1, re-confirmed here |
| `size_of::<ui_contract::UiNodeRecord>()` | **6 416 B** | measured here |
| `size_of::<ui_contract::UiPatchOp>()` | **6 416 B** | measured here |

**The stale "~0.46 MiB per presented node" is dead.** The real fixed per-node host charge is
`size_of::<FlatPresentedNode>() = 6 520 B` — **74× smaller** than the number
`PANEL_RECONCILE_NODE_BUDGET = 16`'s docstring carried since 2026-09-10. 📓️audit-surface-budgets.md §3
guessed correctly that 0.46 MiB was the guest-side `UiValue` arena footprint echoed onto the host side.

### 1.1 What actually bounds a virtualised panel — and it is not bytes

Design §7 assumed the byte ceiling was the binding constraint. **It is not**, and neither is the record
arena. In descending order of tightness, measured:

1. **The ITEM ledger, `SurfaceReconcileLimits::max_items = 4 097`.** Reached at **85–104 nodes**, with
   only **3.3–4.3 MiB of the 8 MiB** surface budget spent. Refusals:
   `items 4100/4097, nodes 104/128, bytes 3 335 960/8 388 608`.
2. **The record arena, `ui_contract::UI_DOCUMENT_NODES = 128`.** One surface document is 128 records
   wide *in total* (`UiSnapshotNodes = UiFixedList<UiNodeRecord, 128>`), root and section records
   included — see §1.2.
3. **The byte ceiling, `SURFACE_RECONCILE_SURFACE_BYTES = 8 MiB`.** Never reached; peak 49.9 %.

The practical consequence for wave 2 is exactly the one design §5 already legislates: **pick rows must
go through `.interaction_domain()`**, because a per-row `Activate` binding with a 4-entry argument map
costs ≈ 30 arena items, and 124 such rows alone exhaust the ledger. In the measured run the per-page
item count falls 3 920 → 1 520 as the window scrolls past the bound rows, and the admitted window grows
accordingly.

### 1.2 `UI_BUILT_CHILDREN_MAX = 128` and `UI_DOCUMENT_NODES = 128` collide

Design §3 sets `UI_BUILT_CHILDREN_MAX = crate::UI_DOCUMENT_NODES` so that "one built node fans out
exactly as wide as one host document child list". That is true of the *child list*, but a full 128-row
section can never be presented, because the section record and the tree root are themselves records in
the same 128-record document:

```
1 (Tree) + 4 (TreeSection) + 4 × rows  ≤  UI_DOCUMENT_NODES (128)   ⟹   rows ≤ 30 per section
```

and with per-row bindings the item ledger cuts that again to **20 rows per section**. So a "128-row
window" is a *logical* `TreeWindow.total`, never a single presented page. The law therefore streams the
512-row document as 7 pages and measures every one of them, which is what scrolling actually does.

**Recommendation for the coordinator:** design §5's `slice.len = min(req.rows, UI_BUILT_CHILDREN_MAX,
total − offset)` is not a sufficient clamp. `TREE_WINDOW_DEFAULT_ROWS = 48` is also above what one
document admits for a multi-section body. The SDK's first-paint budget should be derived from
`UI_DOCUMENT_NODES` minus the container records the body will emit, not from `UI_BUILT_CHILDREN_MAX`.

---

## 2. Files touched

### 2.1 Legacy wgpu tree nodes — new fields (task 1)

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs` (`pub mod ui`):

- **New type `UiTreeWindow { total: u32, offset: u32 }`** with `Clone, Copy, Debug, Default,
  PartialEq, Eq, Serialize, Deserialize, ToValue, FromValue` and `#[serde/value(rename_all =
  "camelCase")]`, plus `leading_rows()` / `trailing_rows(materialised)` helpers.
  It is a **separate type from `ui_contract::TreeWindow` on purpose**: the legacy `UiNode` wire shape
  derives `ToValue`/`FromValue` rooted at `dsl` (`semio_framework_os_kernel`), while the contract
  derives them rooted at `::protocol::value` (`semio-framework-replication`). Those are two different
  traits; `ui_contract::TreeWindow` does not implement `dsl::ToValue`. The whole legacy layer is
  already a parallel type system (`ActionDescriptor`, `UiPresence`, `Label` … are all re-declared), so
  a mirror type is the consistent choice, not an exception.
- `UiTreeItemNode` gains `window: Option<UiTreeWindow>` and `granularity: Option<String>`.
- `UiTreeSectionNode` gains `window: Option<UiTreeWindow>`.
- `UiTreeItemNode::base()` initialises both to `None`.
- 41 struct-literal construction sites across 11 files updated (production + tests).

### 2.2 Reconcile bridge (task 1)

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🔀️reconcile/🦀️.rs`:

- New `fn tree_window(Option<&ui_contract::TreeWindow>) -> Option<UiTreeWindow>` projecting contract →
  legacy.
- `tree_item()` now reads `props.window` and `props.granularity`.
- The `Component::Tree(props)` arm reads `section.window` off each `Component::TreeSection` child.
- The second arm (`Component::TreeSection(_) | Component::TreeItem(_) => UiNode::Stack`) deliberately
  does **not** read `window`: that arm mints the keyed identity/hit-test row, which has no extent of
  its own; the spacer pitch belongs to the painted spec. Documented at the call site.

### 2.3 Spacer pitch in measure and render (task 2)

`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🎯️targets/🧊️wgpu/🦀️.rs`:

- New `fn tree_window_pitch(window, materialised) -> (leading, trailing)` returning
  `offset × TREE_ROW_HEIGHT` and `(total − offset − materialised) × TREE_ROW_HEIGHT`. An unwindowed
  container returns `(0.0, 0.0)`, so every non-virtualised tree keeps its exact current extent.
- New `fn tree_item_expandable(item)` — `!children.is_empty() || window.total > 0`. A row with
  `window.total > 0` shows the fold chevron **with zero materialised children**.
- Applied in `measure_tree_sections_state` (L84), `measure_tree_item_height` (L100), `render_tree`
  (L115) and `render_tree_item` (L152) — measure and render use the *same* helper, so extents agree.
- **No `+N` row anywhere.** The spacer band *is* the representation of the unloaded rows.

`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🪀️widgets/🦀️.rs`: `TreeItem<E>` and `TreeSection<E>`
gain `window: Option<UiTreeWindow>`; `TREE_ROW_HEIGHT` (L192) is unchanged and is the single pitch both
the spacer maths and the rows use.

### 2.4 Documented gaps (task 2, explicitly out of scope)

Two docstrings state the gap, on `UiTreeWindow` and on `tree_window_pitch`:

1. **No host-side window requests for wgpu.** React's `🗣️Interpreter` mounts a viewport observer that
   reports `TreeWindowRequest`s through `ViewModel.tree_windows`; the wgpu shell has no equivalent, so
   a wgpu tree paints only the first-paint window its guest chose, and scrolling into a spacer band
   reveals empty pitch rather than streamed rows. Closing it needs a wgpu scroll/open observer feeding
   the same `ViewModel` field.
2. **The retained mounted-layout path is not windowed.** `🎯️targets/🧊️wgpu/🧮️layout/🦀️.rs`
   (`tree_item_height`/`tree_section_height`), `📌️mounted_layout` and `🖌️paint` walk
   `UiTreeItemNode`/`UiTreeSectionNode` directly and would each need a spacer *node* in the layout
   tree, not just a height. That is a larger change than this packet's brief and is not attempted;
   adding height there without adding the node would mis-position every row.

### 2.5 TypeScript mirror

`🧰️framework/🔨️modules/🧬️schema/📽️projection/🦀️.rs`: new `SchemaMetadata` entry `UiTreeWindow`
(TYPES 195 → 196, count assertion in `🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🔬️app-label/🦀️.rs:1477`
bumped), plus `window?: UiTreeWindow` on `UiTreeSectionNode` and `window?`/`granularity?` on
`UiTreeItemNode`. Regenerated `🧰️framework/🔨️modules/🛂️manifest/🤖️generated/🪪️manifest/🟦️.ts`; the
byte-identity test passes without `SEMIO_TYPEGEN_OUT`.

### 2.6 Fixture the node growth moved

`🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json` — guard `ui::wgpu_engine`,
owner `wgpu::engine::UiSurfaceRegistry`, `elementSizeBytes` **159 896 → 159 936** (+40 B, the two new
`Option` fields on the two legacy tree nodes inside `Option<UiSurfaceSlot>`). This is the only wgpu
failure this packet caused, and it is fixed.

### 2.7 The new reconcile size law (task 3)

`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/🧪️tests/🔬️reconcile-unit/🦀️.rs`, region
`🪟️TreeWindowReconcileSizeLaw`, next to `resident_aggregate_admits_every_reconcile_slot`:

`four_section_full_window_tree_reconciles_under_the_surface_byte_ceiling` builds a `Tree` with 4
`TreeSection`s, each stamping `TreeWindow { total: UI_BUILT_CHILDREN_MAX, offset }`, whose rows carry
label + icon + `granularity`, with an `Activate` binding over a 4-entry `UiMap` on the first 31 rows of
each section and plain rows after that. It walks the admitted window down until admission accepts a
page (the `ui.fixed-capacity` rule of design §5 — a refused row shortens the window, it never faults),
streams all 512 rows as successive pages on one `SurfaceReconciler`, prints
`[DEBUG] flat-presented-node bytes=… rows=… surface_bytes=…` for every page and every refusal, and
asserts:

- every page fits `SurfaceReconcileLimits::max_nodes`;
- every page reconciles under `SURFACE_RECONCILE_SURFACE_BYTES`;
- **no page is ever refused for BYTES** — every refusal is an item/node refusal;
- every declared row is materialised by exactly one page;
- per-node cost leaves a full `UI_DOCUMENT_NODES` arena affordable.

It also fixes `🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📏️ownership/🧪️tests/📏️ownership/🦀️.rs:27`,
whose exhaustive `TreeItemProps` literal did not compile after P1's two new fields.

Measured output:

```
[DEBUG] flat-presented-node bytes=6520 refused rows_per_section=30 nodes=104/128 items=4100/4097 surface_bytes=3335960/8388608
…
[DEBUG] flat-presented-node bytes=6520 rows=80 nodes=85 offset=0   rows_per_section=20 items=3920 surface_bytes=4187905 per_node_bytes=49269 ceiling=8388608
[DEBUG] flat-presented-node bytes=6520 rows=80 nodes=85 offset=20  rows_per_section=20 items=2840 surface_bytes=3843473 per_node_bytes=45217 ceiling=8388608
[DEBUG] flat-presented-node bytes=6520 rows=80 nodes=85 offset=40  rows_per_section=20 items=1520 surface_bytes=3422305 per_node_bytes=40262 ceiling=8388608
…
[DEBUG] flat-presented-node bytes=6520 rows=512 pages=7 declared_rows=512 peak_surface_bytes=4187905 peak_nodes=85 peak_items=3920 total_surface_bytes=23238711 ceiling=8388608 document_nodes=128 max_items=4097
```

---

## 3. The two page-exact laws handed over from P1 §5.1

### 3.1 `surface_canonical_document_completion_transfers_do_not_borrow_the_child_grant` — FIXED

`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📃️document/🧪️tests/📃️document/🦀️.rs`.

**Before:** `if closing && bytes == 4096 { full_close_grant = true }` — a frozen literal.

**Why it broke.** The close turn's grant is not the caller's grant.
`♻️retirement/🌳️typed/📃️document/🦀️.rs:61`, `UiDocumentArena::retire_exact`:

```rust
let grant = maximum_bytes.max(slot.nodes.entries.allocated_bytes());
```

so `close_document_owner` widens its own grant to the retired document's **node-page allocation**,
which scales with `size_of::<ui_contract::UiNodeRecord>()`. With `Component` at 2 568 B that page fit
inside the 4 096 B `SURFACE_COMPONENT_COPY_WORK_BYTES` and the turn released exactly 4 096. With
`Component` at 3 096 B, `UiNodeRecord` is 6 416 B, one page is 7 568 B, and the measured turns became
`7568, 7568, 7568, 7568, 2496` — 32 768 B of surface payload. The frozen equality can never hold again,
for any `Component` wider than it was on 2026-09-15.

**After** — the law is a range over the two constants the implementation itself uses, so it holds for
any `Component` size:

```rust
if closing && bytes >= SURFACE_COMPONENT_COPY_WORK_BYTES {
    assert!(bytes <= document_close_turn_ceiling(), …);   // SURFACE_RECONCILE_PAGE_BYTES
    full_close_grant = true;
    assert!(matches!(&diff.owned_copy, Some(RecordOwnedCopy::Comparison(_))));
}
```

The meaning is preserved and arguably sharpened: a close turn is **never a partial slice borrowed from
the child's budget** — it is at least one whole work page, and at most the one page a `PageBytes` fault
admits. The other three frozen `4096`s in the same test (`comparison_completed`, the copy bound, the
`size_of::<ExistingComponentComparison>()` bound) are now written as
`SURFACE_COMPONENT_COPY_WORK_BYTES`, and the `[DEBUG]` banner prints the live
`size_of::<ui_contract::UiNodeRecord>()` and the derived range instead of two literals.

**Result: passes.** 122 → 123 of the runtime crate's tests green.

### 3.2 `runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads` — NOT a grant law; handed back

`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️retirement/🌲️tree/🧪️tests/🌲️tree/🦀️.rs:38`.

The brief for this one was to re-derive the grant arithmetic from `size_of::<Component>()`/`PAGE_BYTES`.
**I instrumented it before changing anything, and that premise does not survive the measurement.** All
probes on the FIRST fixture case (`container`, which carries no tree props at all):

| probe | grant | completes? | released bytes |
|---|---|---|---|
| `populated` | 1, 2, 3, 4, 8, 16, 32, 64 | **no** — stalls identically | **15** every time |
| `populated` | 4 096 | yes | **3 260** |
| `bare` (`TreeNode::try_new("K", component)`, no bindings/menu/children) | 4 096 | yes | **7** |
| `populated` minus children | 4 096 | yes | **3 245** |

Two independent things are wrong, and neither is expressible as `size_of`-derived page arithmetic:

1. **A budget-independent stall.** Grants 1 → 64 all stop at exactly 15 released bytes and never
   progress again, no matter how long the loop runs. Draining the global queues on every
   non-progressing turn (`ui_contract::close_ui_value_page_one()` +
   `close_built_node_page_one()`, ≈ 399 916 successful drains over 200 000 turns) does **not** unblock
   it, so it is not the global value-page or built-node queue. The minimum viable grant now lies
   somewhere in `(64, 4096]`, where it used to be `1` — the fixture's `"grants": [1, 64, 4096]` pins
   exactly the two values that no longer work.
2. **A byte over-charge, which is the part that says "bug", not "law".** The committed expectation is
   `row["bytes"] (6) + extraPayloadBytes (30) = 36`. The measured total is **3 260**. The decomposition
   is decisive: the bare node still charges **exactly 7** = key `"K"` (1 B) + the container's own
   payload `"A".."F"` (6 B), and the two children still charge **exactly 15** = `"C"+"child"` (6) +
   `"R"+"rejected"` (9). Every one of those is byte-exact and unchanged. The entire 3 224 B excess sits
   in the node's one `ActionBinding` (`args {"x": ["🌊"]}`, capability `"c"`) plus its `MenuRef`
   (`id "m"`, same args) — ~15 B of real payload being charged ≈ 3 238 B, i.e. roughly
   `size_of::<ui_contract::Component>() = 3 096` on top. A retirement that charges a struct's *inline
   size* where it charges its *payload* everywhere else is not a law to re-derive; bumping the fixture
   to 3 260 would freeze the defect.

**Recommendation.** This belongs to `semio-framework-ui-contract`'s retirement
(`♻️retirement/🌲️built/🦀️.rs` `BuiltTreeRetirement::close_step` fields 7/8 — `bindings`, `menu` — and
`♻️retirement/🦀️.rs` `UiValueRetirement::close_step`), which is P1's file set, not P5's. It is very
likely the same defect as P1 §5.2's contract-crate SIGABRT
(`binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases`, then
`UiValueRetirement requires exact terminal closure` inside a `Drop`): both are `ActionBinding`-argument
retirements through the `UiValue` arena that stopped closing exactly when the contract types grew.
Fixing it there should restore `36` and the `grant = 1` minimum, and no fixture edit will be needed.
The sibling `runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission` flaps in
and out of the same failure and has the same cause.

**All instrumentation has been removed from both test files** (verified: `git diff` shows only the
intended Law-A change).

---

## 4. Commands run, with results

Long runs were detached with `nohup` and polled to completion inside the turn — the shared artifact-dir
lock was contended by 45–66 peer `cargo` processes for most of this packet, and single foreground calls
exceeded the 600 s tool cap. Logs under `🗑️generated/p5/`.

| Command | Result |
|---|---|
| `cargo check -p semio-framework-ui --features wgpu-engine` | **ok** |
| `cargo check -p semio-framework-ui --features wgpu-engine,testkit,tui --all-targets` | **ok — 0 errors** |
| `cargo check -p semio-framework-ui-runtime --all-targets` | **ok** |
| `cargo check -p semio-framework-artifact-playbook-playbook --all-targets` | **ok** |
| `cargo test -p semio-framework-ui-runtime --lib four_section_full_window_tree_reconciles_under_the_surface_byte_ceiling` | **ok — 1 passed** (numbers in §1) |
| `cargo test -p semio-framework-ui-runtime` | **122 passed / 2 failed** — both `tree_retirement`, §3.2. §3.1's law is green; it was red before this packet. |
| `cargo test -p semio-framework-ui --features wgpu-engine,testkit` | **415 passed / 6 failed**, before and after the §2.6 fixture fix. `ui_surface_slot_table_…` — the one failure this packet caused — is **green** after the fix and no longer appears; the residue failures shifted within the `prepared` family, which is itself the signature of cross-test residue rather than a code fault. See §4.1. |
| `cargo test -p semio-framework-ui --features wgpu-engine,testkit -- --test-threads=1` | 415 / 6 — same count, different set again |
| `cargo test -p semio-framework --features typegen exports_typescript_bindings` | **ok — 1 passed**, rendered TS byte-equal to the committed mirror |

### 4.1 The six wgpu failures, attributed

Re-run individually in a fresh process (`-- --exact <full::path>`):

| Test | Alone | Attribution |
|---|---|---|
| `wgpu::engine::tests::ui_surface_slot_table_is_heap_first_and_fits_a_bounded_thread_stack` | **ok** | **Mine.** `element_bytes 159896 → 159936`; fixed in §2.6. |
| `wgpu::prepared::tests::preparation_yields_at_the_configured_item_budget` | **ok** | Cross-test residue — a fixed process-permit pool exhausted by earlier tests in the same binary ("test prepared input must fit fixed process permits"). Not P5. |
| `wgpu::prepared::tests::receiver_survives_worker_ownership_of_the_job` | **ok** | same |
| `wgpu::prepared::tests::retained_codec_source_moves_once_and_retires_one_page_per_governed_step` | **ok** | same |
| `wgpu::reconcile::document_tree_reconcile_tests::a_surfaces_second_document_reaches_the_arena_and_a_constant_generation_never_does` | **ok** | same (`ArenaFull` from a process-global arena already spent) |
| `wgpu::engine::tests::large_layout_and_shaping_job_keeps_every_observed_slice_below_eight_ms` | **ok** (0.45 s) | Fleet-load timing flake — observed 17.8 ms / 14.2 ms under 45–66 concurrent `cargo` processes. Not P5. |
| `wgpu::engine::retained_document_hostile_fixtures::max_plus_one_…_last_valid_snapshot` | **FAILS alone** (`ArenaFull, SurfaceId("hostile.surface")`) | **Not P5.** That fixture builds `Component::Separator` records only and names no file this packet touched; it depends entirely on `ui_contract`'s static `UI_DOCUMENT_ARENA` and resident arithmetic, which P1 is changing. Hand to P1 alongside §3.2. |

---

## 5. Handover

- **P3 (SDK):** §1.2 — clamp the first-paint slice by `UI_DOCUMENT_NODES` minus the body's container
  records, not by `UI_BUILT_CHILDREN_MAX`; and note that the item ledger, not bytes, is what refuses a
  window. `TREE_WINDOW_DEFAULT_ROWS = 48` is above what one multi-section document admits.
- **Wave 2 apps:** design §8.2's `.interaction_domain()` migration is not cosmetic — it is what buys
  back the arena items that currently cap a bound-row page at 20 rows per section.
- **P1 / coordinator:** §3.2 and the last row of §4.1 are contract-crate retirement/arena defects, with
  the measurements needed to fix them. No fixture in this packet has been bumped to freeze them.
- **wgpu:** the two documented gaps in §2.4 — host-side window requests, and the retained
  mounted-layout path — are the follow-ups that would make wgpu trees actually stream.
