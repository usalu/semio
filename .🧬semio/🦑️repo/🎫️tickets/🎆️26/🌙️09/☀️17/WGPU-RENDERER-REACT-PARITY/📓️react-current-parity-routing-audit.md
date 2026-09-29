# React and WGPU Current Routing Audit

Read-only source inspection on 2026-09-29. No runtime test was run by this audit and no remaining defect is claimed without paired acceptance evidence. Repository instructions read: root AGENTS.md, UI AGENTS.md, products AGENTS.md, OS AGENTS.md. Paths below are relative to the repository.

## Current Renderer Map

- React UI public target: `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx`. Window shell component: `🧱️elements/🪟️Window/🟦️.tsx:267` explicitly notes that OS dock supplies its own focus/topology controls. `:273–290` conditionally mounts open/focus/close controls; window measures resize is at `:337`.
- React Panel: UI `🧱️elements/🖼️Panel/🟦️.tsx:461–600` derives anchor flow, bottom panel reversal, directional tree context, tab caps and body width. Its native drag/pointer helpers are at `:93–219`; resize handles must not cover tab caps (`:371`). Compare actual hit rectangles, not only visual labels.
- React OS retained interpreter: `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:2740–2787` routes container, text, button, separator, input, select, toggle, key/value, slider, stepper, ring, icon selector, progress, tree, image, surface, extension and table. `:961–1014` reads all six LayoutSpec variants (leaf, stack, grid, overlay, scroll, absolute). Surface kinds at `:378–405` include canvas/world/node graph/text/table/paint/map/board/icon/ink/timeline/block list/diff/event feed.
- WGPU retained UI target: UI `🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs`, `🧮️layout/🦀️.rs`, `📌️mounted_layout/🦀️.rs`, `📥️input/🦀️.rs`, `⚡️events/🦀️.rs`, `🎬️action/🦀️.rs`. OS interpreter counterpart is the Interpreter sibling `🎯️targets/🧊️wgpu/🦀️.rs`. Its structural/frame dump starts at `:4371`; behavioral control/action/surface hooks start at `:5674`.
- WGPU shell counterpart is OS engine `🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`. Dock drop geometry, tree drag maps, persistence, window content rectangles and silhouettes already exist; absence of separate React-named component files is not evidence of missing capability.

## Old Findings That Are Already Repaired

The earlier `📓️astra-next-slices.md` does not describe current source for these areas:

- Driver seven-axis editor, save label, save and delete exist at WGPU Shell `:8874–8931`; save-label and save dispatch exist at `:11626–11641`.
- Tutorial snapshot reads live interaction selections and expanded trees at `:24691–24693`; gesture point resolver handles Scene, Canvas, Entity, Curve and Domain at `:24756–24814`.
- Verified directory session authority and hub workspace state exist at `:4077`, `:4146`; footer session derives from verified authority at `:10713`; workspace actions are handled at `:12532` onward.
- Current parity smoke gates the complete verdict at `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:210`, including structural/pixel/behavioral result. Old claim that smoke gates boot only is stale.
- Dock physical resize was repaired and neutral-oracle tested in `📓️astra-sol-dock-split-resize.md`; inspect runtime after rebuilding rather than implementing a second correction.

## Paired Acceptance Priorities

1. Rebuild and run current parity verify/journey against the same artifact/session, theme, locale, dimensions and DPR. Persist report with structural, pixel and behavioral verdicts and inspect skipped cases. `execution/🟦️.ts:291–302` provides the strict verify gate.
2. Dock: drag a divider by 120 CSS px and compare adjacent body rectangle changes; repeat nested weighted axis and minimum-bound clamps. Verify window-template insertion, all tab corners, tab ordering, close/focus and pointer ownership over resize handles. Use prior neutral `[50,50] -> [62,38]` fixture and real React function as oracle.
3. Panels: open each anchor, inspect body reservation and navbar collision at narrow width; bottom-flow content and tabs must invert together; drag and resize must not intercept tab presses. Hit-test a neighboring tab near the resize strip and compare emitted action identity.
4. Retained widgets: on shared documents compare exact control values, focus, keyboard commit/dismiss, disabled/read-only behavior, select popup scroll ownership, wheel routing and menu capture. Cover every component case listed above rather than inferring parity from serializer round trips.
5. Customization and tutorial: replay save/delete driver; expand/scroll full theme tree; capture/apply selection, expansion, dialog and command-panel state; resolve entity/window/scene points and compare actual ghost animation shape and timing. Existing production implementations require tests, not duplicate additions.
6. Window-instance ownership: two windows of one kind must preserve independent UI, focus, measures and action target. React Shell `:491–523` states this contract; WGPU Shell action scoping at `:417–427` is the corresponding implementation seam.

## Residual Scope Boundary Requiring Product Acceptance

WGPU Shell `:23993–23996` explicitly excludes introduction suppression used by React embedded multi-shell demonstrator hosts because its surface mounts one shell. If the user means parity for multi-shell embedding as well as ordinary renderer surfaces, this is an explicitly documented remaining scope difference. A concrete acceptance case is two embedded shells with introduction suppression enabled independently: no veil should cover the suppressed shell, and the other shell should still start its introduction. Ordinary single-shell parity should not be blocked by this unrelated host topology without establishing that scope.
