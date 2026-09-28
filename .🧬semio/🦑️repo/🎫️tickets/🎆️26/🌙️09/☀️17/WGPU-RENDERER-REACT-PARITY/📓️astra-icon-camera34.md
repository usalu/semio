# Icon camera parity — batch 34

The native request drops orthographic projection and fit. Schema-first neutral cases now cover raw orthographic zoom, half/double preview scaling, landscape/portrait sphere fitting, and perspective zoom with fit. These are authored expectations pending the actual Three.js camera oracle and native implementation.

A direct generic World fit bridge is not sufficient without further audit: Icon fitting measures the loaded model before outline inflation, preserves perspective zoom, and scales an orthographic request from output pixels to preview pixels. The World projection-content framer must not override an authored Icon camera.

## React/Three.js oracle

The schema validation plus all eight camera geometry cases passed through actual production `iconRenderCameraPose`/`buildIconCamera` and Three.js world/projection matrices: **9 passed**, 569 unrelated tests filtered, Vitest 10.09 s, command exit 0. Cases verify pose, family, zoom, perspective effective FOV, centered target, and orthographic pixel displacement after preview scaling. Native implementation and runtime are still pending.

## Native source checkpoint

`IconRenderRequestFields` now retains projection and fit. The production resolver consumes the actual published subject lease AABB, resolves the same sphere-fit pose as React, carries perspective zoom through effective FOV, and scales orthographic zoom from request pixels into the aspect-fit preview. Both camera families emit a complete typed mode/free spec plus `projectionFrame: preserveCamera`. The new native law compares all eight shared cases through the production resolver and WGPU projection matrices. It has not yet executed; no new Cargo invocation was added while existing focused queues drain. SolFlow owns the shared projection-frame policy integration.

The initially considered generic `fit_json` approach was rejected: it fits using already zoom-adjusted FOV and includes outline inflation. These disagree with the actual Icon request semantics. No new fit revision field or producer changes are needed for a deterministic render-only pose.

## Actual Icon frame geometry oracle

The first Chromium measurement found another actual React defect: `rounded-full` resolves to `--radius-full: 0rem` in the shared theme, so ellipse shot frames have square borders and no host ellipse clipping. Landscape measured outer `[0,50,400,200]`, inner `[2,52,396,196]`; portrait `[125,0,150,300]`, inner `[127,2,146,296]`. The first run failed the ellipse-radius assertion (1 pass/1 fail). The semantic ellipse class now uses an explicit 50% radius; the focused rerun is pending. This preserves customization of control corners while honoring the authored shot shape.

## Frame oracle green and native layout checkpoint

The strengthened actual Chromium host oracle passed **2/2** after checking exact shared outer/content/footer rectangles, border2, and semantic ellipse radius50%; latest Vitest17.68s, command exit0. Temporary measurement logs were removed. Caption reserves24px (16px line plus8px bottom padding); the native frame now reserves this space, wraps captions, insets the actual content2px, and removes the incorrect minimum image scale that overflowed large shots. Ellipse chrome now paints a bounded128-segment two-pixel ring. New native layout and ring laws are authored but have not executed.

A complete GPU presentation path remains in progress with SolFlow: no grid/gizmo or interactions; true ellipse mask and authored source aspect in the physical image rectangle. The image’s inner396×196 box for a400×200 frame intentionally differs in aspect from the source shot, so the projection uses source aspect before physical presentation. Badge clipping/geometry still needs completion.

## Font-Loaded Badge Oracle and Native Gate Drain — 2026-09-28

The Chromium frame run6 passed2/2 after loading repository Anta and Share Tech Mono TTF bytes directly with FontFace. This established badge rectangles, sizes, line heights and padding; these measurements now live in the neutral fixture. The native fixture law now uses FontAtlas::shaped_default(), which matches runtime font shaping rather than the synthetic fixed-advance atlas. Run7 tightened loaded-font assertions and failed on the font readiness check; run8 inspects both face statuses and strips either quote style when replacing CSS faces. Badge assertions are not yet accepted as green.

