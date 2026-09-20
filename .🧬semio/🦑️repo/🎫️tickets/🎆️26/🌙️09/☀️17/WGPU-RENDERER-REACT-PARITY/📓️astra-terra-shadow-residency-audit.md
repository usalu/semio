# Terra Audit: React Shadow and Raster Residency Contracts

Read-only source audit on 2026-09-20. This report covers the live React World3d/Icon paths and the main `ui_wgpu` prepared-presenter path. No build, browser run, or code change was performed here. The reported generic renderer shader validation does not exercise the production React parity cases below.

## Production paths inspected

React World3d enables R3F shadows at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:7392-7399`, creates the configured directional sun at `7465-7479`, and assigns GLB roles at `2308-2362`. `WorldCanvas` passes the Boolean through to R3F at `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3729-3734`. The installed R3F Boolean path sets `PCFSoftShadowMap`; the installed current Three runtime converts that deprecated value to `PCFShadowMap` (`node_modules/three/src/renderers/webgl/WebGLShadowMap.js:99-103`).

React Icon sends PNG work through `buildIconScene` and `renderIconPng` in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:322-362,453-461`; SVG goes to `SVGRenderer` at `439-451`. The native Icon consumer parses the request and turns it into a synthetic World3d scene at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🎯️targets/🧊️wgpu/🦀️.rs:6570-6688,6703-6720`.

The main WGPU shadow source is produced by `render_world_3d` (`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs:12180-12394`), traversed as `PassShadow*` by `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎟️prepared/🦀️.rs:2552-2566,2697-2710`, and emitted by `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧊️gpu/🦀️.rs:775-793`. This is the reachable consumer. Native Metal/D3D12 shadow ABI declarations with no equivalent map producer/consumer are outside this finding set.

## Shadow defects

### P1 — WGPU's caster and receiver sets do not represent React roles

React World GLB meshes cast **and** receive exactly when `environment.shadow.enabled` is true (`World3dHost:2357-2358`). Paint/textured meshes have neither property (`World3dHost:2157-2202`). Terrain is explicitly receive-only (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗺️WorldTerrainLayer/🟦️.tsx:207-245`). React Icon assigns both roles only inside `if (material)` (`react/🟦️.tsx:322-338`); an Icon request without material leaves the imported GLB roles untouched.

The WGPU scene schema has no role channel. Every opaque `ScenePass3d.draws` item is rendered into the depth map (`prepared:2557,2697-2710`; `gpu:781-793`), then every opaque and translucent material evaluates `world3d_shadow_visibility` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs:277-317`). `render_world_3d` appends terrain and extra overlay geometry to `draws` (`world/🦀️.rs:12365-12393`), so receive-only terrain becomes a caster. It moves provisional/transparent geometry to `translucent_draws` (`12230-12243,12275-12335`), so a React GLB that remains a caster loses its WGPU caster role. The synthetic native Icon also enables a sun unconditionally and has no role distinction when material is absent (`Scenes/🦀️.rs:6657-6688`).

Smallest coherent packet: add explicit `casts_shadow` and `receives_shadow` data at the scene-draw/instance boundary; derive it from the React rules above before preparing the packet. Traverse only casters for `PassShadow*`, skip shadow sampling for nonreceivers, and cover GLB, terrain, paint/overlay, transparent GLB, and Icon-without-material fixtures.

### P1 — caster selection uses the view frustum instead of the light frustum

WGPU discards each primary draw that does not intersect the **main camera** frustum before it ever reaches `ScenePass3d.draws` (`world/🦀️.rs:12190-12243`). That same already-camera-culled vector is the sole shadow traversal source. A caster outside the viewport but inside the directional light's frustum therefore cannot shadow a visible receiver.

Three calculates its shadow frustum from the shadow camera (`WebGLShadowMap.js:338-350`) and checks `object.castShadow` against that frustum while rendering the depth pass (`506-516`). The current React directional light keeps Three's default orthographic shadow camera `[-5, 5] × [-5, 5]`, near `0.5`, far `500` (`node_modules/three/src/lights/DirectionalLightShadow.js:14-17`); WGPU's directional projection uses the same normal-case bounds (`🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs:1263-1321`). The projection convention itself is not the defect; the input set is.

Smallest coherent packet: form a dedicated caster list before view-frustum culling, filter it with the directional-shadow frustum, and retain the existing view-culled lists for colour. Add an off-camera-caster/visible-receiver parity case.

### P1 — map size, filter, bias, and controls are not exact React behaviour

* React World sets no map size, bias, normal bias, radius, opacity, or softness. `LightShadow` defaults are a 512×512 map, `bias = 0`, `normalBias = 0`, `radius = 1`, and intensity `1` (`node_modules/three/src/lights/LightShadow.js:41-94`). React Icon PNG explicitly makes its map 1024×1024 (`react/🟦️.tsx:357-360`). WGPU hard-codes 1024 for every world and icon (`ui/.../draw/🦀️.rs:652-653,3689-3706`). Higher World resolution is a visible contract change, not equivalence.
* Both React Boolean World shadows and PNG Icon shadows resolve to current Three PCF. Its shader takes five per-pixel-rotated Vogel-disk samples with linear comparison filtering, giving five filtered comparison locations (`shadowmap_pars_fragment.glsl.js:94-149`). WGPU uses a fixed 3×3 grid at nine locations (`shaders/🦀️.rs:290-301`). It cannot reproduce the distribution or temporal/pixel pattern.
* WGPU subtracts `0.0005` from every comparison reference (`shaders/🦀️.rs:299`) and applies raster depth bias `{ constant: 2, slope_scale: 2 }` (`draw/🦀️.rs:2933-2940`). React's unconfigured directional shadow has zero shader bias and normal bias. This changes acne/contact-shadow behaviour.
* React World does not consume environment `shadow.opacity` or `shadow.softness`; WGPU applies both as opacity and kernel scale (`shaders/🦀️.rs:291,301`). Native Icon also synthesizes opacity/softness `1.0` (`Scenes/🦀️.rs:6668`), which still leaves filtering and bias different.
* React Icon SVG has no WebGL shadow-map renderer (`react/🟦️.tsx:439-451,519-526`). Native Icon's parsed request has no `format` field and always uses the synthetic World3d path (`Scenes/🦀️.rs:6617-6632,6703-6720`), so it cannot preserve the SVG no-shadow result.

The WGPU shader also rejects `uv == 0/1` and `ndc.z == 0/1`; Three permits inclusive UV bounds and only rejects depth above one (`shaders/🦀️.rs:287-288`; `shadowmap_pars_fragment.glsl.js:121-147`). This is a small boundary mismatch.

Smallest coherent packet: split World and PNG Icon shadow profiles. Preserve World 512/default bias/radius/opacity behaviour; preserve Icon's 1024 profile; use the same PCF sampling semantics; carry an explicit format or route SVG through an unshadowed path. Do not expose `opacity` or `softness` to World until React does.

## Raster ownership and cleanup audit

### P1 — a full retained set cannot re-offer its unchanged rasters

The table cap is 256 entries/256 MiB (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs:845-849`). Candidate ownership is collected across both `draw` and top-level `overlay` (`prepared/🦀️.rs:1576-1596`) and sealed before Engine/uploads (`renderer/🦀️.rs:13897-13932`). The ledger protects committed, candidate, and previous entries (`draw/🦀️.rs:925-1008`); unowned retirement consequently cannot remove a key in either full frame (`1128-1131`).

