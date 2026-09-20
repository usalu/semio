# Astra Sol World3d Scene Shading

## Result

The retained WGPU World3d path now carries source-colour provenance, resolves semantic neutral from the live customizable panel token, preserves vertex colours only for React's neutral and disabled rows, and supplies each draw's static emissive intensity. Standard-material metalness and roughness are resolved per instance: inline/procedural geometry uses `0/1`, while URL/GLB geometry consumes the live environment pair. The mesh shader uses a guarded inverse-transpose normal, current Three r182 Lambert and GGX multiscatter terms, its exact 16×16 DFG LUT, and ACES output. Mesh, line and textured World families own the output transform; shared UI, vector, blur, glass and scene-blit shaders do not.

An actual browser readback exposed a second fault after the lighting math was correct. Three's WebGL default framebuffer blends transparent World output after sRGB encoding, while a WebGPU sRGB attachment decodes the destination and blends in linear space. That made all thirteen opaque cases agree but left all four transparent cases much too bright. The renderer now keeps compatible sRGB and UNORM views of the same scene/composite textures. UI continues through the sRGB view. Only World mesh, line and textured draws use the UNORM view and explicitly apply Three's sRGB OETF after ACES. This preserves existing encoded destination bytes, including overlapping transparent World draws, without sampling the active attachment or changing shared UI/glass colour.

## Retained Colour and Theme Contract

`SceneColorSource3d` distinguishes `SemanticNeutral`, `Authored`, and `Environment`. `SceneInstanceMaterial3d` carries that provenance, the resolved vertex-colour policy, and static emissive intensity without changing the existing 96-byte GPU instance stride. Snapshot flags and the bounded draw rebuild retain the source identity end to end.

The JSON scene bridge now publishes white plus `SemanticNeutral` when neither the instance nor environment supplies a colour. Retained paint resolves that case from `Theme.panel` on every frame. Authored instance and environment material colours remain unchanged. Mesh schema presence decides whether a neutral/disabled draw uses vertex colour; selected, hovered, highlighted and provisional rows use their semantic solid fill. Static emissive intensities are `0`, `.08`, `.35`, `.2`, and `.2` for the implemented rows. Celebration's animated conic and per-GLB source materials remain S2 rather than being flattened into an incorrect S1 representation.

The semantic panel alias is now authoritative in the styling schemas rather than hard-coded in World code:

- default and premade theme schemas define `chrome.panel` for light and dark appearances;
- styling generation republishes those values to every generated target;
- WGPU `Theme.panel` maps to `chrome.panel`, while hierarchy `levelPanel` remains separate;
- the theme law covers the default CSS alias and a live custom-theme override.

## Shader and Attachment Contract

The mesh vertex stage computes the cofactor inverse-transpose of the model linear block. A determinant guard retains the model-linear fallback for singular transforms, and a finite-length guard prevents zero, NaN, or infinite normals from reaching normalization.

The fragment stage matches the bounded Three r182 `MeshStandardMaterial` profile used by the fixture: Lambert direct and indirect diffuse, correlated Smith GGX, Three's optimized Schlick term, DFG LUT multiscatter compensation, geometry roughness, static emissive, fallback hemisphere/directional lights, and the existing sun/shadow choice. The DFG table is the installed Three `DFGLUTData.js` RG16F table, sampled with bilinear clamp-to-edge semantics.

Every World output family applies Three's ACES matrices and fit with exposure `1 / 0.6`. The WGPU attachment path then conditionally applies the installed Three `sRGBTransferOETF` (`0.41666`, `1.055`, `0.0031308`, `12.92`). `shadow.w` is the backend output-transfer flag already inside the 240-byte global block: the retained WGPU target sets it to one because its World pipelines target the encoded UNORM view; the standalone WebGPU, D3D12 and Metal backends retain zero and their existing linear scene-target contract. Their shaders contain the same guarded transfer branch, but their transparent display parity remains open until their scene targets adopt an equivalent encoded attachment boundary and receive native runtime evidence.

`SceneColorTarget` and `PreparedCompositeTarget` allocate one underlying UNORM texture with compatible sRGB views. Sampling, blur, scene blit, glass, UI and presentation use the sRGB views. The four World colour pipelines use the UNORM format. The prepared scalar dispatcher selects the encoded view for opaque/translucent mesh instances, line vertices and textured instances on both the ordinary scene and glass-foreground composite targets. Shadow cursors are depth-only and never select a colour view.

