# Flow and GIS Child Links and Window Configuration Audit

## Scope and Method

This is a source-only, read-only audit of the Flow, GIS Map, and GIS Terrain ownership contracts. No production code or tests were changed, and no Cargo build was run. The current native `retained-window-input` run is still compilation-only in `🗑️generated/window-document-generation-green-1.log`; this report makes no native-green claim.

The audit separates a parent document's authored fields from its child document handles and from state that belongs to one concrete window. A derived child handle is still a real artifact/snapshot field: the parent owns the slot and its stable link, while its child store owns the child document's contents.

| Artifact | Parent-authored state | Child-link contract | Concrete-window state |
| --- | --- | --- | --- |
| Flow | `schema` | `content: ArtifactChild<SemioFlowSnapshot>` | `camera` |
| GIS Map | `positions`, `routes`, `regions`, optional `image` link | fixed derived `drawing` and `value` links | camera, layers, render and vector presentation |
| GIS Terrain | `exaggeration`, `imported_features_json` | derived `mesh` link | camera |

## P1 — Flow Facets Still Describe Retired Inline Content and Persist a Viewport in the Document

`✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` defines `FlowArtifact` with `schema`, `camera`, and the `content` child. Its snapshot counterpart in `🧬️schema/📸️snapshot/🦀️.rs` has the same shape. The root module `✏️s/🔌️plugins/🌊️flow/🗿️artifacts/🌊️flow/🦀️.rs` constructs that child by translating working widgets/layout to Semio Flow nodes and synapses to edges (`flow_content_snapshot_from_working` and `flow_content_child_handle`). This is deliberate composition, not an inline-data compatibility layer.

Yet all Flow TypeScript, GraphQL, and JSON facets still expose `widgets`, `synapses`, and `layout` in the artifact, snapshot, and diff types:

- `.../🌊️flow/.../🧬️schema/🟦️.ts`, `🔗️.graphql`, and `🔣️.json`
- `.../🌊️flow/.../🧬️schema/📸️snapshot/🟦️.ts`, `🔗️.graphql`, and `🔣️.json`
- `.../🌊️flow/.../🧬️schema/🔺️diff/🟦️.ts`, `🔗️.graphql`, and `🔣️.json`

They must declare the composed `content` child handle and remove the retired inline fields. The nested artifact form in the diff must use the same complete artifact contract. This is a live cross-runtime serialization mismatch, not evidence that the retired fields should be restored.

The camera needs a second correction before those facets are made consistent. `✏️s/🔌️plugins/🌊️flow/✏️editor/🎭️modes/✏️edit/🪟️windows/🌊️main/🦀️.rs` renders `NodeGraphViewport` from the rendered window configuration. But both `🎮️commands/🔭️node-graph-viewport/🦀️.rs` and `🎮️commands/🎯️focus-selection/🦀️.rs` emit `FlowConfigMutation::SetCamera` through the application config lane. `✏️editor/🎚️config/🧬️schema/🦀️.rs` therefore persists a viewport in configuration shared by every Flow window, while the document artifact also serializes it. Multiple Flow main windows cannot retain independent viewpoints.

### Required Flow Design

Create a typed configuration schema for the concrete `flow-main` window kind. The viewport commands must resolve their trusted `ViewModel` window address, read and write that exact window configuration, and publish an exact-window configuration event. The Flow document snapshot, artifact, and diff must drop `camera`; document-to-fixture conversion must use an explicit deterministic default viewport and must never read a current window configuration.

The Flow `content` link remains in artifact/snapshot/diff, including text and Pack forms. It must retain its child identifier and `s.stdio.semio@v1/flow` target typing. It is the parent reference to the composed document; copying widgets, edges, or layout back into the parent would duplicate ownership.

The remaining Flow presentation fields in `FlowConfig` (`preview_off_node_ids`, LOD, proximity, grid settings, and catalogue/automation panels) should be inventoried in the same migration. Each is currently a view or interaction preference; a document field should be retained only where a command or runtime proves it changes shared authored content. Camera is the required first move because it is directly used by the window renderer and mutations.

