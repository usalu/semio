# Browser 21 Acceptance Checklist

## Evidence boundary

This is a read-only census for the next paired browser run. It uses the fresh partial browser evidence in [Checkpoint 20](./📓️astra-checkpoint20-browser.md), the current Native 139 targeted receipt at `🗑️generated/astra-runtime/renderer-native139-nine-and-measures/run.log`, and the existing physical-probe contracts. It does **not** treat an old checkpoint, a source-only audit, a neutral fixture, or a successful locator call as runtime acceptance.

Checkpoint 20 establishes a useful baseline: fresh paired Puzzle boot, physical Display footer, Settings open/General/close, later physical close/reopen, and 47.274 seconds of post-close console silence. It does not establish the full 57-step journey, Dock8, accessibility activation, or surface-family parity. Native 139 is also an incomplete gate: its selected batch ran 11 tests, with 4 passing and 7 failing. Its in-flight Window Options, Map/Canvas, engine provenance, Find, and Display tests must be rerun after their owners complete; their current failures are not a final product census.

The current Window Options compact Tree/checkbox work, Settings overflow/accessibility work, and Map/Canvas retirement work remain their owners' active scopes. They are prerequisites and revalidation rows below, not newly assigned root causes.

## Highest-priority browser 21 gates

| Priority | Existing scenario | Why it is required now | Acceptance evidence |
| --- | --- | --- | --- |
| P0 | The shared 57-step Puzzle journey in `🐍️parity-interact-probe.mjs:694-786` | Checkpoint 20 was deliberately partial. The same fresh pair must exercise chrome panels, App/General/Appearance/Language/Drivers Settings, pane unfold/fold, Actions scrolling, gutter drag, focus/unfocus/refocus, close-all/reopen, five World gestures, palette, shortcuts, example switch, and role switch. | Both React and WGPU complete every named step with its prescribed state observation and screenshot; no new console worker/presentation error; each click has its required visible or journalled consequence. Do not count a locator's successful dispatch as a state change. |
| P0 | The exact eight-case Dock adapter in `🔬️dock-interactions/📜️script.ts` | The old WGPU Dock8 receipts stopped at the boot census, so they neither prove a current defect nor establish repaired behavior. The adapter already covers the user-visible dock contract. | Fresh paired receipts for split-left, split-right, split-top, split-bottom, tab-merge, tab-reorder, Escape cancellation, and template-configuration. Preserve existing identities, geometry/topology, and the template's orthographic camera as specified in [the Dock contract](./📓️astra-sol-dock-interactions.md). |
| P0 | Popup-close then immediate window-cap close, followed by final-close/reopen | Checkpoint 20 observed the first Top-close immediately after popup dismissal fail, while a later physical close succeeded. This is a fresh, user-visible counterexample and must not be hidden by a delayed retry. | A new narrow sequence: open popup, dismiss it, immediately close the intended cap once, then assert the exact window/tab/body disappears, focus changes as expected, and no stale candidate or console fault appears. Keep the already-observed final-close empty-dock/reopen check as a second sequence. |
| P1 | Semantic accessibility activation, including Settings and Chat | In checkpoint 20, WGPU mirror-switch `getByRole(...).click()` calls returned but changed no state; physical clicks did. General labels were present, but that does not prove activation. Earlier Chat textbox concerns have no fresh acceptance receipt. | For each semantic control exercised by the 57 journey, perform keyboard/AX activation and assert the same state mutation as physical input. Include Settings tab selection and a named, enabled, editable Chat textbox that accepts text. Run only after the current AX/overflow repair, and retain the physical counterpart in the receipt. |
| P1 | Actions/Search scrolled-row and Select-popup escape | This is not yet a browser observation, but a high-confidence source risk: paint and host-hit positions subtract scroll offsets while `EventRouter::hit_test_node` does not; Select overlay children can be painted/published outside their scroll viewport but rejected by the router. See [the Tree audit](./📓️terra-tree-final-audit.md). | Add two physical cases before accepting panes: wheel to a terminal Actions/Search row and activate the visibly revealed row; open a Select whose option extends outside a scroll root and commit that option. The observed action must identify the visible row/option, not its pre-scroll predecessor. |
| P1 | Paired World3d scene probe, already embedded in the 57 journey | The World probe is the only existing product physical scene adapter: orbit, middle-button pan, wheel zoom, instance pick, and context menu. Checkpoint 20 did not accept all five after the current compositor/lifetime changes. | Fresh paired five-gesture receipts, including camera/action deltas and screenshots after the settle window. Keep visual comparison scoped to the same component state; this is required before using the Puzzle journey as evidence for non-World surface kinds. |

