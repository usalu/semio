# 📓️ F2 — SDK body-wide node ledger, request priority, key uniqueness

Packet **F2** (SDK + framework Rust), 2026-09-17. Normative inputs: `📓️wave4-resume-brief.md`,
`📓️design-virtualised-tree.md` §3–§5/§7, `📓️p1-contract.md`, `📓️p3-sdk.md`, `📓️p5-wgpu-and-reconcile-law.md`,
`📓️w3-browser-verification.md` §6, and the coordinator's post-S3 decisions (`📓️s3-review-streaming-loop.md`
findings 1, 3, 4).

> ✅️ Complete. §6 is the run table — anything not listed there was not run. §5 answers
> `📓️f1-host-scroll-streaming.md` §10 point by point.

## 1. What the packet had to close

`📓️w3-browser-verification.md` §6.2 caught the fem3d **House** artifact panel surface faulting at
`1:framework.panel.artifact — nodes: 129 vs max_nodes: 128`. Per-container windowing bounds each
*container* at `UI_BUILT_CHILDREN_MAX`; nothing bounded the **body**. A previous agent had started a body
ledger inside `TreeWindows` and died before verifying it; S3 then found that ledger and the host's request
cap were two independently-authored numbers that do not compose (finding 1), that request order is the
host's alphabetical key order and not document order (finding 3), and that `node_key` collisions are
unenforced (finding 4).

## 2. Changes

### 2.1 One budget, defined once — `semio_framework_ui_contract`

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` (next to `TreeWindow`):

```rust
pub const TREE_WINDOW_FIXED_NODE_HEADROOM: usize = 16;
pub const TREE_WINDOW_BODY_NODE_BUDGET: usize = crate::UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM; // 111
```

The headroom **moved** out of the plugin SDK into the contract, so the derivation lives in one place and
both sides of the wire read the same number. The SDK now re-exports them
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, region `🔖️PanelWindowing`, plus the crate-root
`pub use app::{…, TREE_WINDOW_BODY_NODE_BUDGET, TREE_WINDOW_DEFAULT_ROWS, TREE_WINDOW_FIXED_NODE_HEADROOM}`),
so `semio_framework_plugin::TREE_WINDOW_FIXED_NODE_HEADROOM` still resolves for every existing caller.

The headroom's value is justified on the constant itself: the framework's own `ui_history_panel` is the
widest un-windowed block in the fleet (an Actions section wrapping undo, redo and three filter rows, plus
its own section node ≈ 7 records) and the fem/cad/puzzle outliners add none; 16 doubles that.

### 2.2 Cost model — `1 + rows` per container, every node charged exactly once

`TreeWindows` (same region) is now:

```rust
pub struct TreeWindows<'a> {
    requests: Vec<(&'a TreeWindowRequest, Cell<usize>)>, // request + its outstanding reservation
    budget: Cell<u32>,                                   // shared first-paint viewport allowance
    nodes: Cell<isize>,                                  // the body ledger, may go negative
    reserved: Cell<usize>,                               // records held for requests not yet reached
    claimed: RefCell<Vec<String>>,                       // node keys this body has published
    nested: Cell<bool>,                                  // "we are inside a parent's row closure"
}
```

- `debit_container()` charges **1** for a container's own node — *unless* `nested` is raised, i.e. the
  container is being built as a row of an enclosing window, in which case its parent's row grant already
  paid for it. `tree_window_rows` raises and restores `nested` around each `row(entry)` call. This fixes the
  "a nested `tree_window_item` is charged twice" under-materialisation S3 quoted.
- `grant(want)` takes rows off the ledger; `debit(n)` is unconditional (a container node is not optional
  once the panel decided to build it), so an overdrawn ledger simply grants nothing afterwards and
  `nodes_remaining()` saturates at 0.
- Law `a_nested_container_is_charged_exactly_once` pins the equality
  `TREE_WINDOW_BODY_NODE_BUDGET - nodes_remaining() == body_nodes(built)`.

### 2.3 Request priority — order independence (S3 finding 3)

The host's request order is its node keys sorted **alphabetically** (wire byte-stability), not document
order, so document-order charging could starve a visible container behind an off-screen one that merely
sorts earlier. The ledger is therefore split at construction, before any container exists:

- `TreeWindows::seated` reserves `1 + min(rows, UI_BUILT_CHILDREN_MAX)` per request, **in request order**,
  until the budget is exhausted. A host that over-asks is clamped there — deterministically, tail last.
  `open: Some(false)` reserves nothing.
- An **unrequested** container (first paint) is served by `grant_unreserved` — out of
  `nodes_remaining() - reserved` only — and additionally clamped by the shared viewport `budget`.
- A **requested** container releases its own reservation when the render reaches it and is then served out
  of the whole remaining ledger, wherever it sits in the body.
- `rows: 0` with `open: true` (the host's "measured, open, off screen" spelling) is a valid request: the
  container node is built, `total`/`offset` are stamped, zero rows are materialised, and it never falls
  back to the first-paint default.
- A stale `offset` clamps to the **last** window (`total - rows`), not to `total - 1`, so a non-empty open
  container with `rows > 0` never comes back empty.

### 2.4 `node_key` uniqueness (S3 finding 4)

`TreeWindows::claim_window(node_key, entries)` refuses the second container in one body to claim a key,
with a distinct code `ui.tree-window.duplicate-key`.

⚠️ **Refinement, deliberate**: only a container that actually PUBLISHES a window claims. A container with
no entries that the host has never addressed stamps nothing and is not addressable, so two of them cannot
steer each other — which keeps trees whose *leaves* are built through `tree_window_item`
(`TreeWindowKit::render`, the stdio JSON/XML editors, the writer AST, procedural's structure window) out of
the uniqueness requirement entirely. It binds containers, not rows. §5 lists what still trips.

## 3. Files changed

| File | What |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` | `TREE_WINDOW_FIXED_NODE_HEADROOM`, `TREE_WINDOW_BODY_NODE_BUDGET` + the cost model on the docstring |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | region `🔖️PanelWindowing`: reservation ledger, exact-once charging, claim, offset clamp, re-exports |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-panel-kit/🦀️.rs` | the laws in §4 |
| `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | `house()` fixture + two House laws; `wide_view()` rows 128 → 8 (see §5.3) |

