# Checkpoint11 Runtime and Remaining Failures

## Artifact and Native Boundary

WGPU activation11 passed its full canonical 20-task graph in 6m15s. React activation5 passed its canonical 14-task graph in 3m22s. Five WGPU artifact hashes were sealed after renderer compilation, remained unchanged through both activations, and remained unchanged after the paired browser run. The wasm contains both unique bounded topology-journal and priority topology-refresh diagnostic strings.

UI15 passed 610/610 with no skips. World10 passed 224/224 selected tests with 243 other tests filtered. Native19 passed 1,172/1,173 with no skips. Its sole red result is the Display law reaching World3dState Drop before dynamic owners become terminal empty. The Chrome executor's final test-only cleanup may have landed after that compiler started; it requires a new runtime test receipt. No Drop guard was suppressed.

## Paired Browser Result

The 13-step paired journey completed with one WGPU outcome error and ten action/structure-ledger differences. Those ledger differences are not ten proven user-visible failures: DOM fullscreen entry/exit succeeds in both renderers despite different journal action counts. The one explicit outcome error is Settings close.

The new-window regression is now green: both renderers close every window, drag the real retained Display transfer handle into the empty dock, create a new window identity, and publish a live World3d body immediately. WGPU creates `puzzle3d-main-2` and React creates `puzzle3d-main-3`. Both then route orbit, pan, and zoom. No example switch is needed. Generated identities remain intentionally unpaired in the camera comparison.

This browser result establishes the normal creation path, not its producer-refusal branch. Terra's later source audit found that refresh_ui still acknowledges topology publication after swallowing a per-body render failure; that negative branch is assigned to Sol Chrome.

## Settings Close and Layout

General now has inline Tree controls, but the screenshot remains materially different. Driver children still paint despite default closed; all sections grow downward and the panel occupies almost the entire screen height.

The Settings PanelTab hit is `[1300, 974.39996, 67.69668, 22.4]`. The footer Settings Toggle hit is `[1285.0319, 974.4, 67.696655, 22.4]`. The probe deliberately resolves the Toggle, but its centre is inside both rectangles. The click removes the child tab hits without removing General's visible panel and records no toggle journal. React's lowest panel row is near y952 and its footer Settings button is near y975.

The root panel tab must sit above the footer band. This is a generic panel-bounds/chrome ownership failure; a Settings-specific routing override is not the repair. Sol Window owns this correction plus generic retained Section collapse and upward Tree flow. Select popup origin and accessibility visibility/options remain later blocking work.

## Camera Runtime Still Fails

The green native camera fixture is not final runtime acceptance. Both WGPU boot poses initially remain the old poses after the probe declares quiet. Perspective only fits later at the app Settings step; Top never fits before its window closes.

Later WGPU Perspective target is `[5.40000534, 2.33826852, 1.5]`, compared with React `[5.4054, 2.3406, 1.5015]`. Their near-exact 1.001 factor corresponds to React outline geometry scaling (`outline.scale.setScalar(1.001)` and border `lineSegments scale={1.001}`), which Three Box3 includes in the instances group. The native fit cursor measures mesh bounds alone. This is a source-supported explanation to validate in a rendered-bounds fixture.

Later WGPU Perspective eye is `[16.62047, -8.88220, 9.91535]`, compared with React `[16.6917, -8.9457, 9.7781]`; viewport widths differ too. All-visible-window scheduling, actual rendered extent, and direction preservation remain assigned camera work. Reports now separate camera component differences from viewport geometry.

## Chrome and Latency Evidence

The probe captures actual React computed font, icon, padding, and gap metrics for the next chrome-width repair. React Artifact and Settings use 11.2px Anta at weight500, 3.2px gap/padding, and two 16px icons (leading icon plus trailing drag handle). Artifact width is76.203125 and Settings81.546875. WGPU's narrower chrome has no matching visible trailing transfer handles in these controls. Preserve this evidence instead of attributing the whole width delta to font metrics.

The new latency aggregation is active. The final snapshot has2,023,371 scalar observations,1,501 unique phase-summary evictions, and zero refused observations. WorkerTick has a lifetime maximum2,271,600µs; transactionRouteIntents has a matching2,271,199µs maximum. These overlapping scopes are not independent durations. The aggregate now exposes the slow phase instead of overwriting its history with scalar retirement loops, but the long task remains a performance concern. The paired run included a concurrent native build and is not an isolated performance benchmark.

Evidence is under `🗑️generated/astra-runtime/paired-checkpoint-11`. No complete renderer parity is claimed.
