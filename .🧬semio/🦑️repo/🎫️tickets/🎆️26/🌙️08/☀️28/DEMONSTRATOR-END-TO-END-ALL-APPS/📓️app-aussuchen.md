# Aussuchen (sourcing.curate@1) — fixture, surface binding & interactivity audit

Repo root: `/Users/ueli/Documents/semio`. All paths below are relative to it unless absolute.

## 1. Editor component, mode, and default windows

Main editor dispatch/manifest file:
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`

Mode/layout file (single mode `curate`):
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/✏️edit/🦀️component.rs`

```rust
// 🦀️component.rs:1-2 (mode)
//! 🗂️ Sourcing curate app — the `curate` mode: the three-column pool/curated+preview/grid workspace.
//! Sourcing has exactly one mode, so this is also the app's `default_mode_id`/`default_layout`.
```
```rust
// layout(), lines 24-38
pub fn layout() -> WindowLayout {
    WindowLayout { root: WindowLayoutRoot::Axis(WindowLayoutAxisNode { kind: "row".into(), size: None, children: vec![
        WindowLayoutChild::Axis(... size: Some(0.34), children: vec![sourcing_stack(pool::SOURCING_CURATE_WINDOW_POOL, "Pool", None)] }),
        WindowLayoutChild::Axis(... size: Some(0.33), children: vec![
            sourcing_stack(curated::SOURCING_CURATE_WINDOW_CURATED, "Curated", Some(0.55)),
            sourcing_stack(preview::SOURCING_CURATE_WINDOW_PREVIEW, "Preview", Some(0.45))] }),
        WindowLayoutChild::Axis(... size: Some(0.33), children: vec![sourcing_stack(grid::SOURCING_CURATE_WINDOW_GRID, "Grid", None)] }),
    ]}) }
}
```
A `#[cfg(test)]` test (`the_default_layout_lists_every_window`, lines 48-54) asserts the JSON contains all four window kind ids.

**Confirmed: the app opens exactly four windows by default**, three-column layout (pool | curated-over-preview | grid):

| window_kind_id | body_key (`SurfaceKind`) | title | file:line |
|---|---|---|---|
| `sourcing-pool` | `sourcing.pool` (`SurfaceKind::Table`) | Pool | `.../🪟️windows/🏊️pool/🦀️component.rs:11-33` |
| `sourcing-curated` | `sourcing.curated` (`SurfaceKind::Table`) | Curated | `.../🪟️windows/🧺️curated/🦀️component.rs:9-31` |
| `sourcing-preview` | `sourcing.preview` (`SurfaceKind::World3d`) | Preview | `.../🪟️windows/👁️preview/🦀️component.rs:11-33` |
| `sourcing-grid` | `sourcing.grid` (`SurfaceKind::World3d`) | Grid | `.../🪟️windows/🔢️grid/🦀️component.rs:12-35` |

Note: the ticket's reported window ids (`sourcing.pool` etc.) are actually the **body keys**; the real `window_kind_id`s (used in the layout tree and `s/plugin/🎪️demonstrator/🔣️descriptor.json:44789-44833`) are the dash-form `sourcing-pool`/`sourcing-curated`/`sourcing-preview`/`sourcing-grid`. Both id spellings appear correctly in the generated descriptor (`🔣️descriptor.json:36353-42601`).

Render dispatch (editor `🦀️component.rs:738-753`):
```rust
match body_key {
    pool::SOURCING_CURATE_BODY_POOL => pool::render(snapshot, config, labels)...,
    curated::SOURCING_CURATE_BODY_CURATED => curated::render(snapshot, labels)...,
    preview::SOURCING_CURATE_BODY_PREVIEW => preview::render(snapshot, &[], labels)...,   // see §3/§5
    grid::SOURCING_CURATE_BODY_GRID => grid::render(snapshot, config)...,
    _ => semio_framework_plugin::built_text_to_component_tree(Label::data("")),
}
```