## 4. Laws

In `🔬️app-panel-kit` (`panel_kit_tests`), new this packet:

| Law | Pins |
| --- | --- |
| `the_body_node_budget_is_the_record_arena_less_the_root_and_the_fixed_reserve` | the derivation |
| `the_host_tree_element_declares_the_same_body_node_budget` | reads `🌳️Tree/🟦️.tsx` and asserts the literal `TREE_WINDOW_BODY_NODE_BUDGET = 111` |
| `a_nested_container_is_charged_exactly_once` | `spent == body_nodes(built)` |
| `an_open_request_for_zero_rows_materialises_nothing_and_keeps_its_offset` | the off-screen spelling |
| `a_stale_offset_clamps_to_the_last_window` | `offset + length == total`, and the shorter-than-window case |
| `a_duplicate_node_key_in_one_body_is_refused` | section↔section, section↔nested item, and the leaf exemption |
| `a_closed_container_costs_exactly_one_node` | closed cost |
| `a_host_request_outranks_a_first_paint_default_earlier_in_document_order` | the starvation case |
| `host_requests_beyond_the_ledger_are_clamped_in_request_order` | over-ask clamp, totals still stamped |
| `a_nested_tree_window_item_inside_a_window_section_stays_inside_the_arena` | fem3d's own shape |
| `a_row_error_that_is_not_fixed_capacity_still_propagates` | only `ui.fixed-capacity` shortens |

Pre-existing and kept green: `a_closed_container_materialises_no_rows_and_still_stamps_its_total`,
`a_host_request_materialises_exactly_its_slice_and_opens_a_closed_author_default`,
`a_first_paint_spends_one_shared_viewport_budget_in_document_order`,
`a_window_never_exceeds_one_built_children_page`, `a_refused_row_shortens_the_window_instead_of_faulting`,
`nine_windowed_sections_share_one_body_wide_node_ledger`,
`a_nested_group_item_charges_the_body_wide_node_ledger` (nested keys made unique),
`a_first_paint_never_outspends_the_node_ledger`, `ui_history_panel_stays_inside_the_body_node_ledger`,
`ui_history_panel_windows_its_commands_without_a_continuation_row`,
`interaction_domain_stamps_one_tree_level_activate_binding`.

fem3d (`📌️panels/🗿️artifact/🧪️tests/🔬️unit`):
`a_house_sized_first_paint_keeps_the_whole_body_inside_the_record_arena` and
`a_house_sized_body_honours_every_capped_host_window` — 63 nodes, 63 supports, 8 solids, four load cases of
twelve loads each, all open; the second drives it with a request set whose `Σ(1 + rows)` is inside
`TREE_WINDOW_BODY_NODE_BUDGET` exactly as the React host caps its own report, and asserts every requested
window is honoured **in full**.

## 5. Point-by-point against `📓️f1-host-scroll-streaming.md` §10

