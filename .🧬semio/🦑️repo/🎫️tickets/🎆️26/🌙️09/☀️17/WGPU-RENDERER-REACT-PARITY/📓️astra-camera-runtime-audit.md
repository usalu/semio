# Checkpoint10 Camera Runtime Audit

The fresh full paired journey now reads React `data-viewport-camera-json` as well as WGPU liveCamera. Its boot and dismiss-tour snapshots prove identical delivered camera seeds but different live poses. This excludes producer camera-seed divergence as the initial cause.

| Pane | React live pose | WGPU live pose |
| --- | --- | --- |
| Top | position[5.4054,2.3406,21.3572],target[5.4054,2.3406,1.5015],zoom7.7595 | position[7,0,9.46],target[7,0,0.01],zoom7.86963 |
| Perspective | position[16.6917,-8.9457,9.7781],target[5.4054,2.3406,1.5015],fov50 | position[9.17,-5.67,4.2575],target[3.5,0,0.005],fov50 |

The actual pose records are in `🗑️generated/astra-runtime/paired-checkpoint-10-full/steps.json`. React viewport width is523.77/1053.92 versus WGPU531.2/1062.4 due to separate unresolved dock spacing. That small width difference cannot explain the missing perspective reframe or different target.

Source trace: React WorldAutoFit uses producer bounds when available and otherwise measures the actual rendered instance group with Three Box3. WGPU `sync_world3d_scene_fit` requires both boundsMin/boundsMax and returns without measuring renderer-owned geometry when they are absent. WGPU runs this fit before projection-content framing and document-lane sync. Its projection helper handles only parallel cameras and computes bounds from instance positions/reference footprints. The older comment claiming this matches a visually observed React perspective is contradicted by this fresh runtime. The next implementation packet must inspect the actual fit lane and loaded instance bounds, add bounded incremental extent measurement from renderer-owned geometry, and follow current React ownership/revision/seed rules. Do not auto-fit merely from placeholder instance origins or silently override a user-moved camera.

React sun DOM record says enabled=false, so three fallback directional lights and a1.35-intensity hemisphere light are active in addition to ambient. WGPU material appears nearly white compared to React gray in both theme runs. Further audit must verify the fallback light/material/color-transfer and normal paths; no lighting root cause is yet established.

## Material Source Divergence

React's neutral mesh palette resolves `var(--panel)` live with the active theme; its hover/selection/disabled/provisional styles likewise resolve semantic tokens. WGPU `scene_bridge_neutral_color` instead hard-codes linear[0.78,0.79,0.82,1] unless an environment material color is declared. Its accompanying assertion that React has the same theme-independent fallback is false. This explains a concrete missing theme path and is a required scene-paint repair.

The WGPU shader already includes the hemisphere and all three fallback directional lights, so their absence is not the lighting cause. Its indirect contribution is `indirect * base_color * (1-metalness)` while direct diffuse includes reciprocalPI. Compare the installed Three physical-shader indirect BRDF and final tone/color conversion before choosing a correction; there is no accepted photometric oracle yet.

Installed Three source confirms missing reciprocalPI for WGPU indirect diffuse: `lights_physical_pars_fragment.glsl.js` RE_IndirectDiffuse_Physical multiplies irradiance by BRDF_Lambert; `common.glsl.js` BRDF_Lambert returns reciprocalPI×diffuseColor, and `lights_pars_begin.glsl.js` ambient irradiance returns the input light color unchanged. This is a concrete energy mismatch alongside the missing theme palette. The outgoing WGPU fragment currently returns lit HDR color directly; React Canvas tone-mapping configuration still needs verification.

## Repeatable Camera Evidence

The ticket interaction probe now writes `cameras.md` from the published live cameras. It pairs only identical visible window identities, reports eye/target/up/zoom/FOV deltas, and reports viewport width/height separately. It does not compare local WGPU origins against React page origins or use retired mesh diagnostics. Replaying recorded checkpoint10 through Bun/Nx reproduced the known Top eye delta of 11.89720, Perspective eye delta of 7.52170, and target component delta of 2.34060. This was report recomputation, not another runtime test. The camera implementation now has three passing actual-Three oracle laws; integrated native18 and fresh browser evidence remain pending.

## Initial Template Handoff Follow-Up

After checkpoint11, Sol traced the persistent direction discrepancy to initial template delivery. React's live Perspective direction equals the normalized threePoint template `[0.75,-0.75,0.55]`, approximately `[0.6277308,-0.6277308,0.4603348]`. WGPU's observed direction remains the delivered seed, approximately `[0.624695,-0.624695,0.468521]`. The Shell stores the per-window projection template, but its initial World3d residency path does not apply it; the existing `apply_world3d_projection_spec` is called by later row presses. Top's delivered seed masks that direction issue.

The assigned repair applies the stored template once to a newly resident matching World state, includes the actual rendered outline scale1.001 in fit bounds, and proves continued scheduling for every pending visible fit. Increasing a fixed per-paint work grant alone is insufficient evidence for the Top pane, which failed to fit across the complete checkpoint11 journey. A multi-pane production pump law and fresh browser camera diagnostics are required. These are source findings and assigned work, not an accepted runtime repair.
