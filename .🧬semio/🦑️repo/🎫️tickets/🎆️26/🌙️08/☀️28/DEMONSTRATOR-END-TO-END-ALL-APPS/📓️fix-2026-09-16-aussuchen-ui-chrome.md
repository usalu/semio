# Aussuchen (`s.sourcing.curation@1/*#editor`) — UI chrome audit verification + fix

Date: 2026-09-16. Repo root `/Users/ueli/Documents/semio`.
Crate: `semio-s-artifact-sourcing-curation`
(`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/📦️packages/🦀️rust/Cargo.toml:2`).

All paths below are relative to the repo root; the subset prefix
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/` is abbreviated `…/`.

## 0. Path drift — the 08-28 audit no longer addresses real files

`📓️app-aussuchen.md` cites `🗿️artifacts/🗂️curate/…/✏️editor/🦀️component.rs`. Neither the artifact
folder nor the file name exists any more:

| audit path / name | current |
|---|---|
| `🗿️artifacts/🗂️curate/` | `🗿️artifacts/🗂️curation/` |
| `…/✏️editor/🦀️component.rs` | `…/✏️editor/🦀️.rs` |
| `🎮️commands/📄️set-active-example/` | `🎮️commands/🎬️set-active-example/` |
| `🎮️commands/🧺️curate-add/` | `🎮️commands/➕️curation-add/` |
| `🎮️commands/🔍️sort-table/` | `🎮️commands/↕️sort-table/` |
| `CurateSnapshot` / `SourcingCurateConfig` / `curateAdd` | `CurationSnapshot` / `SourcingCurationConfig` / `curationAdd` |

## 1. Findings — the audit's §4/§7 chrome claims are STALE

Verified against the tree as of 2026-09-16, **before** any edit in this pass:

| audit claim (08-28) | current reality | evidence |
|---|---|---|
| "`sourcing_action()` … defined but never called anywhere in the plugin" | **FALSE.** Called 4× in the pool filter bar. | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs:89,95,104,112` |
| "`SOURCING_DRAG_MIME` … declared and never referenced again" | **FALSE.** Set on every sourcing table scene. | `…/✏️editor/🦀️.rs:100` — `scene.row_drag_mime = Some(SOURCING_DRAG_MIME.into())` |
| "No search box … anywhere in the pool/curated/grid render code" | **FALSE.** `filter_bar()` builds a text input, one toggle per module, a typology `select` and a min-availability number input, each bound to its own command. | `…/🏊️pool/🦀️.rs:84-116`, mounted at `:129` |
| "no sortable column headers" | **FALSE.** Pool declares `sortable: true` on name/module/availability; the renderer turns a sortable header click into `action: "sortTable"`. | `…/🏊️pool/🦀️.rs:119-125`; `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/📊️Table/🟦️.tsx:202-209` |
| "no drag source/drop target" | **FALSE.** Every row carries `_drag: {objectId}`; `drop_action_json` is `dropOnPool`/`dropOnCurated`; the React host wires `dataTransfer.setData(rowDragMime, …)` and the matching drop handler. | `…/✏️editor/🦀️.rs:83-106`; `📊️Table/🟦️.tsx:167-188,214-218` |
| "`TableView` cells are plain strings with zero row actions" | **FALSE.** The windows use `TableScene` + typed `TableCell` (`Text`/`Number`/`Stepper`/`Buttons`); Curated already had a per-row `trash-2` remove button. | `…/🧺️curated/🦀️.rs:45-53` |
| "Preview permanently stuck on 'No selection'" | **FIXED by a peer.** `render_with_request_context` reads `interaction.selection(SOURCING_ROWS_DOMAIN)` and passes the ids to `preview::render`. | `…/✏️editor/🦀️.rs:1042-1055` — untouched by this pass |
| §2 `demo-stock` is real | **STILL TRUE** (oracle moved to `…/✏️editor/🧫️fixtures/📦️expected-stock.json`). | `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs:109` |

**Net: the audit's headline — "fully implemented commands with ZERO UI chrome" — is wrong for the
current tree.** Almost all of the chrome landed between 08-28 and today.

### 1b. Real gaps that were still open (all fixed in §2)

1. **Pool had no curate/uncurate row action.** Curating from the Pool was only possible by nudging the
   `curated` stepper or by drag-and-drop.
