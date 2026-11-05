# Lowpoly Artifact Investigation

The shared geometry export path reads authored mesh_content instead of current managed mesh_state. Authored source is an independent field and mesh edits update managed state, so OBJ, STL, PLY, GLB, DWG, LAS and PNG exports can silently publish stale geometry or skip objects with no authored source.

A language-neutral JSON regression fixture defines a source triangle, independently edited managed positions, object transform, expected world positions and triangle indices. The Rust test compares shared export projection and independently imports GLB output through the existing third-party gltf crate. It covers stale, empty and invalid independent source text, plus present empty state overriding authored geometry.

Validation is queued through repo tools exec cargo -- test -p semio-s-artifact-lowpoly-lowpoly-io managed_geometry_exports_current_state_independently_of_authored_source. The public TypeScript package suite is also queued through repo run @semio-tech/lowpoly-lowpoly:test. Both respect machine capacity.

The fix selects present managed mesh_state before considering source text, including present empty managed geometry. Invalid managed state is reported as an export error. With no managed state, independent authored geometry retains its existing behavior. The shared test helper now verifies the managed content-addressed handle on managed imported objects.

Changed files: IO root Rust module; IO unit tests; IO neutral managed-export.json fixture; this report. IO diff whitespace check passes. Test launch preceded production edit, but machine capacity prevented the test from reaching execution before the fix; no red-run claim is made.

The direct registered Cargo regression command failed before compilation because the repository compiler ingress requires an authentic admitted compiler store. No enforcement was changed. A synthesized IO test target was then cancelled before starting after the runtime track added the admitted combined editor + engine + IO test target (@semio-tech/lowpoly-lowpoly-rs:test-runtime). The runtime track owns that shared execution. Public TypeScript package execution remains active behind the shared build dependency lock.

The unchanged public TypeScript artifact baseline suite was cancelled while still waiting for the shared build dependency lock, following root test selection to free six reserved CPU jobs for changed Rust paths and live boot. It did not execute tests and is not reported as passed. Native export checks are covered by the reusable composition target repo run @semio-tech/lowpoly-plugin:test-runtime.

Static verification confirms every geometry exporter (OBJ, STL, PLY, GLB, DWG, LAS, PNG) uses the shared fixed projection. OBJ, STL, PLY and glTF module descriptions were corrected to name current geometry as authoritative. Final IO git diff --check passes. No TypeScript or runtime dependency changes were made on this track. The runtime track owns the admitted native suite and any results are pending; this report makes no native test pass claim.

The reusable native invocation is repo run @semio-tech/lowpoly-plugin:test-runtime, not a ticket-script target. The earlier temporary shared target was superseded before execution.

## Import and UV Audit

OBJ, STL, PLY, GLB and DWG geometric imports all route through snapshot_from_parts, which constructs LowpolyMeshState::from_mesh, derives managed_mesh_child_handle, and stores the state alongside independent source. All seven geometry exporters use world_parts, which now correctly admits managed objects even when source is empty. Primitive add_primitive calls prepare_paint_mesh (unwrap_uv) before capturing state and deriving its handle; reload/sync preserves typed state and never rewrites independent source. The managed export fixture now includes a 90-degree Euler Z rotation plus nonuniform scale and translation; the independent GLB reader checks final POSITION and index accessors for stale, empty and invalid independent source cases.

## GLB Scene Import Defect and Fix

The prior lowpoly GLB importer iterated document.meshes directly. It ignored document.scene, scenes[].nodes, nodes[].children, mesh instances, node.matrix and translation/rotation/scale, and primitive.mode. A valid triangle mesh instanced twice under translated/rotated/scaled nodes therefore imported once at its untransformed coordinates; even a LINES primitive was silently grouped as triangles.

The existing owned API is semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::{GltfNode,GltfScene,GltfSnapshot,GltfAccessorType,GltfComponentType}, io::decode_accessor, io::import::decode_glb and io::export::encode_glb. GltfNode owns optional affine matrix [f64;16], translation [f64;3], quaternion rotation [f64;4], scale [f64;3], children and mesh indices. No runtime library is added or leaked. The existing gltf scene_geometry presentation helper silently skips malformed input and unsupported modes, so the Lowpoly importer keeps an error-reporting boundary rather than using that lossy helper.