## Surface-family closure after the core journey

The UI contract still has 15 `SurfaceKind` variants at `🧰️framework/🔨️modules/🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81-129`. Native generic routing or a mounted-kind assertion does not accept a product surface. The following table identifies the shortest honest browser route; it is based on [the surface acceptance inventory](./📓️terra-surface-runtime-acceptance-plan.md).

| Family status | Families | Browser 21 / next-fixture requirement |
| --- | --- | --- |
| Existing physical route, fresh pair still required | World3d, InkCanvas | Run the five Puzzle World gestures. Re-run Note's existing clipboard, editor-precedence, trusted-cancel, and next-commit adapter on both renderers. |
| Existing specimen but no sufficient physical action | Canvas2d, NodeGraph, TextEditor, Table, Paint2d, VirtualFileSystem, TiledMap, Board2d, IconRender, GraphTimeline, BlockList | Draw must execute calibrated creation and Escape, rather than discovery only; Layout needs preview/one-create/cancel. Add app-visible operations for Flow graph, Writer/Imperative text and table, Raster stroke/cancel, GIS map, Puzzle2d board, VCS timeline, Forms block list, and Shooting icon pixels. Space VFS may currently accept only root rendering/focus because its roster exposes no actionable child rows; that limitation must be explicit in its receipt. |
| No product specimen | DiffView, EventFeed | Add a registered seeded app/fixture first. Renderer unit coverage cannot be represented as physical product parity. |

This makes the next completion sequence concrete: Browser 21 should execute the P0/P1 rows against Puzzle, Note, and the active Window Options/Settings repairs; subsequent focused activation pairs should cover the named application specimens. A single Puzzle run cannot close the 15-family gate.

## Revalidation rows for active work

- **Window Options:** the compact right-anchored Tree, checkbox treatment, German labels, and authored values need one paired screenshot plus semantic and physical control checks. Checkpoint 20's flat, full-width WGPU presentation and `Sun=1` / `Spacing AX=10.5` are historical counterexamples, not a pass.
- **Settings:** verify all declared six leaves can be reached visually, by keyboard, and by AX without horizontal truncation; switching locale must change visible labels. The current three-tab build is explicitly unaccepted.
- **Map/Canvas and retained replacement:** Native 139's Map and Canvas generation tests must be green before their physical adapters are credited. Browser coverage must then establish that a replaced/removed target cannot receive a stale terminal action.
- **Find/palette:** Native 139's physical Find-row test currently reaches `action program missing`; run its repaired native law and include palette mouse and keyboard activation from the shared journey.

## What not to carry forward as current failures

Checkpoint 18's Dock8 boot stall and older frame/GLB timings are historical diagnostics, not Browser 21 verdicts. They warrant normal console/frame monitoring in the fresh receipts, but do not justify a claim that today’s implementation still has those failures. Conversely, the partial checkpoint 20 successes do not justify marking any unrun row above green.

## Completion rule

Do not call the parity gate complete until all of the following are present:

1. Fresh paired 57-step and Dock8 receipts, with screenshots and error-free console evidence.
2. A direct regression receipt for immediate post-popup close and for AX/keyboard activation producing the same state as a physical gesture.
3. The two scroll/overlay interaction laws above, then their physical browser receipts.
4. A per-family receipt or an explicit seeded-product fixture for every SurfaceKind; no DiffView/EventFeed substitution with renderer-only tests.
5. Active Native 139 ownership/lifetime tests rerun to terminal status before their corresponding browser receipts are accepted.

**Validation:** static review only; no browser, build, or test command was executed for this checklist.

