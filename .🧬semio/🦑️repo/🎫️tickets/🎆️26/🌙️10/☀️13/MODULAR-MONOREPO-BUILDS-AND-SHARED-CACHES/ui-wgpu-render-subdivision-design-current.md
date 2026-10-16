# UI WGPU Ordinary-Bounded Subdivision Design

This is a proposed Source architecture for review, not another approved native attempt or a claim that unimplemented package closures already pass. Native91733 remains terminal exit1 at replication Source admission; no compiler/GPU test ran. No Cargo, Rust, image/store operation, production Rust move, manifest change or cap change ran during this design task. The old768KiB child metadata cannot override the unchanged ordinary262144byte/32file/4000active-line cap and must be replaced with ordinary-bounded owners.

## Exact Replication Paths And Clean Dependency Scope

The shortest real-manifest path, including the selected child's declared dev edges in the inventory, is:

1. `semio-framework-ui-wgpu-render`: `ui_scene` in `🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/📦️packages/🦀️rust/Cargo.toml` selects `🖱️ui/🎬️scene/📦️packages/🦀️rust/Cargo.toml`.
2. `semio-framework-ui-scene`: direct `semio-framework-ui-scene-doc` selects `🖱️ui/🎬️scene/🎬️scenes/📜️doc/📦️packages/🦀️rust/Cargo.toml`.
3. `semio-framework-ui-scene-doc`: `protocol` selects `📡️replication/📦️packages/🦀️rust/Cargo.toml`.

All paths above are under `🧰️framework/🔨️modules`. These three normal edges are unconditional; this witness is present in the actual native-selected dependency graph, as confirmed by91733 reaching replication. The pure inventory follows actual manifests, independently parses TOML, includes root dev dependencies and uses graphlib SCC. It currently observes125 firstparty owners. Its complete graph is a conservative declared target/optional union, not a reimplementation of Cargo feature resolution; the report does not label125 as a compiler-resolved package count.

Simply replacing the child's `ui_scene::math` facade import will not remove replication. Draw-contract, prepared and prepared-staging each have another direct aggregate-scene dependency. Replace their genuine math-only imports with the existing actual scene-algebra, scene-mesh, scene-pass and, where genuinely used by gizmo/tests, scene-camera owners. Staging uses only Mesh3dLease and should select scene-mesh alone. Do not create another math facade crate.

Scene-mesh has two further aggregate edges:

- Its `protocol` imports are exclusively canonical `value::{RetainedCloneGrant, RetainedCloneProgress, ControlledRetirement, RetirementDemand, RetireOwned, RetainedOwnerGate}`. Use their existing `semio-framework-value` owner directly; preserve the exact gate/retirement implementation, not a structural substitute.
- Scene-mesh imports only `ComponentReferenceTable` from mesh-engine. The actual table is3540bytes/68lines in `🏗️mesh-engine/🎯️component-references/🦀️.rs`, with existing5269byte/35line custody tests. Its codecs/retirement use only canonical Value, while its test oracle uses serde. Create the real `semio-framework-mesh-component-references` owner there, physically owning that body and tests, with direct Value dependency. Update actual mesh-engine and scene-mesh consumers to that canonical type owner. This avoids bringing mesh-engine's separate MeshData/codecs/causal histories into UI GPU dependencies. Mesh-engine itself genuinely uses `protocol::causal::HistoryFoldIndex/HistoryFoldSet` and its decoder; those must not be deleted speculatively.

A modeled graph replacing these four scene-facade edges and the two mesh aggregate edges reaches90 existing/new foundational owners, with no replication path and no aggregate UI path. Adding the proposed execution/device owners below reaches98 declared owners. Both independent graphlib SCC and a separate DFS found no cycle. This is a proposed-edge proof, not a post-implementation Cargo result. Generated evidence: `🗑️generated/ui-wgpu-subdivision-dependencies.json` and `ui-wgpu-subdivision-proposed-edges.json`.

## Recommended Real Compilation Owners

Let D be `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw`. Each proposed package owns real implementation and its native tests. No package mounts another package's implementation with path/include, and product still sees the existing opaque GpuContext/present/readback/EngineRasterBuild seam.