2. **Curated had no sortable headers and ignored the sort state** — `sourcing_table(..., None)` at
   `…/🧺️curated/🦀️.rs:57`, all four columns `sortable: false` at `:39`, and `curated::render` did not
   even receive `&SourcingCurationConfig`.
3. **Pool's sort fell through to name for any unknown column** (`_ => a.name.cmp(&b.name)`,
   `…/🏊️pool/🦀️.rs:47`), so sorting Curated by its own `count` column would silently re-sort the Pool.
4. **`stockFromCatalogue` had no chrome** — palette-only (`…/✏️editor/🦀️.rs:1177` `.mutation(...)`).

### 1c. 🚨 BLOCKING FINDING — `setContributions` is hard-dead at runtime

This is the answer to "does the Pool list rows from contributed modules when contributions arrive?"
**It cannot, ever, in the running app.**

- `…/✏️editor/🦀️.rs:388` — `const SOURCING_CURATION_CONFIG_TEXT_BYTES: usize = 96;`
- `…/✏️editor/🦀️.rs:425` — the retained-config footprint prices the whole contribution blob against it:
  ```rust
  SourcingCurationConfigMutation::SetFilterQuery { value } | SourcingCurationConfigMutation::SetContributions { json: value } => (1, value.len()),
  ```
- `…/✏️editor/🦀️.rs:437` — `if retained_bytes > SOURCING_CURATION_CONFIG_TEXT_BYTES { return Err(...) }`
- `…/✏️editor/🦀️.rs:414-415` — and `contributions_json.len()` is *also* charged against the same 96-byte
  envelope for the config **base**, so even a config that somehow held a contribution would refuse
  every subsequent filter/sort edit.

A bare `ProgramContributionEntry` envelope for one `sourcing.module` topic blows 96 bytes before any
payload (the `topic` string is 16 bytes and the `appId` value is 17). The minimal realistic
contribution in my fixture is **~700 bytes**. So `setContributions` is silently refused for every
possible real input — confirmed empirically: dispatching it through the live app left
`config.contributions_json` at `"[]"` and the subsequent restock produced only the 10 authored kinds.

I did **not** raise the envelope. Doing so is not a UI-chrome change: the surrounding budget
(`SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES = 768`, asserted as `* 4 + 1_024 == 4_096` in
`retained_config_preparation_matches_the_json_oracle_and_rejects_maximum_plus_one`) and the
`bounded_first_step(8_192, 64, 1, 16_384, 7_500)` tool grant would all have to move together. That is
a bounded-memory-contract decision for the coordinator. Instead I pinned the ceiling with a failing-
the-day-it-changes gate test (§2, test 8) so it cannot stay wrong silently.

The three `sourcing-module-{beams,slabs,windows}` extensions contribute the *same* kinds the schema's
own `beams`/`windows`/`slabs` modules already provide (`…/🧬️schema/🦀️.rs:713-716`), so the
demonstrator's visible behaviour is unaffected today — the hot-swap path is what is dead.

## 2. Edits

### `…/✏️editor/🗣️terminology/🦀️.rs`
- `:23-25` — three new en/de label rows: `col_actions` ("Actions"/"Aktionen"), `curate`
  ("Curate"/"Kuratieren"), `restock` ("Restock From Catalogue"/"Bestand aus Katalog").

### `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🏊️pool/🦀️.rs`
- `:9` — import `UiTreeActionPlacement`, `UiTreeItemAction`.
- `:40-42` — new `SOURCING_CURATION_POOL_SORT_COLUMNS` (`name`/`module`/`availability`).
- `:44-56` — `pool_kinds` now applies the shared sort **only** for its own columns; a foreign column
  leaves document order instead of falling through to name.
- `:58-67` — new `pool_row_actions`: `curationAdd` (icon `plus`) always, plus `curationRemove`
  (icon `minus`) once `curated_count > 0`. Never more than two.
- `:69-81` — `pool_row` gained the `actions` cell and a `labels` parameter.
- `:126-128` — restock button in the filter bar: `ui::button(labels.restock)` icon `refresh-cw`,
  id `sourcing-pool-restock`, `Trigger::Activate` → `stockFromCatalogue`.
- `:138-141` — `render` declares the `actions` column and threads `labels` into each row.