## Shared Fixture and Independent Oracles

The schema-first fixture is:

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️scene-shading/🔣️.json`

Its schema records exact scene geometry, material, camera, fallback and sun lights, panel appearances, provenance rows, state rows, transformed-normal and BRDF samples, ACES inputs, render profile, and twenty RGBA8 oracle rows. The hemisphere position is `[0, 0, 1]`, and its ground source is `#9aa0ab` with the exact Three-linear value. Three compositing cases cover disabled over opaque light, provisional over opaque dark, and ordered authored-to-provisional overlap.

The independent Three Chromium run uses installed Three r182 `MeshStandardMaterial`/`MeshBasicMaterial`, a real 64×64 WebGL canvas, DPR 1, antialiasing off, transparent black clear, `premultipliedAlpha=false`, ACES exposure 1, sRGB output, and centre pixel `[32, 32]`. Three oracle 7 reproduced all twenty recorded rows byte-exact on ANGLE Metal / Apple M1 Max.

The independent WebGPU Chromium harness consumes the production `WORLD3D_SHADER`, exact production vertex/instance/global layouts, production blending, and the dual attachment-view contract. WebGPU oracle 3 used shader SHA-256 `2c9da2f75944c09b23f99eef8572ed2a2cb782228d0fdfd1d29df1187162ab9f` on Apple `metal-3` and matched all twenty Three rows byte-exact. This includes alpha over transparent black, alpha over two opaque destinations, and two ordered translucent layers. Evidence is in:

- `🗑️generated/astra-runtime/three-shading-oracle-7/report.md`
- `🗑️generated/astra-runtime/wgpu-shading-oracle-3/report.md`

The repository TypeScript law independently uses installed Three for colour conversion, `Matrix3.getNormalMatrix`, material state, ShaderChunk BRDF/DFG/ACES/OETF upgrade guards, fixture linkage and compositing-row ownership. Its focused Bun/Nx run passed one file and three tests in 2.0 seconds Nx time.

## Rust Laws and Compiler Receipts

Rust laws cover:

- semantic/authored/environment provenance through the actual JSON bridge and bounded retained snapshot ingestion;
- live light/dark/custom panel resolution, vertex-colour preservation and static emissive policy;
- unchanged 96-byte instance packing with policy bits, emissive intensity, metalness and roughness;
- byte-identical canonical and retained-WGPU WGSL for all three World shader families;
- Naga parse and validation, inverse-transpose/BRDF/DFG/ACES/OETF ownership, and HLSL/MSL mirror symbols;
- all World colour cursor families selecting the encoded view in ordinary and glass-foreground phases while UI and shadow cursors do not;
- all four WGPU World pipelines targeting the compatible UNORM format.

The final native shader/render gate passed 136/136 with no skips, including Naga, backend-neutral surface laws, compatible attachment views, encoded World cursor routing, and canonical/target WGSL identity after the exact OETF correction. UI 21 passed 616/616. The permanent Bun/Nx pixel gate passed all twenty recorded Three rows and all twenty production-WGSL WebGPU rows byte-exact. Canonical activation 12 passed with all five renderer hashes stable, and React activation 6 completed against the same fixture contract. No Cargo or native build was run by Sol.

## Explicit Remaining Scope

S2b now owns celebration's animated conic and painted standard-material texture paths, as recorded below. The older audit's claim that `GlbInstanceMesh` retains a GLB child's source metalness/roughness was incorrect: the current executable React host always replaces that child material with a new `MeshStandardMaterial`; without an environment override its values are exactly `0` and `1`. React's projected transparent ordering remains S2c. Standalone WebGPU, D3D12 and Metal transparent attachment parity remains open as described above.

Full retained-renderer browser integration is also separate from the controlled production-shader readback. The next paired activation must confirm that live Puzzle3D receives the corrected panel colour and World output while preserving UI/glass appearance.

## S2 Schema and Three Reference Boundary

The shared fixture now declares the current React material contract before any WGPU S2 production change:

