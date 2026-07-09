---
name: CAD Renderer Parity Restore
overview: "Restore the `cad` plugin (Rust/WASM + generic `World3dHost`) to premigration functional parity: real per-object brep meshes for all four panes (Shape/Building/Energy/Structure Classic), and working click-driven construction interactions."
todos:
 - id: geometry-types
   content: Add CadGeometry (vertex/edge/wire/face/shell/solid) types to cad/rs/lib.rs matching fixture schema
   status: completed
 - id: brep-import
   content: Replace cad_document_pane_objects's centroid hack with real kernel topology import + per-object solid/surface/curve handle resolution in cad/plugin/rs/lib.rs
   status: completed
 - id: remove-placeholder-mesh
   content: Delete TYPOLOGY_MESH_URLS + bogus default_document mesh_url; tessellate real per-object meshes
   status: completed
 - id: curve-surface-render
   content: Render curve-only and surface-only primitives (Structure-Classic walls/columns, Energy baseplate) as tube/panel meshes
   status: completed
 - id: raycast-pointer
   content: Add raycast-to-plane + worldPointerDown dispatch with 3D position in world-3d-host.tsx, gated on engagement.sessionActive
   status: completed
 - id: merge-pointer-command
   content: Merge engagementPointerDown into worldPointerDown handling in cad/plugin/rs/lib.rs; update tests
   status: completed
 - id: toolbar-buttons
   content: Add Box/External Wall/Slab/Column construction buttons to build_cad_play_toolbar
   status: completed
 - id: expand-catalog
   content: Port beam/wall/internal-wall/mushroom-column interactions+transformations needed by the shipped forest fixture
   status: completed
 - id: verify
   content: cargo test -p cad-plugin + manual dev-server smoke test on hexagonal-cut-concrete-forest-left example
   status: completed
isProject: false
---

# CAD Renderer Parity Restore

## Root-cause findings (confirmed by reading code, not assumed)

The `cad` module already migrated off the old React stack (`cad/renderer/react`, `cad/renderer/core` — both deleted, correctly, as part of the repo-wide move to Rust `PluginApp` + generic `World3dHost`/`os-shell.tsx`, see `PLUGIN-OS-ARCHITECTURE-REFACTOR` ticket). `cad/renderer/js/index.tsx` (7431 lines) is dead leftover code — it was only ever a dependency of the deleted `cad/renderer/react`, not a gap to revive. **The fix is entirely inside `cad/plugin/rs/*` (Rust) plus one shared frontend file, `framework/renderer/react/components/world-3d-host.tsx` — no new per-tech React code should be written**, consistent with the new architecture.

Three concrete, verified bugs:

1. **"Shape shows a wrongly-positioned mesh, no brep"**: `cad/asset/play/hexagonal-cut-concrete-forest-left.model.json` embeds full BREP topology per pane (`geometry.{vertices,edges,wires,faces,shells,solids}` with real plane/line primitives — confirmed by inspection, e.g. model 0 has 71 vertices / 126 edges / 57 faces / 1 solid). `cad_document_pane_objects` (`cad/plugin/rs/lib.rs:307-360`) throws this topology away and only computes a crude centroid/bbox from vertices (`object_origin_from_vertices`, `object_extent_from_vertices`). Every object then gets a `mesh_url` from the static `TYPOLOGY_MESH_URLS` table (`lib.rs:71-80`) which maps **every** typology to the **same single placeholder GLB** (`/mesh/hexagonal-cut-concrete-forest-left.glb`) — so a pane with N objects renders N overlapping copies of the _entire_ forest mesh, each at a different centroid. For the Shape pane the one object's typology is `""` (empty — it's the raw imported solid), which doesn't match any `TYPOLOGY_MESH_URLS` entry either, so it falls through to `typology_brep_mesh` with `solid_handle: None`, producing a generic procedural **box** sized to the bbox — never the real hexagonal geometry.