### `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧺️curated/🦀️.rs`
- `:3-6` — imports `SourcingCurationConfig`, `CuratedItem`, `ObjectKind`, `SortDirection`.
- `:37-40` — new `SOURCING_CURATION_CURATED_SORT_COLUMNS` (`name`/`availability`/`count`).
- `:42-57` — new `curated_rows(document, cfg)`: resolves each `CuratedItem` against stock and applies
  the shared `Filters::sort` when it names one of this table's own columns.
- `:59-69` — new `curated_row_actions`: `curationAdd` while `count < availability`, then
  `curationRemove`. Never more than two.
- `:71-87` — `render` takes `&SourcingCurationConfig`, declares name/availability/count as
  `sortable: true`, labels the `actions` column, and passes `cfg.filters.sort` into `sourcing_table`
  so the renderer draws the sort indicator on the right header.

### `…/✏️editor/🦀️.rs`
- `:1033` — the one-line call-site change `curated::render(snapshot, labels)` →
  `curated::render(snapshot, config, labels)`. **Nothing else in this file was touched** — in
  particular `render_with_request_context`'s preview selection plumbing (`:1042-1055`) is untouched.

### `…/✏️editor/🧪️tests/🔬️unit/🦀️.rs` (shared test context)
- `:101-122` — three new helpers in `pub(crate) mod context`: `table_scene_of` (recursive surface
  finder), `table_of` (renders a body and decodes its column ids + row records), `row_ids`.

### `…/✏️editor/🦀️.rs` (visibility only, for the §1c gate test)
- `:388` — `const SOURCING_CURATION_CONFIG_TEXT_BYTES` → `pub(crate)` (+ docstring).
- `:424` — `fn sourcing_curation_config_mutation_footprint` → `pub(crate)`.
  Both are reached from the window test modules as `crate::editor::sourcing::component::…` — the
  windows live under `editor::sourcing::modes::…`, siblings of `component`, so module-private was not
  visible to them.

## 3. Tests

Both window test files were extended. **13 new tests**, all through the real commands — no new state
model, no bypassed dispatch.

`…/🏊️pool/🧪️tests/🔬️unit/🦀️.rs`
1. `pool_rows_offer_curate_and_only_offer_uncurate_once_curated` — the ≤2 row-action budget, and the
   uncurate button appearing only once curated.
2. `pool_sort_applies_only_to_its_own_columns` — foreign column keeps document order; `name`/`desc`
   really reverses.
3. `the_filter_bar_binds_each_control_to_its_own_command` — query, typology, min-availability, restock
   and one toggle per module are all present by id.
4. `dispatching_the_search_box_narrows_the_rendered_pool` — **live app**: `setFilterQuery` shrinks the
   rendered row set, and every surviving row was already there.
5. `dispatching_a_column_header_reorders_the_rendered_pool` — **live app**: `sortTable` reorders the
   rendered rows without adding or dropping any.
6. `the_curate_row_action_moves_a_row_into_the_curated_table` — **live app**: `curationAdd` puts the
   row in the Curated body, bumps the Pool stepper to 1, and grows that row to two actions.
7. `a_contributed_module_reaches_the_filter_bar_and_then_the_pool_rows` — a fake `sourcing.module`
   `ProgramContributionEntry` gains its own filter toggle, and the **production**
   `stock_from_catalogue::handle` merges its kind into the document so `pool_kinds` lists it.
8. `set_contributions_is_refused_by_its_retained_config_envelope` — 🚨 the §1c gate.
9. `the_restock_button_publishes_a_merged_catalogue_document` — **live app**: the restock button's
   command publishes a `LoadDocument` and is idempotent over the authored demo stock.
   (existing `pool_row_carries_the_drag_payload…` and `pool_scene_names_columns_by_id…` extended for
   the new `actions` column, `sortable` flag and restock control.)

`…/🧺️curated/🧪️tests/🔬️unit/🦀️.rs`
10. `curated_columns_are_sortable_and_echo_the_active_sort` — three sortable headers, the `actions`
    column, `sort_json` reaching the renderer, drag mime and `dropOnCurated` drop target.
11. `curated_rows_follow_the_shared_sort_only_for_their_own_columns` — `count` asc/desc really
    reorders; the pool's `module` column does not.