- inline and GLB standard defaults resolve to metalness `0`, roughness `1`; a GLB's authored `0.85`/`0.12` pair is intentionally replaced;
- an environment override resolves per draw to `0.35`/`0.58`;
- painted textures use `MeshStandardMaterial`, with the exact neutral/selected/disabled vertex-colour, emissive, opacity and depth-write policies;
- celebrated instances use React's raw `ShaderMaterial` conic stops, 1.2-second phase, `DoubleSide`, and opacity-dependent depth writes, without ACES or output chunks;
- transparent standard meshes sort by projected transformed bounding-sphere centre, then retain object insertion order for exact projected-depth ties, while keeping `depthWrite=true`.

The installed Three r182 Chromium oracle records fifteen RGBA8 rows and the actual `onBeforeRender` order for three two-layer scenes. Its first ordering run supplied a useful red receipt: decimal `0.4` in Float32 geometry plus double-precision translation left opposite sub-ULP residual depths, so the purported tie sorted by depth. The language-neutral fixture now uses exactly representable `0.5` offsets. Three then proved far-before-near for translated off-origin spheres, red-before-blue for the first stable tie, blue-before-red for the reversed tie, and distinct byte output for the two tie orders. The recorded rerun reproduced all fifteen pixels byte-exact.

Two of those pixels come from one mixed-material scene: inline geometry at the left uses React's fixed `0/1`, while a GLB at the right uses the environment's `.35/.58`. The current production WGSL harness uses the real pass-global environment material for both instances. It supplied the required S2a red: the inline sample rendered `[198,198,192,255]` against Three's `[215,215,210,255]`, a `[-17,-17,-18,0]` delta, while the adjacent GLB matched `[199,199,192,255]` exactly. This is the concrete same-pass state the global material cannot represent; per-instance/per-draw material data is required.

Exact gate:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/s2a-three-oracle-6" bun nx run @semio-tech/framework-renderer-react:scene-shading-pixel-check --skip-nx-cache -- s2-reference
```

Final reference result: target passed in 4.0 seconds, fifteen actual Three pixels matched byte-exact, and all three actual draw-order traces matched the fixture. Evidence is in `🗑️generated/astra-runtime/s2a-three-oracle-6/world3d-scene-shading/shading-s2-oracle/`. The production-WGSL red is in `🗑️generated/astra-runtime/s2a-wgpu-red-1/world3d-scene-shading/shading-s2-wgpu/`.

The exact red command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/s2a-wgpu-red-1" bun nx run @semio-tech/framework-renderer-react:scene-shading-pixel-check --skip-nx-cache -- s2-material-wgpu
```

## S2a Production Boundary

S2a carries only resolved standard-material metalness and roughness. `SceneInstanceMaterial3d` and backend-neutral `MeshInstance` gain those two scalars with exact defaults `0` and `1`. World assigns the environment pair only to URL/GLB draws; inline and painted standard draws retain `0/1`, matching the executable React host. Celebration, texture sampling, transparent depth writes and sorting remain later coherent packets.

The public GPU instance layout remains 96 bytes. The existing four-float flags lane is repacked as `[policyBits, emissiveIntensity, metalness, roughness]`, where exact integer bit 0 means preserve vertex colour and bit 1 means receive shadows. Canonical WGSL, its retained-WGPU mirror, HLSL and MSL decode that same contract. The retained WGPU, standalone WebGPU, D3D12 and Metal packers publish the same values, and shadow-only instances retain their material values while changing only the receive-shadow bit. This removes the pass-global material ambiguity without changing instance stride, buffer capacity or any global uniform layout.

The production WebGPU rerun used shader SHA-256 `e1fa736ef6f3a71c4499990f737e95104a03dc77b9dcea3417ab16b3edda1d96`. Both samples are now byte-exact against the recorded actual Three result: inline `[215,215,210,255]` and adjacent GLB `[199,199,192,255]`. The focused S2a gate passed in 6.6 seconds Nx time. The full prior shading/compositing gate then reproduced all twenty Three pixels and all twenty production-WGSL pixels in 9.5 seconds Nx time. Canonical and retained-WGPU mesh shader strings are byte-identical at 20,861 characters. Evidence is in:

- `🗑️generated/astra-runtime/s2a-wgpu-green-1/world3d-scene-shading/shading-s2-wgpu/`
- `🗑️generated/astra-runtime/s2a-shading-regression-1/world3d-scene-shading/`

