# Current WGPU Layout Launch And Journey Packet

2026-10-29. **SourceReady: source verification passed; live runtime acceptance pending.** Authored source work only. No activation, server, browser, native compile, setup, or cache maintenance was run by this lane.

## Declared Launch

The canonical commands are `repo run @semio-tech/framework-os-dev:activate-layout-wgpu-prototype` and `repo run @semio-tech/framework-os-dev:serve-layout-wgpu-prototype`. The second target depends on the first. No renderer manifest duplicate target is needed.

`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🎮️playground-targets/🟨️.mjs:241` generates these serve/dev targets from admitted playground metadata. The target carries the source composition path through `SEMIO_WGPU_COMPOSITION_PATH`. Its server program `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts` receives Script Original and serves finite producer artifacts through the owned Vite boundary.

The supplied catalog row `✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🎠️playgrounds.json` selects layout, `s.layout.layout@1/*#editor`, WGPU port **6179**, and composition `✏️s/🧑‍💻dev/🧩️catalog/🧊️wgpu/🟦️.ts`. That composition uses the generated Specific OS catalog and completed WGPU configuration. React6079 is a separate renderer.

After root releases rollout, run the retained journey with `repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/browser-journey.mjs`. Its default navigation selects layout and the declared demo example on6179. The browser context explicitly requests English. Generated evidence is written beneath the ticket `🗑️generated/layout-current` directory.

## Authored Journey Changes

`browser-journey.json` version3 declares current renderer/canvas/identity/surface suffix and canonical target names. `browser-journey.mjs` reads the actual WGPU accessibility mirror through `data-node-key` and `data-window`, observes native dock tabs via `aria-selected`, and captures the singleton `semio-wgpu-canvas`. Mirrored controls remain clipped; activation uses their supported native keyboard focus and Enter path rather than pointer visibility or DOM click dispatch. Inputs use native input and blur, and selects use the engine-owned option keys. No renderer test dispatch, fake-ready state, or editor API bypass is added.

The exact create, immediate undo/redo while Artifact stays published, story edit, real downloads, independent SVG/PNG geometry and PDF text, original-owner divergence, new-owner archive restore, and exclusion laws remain required. Health rejects boot faults, console failures, accessible alerts and unnamed actionable controls. The real mirror has revisioned generation identities and refreshes asynchronously; live selector/panel ownership verification remains necessary.

`browser-wgpu-source-cases.json` provides language-neutral cases for current mirrored page/frame identities, selected owner and closed-panel observation. `browser-wgpu-source-check.mjs` parses the actual journey with TypeScript and executes its real snapshot observer against independent jsdom DOM fixtures; this is a source/observation check, not runtime product proof.

## Verification And Limits

The ordinary semantic check command `repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/browser-wgpu-source-check.mjs` was started under normal admission (session63885). That original attempt completed with the harness extraction defect recorded below. The repaired final check has now passed. No `--light` test execution, capacity alteration, or synthetic process envelope was used.

Current assumptions requiring live confirmation: dock Blueprint IDs carry the current dot-free window suffix; active retained panel surface IDs equal their authored tab IDs; story textarea identity contains its authored story content key; staged numeric control is a descendant of its published argument row; option accessibility projection is refreshed before verification. All are derived from current source but have not been observed on a running server in this lane.

`validateCanvasCapture` measures painting variance only. It does **not** prove text/background contrast or complete visual layout. The required rendered-text-contrast step below now measures a specific real text control. Actual runtime measurement and screenshot review remain pending; no browser, global accessibility or end-to-end pass is claimed.

## Files Changed

- browser-journey.json
- browser-journey.mjs
- browser-wgpu-source-cases.json (new retained input)
- browser-wgpu-source-check.mjs (new retained input)
- browser-current-launch-packet.md (this report)

## Source Check Repair

Original session63885 completed with exit1 after normal capacity admission. TypeScript AST extraction incorrectly read the snapshot declaration initializer as a call; it is an arrow function whose body is the call. The observed failure was `TypeError: undefined is not an object (evaluating node.initializer.arguments[0])` at browser-wgpu-source-check.mjs:12. Corrected extraction now reads initializer.body.arguments[0]. This was a harness defect, not a product runtime observation.

## Grounded Rendered Text Contrast

The public, read-only `window.semioWgpuIntrospection.dumpStructure(surfaceId)` returns the current retained tree laid out and painted by the WGPU engine. Its semantic node path carries the declared control ID, and each measured node carries actual text, bounds, linear foreground/background RGBA and visibility/disabled state. `dumpFrameStats(surfaceId)` reports actual owner glyphs. This does not send an action or mutate editor state.