The old empty-dock native queue terminated before assertions after48m49 with BoardHost::camera missing in its dependency snapshot. Sol confirmed both camera and set_camera_silent exist in current BoardHost source; the failure is retained as a failed build, not a test pass. No root Cargo job remains. The projection agent is running one exact focused publication law on its coherent atomic-frame source.

## Accepted Font and Badge Geometry — 2026-09-28 00:04 CEST

Frame run9 passed2/2 (Vitest18.28s, Nx21.2s) with the exact neutral badge bounds, font size, line height, padding and family assertions enabled. Chromium confirms both embedded Anta and Share Tech Mono are loaded. The test removes matching CSS FontFace rules before installing local binary font faces: FontFaceSet.delete cannot remove CSS-connected faces, which caused run7/run8 readiness failures despite the correctly loaded binary faces. All temporary badge console logs have been removed. Native shaped-font geometry remains unrun.

Icon status now has a neutral EN/DE fixture and actual React-host lifecycle oracle. The initial RED run failed on the missing localized failure heading. Both renderers now use selected-language empty/loading/failure labels; React retains the raw port diagnostic beneath its stable failure heading. A first postchange run caught a test adapter mismatch (`queryByText` absent on the owned render result); the corrected test uses the scoped container text and is running. Native scene lifecycle and shell locale carriage laws are staged, not executed.

## Localized Status Oracle Accepted — 2026-09-28 00:05 CEST

Status run2 passed2/2 (Vitest12.41s, Nx15.4s). The real React host renders empty/loading/failure in both selected languages, preserves the rejected-port diagnostic, and emits the ready image without stale status. Native SceneChromeLabels and lifecycle laws are staged and source-coherent but unrun. The Icon synthetic World now supplies presentation_json: no grid/gizmo/interaction, transparent clear, request shape mask and source aspect. Sol is finishing the GPU mask before the next artifact build.

A read-only follow-up is checking whether actual asset fetch/decode failures reach the Icon status predicate; snapshot faults alone may not be a complete transport-failure signal. This remains an open verification item.

## Asset Miss and Foreground Ellipse Integration — 2026-09-28

The decoder already retains refused URLs in World3dState.asset_url_misses, and world3d_asset_url_missed is a public reader. Icon now uses that existing refusal state together with snapshot faults and mesh residency, so a terminal decode miss leaves Rendering. A native lifecycle law marks the actual state and proves the miss is URL-specific with no snapshot fault; it is staged, not run. The expanded React status oracle remains GREEN2/2 (Vitest16.45s, Nx21.6s).

Transport failures are a separate unresolved boundary: native fetch failures currently record_frame_fault, and the browser worker catch quarantines non-reference assets with asset-stream-fault. This is not solved by the Icon status change, and remains a full-parity gap requiring failure-vs-capacity classification and owner cleanup tests.

The production UI instance packet now carries an opt-in ellipse rectangle; regular instances initialize it to zero. The UI shader clips coverage in logical coordinates with derivative antialiasing. Icon applies this only to emitted badge/status quads, preserving the frame border and footer. A shared probe fixture plus actual Chromium CSS and production WebGPU shader readback test is running. Native layout and pipeline compilation are not yet accepted.

Shooting React serve is running on6019, but reports its staged app component differs from current sources. It can locate and inspect the existing app route; it cannot establish a fresh paired acceptance receipt until activation rebuilds it.

## Actual UI Shader Readback Accepted — 2026-09-28 00:20 CEST

Frame run11 passed **3/3** (Vitest15.48s, Nx18.5s, exit0). The additional test compiles the production UI WGSL in real Chromium WebGPU and reads back both solid badge and glyph instances at shared rectangle/ellipse probes. The mounted CSS host compares the same probes and exact loaded-font geometry. The pointer-events override is scoped to test hit measurement and restored afterward. Native pipeline/layout laws remain unrun.