2. **"No energy/BIM/structural models shown"**: the fixture's Energy pane object uses `primitives.surface` (a single planar face, not a solid) and Structure-Classic objects use `primitives.curve` (centerline wires) — never `primitives.solid`. `resolve_object_mesh_url`/`TYPOLOGY_MESH_URLS` only covers `energy.energy.externalwall`, `structure.structure.onewayreinforcedconcreteslab`, `structure.structure.reinforcedconcretecolumn` — typologies like `energy.energy.baseplate` and `structure.structure.reinforcedconcreteinternalwall` (the majority of the Structure-Classic objects) aren't in the table at all, so `resolve_object_mesh_url` returns `None` and those objects render nothing.

3. **"No interactions work"**: `cad/plugin/rs/interaction.rs` has real, tested statechart logic for 4 interactions (box/external-wall/slab/column) that call the real brep kernel on commit. But nothing in the frontend can ever drive it to completion:
   - `framework/renderer/react/components/world-3d-host.tsx`'s `handlePointerDown/Move/Up` (`:1110-1203`) only ever dispatch `worldPick`/`worldSelect`/`setSelection` (2D screen-space marquee selection) — there is **no raycast against a work plane and no dispatch of a 3D point** anywhere in the shared host.
   - `cad/plugin/rs/lib.rs:2716` implements `"engagementPointerDown"` (expects a `position` arg) to drive `apply_event(session, "pointer.down", point)`, but no frontend code ever sends that command.
   - `cad/plugin/rs/lib.rs:2756` explicitly no-ops the command name that _is_ generically available: `"noop" | "worldPointerDown" => return Vec::new()`.
   - The play toolbar (`build_cad_play_toolbar`, `lib.rs:1880-1985`) only exposes View/Save/Transfer buttons — no button starts a box/wall/slab/column interaction; only the generic engagement-bar `possibleEngagements` chips (from `window_engagements`) can start a session, and REPL text (`parse_repl_line`) can only fire keyword transitions (`s`, `set.height 2.5`) — it has no way to type/click a 3D point.
   - Net effect: a session can be _started_ (via engagement-bar chip) but can never receive its first point, so it appears completely dead.

## Fix plan

### 1. Real per-object brep from embedded topology (fixes bug 1 + unblocks 2)

- In `cad_document` (`cad/rs/lib.rs`) add geometry types mirroring the fixture schema (`CadVertex`, `CadEdgeCurve{Line}`, `CadWire`, `CadFace{planar}`, `CadShell`, `CadSolid`) and a `CadGeometry` container, parsed alongside `objects[]` in each pane's fixture JSON.
- In `cad/plugin/rs/lib.rs`, replace `cad_document_pane_objects`'s throwaway vertex-centroid logic with a real importer: build each vertex via `BrepkitKernel::vertex_sync`, each line edge via `line_curve_sync`, each wire via ordered edge traversal (`polyline_wire_sync`/`face_from_wire_sync`), each planar face via `planar_face_from_wire_sync`, and shells/solids via `sew_faces_sync`. Cache the resulting `GeometryHandle` per fixture geometry id (`...-solid-313` etc.) in the kernel's registry, and resolve each object's `primitives.{solid|surface|curve}` reference to that handle, storing it as `solid_handle` (extend `CadObject`/`CadPrimitiveSlot` if a surface/curve handle needs its own field).
- Set object `origin: [0,0,0]` (fixture vertex positions are already world-absolute) and tessellate the _real_ handle per object (extending `typology_brep_mesh`/`resolve_object_mesh_url` to prefer a per-object tessellated mesh over `TYPOLOGY_MESH_URLS`). Delete `TYPOLOGY_MESH_URLS` and the `default_document()` box's bogus forest-GLB `mesh_url` (`lib.rs:520`) — both are placeholder hacks superseded by real geometry.
- For `curve`/`surface`-only objects (Structure-Classic walls/columns as centerlines, Energy baseplate as a bare face), render the wire as a thin tube and the face as a thin double-sided panel (no thickness data exists in the fixture to extrude by) — visually correct "wireframe/surface" BIM representation instead of invisible.