The importer now selects the declared default scene (or the first scene when no default is declared); scene-less node documents traverse root nodes. It composes parent-world and local TRS/column-major affine matrices, preserves separate node instances and names, and caches each decoded source mesh. It refuses ambiguous matrix-plus-TRS, invalid transforms, out-of-range references, repeated parents/cycles, nontriangle modes, non-FLOAT POSITION accessors, invalid index types, partial triangles, invalid indices and unrepresentable coordinates. Geometry is lowered through the existing managed constructor.

The language-neutral gltf-scene.json fixture defines a triangle, a parent translation [10,20,30], a child using quaternion Z rotation + nonuniform scale + translation, a second instance using an explicit affine matrix, and an unselected second scene. Expected world coordinates are handwritten. The existing test-only third-party gltf crate independently loads the exported external-format GLB, exposes its node transform matrices and POSITION accessor, and its transformed points agree with both the fixture and the imported managed mesh. Modes 0,1,2,3,5,6 and a cycle are explicitly tested as refusals. Tests were added before the corresponding importer change; shared native validation remains with the runtime track.

Additional changed files: IO GLB importer module; IO neutral gltf-scene.json fixture. IO git diff --check passes after these changes.

## Primary glTF Reference

Verified node instantiation and reflected winding against the [Khronos glTF 2.0 specification, section 3.7.4](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#instantiation). A mesh can be instanced by multiple nodes, and each node global transform controls triangle winding. Baking reflected node transforms into a mesh therefore requires reversed face winding in Lowpoly.

Reflected scene instances now reverse face indices when the composed world linear determinant is negative. The neutral fixture includes a negative-scale instance with expected reversed [2,1,0] winding; the existing third-party gltf reader supplies independent local transforms, decomposed scale signs and accessor indices in the native test. A scene-less cycle is explicitly exercised and rejected even when it has no root nodes.

The retained ticket input artifact-oracle.mjs also validates both neutral fixtures using the already-installed third-party Three.js Matrix4, Quaternion, Euler and Vector3 implementations. It passed via repo tools exec --light node -- <ticket>/artifact-oracle.mjs: managed export world coordinates and all three scene instance coordinates/winding agree independently. This short fixture arithmetic check validates expected data; it is not a substitute for the pending native compiled tests.

Read-only rustfmt syntax probes were cancelled before execution because the registry admitted them through Cargo capacity. They provide no syntax or compilation result. Only the independent Three.js fixture check and IO whitespace check are confirmed at this stage.

## Mixed-Axis Euler Convention Audit

No implementation mismatch was found. world_parts delegates to inferences::apply_transform, which scales then rotates with euler_degrees_to_quaternion. Its quaternion formula matches Three intrinsic Euler XYZ (matrix Rx*Ry*Rz), not extrinsic sequential X-then-Y-then-Z rotation. Both Lowpoly editor Model and viewer Model import the same quaternion helper and emit it unchanged in the world3d scene; World3dHost::worldInstanceQuaternion constructs Three Quaternion directly from the four emitted components. The existing inferences neutral transform fixture also contains simultaneous XYZ quarter turns.

A short read-only calculation independently compared the shared quaternion formula with installed Three.js for degrees [30,45,60] and scaled local [2,6,12]. Both yield [5.518153541250351,-1.6281283922600767,12.284102711777642]. Applying extrinsic X then Y then Z instead yields [6.13815398984777,9.023899420511215,8.05557600953608]. This demonstrates the distinction beyond the artifact export fixture's single-axis rotation. The ticket artifact-oracle already uses Three default Euler XYZ, agreeing with actual Lowpoly scene semantics. No production math was changed. IO wording mentioning sequential XYZ should be corrected to intrinsic XYZ; this was reported to root before any such edit.

The final managed-export fixture now uses mixed nonzero intrinsic XYZ degrees [30,45,60]. Its explicit expected world coordinates were obtained independently from Three.js and handwritten into the neutral JSON; shared projection and GLB POSITION accessors compare with 1e-5 tolerance for binary32 lowering. The IO overview now explicitly names intrinsic XYZ rotation. The retained Three.js fixture oracle was rerun successfully against the strengthened fixture, and IO whitespace checks pass. Production math remained unchanged; IO source is frozen pending native diagnostics.

## Outbound Reflected Geometry

The shared world_parts path now also reverses polygon winding when the Lowpoly object's scale determinant is negative, so all seven exporters bake reflected objects with the same outward-facing orientation as the model view. The sign uses a product of f64-converted f32 scale components, avoiding underflow for the smallest valid binary32 scales. The neutral managed export fixture adds a mixed-rotation reflected scale [-2,3,4], independently established expected world coordinates and [2,1,0] winding. A tiny reflected scale made from the minimum binary32 subnormal exercises sign preservation. Native regression additionally checks that export preserves the complete snapshot, including paint, literal source and managed owner/handle. Managed test handles now match their typed state.

The retained Three.js oracle independently validates both outbound coordinate cases and both reflected scale signs, alongside the scene-instance cases; its rerun passes. IO whitespace checks pass. Source is frozen again pending integrated native diagnostics.

## Continuation 2: Complete Forest Admission and Entity Libraries

The disconnected-cycle gap is fixed by validating the complete node graph before selecting geometry. The owned GltfDocument node/scene schema is unchanged. One linear admission pass validates child and mesh indices and single-parent edges; all scene-root references must be in range, unique within their own scene and parentless. A traversal from every graph root must visit every node, exposing disconnected cycles even beside a valid rooted mesh. Roots may be reused across different scenes, and separate nodes may still instance the same mesh. The algorithm uses O(nodes + child edges + scene-root references) time and linear memory.

Khronos glTF 2.0 sections 3.5.1–3.5.2 require globally disjoint strict node trees, forbid scene roots from having parents, and permit the same root across different scenes; section 5.27.1 requires unique roots per scene. Scene-less assets are entity libraries, so an importer should retain their geometry definitions. See the [primary scene specification](https://registry.khronos.org/glTF/specs/2.0/glTF-2.0.html#scenes).

For a scene-less library, root node instances retain their transforms and each unreferenced mesh definition is also imported at identity. A mesh-only GLB therefore imports successfully. When scenes exist, only the selected scene contributes geometry. Complete graph validation precedes both paths, so unused or disconnected malformed structure cannot hide behind valid instances or library geometry. This supersedes the earlier selected-hierarchy-only structural policy in this report.

The new neutral gltf-forest.json corpus has 15 cases: mesh-only library; translated root instance plus unused mesh definition; selected-scene exclusion of unused definitions; valid rooted/repeated mesh instances; roots reused by multiple scenes; valid geometry alongside orphan cycles; duplicate/multiple-parent edges; invalid node, mesh and scene references; and invalid scene-root ownership/duplicates. The Rust regression compares imported geometry with the existing third-party gltf reader, checks library coordinates against its node transform and accessor positions, and independently checks malformed index refusal where supported by that reader.

The retained artifact-oracle.mjs now also uses the already-installed test-only Graphlib implementation to validate all 15 forest expectations and resulting instance/definition counts. Three.js independently validates the translated library instance and identity mesh definition alongside the prior transform/winding corpus. This oracle passed. IO git diff --check passes. A final read-only registered rustfmt --check probe parsed both the current GLB importer and current IO unit tests; it completed in 9.59 seconds with exit 1 solely for formatting differences and no syntax errors. Its generated output log was inspected and deleted. This is syntax parsing only, not typechecking. No native build or test was started; those remain blocked by the shared setup Source-preservation fence.

Continuation 2 changed files: IO GLB importer module; IO unit test module; new IO tests/fixtures/gltf-forest.json; retained ticket artifact-oracle.mjs; this report. No runtime dependency or shared glTF schema changes were added. Source is settled for continuation.