| Owner | Actual responsibility and location | Measured current body allocation before new interface code |
| --- | --- | --- |
| ui-render-device | Firstparty resource/encoder/submission contract, descriptor validation and exact resource refusal types, under `🖌️render/🔌️device` | New schema/implementation; explicit small contract target, not claimed measured |
| ui-wgpu-device | Actual Device/Queue/session, owned resources, command encoder/pass, submission/completion, map/readback primitives and genuine native/browser capture, D/`🎮️execution/🔌️device` | Device creation extracted from current70928B execution body, plus2615B platform body; firstparty wrapper implementation is new and requires a measured gate |
| ui-render-prepared-shaders | Actual current prepared WGSL constants and authored/painted shader generation, D/`🎨️shaders` |55910B/1138lines, plus actual shader-generation/Naga laws |
| ui-wgpu-pipelines | Actual pipeline/layout/atlas construction and resource bundle, D/`🔧️pipeline/🏭️factory` |66983B/942lines constructor plus selected private field/layout records from11374B/241line helpers |
| ui-wgpu-raster | Actual bounded raster reservation, allocation, staging, ownership and typed close handback, D/`🖼️raster` |83543B/1784lines raster body +37102B/693line residency tests +8853B/170line witness tests =129498B/2647raw lines before small roots/interfaces/assets |
| ui-wgpu-mesh | Actual mesh upload/cache/retirement and exact native source handback, D/`🕸️mesh` |14436B/313line mesh body + at most8379B/191line firstparty world vertex/instance layout body; genuine lease/handback tests move with their owner |
| ui-wgpu-draw-execution | Actual UI/vector/world/blur/glass command encoding, D/`🔧️pipeline/🎬️execution` |95336B/1635line execution +11374B/241line helper upper allocation +6008B/155line frame-buffer helpers;38113B/688line draw laws allocated here or their actual neutral producer |
| ui-wgpu-render (current package) | Actual prepared frame/present cursor orchestration, targets, readback, EngineRasterBuild and platform composition | Current70928B/1163line orchestration +14289B/285line targets +5604B/136line readback +7368B/43line canvas +2615B/47line ingress upper allocation; native present/readback/engine tests are28554B/488lines +6845B/104lines +6394B/116lines before removing Source-only portions |

These are measured whole-file or identified-domain body bytes/raw lines in the current tree. They are not predicted compiler closures and do not subtract cfg branches or count new code as zero. Constructor/helper/vertex allocations have overlapping *upper* entries above to avoid false precision about final field ownership. The implementation must author a disjoint file/test inventory and run the real Source closure reader on every default/feature/test target before any native dispatch. Each owner gets exactly the unchanged ordinary max bounds; no per-package extension or global increase. Practical headroom targets are <=192KiB/3000active lines for resource/execution owners and <=128KiB/2200lines for the device contract/backend/factory. If an implemented closure exceeds those practical targets, resolve the domain subdivision before qualification rather than deleting laws.

The proposed device owner is not a forwarding wrapper over the current render crate. It physically owns actual backend creation/allocation/encode/submit/map/drop code. The existing GpuContext assembles lower actual producers rather than mounting them. The shader owner physically owns shader programs/transformations. Mesh/raster owners physically own original resource state and bounded operations. The neutral contract owns schema/type/descriptor admission semantics, not an empty reexport facade.

## Opaque Firstparty Device API

Recommend the neutral contract and concrete actual WGPU provider; no new internal raw-WGPU reexport is needed. Existing approved platform Window/OffscreenCanvas ingress remains explicit. Existing CanvasScene native interop can be consumed *inside* the WGPU device implementation, where its foreign Device/Queue/View are already hidden from the new API.

Author schema first for finite buffer/texture/layout/pipeline/attachment/copy descriptors, units and closure dispositions at `🖌️render/🔌️device/🧬️schema/🔣️.json`, then language-neutral bounds/refusal/output cases. Reuse the existing actual `ui-render-shader-contract` PipelineSpec/vertex/bind-group spec vocabulary rather than inventing another WGPU descriptor mirror. Fill only genuinely missing format/usage/submission terms in the resource contract; no arbitrary extension maps or external ABI types.

The concrete backend privately owns `SessionRoots { device, queue, ... }`. Public firstparty Buffer/Texture/View/Sampler/Layout/Pipeline/Encoder/Submission owners wrap actual backend resources and retain their actual issuing session. Fields/constructors are private, owners are not deserializable, and no PID, marker, integer ID, copied receipt or resource-name lookup authorizes work. A view retains its actual texture/session root. The provider checks actual private session identity before every cross-resource encode/bind/copy; it does not accept a caller-supplied equality boolean as authority.

The neutral device contract can use static-dispatched associated firstparty resource types, as the existing backend architecture already prefers. The concrete provider's associated types are firstparty opaque structs, never aliases to WGPU types. Pipeline/raster/mesh/draw implementations are generic over that firstparty interface; the product context remains a concrete opaque newtype over the actual WGPU provider, with no public arbitrary-provider injection constructor. Neutral schema/descriptor test data creates no hardware owner. Runtime custody tests use the actual concrete provider only.

Required operations are concrete: actual capture/prepare session; create/write/retire buffer; create texture/view/sampler; construct shader/layout/pipeline from firstparty specs; begin scoped encoder/render pass with borrowed actual attachments; bind actual firstparty buffers/groups/pipelines; encode draw/copy; consume command buffer into actual submitted work; await/poll genuine backend completion; map/unmap bounded readback and close. Scoped pass/encoder lifetimes retain borrowed resources. Backend completion derives from the real queue/device signal, never elapsed time or a data notification pretending to be death. Keep accepted work and original resource roots through required settlement before final release. No new Core lease/control/namespace is part of this proposal.