| F1 point | Matched | Test that proves it |
| --- | --- | --- |
| 1. `TREE_WINDOW_BODY_NODE_BUDGET = 111` on one line in `🌳️Tree/🟦️.tsx` | ✅ The Rust constant lives in `semio_framework_ui_contract` and is derived, not typed: `UI_DOCUMENT_NODES - 1 - TREE_WINDOW_FIXED_NODE_HEADROOM = 111`. The SDK ledger opens on it. | `the_body_node_budget_is_the_record_arena_less_the_root_and_the_fixed_reserve` + the parity law `the_host_tree_element_declares_the_same_body_node_budget`, which reads the TS file and asserts the literal `TREE_WINDOW_BODY_NODE_BUDGET = 111;` |
| 2. Cost `1 + rows` per container, nested containers charged ONCE, no extra per-body headroom on top | ✅ `debit_container()` skips a container built inside a parent's row closure (`tree_window_rows` raises `nested`), so the parent's row grant is the only charge. Nothing is charged on top of the 111. | `a_nested_container_is_charged_exactly_once` asserts `spent == body_nodes(built)` exactly (`1 + 3 + 3×4 = 16`) |
| 3. Every container present, off-screen ones at `rows: 0`, cost one node, order-independent | ✅ Reservations are computed at `TreeWindows::seated` before any container is built, in request order, so the host's alphabetical key order cannot starve anything. `rows: 0` reserves 1 and materialises 0. | `a_host_request_outranks_a_first_paint_default_earlier_in_document_order`, `host_requests_beyond_the_ledger_are_clamped_in_request_order`, `an_open_request_for_zero_rows_materialises_nothing_and_keeps_its_offset` |
| 4. `rows: 0` is not "closed" — stays open, keeps its `offset` | ✅ `slice` takes the request branch whenever `open != Some(false)`; the zero-row case stamps `{ total, offset }` from the request and never falls back to the first-paint default. | `an_open_request_for_zero_rows_materialises_nothing_and_keeps_its_offset` (stamps `TreeWindow { total: 60, offset: 40 }`, zero children) |
| 5. Duplicate `node_key` reads as one fault with the host's log | ✅ `ui.tree-window.duplicate-key`, message `[tree-window] duplicate key "<key>" in this panel body — … give every windowed container its own node key`, the same wording as `🗣️Interpreter/🟦️.tsx:1650`. | `a_duplicate_node_key_in_one_body_is_refused` |

Additionally, beyond §10: a stale `offset` clamps to the LAST window (`total - rows`) rather than `total - 1`
(`a_stale_offset_clamps_to_the_last_window`), and a key the host filed twice is seated once so its
reservation cannot strand records.

## 6. Verification — what was actually run

All foreground, shared build dir, `DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0
CARGO_PROFILE_WASM_DEV_DEBUG=false`, no `CARGO_TARGET_DIR`. Logs under `🗑️generated/f2/`.

| Command | Result |
| --- | --- |
| `cargo test -p semio-framework-plugin --lib panel_kit_tests` | ✅ **26 passed; 0 failed** (773 filtered out) — every law in §4, parity law included |
| `cargo check -p semio-framework-plugin --target wasm32-wasip2` | ✅ `Finished dev profile in 24.44s`, 0 errors |
| `cargo test -p semio-framework-plugin --lib` (whole crate) | ⚠️ 309 ok, 78 FAILED, then SIGABRT. **All 26 panel-kit laws pass inside this run.** See §7. |
| `cargo test -p semio-framework-ui-contract` | ⚠️ 34 ok, 4 FAILED → SIGABRT. Pre-existing, bisected in BOTH directions in `📓️verify-retirement-laws-preexisting.md` §3. |
| `cargo test -p semio-framework-ui-runtime` | ⚠️ **122 passed; 2 failed; 2 ignored.** P5's `reconcile::tests::four_section_full_window_tree_reconciles_under_the_surface_byte_ceiling` ✅. Both failures pre-existing (`verify-retirement-laws-preexisting.md` §2, `p5-wgpu-and-reconcile-law.md` §3.2). |
| `cargo test -p semio-s-artifact-fem-3d --features component-app-assembly` | ⚠️ 1087 passed, 42 failed. **Both House laws pass.** All 42 are the demo-document swap, §8. |
| `cargo test -p semio-s-artifact-fem-3d --features component-app-assembly panels::artifact` (after the `wide_view` fix) | 9 passed, 6 failed — all six on demo CONTENT (`n00_g`, `"l1"`, `nodes.len() == 16`, `"Knoten (16)"`), none on windowing. |

## 7. Pre-existing failures, with the evidence

