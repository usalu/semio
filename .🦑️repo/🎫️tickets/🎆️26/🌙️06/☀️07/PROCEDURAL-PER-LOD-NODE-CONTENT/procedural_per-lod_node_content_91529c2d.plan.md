---
name: Procedural per-LOD node content
overview: "Give every DAG node a PascalCase name, a PascalCase abbreviation, and an icon, then rewrite the shared DAG host LOD rendering so each zoom tier shows the requested content (minimap: body only; overview: icon; compact: abbreviation; normal: name; detail: icon+abbreviation; micro: icon+name)."
todos:
 - id: data-model
   content: Add abbreviation + icon to NeuronKindInfo and DagNodeSpec; author values for all neuron kinds + modules
   status: completed
 - id: populate
   content: Populate abbreviation/icon in flow/core widget_to_dag_node, CatalogueItem, and flow/react types
   status: completed
 - id: pascalcase
   content: Add to_pascal_case helper and normalize name + abbreviation at node construction
   status: completed
 - id: icon-pipeline
   content: Extract shared IconPaintCache from BoardHost and wire it into DagHost with paint_node_icon
   status: completed
 - id: lod-rewrite
   content: Replace DagDrawLod content selectors and rewrite paint loop for per-LOD icon/abbreviation/name
   status: completed
 - id: tests
   content: Extend existing Rust + TS test modules for new fields, LOD selection, and PascalCase
   status: completed
 - id: verify
   content: Rebuild wasm, run nx tests, and confirm runtime per-tier rendering with DEBUG logs
   status: completed
isProject: false
---

## Decisions locked

- Applies to the shared DAG host (procedural, flow, dag all change).
- `abbreviation` + `icon` added to neuron-kind / node-kind definitions and the `DagNodeSpec` schema, authored for every node.
- Minimap keeps the node body silhouette (fill/stroke); only text and icons are suppressed there.

## Per-LOD content target

- minimap: body only (no text, no icon)
- overview: icon only
- compact: abbreviation
- normal: name
- detail: icon + abbreviation
- micro: icon + name

## 1. Data model: name, abbreviation, icon

- [neural/engine/lib.rs](neural/engine/lib.rs): add `pub abbreviation: String` and `pub icon: String` to `NeuronKindInfo` (lines 184-195), following the existing `name`/`summary` display-metadata precedent. Update all `NeuronKindInfo { .. }` literals + tests.
- Author `abbreviation` (PascalCase) + `icon` for every neuron kind in the flow modules: [flow/modules/math/lib.rs](flow/modules/math/lib.rs), `text`, `list`, `logic`, `dictionary` `lib.rs`, and any procedural/brep neuron kinds surfaced through `proceduralExtensionHost`. Icons use the existing icon-codec encodings (prefer `emoji:` strings, e.g. `emoji:➕️`).
- [mathematical/graph/port/directed/dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs): add `pub abbreviation: String` and `pub icon: String` to `DagNodeSpec` (lines 231-244) with `#[serde(default)]`; thread through `DagNodeSpec::computation` and all spec literals/tests.

## 2. Populate fields when building nodes

- [flow/core/lib.rs](flow/core/lib.rs) `widget_to_dag_node` (lines 228-290): for neuron widgets pull `abbreviation`/`icon` from `kind_infos`; for IO widgets (`InputSlider`, `InputNote`, `OutputPreview`, `OutputAction`, plus `Select`/`Screen` kinds) supply PascalCase names/abbreviations and default emoji icons. Extend `CatalogueItem` (lines 359-387) and `static_catalogue_sections` with abbreviation/icon so spawned nodes inherit them.
- [flow/react/index.tsx](flow/react/index.tsx): add `abbreviation`/`icon` to `FlowModuleNeuronKindV1` and `CatalogueItem` types so manifest data carries through.

## 3. PascalCase enforcement

- Add a `to_pascal_case` helper (Rust, in the `directed` crate near `DagNodeSpec`) and apply it to `name` + `abbreviation` when constructing/deserializing DAG nodes in `flow/core` so authored values are normalized. Cover with a unit test.

## 4. Icon rendering in the DAG host

- Extract the board host icon pipeline into a shared, reusable unit in the `directed` crate so there is a single source of truth: move `get_or_build_icon_paint` + `icon_vector_cache` + `themed_icon_lookup` (currently `BoardHost` fields/methods in [normal/board_host.rs](mathematical/graph/port/directed/normal/board_host.rs) lines 294-660, 2460-2570) into a shared `IconPaintCache` struct (place in `types.rs` under a new `#region Icons`). `BoardHost` and `DagHost` both hold one.
- Add `icon_paint_cache` + `themed_icon_lookup` to `DagHost` (lines 989-1009) and a `paint_node_icon` helper mirroring the board host's icon paint call, resolving icons via `infinite_canvas::icon_codec` (self-contained `emoji:`/`svg`/`data:` strings; `themed_icon_lookup = |_| None` default).

## 5. Rewrite LOD content selection + paint

- In [dag/lib.rs](mathematical/graph/port/directed/dag/lib.rs) replace `DagDrawLod::shows_name`/`name_is_horizontal` (lines 920-926) with content descriptors:
  - `node_icon_visible(self) -> bool` -> true for Overview, Detail, Micro.
  - `node_label(self) -> DagNodeLabel` (`None | Abbreviation | Name`) -> Minimap/Overview: None; Compact: Abbreviation; Normal: Name; Detail: Abbreviation; Micro: Name.
- Update the paint loop (lines 2032-2205): after the minimap `continue`, draw the icon (when `node_icon_visible`) centered/leading, and draw the selected label text (`node.abbreviation` or `node.name`) per `node_label`. Keep existing structural gates (`shows_computation_layout`, `shows_handles`, `shows_controls`, `shows_port_labels`) for sections/handles/controls; only the name/icon content changes. Adjust `paint_node_name_horizontal` / `paint_computation_node_name` / `paint_io_widget_name` call sites to consume the new label/icon model.
- Update `DAG_LODS` descriptions (lines 754-790) to match the new content.

## 6. Tests

- Extend existing test modules only (no new files): `dag/lib.rs` `#region Tests` (LOD label/icon selection per tier, serde round-trip of new fields, PascalCase normalization), `flow/core/lib.rs` (widget→node abbreviation/icon population), `neural/engine/lib.rs` + `flow/modules/*` (NeuronKindInfo new fields), and `procedural/play/index.ts` if catalogue assertions touch the new fields.

## 7. Build + verify

- Rebuild the affected wasm crates and run the relevant nx test targets (registered via `launch.json`), then confirm runtime behavior in the procedural shell at each zoom tier with `[DEBUG]` logs of the resolved `(lod, icon, label)` before removing them.

## Open follow-up (non-blocking)

- Rendering icons inside the workbench catalogue tree (`buildProceduralPlayKindsTree`) is out of scope unless desired; the data will be available if we choose to add it later.