## 2. `setActiveExample` — is `demo-stock` real?

Command handler:
`✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎮️commands/📄️set-active-example/🦀️component.rs`
(exact path: `.../🎮️commands/📄️set-active-example/🦀️component.rs:19-27`)

```rust
pub fn handle(payload: &SetActiveExample, _doc: ..., _cfg: ...) -> Result<Emit<...>, Fault> {
    let text = match payload.example_id.as_str() {
        "" | EMPTY_EXAMPLE_ID => crate::artifacts::curate::dsl::EMPTY_CURATION_TEXT,
        DEMO_STOCK_EXAMPLE_ID => crate::artifacts::curate::dsl::DEMO_STOCK_TEXT,
        _ => return Err(Fault::from("sourcing.example.unknown")),
    };
    let next = <CurateSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| Fault::from(error.to_string()))?;
    Ok(Emit { effects: vec![reset_document_effect(&next)], ..Default::default() })
}
```
Only two match arms exist: `demo-stock` and `empty-curation` (plus `""` aliasing to empty); anything else is a hard `Err("sourcing.example.unknown")` — **there is no catch-all/no-op branch**.

**`demo-stock` is real and non-empty**, settling the disagreement with hard evidence:

- `DEMO_STOCK_TEXT` is `include_str!`'d from an inline DSL fixture file, not from a `📚️examples` example-picker asset:
  `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/📸️snapshot/📝️text/🦀️component.rs:12`:
  `pub const DEMO_STOCK_TEXT: &str = include_str!("../../../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio");`