**`semio-framework-plugin` (78 FAILED → SIGABRT).** Not one is a panel/tree/window law. By module:
30 `plugin_builder_contract_tests`, 10 + 6 + 4 `mutation_fixture::{transaction,surface,dummy}`,
7 `app_builder_tests` (the interactive-job classification gate P3 §6 recorded), 5
`plugin_builder_dependency_tests`, 3 `schema_stamping_tests`, 3 `typed_command_full_operation_tests`, and
singles elsewhere. The process aborts inside
`plugin_builder_contract_tests::a_settled_reactor_turn_retains_nothing_the_guest_cannot_afford`
("a settled reactor turn retains 384172 B") whose destructor then panics
("runtime lifetimes require terminal exact ACK before teardown" → `panic in a destructor during cleanup`
→ SIGABRT), which takes the rest of the binary down with it. Three failing names merely CONTAIN
"window"/"tree" — `build_definition_accepts_introduction_with_declared_window_utility_and_action_targets`
(the classification gate), `fixture_projection_retires_exact_tree_before_return_error_or_panic` (the
retirement-arena family) and `addressed_window_action_injects_the_exact_window_instance_into_the_typed_handler`
(the plugin-runtime contract family) — none touches `TreeWindow`, `TreeWindows` or a panel body.

**`semio-framework-ui-contract` (4 FAILED → SIGABRT).** `binding_copy did not retire exact owners` →
`UiValueRetirement requires exact terminal closure` → `panic in a destructor during cleanup` → SIGABRT.
`📓️verify-retirement-laws-preexisting.md` §3 reproduces this byte-identically on the FULLY pre-change tree
(`f7fef5746d`, constants and struct fields reverted) and records that P1 also bisected the constants alone
on the post-change tree. F2's own change to this crate is two `pub const` declarations that nothing in the
crate reads.

**`semio-framework-ui-runtime` (2 FAILED).** `runtime_tree_retirement_preserves_occupied_sources_and_closes_exact_payloads`
(hard, pre-change and post-change alike, `verify-retirement-laws-preexisting.md` §2) and
`runtime_tree_retirement_handback_preserves_partial_owner_until_full_readmission` (recorded as flapping in
`📓️p5-wgpu-and-reconcile-law.md`).

## 8. ⚠️ fem3d: the bundled demo document was swapped under the crate's tests (peer-owned)

Commit `0b460ed19f` (2026-09-17 **12:02**, i.e. during this packet) changed `fem3d_boot_snapshot()` from

```rust
parse_dsl(FEM3D_EXAMPLE_TEXT).unwrap_or_else(|_| empty_fem3d_snapshot())
```
to
```rust
parse_dsl(crate::examples::concrete_forest::PRIMARY_TEXT).or_else(|_| parse_dsl(FEM3D_EXAMPLE_TEXT)).unwrap_or_else(…)
```

so `demo()` is now the **Betonwald** document (20 nodes `lc1b/lv0/uv6…`, 20 elements `l_col1…`, 0 solids,
2 supports, 2 load cases × 16 loads, 2 combinations, 1 material, 2 sections) instead of the two-storey hall
(16 nodes `n00_g…`, loads `l1`/`l2`/`l3`). All 42 failures assert on the OLD document's ids and counts —
36 of them in `commands::*`, `interaction::*`, `gumball::*` and `inspection::*`, which have nothing to do
with panels (`fem3d.patch.node-missing`, `fem3d.focus-entity.unknown-entity`, …). **Not fixed here**: the
whole crate needs one sweep by whoever swapped the example, and re-spelling six panel laws while 36 sibling
laws stay stale would only hide it.

**One part of it WAS mine and is fixed.** `wide_view()` asked for `rows: 128` on three sections. Under the
new reservation ledger that reserves `3 × (1 + 128)`, clamped to the whole 111-record budget, starving the
six sections the host has not addressed — `combinations` materialised 1 of 2 and `uls` lost both its terms.
`wide_view()` now asks for `rows: 4` (those three sections hold four rows between them), i.e. exactly what
the React host's own cap allows, and the demo body materialises **whole** again: both combinations with both
terms, both cases with all 16 loads, all 20 nodes and elements, 97 of the 128 records. Verified in the
re-run's projected body JSON (`🗑️generated/f2/fem3d-artifact.txt`).

## 9. w3 §6 item 1 — `items: 4098 vs max_items: 4097` on `fem3d-model` / `fem3d-results`

**Verdict: NOT caused by this ticket's budget changes.** Time-boxed investigation, evidence:

1. `max_items: 4_097` is a hard literal in `SurfaceReconcileLimits::default()`
   (`🧰️framework/🔨️modules/🖱️ui/🧠️runtime/♻️reconcile/🦀️.rs:584`), derived from neither
   `UI_BUILT_CHILDREN_MAX` nor `UI_VALUE_PAGE_ROWS`.
