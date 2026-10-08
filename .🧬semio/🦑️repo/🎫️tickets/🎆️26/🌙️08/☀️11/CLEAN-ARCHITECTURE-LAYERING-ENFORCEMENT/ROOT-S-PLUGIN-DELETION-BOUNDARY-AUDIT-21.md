# S Plugin Deletion Boundary Audit 21

Read-only current authored-source inspection, 2026-10-08. No source edits, test runs, runtime invocation, cleanup, Git mutations or worktrees. Bounded source inventory used rg; copied/ticket/node_modules sources excluded from findings. Complete captured bodies and hashes: `generated/root-s-plugin-audit-21/complete-source-bodies.json`. This is a focused positive witness, not a claim of complete S deletion safety.

## Confirmed executed static boundary

`✏️s/🔨️modules/🏗️fem/⚙️engine/◻️2d/🕸️meshing/🦀️.rs:10–11` imports `SemioPoint3`, `SemioMesh`, `SemioMeshSnapshot`, `SemioPrimitive`, `SemioTopology` from `semio_s_artifact_stdio_semio`. `build_semio_mesh_snapshot` at line 173 constructs these types and exposes the foreign snapshot as its crate-visible return. This is real compiled code: FEM 2D artifact root `🦀️.rs:32–39` mounts this exact source as `fem2d_engine::meshing`, and OBJ and STL serializer leaves call it at line 27. Its document input also comes from the Specific FEM artifact (`crate::{Fem2dSnapshot,FemElement}`), so physical module placement does not establish independence.

The sibling 3D meshing source has the same foreign imports and return at line 250; its OBJ and STL serializer leaves each call it at line 27. These are separate complete execution slices; converting only 2D must not claim 3D fixed.

The FEM 2D crate Cargo manifest lines 44–49 unconditionally depends on stdio csv/json/md/obj/semio/stl, with semio conversion-mesh feature. Thus deleting stdio or one relevant artifact prevents the remaining FEM artifact building before neutral consumer registration can occur. This is not merely display text or an unused repository-directory field. Explicit integration leaves may compose installed providers, but their owning core crate must not require absent providers; optional install composition must own these dependency edges.

## Smallest next complete slice

Select the 2D extruded region surface to OBJ/STL slice. Its only direct callers are the OBJ `🔖️3.0/✳️any` and STL `🔖️ascii/✳️any` leaves under `🚪️io/📤️export/🧵️serializers/🗿️artifacts`. Preserve the full source bodies, not just the imported lines. Preserve triangulate → extrude_tri_mesh → split_to_tets → boundary_faces, per-region id and thickness, winding, bar/beam empty behavior, and current failed-triangulation behavior until a separately specified error change.

Existing neutral kernel contracts are `crate::mesh::PlanarDomain`, `MeshOpts`, `TriMesh2`, `VolumeMesh { points: Vec<[f64;3]>, cells: Vec<Cell> }`, and `boundary_faces(&VolumeMesh) -> Vec<[u32;3]>` in `✏️s/🔨️modules/🏗️fem/⚙️engine/🕸️mesh/🦀️.rs`. These already represent geometry without stdio. Use those as the kernel boundary; keep FEM document traversal with the FEM artifact and foreign snapshot construction/encoding in separately installed Specific conversion extensions. A schema-first neutral region identity plus volume/surface aggregate still needs declaration if desired: no such exact named aggregate was confirmed in this audit. Do not pretend `VolumeMesh` alone carries region identity. Framework 3D `MeshTransfer` exists but stores f32 positions; blindly substituting it would narrow existing f64 export precision.

Receiving integration must include both serializers, artifact module assembly, IO declaration, Cargo edges and tests. Moving the function alone leaves deletion failure. Foreign serializer types must disappear from the independently installable FEM core compile closure, not be reexported or wrapped in compatibility aliases.

## Validation ownership and limitations

Existing owning Rust test is `🌐️any/🚪️io/🧪️tests/🔬️unit/🦀️.rs`: `geometry_exports_are_real` checks STL prefix and successful OBJ; declaration tests assert seven entries and exact foreign kinds. Preserve these assertions/corpus and relocate their integration ownership if composition changes. These assertions are too weak to certify numeric geometry equivalence: new portable language-agnostic vectors should compare positions, triangle winding/counts, bounding box, volume and empty/nonempty region outputs through both exports and a third-party parser. No current portable third-party OBJ/STL oracle was verified; nearest FEM plugin oracle manifest explicitly registers no oracle. Do not claim parity or runtime success.

Existing FEM 2D Rust project has `check` and `test` targets invoking its own `📜️script.ts`; root launch has `@semio-tech/fem-2d:test` at line 28527 and snapshot SQLite targets, while generic launch command `bun nx run ${input:projectTarget.canonical-architecture}:canonical-architecture` exists at line 72154. Validate exact target availability before choosing a new route; no new route registered here.

## Canonical enforcement scope

Taxonomy authority: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`. Current `dependencyDirections.rules` are `io-renderer-independent` and `framework-no-products`. Cargo rules are `cargo-framework-no-implementation-role`, `cargo-modules-no-plugins`, `cargo-plugins-no-extensions`, `cargo-plugins-no-artifacts`, `cargo-framework-no-products`. `cargo-modules-no-plugins` targets manifests whose source owner is under S modules; mounted source compiled inside an artifact can escape that owner classification. A parser-backed source import/type dependency witness is necessary alongside Cargo checks. Area-layer comment defines deleting implementation must leave repo-wide/framework correct; S-specific parent-child deletion needs the additional user rule, not an invented existing taxonomy rule.

S root Cargo membership explicitly excludes plugin/dev trees and names neutral module crates; no specific plugin Cargo edge was found there. S package workspace wildcard is discovery, not a hardcoded installed-plugin runtime factory. Broad search of authored S module TS/Rust/package manifests found no direct plugin path import besides the confirmed Rust foreign crate imports; this bounded result does not certify computed factories globally.

Concurrent peer identities were not inspected; no source mutation was attempted and no work was stopped on account of others.
