# World3d and IconRender Scene Lighting Packet

Source-stable on 2026-09-20. This packet implements the bounded environment-lighting and neutral-material slice identified in `📓️astra-terra-scenes-current.md`. It does not change World input snapping, reference-image residency, window lifecycle, template dragging, or asset retirement.

## Result

The production World3d path now resolves the configured ambient light, enabled sun, camera position, and neutral material into `ScenePass3d`. The WGPU draw preparation carries that complete record into a 160-byte aligned uniform shared by opaque meshes, translucent meshes, lines, and textured planes. The canonical WGSL uses the values for ambient indirect light, a configured directional sun, and GGX direct lighting with metalness, roughness, and emissive intensity. Selection and hover keep their semantic emissive override.

When the configured sun is disabled, the shader keeps React World3dHost's current fallback rig: hemisphere light plus the three fixed studio directional lights. Ambient remains present in both branches. World environment emissive is active only when the neutral material color override exists, matching the React neutral-GLB branch. IconRender supplies React's `#9aa0ab` neutral color when an Icon material record omits color, marks its sun enabled, and reaches the same World pass contract.

The backend-neutral render scene and the WebGPU, D3D12, and Metal GPU records now expose the same camera, lighting, and neutral-material fields. Their WGSL, HLSL, and MSL shader mirrors use the same environment channels and compact GGX equations. No runtime dependency was added.

## Shared contract and laws

`ui/render/🧬️schema/🔣️.json` defines `World3dLightingFixture` and its environment/material/vector records. The neutral fixture at `♾️infinite/🌍️world/🧪️fixtures/🌞️scene-lighting/🔣️.json` deliberately uses non-default ambient color/intensity, sun color/intensity/direction, metalness, roughness, emissive, and emissive intensity. Both the World and IconRender laws consume this same fixture.

Coverage added by the packet:

- The actual World widget builds a real scene pass and asserts that the fixture's camera, linear colors, intensities, material fields, and sun direction reach that pass.
- IconRender serializes the same fixture into the exact World environment record.
- GPU-uniform laws assert the 160-byte layout and every packed environment channel.
- The canonical shader-contract law requires the environment/GGX semantics in WGSL, HLSL, and MSL, and requires byte identity between every canonical World WGSL variant and the WGPU-target copy.
- A Naga test parses and validates the mesh, line, and textured World WGSL modules.
- The independent React-side oracle validates the shared schema, constructs Three `AmbientLight`, `DirectionalLight`, and `MeshStandardMaterial`, uses the React `sunPositionFromAzimuthElevation` helper, and verifies the fixture's light direction and standard-material inputs.

## Validation completed here

- Focused React/Three oracle through the existing Nx target: **1 passed, 652 skipped**. Command: `NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache -- --run '../../../../🧪️tests/🔬️engine-contract/🟦️.ts' --silent=false --reporter=verbose --testNamePattern='shared World3d lighting fixture'`.
- Rust source parse/format check: `rustfmt --edition 2021 --emit stdout` succeeded for all 21 Rust files touched by this packet.
- The schema and fixture parse as JSON; the focused oracle also passed Ajv strict validation against the new schema definition.
- All three shared-fixture include paths resolve to the same checked-in fixture.
- A source scan found no remaining direct `World3dGlobals` or backend `WorldGlobalsGpu` initializers bypassing `from_pass`, and no stale 80-byte World uniform minimum. The 256-byte dynamic-ring stride remains correctly aligned.

The Rust, Naga, native GPU, wasm, and paired runtime/pixel verdicts are intentionally pending the root-owned build and activation queue. This agent did not start Cargo, native, or wasm builds.

## Exact remaining asset-material boundary

The current native GLB materialization path represents geometry channels and per-vertex color but does not yet represent authored GLTF metallic/roughness materials and textures as mesh material resources. This packet preserves the existing vertex colors and separate textured-reference path, and it does not apply an Icon neutral override when `request.material` is absent. Exact preservation and rendering of authored GLB material/texture resources therefore remains a separate asset-schema and residency packet. Shadows also remain outside this packet because the current native pass has no shadow-map consumer.

The packet is source-stable for root integration. Any compiler or shader-validator finding from the queued native/WGPU build takes priority as a bounded integration repair.