### 2. Fix interaction plumbing end-to-end (fixes bug 3)

- In `framework/renderer/react/components/world-3d-host.tsx`, extend `handlePointerDown`/`handlePointerUp` (or add a dedicated handler gated on `engagement.sessionActive`) to raycast the pointer against the active construction plane (ground `z=0`, consistent with existing camera-up `[0,0,1]`) and dispatch a 3D point. Reuse the existing `worldPointerDown` command name (stop treating it as a permanent no-op) with payload `{ pane, position: [x,y,z], shiftKey, ctrlKey, metaKey }`; derive `pane` from the surface id suffix (e.g. `cad.play.scene3d/shape` → `shape`), matching `cad_pane_id_from_suffix`.
- In `cad/plugin/rs/lib.rs`, merge `"engagementPointerDown"` into the `"worldPointerDown"` arm (single source of truth per AGENTS.md — no duplicate command names) and update tests accordingly.
- Add toolbar buttons for the 4 already-implemented interactions (Box/External Wall/Slab/Column) to `build_cad_play_toolbar`, each dispatching `engagementPossibleSelect` (or directly starting the session), so interactions are discoverable without relying solely on the engagement-bar text chip.
- Verify the REPL `set.height <n>` / point-click / commit round trip end-to-end for at least the Box interaction via a manual dev-server smoke test (draw a box in the Shape pane).

### 3. Expand interaction/transformation coverage for the shipped fixture (parity for "hexagonal column" example)

- The forest fixture's typologies (`building.building.beam`, `.wall`, `structure.structure.reinforcedconcreteinternalwall`, the concrete "mushroom/hexagonal column" family under `cad/asset/modelDefinition/aec.building.concrete/action/constructMushroomColumn.json` etc.) have **no** Rust interaction/transformation counterpart yet — only box/external-wall/slab/column exist in `INTERACTION_CATALOG`/`CAD_TRANSFORMATION_SPECS`.
- Port the remaining premigration interactions needed to construct/derive every typology actually present in `hexagonal-cut-concrete-forest-{left,right}.model.json` (beam, wall, internal wall, mushroom column) into `cad/plugin/rs/interaction.rs` + `transformation.rs`, following the existing statechart pattern (each is a small, self-contained `match` arm addition — no new architecture needed).
- Broader spatial.shape general-purpose tooling (circle/arc/loft/sweep/fillet/chamfer/boolean ops, full FEM typologies) is a much larger, open-ended catalog (~40 JSON-defined actions under `cad/asset/modelDefinition/spatial.shape/`) not exercised by the shipped fixture; track as a explicit follow-up rather than blocking this ticket, and note it in the ticket close-out summary.

## Files to touch

- [cad/plugin/rs/lib.rs](cad/plugin/rs/lib.rs) — geometry import, mesh resolution, toolbar, `worldPointerDown` merge.
- [cad/plugin/rs/interaction.rs](cad/plugin/rs/interaction.rs) — new interaction catalog entries.
- [cad/plugin/rs/transformation.rs](cad/plugin/rs/transformation.rs) — new transformation coverage.
- [cad/rs/lib.rs](cad/rs/lib.rs) — `CadGeometry`/vertex/edge/wire/face/shell/solid types on `CadScene`/`CadObject`.
- [framework/renderer/react/components/world-3d-host.tsx](framework/renderer/react/components/world-3d-host.tsx) — raycast-to-plane + `worldPointerDown` dispatch (shared by all plugins with engagement sessions, e.g. `procedural`, `shooting`, so verify it doesn't regress their marquee-selection behavior).

## Verification

- `cargo test -p cad-plugin` (existing + new tests for geometry import and new interactions).
- Manual dev-server pass on the `hexagonal-cut-concrete-forest-left` example: Shape pane shows the real hexagonal-cut solid; Building pane shows distinct slab/column/beam/wall meshes at correct positions; Energy pane shows the baseplate panel; Structure-Classic pane shows wall/column/slab wireframes+panels; drawing a new Box via toolbar+click works end to end.
