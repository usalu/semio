# 📓️ A6b — process3d + block 2d/3d/5d, windowed-tree migration finished

Packet A6b of ARTIFACT-TREE-VIRTUALISED-STREAMING, resuming the dead A6 agent. Crates:

| Crate | Path (crate root `🦀️.rs` under) | Feature flag |
|---|---|---|
| `semio-s-artifact-process-process3d` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d` | none (`ui-contract` non-optional) |
| `semio-s-artifact-block-2d` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻️2d` | `component-app-assembly` |
| `semio-s-artifact-block-3d` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d` | `component-app-assembly` |
| `semio-s-artifact-block-5d` | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d` | `component-app-assembly` |

## 1. What A6 had already landed (verified on disk, not taken on trust)

Every `PanelTreeBuilder` call site in all four crates already used `window_section` /
`window_section_or_placeholder` and took `&TreeWindows<'_>` **except the three block inspectors**:

- process3d `📌️panels/{🗿️artifact,🔍️inspection,🛍️catalogue,🛠️workshop}` — migrated, `TreeWindows::for_body`
  built once in `✏️editor/🦀️.rs:1342` and threaded into all four bodies.
- block 2d/3d/5d `📌️panels/🗿️artifact` — migrated, with the full (a)–(d) law set already present.

## 2. What I changed

### 2.1 Block inspector panels migrated off `.section(…, UiFixedList)` (the last app-local list build)

| File | Change |
|---|---|
| `…/🧱️block/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📌️panels/🔍️inspection/🦀️.rs` | new `pub const BLOCK2D_INSPECTOR_SUMMARY`; new private `enum InspectorField<'a> { Text, Readonly }` + `inspector_row`; `render(.., windows: &TreeWindows<'_>)` builds the 5 fields as DATA and calls `.window_section(windows, BLOCK2D_INSPECTOR_SUMMARY, …, &fields, inspector_row)`; `UiFixedList` import dropped, `TreeWindows` added |
| `…/🧱️block/🗿️artifacts/🧊️3d/…/📌️panels/🔍️inspection/🦀️.rs` | same shape; `enum InspectorField { Text, Representation, Readonly }` (4 fields); `render(.., active_representation_id, labels, windows)`; the crate-local `semio_framework_plugin::ui_node_list([...])` build is gone |
| `…/🧱️block/🗿️artifacts/🖐️5d/…/📌️panels/🔍️inspection/🦀️.rs` | same shape; `enum InspectorField { Text, Readonly }` (3 fields); `ui_node_list` build gone |
| `…/◻️2d/…/✏️editor/🦀️.rs:527`, `…/🧊️3d/…/✏️editor/🦀️.rs:979`, `…/🖐️5d/…/✏️editor/🦀️.rs:500` | each `inspection_panel::render(…)` arm now passes `&TreeWindows::for_body(view_state, <BODY_KEY>)` |

The rows are recorded as borrowed data (`&'a str` / `String`) and only materialised inside the window's
`row` closure, so a field a closed or off-viewport section will not show is never built.

### 2.2 Window laws added (design §8 (a)–(d), per 📓️wave2-app-brief §4)

New `//#region 🪟️WindowLaws` blocks, each with the shared `container_of` / `project` /
`window_view` helpers used by the already-migrated artifact panels
(`project_and_retire_fixture_tree`, never `serde_json::to_string` on a `BuiltNode`):

| Test file | Laws |
|---|---|
| `…/◻️2d/…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | `the_inspector_summary_stamps_its_total_and_publishes_no_continuation_row` (a), `a_closed_inspector_summary_stamps_its_total_with_no_rows` (b), `an_inspector_window_request_materialises_exactly_its_slice` (c), `the_inspector_tree_binds_no_interaction_domain_and_its_rows_keep_their_controls` (d) |
| `…/🧊️3d/…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | same four |
| `…/🖐️5d/…/📌️panels/🔍️inspection/🧪️tests/🔬️unit/🦀️.rs` | same four |
| `…/🧊️process3d/…/📌️panels/🛍️catalogue/🧪️tests/🔬️unit/🦀️.rs` | `an_oversized_catalogue_stamps_its_full_capability_count` (a), `a_closed_catalogue_section_stamps_its_total_with_no_rows` (b, both the author-closed stock section and a host-closed request), `a_catalogue_window_request_materialises_exactly_its_slice` (c), `catalogue_rows_are_action_rows_not_domain_pick_targets` (d) |
| `…/🧊️process3d/…/📌️panels/🛠️workshop/🧪️tests/🔬️unit/🦀️.rs` | `an_oversized_workshop_stamps_its_full_machine_count` (a), `a_closed_workshop_section_stamps_its_total_with_no_rows` (b), `a_workshop_window_request_materialises_exactly_its_slice` (c), `workshop_machine_rows_declare_their_granularity_and_carry_no_activate_binding` (d) |