Activation 14's renderer Wasm includes the S2a shader contract. Its sealed renderer artifact has SHA-256 `2fcbda8f46240f76a037694ed089f66caeec8621433de2bc76d7e450a0a2f9de`; the distinctive canonical-WGSL literals are `let metalness = clamp(in.flags.z, 0.0, 1.0);` and `let roughness = min(max(in.flags.w, 0.0525) + geometry_roughness, 1.0);`.

## S2b/c Fail-First Production Evidence

The permanent pixel runner has two additional modes that execute the current production shaders without changing production layouts or behavior. Both modes use the same actual Three rows already recorded in the shared fixture.

`s2-current-mesh-wgpu` uses `WORLD3D_SHADER`, exact mesh/global/instance layouts, and one actual WebGPU draw for each controlled case. It proves celebration is presently a static standard-material tint rather than React's animated raw conic: all four phases produced the same current colour, `[255,120,127,255]` for opaque and `[158,75,79,158]` for the translucent row. Their deltas from actual Three were `[3,109,106,0]`, `[188,-6,21,0]`, `[126,1,62,0]`, and `[5,39,76,0]`. It also proves the current insertion order is insufficient for translated off-origin transparent meshes: WGPU submitted near-blue then far-red and produced `[157,99,126,197]`, while Three submitted far-red then near-blue and produced `[121,107,155,198]`. Both exactly projected-depth tie cases were already byte-exact in their respective insertion orders, so the required correction is a projected-depth stable ordering key rather than a different tie rule. The expected fail-first command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/s2bc-current-mesh-red-3" bun nx run @semio-tech/framework-renderer-react:scene-shading-pixel-check --skip-nx-cache -- s2-current-mesh-wgpu
```

`s2-current-texture-wgpu` uses the actual `WORLD3D_TEXTURED_SHADER`, its 20-byte vertex and 80-byte instance layouts, a one-pixel production sRGB texture, and the compatible World attachment view. This is the only current textured World shader; the retained mesh-paint registry does not yet route painted mesh geometry to it. All three controlled samples reached the intended pixel assertion. Current unlit output versus actual Three `MeshStandardMaterial` was neutral `[127,189,214,255]` versus `[130,189,218,255]`, selected `[87,11,55,255]` versus `[237,68,104,255]`, and disabled `[57,85,96,115]` versus `[59,85,98,115]`. The selected `[-150,-57,-49,0]` delta additionally proves that a sampled reference-plane tint cannot represent React's selected solid/emissive painted mesh policy. The expected fail-first command was:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-runtime/s2bc-current-texture-red-1" bun nx run @semio-tech/framework-renderer-react:scene-shading-pixel-check --skip-nx-cache -- s2-current-texture-wgpu
```

Evidence is in:

- `🗑️generated/astra-runtime/s2bc-current-mesh-red-3/world3d-scene-shading/shading-s2-current-mesh/`
- `🗑️generated/astra-runtime/s2bc-current-texture-red-1/world3d-scene-shading/shading-s2-current-texture/`

## S2b/c Coherent Production Boundaries

Celebration needs a dedicated bounded draw family. Its per-instance payload is model transform, three linear conic stops, angle/phase and opacity; frame time remains a pass-level value. The vertex stage retains local object position for the conic coordinate, and the fragment stage emits React's raw `ShaderMaterial` result without Standard lighting, ACES or the S1 output chunks. Its pipeline is `DoubleSide`; opaque rows write depth and translucent rows do not. This keeps the animated material contract separate from standard mesh state and permits the existing actual Three phase rows to remain the oracle.

Painted mesh texture needs its own standard-lit mesh family, separate from the reference-plane Basic texture family. Its vertex contract includes the resident mesh position, normal, colour and UV; its texture is linear `NoColorSpace`, not the reference image's sRGB source. Sampling multiplies the standard base/vertex input before the established S1 BRDF, emissive, ACES and explicit output transfer. The retained paint-texture key must select this family only after both mesh and texture generations are resident. Neutral and disabled preserve vertex colour; selected and other semantic states use the solid/emissive policy already proved by the fixture. React's standard material remains `DoubleSide` and writes depth for transparent rows.

Transparent standard instances need a bounded stable sort before cursor emission. The primary key is the camera-projected depth of the transformed resident-mesh bounding-sphere centre; the tie key is the original draw and instance insertion ordinal. The admitted scene caps the sortable population, so the implementation can reuse a fixed-capacity retained scratch lane rather than allocate or traverse unbounded content. Sorting must occur per visible instance because one draw can contain off-origin instances at different projected depths. The current flattened GLB mesh cannot reproduce distinct child-object sort keys; that asset-contract limitation remains explicit rather than inventing source-child materials or an unbounded compatibility layer.

