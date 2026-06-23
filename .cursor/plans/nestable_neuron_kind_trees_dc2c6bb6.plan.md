---
name: Nestable Neuron Kind Trees
overview: Add a first-class authoring mechanism so neuron kinds declare their own nesting, render the kinds catalogue palette as a recursive tree, and tidy procedural's flat brep sections into a proper authored tree.
todos:
  - id: rust-catalogue
    content: "flow/core/lib.rs: add recursive CatalogueGroup struct + groups field on CatalogueSection (serde camelCase, skip empty); update static sections and test literals so nesting round-trips."
    status: completed
  - id: flow-model
    content: "flow/react/index.tsx: add group to FlowModuleNeuronKindV1, recursive CatalogueGroup + groups on CatalogueSection, nest kinds by authored group in catalogueSections(), add flattenCatalogueItems and update flowRankCatalogueSuggestions."
    status: completed
  - id: shared-builder
    content: "flow/react/index.tsx: add exported buildCatalogueKindsTreeSections(sections, idPrefix, dragDataFn) recursive tree builder; update file tests."
    status: completed
  - id: play-builders
    content: "flow/play/index.ts and procedural/play/index.ts: reimplement buildFlowPlayKindsTree / buildProceduralPlayKindsTree on the shared builder; keep empty state; update tests."
    status: completed
  - id: procedural-author
    content: "procedural/react/index.tsx: add group param to brepKind, restructure BREP_CATALOGUE_SECTIONS into one nested Brep tree, update catalogueSections() override and tests."
    status: completed
  - id: validate
    content: Run Rust + TS test targets for flow/core, flow/react, flow/play, procedural/react, procedural/play; verify nested draggable palette at runtime.
    status: completed
isProject: false
---

# Nestable Neuron Kind Trees

## Goal
Give neuron-kind authors an explicit way to declare nesting (no auto-derivation from IDs), render the kinds catalogue/palette as a recursive tree in both flow and procedural workbenches, and restructure procedural's flat `BREP_CATALOGUE_SECTIONS` into a proper authored tree.

## Mechanism (author-controlled)
Each neuron kind gains an optional, author-declared `group: readonly string[]` — an ordered list of human-readable group titles (e.g. `["Primitives 3D"]`, `["Solid", "Booleans"]`). The catalogue model becomes recursive; the workbench tree builder renders it recursively. Empty/absent `group` means the kind sits at its section root, so existing flat modules keep working.

## Changes

### 1. Rust catalogue round-trip — [flow/core/lib.rs](flow/core/lib.rs) (`#region 🔖Catalogue`)
The catalogue passes through `FlowSession` (`set_host_catalogue_json` → `catalogue_json()` deserializes into `Vec<CatalogueSection>`, appends static Inputs/Outputs, re-serializes in `merge_catalogue_sections`). Serde drops unknown fields, so nesting must be modeled in Rust:
- Add recursive `CatalogueGroup { id, title, groups, items }` (serde `camelCase`, `skip_serializing_if` empty/none).
- Add `#[serde(default, skip_serializing_if = "Vec::is_empty")] pub groups: Vec<CatalogueGroup>` to `CatalogueSection`.
- Update `static_catalogue_sections()` and the in-file test literals to include `groups: vec![]`.

### 2. Catalogue model + builders — [flow/react/index.tsx](flow/react/index.tsx)
- Add `group?: readonly string[]` to `FlowModuleNeuronKindV1` (`#region 🔖ExtensionHost`).
- Add recursive `CatalogueGroup` interface and `groups?: readonly CatalogueGroup[]` on `CatalogueSection` (`#region 🔖Catalogue`).
- `FlowExtensionHost.catalogueSections()`: nest each module's kinds by their authored `group` path (helper that builds ordered nested groups with stable ids from the title path). Kinds without a group stay in `items`.
- Add exported `flattenCatalogueItems(sections)` (recurses `groups`) and switch `flowRankCatalogueSuggestions` (and any spotlight flatten) to it so search still sees every leaf.
- Add exported shared builder `buildCatalogueKindsTreeSections(sections, idPrefix, dragDataFn)` returning `UiTreeSectionNode[]`, mapping group -> nested `UiTreeItemNode` (`items`) and kind -> draggable leaf (reuses `flowPlayCatalogueItemDragData`). The renderer already recurses `UiTreeItemNode.items`.
- Update affected tests in the file's `#region 🧪Tests`.

### 3. Workbench tree builders use the shared recursive builder
- [flow/play/index.ts](flow/play/index.ts): reimplement `buildFlowPlayKindsTree` via `buildCatalogueKindsTreeSections(sections, "flow-play-kinds")`; keep the empty/loading state; drop local `flowPlayKindsTreeItem`. Update tests.
- [procedural/play/index.ts](procedural/play/index.ts): same via `buildProceduralPlayKindsTree(... , "procedural-play-kinds")`; drop local `proceduralPlayKindsTreeItem`. Update tests.

### 4. Author proper brep tree — [procedural/react/index.tsx](procedural/react/index.tsx) (`#region 🔖BrepFlowModule`)
- Extend `brepKind(...)` with a `group` (title path) argument and set it on every kind.
- Restructure so brep is authored as ONE nested "Brep" tree: a single top-level section with nested groups (Primitives 3D, Draw 2D, Curves, Surfaces, Solid, Booleans, Transforms, Intersections, Evaluate, Measure, Query, Repair, IO, Gears, Legacy) — authors may nest deeper as desired.
- Update the `catalogueSections()` override to emit the single nested Brep section (built from kinds' `group` paths via the shared nesting helper); keep `kindInfosJson()` / `listEntries()` / `BREP_MODULE_MANIFEST`.
- Update tests in the file's `#region 🧪Tests` (catalogue/kinds-tree assertions now expect nested groups).

## Validation
- Build flow + procedural WASM where needed; run `nx` test targets for `flow/core` (Rust), `@semio-tech/flow-react`, `@semio-tech/flow-play`, `@semio-tech/procedural-react`, `@semio-tech/procedural-play`.
- Confirm at runtime via the procedural play workbench that brep kinds render as a nested tree and remain draggable (check console, since correctness must be observed, not assumed).

## Notes / decisions
- WASM flow modules (math/text/etc.) stay flat by default; the optional Rust `NeuronKindInfo.group` field for them is an out-of-scope follow-up — the mechanism already supports them via the recursive model.
- Group node ids are derived by slugifying the authored title path purely for stable React keys; the hierarchy itself is fully author-declared, never inferred from the kind ID.