12. `curated_rows_offer_curate_and_uncurate_within_the_two_action_budget` — and the curate button
    disappears at saturation.
13. `dispatching_curate_sort_and_uncurate_changes_the_rendered_curated_table` — **live app**: two
    curates → both rows; `sortTable count desc` → larger count first; `curationRemove` → one row left.

### Counts

| | tests | passed | failed | ignored |
|---|---|---|---|---|
| before (baseline, 10:0x) | 129 | 126 | 2 | 1 |
| after | **142** | **141** | **0** | 1 |

+13 tests, suite fully green.

The 2 baseline failures —
`example_load_settles_through_the_host_document_archive_door` and
`a_saved_curation_archive_carries_its_catalog_member_and_reloads_with_its_edits`, both
`plugin.internal: document archive replacement sealed its member roster before the archived members
were admitted` — were **not** mine and are **no longer failing**. A peer landed a change to
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` (mtime 12:30:52) during this pass that fixed that
`document_archive` seam. The same peer change briefly broke `editor_surface`'s
`From<VcsArtifactApp<EditorApp<E>, E::Members>>` bound repo-wide (seen at `…/🪆️subsets/✳️any/🦀️.rs:33`);
that resolved on its own before my final run. Worth knowing when reading any log timestamped in
between.

## 4. Commands

```sh
cd /Users/ueli/Documents/semio
DEVELOPER_DIR=/Library/Developer/CommandLineTools RUSTFLAGS=-Awarnings CARGO_TERM_QUIET=true \
  CARGO_PROFILE_WASM_DEV_DEBUG=false CARGO_BUILD_JOBS=4 \
  cargo test -p semio-s-artifact-sourcing-curation --lib                    # 141 passed, 0 failed, 1 ignored
DEVELOPER_DIR=… cargo check --target wasm32-wasip2 -p semio-s-artifact-sourcing-curation   # exit 0
DEVELOPER_DIR=… cargo check --target wasm32-wasip2 -p semio-s-plugin-sourcing              # exit 0
```

⚠️ `--features component-app-assembly` does **not** apply here: neither
`semio-s-artifact-sourcing-curation` nor `semio-s-plugin-sourcing` declares that feature (cargo lists
the ~54 packages that do; sourcing is not among them). The bare wasm check is the right gate for this
crate.

Logs (under `🗑️generated/`): `aussuchen-baseline-test.txt`, `aussuchen-build-1.txt`,
`aussuchen-build-2.txt`, `aussuchen-after-test.txt`, `aussuchen-wasm-check.txt`.

## 5. What remains for the coordinator's browser pass on :6029

1. **Pool window** — confirm the filter row renders above the table: search box, three module toggles,
   typology dropdown, min-availability number field, and the new **Restock From Catalogue** button.
   Type in the search box and confirm rows narrow live.
2. **Sortable headers** — click Name / Module / Availability in the Pool and Name / Availability /
   Count in Curated; the rows must reorder and the header must show the direction indicator.
3. **Row actions** — each Pool row should show a `plus` (Curate) button, gaining a `minus` once
   curated; each Curated row a `plus` and a `trash-2`. Verify the icon set actually resolves `plus`,
   `minus`, `refresh-cw` and `recycle` in the OS icon registry — I only asserted the ids, not that the
   renderer has glyphs for them.
4. **Drag and drop** — drag a Pool row onto Curated and back. This is the one path unit tests cannot
   cover: `row_drag_mime`/`drop_action_json` are asserted on the scene, but the actual
   `dataTransfer` round-trip is React-side (`📊️Table/🟦️.tsx:167-188,214-218`).
5. **Preview** — a peer landed the selection→render threading; clicking a Pool row should now show a
   mesh instead of "No selection". Not exercised by this pass.
6. 🚨 **Decide on §1c.** If the demonstrator is meant to hot-swap `sourcing.module` contributions,
   `SOURCING_CURATION_CONFIG_TEXT_BYTES` (96) and very likely
   `SOURCING_CURATION_CONFIG_STORE_MAXIMUM_BYTES` (768) plus the `bounded_first_step` grant must be
   raised together, and `set_contributions_is_refused_by_its_retained_config_envelope` deleted. Until
   then contributed modules can never become Pool rows in the running app.