Law (d) is honest about the surface it is on:
- **Workshop** is the real domain case — the machines rows carry `granularity` + no `Activate`
  binding of their own (the remove verb stays a `RowActionPlacement::Menu` row action) and the tree
  carries exactly one `interactionSelect` for `PROCESS3D_INTERACTION_DOMAIN`.
- **Catalogue** rows are install/add ACTIONS, not domain targets (the panel deliberately declares no
  `interaction_domain`); the law pins exactly that: no `interactionDomain`, no tree-level binding,
  every row keeps its own `addStep` binding and declares no pick granularity.
- **Block inspectors** are FORMS: the law pins no `interactionDomain`, no tree-level
  `interactionSelect`, and every field nesting its own control. The domain-pick form of (d) is
  already pinned on each app's `📌️panels/🗿️artifact`.

Fixtures used: 80 capabilities on one uncataloged machine (catalogue), 60 installed machines
(workshop), the real field lists (inspectors — 5/4/3 rows, so (a) asserts the exact count and (c)
asserts an exact 2-row slice by key).

### 2.3 Node-key uniqueness (F2's new loud duplicate-`node_key` SDK error)

Checked every container id each body can emit:
- block 2d/3d/5d inspector: one section id per body.
- process3d catalogue: `process3d-play-catalogue.workshop`, `process3d-play-catalogue.stock`,
  `process3d-play-catalogue.{catalog_id}` — the live catalog ids are `geometry`/`metal`/`wood`/
  `robotic`/`concrete` (no collision with `workshop`/`stock`), and `catalog_sections` dedupes.
- process3d workshop: `process3d-play-workshop.machines` + `process3d-play-workshop.catalog.{id}`,
  deduped by `installed_catalogs` (already pinned by `the_real_extension_pack_renders_each_catalog_section_once`).
- process3d inspector: the selection branches are mutually exclusive; `render_machine` emits
  `…machine` + one `…capability.{capability.id}` per capability, unique within a machine.
- Artifact panels: two distinct section ids each.

## 3. Sweeps — nothing left to delete

Repo-grep over `✏️s/🔌️plugins/🧱️block` and `✏️s/🔌️plugins/🏭️process` (source only, `/dist/` rmeta
excluded), `*.rs`/`*.ts`/`*.json`:
- `setPanelPage` / `set-panel-page` / `panel_pages` / `panelPage` / `PanelPage` — **0 hits** (command
  dirs, enums, dispatch, publication contracts, `toolIds`, config structs, `🧬️schema/*`, fixtures).
- `paged_panel_section` / `panel_continuation_row` / `panel_page_rows` / `PanelRowBudget` /
  `SECTION_ROWS` / `IDS_ROWS` / `LIST_ROWS_MAX` / `CATALOGUE_GROUP_ROWS` /
  `PANEL_RECONCILE_NODE_BUDGET` — **0 hits**.
- crate-local `fn ui_node_list` — **0 hits**; the one remaining `ui_node_list` call
  (`◻️2d/…/🎭️modes/✏️edit/🪟️windows/📋️board/🦀️.rs:49`) is the SDK's and is a canvas window, not a tree.
- `.more` / `"+…"` labels / `…more` / `.take(N)` list truncation in panel code — **0 hits**; every
  remaining match is a doc comment or one of the new law assertions.
- No `[[test]]` lanes in any of the four `Cargo.toml`s — `--lib` is the whole suite.

## 4. Verification (all foreground, shared build dir, no `CARGO_TARGET_DIR`; `DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0 CARGO_PROFILE_WASM_DEV_DEBUG=false`)

Logs under `🗑️generated/a6b/`.

| Command | Result |
|---|---|
| `cargo check -p …process-process3d -p …block-2d -p …block-3d -p …block-5d --features <block ×3>/component-app-assembly --keep-going` | ✅ `Finished dev profile in 1m 51s`, 0 errors (pre-change baseline) |
| `cargo test -p semio-s-artifact-process-process3d --lib -- --skip vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` | **baseline (pre-change): 315 passed / 31 failed / 3 ignored** — `baseline-process3d-test.txt` |
| `cargo test -p semio-s-artifact-process-process3d --lib --no-fail-fast -- --skip vcs_…fail_closed` | **323 passed / 31 failed / 3 ignored** — `run1-process3d-test.txt`. +8 = the 8 new catalogue/workshop laws; failure NAME SET byte-identical to baseline (`diff` empty) |
| `cargo test -p semio-s-artifact-block-2d --features component-app-assembly --lib --no-fail-fast` | **212 passed / 49 failed** — `run1-block-2d-test.txt`. Stale A6 log: 208 / 49. +4 = the 4 new inspector laws; failure set identical |
| `cargo test -p semio-s-artifact-block-3d --features component-app-assembly --lib --no-fail-fast` | **250 passed / 99 failed** — `run1-block-3d-test.txt`. Stale A6 log: 246 / 99. +4; failure set identical |
| `cargo test -p semio-s-artifact-block-5d --features component-app-assembly --lib --no-fail-fast` | **290 passed / 75 failed** — `run1-block-5d-test.txt`. Stale A6 log: 286 / 75. +4; failure set identical |
| `cargo check -p semio-s-artifact-process-process3d --target wasm32-wasip2` | ✅ `Finished dev profile in 1m 07s`, 0 errors — `wasm-process3d.txt` |
| `cargo check -p semio-s-artifact-block-2d --features component-app-assembly --target wasm32-wasip2` | ✅ `Finished dev profile in 4.28s` — `wasm-block-2d.txt` |
| `cargo check -p semio-s-artifact-block-3d --features component-app-assembly --target wasm32-wasip2` | ✅ `Finished dev profile in 9.53s` — `wasm-block-3d.txt` |
| `cargo check -p semio-s-artifact-block-5d --features component-app-assembly --target wasm32-wasip2` | ✅ `Finished dev profile in 5.70s` — `wasm-block-5d.txt` |

