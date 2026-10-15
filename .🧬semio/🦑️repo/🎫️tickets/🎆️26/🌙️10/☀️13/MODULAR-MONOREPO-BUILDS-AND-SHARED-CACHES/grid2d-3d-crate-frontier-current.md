# Grid2d → 3D Crate Frontier (Read-Only Audit)

Audit date: 2026-10-15. No build, compiler invocation, Source modification, git mutation, or cap change was performed. Concurrent BREP/3D edits were inspected only as the current physical snapshot; their completion is not assumed. Original14350 evidence comes from `rust-mutation-collector-current.md` and is not re-executed here.

## Concrete Consumer Path

The actual binary manifest is `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/🏭️bridge/📦️packages/🦀️rust/Cargo.toml`; its bin is `../../🦀️.rs`. Its dependency on `semio-s-artifact-wfc-grid2d` resolves to `✏️s/🔌️plugins/🀄️wfc/🗿️artifacts/🔲️grid2d/📦️packages/🦀️rust/Cargo.toml`. That artifact declares unconditional `semio-framework-os-flow` and `semio-framework-os-infinite` dependencies. Their workspace paths are defined in the artifact-root `Cargo.toml` at lines 53 and 56.

Verified directional paths:

- bridge → grid2d artifact → OS flow → root 3D.
- bridge → grid2d artifact → OS infinite → root 3D.
- OS flow also → OS infinite → root 3D.

Flow manifest: `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust/Cargo.toml`, line 45, unconditional `semio-framework-3d`.
Infinite manifest: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/Cargo.toml`, line 48, unconditional renamed `semio_framework_3d` package dependency.

**Actually used 3D symbols on these edges: none found.** Explicit recursive searches of both physical OS module trees, including an unrestricted `rg -uuu` pass over Rust files, found no `semio_framework_3d` Rust reference. Explicit search of the grid2d Rust tree found neither `semio_framework_os_flow` nor `semio_framework_os_infinite`. These are manifest reachability edges rather than symbol demand from this consumer. Rust compile-time dead-code elimination does not remove Cargo dependencies or exempt their Source from admission.

The bridge itself reads exactly `Grid2dMutation` and `Grid2dSnapshot` through the standard/any schema path and the `Mutation<Snapshot>::DESCRIPTORS` associated constant. It uses the OS protocol Mutation trait plus `inventory_for_command`, `replication_inventory_leaves`, `MutationInventoryContribution`, `MutationInventoryCoordinate`, `MutationInventoryError`, `MutationInventoryLeaf`, and `MutationInventoryProgress`. No 3D symbol occurs in the binary. Its contribution excludes editor/viewer inventory coordinates, but this does not gate those modules out of the artifact crate.

## Actual Root 3D Source Boundary

Manifest: `🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust/Cargo.toml`; `[lib] path = "🦀️.rs"` resolves to the adjacent `🦀️.rs`, not a module-root file (none exists at `🧊️3d/🦀️.rs`). The lib unconditionally mounts:

| Module | Physical file | Direct file bytes observed |
|---|---|---:|
| mesh | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh/🦀️.rs` | 84,644 |
| rigid | `🧰️framework/🔨️modules/🧊️3d/🌀️rigid/🦀️.rs` | 8,774 |
| collision | `🧰️framework/🔨️modules/🧊️3d/🧿️collision/🦀️.rs` | 29,717 |
| inertia | `🧰️framework/🔨️modules/🧊️3d/🛞️inertia/🦀️.rs` | 6,047 |

These four immediate files alone total 129,182 bytes; this is deliberately **not** a complete admitted Source census. Mesh additionally mounts modeling, quality, and surface implementations. Only test-support is `cfg(test)` at root. Root has no feature table or production module cfg gates. Collision depends on `crate::rigid::{Isometry3, Point3, Vector3}`. Mesh quality depends on `crate::inertia::{principal_inertia, to_centroid, InertiaTensor, PrincipalInertia}`. Mesh modeling directly uses BREP-vector `Pnt3`, `Vec3`, `Affine3`, and `Mat3`. The root manifest currently depends on BREP-vector, mesh-engine, pack-json, value, and value-derive; parry3d/serde/serde_json/trace are dev dependencies.

BREP is already separate in this snapshot: root does not mount its full engine/queries. BREP analysis still has a real reverse dependency on root solely visible here as `semio_framework_3d::inertia::{combine, principal_inertia, InertiaPiece, PrincipalInertia}` in `🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🔎️analysis/🛞️mass/🦀️.rs`. That is an honest future inertia ownership cut, not a reason for grid2d to compile mesh/collision.

## Smallest Honest Cut

1. Remove the unused grid2d artifact manifest dependencies on OS flow and OS infinite after checking macro-generated paths in its dispatch code. Source grep found no named use; this is the narrowest consumer-local closure reduction and preserves every inventory/schema input.
2. Independently remove the unused 3D manifest dependencies from OS flow and OS infinite. Their mounted roots (`…/📦️packages/🦀️rust/🦀️.rs`) use real modules through physical path mounts, but neither tree names this crate. This fixes the unnecessary canonical edges even for other consumers that legitimately need flow/infinite.
3. Another unused-looking edge exists in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/Cargo.toml:45`; recursive renderer Rust search found no root-3D symbol. Treat that as a separate check before claiming the entire native closure loses 3D.

No new crate is needed for this specific unused-edge cut. If real inertia consumers remain after concurrent refactoring, give inertia its own canonical crate at the existing `🛞️inertia` owner and have mesh quality and BREP analysis depend inward on it. Collision → rigid remains an explicit separate directional relationship; do not move it into an inertia consumer or add a compatibility root barrel.

Verification needed after coordinated edits: inspect actual macro expansions/declared generated dependencies, refresh only affected lock ownership through the existing owned command, and re-run the genuine grid2d collector with locked selectors and existing bounds. This audit does not claim a complete closure, runtime success, compiler success, or that removing the three direct edges alone resolves every transitive path. It identifies concrete unnecessary reachability that should be removed before splitting a root crate merely to fit this consumer.


## Owner Implementation Successor

Root removed the five audited unused Cargo edges: grid2d to flow/infinite, flow to root 3D, infinite to root 3D, and wgpu to root 3D. These were five declarations in four physical manifests. No module Source, input membership, Source budget, feature gate or implementation substitute changed. The native producer's preceding actual Source-boundary refusal is retained as the pre-change failure. Owned offline lock refresh 17541 is running; the genuine locked grid2d successor follows only after it settles. This does not yet claim that grid2d, flow, infinite or wgpu compiles with the changed graph.


Owned offline lock refresh 17541 passed in 8.5 seconds and resolved 201 packages, compared with the preceding 549-package graph. The normal locked grid2d successor 51937 then ran for 31.9 seconds and genuinely refused `semio-framework-actor` at the physical Source-byte gate before compiler allocation. The earlier root-3D frontier was removed from this selected graph; the Actor frontier is separate and remains. No compiler or inventory result is claimed. Actor's root file measures 260,458 bytes; the read-only native production candidate inventory is approximately 423,898 bytes across 18 files, not a compiler census. Storage execution now owns an acyclic actual Actor decomposition under the unchanged admitted limits. Root will refresh the affected lock and repeat this native collector only after its architecture Source checkpoint settles.

The exact settled Root lock and failed native transcripts were deleted after this record. No input, shared output directory, foreign process or unknown artifact was removed.
