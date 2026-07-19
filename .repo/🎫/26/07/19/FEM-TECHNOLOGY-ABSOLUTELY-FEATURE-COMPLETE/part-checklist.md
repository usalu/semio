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
- [x] launch.json 🧪test🏗️fem entry