2. The two faulting surfaces are **World3d scene bodies**, not panel trees: `FEM3D_WINDOW_MODEL`/
   `FEM3D_WINDOW_RESULTS` render `world_3d_surface(FEM3D_BODY_MODEL, &model_scene(..))` over a
   `World3dScene` (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/🎬️scenes/🦀️.rs:349`) — a record of JSON `String`
   lanes with **no `BuiltNode` children** (so `UI_BUILT_CHILDREN_MAX`, a child-list fan-out bound, cannot
   apply) and **no action argument maps** (so `UI_VALUE_PAGE_ROWS`, which sizes the argument arena through
   `UI_VALUE_ADMISSION_SLOTS`/`UI_VALUE_AGGREGATE_ITEMS`, cannot apply).
3. `items` is the semantic-ownership census (`SurfaceSemanticUsage`, charged per node field and per page of
   string bytes), so it scales with the scene payload's SIZE — i.e. with the document, which is exactly what
   changed when the probe switched the example to House.
4. `fem3d_scene_parts` (`…/🌐️any/🎬️scene/🦀️.rs:331`), the builder of both lanes, references neither constant.

Left alone, as instructed. It is a real defect — a large FEM model cannot publish its scene surface — but it
belongs to whoever owns the World3d scene lane budget, not to this ticket.

## 10. `node_key` collision exposure across the fleet (coordinator decision 3)

Audited every `tree_window_section` / `window_section` / `tree_window_section_or_placeholder` /
`tree_window_item` call site in the repo. **No app fails unconditionally** — there is no constant key inside
a loop anywhere. Every finding is data-dependent. Ranked:

| Risk | Where | Why |
| --- | --- | --- |
| **HIGH (data-triggered)** | `🗄️stdio` → `🧾️json` **🧱️base** subset, editor + viewer main window (`…/🪟️main/🦀️.rs:44,52`) | the node key is the member path (`k={key}`), and RFC 8259 base PRESERVES duplicate object member names, so `{"a":{…},"a":{…}}` claims one key twice. The `🛜️i-json` sibling is safe (it rejects duplicate member names). Narrowed by the leaf exemption in §2.4: only duplicates that BOTH have children collide. |
| MEDIUM | `📐️cad` artifact panel (`…/📌️panels/🗿️artifact/🦀️.rs:59`) | raw `&object.id` across **four** pane sections of one body. The shipped play fixture's 24 objects are disjoint (`-bim-N`/`-energy-N`/`-classic-structure-N`), but an imported model reusing ids across panes breaks the panel. |
| MEDIUM | `🏗️fem` ◻️2d (`:281`,`:296`) and 🧊️3d (`:277`,`:293`) artifact panels | raw `&case.id` and `&combination.id` in one body — two entity kinds sharing one id space. |
| MEDIUM | `✒️writer` artifact panel (`:62`) | AST id `jack-ast-{kind}-{start}-{end}`; two same-kind siblings with the same span collide. |
| MEDIUM | `🌊️flow` catalogue (`:71`) | section key built in a loop from HOST-supplied catalogue section ids; nothing dedups them. |
| LOW–MED | `🏭️process` inspection (`:149`) and workshop (`:88`), `🌀️procedural` structure window (`:71`,`:76`) and 🧊️generation3d catalogue (`:72`) | keys from capability / catalog / module / palette ids, none deduped. |
| safe | `🔋️energy` (one `EntityId` space by design), `🧩️puzzle` (`.or_else` chains), `🖍️draw` (exclusive match arms), fem **inspection** panels (exclusive `match kind` arms), and ~20 others | distinct literals or properly namespaced |

⚠️ **I did NOT namespace the fem/cad ids, and the coordinator should decide before anyone does.** The window
node key IS the built node's `key` (`🗣️Interpreter/🟦️.tsx:1823` `windowKey: record.key`), and that same key is
the **pick target id** the tree-level `interactionSelect` dispatches (design §6.2, `targets: [{granularity,
id: record.key}]`). Namespacing a load case's key to `"{NS}.load-case.{id}"` would either break the pick or
break the host↔guest key match. The clean fix is one layer up: key host window state by
`granularity + key` rather than `key` alone, or give the DOM a `data-tree-window-key` that is genuinely
independent of `record.key`. That is a P4b/F1 contract change, not an app edit.

## 11. What is NOT finished

- The six fem3d artifact-panel laws and the 36 sibling command/interaction laws stale-dated by the
  concrete-forest demo swap (§8) — needs one sweep by the fem owner.
- The `items: 4098 > 4097` World3d scene-surface fault (§9) — evidence recorded, owner unassigned.
- The `node_key` collision exposure (§10) — the SDK now refuses loudly; the app-side ids are unchanged
  pending the coordinator's call on the pick-id/window-key coupling.
- No browser re-run: `🐍️tree-window-probe.mjs` on fem3d House would now be worth re-running (the body
  ledger plus F1's observer fix are both in), but this packet started no servers.

---

# 📍️ Checkpoint — container PATH identity, headroom, fem3d repairs (2026-09-17, after the R2 hand-off)

Sections 1–11 above describe the packet as of the BODY-LEDGER milestone. Everything below supersedes them
where they differ. Written before the re-runs so the on-disk state is auditable if this session is cut again.

## C1. On disk, done

**Container identity is the window PATH** (coordinator decision superseding §10's open question):

| Where | What |
| --- | --- |
| `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🧩️component/🦀️.rs` | `pub const TREE_WINDOW_PATH_SEPARATOR: &str = "\u{1f}"` beside `TREE_WINDOW_BODY_NODE_BUDGET`, with the path definition and the "U+001F cannot occur in an authored node key" justification |
| `🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` | `TreeWindowRequest.node_key` documented as the PATH, with the `uls`-in-two-collections and cad-four-panes examples |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` | `TreeWindows.nested: Cell<bool>` → `path: RefCell<Vec<String>>`; `path_of(node_key)` joins the stack; `tree_window_rows` pushes the container's own key around EVERY row closure (so the stack is also what `is_nested()` — the charge-once rule — reads); `is_open`/`slice` resolve the path and callers still pass the plain id; `sliced`/`stamp`/`claim_window` key by path; re-exports extended |
| test file `🔬️app-panel-kit/🦀️.rs` | `one_id_under_two_parents_keeps_two_independent_windows`, `a_window_path_is_the_key_at_the_top_and_the_parent_chain_below`, separator half of the parity law |

**Headroom** (`TREE_WINDOW_FIXED_NODE_HEADROOM` 16 → **24**, budget 111 → **103**): measured, not guessed —
energy's inspector with a fenestration selected is the fleet's fattest un-windowed block at **21** records
(`fenestration_rows` pushes exactly 16 field rows, `…/📌️panels/🔍️inspection/🦀️.rs:287-308`, plus its own
section node plus `Actions` + ≤3). Next: fem2d/fem3d solid inspector 15, history panel 7. Pinned by
`tree_window_headroom_covers_the_fattest_shipped_panel`, which rebuilds that body shape and asserts the
un-ledgered count is 21 and the whole body still fits `UI_DOCUMENT_NODES`. The TS literal was moved to
`TREE_WINDOW_BODY_NODE_BUDGET = 103` in the same edit, which `📓️f1-host-scroll-streaming.md` §10.1
explicitly authorises ("if the SDK ledger's real figure is not 111, change the Rust constant and this line
together"). **F1 must re-check `capTreeWindowRequests`' own tests against 103.**

*Why not "make non-windowed nodes debit for real":* not feasible without changing
`PanelTreeBuilder::section`'s signature in every app panel (dozens of crates, peers editing them live), and
it would not even close the hole — energy builds its windowed selection list BEFORE its 21 fixed rows, so a
charge at `.section()` time arrives after the grant it was supposed to bound. The headroom + law is the
honest bound.

**fem3d panel laws repaired against the live demo.** A peer swapped `fem3d_boot_snapshot()` to the
concrete-forest example (commit `0b460ed19f`, §8): 20 nodes `lc1b…uv6`, 20 elements `l_col1…u_b6`, 0 solids,
2 supports `s_c1/s_c2`, 2 load cases × 16 loads (`d_*`/`q_*`), 2 combinations `uls`/`sls`, 1 material `c30`,
2 sections `hex30`/`beam30x45`. Updated in
`✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs`: the six window/panel
laws' ids, labels, German headers and counts; `wide_view()` down to `rows: 2` (the concrete-forest body needs
97 of the 103 records, so the three host-addressed sections may only reserve what they actually hold); and a
`nested_request(section, key, …)` helper so `wind` and the House law's `hc0…hc3` are addressed by their PATH
(`…load-cases␟wind`) rather than a bare id, which is what the new identity requires.

## C2. Verified after these changes

- `cargo test -p semio-framework-plugin --lib panel_kit_tests` → ✅ **29 passed; 0 failed** (773 filtered
  out). The three new laws and the separator parity all green; every earlier law still green at the new
  103-record budget (their expectations are derived from `TREE_WINDOW_BODY_NODE_BUDGET`, not typed).

## C3. Still to re-run in this session

- `cargo check -p semio-framework-plugin --target wasm32-wasip2`
- `cargo test -p semio-s-artifact-fem-3d --features component-app-assembly panels::artifact`

Each is appended to §C4 as it lands.

## C4. Re-runs after the PATH + headroom + fem3d changes

All foreground, shared build dir, `DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0
CARGO_PROFILE_WASM_DEV_DEBUG=false`, no `CARGO_TARGET_DIR`. Logs under `🗑️generated/f2/`.

| Command | Result | Log |
| --- | --- | --- |
| `cargo test -p semio-framework-plugin --lib panel_kit_tests` | ✅ **29 passed; 0 failed** (773 filtered out) | `panel-kit-path.txt` |
| `cargo check -p semio-framework-plugin --target wasm32-wasip2` | ✅ rc 0, **0 errors**, `Finished dev profile in 1m 14s` (40 warnings ⇒ expansion really ran) | `wasm-check.txt` |
| `cargo test -p semio-s-artifact-fem-3d --features component-app-assembly panels::artifact` | ✅ **15 passed; 0 failed** (1114 filtered out) — was 9/6 before the demo-document repairs | `fem3d-artifact.txt` |

The fem3d artifact panel is now fully green, including both House laws, the nested-by-path `wind` /
`hc0…hc3` requests, and `demo_document_lists_every_section_with_its_own_count` materialising the whole
concrete-forest body (97 of 103 records) again.

## C5. fem3d inspection — the two WINDOWED laws repaired too

The inspector's windowed containers (R2 §1.2: the multi-selection list, a load case's loads, a
combination's terms) had two laws still naming the old demo's entities. Repaired in
`…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs`: `a_selected_load_case_lists_its_loads_as_picks_3d`
(`"l2"`/`"l3"` → `"q_l_spine"`/`"q_l_b0"`, the live case's own loads) and
`a_multi_selection_headers_the_count_and_inspects_the_first_3d`
(`["n00_g","n20_g","e1"]` → `["lc1b","lc2b","l_col1"]`). Both green.

| Command | Result | Log |
| --- | --- | --- |
| `cargo test … fem-3d --features component-app-assembly panels::` | **26 passed; 9 failed** (was 15 passed / 11 failed) | `fem3d-panels-final.txt` |
| `cargo test … fem-3d --features component-app-assembly` (whole crate) | **1095 passed; 34 failed** (was 1087 / 42) | `fem3d-full.txt` |

**The 9 remaining panel failures are deliberately left**, and none is window-related: they are the
inspector's bounded FIELD FORMS (`a_selected_node_renders_bound_ordinate_inputs_3d`,
`a_selected_frame_offers_its_reference_selects_and_roll_3d`,
`a_selected_support_renders_one_toggle_per_dof_3d`,
`a_selected_material_mixes_inputs_and_a_bounded_slider_3d`,
`a_selected_solid_edits_its_extrusion_and_shows_its_polygon_read_only_3d`,
`a_selected_load_names_its_owning_case_3d`, `a_node_section_carries_a_number_input_bound_to_patch_node_3d`,
`the_actions_group_binds_focus_and_delete_as_tree_items_3d`,
`german_resolves_every_field_label_the_inspector_binds_3d`). They select entity KINDS the concrete-forest
document no longer contains at all — it has **zero solids**, no `steel`, no `hea200` — so they cannot be
repaired by substituting ids; they need new fixtures, which is the fem owner's sweep, not a window repair.
The other 25 failures are outside `panels::` entirely (`commands::patch_*`, `interaction::gumball::*`,
`scene::*`, `viewer::*`), same cause.

## C6. What F1 must re-check after this packet

1. **`TREE_WINDOW_BODY_NODE_BUDGET` is now 103, not 111** (headroom 16 → 24, §C1). The TS literal in
   `🌳️Tree/🟦️.tsx` was updated in the same edit per `📓️f1-host-scroll-streaming.md` §10.1, and the Rust
   parity law `the_host_tree_element_declares_the_same_body_node_budget` is green against it — but
   `capTreeWindowRequests`' own component tests in `🌳️Tree/🧪️tests/🧩️component/🟦️.tsx` assert
   `nodeCost(capped) === TREE_WINDOW_BODY_NODE_BUDGET`, so re-run them.
2. **`TREE_WINDOW_PATH_SEPARATOR` parity** is now pinned by the same Rust law: it accepts either the raw
   U+001F character or the `""` escape on the `export const … = "…";` line. F1's current spelling
   (the raw character) passes.
3. The guest joins a path as `outer SEP … SEP own-key`, with NO leading separator for a top-level section —
   `treeWindowPath(parentWindowPath, windowKey)` in `🌳️Tree/🟦️.tsx:973` already matches this exactly.

## C7. Still open after this packet

- The 9 fem3d inspector field-form laws and the 25 non-panel fem3d laws (§C5) — the fem owner's fixture
  sweep after the concrete-forest demo swap.
- `items: 4098 > max_items: 4097` on the fem3d World3d scene surfaces (§9) — proven unrelated to this
  ticket's budgets, owner unassigned.
- App-side TRUE-sibling key duplicates the path rule cannot remove (§10 remains accurate for these two
  only): stdio `🧾️json` 🧱️base with duplicate object member names that BOTH have children, and writer's
  same-span AST ids. Every other app in §10 — cad's four pane sections, fem's case-vs-combination ids,
  process, procedural, flow's catalogue — is collision-free by construction now that identity is the path.
- No browser re-run; this packet started no servers.

## C8. Separator corrected to a PRINTABLE glyph — the view-context identifier law

A browser probe on fem3d caught the U+001F separator being rejected at the process boundary: the
view-context admission (`parseResolvedPluginViewState` / `admitCrossingViewContext`,
`🧰️framework/🔨️modules/🛂️manifest/🟦️.ts:977`) refuses any identifier carrying a C0 or DEL code point or
longer than 256 code points, so every refresh, action and pick carrying a nested path threw
`view context: invalid identifier` — killing streaming, lazy expand and picks as soon as one nested
container existed. Corrected:

| Change | Where |
| --- | --- |
| `TREE_WINDOW_PATH_SEPARATOR` is now **U+241F SYMBOL FOR UNIT SEPARATOR (`␟`)** — the printable glyph, not the C0 control it depicts | `🧬️contract/🧩️component/🦀️.rs`, with the fault and the law that forced it on the docstring |
| A container key that already CONTAINS the separator is refused, `ui.tree-window.separator-in-key` | `TreeWindows::admit_key`, called first by `tree_window_section`, `tree_window_section_or_placeholder` and `tree_window_item` |
| A path is documented as a view-context identifier (printable, ≤ 256 code points) | `TreeWindowRequest.node_key` in `🛂️manifest/🦀️.rs`, and `TreeWindows::path_of` |
| The parity law now accepts ONLY `TREE_WINDOW_PATH_SEPARATOR = "␟";` | `the_host_tree_element_declares_the_same_body_node_budget` |

**Rust side checked (item 3):** the identifier law lives only in the TS admission — `🛂️manifest/🦀️.rs`
carries no `invalid identifier` / control-character check of its own, and `ViewModel`/`TreeWindowRequest`
are admitted there by shape and capacity (`tree window capacity exceeded`, 128 entries) rather than by
character class. So the TS law is the one gate a path must pass, and `␟` passes it.

**Length (item 4):** a path over 256 code points is one the host never files a request for, so the guest
must treat that container as UNREQUESTED — author default plus the shared first-paint budget. That is
already exactly what an absent seat means in `slice`, and it is now pinned by
`a_path_the_host_cannot_send_renders_as_an_unrequested_container` (a 300-character key still stamps its
full extent and takes its first-paint share).

**⚠️ stdio json/xml build keys that WILL exceed it — handed to A8c, not edited here.**
`…/🗄️stdio/🗿️artifacts/🧾️json/🏅️standards/🔖️rfc8259/🪆️subsets/🧱️base/👁️viewer/…/🪟️main/🦀️.rs:48`
already sets each node's id to `path.join("/")` — the WHOLE path, not the sibling segment — and the same
line exists in the 🧱️base editor and both 🛜️i-json copies, with XML's positional ids the same shape. The
window path then joins those keys again, so the addressable identifier grows QUADRATICALLY with depth:
with ten-character member names a depth-7 document is already past 256 code points, and every container
below that silently stops streaming (it still renders, at its first-paint slice).

The fix is one line per file and makes the path *cheaper*, not more expensive: give each node the
**sibling** key it already computes (`format!("k={}", member.key)` / `format!("i={index}")`) instead of
`path.join("/")`, and let `TREE_WINDOW_PATH_SEPARATOR` reconstruct the full path — which is what the path
mechanism is for. These are `TreeWindowKit` view trees with no interaction domain, so the keys are not
pick target ids and nothing else reads them. **Not applied here: A8c is live in that crate (the file was
written at 15:47 while this packet ran).** Re-read before editing.

| Command | Result | Log |
| --- | --- | --- |
| `cargo test -p semio-framework-plugin --lib panel_kit_tests` | ✅ **31 passed; 0 failed** (773 filtered out) — the two new laws included | `panel-kit-sep.txt` |
| `cargo check -p semio-framework-plugin --target wasm32-wasip2` | ✅ rc 0, **0 errors**, `Finished dev profile in 38.66s` | `wasm-check.txt` |

⚠️ **Guests must be restaged** before any further browser run: the separator is part of the wire identity,
so a guest built before this change answers a different string than the host now sends.
