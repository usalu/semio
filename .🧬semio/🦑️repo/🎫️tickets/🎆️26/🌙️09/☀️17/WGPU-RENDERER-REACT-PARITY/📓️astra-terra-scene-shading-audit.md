# World3d Scene Shading and Theme Parity — Terra Audit

## Scope and evidence

Read-only source audit for the World3d visual mismatch. No production files changed; no Cargo task, build, or browser run was performed. The only runtime inspection was the installed Three material evaluator (`three` revision `182`): `MeshStandardMaterial`, `MeshBasicMaterial`, `LineBasicMaterial`, and `PointsMaterial` all report `toneMapped: true`.

The sealed evidence is:

- `🗑️generated/astra-runtime/paired-checkpoint-10-full/react/02-dismiss-tour.png`
- `🗑️generated/astra-runtime/paired-checkpoint-10-full/wgpu/02-dismiss-tour.png`

At that same checkpoint, the WGPU model is substantially whiter than the React model. The companion camera audit records a live-pose difference, so this audit does **not** assign geometric framing differences to shading. It isolates the independent neutral colour, lighting, normal, and output-transform failures that produce the white world even at an equivalent pose.

## Actual pipeline

| Stage | React reference | WGPU implementation | Parity result |
| --- | --- | --- | --- |
| Neutral source | `World3dHost/🟦️.tsx` resolves `MESH_STYLE_PAINT.neutral.fill` from live `var(--panel)` through `resolveMeshStylePalette`. | `🌍️world/🦀️.rs:scene_bridge_neutral_color` bakes `[0.78, 0.79, 0.82, 1]` into every source instance without an authored/environment material colour. | **Fail.** The producer has no theme and the retained neutral branch later returns that baked colour unchanged. |
| State palette | React creates a material from the selected palette row: fill, line, emissive intensity, opacity, and its celebrated conic variant. | `mesh_style_paint` has the seven nominal rows, but `world3d_style_paint` only applies fill/opacity to non-neutral instances. `WORLD3D_SHADER` supplies instance emissive only for selected (`.35`) and hovered (`.08`). | **Partial.** Highlighted/provisional `.2` and celebrated `.55` are not wired into the shader; celebrated conic is absent. Disabled opacity happens to survive. |
| Vertex-colour policy | `PaintTexturedMesh` keeps vertex colours only for neutral/disabled (`color=#fff`, `vertexColors=true`); selected/hovered/etc. use a solid semantic fill. | Vertex stage always computes `instance.color * vertex.color`. | **Fail for coloured geometry.** Neutral is tinted rather than preserving source colour, while active states still multiply source colour instead of becoming solid semantic paint. |
| Material parameters | Inline and painted meshes are `MeshStandardMaterial(metalness=0, roughness=1)`. A URL GLB rebuild retains its source metalness/roughness unless an environment material supplies overrides. | `SceneMaterial3d` is one pass-global default/override. With no environment material it is `(0, 1, black, 0)`, so no per-GLB source material survives. | **Equivalent only for the inline/painted default profile.** GLB material parity remains a separate input-model gap. |
| Working colour | CSS/hex colours reach Three through `Color`, with colour management enabled. | `parse_color` linearizes sRGB; WGPU `Theme` paints are already linear. Uploaded rasters are `Rgba8UnormSrgb`, so sampling produces linear values. | **Aligned.** Do not add a second sRGB conversion. |
| Normal | Three transforms with `normalMatrix`, the model-view inverse transpose. | `WORLD3D_SHADER` normalizes `mat3(model) * normal`. | **Fail under non-uniform scale.** WGPU needs the inverse-transpose of the world model linear block because its lighting vectors are world-space. |
| Direct and indirect BRDF | Three r182 `MeshStandardMaterial` uses `BRDF_GGX_Multiscatter` plus Lambert diffuse; `diffuseContribution = base * (1 - metalness)`. Both direct and indirect diffuse use `BRDF_Lambert`, hence `1 / pi`. | WGPU has a simplified GGX direct term and correctly applies `1 / pi` there, but returns `indirect * base * (1 - metalness)` with no reciprocal-pi factor. | **Fail.** The fallback ambient plus hemisphere diffuse is π too bright; the direct lobe is also not the current Three standard-material lobe. |
| Output transform | `WorldCanvas` supplies no `flat` or `linear`; R3F 9.7 configures sRGB output and `ACESFilmicToneMapping`. Every material class used by this World participates in that transform. | `WORLD3D_SHADER` returns HDR linear radiance directly. `WORLD3D_LINES_SHADER` and `WORLD3D_TEXTURED_SHADER` also return untransformed RGB. | **Fail.** Values above one clamp in the sRGB render attachment instead of being compressed by ACES. |