- That file (`.../📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio`) contains 10 authored `stock-extra` rows across beams/windows/slabs (Glulam GL24h, KVH C24, Steel IPE200, Steel HEA160, three window kinds, three slab kinds) with real `availability`/`typology-path` values, and an empty `curated` block.
- Those exact 10 rows are the single source of truth `demo_stock()` in the schema module composes from the three built-in sourcing modules (**not** the `sourcing-module-beams`/`-slabs`/`-windows` extension crates directly — those extensions only *contribute a topic* for hot-swap/host discovery; the canonical demo content lives in the schema's own `beams`/`windows`/`slabs` sub-modules):
  `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️component.rs:366-521` (`pub mod beams { … BeamsModule::demo_kinds() … }`, `pub mod windows { … }`, `pub mod slabs { … }`), aggregated at line 681: `let mut modules: Vec<SourcingModules> = vec![beams::BeamsModule.into(), windows::WindowsModule.into(), slabs::SlabsModule.into()];`.
  `demo_stock()` at line 738-740: `sourcing_modules("[]").iter().flat_map(|module| module.demo_kinds()).collect()`.
- `default_document()` (schema.rs:748-751), which is `SourcingCurateApp::initial_snapshot()` (editor `🦀️component.rs:663-665`), parses the SAME `DEMO_STOCK_TEXT` — **the app's initial document on load already equals the demo-stock example**.
- The extension crates `✏️s/🔌️plugins/🪵️sourcing/🧩️extensions/{🪵️beams,🧱️slabs,🪟️windows}/🦀️component.rs` are declarative-only bundles (`ExecutionMode::Declarative`, zero `.handler(...)`) that `contributes_topic("sourcing.module", { kindsJson: serde_json::to_string(&module.demo_kinds()) ... })` — i.e. they republish the same schema-owned `demo_kinds()` for hot-swap discovery via `stockFromCatalogue`/`available_modules`, they do not define separate content.
- Test oracle proving it end-to-end through the real dispatch/registry/job pipeline: `retained_example_load_publishes_authored_stock_and_closes_exact_owners` (editor `🦀️component.rs:962-1003`) dispatches `SetActiveExample{ example_id: DEMO_STOCK_EXAMPLE_ID }` through the full retained-tool job machinery and asserts `assert_eq!(stock, oracle)` against `📚️examples/🎬️demo/🧪️expected-stock.json`.
- The manifest exposes this choice to the UI as a real staged argument form (not just descriptor JSON boilerplate): `.action_args("setActiveExample", vec![ActionArgDef::select("exampleId", ..., vec![ActionArgOption::new(DEMO_STOCK_EXAMPLE_ID, ...), ActionArgOption::new(EMPTY_EXAMPLE_ID, ...)]).default_value(DEMO_STOCK_EXAMPLE_ID)])` — `🦀️component.rs:885-893`.

**Verdict for item 2: `demo-stock` is a real, fully-populated 10-item stock inventory, sourced from inline Rust module data (`schema::{beams,windows,slabs}`) via an authored DSL fixture text file, not a stub and not merely a descriptor artifact.**

## 3. Document → window surface tracing

- **Pool** (`.../🪟️windows/🏊️pool/🦀️component.rs:37-70`): `pool_view()` calls `filtered_stock(document, &cfg.filters)` (schema.rs:262-274, reads `document.stock_extra` via `stock_of`), applies `cfg.filters.sort`, and maps each `ObjectKind` to a row `[name, module_id, typology_path, availability, curated_count]`. Needs: `document.stock_extra` (populated) + `cfg.filters`/`cfg.filters.sort` (session-only view state in `SourcingCurateConfig`).
- **Curated** (`.../🪟️windows/🧺️curated/🦀️component.rs:35-49`): iterates `document.curated`, looks up each `object_id` against `stock_of(document)`, emits `[name, availability, count]`. Needs: `document.curated` + `document.stock_extra` (to resolve names/availability).
- **Preview** (`.../🪟️windows/👁️preview/🦀️component.rs:44-55`): takes `selected_ids: &[String]`, finds the first id in `stock_of(document)`, and if found builds a `MeshView` via `kind_mesh_json`/`instance_json` (schema.rs:236-255); otherwise returns the "No selection" text placeholder. Needs: a non-empty `selected_ids` slice from the render call site.
- **Grid** (`.../🪟️windows/🔢️grid/🦀️component.rs:39-61`): builds one mesh per distinct `filtered_stock(document, &cfg.filters)` kind, places instances on a `grid_placement`/`grid_scale` layout (schema.rs:710-731). Needs: `document.stock_extra` + `cfg.filters` (same data as Pool, no selection needed).

Stock provenance for all four: `pub fn stock_of(document: &CurateSnapshot)` in `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curate/🦀️component.rs:220-223`, mapping `document.stock_extra` rows 1:1 into `ObjectKind`s.

## 4. Interactivity — commands vs. actual UI wiring

Dispatch bridge (editor `🦀️component.rs:172-210`, `sourcing_curate_command_from_action`) maps 15 action ids to `SourcingCurateCommand` variants; `app_commands!` enum at lines 128-151. All 15 command handlers under `🎮️commands/*` are implemented (no `todo!()`/stub bodies):

| command | file | real logic? |
|---|---|---|
| `setDocument` | `📄️set-artifact-json/🦀️component.rs:16-24` | yes — bounded JSON→`CurateSnapshot` import, dev-only ("kept out of the command palette") |
| `setActiveExample` | see §2 | yes |
| `stockFromCatalogue` | `📄️stock-from-catalogue/🦀️component.rs:19-31` | yes — merges every not-yet-present catalogue kind from `available_modules`, tested for idempotency |
| `curateAdd` / `curateRemove` / `curateSetCount` / `dropOnPool` / `dropOnCurated` | `🧺️curate-add`, `🧺️curate-remove`, `🧺️curate-set-count`, `🧺️drop-on-pool`, `🧺️drop-on-curated` (`/🦀️component.rs`) | yes — all route through `curation_decision_for_delta`/`_for_set` (schema.rs:310-330) and emit real `SourcingMutation`s (create/change-count/delete), or emit nothing on no-op. Round-trip covered by `curate_add_and_remove_round_trip_through_operations` etc. |
| `setFilterQuery` / `setFilterModule` / `setFilterTypology` / `setFilterMinAvailability` / `sortTable` | `🔍️set-filter-*`, `🔍️sort-table` | yes — emit `SourcingCurateConfigMutation`s consumed by `filtered_stock`/pool sort (verified in §3) |
| `setLocale` / `setContributions` | `🗣️set-locale`, `🧩️set-contributions` | yes (not reviewed in depth; out of ticket's fixture/window scope) |

**But the rendered windows never wire any of this to a UI affordance:**

- `pool::render`/`curated::render` build a `TableWindowKit`-backed `TableView { columns: Vec<String>, rows: Vec<Vec<String>> }` — plain strings only. The SDK's `TableView` doc comment says outright: *"cells are plain strings; typed cells (`TableCell`) are a renderer concern, not this SDK-level view-model's"* (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:24948-24953`). The framework does define a `TableRowAction`/`TableRowActions` mechanism for clickable per-row buttons (same file, ~line 24983 onward) and an `editable_window_kind()` with a `"set-cell"` action, but **the sourcing windows use neither** — every `WindowKindDefinition` for pool/curated/preview/grid sets `actions: Vec::new()`, `interactions: Vec::new()`, `capabilities: Vec::new()` (each window's `definition()`).
- `sourcing_action(...)`, the app's own helper for building `(ActionId, UiValue)` action bindings for chrome (editor `🦀️component.rs:64-68`), is **defined but never called anywhere in the plugin** (`grep -rn "sourcing_action(" ✏️s/🔌️plugins/🪵️sourcing/` returns only its own definition). No window body ever attaches a click/drag action to a row.
- `SOURCING_CONTROLLER_ID`/`SOURCING_DRAG_MIME` (editor `🦀️component.rs:59-60`) is likewise declared and **never referenced again** anywhere in the crate — there is no drag-source/drop-target wiring keyed to it despite `dropOnPool`/`dropOnCurated` existing as commands, and despite the pool window's own doc comment claiming "the full stock catalogue with filter chrome **+ drag source**" (`.../🏊️pool/🦀️component.rs:1`).
- No search box, module-toggle checkboxes, typology tree, min-availability slider, or sortable column headers are built anywhere in the pool/curated/grid render code — `setFilterQuery`/`setFilterModule`/`setFilterTypology`/`setFilterMinAvailability`/`sortTable` are declared `hidden_view_action` (`in_palette: false`, `🦀️component.rs:780-785,878-883`) with **no `.action_args(...)` staged form** (unlike `setActiveExample`, which does get one). They are dispatchable as typed commands/tools but have no discoverable UI control in this app's own chrome.
- The only interactivity that IS wired generically by the framework is row/object **selection**: `.interaction(InteractionDefinition{ id: "rows", selection: SelectionSpec{ modes: [Single], methods: [Pick], ... } })` attached to pool/curated/grid via `.window_kind_interactions(..., vec![InteractionRef::new("rows")])` (`🦀️component.rs:851-867`). This is real (framework-injected verbs `interactionSelect`/`clearSelection`/etc.), letting a user click a pool/curated row or a grid mesh instance to select an object — **but see §5: that selection currently never reaches the Preview window's render, so clicking a row/instance has no visible effect today.**

## 5. Dead ends / placeholders that make a window blank or inert

No `todo!()`/`unimplemented!()` in the sourcing plugin (`grep -rn "todo!\|unimplemented!" ✏️s/🔌️plugins/🪵️sourcing/` — zero hits). The only "not yet implemented" hits are unrelated txt import/export serializers (`.../🚪️io/📥️import/…/📄txt/🔖️utf-8/✳️any/🦀️component.rs:18,21` and `.../🚪️io/📤️export/…/📄txt/🔖️utf-8/✳️any/🦀️component.rs:12,21`, all `Err("txt {import,export} not yet implemented")`), out of this ticket's window/interactivity scope.

The one real blank/inert path in scope:

**Preview window is permanently stuck on its "No selection" placeholder** — not a `todo!()`, but a genuine functional gap, self-documented in the code:
```rust
// .../🪟️windows/👁️preview/🦀️component.rs:37-43
/// 👁️ `selected_ids` is the "rows" interaction domain's current selection — `ArtifactApp::render`
/// carries no `InteractionView` (ticket 26/08/14/FIRST-CLASS-HOVER-AND-SELECTION-MECHANISM: the
/// breaking pass only threaded it into `handle`/`copy_fragment`/`cut_operations`), so the app-level
/// call site always passes an empty slice and this window degrades to its "no selection" default
/// until a future wave threads interaction into render.
```
Call site (editor `🦀️component.rs:744-749`): `preview::SOURCING_CURATE_BODY_PREVIEW => preview::render(snapshot, &[], labels)...` — the `&[]` is hard-coded, never populated from the live selection. Confirmed by test `renders_via_the_app` (preview `🦀️component.rs:92-97`): *"`render` carries no `InteractionView` yet, so the app-level render always shows the placeholder"* — `assert!(render_body(&mut app, SOURCING_CURATE_BODY_PREVIEW).await.contains("No selection"))`. `preview::render`'s own selection-driven mesh logic is correct and unit-tested in isolation (`preview_renders_selected_mesh_id`, lines 68-75) — the gap is purely in the app-level call site never being given a selection, a discovered framework limitation, not app-local negligence.

## 6. Verdict

1. Aussuchen genuinely opens four windows by default (Pool/Curated/Preview/Grid, Table/Table/World3d/World3d), three-column layout — confirmed in Rust source and a passing layout test, matching the demonstrator descriptor.
2. `demo-stock` is real: a 10-item stock catalogue (4 beams, 3 windows, 3 slabs) sourced from the schema's own `beams`/`windows`/`slabs` module structs, baked into an authored DSL fixture and asserted against an oracle JSON in tests — this settles the two earlier agents' disagreement in favor of "real".
3. The app's initial document already equals demo-stock (`initial_snapshot() == default_document() == parse(DEMO_STOCK_TEXT)`), so on first load Pool/Curated/Grid should render populated real data with no user action needed.
4. Pool and Grid correctly compute/render filtered real stock; Curated correctly renders the (initially empty) curated set; all backed by real, tested pure functions.
5. Preview is functionally dead today: the render call site always passes an empty selection, so it permanently shows "No selection" regardless of what a user clicks — a discovered framework gap (selection not threaded into `render`), not a stub in this app's own code.
6. Curate-add/remove/set-count/drop-on-pool/drop-on-curated and all five filter/sort commands are fully implemented, mutation-correct, and unit/round-trip tested at the dispatch layer.
7. None of those commands have any UI affordance wired into the rendered windows: `TableView` cells are plain strings with zero row actions, `sourcing_action()`/`SOURCING_DRAG_MIME` are defined but never invoked, and the filter commands have no search box/checkbox/slider chrome or staged action-args form.
8. The only real interactivity wired end-to-end today is generic single-object row/instance selection (framework "rows" interaction domain on Pool/Curated/Grid) — but it currently has no visible consequence because of the Preview gap in (5).
9. No `todo!()`/`unimplemented!()` in the sourcing plugin; the only genuine stubs are unrelated txt import/export serializers, outside this ticket's scope.
10. To reach "correct fixture + interactive": nothing needed for the fixture (already correct); interactivity needs (a) the framework's selection→render plumbing landed and Preview wired to use it, and (b) actual UI chrome/action bindings added to Pool/Curated (search box, module/typology/availability filters, sortable headers, drag source/drop targets or row-action buttons) since the backing commands already work correctly.