## S2b Production Boundary and Green Receipts

S2b implements the first two boundaries above and leaves the projected-depth sort as S2c. `ScenePass3d` now owns explicit material draws. Painted and celebration families therefore keep their distinct topology, shader, culling, opacity and depth-write policies without overloading the ordinary Standard draw or the reference-plane Basic texture family.

The resident mesh vertex is 48 bytes: position, normal, colour and UV. Ordinary Standard, shadow, painted and celebration pipelines consume the same mesh table. Painted draws select the generation-owned `mesh-paint:{meshKey}` raster only after the current mesh has one UV per vertex. The public `apply_decoded_mesh_paint_image` boundary accepts exact RGBA pixels produced by an asynchronous owner, rejects malformed extents, enforces the existing bounded raster item ceiling and publishes through the existing generation-owned paint registry. It performs no base64 or raster-codec work in the World frame. Full-quality 64 MiB decode, pooled chunk upload, cancellation and stale-generation publication remain a shared image-owner packet with the ReferenceImage worker boundary.

The generated painted WGSL reuses the canonical S1 normal, Standard BRDF, light, shadow, emissive, ACES and output-transfer implementation. Its additional map lane matches React's `NoColorSpace` paint semantics while the resident GPU raster remains an sRGB texture. Neutral and disabled rows preserve vertex colour; selected and other semantic rows retain the already-proved solid/emissive policy. Painted material is `DoubleSide`, and both opaque and translucent rows write depth as React's `MeshStandardMaterial` does.

Celebration uses a dedicated 128-byte instance payload for the model matrix, three linear conic stops, angle and opacity. Its raw WGSL keeps local object position for the conic coordinate and emits the interpolated colour directly, with no Standard lighting, ACES or OETF. The phase is derived from the live clock on the same 1.2-second period as React. Both celebration pipelines are `DoubleSide`; opaque draws write depth and translucent draws do not.

Prepared measurement and retirement visit the material draw, mesh key, optional paint-texture key, instance value and instance identifier incrementally. Raster residency walks painted material keys before UI and overlay rasters. World census, retained heap measurement and frame retirement include all material draws and their instances. No grant, capacity, global layout or 96-byte Standard instance stride changed.

The actual browser WebGPU gates execute the production shader sources and exact production layouts against the previously recorded installed-Three rows. On Apple `metal-3`:

- celebration shader SHA-256 `8501510eb3d728ac25c502d1e07478f6a9a8df752e5c9793e0bf1161454d53c8` matched all four phases byte-exact, including the transparent row;
- generated painted shader SHA-256 `85953de0b7eb7b6526cad4848e5f6afaea36aeb29e930845bb31df46b8b76620` matched all three neutral, selected and disabled rows byte-exact.

