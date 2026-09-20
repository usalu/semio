# Next Parity Slices After Window and Popup Integration

Current-source reconnaissance on 2026-09-19; this is not a runtime acceptance report. Paths below are relative to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine` unless stated otherwise.

## Shell Geometry

`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` (`render_navbar_step`) still advances the brand/title/example/role cluster directly after leading tabs. The fresh React reference centers that cluster in a separate positioning region. At 1280×720, React's app name starts around x=375 and the example trigger at x=546; the old wasm baseline places them substantially farther left. The current Dock painter uses `theme.control_height` for the tab strip and only changes text/border for active tabs. React's measured Top cap wrapper is 28.765625 px tall, with a primary-colored active cap. A Terra audit should derive the exact shared-token formulas, narrow-width collision behavior, tab/utility/anchor hit geometry and focus/hover/active styling before a Sol implements them.

## Driver Editor Completeness

This is a confirmed current-source missing feature. `📌️ChromePanels/🟦️.tsx:514–540` exposes seven driver axes (labels, label tier, drag, chrome, gumball, tooltips and hotkeys), then save-label/save/delete controls. `🏛️ShellHost/🟦️.tsx:8499–8535` owns the draft/save/delete effects. The wgpu shell at `build_settings_general_ui` (around 7033) explicitly limits its implementation to a driver selector; there are no equivalent dispatch arms. This prevents customization available in React. The next implementation should use the same owned `OsUiDriver` contract, canonical preference event lane and localized control ids, with one neutral fixture consumed by both target tests.

## Tutorial Snapshot Completeness

`tutorial_capture_ui_snapshot` in the wgpu shell still publishes an empty interaction selection map and empty expanded-tree ids (around 19144–19151). React reads the actual interaction selection through its tutorial bridge and diffs both fields (`🏛️ShellHost/🟦️.tsx:1092–1110`, `7162`, `7174`). A fresh audit must trace the production apply path and retained tree state before implementation; the current empty capture cannot preserve selected or expanded state through tutorial takeover.

## Theme Contract Cleanup

Current wgpu source still includes the old five-paint `CustomChromeTheme`/`ChromeColorOverrides` model alongside the full `ThemeDocument` editor, with an `is_document` fallback. This conflicts with the repository's greenfield/no-compatibility rule and introduces a theme format React cannot consume. Remove the obsolete model and its tests once the canonical path is fully verified, updating owned fixtures directly. Do not add migrations or another compatibility adapter. Separately audit the current theme editor's paging against React's virtualized/scrolling behavior and the retained document budget; a prior report's declared substitution is not sufficient parity evidence.

## Scene Audit Should Recheck Current Source

Many Wave 14 source gaps are already repaired and should not be reimplemented: current source contains production relocate, modifier-aware scene input, Table/BlockList drag routes, Paint2d marquee/navigator calls, DiffView line gutters, and World3d sub-object/celebration lanes. The scene auditor should focus on fresh runtime framing, materials, glyphs, grids, picking and multi-app boots after the build, then identify only currently failing routes.

## Verification Infrastructure

The permanent parity runner is `🧑‍💻dev/🧪️tests/⚖️parity/🏃️execution/🟦️.ts`, behind `@semio-tech/framework-os-dev:parity` and `📜️script.ts parity smoke|triage|probe|verify|sweep`. Its smoke route intentionally gates only boot, even when the reported structural/pixel/behavioral status fails. Therefore smoke success cannot close this goal. Use explicit verify/journey evidence and inspect skipped semantic probes. Keep generated output under this ticket's generated directory and reuse current, completed activations.
# Current Follow-Up Evidence — 2026-09-20

Chrome geometry and the complete driver editor are actively assigned. The source-only scene audit is complete in `📓️astra-terra-scenes-current.md`: disabled grid snapping, omitted lighting/material inputs, and reference-image removal/replacement residency are confirmed execution work, not speculative old-wave findings.

Root rechecked the tutorial source after that audit. In addition to empty selection/tree capture, WGPU captures corner panel entries by PanelGroup while current React `captureTutorialUiSnapshot` uses all `ANCHORS`; WGPU snapshot apply only clears dialogs when absent and never restores a requested dialog; its command-panel apply only opens, never closes. WGPU gesture resolution explicitly returns `None` for Scene/Canvas/Entity/Curve/Domain. These require a bounded fresh tutorial bridge audit against current shared state, projection resolvers, and real React behavior before assigning an implementation. Evidence: Shell WGPU `tutorial_capture_ui_snapshot`, `tutorial_apply_ui_snapshot`, `tutorial_resolve_gesture_point`; React ShellHelpers `captureTutorialUiSnapshot` and `applyTutorialUiSnapshotToShell`. No runtime/tutorial pass is claimed.
# Fresh React Theme Observation — 2026-09-20

The activated React puzzle3d reference at `http://127.0.0.1:6313/?plugin=puzzle3d` was opened through CUA: Settings → Theme → Colors. The real editor uses expandable tree groups (Appearances, Metrics, Opacities, Radii, Strokes, Fonts, Spacing, Colors) and exposes Import, Export, Reset, Save as, Name and Theme controls. Opening Colors exposes its full color list; there are no page-selection controls in the observed DOM. The containing scroll viewport is 643 px high with `overflow-y:auto` and 1394 px content; the tree itself is 1392 px high. This establishes scrolling/expansion as the reference interaction and gives a concrete test against the WGPU editor's paging substitution. It is a React-only observation, not a paired WGPU acceptance result.

The reference console emitted one controlled/uncontrolled input warning while opening this editor (`value` and `defaultValue` both supplied). A first AX click timed out; fresh DOM inspection and the actual button role succeeded. Browser tab 3 remains at the expanded Colors group for subsequent comparison; no theme value was edited.
# Hub Renderer Boundary — 2026-09-20

The chrome geometry packet correctly leaves hub actions absent instead of inventing a sign-in affordance with no target, but this remains a feature gap. React's `ShellSync::hubConnectionSummaryV1` folds every attached document with priority signedOut→live→connecting→backoff/reconnecting→offline and uses the maximum live peer count (not a sum). `ShellHost` derives session presence from `verifiedSessionAuthority`, always mounts the workspace, and `openHubWorkspace` opens it in playground mode as well as host mode; host mode also navigates `/hub`. The footer exposes `framework.hub.signIn` only for signed-out state and supplies its real opener. WGPU's new projection covers only the current native document and has no equivalent browser transport/session publication or workspace opener. The next host audit must identify the existing canonical session/directory/workspace transport to reuse and trace the renderer boundary; do not replace the real session authority with a UI-only signed-in flag. Relevant React source: `🧱️elements/🔄️ShellSync/🟦️.tsx:176–258`, `🧱️elements/🏛️ShellHost/🟦️.tsx:9024–9037,10907–10918`. Runtime authentication has not been tested.