### Flow Native Tests

1. Open two `flow-main` windows for one document, set distinct cameras through exact-window command contexts, and assert that each renderer receives its own camera. Assert the document snapshot and app-config bytes do not change.
2. Persist, close, and reopen both windows; assert each concrete window restores its own camera and a document replacement or same-byte reload does not reset it.
3. Convert a document to and from its working fixture with no window context. Assert fixed default viewport behavior and byte-stable document output, proving no current viewport leaks into a snapshot.
4. Validate the artifact, snapshot, and nested-diff artifact with native, text, Pack, and Ajv fixtures. The current inline `widgets`/`synapses`/`layout` shape must be rejected and the typed `content` handle accepted.

## P1 — GIS Map Must Publish All Three Parent Child Slots Across Facets

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🦀️.rs` establishes three distinct link categories:

- `positions`, `routes`, and `regions` are authored parent data.
- `drawing` and `value` are composed children with stable admitted member identities (`gismap-drawing` and `gismap-value`). Their handles are derived by `gis_map_snapshot_with_derived_children`, while the child stores own their contents.
- `image` is an optional authored `ArtifactChild<SemioImageSnapshot>` link. The module expressly preserves a supplied image handle even though there is no raster baseline.

The native snapshot schema, `.../🧬️schema/📸️snapshot/🦀️.rs`, correctly contains `positions`, `routes`, `regions`, `drawing`, `image`, and `value`. The native artifact schema, `.../🧬️schema/🦀️.rs`, currently retains only the optional `image` field and derives/drops the other two link values. The external artifact and snapshot facets omit all three:

- `.../🗺️gismap/.../🧬️schema/🟦️.ts`, `🔗️.graphql`, and `🔣️.json`
- `.../🗺️gismap/.../🧬️schema/📸️snapshot/🟦️.ts`, `🔗️.graphql`, and `🔣️.json`

The correct repair is to expose `drawing`, `image`, and `value` as child-handle fields in every artifact and snapshot facet and to add `drawing` and `value` to the native `GisMapArtifact`. `to_snapshot` may continue to derive the fixed handles; `from_snapshot` and `set_snapshot` must retain or consistently validate/re-derive them. A fixed handle does not make a link disposable: it is needed to find and validate the separate child document.

The direct `GisMapDiff` fields can remain limited to the directly mutable authored fields (`positions`, `routes`, `regions`) because the derived links have no independent parent mutation and image has no mutation command today. Its nested `artifact` field must nevertheless use the complete artifact schema, including the three links. When image mutation is added, its sparse mutation must distinguish omitted, clear, and set (`Option<Option<ArtifactChild<...>>>`).

The existing route replacement test at `.../🗺️gismap/🎮️mutations/♻️replace-route-data/🧪️tests/🦀️.rs` already checks that feature edits retain the stable drawing/value identities. Preserve that invariant while adding serialization facets; do not replace those slots with parent copies of the child content.

GIS Map's `✏️editor/🎚️config/🧬️schema/🦀️.rs` labels `Gis2dConfig` as view-state but stores `layer_visibility`, `camera_json`, `render_mode`, `vector_style`, LOD, and stroke scale in application config. Each applies to a concrete map window. Move this typed group to map-window configuration and route commands through their exact `ViewModel` window address. Any live camera presence message must also carry that concrete window address if separate map windows are to receive independent ephemeral viewpoints.

### GIS Map Native Tests

1. Make a map snapshot with a supplied image and derived drawing/value handles. Verify native, text, Pack, and Ajv round trips retain all three links and target types.
2. Replace route, region, and position data; assert drawing/value child identifiers remain the admitted stable slots and the relevant child content changes only in its child store.
3. Exercise absent, set, and clear image once an image command exists. Assert each produces a distinct correct artifact/snapshot/diff outcome.
4. Open two map windows, change camera and layer/render preferences independently, and assert isolated rendering, persisted per-window reopen behavior, and unchanged document bytes.

## P1 — GIS Terrain Mesh Link Is a Real Facet, but Its Current Content Address Does Not Describe Its Payload

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs` has authored `exaggeration` and `imported_features_json` plus a `mesh: Option<ArtifactChild<SemioMeshSnapshot>>`. Its native snapshot has all three fields. `GisTerrainArtifact::to_snapshot` derives the mesh; `from_snapshot` and `set_snapshot` retain it. This is the same parent-child model as GIS Map's derived links.

