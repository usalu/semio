# Fem Feature Complete — Per-Part Gate Checklist

Gate: real formulation, numeric worked-example test, doc-comment updated where a hook stays None, no leaked external types.
Verified: `CARGO_TARGET_DIR=.../target-fc cargo test -p fem_core -p fem_2d -p fem_3d -p fem-plugin` — 172/172 passed (92 fem_core + 31 fem_2d + 29 fem_3d + 20 fem-plugin), zero warnings.
Also verified: `bun nx run @semio-tech/fem-2d-rs:wasm` + `fem-3d-rs:wasm` (both release wasm32-wasip2 builds succeeded), `cargo build -p fem-plugin --target wasm32-unknown-unknown` (succeeded), `bun nx run @semio-tech/framework-renderer-react:test` (170/170 passed, no regressions).

## fem_core — verified: 92/92 passed
- [x] mass(): Tri3Cst / Tri6Lst / Quad4 / Quad8 / Tet4 / Hex8 / PlateDkt / ShellFacet3 — row-sum + modal tests
- [x] geometric_stiffness(): Bar2 / Bar3 / plane continuum / Tet4 / Hex8 / ShellFacet3; PlateDkt documented None
- [x] equivalent_nodal_loads(): Bar2 / Bar3; continuum documented None (trait doc updated)
- [x] analyses::nodal_averaged_scalar — patch test exact + shared-node strictly-between test
- [x] mesh::boundary_faces — extruded-box surface area test + outward-winding spot check
- [x] buckling axial-null-space regularization comment updated

## fem_2d — verified: 31/31 passed
- [x] density plumb to Tri3Cst; native region self-weight equilibrium test (region_self_weight_via_solve_all_matches_total_mass_times_gravity)
- [x] RegionMesh.node_ids + fem2d_nodal_von_mises

## fem_3d — verified: 29/29 passed
- [x] FemSolid schema + FemMaterial.nu + FemLoad enum (nodal/memberUdl/area)
- [x] SetSolid/RemoveSolid ops with true inverses — round-trip test
- [x] solid meshing in resolve_geometry + area-load tributary translation — pressure equilibrium test (exact, mesh-independent)
- [x] fem3d_mesh_preview + fem3d_nodal_von_mises
- [x] example fixture: solid + pressure case + kind-tagged loads

## fem-plugin — verified: 20/20 passed
- [x] 2D reaction value labels
- [x] 2D nodal-averaged banded contours (marching-triangle Sutherland-Hodgman clip, both apps consume nodal averaging)
- [x] 3D oriented member prisms (quaternion align-to-direction + roll composition)
- [x] 3D solid surface mesh + vertex-color von Mises contours + legend caption
- [x] 3D modal/buckling/static captions via ui_stack_vertical

## framework (root fix) — verified: 170/170 passed (no regressions)
- [x] react renderer WorldMeshData.colors passthrough + vertexColors material

## tooling
- [x] fem/core/rs + fem/plugin/rs project.json/script.ts (test targets)
- [x] launch.json 🧪️test🏗️fem entry

---

# Round 2 — End-to-end runtime fix (apps booted empty, features unreachable from UI)

Gate: same as above. Verified: `CARGO_TARGET_DIR=.../target-fc cargo test -p fem_core -p fem_2d -p fem_3d -p fem-plugin` — 189/189 passed (92 fem_core + 33 fem_2d + 31 fem_3d + 33 fem-plugin), zero warnings. Live-verified in the browser via `fem2d-react-dev`/`fem3d-react-dev` dev previews (screenshots): 2D model+results render at boot with a visible deformed shape, banded von Mises contour + legend, reaction labels, moment diagram, and a correctly-scaled modal mode shape ("Mode 1: 451.346 Hz"); 3D model+results render at boot with the oriented column member, grid, and a "Case: dead" caption confirming the full solve pipeline (some screenshots were disrupted by a concurrent session's unrelated hot-reload churn on shared frontend files — not a regression from this work).

## P1 — setActiveExample (root cause of the empty-app bug)
- [x] `SetDocument` op + diff short-circuit/absorb + true inverse added to `Fem2dOp`/`Fem3dOp` (mirrors `gis/plugin/rs`)
- [x] `setActiveExample` handler + `.operation` registration + action args + EN/DE labels in both apps
- [x] tests: loads default fixture / empty on unknown id / declared as `ActionKind::Operation` (both apps)

## P2 — example fixtures rebuilt to demonstrate every feature
- [x] 2D: L-frame (column + 2-segment beam) + loaded slab region; `dead` (self-weight + memberUdl ×2 + area load), `live` (nodal), `uls` combination; illustrative buckling column
- [x] 3D: column + cantilever beam + loaded solid; `dead` (self-weight + memberUdl + area load), `live` (nodal), `uls` combination; illustrative buckling column
- [x] fixture-coupled tests updated (node/element counts, case ids, nodal von Mises non-empty, buckling factor finite >1)

## P3 — deformation scale unification
- [x] static views honor `doc.analysis.deformation_scale` (hardcoded `DEFORM_SCALE_2D`/`DEFORM_SCALE_3D` removed)
- [x] modal/buckling views use `normalize_mode_shape` + `fem2d_model_extent`/`fem3d_model_extent` × `MODE_SHAPE_AMPLITUDE_RATIO` (deterministic, mass/Kg-normalization-independent amplitude)
- [x] `FemAnalysisSettings.deformation_scale` docstrings updated (static-only)

## P4 — complete UI action set
- [x] `addRegion` (2D), `addSolid` (3D), `addAreaLoad`, `addMemberUdl` (new in 3D), `addLoadCase`, `addCombination`, `setSelfWeight`, `setAnalysisSettings`
- [x] `removeSelection` extended to regions/solids/combinations
- [x] shared `fem2d_resolve_load_case`/`fem3d_resolve_load_case` helper; `caseId` arg added to existing load actions
- [x] tests for every new action (op shape, targeting, partial-args preservation)

## P5 — dev hot-swap watcher
- [x] `pluginWatchRoot` helper: non-framework plugin crates watch their top-level app directory (covers dependency crates + example fixtures), framework-hosted crates keep the narrow watch