WASM24 is compiling through the repository Trunk target with two build jobs. The queued projection native process remains alive under Cargo PID91210; no second native batch was started. Two prescribed subagents exhausted account usage; the remaining Sol agent is repairing transport-failure isolation and root continues implementation and verification.

## Remaining Icon Stroke Gap — 2026-09-28 00:30 CEST

Production React applyIconMaterial adds Three.EdgesGeometry at its default1degree threshold, black by default, material.stroke override, and none/transparent disabled. Native IconRenderMaterialFields omits stroke entirely; its synthetic World path has no equivalent edge emission. The native GLB materializer currently seals Mesh3dSchema with edges0, so the existing lease edge reader is not sufficient. Correct support requires a bounded/cancellable edge derivation using local mesh primitive transforms (React scales each outline1.001 before its node transform), plus the presentation-owned color/visibility and a third-party Three oracle. This is a confirmed source gap, not a completed feature.

## World Mask Oracle and Narrow Frames — 2026-09-28 00:35 CEST

Frame run12 passed4/4 (Vitest20.31s, Nx23.6s) including actual production World3d postprocess WGSL. Patterned captures and backdrops verify per-pixel sampling in an offset viewport, restoring backdrop outside the ellipse, authored scene clear, and untouched pixels outside the scene viewport. This proves the shader, not the native cursor ordering.

A fifth narrow32×256 portrait case exposed two more differences. React shrink-to-fit wraps its badge into two lines; native used one full-width line. Rectangle foreground also lacked the frame's overflow clip. Run13 failed2/4: the first failure rejected the authored one-line size assumption; the UI shader fixture additionally demonstrated missing rectangular scissor. Native now uses face-specific wrapping through the existing greedy wrapper, the actual min/preferred badge width rule, and a scoped intersecting scissor for badge/status. The neutral fixture and native lines/parent-scissor law are updated. Run14 is running; native laws remain unrun.

## Build Failure Correction — 2026-09-28 00:36 CEST

WASM24 failed E0599 at EngineCanvas::puzzle_board_camera. Current source's camera getter at normal-board3599 belongs to BoardWheelPlan, not BoardHost. The earlier report treating the old native failure as stale input was incorrect. Root added BoardHost::camera adjacent to its existing set_camera/set_camera_silent, reading the actual accepted pose. WASM25 has started with this correction and the current Icon clipping/wrapping source. The native projection queue remains the single native invocation.

Shooting route exploration also observed the alternate fixture reverting to Default Base Icon after keyboard selection, with no browser error/warning. The app component is still stale, so this is a recorded exploration symptom requiring a fresh app build before diagnosis or acceptance.

## Five-Case Frame Oracle Accepted — 2026-09-28 00:37 CEST

Frame run14 passed4/4 (Vitest15.51s, Nx18.1s, exit0). The fifth case verifies actual narrow-frame badge shrink-to-fit width56.59375, two15px lines, native-equivalent rectangle scissor, and current production UI and World shaders. DOM Range geometry confirms the rendered lines are `32×256 ·` and `rectangle`. Native source and fixture laws remain unrun.

## WASM25 Shared Dependency Failure — 2026-09-28 00:41 CEST

WASM25 failed before renderer compilation: the concurrent framework-plugin operation-progress impl targets VcsArtifactApp<A> while its callers are generic VcsArtifactApp<A,M>, so dispatch_operation_cancellation and take_operation_progress_scope are unavailable there. Root sent the exact mismatch to the Raster owner and is preserving their repair. Native projection PID91210 has now acquired compilation after about40minutes queued. No second native batch or blind WASM retry was launched.

## Shared Build Restart — 2026-09-28 00:44 CEST

The Raster owner corrected the operation-progress generic member impl. The prior projection native run ended42m19 before assertions on that same dependency error. Root started native-parity35 (session74954) and WASM26 (session52378), both jobs2. The native filter covers projection publication, Icon family/locale, and Display ordering; the exact new unavailable_native_asset_isolated_to_its_exact_world_owner law needs a subsequent focused warm run because its name does not match the initial asset-miss regex. None of these native assertions are accepted yet.