**All 20 window laws across the four crates pass** (4 artifact ×4 crates were already green; the 20
are: 4 block-2d inspector + 4 block-3d inspector + 4 block-5d inspector + 4 process3d catalogue +
4 process3d workshop, all newly added here), plus process3d's pre-existing
`a_capability_section_stamps_its_total_closed_and_materialises_it_open` (inspector (b)/(c)).

No new compiler warnings in any file this packet touched.

## 5. Triage of the remaining failures — all pre-existing, none this ticket's

The failure NAME SETS before and after my change are identical in every crate (`diff` of the
`failures:` blocks is empty except for cargo's own `--no-fail-fast` "1 target failed" line). None of
them touches a tree, a panel container, a window, `setPanelPage`, `UI_BUILT_CHILDREN_MAX` or
`UI_VALUE_PAGE_ROWS`. They fall into four peer-owned framework faults:

1. **`edit history insertion requires its exact mutation retirement factory`** —
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15873`
   (`reserve_edit_history_slot` refuses a store with `mutation_retirement_factory == None`). That
   file was last changed `df7a4916bd 2026-09-16 23:33:58`, by the concurrent store/retirement work,
   not by this ticket (which touches the plugin panel kit, the UI contract and the OS host). This is
   what makes every process3d `editor::process3d::wasm::tests::*` and every
   `!result.mutations.is_empty()` dispatch assertion fail — including the six
   `panels::inspection::tests::{add_step_*, measure_arg_routes_*, selected_*}` and three
   `panels::workshop::tests::{add_workshop_machine_action_installs,
   catalogue_reflects_workshop_after_machine_removal, remove_workshop_machine_action_removes_the_machine}`.
   The `selected_*` inspector tests fail *downstream* of this: their `interactionSelect` never lands,
   so `render` correctly falls back to the empty state — the panel itself is fine, which is exactly
   what the neighbouring `a_capability_section_stamps_its_total_closed_and_materialises_it_open`
   (same panel, explicit selection, no dispatch) proves by passing.
2. **`artifact store reached Drop without its exact terminal-empty shallow-shell witness`** —
   `🧰️framework/…/🏪️store/🦀️.rs:18451`. This is what fails `renders_inspector_fields` in all three
   block crates. Proof it is not mine: the STALE A6 log
   (`🗑️generated/a6/block-2d-test.txt:1025`) shows the identical panic at the identical
   `store/🦀️.rs:18451` for the identical test, before the inspector was migrated.
3. **`interactive-job.live-instance`: "typed command '…' does not belong to the mounted live app
   instance"** — the block component/dispatch unit tests.
4. **Float canonical-form / JSON round-trip drift** — the large `committed_json_is_canonical`,
   `committed_diff_is_canonical` and `json_round_trips_every_example` families in block 2d/3d/5d
   (e.g. `-0.4283989982167899` vs `-0.42839899821678995`) and process3d's two
   `binary::retained_laws`. Pure schema/fixture serialisation, no UI.

Also confirmed pre-existing and skipped per the packet brief: process3d's
`vcs_artifact_app_production_maintenance_swap_is_authoritative_and_fail_closed` hangs forever
(run with `-- --skip` for it).

## 6. Not finished / caveats

- The four crates are **not green overall** and cannot be made green from this packet: the four
  faults in §5 are framework/schema-owned. Everything this packet owns (window laws, panel
  compilation, wasm) is green.
- I did **not** take a fresh pre-change `cargo test` baseline for the three block crates: the fleet
  had ~41 concurrent cargos queued on the shared `target/debug/.cargo-artifact-lock` and one run sat
  blocked for 51 min without acquiring it, so I killed my own queued run and compared against A6's
  stale logs (`🗑️generated/a6/block-*-test.txt`) instead. That comparison is sound for triage: the
  failure name sets match exactly and the pass counts differ by exactly the number of laws I added.
  The process3d baseline **is** fresh and mine (`baseline-process3d-test.txt`).
- No TS/schema/fixture regeneration was needed — these plugins never had `setPanelPage` or
  `panel_pages` in any generated artefact.