### Colour-space boundary

The WGPU target chain is already an sRGB boundary, not a double-encoding bug:

1. `GpuContext` chooses an sRGB view (`color_target_format`) for the offscreen scene, composite target, and final surface view.
2. `SceneColorTarget` stores the pre-glass scene, then `SCENE_BLIT_SHADER` samples it into the composite; the final blit samples composite to the sRGB surface. Sampling decodes and sRGB attachment writes encode, so ordinary in-range linear content stays linear through each intermediate operation.
3. The world shader instead writes **HDR** radiance to an 8-bit sRGB target. There is no ACES pass before the attachment write, so highlights saturate. Moving ACES into `SCENE_BLIT_SHADER`, or changing the shared scene target before first separating UI content, would tone-map the 2D UI and glass inputs as well.

The safe first boundary is therefore a shared `world3d_output_transform` called in every **World3d shader family**, immediately before its write to the existing sRGB scene target. It takes linear HDR RGB, ports Three r182 `ACESFilmicToneMapping` exactly (including exposure `/ 0.6`, matrices, fit, and clamp), and leaves alpha untouched. The attachment then performs the sole linear-to-sRGB encoding. UI, blur, composite, and present shaders remain unchanged.

### Lighting and normal details

The checkpoint's disabled-sun branch is represented on both sides by ambient white `1.15`, the same hemisphere colours/intensity, and the same three directional fallback lights. Configuration is not the source of the brightness mismatch.

The important numerical counterexample should become a fixture case:

```text
model scale      = (2, 1, 0.5)
object normal    = normalize(1, 1, 1)
current WGPU     = normalize(2, 1, 0.5) = (0.872872, 0.436436, 0.218218)
Three/world-correct = normalize(0.5, 1, 2) = (0.218218, 0.436436, 0.872872)

irradiance       = (1, 1, 1)
base, metalness  = (0.25, 0.50, 0.75), 0
current indirect = (0.25, 0.50, 0.75)
Three Lambert    = (0.0795775, 0.1591549, 0.2387324)
```

The second row is exactly the omitted `1 / pi`, before the output transform. A reciprocal-pi-only patch is insufficient for full material parity: current WGPU uses a simplified GGX visibility term while Three r182 uses `BRDF_GGX_Multiscatter`, `D_GGX`, `V_GGX_SmithCorrelated`, `F_Schlick`, and Three's energy compensation. The reference profile can be bounded to the properties React actually creates for inline/painted meshes: base colour, vertex colour/map policy, metalness, roughness, emissive, and opacity; no clearcoat, sheen, transmission, normal map, or IBL need be introduced.

## Bounded Sol implementation packet

### S1 — default World3d parity

1. **Make colour provenance explicit in the retained scene schema.** Add a language-neutral colour-source discriminator to `World3dSnapshotItem`/`Instance3d`: authored instance colour, environment material colour, or semantic neutral. Preserve it from `publish_world3d_scene_bridge_snapshot`; do not materialize the semantic-neutral case as `.78/.79/.82` there.
2. **Resolve semantic neutral at retained paint time.** In `world3d_style_paint`, choose `theme.panel` only for a neutral instance whose source is semantic-neutral. Preserve authored and environment colours. This lets a live appearance change repaint retained worlds without a new guest snapshot. Update the existing unit law that currently asserts the baked neutral is retained.
3. **Carry the resolved material mode with each draw.** Encode whether vertex colours are preserved and the resolved static emissive colour/intensity. Match React's neutral/disabled vertex-colour rule and its selected, hovered, highlighted, provisional, and disabled static rows. Keep celebration's animated conic in S2 below rather than falsely reducing it to a flat fill.
4. **Port the non-optional r182 standard-material math to `WORLD3D_SHADER`.** Use the current Three formulas for the bounded profile, especially Lambert for both direct and indirect diffuse and the standard GGX multiscatter direct term. Replace the vertex normal transform with a guarded inverse-transpose of the model 3×3 block, then normalize in world space.
5. **Apply one World-only ACES transform.** Add the r182 ACES helper to mesh, line, and textured World3d fragment exits. Do not alter `SceneColorTarget`, `PreparedCompositeTarget`, `SCENE_BLIT_SHADER`, surface formats, or any UI shader in this packet.