All external terrain artifact and snapshot facets omit `mesh`:

- `.../🏔️gisterrain/.../🧬️schema/🟦️.ts`, `🔗️.graphql`, and `🔣️.json`
- `.../🏔️gisterrain/.../🧬️schema/📸️snapshot/🟦️.ts`, `🔗️.graphql`, and `🔣️.json`

Add the optional typed mesh handle to each. Keep direct terrain diff fields focused on directly authored terrain properties, but ensure its nested artifact form uses the complete terrain artifact contract.

There is also a concrete identity defect in the native terrain root. `gis_terrain_mesh_child_handle` derives the mesh child identifier from `exaggeration` plus `imported_features_json`, but `gis_terrain_mesh_from_snapshot` emits a flat quad with `z = 0` regardless of exaggeration. Thus a `change-exaggeration` mutation produces a new content-addressed child ID for equal child bytes. The claim that the identifier names the content is false, and the editable property has no mesh effect.

The implementation must choose and test one semantic rule:

1. If exaggeration is intended to modify terrain geometry, construct mesh vertices from a defined elevation source and assert changing exaggeration changes encoded mesh content and hence its child identifier.
2. Until a height source exists, calculate the child identifier from the canonical encoded `SemioMeshSnapshot`, not input fields. Equal generated mesh content then retains one identity, while `exaggeration` remains a parent field with no claimed mesh effect.

The root documentation calls exaggeration the terrain's editable property and describes a height lift, so option 1 is the intended eventual domain behavior. Option 2 is required if the current no-height model remains; retaining input-keyed identity with byte-identical meshes is incorrect under either policy.

`✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/✏️editor/🎚️config/🧬️schema/🦀️.rs` keeps `Gis3dConfig.camera_json` in app configuration although it is a free/live viewport. Move it to a typed terrain-window configuration and have viewport commands/readers use exact window addresses. It must be persisted independently for concurrent terrain windows and remain absent from terrain artifact/snapshot state.

### GIS Terrain Native Tests

1. Round trip terrain snapshots carrying a mesh link through native, text, Pack, and Ajv. Verify artifact/snapshot/diff nested-artifact facets agree exactly.
2. Generate two terrains with the same imported data and distinct exaggerations. If mesh bytes are equal, assert equal child IDs; if exaggeration is implemented geometrically, assert changed mesh bytes and changed child ID. Never allow only the ID to change.
3. Validate a mesh child envelope is reachable through its parent slot after feature import/replacement, while the child contents stay owned by the mesh child store.
4. Open two terrain windows, assign separate cameras, then persist and reopen. Assert renderer isolation and unchanged terrain document bytes through camera changes and document replacement.

## Implementation Order

1. Introduce typed per-kind window configuration and migrate Flow camera, GIS Map view settings, and GIS Terrain camera commands/renderers to exact window scopes. Add isolated multiwindow persistence tests before removing document/app-config fields.
2. Repair Flow artifact/snapshot/diff external facets around the composed `content` child and remove inline legacy members plus document camera.
3. Make GIS Map's three child slots and Terrain's mesh slot uniform in native artifact models and every external artifact/snapshot/nested-diff facet.
4. Resolve the terrain mesh identity/payload contradiction before treating its derived child as content addressed.

These changes preserve the intended child boundaries: the root manages typed links and the child stores manage the actual Flow, Drawing, Value, Image, and Mesh content. They also make viewport mutation ownership match the SDK's concrete window persistence and replacement lifecycle.