`ensure_raster_step` always reserves and creates a fresh staged texture when it receives an upload (`1992-2105`), without first recognizing an unchanged live key. At capacity, `prepare_admission_step` cannot create the staged replacement and returns `raster texture credits are owned by retained frames` (`1840-1868`). This is reachable: World intentionally re-offers reference pixels every render after abort safety and documents that it expects table key dedupe (`world/🦀️.rs:10656-10675`). A 256-key committed packet followed by an unchanged 256-key candidate therefore stalls/aborts rather than presenting, and multiple panes can fill the same shared packet limit.

Smallest coherent packet: attach a content generation/hash or table acknowledgement to each raster offer; only reuse a live key when that immutable content identity matches. Keep staging for changed pixels. Add tests for a second identical 256-key packet, a 256-key mixture split between draw and top overlay, cancellation after partial staging, and a full changed-set response that explicitly defines the required peak-memory/back-pressure policy. Key equality alone is unsafe because the payload can change.

### P2 — EngineCanvas allocates and retains a second unaccounted target per live surface

EngineCanvas renders into `build.texture/view` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs:1471-1477`) and stages that rendered texture into the raster table (`1492-1509`). It then allocates an equal-size `replacement_texture/view` (`1479-1490`) and publishes the unused replacement pair as `EngineGpuSurface` (`1266-1297`). The live pair is only replaced/retired or closed (`1257-1289,1581-1585`); it is not sampled or rendered by this module. The table admission accounts for the rendered target, not the second live target.

This is eventually released, so it is not a permanent leak. It nevertheless doubles the target memory retained for each EngineCanvas surface and bypasses the raster capacity accounting. Remove the replacement allocation/publication pair unless another verified consumer needs it; otherwise admit/account it as a separately owned resource and add a live-surface memory-cap test.

### Ownership paths that are sound in the inspected source

Candidate scan/seal occurs before texture admission, abort removes staged candidate work before clearing candidate ownership (`draw/🦀️.rs:2280-2290`), and commit promotes staged work before the ledger advances (`2262-2277`). After a commit, `AppPresentedRetirement` retires the previous prepared packet before releasing previous raster ownership and then scavenges unowned entries (`renderer/🦀️.rs:13423-13510`). I found no additional cleanup defect in that ordering, and the scan includes both normal and top-overlay raster channels.

## Verification boundary

This is a static production-path audit. I did not run tests or builds. Runtime WGPU activation remains unverified here because the observed shell failure was in concurrent Hub edits, not a shadow assertion.
