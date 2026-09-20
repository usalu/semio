# World3d and IconRender Exact Shadow Parity

Source/API stable on 2026-09-20. This report supersedes the packet's earlier provisional 1024/nine-tap/opacity-softness implementation. It implements every production finding in `📓️astra-terra-shadow-residency-audit.md` for the reachable prepared `ui_wgpu` World3d and native IconRender consumer.

## Schema and independent oracle

The language-neutral contract is `🧰️framework/🔨️modules/🖱️ui/🖌️render/🧬️schema/🔣️.json#/$defs/World3dShadowParityFixture`. Its fixture is `♾️infinite/🌍️world/🧫️fixtures/🌑️scene-shadow-parity/🔣️.json` and fixes:

- World Canvas: 512 map, current Three PCF, zero bias/normal bias, radius one, intensity one, and ignored environment opacity/softness;
- Icon PNG: 1024 map;
- Icon SVG: no shadow map;
- opaque and transparent GLB cast-and-receive, terrain receive-only, Paint/overlays neither, Icon material cast-and-receive, and Icon without material neither;
- a caster outside the main-camera frustum but inside the directional-light frustum;
- five immutable Vogel-disk offsets rotated by Three's interleaved-gradient noise for a fixed fragment coordinate.

The React engine-contract law validates that schema, constructs real Three meshes, cameras, frusta and directional shadows, checks installed Three's PCF shader chunk, and independently reconstructs the five samples. Focused Bun+Nx result on the final fixture/schema: **1 passed, 656 skipped**.

## Scene contract and roles

`SceneDraw3d` now carries `SceneShadowRole3d { casts, receives }`. `ScenePass3d` owns a separate `shadow_draws` channel, and `SceneShadow3d` owns `enabled`, `map_size`, and the light view/projection. The pass no longer exposes React-dead opacity or softness as active GPU controls.

World assigns roles at final draw assembly:

- every GLB instance is cast-and-receive while the effective configured sun shadow is enabled, including provisional/transparent GLB;
- terrain is receive-only;
- component overlays, previews, traces, vertex/vortex markers, gumball geometry, lines and reference-image/Paint geometry are neither.

IconRender now parses the request format as the closed `svg | png` vocabulary. Only PNG with a material selects the 1024 shadow profile. SVG and material-absent PNG select the unshadowed profile. Ordinary World selects the 512 profile. Authored GLB materials/textures remain outside this packet because the audited React `GlbInstanceMesh` path also replaces them with the neutral material.

## Light-frustum caster channel and ownership

World creates styled/chunk-visible GLB instances once, then independently tests their transformed AABBs against the directional-light frustum and main-camera frustum. `shadow_draws` is formed before main-camera rejection; colour lists remain main-camera-culled. The exact fixture's `offscreen-caster` therefore appears only in the shadow channel while the visible receiver appears in both.

The caster channel is a retained owner rather than an unmeasured view:

- `needed_mesh_keys` is the union of colour-visible and light-visible meshes;
- `ensure_mesh`, the frame upload request, exact key/version residency filtering and the World mesh pool all include an offscreen-only caster;
- retained-output admission counts the caster vector capacity, mesh keys, instances and instance ids;
- the prepared cursor clears once and emits one caster instance per cancellable scalar;
- `DrawList::retire_step` drains caster ids, instances, mesh keys and vector capacity progressively;
- frame census includes caster draws/instances.

This preserves bounded admission, cancellation and retirement when a caster is absent from every colour list.

## GPU and shader consumer

Prepared traversal reads only `shadow_draws` entries with `casts == true`. Colour submission carries the owning draw's `receives` bit in `World3dGpuInstance.flags.z`; nonreceivers branch around shadow-map sampling.

The shadow target resizes to the pass-owned map size, giving World 512 and Icon PNG 1024. The depth prepass uses the default zero raster bias. The comparison sampler remains linear, matching Three's hardware-filtered PCF locations.

The canonical WGSL and WGPU copy are byte-identical. Their shadow function now matches current Three's reachable semantics:

- five Vogel-disk comparison samples;
- per-fragment interleaved-gradient-noise rotation;
- radius exactly one texel;
- comparison reference `ndc.z` with no shader bias;
- inclusive UV bounds and only `z > 1` rejected;
- no opacity or softness consumer;
- enabled and receiver early branches before sampling.

The dedicated shader law rejects the old fixed 3×3 kernel, `0.0005` bias, and opacity/softness uniform reads. The existing Naga law remains the authoritative root-owned compile/validation gate.

## Rust laws added or strengthened

- The exact neutral fixture maps to all World/Icon roles and both map profiles.
- A real World scene bridge/render pass retains a light-visible `offscreen-caster` in `shadow_draws`, rejects it from colour draws, retains the visible receiver, and proves the caster mesh is uploaded and pool-owned.
- Prepared traversal orders shadow before textured/opaque/line/translucent colour work and uses the dedicated caster channel.
- Disabled shadows publish no shadow scalar.
- Offscreen caster storage is included in retained-output accounting and reaches terminal progressive retirement.
- Icon PNG/material, PNG/no-material and SVG/material requests select the exact React profiles.
- The canonical shader law fixes the five-sample PCF consumer and Naga continues to validate every World WGSL variant.

## Validation

- Focused React/Three oracle through the existing Nx target: **1 passed, 656 skipped**. Command: `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run '../../../../🧪️tests/🔬️engine-contract/🟦️.ts' --silent=false --reporter=verbose --testNamePattern='neutral exact-shadow corpus'`.
- The render schema and exact fixture parse as JSON; Ajv strict validation passed inside the focused oracle.
- `WORLD3D_SHADER`, `WORLD3D_LINES_SHADER`, and `WORLD3D_TEXTURED_SHADER` are byte-identical between the canonical contract and WGPU target sources.
- Every touched Rust source parses with `rustfmt --edition 2024 --emit stdout`.
- No Cargo, native, wasm, activation or browser-generation job was started by this agent. Root owns the fresh UI/World/renderer/Naga build and paired runtime verdict.

## Boundaries and follow-up evidence

Terra identified the main prepared `ui_wgpu` renderer as the reachable shadow consumer. Native Metal/D3D12 declarations have no equivalent map producer/consumer and were explicitly outside that finding set, so this packet does not invent an unreachable render graph for them.

Root's immutable checkpoint-7 paired journey also found a World camera/viewport framing discrepancy and a separate gizmo rectangle defect. Those were observed before this shadow schema checkpoint. The gizmo owner is repairing its painter, and camera/projection binding remains a separate scene-camera packet; neither changes the shadow roles, light-frustum ownership, sampling profile or caster retirement implemented here.
