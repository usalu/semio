# Wave O — The Virtualised Outliner and the Explicit `UiValue` Arena Bound (2026-09-09)

Two defects, one root cause. The 3d outliner could not render the Nakagin example
(`📓️2026-09-09-wave-G2-locale-and-tree-memo.md` §5.1), and the retained tree competed for a 256-slot
process-global value arena across 64 session slots (§2.4 / §6 item 3). Measurement showed both are the same
thing: **the `UiValue` argument arena was sized at 256 pages, which is about eleven outliner rows for the
whole process.**

Both are closed. The outliner now pages, the catalogue pages with it, and the arena's size is a derivation
from the UI contract's own constants instead of a magic 256.

---

## 1️⃣ The measurement that changed the design

The brief's hypothesis was the fixed UI list capacity (`UiFixedList<T, 32>` / `UI_BUILT_CHILDREN_MAX = 32`)
and suggested deriving capacities from `DOCUMENT_OBJECT_SLOTS`/`DOCUMENT_VORTEX_SLOTS`. A temporary probe
(added to the panel's own test file, then removed) rendered synthetic fixtures of growing size:

```
[DEBUG] objects=1   ok children=4
[DEBUG] objects=8   ok children=4
[DEBUG] objects=12  ERR code=ui.fixed-capacity message=fixed UI map entry admission failed
[DEBUG] objects=32  ERR code=ui.fixed-capacity message=fixed UI map entry admission failed
[DEBUG] objects=180 ERR code=ui.fixed-capacity message=fixed UI map entry admission failed
[DEBUG] last_ok=8
```

It fails at **twelve objects**, not at thirty-three — so the 32-child cap was never reached. The message is
`fixed UI map entry admission failed`, i.e. `ui_value_map`'s `UiMapBuilder::push` refused, i.e. the
process-global `UiValueArena` ran out of **pages**, not the panel out of list slots.

Arithmetic that matches the observation exactly. One outliner object row authors:

| argument | collections | pages |
|---|---|---|
| `interactionSelect` args map (`domainId`/`merge`/`method`/`targets`) | 1 | 4 |
| `setSelectionFlag` hidden — map (`entity`/`flag`/`ids`/`value`) + nested one-id list | 2 | 5 |
| `setSelectionFlag` locked — same | 2 | 5 |
| **object row total** | **5** | **14** |
| one nested vortex row (select only) | 1 | 4 |

At two vortices per object that is 22 pages per object against `UI_VALUE_AGGREGATE_ITEMS = 256`: the arena
is exhausted at ⌊256/22⌋ = 11 objects. Observed last success 8, first failure 12. ✅

**Consequence for the brief's first option.** Deriving the panel's capacities from `DOCUMENT_OBJECT_SLOTS`
(2048) would need ~45,000 arena pages at 1,056 bytes each — ~47 MB of process-global backing, past the
contract's own `UI_RESIDENT_AGGREGATE_BYTES` (32 MiB) ceiling, and a 2048-wide `BuiltChildren` would make
every node that pushes a single child allocate a 16 KiB backing. The brief's second option — virtualisation
— is the only one that is both correct and affordable, and it is what this wave implemented.

---

## 2️⃣ Defect 1 — the outliner is virtualised

`✏️editor/📌️panels/🗿️artifact/🦀️.rs`. The per-entity `for` loops are gone; the panel now assembles **one
bounded page**.

### 2.1 The two bounds it respects

```rust
/// 🗂️ Rows one section of this page materialises before it truncates: a built node admits
/// `UI_BUILT_CHILDREN_MAX` children and the last of them carries the section's continuation row.
pub const SECTION_ROWS: usize = ui::UI_BUILT_CHILDREN_MAX - 1;      // 31
pub const SECTIONS: usize = 4;                                      // objects/refs/volumes/attractions

fn page_rows() -> usize {
    ui::UI_VALUE_PAGE_ROWS.min(ui::ui_value_headroom().rows())      // 31, clamped by live credit
}
```

- **Per section** (`SECTION_ROWS`): the built-children contract. 31 rows + 1 continuation row = exactly the
  32 children a node admits. This holds for a section's rows *and* for an object's nested vortex rows.
- **Per page** (`page_rows`): `UI_VALUE_PAGE_ROWS` from the contract (§3), clamped by what the arena still
  admits. The clamp is what makes the builder **total**: a document an order of magnitude past the page, or
  a process whose other panels already hold their own pages, yields a shorter page with continuation rows
  rather than a `ui.fixed-capacity` refusal inside one row's argument map.

### 2.2 The mechanism

- `struct RowBudget(usize)` — the allowance one render spends across its sections in assembly order
  (objects → references → target volumes → attractions). `spend()` is a `checked_sub`, never saturating.
- `paged_section(section_id, entries, budget, row)` — pushes rows while both `SECTION_ROWS` and the shared
  budget last, then appends a continuation row whenever entries were left out.
- `RowBudget::nested(reserved, build)` — a nested build draws on an allowance that cannot reach the slots
  its siblings still need, and whatever it actually spent is settled back against the shared budget. It is
  used at both levels: a section reserves the sections after it, and a row reserves its section's remaining
  rows. **This was measured, not designed on paper:** without it, the catalogue law's 200-kind ×
  40-template fixture materialised **four** kind rows, because kind 0's nested templates ate the page. With
  it the same fixture fills its section and the nesting takes only the surplus, in document order.
- `continuation_row(section_id, omitted)` — id `"<section>.more"`, icon `ellipsis`, label `"+{omitted}"`.
  **Label decision:** a digit string carries the same meaning on every locale × terminology axis this app
  authors, so paging stays visible in the tree without adding an eleventh label field that the byte-exact
  terminology gate (`interactivityPuzzleFillPreviewJsonFailures`, W-G2 §1.1) would then have to pin — and
  without editing `🗣️terminology/🦀️.rs`, which is not this wave's file.
- One `*_row` fn per entity kind (`object_row`, `vortex_row`, `reference_row`, `target_volume_row`,
  `attraction_row`), extracted from the old loop bodies **verbatim**. Nothing about a row's content changed:
  every object/reference/target-volume row still carries its `interactionSelect` activation binding and both
  `setSelectionFlag` hide/lock `RowAction`s, and the memo (`document_tree_cached`, keyed on the geometry
  fingerprint + label-set identity, in W-D's `✏️editor/🦀️.rs`) was not touched.
- Section ids are now built from a single `const ROOT: &str = "puzzle3d-play-document"` instead of five
  repeated literals. The strings are byte-identical to before.

### 2.3 What Nakagin now renders

Budget 31 rows, allocated by two reservations that fall out of the same `RowBudget::nested` primitive:
each section is assembled with the sections after it reserved (`SECTIONS - 1`, `- 2`, `- 3`, `- 4`), and
each row's children are assembled with that section's remaining rows reserved.

The emergent behaviour is exactly the one the brief asked for, without a special case for it:

| document | what the page materialises |
|---|---|
| Nakagin, 180 objects × ~2 vortices | 28 object rows, **no vortex children** (every object row is competing for the page), `+152`, then one row each for references / target volumes / attractions with their own `+N` |
| a small document, 5 objects × 2 vortices | all 5 object rows **with all 10 vortex children**, no continuation |

"collapsed objects do not emit their vortex children" is therefore a consequence of the reservation under
pressure rather than a rule the builder hard-codes — a small document keeps the nested rows it can afford.
Node fan-out never exceeds 32 anywhere.

---

## 3️⃣ Defect 2 — the arena bound is now derived, and the derivation is stated

`🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs`. `UI_VALUE_ADMISSION_SLOTS` and
`UI_VALUE_AGGREGATE_ITEMS` were both the literal `256` with no stated reason. They are now products of
constants that each name a real contract:

```rust
pub const UI_VALUE_ARGUMENT_ENTRIES: usize = 4;                 // widest authored argument map
pub const UI_VALUE_ROW_ACTIONS: usize = 2;                      // hide + lock, each map + nested id list
pub const UI_VALUE_ROW_COLLECTIONS: usize = 1 + 2 * UI_VALUE_ROW_ACTIONS;                              // 5
pub const UI_VALUE_ROW_ITEMS: usize = (1 + UI_VALUE_ROW_ACTIONS) * UI_VALUE_ARGUMENT_ENTRIES + UI_VALUE_ROW_ACTIONS;  // 14
pub const UI_VALUE_PAGE_ROWS: usize = crate::UI_BUILT_CHILDREN_MAX - 1;                                // 31
pub const UI_VALUE_LIVE_PAGES: usize = 1;
pub const UI_VALUE_ADMISSION_SLOTS: usize = UI_VALUE_LIVE_PAGES * UI_VALUE_PAGE_ROWS * UI_VALUE_ROW_COLLECTIONS;  // 155
pub const UI_VALUE_AGGREGATE_ITEMS: usize = UI_VALUE_LIVE_PAGES * UI_VALUE_PAGE_ROWS * UI_VALUE_ROW_ITEMS;        // 434
pub(crate) const UI_VALUE_HANDBACK_WORDS: usize = UI_VALUE_ADMISSION_SLOTS.div_ceil(64);               // 3
```

`UI_VALUE_ROW_COLLECTIONS`/`UI_VALUE_ROW_ITEMS` are exactly the per-row costs measured in §1, so the
derivation is calibrated against a real panel rather than a guess. The page grew 1.7× (256 → 434 pages,
256 → 155 collections is a *reduction*, because a collection is now sized per row rather than per page).

### 3.1 The ceiling that actually binds: the arena is charged to the resident authority

**This wave's most important finding, and it overturned a first attempt.** `🎟️resident/🦀️.rs` starts the
resident ledger's `snapshot.bytes` at `CONTRACT_BACKING_BYTES`, which *includes*
`crate::action::resident_static_backing_bytes()` — the whole `UiValue` arena. Its fixture says so out loud
(`🎟️resident/🗃️fixed/🧫️fixture/🔣️.json`: `"staticCountsAgainstAggregate": true`), and the aggregate ceiling
is pinned to exactly four resident surfaces by a language-agnostic law
(`🧪️tests/🔬️fixed-list-storage/🟦️.ts:130`: `pressureRoots * pressureReservationBytes ==
pressureAggregateBytes`, i.e. 4 × 8 MiB == 32 MiB).

So **every byte this arena reserves is payload the live surfaces cannot have**, and `UI_RESIDENT_AGGREGATE_BYTES`
cannot be raised to compensate without breaking that authored law.

Measured, with `cargo test -p semio-framework-ui-runtime --lib -- --test-threads=1 --skip
surface_ownership_resident_return_maintenance_preserves_contended_credit` (that one test deadlocks — §7.6):

| arena | backing | ui-runtime result |
|---|---|---|
| 256 pages / 256 collections (**HEAD baseline**) | ~0.31 MiB | 106 passed, **17 failed** |
| 6944 / 2480 (first attempt, 4 sections × 4 live pages) | 7.37 MiB | 93 passed, **30 failed** |
| 1736 / 620 (4 sections × 1 live page) | 1.93 MiB | 105 passed, **18 failed** |
| 868 / 310 (2 sections × 1 live page) | 0.96 MiB | 105 passed, **18 failed** |
| **434 / 155 (shipped)** | **0.46 MiB** | **106 passed, 17 failed — baseline parity** |

The extra failures are `Credits { … }` aggregate-capacity refusals in the reconcile/output-pool laws, i.e.
exactly the resident payload the arena took. The shipped size is the largest measured point that holds the
suite at its pre-existing baseline, and it is why `UI_VALUE_PAGE_ROWS` is one node's worth of rows rather
than one per section. It is also, independently, the right UI number: a panel viewport shows about that many
rows, and the missing piece is a paging cursor to move through them (§7.3), not a taller page.

### 3.2 Why the 64 session slots do not multiply the factor

The worry in W-G2 §2.4 was `PUZZLE3D_SESSION_SLOTS = 64` retained trees × one page each. Read against the
arena, **a memoized tree costs no additional arena credit**: `BuiltNode::credited_clone` reaches
`UiList/UiMap::credited_clone` → `UiValueArena::try_clone_handle`, which increments `aliases` and allocates
neither a collection nor a page. A retained tree and the live tree it aliases share one page's worth of
credit; the alias only keeps that credit from being *returned*. So the multiplier is the number of
**distinct** live pages.

`UI_VALUE_LIVE_PAGES = 1` therefore does not mean "only one panel may render". It means the arena reserves
one page of headroom, and every author beyond the first reads `ui_value_headroom()` and pages against what
is left. That is what closes the defect: there is no state in which a live build *fails* because other
panels or stale slots hold credit — it degrades to a shorter page with continuation rows.

### 3.3 Measured cost of the shipped size

```
[DEBUG] arena pages=434 page-bytes=1056 collections=155 collection-bytes=120
        aggregate-bytes=476904 backing-bytes=483192 elapsed-ms=7
```

0.46 MiB of lazily-allocated backing (`LazyLock<Mutex<UiValueArena>>`, so nothing is paid by a process that
authors no `UiValue` collections). Construction stays inside the existing
`arena_initialization_is_a_fixed_control_and_page_taxonomy` law's 8 ms budget. That census `eprintln!` is now
a permanent part of the law, matching the `[DEBUG]` census style the sibling contract tests already use.

### 3.4 Two couplings the resize forced apart — both real design fixes, not workarounds

1. **Comparison depth was `UI_VALUE_ADMISSION_SLOTS`.** `ValueComparison { frames: [ValueFrame;
   UI_VALUE_ADMISSION_SLOTS] }` (`⚖️compare/🦀️.rs`) is a *nesting-depth* frame stack living by value inside
   `UiComponentComparisonCursor`, whose fixture pins `size_of::<UiComponentComparisonCursor>() <= 4096`.
   Tying a value's nesting depth to how many collections the arena admits is wrong on its own terms, and it
   would have blown that envelope at the first attempt's 2480 frames (19.8 KiB). New
   `pub const UI_VALUE_NESTING_DEPTH: usize = 256` carries the depth; the cursor's size is unchanged, and
   the `⚖️compare` fixture's `logicalDepth: 256` is unchanged with it.
2. **Handback mailbox words were the literal `4`.** `UiArenaHandbacks<SLOTS, WORDS>` static-asserts
   `WORDS * 64 >= SLOTS`. New `pub(crate) const UI_VALUE_HANDBACK_WORDS: usize =
   UI_VALUE_ADMISSION_SLOTS.div_ceil(64)` replaces the literal at its four use sites, so the mailbox can
   never fall behind the slot row again.

### 3.5 New additive API

```rust
pub struct UiValueHeadroom { pub collections: usize, pub items: usize }
impl UiValueHeadroom { pub fn rows(self) -> usize }
pub fn ui_value_headroom() -> UiValueHeadroom
```

Reads the arena's free credit; admits nothing, retires nothing. This is what makes the outliner total, and
it is the primitive any other virtualised author needs. `rows()` expresses the free credit in the same unit
the page ceiling uses.

---

## 4️⃣ Defect 3 — sibling panels

### 4.1 Catalogue — fixed (`✏️editor/📌️panels/🛍️catalogue/🦀️.rs`)

Would overflow. Nakagin declares 12 object kinds and 18 vortex kinds, but `DOCUMENT_KIND_SLOTS = 256`
admits far more, and each of the four kind sections pushed one row per entry into a 32-slot
`UiFixedList` — so a catalog with 33 object kinds, or one object kind with 33 vortex templates, faults with
`fixed catalogue admission failed`. Arena pressure is mild here (an object-kind row authors one 1-entry
`objectKind` map = 1 collection + 1 page; the vortex/cable/attraction kind rows author no arguments at all),
but the child cap binds regardless.

Now paged on the same primitives, imported from the outliner rather than restated
(`use crate::editor::puzzle3d::panels::document::{page_rows, paged_section, RowBudget}` — CLAUDE.md's
"if code is repeated, it MUST be close to each other"; the panels are siblings, and the shared helper module
`✏️editor/🦀️.rs` belongs to wave W-D). The now-unused local `fixed_nodes` helper was removed, and the four
section ids come from a `const ROOT` like the outliner's. `object_kind_vortex_items` pages the nested
templates against the same budget.

### 4.2 Inspection — reported, not edited (W-D owns it)

`✏️editor/📌️panels/🔍️inspection/🦀️.rs` is bounded by construction on rows: it renders one selected entity,
at most ten `read_only`/`flag_row` rows in a single section, so neither the 32-child cap nor the page
ceiling is at risk. **One real hazard remains, and it is not the row count:**

```rust
fn id_list_value(ids: &[String]) -> UiAssemblyResult<UiValue> { … for id in ids { builder.push(…) } … }
```

`flag_row` embeds the *whole selection* as a nested `ids` list, one arena **page per selected id**, and does
it twice (hidden + locked) per entity that has flag rows — objects, target volumes and references. Selecting
180 objects therefore authors ≈360 pages from the inspector alone; with `UI_VALUE_MAX_ITEMS = 256` capping
one collection, a selection over 256 ids fails outright with
`puzzle3d inspector action list entry admission failed`. The clean fix is the same shape as this wave's:
bound the id list the inspector embeds (a `patchInspector` that reads the live selection, or an explicit
capped id list with the count in the row description). Left to W-D, whose file it is.

---

## 5️⃣ Files changed

| path | change |
|---|---|
| `✏️s/…/🧊️3d/…/✏️editor/📌️panels/🗿️artifact/🦀️.rs` | virtualised: `ROOT`, `SECTION_ROWS`, `SECTIONS`, `page_rows`, `PANEL_PAGE_GUARD` (`#[cfg(test)]`), `RowBudget` (+`nested`), `continuation_row`, fail-soft `paged_section`, five extracted `*_row` fns, paged `render` |
| `✏️s/…/🧊️3d/…/✏️editor/📌️panels/🗿️artifact/🧪️tests/🔬️unit/🦀️.rs` | 3 new laws + `scaled_fixture`/`walk`/`page_guard` helpers beside W-G2's memo law (which also takes the guard) |
| `✏️s/…/🧊️3d/…/✏️editor/📌️panels/🛍️catalogue/🦀️.rs` | paged sections (each reserving the ones after it) + paged nested vortex templates; `ROOT`; `fixed_nodes` removed |
| `✏️s/…/🧊️3d/…/✏️editor/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | 1 new law (over-wide catalog); the existing drag-data law takes the page guard (one line) |
| `🧰️framework/…/🖱️ui/🧬️contract/📦️packages/🦀️rust/🎬️action.rs` | derived arena constants; `UI_VALUE_NESTING_DEPTH`; `UI_VALUE_HANDBACK_WORDS`; `UiValueHeadroom` + `ui_value_headroom()` |
| `🧰️framework/…/🖱️ui/🧬️contract/⚖️compare/🦀️.rs` | `ValueComparison::frames` sized by `UI_VALUE_NESTING_DEPTH` |
| `🧰️framework/…/🖱️ui/🧬️contract/⚖️compare/🧪️tests/⚖️compare/🦀️.rs` | `logicalDepth` reads the new const; page-index literal 255 → 433 |
| `🧰️framework/…/🖱️ui/🧬️contract/⚖️compare/🧫️fixture/🔣️.json` | `pageCount` 256 → 434; `littleEndian` `ff00…` → `b101…` |
| `🧰️framework/…/🖱️ui/🧬️contract/♻️retirement/🦀️.rs` | 3 × `UiArenaHandbacks<…, 4>` → `<…, UI_VALUE_HANDBACK_WORDS>` |
| `🧰️framework/…/🖱️ui/🧬️contract/♻️retirement/🧪️tests/♻️retirement/🦀️.rs` | same, 1 site |
| `🧰️framework/…/🖱️ui/🧬️contract/🧪️tests/🔬️action-unit/🦀️.rs` | page-domain law fills `UI_VALUE_AGGREGATE_ITEMS`, not `UI_VALUE_MAX_ITEMS`; arena census `[DEBUG]` line |
| `.🧬semio/…/PUZZLE-3D-END-TO-END/📓️2026-09-09-wave-O-outliner-and-arena.md` | **new** — this report |

Not touched: `✏️editor/🦀️.rs`, `📌️panels/🔍️inspection/🦀️.rs`, `⏳️precompute/**`, the mutation fixtures
(W-D), `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (peer fleet), `🗣️terminology/**` (W-G2),
`launch.json` / `🧩️launch.seed.jsonc` (no new executable command was introduced), any `🗑️generated` folder.

---

## 6️⃣ Verification

All foreground, private seeded target
`…/scratchpad/target-p3d`, `RUSTC_WRAPPER=""`.

### 6.1 New laws

`editor::puzzle3d::panels::document::tests`

- `the_outliner_pages_a_document_scale_fixture_without_exceeding_the_fixed_page` over
  `(180 objects × 2 vortices)`, `(1200 × 2)` and `(1200 × 0)`: `render` returns `Ok`; materialised
  interactive rows ≤ `UI_VALUE_PAGE_ROWS` and ≥ 1; **no node declares more than `UI_BUILT_CHILDREN_MAX`
  children**; object rows ≤ `SECTION_ROWS`; every materialised object row still has exactly 1 activation
  binding and exactly 2 row actions; at least one continuation row.
- `a_document_that_fits_the_page_keeps_every_nested_vortex_row` — 5 objects × 2 vortices materialises
  exactly 15 interactive rows and **zero** continuation rows, pinning that the reservation only bites
  under pressure.
- `the_outliner_renders_the_nakagin_example` — the real `nakagin_fixture()`, the case that used to fault.

`editor::puzzle3d::panels::catalogue::tests::the_catalogue_pages_an_over_wide_kind_catalog_without_exceeding_the_fixed_page`
— a 200-kind × 40-template catalog renders, every section stays inside `UI_BUILT_CHILDREN_MAX`, and the
truncated one carries its continuation row.

Every law that materialises a page takes `super::PANEL_PAGE_GUARD` (a `#[cfg(test)]` mutex beside
`page_rows`, in the same spirit as the `#[cfg(test)] thread_local!` build counters). Without it two page
laws running in parallel each observe a partly spent process-global arena and page shorter than their own
law expects — which is how the pre-existing
`kinds_tree_object_drag_data_carries_object_kind_and_mesh_url` was made to fail once during this wave; it
takes the guard now too (its only edit, one line).

### 6.2 Command tails

```
$ cargo check -p semio-framework-ui-contract --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 3.84s
    → 0 errors. 7 warnings, all pre-existing `unnecessary qualification` in peer-owned test files.

$ cargo test -p semio-framework-ui-contract -j 4 -- --test-threads=1 \
      --skip poisoned_arena_lock_recovers_without_losing_fixed_authority
test result: ok. 165 passed; 0 failed
test result: ok. 2 passed; 0 failed
    → the whole contract suite green at the new constants, including the compare fixture,
      the arena page-domain law and the 8 ms arena-init law. The one skipped test deliberately
      poisons the process-global arena mutex and fails every value-retirement test after it,
      at HEAD's constants exactly as at these (§7.7).

$ cargo test -p semio-framework-ui-runtime --lib -j 4 -- --test-threads=1 \
      --skip surface_ownership_resident_return_maintenance_preserves_contended_credit
test result: FAILED. 106 passed; 17 failed
    → byte-identical failure set to HEAD's 256-page arena, measured by reverting the two
      constants and re-running (§3.1). The skipped test deadlocks at HEAD (§7.6).

$ cargo check -p semio-framework-ui --tests / -p semio-framework-ui-scene --tests
    Finished — 0 errors each.

$ cargo check -p semio-framework-plugin --tests -j 4
    Finished — 0 errors, 17 pre-existing peer-owned warnings.

$ cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly -j 4
    Finished `dev` profile [unoptimized] target(s) in 1m 11s      → 0 errors
$ cargo check -p semio-s-artifact-puzzle-3d --features component-app-assembly --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 42.80s      → 0 errors
    → 5 warnings, every one in W-D's `✏️editor/🦀️.rs` and W-X's `🧪️tests/🔬️unit/🦀️.rs`; none in this
      wave's files. Both targets were transiently broken for ~40 min mid-wave by a peer's live
      `Puzzle3dConfig` refactor (191 → 135 → 59 → 0 errors, 0 of them ever in this wave's files).

$ cargo check -p semio-s-artifact-puzzle-5d --features component-app-assembly --tests -j 4
    Finished `dev` profile [unoptimized] target(s) in 1m 57s      → 0 errors

$ cargo test -p semio-s-artifact-puzzle-3d --features component-app-assembly --lib -- editor::puzzle3d::panels
[DEBUG] catalogue page kinds=200 objects-section-children=26
[DEBUG] outliner page objects=5 vortices=2 interactive=15
[DEBUG] outliner page objects=180 vortices=2 interactive=28 continuations=29 object-rows=28
[DEBUG] outliner page objects=1200 vortices=2 interactive=28 continuations=29 object-rows=28
[DEBUG] outliner page objects=1200 vortices=0 interactive=28 continuations=1 object-rows=28
[DEBUG] nakagin outliner objects=180 rows=62
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 569 filtered out
```

`bun ./📜️script.ts verify interactivity` still aborts at the same framework-owned
`interactivityLiveReconcileSelfTests` `per-surface-credit-cap` blocker W-G2 §4.1 attributed to peer commit
`de617a7c17` — byte-identical message, unchanged by this wave. The puzzle-owned verdict is reachable and
green via the ticket's own probe:

```
$ bun .🧬semio/…/🔍️probe-wave-G2-puzzle-interactivity-clauses.ts
PASS interactivityPuzzleFillEnvelopeSelfTests
PASS interactivityPuzzleFillP4eSelfTests
PASS interactivityPuzzleFillPreviewJsonSelfTests

$ bun ./📜️script.ts verify interactivity apps   →   grep -ci puzzle = 0
```

### 6.3 rustfmt

`rustfmt --edition 2021 --config-path rustfmt.toml --check` exit 0 on all ten changed Rust files.
(The repo's `rustfmt.toml` is edition 2021 / `max_width = 250` — the default `--edition 2024` reports
spurious diffs.)

---

## 7️⃣ NOT done / not verified

1. **No runtime/UI confirmation.** No app was booted. The page is proven by unit laws over
   `document::render`/`catalogue::render` and by the arena census — not by a click. In particular nobody has
   *seen* a `+149` continuation row in the shell.
2. **A refused row now ends its section instead of the render** (`paged_section`'s fail-soft arm). It
   catches every `ui.fixed-capacity` error, which is the right call for arena pressure but also swallows a
   genuine data fault of the same class — a label longer than `UI_TEXT_MAX_BYTES`, say — into "the section
   stopped here". The page stays honest (the `+N` row reports the omitted count) but the cause is not
   distinguishable from the outside. Splitting the arena's refusal from an author's own over-long value
   would need distinct error codes in the panel's helpers, which is a wider rename than this wave took.
3. **The continuation row is inert.** It reports the omitted count; it does not page forward. Real paging
   needs an outliner cursor in view-local config (`host_configuration_mutation` /
   `puzzle3d_config_store_mutation_bytes`, W-D's file) plus a `setOutlinerPage` action with its own
   interactive-job factory proof. Until then the outliner shows the document's first page only. This is the
   single biggest remaining gap in the feature.
4. **`page_rows()` reads live arena state, so a page built under pressure is shorter — and the memo caches
   it.** `document_tree_cached`'s key is (geometry fingerprint, label set); it does not include the realized
   row count, so a short page stays served until the fixture or the label set changes. Strictly better than
   the `Err` it replaces, but it is a wart, and closing it means adding the realized budget to
   `Puzzle3dDocumentTreeKey` in W-D's file.
5. **The retirement pump is the throughput bottleneck, not the arena size.** The plugin reactor drains the
   value arena at `close_ui_value_page_with_grant(1, 4096)` once per turn
   (`🔌️plugin/⚛️reactor/🔄️turn/🦀️.rs:174`) — with a 1,056-byte page slot that is ~3–4 pages per turn, while
   one outliner page rebuild admits up to 31 × 14 = 434. During a fixture-changing interaction (fill,
   brush) the arena will sit near-full and `page_rows()` will hand back short pages until it drains. Raising
   that grant is a framework interactivity-budget decision in the reactor's owner's file; **not attempted
   here**, and it is the honest next wave.
6. **`semio-framework-ui-runtime` is red at HEAD and this wave holds it exactly at that baseline** — 106
   passed / 17 failed, byte-identical failure set at the 256-page arena and at the shipped 434-page one
   (§3.1). Two independent pre-existing defects live in that crate and neither is touched here: (a)
   `reconcile::tests::ownership::surface_ownership_resident_return_maintenance_preserves_contended_credit`
   **deadlocks** — sampled with `sample(1)`, the thread is parked in
   `release_surface_reconcile_handback` → `Mutex<SurfaceReconcileHandbackRegistry>::lock`, a re-entrant
   blocking lock on a registry the test is deliberately holding contended, which hangs the whole harness and
   is why every measurement above skips it; (b) 17 reconcile/transaction laws fail with `Credits { … }`
   aggregate refusals and leaked resident reservations that cascade to the tests after them. Every one of
   them passes when run alone.
7. **`semio-framework-ui-contract`'s test suite is red at HEAD, independently of this wave.** Attribution
   experiment: with the constants temporarily reverted to `256` (and the compare fixture with them), the
   default parallel run still fails
   `action::binding_copy_tests::retained_binding_copy_cancel_and_arena_contention_keep_exact_aliases`
   (`assertion failed: reader.terminal_is_empty()`), and `--test-threads=1` still fails
   `document::document_component_compare_tests::retained_document_component_compare_cancel_and_contention_keep_live_document_and_incoming_root`
   with `"UI value retirement arena is poisoned"`. Cause of the latter, confirmed: the suite contains a test
   that *deliberately* poisons the process-global arena mutex
   (`poisoned_arena_lock_recovers_without_losing_fixed_authority`), and `close_ui_value_page_with_grant`
   fails closed on a poisoned lock — so every value-retirement test scheduled after it errs. The failing
   *set* varies run to run, which is the signature of a race over one process-global arena rather than of a
   capacity change. The deterministic protocol
   (`--test-threads=1 --skip poisoned_arena_lock_recovers_without_losing_fixed_authority`) is **fully green
   at the new constants**: 165 + 2 passed, 0 failed. Isolating that poisoning test into its own process (or
   making the retirement pump recover a poisoned lock the way `with_ui_value_arena` already does) is a
   framework-owned repair this wave did not take on.
8. **`verify interactivity` still aborts before its own verdict**, at the framework-owned
   `interactivityLiveReconcileSelfTests` `per-surface-credit-cap` blocker W-G2 §4.1 attributed to peer commit
   `de617a7c17`. Unchanged by this wave and not repaired here.
9. **The inspection `ids`-list hazard** (§4.2) is reported, not fixed — W-D owns that file.
10. **`editor::puzzle3d::component::tests` is still unrunnable at HEAD**, exactly as W-G2 §5.2 recorded: a
   single test overflows its stack, the abort is non-unwinding, and the whole process dies, so cargo marks
   every in-flight test in that module FAILED. Running one of them under `RUST_MIN_STACK=128M` gets past the
   overflow and surfaces the real assertion, which is **not** this wave's:
   `document_and_kinds_trees_use_german_reuse_section_labels` fails at
   `assert!(document_json.contains("Baukomponenten"))` because `testkit::render_body` renders with
   `ViewModel::default()` — `Locale::En` / `Terminology::Native`, both `#[default]` — while the assertion
   expects the `reuse_de` cell. Since W-G2 made `puzzle3d_labels` resolve purely from
   `view_state.locale`/`view_state.terminology`, an English/native render can only produce `"Objects"`.
   Paging cannot change which language a section label resolves to; the fix is for that test to render with
   a de/reuse `ViewModel`, and it belongs to the label-axis owner. The `panels` module's own five tests all
   pass in isolation.
11. **The page is as large as the resident ceiling allows, not as large as the UI would like.** 31 rows is
   the largest measured size that keeps `semio-framework-ui-runtime` at its baseline (§3.1). Making it
   larger honestly requires one of: shrinking `UiPageSlot` (1,056 bytes, dominated by two inline 512-byte
   `UiText`s — a page carrying four short strings costs a kilobyte), cheaper row arguments (the
   `setSelectionFlag` map + nested single-id list is 10 of a row's 14 pages), or lifting the authored
   `pressureRoots * pressureReservationBytes == pressureAggregateBytes` law so the aggregate ceiling can
   exceed four surfaces. All three are framework-owned and none was attempted here.