This fixes the screenshot's default neutral world, state rows that are static, scaled normals, direct/indirect energy, and output compression without introducing a new runtime dependency or touching Sol's camera work.

### S2 — explicitly separate remaining material/state work

- Port the celebrated conic material as an instance conic-stop payload plus the same deterministic time/phase contract React uses; validate its animation separately.
- Extend the scene-asset/material contract before promising GLB parity. React retains GLB metalness/roughness when there is no environment override; the current pass-global `SceneMaterial3d` cannot represent that. Do not replace per-mesh material provenance with another global override.
- Cover texture-map/vertex-colour combinations after S1. The existing WGPU textured pass is unlit while React's `PaintTexturedMesh` is `MeshStandardMaterial`; its output transform alone is not a lighting-equivalence fix.

## Validation seam

Add a schema-first, language-neutral fixture at:

`🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🎨️scene-shading/🔣️.json`

and its schema beside the existing World fixture schemas. The fixture should declare linear source values, model transform, normal, camera, fallback/sun lights, material mode, colour provenance, and state. It should carry display `rgba8` oracle samples rather than hand-written sRGB conversions, with the reference metadata `{ threeRevision: "182", fiber: "9.7.0", toneMapping: "ACESFilmic", outputColorSpace: "srgb" }`.

Required cases:

1. neutral semantic source in light and dark appearances;
2. authored and environment-material colour provenance (to prove they are not rethemed);
3. fallback ambient/hemisphere case containing the reciprocal-pi numeric sample above;
4. non-uniform model normal case above;
5. neutral/disabled vertex-colour preservation and selected/hovered/highlighted/provisional solid fill plus emissive;
6. an HDR case whose only accepted expected output comes from the React browser render oracle.

The **primary oracle** should mount the controlled fixture through the installed React `WorldCanvas`, render a fixed pixel region with antialiasing disabled, and read display RGBA bytes from the actual WebGL canvas. A WGPU offscreen/readback test consumes the same JSON and compares the same positions within a small byte tolerance. Add a small TypeScript source guard against the installed Three chunks (`BRDF_Lambert`, physical indirect diffuse, ACES) so upgrading Three forces re-recording the browser oracle. This uses already-installed Three/R3F only; it is not a runtime dependency.

## Handoff boundaries

- The visual blocker is independent of the active World camera fitting work; do not edit camera/orbit code for S1.
- Do not interpret the WGPU white screenshot as an sRGB double-encode. Linear parsing and sRGB targets are already intentionally paired.
- Do not tone-map the shared scene/composite blit until World and UI are separately colour-resolved; that would change UI and glass appearance.
- Existing `the_mesh_style_table_is_reacts_whole_paint_table` is valuable but incomplete: it validates palette values while its final assertion currently codifies the neutral-theme defect and does not exercise shader emissive, vertex-colour mode, BRDF, normals, or ACES.

## Source basis

- React palette/material/light scene: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx`
- React canvas defaults: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/🟦️.tsx`
- WGPU World producer/style/render: `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs`
- WGPU scene records: `🧰️framework/🔨️modules/🖱️ui/🎬️scene/📐️math/🦀️.rs`
- WGPU shader/target/present: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🎨️shaders/🦀️.rs`, `🖍️draw/🦀️.rs`, `🧊️gpu/🦀️.rs`
- Installed reference: `node_modules/three` r182 shader chunks and `@react-three/fiber` 9.7 configuration.
