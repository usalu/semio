# Native IFC4x3 Exchange Implementation

## Implemented Route and Exact Registration

The new IFC4x3 Rust `native_exchange` module provides actual `ArtifactComposer` routes from IFC4x3 native packs to glTF2.0 native packs and back. The IFC artifact declaration registers both through normal `.composers(...)`; the existing central `composer_identity_claims` mechanism validates the foreign output claim. The IFC artifact definition declares its glTF dependency and exact foreign route `17:s.stdio.ifc@4x3/*18:s.stdio.gltf@2.0/*`. The reverse composer uses IFC's existing ADD2 dialect capability. No bypass capability or IFC4 strict STEP ingress was introduced.

Controlled framework IO entries are registered by the normal IFC contribution `assembly()` after its definition/declaration validates, and by the existing explicit root `io_registry::register()` boundary. Exact call chain inspected: `ContributionRegistry::artifact_assemblies` invokes `(contribution.assembly)()` in `stdio/📇️registry/🦀️.rs`, which reaches IFC `assembly()`, `runtime_assembly("ifc",...)`, then `io_mechanism::io_register(native_exchange::io_entries())`. The current old `ArtifactDeclarationBuilder` has no field/method for `IoEntry`; it does have the separate composer declaration. Registration of an identical static IO entry is idempotent under the existing registry mechanism. Registration currently occurs during assembly rather than the later general plugin transaction; central transactional declaration support for these old-root IO entries remains a framework boundary.

Three controlled entries are concrete:

- IFC4x3 native artifact → glTF2.0 **native artifact pack**, lossy.
- glTF2.0 native pack or checked natural GLB/JSON → IFC4x3 **native artifact pack**, lossy. GLB sniffing reports the source glTF dialect. Framework deserializer publication uses the original target native encoder.
- IFC4x3 native artifact → framework binary carrier containing natural **GLB2.0 bytes**, lossy. The final published bytes are allocated by the original caller's encoder; a zero output allocation budget refuses publication.

Root registry helpers include the new composers as well. `native_host.rs` exercises normal declaration/assembly discovery and controlled route publication; `native_exchange.rs` tests actual erased composer native pack outputs in both directions.

## Geometry and Refusal Contract

Native export reuses the shared IFC schema geometry projector, which returns source-unit geometry and evaluated local placements/context WCS. Exactly one project and one valid SI metre length unit are required; every supported SI prefix, including the official example's millimetres, converts to metres. Conversion-based length units, missing/ambiguous units, complex entity evaluation and explicit map/CRS operations are refused. No implicit unit scale is supplied. Source `(x,y,z)` becomes glTF `(x,z,-y)` after scaling. Float32 overflow refuses atomically. Product and representation IDs are string extras. Any shared projector issue, cancellation, limit or absence of geometry refuses the whole destination instead of publishing partial meshes.

Import uses the target artifact's actual `GltfSnapshot`, native pack codec, native GLB/JSON reader and shared accessor byte reader. It accepts selected static scene triangles with dense float32 POSITION and dense unsigned integer indices, or implicit sequential indices. It evaluates finite affine matrix/TRS hierarchy transforms and reflected winding, then constructs real ADD2 project/context, SI units, Cartesian point lists, triangulated face sets, representations and proxy entities in metres/Z-up. It does not manufacture BIM families, engineering sections or other authored semantics.

The existing glTF `validate_document` only checks a nonempty version, so this implementation additionally checks exact2.0, whole node graph missing children/cycles/multiple parents, unique parentless scene roots, bounded depth/work, finite/unit-quaternion affine transforms, accessor type/density/count/offset/stride/component alignment and declared buffer-view extents, unsigned integer index range and triangle topology. Animation, skin/morph deformation, glTF extensions and unresolved external buffer data are refused. Materials, normals, hierarchy, geographic metadata and authored/analytical information omitted by projection appear in explicit loss diagnostics. Default/selected scene geometry is the supported projection.

## Source-Owned Validation and Control

`step.rs` now exposes `validate_ifc4x3_graph_controlled`, a small wrapper around the existing graph validator, so geometry conversion does not allocate or format complete STEP text merely to validate a graph. Export/import use that interface with a4million-value work ceiling. Entity/unit, geometry, vertex, triangle, primitive, node graph and scene-root loops expose caller progress/cancellation. The graph operation refuses beyond1million entities; geometry limits use the shared projector's2million work,200000vertices,400000triangles and depth64. Import also caps64000node visits and64MiB of supplied buffers. GLB framing verifies version, complete declared length, aligned JSON/BIN extents, one embedded buffer and padding.

