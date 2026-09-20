# Reference Visual Parity

## Executable React Contract

The current `World3dHost` publishes authored reference `id`, URL, origin, width, locked state and opacity, and constrains the reachable source to `mediaKind: "image"`. `WorldReferenceLayer` then uses the authored id for group identity, selection and hover. The retained WGPU record currently drops that id and returns the URL from picking, so two authored references sharing a URL cannot remain distinct.

React resolves reference visibility and appearance in this order:

1. Hidden references are absent unless explicitly revealed. Locked references remain visible, reject picking and multiply authored opacity by `0.35`.
2. Selected, inspectable references multiply the resulting content opacity by `0.5`, place `--active-base` behind the image and use the primary/active paint for the outline.
3. Hovered references, and revealed hidden references, place `--hover-base` behind the image and use `--accent-secondary` for a `0.9` opacity outline.
4. The background, image and outline use render orders `-11`, `-10` and `-9`. The image and background are `DoubleSide`, transparent, `depthWrite=false`, `toneMapped=false` `MeshBasicMaterial` draws. The outline is the source `LineBasicMaterial` policy.

The reachable ordinary-image route uses `TextureLoader` and leaves the texture at Three `NoColorSpace`. The lower `ReferenceMediaPort` explicitly assigns `SRGBColorSpace` only to canvas rasters produced for SVG and PDF. That lower route is not currently reachable from `World3dHost`, which always publishes `mediaKind: "image"`. The neutral contract therefore keeps both profiles explicit and does not apply one blanket transfer policy.

Principal source anchors:

- `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx:7583`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:179`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3805`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx:3967`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:682`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:689`

## Neutral Schema and Fixture

`framework.world3d.reference-visual/v1` is separate from the shared raster ownership contract. Its two identity rows deliberately share one URL while retaining distinct authored ids. Source profiles record the reachable `TextureLoader`/`NoColorSpace` image lane separately from the currently unreachable `CanvasTexture`/`SRGBColorSpace` SVG/PDF lane. State rows cover authored opacity, lock dimming, hidden retirement, hidden reveal, hover, selection, locked-selection suppression, both light and dark hover semantics, content/background composition and outline pixels.

Files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧬️schema/🖼️reference-visual/🔣️.json`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🖼️reference-visual/🔣️.json`

The fixture is compatible with the proposed shared full-resolution raster owner: it describes source color space and visual state while leaving decode, pooled pixel ownership, upload progress and cancellation to `scene-raster-ownership`.

## Actual Three Oracle

The permanent Bun/Nx command bundles installed Three r182 into fresh Chromium, renders the exact layered `MeshBasicMaterial`/`LineBasicMaterial` scene, validates the schema and identity/state equations, and reads RGBA8 pixels. It uses a 64×64, DPR 1, no-antialias, non-premultiplied WebGL context on ANGLE Metal / Apple M1 Max. Thirteen recorded rows reproduced byte-exact, including the deliberately different transfer result for the same texel and opacity:

- reachable image `NoColorSpace`: `[82, 118, 141, 255]`;
- explicit canvas-raster `SRGBColorSpace`: `[39, 83, 122, 255]`;
- locked: `[32, 56, 68, 255]`;
- selected content/outline: `[221, 91, 121, 255]` / `[248, 65, 84, 255]`;
- light hover content/outline: `[162, 191, 208, 255]` / `[106, 191, 182, 255]`;
- dark hover content: `[101, 135, 157, 255]`;
- hidden clear: `[6, 23, 28, 255]`.

Exact refreshed green command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-reference-visual-three-3" bun nx run '@semio-tech/framework-renderer-react:scene-shading-pixel-check' -- reference-visual
```

The target passed in 1.5 seconds Nx time. Evidence is in `🗑️generated/astra-reference-visual-three-3/world3d-reference-visual/three-reference/`.

The same installed-Three execution also constructs a `PlaneGeometry` from the authored width and decoded natural dimensions, translates it by the reference origin, and validates its bounding corners. The checkpoint-15 source dimensions `2275 × 2560` and authored width `50` produce a centered `50 × 56.2637362637` plane at `[7, 0, 0.01]`, with corners `[-18, -28.1318681319, 0.01]` through `[32, 28.1318681319, 0.01]`. This proves the remaining square runtime image is a decoded-dimension publication/redraw defect rather than a camera, origin, or plane-transform difference.

## Retained Production Implementation

The World record now retains authored reference id, opacity and locked state. The interaction registry admits each unlocked reference by authored id, even when multiple records share a URL, and the reference hover/select plans publish that id with `reference` granularity. Locked and hidden references do not enter the interaction registry.

The visual derivation follows live WGPU theme values on every paint:

- authored opacity is preserved;
- locked references multiply it by `0.35` and suppress hover/selection;
- selected content multiplies it by `0.5`, uses the selected/active background and the primary celebration paint for its outline;
- hover uses the row-hover background and secondary celebration paint at `0.9` outline opacity;
- hidden references retire from draw and interaction output.

The reference plane reads the published natural raster dimensions and derives height from `widthWorld / (naturalWidth / naturalHeight)`. The origin remains the plane centre, matching `WorldReferenceLayer`. The decoded pixel owner, generation lease, upload progress, cancellation and publication wake remain in the separate `scene-raster-ownership` packet; this change does not duplicate those responsibilities.

`TexturedInstance3d` carries a 96-byte instance with model, semantic background, content opacity and source-transfer profile. The Basic shader composes the semantic background and texture in the compatible encoded world attachment, applies the `NoColorSpace` recovery used by the reachable image host, skips ACES, and retains the distinct explicit `SRGBColorSpace` canvas profile. The canonical shader-contract and target shader constants are byte-identical.

Principal production and focused-law files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🖍️draw/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs`
- `🧰️framework/🔨️modules/🖱️ui/🖌️render/✨️shader-contract/🦀️.rs`

## Actual Production-WGSL WebGPU Acceptance

The second permanent route extracts the exact `WORLD3D_TEXTURED_SHADER` and `WORLD3D_LINES_SHADER` constants, creates the production 20-byte plane, 96-byte instance, 28-byte line and 240-byte global layouts, and executes their actual blend/depth/output-transfer policy in Chromium WebGPU. All thirteen Three rows reproduce byte-exact on Apple Metal WebGPU, including `NoColorSpace` versus explicit `SRGBColorSpace`, authored alpha over semantic background, light/dark hover, selection, lock suppression, hidden retirement and the two outline colours.

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true SEMIO_TEST_ARTIFACT_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️17/WGPU-RENDERER-REACT-PARITY/🗑️generated/astra-reference-visual-wgpu-1" bun nx run '@semio-tech/framework-renderer-react:scene-shading-pixel-check' -- reference-visual-wgpu
```

The target passed in 1.9 seconds Nx time. Evidence is in `🗑️generated/astra-reference-visual-wgpu-1/world3d-reference-visual/production-wgpu/`.

## Backend ABI Scope

The live WGPU target and canonical shader contract agree on the complete reference-texture ABI: 20-byte plane vertices, 96-byte instances, `background` at location 7/byte 64 and `appearance` at location 8/byte 80. The two WGSL constants are byte-identical, and the permanent WebGPU oracle constructs exactly that contract rather than a copied test shader.

There are no HLSL or MSL reference-texture variants to update. The D3D12 and Metal World encoders explicitly document and omit `SurfacePass::textured_draws`; their shader sets and pipeline builders contain only World mesh and line families. Their existing mesh-instance ABI remains 96 bytes and is unrelated to the new reference Basic instance. This S3 packet therefore keeps every implemented reference-texture backend consistent, while the already documented D3D12/Metal reference-underlay absence remains a separate backend capability gap. The native application path under acceptance uses the WGPU implementation exercised above.

## Command Integration

The oracle remains in the existing permanent `📜️script.ts` and is routed by the renderer React `ScriptRouter` and the Nx `scene-shading-pixel-check` target. `.vscode/launch.json` exposes the default S1 regression, S2 reference, material, current diagnostic, ordered, painted and celebration modes, plus the Three and production-WGPU reference modes in the existing build group. JSONC parsing reports 397 configurations and the two reference entries are present. No package-level alias is required because the public executable surface is the Nx project target.

SVG/PDF pose, page selection and media-kind publication remain outside this packet because current `World3dHost` does not expose them. The full-resolution pooled raster lease remains with the separate raster-ownership packet; its acceptance must retain the natural `2275 × 2560` dimensions and schedule a redraw when those dimensions publish.

## Astra generic shader contract follow-up

Shader9 ran136 tests:135 passed and one old law failed because it required ACES in every World shader family, including reference textures. That contradicts the actual React reference `MeshBasicMaterial toneMapped=false` policy and the13-row Three/WebGPU oracle. Astra corrected the law to require ACES for Standard meshes and ordinary lines, explicit Basic attachment transfer without ACES for reference textures, and no World transform in shared UI/blit shaders. Production shader code is unchanged by this follow-up. Shader10 remains queued behind the active native renderer gate.

## Root Shader 10 Validation

The uncached `@semio-tech/ui-render-rs:test` run completed with 136 of 136 tests passing after correcting the reference material tone-mapping policy law. This validates the authored shader policy and contracts; the full application reference geometry and appearance still require a fresh browser checkpoint.