Source evidence: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🎯️targets/🧊️wgpu/🔬️introspection/🧾️dump-types-paths-and-walk/🦀️.rs` defines DumpNode, maps `UiNode::Input` to its real value plus `theme.text`/`theme.input_bg`, and walks mounted node bounds. `🏗️structure-dump-and-table-accessibility/🦀️.rs` reads the same engine tree and theme. The host `🌐️browser-host/🟦️.ts:73` owns the public read-only introspection interface. The staged numeric argument mapping in Shell `🚶️session/🧾️panel-records/🦀️.rs` creates `UiInput` with the argument's bare key and blur commit.

The journey's required **rendered-text-contrast** step measures the staged addFrame X field after actual keyboard/form input. It ties the projected node to the *same* current mirror control's `data-window`, `data-node-key`, input kind and native value251. It requires matching owner, visible enabled text, positive actual layout bounds, actual painted owner glyphs, opaque RGBA and WCAG normal text contrast >=4.5:1. The helper computes luminance directly from the renderer's linear color floats; it does not incorrectly gamma-decode them again. A transparent or missing background fails instead of inventing a backing color. Resting buttons publish transparent chrome backgrounds, so they are not the runtime measurement target.

`browser-text-contrast-cases.json` is the language-neutral measurement corpus; `browser-text-contrast.mjs` is the retained helper. The ordinary source checker independently compares the expected ratios and readability with the already installed colord accessibility plugin after converting linear colors to sRGB. Cases include black/white21:1, low contrast,4.5boundary, missing opaque backing, disabled/wrong node, absent glyphs, zero bounds and actual input text. No external runtime dependency is introduced to the product; these dependencies stay in the ticket verification tool.

This is a specific semantic render-color measurement, not an assertion about hidden mirror CSS, canvas variance, every control or pixel-level anti-aliasing. The runtime dump, same-node values, ratio and screenshot will only be produced when root releases the real launch and the browser executes this step. Its presence is mandatory in browser-journey.json and final contract verification; it cannot be skipped to claim journey completion.

## Subsequent Source Verification

After the AST extraction repair, normal admitted session35292 completed with exit0. Exact check output: `[DEBUG] WGPU source/jsdom cases passed: 3; rendered text/colord cases passed: 9`. The retained log is temporarily under `🗑️generated/browser-wgpu-source-check.log`. An additional input-kind case and the final runtime input target were then added; final source check session69570 completed with exit0. No semantic check used --light.

Additional retained inputs: browser-text-contrast-cases.json and browser-text-contrast.mjs. Runtime source readiness remains separate from the final browser execution result.

## Final SourceReady Result

Final command: `repo tools exec bun -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️16/LAYOUT-PLUGIN-END-TO-END/browser-wgpu-source-check.mjs`.

Session69570 preserved normal capacity admission and completed **exit0**. Exact terminal payload: `[DEBUG] WGPU source/jsdom cases passed: 3; rendered text/colord cases passed: 10`. The full final admission and command log is temporarily retained in `🗑️generated/browser-wgpu-source-check-final.log` for root's final evidence cleanup. No duplicate was started while an earlier source check remained active.

**SourceReady** means the current journey parses, its actual WGPU observer passes three independent DOM fixture cases, and its semantic contrast helper passes ten neutral cases with independent colord checks. It does not mean activation, native build, boot, actual event delivery, downloads, contrast measurement or archive restoration passed. Root still owns the separate release of the live WGPU activation/serve/browser gate.

## Asynchronous Mirror Recheck And Repair

Independent source review identified that immediate Select assertions and fixed150/200ms staging sleeps were shorter than the production mirror's400ms refresh floor plus native dispatch/introspection latency. This was a harness race; no live browser failure was inferred.

`browser-projection.mjs` now owns bounded condition polling. `stageAction` rereads current controls from the current mirror on every poll and waits for actual expansion, post-option closure, reopened selected-option retention and Escape closure. Fresh Locator factories reacquire the trigger and option before every native activation. The fixed staging delays were removed. Story's250ms delay was replaced with an actual mirror node replacement, matching value and native blur condition. The contrast witness explicitly asserts the native X field value equals the contract's251.

`browser-projection-cases.json` declares four language-neutral cases: delayed450ms open,850ms selected-option publication,600ms Escape close, and absent-publication timeout. The ordinary source checker tests the shared polling helper against independent jsdom replaceChildren publications with a controlled clock. Each positive case proves that the previous element is disconnected and still carries stale attributes, while the helper succeeds by reading the freshly published nodes.

Final post-repair command is the same ordinary `repo tools exec bun -- .../browser-wgpu-source-check.mjs`; session72725 completed **exit0**. Exact output: `[DEBUG] WGPU source/jsdom cases passed: 3; rendered text/colord cases passed: 10; delayed projection/jsdom cases passed: 4`. Full admission/output temporarily retained under `🗑️generated/browser-wgpu-source-check-projection.log`. SourceReady is restored for this revision. No activation, server, browser, test dispatch or live runtime claim occurred.

Additional retained files: browser-projection.mjs and browser-projection-cases.json. Real WGPU event delivery, actual contrast projection and media/archive journey execution still await root's launch release.

## Contract-Driven Launcher Repair

Independent audit found launch-layout.py still selected historical React activation and serve targets. Root normal managed Python RED80257 exited1 and reproduced the exact activation mismatch. The launcher now reads renderer.activationTarget and renderer.serveTarget from browser-journey.json rather than duplicating target names. Its portable workspace/catalog/backend behavior stays explicit.

Normal managed Python GREEN41248 exited0: four command-interception cases cover activation, serve, probe and an explicit Bazel backend. The test uses the neutral journey contract and unittest.mock to verify actual command, workspace and catalog; no target or browser is launched. Retained input is browser-launch-check.py; admission/terminal logs are in the generated browser-launch-red/green logs.

The low-complexity lane also fixed pre-effect output path validation; see browser-output-paths.md for neutral POSIX/Windows cases and independent Ajv/path checks. Root queued final ordinary WGPU source observer recheck5084 after this journey import change. Actual UI runtime acceptance remains pending.

Final ordinary WGPU source recheck5084 is terminal exit0 after path-helper integration:3independent jsdom observer cases,10colord contrast cases and4delayed projection cases. Launcher41248 also passed4intercepted-command cases. No real server/browser/host outcome is implied.