The callback-controlled APIs do not create a fresh cancellation authority. Controlled native pack receiving/publication calls the original framework decoder/encoder. Pure composer APIs themselves have no caller-control parameter; their wrappers therefore use fixed bounds and cannot promise host-owned cancellation.

Allocation/latency boundary: domain projection vectors, graph construction, shared geometry evaluation internals and existing synchronous glTF JSON/GLB/accessor codecs still have temporary allocations outside a resumable original-wallet cursor. GLB/JSON and accessor stages have bounded admission and checks before/after, but cannot be interrupted inside those existing codec calls. The final raw GLB owned bytes use `NativeEncodeControl.copy_bytes`; this necessarily leaves the codec's temporary byte vector alive until the handoff completes. These routes are **not** proof of strict per-step allocation, funded temporary retirement or bounded cursor latency. Do not claim full owned-job conformance from checkpoint presence.

## Files

Production: new `any/🚪️io/native_exchange.rs`, `native_gltf.rs`; a graph-validation wrapper in `step.rs`; standard Rust module/dependency additions; minimal IFC artifact root registration and definition capability/dependency additions. Existing TypeScript facade, editor, MVD and build infrastructure were preserved.

Tests: `any/🚪️io/🧪️tests/native-exchange.feature`, `native_exchange.rs`, `native_host.rs`, `native-oracle.test.ts`. Source fixtures are existing immutable official `triangulated.ifc` and Khronos `box.glb`, with adjacent source URLs, commits and SHA256 records unchanged. No new runtime third-party dependency was introduced.

## Validation Actually Performed

- `rustfmt --edition2024 --emit stdout` parsed both new native production modules and new Rust test modules successfully; this is syntax validation, **not** Rust type checking or execution.
- `cargo test -p semio-s-artifact-stdio-ifc-4x3 --lib native_exchange --no-default-features` was rejected before compiler version probing by the repository's authentic compiler-ingress policy: `Original compiler ingress requires the repository launcher and authentic enforcing store`. No enforcement bypass, compiler wrapper change, build-system edit or unapproved launcher was used. Currently available tools expose no repo task/run route. The existing documented direct-read/Bun fallback remains in force for noncompiler checks.
- Focused Bun run: **5passed,2skipped,0failed,27assertions** (existing glTF exchange tests plus new native-output oracle tests). Native-output oracle tests correctly skip when `SEMIO_IFC_NATIVE_ORACLE_OUTPUT` is absent; no native Rust output has yet been generated or independently validated.
- Final full exchange Bun run with existing IfcOpenShell oracle results after the shared identity update: **62 passed, 2 skipped, 0 failed, 265 assertions**, across 64 tests in six files. Earlier concurrent edits briefly caused an `invalidSchema` failure and one module-load error; the settled re-run passed. The two skips remain the new native-output oracles, whose actual Rust outputs do not yet exist.

Prior independent IfcOpenShell0.8.3/OpenCascade and three.js results remain valid for the existing TS implementation and source fixture expectations:8vertices/12triangles, IFC SI bounds `(.5,-.5,0)`–`(1.5,.5,2)`, glTF bounds `(.5,0,-.5)`–`(1.5,2,.5)`. Those results are **not** substituted for native execution.

## Native Execution and Independent Oracle Reproduction

Use the repository's authorized compiler route when available. Set `SEMIO_IFC_NATIVE_ORACLE_OUTPUT=<ticket>/🗑️generated/native-exchange`, then run the new native exchange Rust tests and root `native_exchange_tests`. The erased native composer test writes `native-triangulated.glb` and its reimported `native-triangulated.ifc` below that output directory.

Run the retained `ifc-oracle.py` test interface with bundled Python3.12, test-only IfcOpenShell0.8.3 module directory `<ticket>/🗑️generated/ifc083`, native emitted IFC input, and output directory `<native-output>/oracle`. Then run `bun test <any>/🚪️io/🧪️tests/native-oracle.test.ts` with the same native-output environment. It checks native-produced GLB through three.js GLTFLoader and actual native-produced IFC through independent IfcOpenShell metadata/geometry. Generated native oracle inputs and canonical outputs remain separate. Do not enable these tests using TS-produced files under the native names.

## Remaining Scope

Native compilation/type checking, native host execution, original-control accounting/retirement across every temporary domain allocation and the native-output differential oracle remain unverified/unfinished until the authentic compiler route is available. The following audited implementation wave adds native BIM spatial and unloaded FEM domain routes; its exact admission and remaining receiving limitations are in `native-domain-exchange-implementation.md`. Geometry audit fixes are in `native-exchange-fixes.md`. Broad infrastructure alignment evaluation, unsupported geometry evaluators, general authored BIM/FEM parity and MVD/EXPRESS conformance are not claimed.