The exact commands were:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-s2b-green" bun nx run '@semio-tech/framework-renderer-react:scene-shading-pixel-check' -- s2-celebration-wgpu
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-s2b-green" bun nx run '@semio-tech/framework-renderer-react:scene-shading-pixel-check' -- s2-painted-wgpu
```

Their reports are:

- `🗑️generated/astra-s2b-green/world3d-scene-shading/shading-s2-celebration/report.md`
- `🗑️generated/astra-s2b-green/world3d-scene-shading/shading-s2-painted/report.md`

Rust source and focused laws are syntax-clean under `rustfmt --emit stdout`; Sol did not run Cargo or claim a native compiler receipt. The next serialized UI/renderer native gate must compile the new scene-pass public shape, verify retained budget measurements, Naga pipeline construction and the World decoded-paint publication law. S2c still owns projected transformed bounding-sphere depth sorting with stable insertion ties, including off-origin instances and the recorded flattened-GLB limitation.

## S2c Projected Transparent Ordering

S2c routes every transparent ordinary Standard instance into the material lane, alongside the painted and celebration families, then orders all transparent material objects by the camera-projected centre of the resident mesh bounding sphere after the instance model transform. This is Three r182's actual `geometry.boundingSphere.center → matrixWorld → projection × view` key. The local centre is the mesh AABB centre because Three's `BufferGeometry.computeBoundingSphere` derives it from that box. Descending projected depth gives far-to-near submission. An explicit producer ordinal breaks exact depth ties, so the implementation can use allocation-free `sort_unstable_by` while retaining Three's stable result.

The sort population comes only from the admitted World draw/instance capacities. It moves owned instances into a bounded item vector, keeps mesh/material metadata once per source draw, and coalesces adjacent sorted instances from the same source back into one retained draw. Missing or non-finite geometry receives the terminal depth key and is subsequently removed by the existing exact mesh-residency filter. No state slot, GPU vertex/instance layout, shader uniform or global budget changed.

Transparent Standard has its own WGPU pipeline because React keeps `MeshStandardMaterial.depthWrite=true` and `FrontSide` when `transparent=true`. The existing overlay-transparent pipeline retains its depth-write-off policy. Painted Standard remains `DoubleSide` with depth writes; transparent celebration remains `DoubleSide` without depth writes. This preserves the three distinct executable React policies instead of changing the shared overlay path.

The Rust law consumes the shared schema-first `depthOrderCases` and the same camera as the browser reference. It covers translated, off-origin local mesh centres plus both exact-depth insertion orders. The installed Three r182 Chromium oracle was rerun after the S2b common-stride change: all fifteen S2 rows were byte-exact, and its three actual `onBeforeRender` traces remained far-red/near-blue, red/blue for the first tie, and blue/red for the reversed tie. Exact command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-s2c-three-reference" bun nx run '@semio-tech/framework-renderer-react:scene-shading-pixel-check' -- s2-reference
```

The gate passed in 3.7 seconds Nx time. Evidence is in `🗑️generated/astra-s2c-three-reference/world3d-scene-shading/shading-s2-oracle/`. The earlier production-WGSL red remains the behavioral before receipt: near-blue/far-red insertion produced `[157,99,126,197]`, while actual Three's projected order produced `[121,107,155,198]`; both stable tie cases were already byte-exact. Rust sources and laws parse under `rustfmt --emit stdout`, and `git diff --check` is clean. Sol did not run Cargo or claim the World/native compiler receipt.

The controlled WebGPU acceptance then used production `WORLD3D_SHADER` SHA-256 `e1fa736ef6f3a71c4499990f737e95104a03dc77b9dcea3417ab16b3edda1d96`, the exact 48-byte mesh and 96-byte Standard instance layouts, `FrontSide`, alpha blending, `LessEqual`, and the repaired `depthWrite=true` policy. It submitted each fixture's order consumed by the Rust production law and matched all three actual Three pixels byte-exact: `[121,107,155,198]`, `[121,107,155,198]`, and `[157,99,126,197]`. Exact command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-s2c-wgpu-green" bun nx run '@semio-tech/framework-renderer-react:scene-shading-pixel-check' -- s2-ordered-mesh-wgpu
```

The gate passed in 4.4 seconds Nx time. Evidence is in `🗑️generated/astra-s2c-wgpu-green/world3d-scene-shading/shading-s2-ordered-mesh/`. This browser harness executes the repaired production shader, layouts, blend and depth policy, but it cannot call the Rust World sorter inside Chromium. Production sort-key and stable-tie acceptance therefore belongs to the fixture-consuming Rust law and the upcoming World native receipt; the WebGPU gate proves that the order emitted by that law produces Three's actual pixels.

The current flattened GLB lease still represents one combined mesh, so it cannot reproduce distinct child-object keys from a multi-child source asset. S2c orders every retained instance of that combined mesh correctly and records the child-level limitation without inventing a legacy source-material layer.

## Permanent Command Routing

`scene-shading-pixel-check` is registered in the renderer React `📜️script.ts` router and exposed as an uncached Nx target whose command is `bun ./📜️script.ts scene-shading-pixel-check` with the workspace browser dependency. `.vscode/launch.json` now includes the default S1 regression and every retained S2 mode: reference, material, current diagnostic, ordered, painted and celebration. The launch file parses as JSONC and `nx show project @semio-tech/framework-renderer-react --json` resolves the target to the expected router command and `deps-browsers` prerequisite.

## Principal Source Files

- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/✨️shader-contract/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/✨️hlsl/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/✨️msl/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`
- `🧰️framework/🔨️modules/🖱️ui/🎨️styling/🌓️theme/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧬️schema/🎨️scene-shading/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️scene-shading/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🎨️world3d-scene-shading/🟦️.ts`
