# Web OS lane F handoff

Test first: `os 🧪️tests/🪟️chrome-frame-laws/🟦️.ts` (source scan: no shadow or radius utility in any OS element source, glass chips marked, diagram node framed, footer spacers, square presence labels, glass notice bands). It failed 14 cases before the edits and passes now.

Run: `lockf -k /private/tmp/claude-501/semio-vitest.lock node node_modules/vitest/vitest.mjs run --config <ticket>/web-os-vitest.config.ts` (new config: laws test, notice-bands, canvas-presence, World3dHost, TextEditor, DiffView, ink, Paint2d, saved-state). Result 204 of 205 pass plus 3 failing files, identical to the baseline run before my edits: DiffView "measures" (Chromium missing), World3dHost line-hit and grouped-hover (fixture needs Chromium/wasm). I added no failures.

## Verified audit rows (F-1..F-6)
- F-1 fixed: diagram node is a `SharedBorderRow` with header and body cells, no rounded/border/shadow.
- F-2 fixed: terrain label, attribution, scalar legend, mismatch, compute/surface status panes, annotation labels are `data-glass-chip` glass, no rounded/shadow.
- F-3 fixed: `footerLeadingSpacer`, `footerTrailingSpacer`.
- F-4 partly: diff divider is a `bg-border` column (no border class). Ink table `th/td border` kept: a drawn data table (content).
- F-5 fixed as square label (presence label radius removed, also the other two labels); cursor dots, view rect and marks kept (content).
- F-6 fixed: completion popup is a `SharedBorderRow` glass menu, no border/rounded/shadow; its trigger uses `ui-glass`.

## Sweep (whole OS React tree)
- Removed all `rounded`/`rounded-sm|md` and `shadow-sm|md|lg` utilities (24 files: AgentApprovals, AgentChatPanel, Interpreter, BlockList, EventFeed, ShellHost, NodeGraph, Paint2d, Ink, time-travel, ...). `rounded-full` dots and progress bars stay.
- Opaque chips on canvases now glass with `data-glass-chip`: NoticeStack and FloatingPane (notice-bands), directory bootstrap notice (now a NoticeStack), session notice, local-catalog files section, wire refusal, Paint2d status chips and tool panel, Ink editor overlay, presence overlay and DAG overlay, canvas context-menu trigger.
- Hand-drawn borders on chrome moved to the compositor: transient notice (frame ref), surface-role chip in the navbar (`SharedBorderRow as="span"`), notices.
- Paint tool panel cells lost their opaque `bg-background`.
- Plugins (`✏️s/🔌️plugins`): only one TSX/CSS hit, the CAD legend swatch `rounded-xs` (removed). audit2-plugins TS/CSS rows F6-F12 are in the shared ui module and F13 in course: not touched.

## Open (outside my files)
- Law 4 for the canvas: `Layout` (shared ui) keeps Navbar and Footer in flow (`flex-shrink-0`) and only runs content beneath them with `scrollCanvas`; the edgeless OS canvas therefore does not run beneath the glass bands yet. Needs a ui Layout change (canvas absolute inset-0 plus band-height clearance).
- `ContextMenuTriggerButton` (ui) forwards no `data-*`, so the OS triggers use `ui-glass` instead of `data-glass-chip`.
- Pre-existing opaque in-flow surfaces left: close-guard dialog and staging dialog (modal dialogs), TaskManager sticky `bg-background` cells, ink block card `border` (content).

## Follow-up (frameless surfaces, opaque leftovers, suite)

1. Frameless consumers of `floatingMenuSurfaceClass` and `floatingRibbonSurfaceClass`: now `data-glass-chip` on `TiledMapHost` popup, `catalogue-spotlight`, and both r3f projection switches (`♾️infinite/🌍️world/🎨️r3f/📐️projection`: kind switch ribbon and orbit switch). Grep of os, infinite and plugins shows no other consumer (`canvas-pick` is ui). Pinned in `chrome-frame-laws`.
2. Opaque leftovers: close guard is glass (`glassClass`, `data-glass-chip`), staging dialog glass (`bg-menu` gone), install notice and check-in band lost `bg-menu` (they sit in glass stacks). Kept opaque on purpose: TaskManager sticky `right-0` cells (`bg-background`; scrolling cells run beneath them and would show through glass); ink block card `bg-background/90` plus `border` (a content object on the canvas, readable over strokes, its border is the probe-art of the block).
3. Full OS renderer suite (`os-vitest.sh`, lock held): 46 failed, 3534 passed, 28 files (the 38 of recipes2 section 3 plus run noise). Result is not changed by my edits: no failing assertion mentions chips, frames, bands or palette.
   - Environment, Chromium missing (`browserType.launch: Executable doesn't exist ... chromium_headless_shell-1248`, `npx playwright install` needed), 22 launch errors in the log: chrome-palette (3), diff-view-produced-surface, retained-toggle-semantics, text-input-oracle, text-advances, retained-frame-progress, asset-cancellation, icon-svg-lighting, prepared-readback, EventFeedHost layout, GraphTimelineHost layout, Table button-accessibility, empty-dock-notice, time-travel geometry, IconRenderHost (gpu-export, png-export, download, frame-presentation), World3dHost line-hit and grouped-hover fixtures. chrome-palette and diff-view contain no chip/frame/gap assertion (grep), so they are pure environment.
   - Worker/wasm/boot: embedded-mount (9), package-integration (8), hidden-tab-surfaces and node-graph-hosts (suite fails to load), MediaTransportHost, turn-patch-batch (60 s timeout), shell-option-locks (`Bun is not defined`).
   - Behavioural, not mine and not about chrome: editable-controls FEM field-intent (4, intents never emitted), procedural-descriptors (editor action key lists three ids, test expects one), reprojection-status (load projection folds `[1,false,true]` not `[1,true,true]`, matches the in-flight document-load-gate work of another agent). I did not touch these.