Pipeline factory constructs and owns UiPipelineSet with actual opaque pipelines/layout/atlas resources. Mesh owns its canonical vertex layout data and exact MeshGpuSourceReturn. Raster receives actual layout/globals/glyph owners as firstparty opaque resources, preserving original reservation tuple, nonce, bytes and parent session identity. Draw execution receives real pipeline/mesh/raster/target owners, issues actual commands and leaves final submission/presentation to the upper context. Cross-package handles remain actual typed owners, not integer registries. The top product API never exposes backend resource types.

EngineRasterBuild remains the exact canonical owner of admission/renderer/texture/view/session residue. Returned cancellation/staging restores the same owner; Retained is emitted only after the real table close cursor owns the consumed tuple. The typed cancellation fix and actual sibling-session/root comparison are mandatory invariants through all moves. The actual CanvasRasterSession implementation belongs in the concrete device provider as an opaque CanvasRasterizer; upper EngineRasterBuild owns that firstparty object and exact table admission. This keeps the actual Device/Queue/View needed by Vello within one backend implementation.

## Shader Semantics Must Remain Exact

Do not naively replace current shaders with the older shader-contract constants. Measured UI_SHADER differs in affine kind10/11, clip ellipse and logical position; WORLD3D_SHADER differs in clip plane, emissive/cutoff lane and section clipping. Seven other constants compare byte-equal, but those two do not. The new prepared shader owner preserves current exact programs and authored/painted transformations. Its actual family descriptors may reuse neutral PipelineSpec types, while existing alternate backend shader families remain their own contracts until explicitly reconciled. No old/new compatibility layer is introduced.

## Native Source Inspection Versus Native Behavior

Move only Source inspection into Source-owned TS/independent AST laws:

- a_textured_world_instance_is_encoded_into_its_pass_target: move the two string checks; retain actual Rust prepared traversal assertions.
- every_world_color_cursor_uses_the_encoded_composite_attachment: retain actual cursor-to-attachment behavior; move whole-file descriptor registration/count/encoding checks and their neutral hostile vectors.
- standard_translucent_world_pipeline_preserves_react_depth_and_side_policy: entirely Source inspection; transfer its blend/cull/depth policy to AST/neutral descriptor laws.
- the_world_textured_pass_is_a_blended_underlay_on_a_centred_plane: retain actual native struct stride/vertex/UV assertions; move only descriptor string checks to AST.
- every_successful_gpu_transition_is_measured_before_returning and other real cursor/resource/pixel laws remain native. Actual headless pipeline construction belongs with the upper real session/pipeline assembly test, not the source-inspection oracle.

No native inspection body was removed yet. The existing registered pure harness now demonstrates the replacement technique: our Rust token/paired-field inspection and independent installed web-tree-sitter Rust AST agree on all19 encoded pipeline descriptors, and both reject wrong-attachment and duplicate-label mutations against the existing language-neutral expected corpus. Both parsers inspect actual Source; no GPU object or compiler is involved. This can transfer Source-only laws without copying whole production pipeline bodies into native compile units. The first new AST test attempt exposed a test-only locale/codepoint ordering mismatch; it was fixed by consistent codepoint ordering, without changing production or expected pipeline policy.

## Safe Implementation Sequence For Review

1. Make the real component-reference owner and direct canonical Value/math producer edges cohesive, with language-neutral codec/capacity/custody tests and independent JSON/SQLite observations. Update every actual consumer/manifest/schema/launch/catalog entry together. No Root/Native lease changes.
2. Author finite neutral device/resource schema and opaque contract, plus actual backend provider; physically move device/session/allocation/encoder/readback primitive code. Test invalid descriptors with Ajv/SQLite and preserve real-device foreign-session/closure laws for the later native window. Request separate review before any raw backend type export if the opaque contract proves insufficient; none is assumed authorized here.
3. Move exact prepared shader implementation, actual pipeline factory and actual mesh/raster/draw executors into the proposed owners with disjoint native tests. Keep all original ownership and pixel assertions; move Source-only inspection to the demonstrated TS/AST route.
4. Replace the current768KiB metadata with unchanged ordinary bounds and qualify every actual complete feature/test closure, target cfg, manifests, registry, launch/seed and SCC in BOTH pure/strict modes. Only after Root reviews the coherent Source epoch may a separately authorized native operation execute. Existing91733 must not be retried unchanged.

## Current Pure Qualification

No production Rust/manifests changed during this design. The dev-root graph and new independent AST law expand the retained10/906 checkpoint:

| Gate | Handle | Outcome |
| --- | --- | --- |
| Source prototype |53949| exit0,11tests/921expectations,9.67s |
| Source release |43038| exit0,11tests/921expectations |
| strict prototype |44896| exit0 |
| strict release |51521| exit0 |

All terminal handles collected. Logs `🗑️generated/ui-wgpu-subdivision-qualified-{test,types}-{prototype,release}.log`. Earlier test-only AST ordering failures42848/50407 and strict1463/96853 are collected too. Prior native and Source results remain separately reported; no native/Rust/GPU success or compiler throughput gain follows from this design.
