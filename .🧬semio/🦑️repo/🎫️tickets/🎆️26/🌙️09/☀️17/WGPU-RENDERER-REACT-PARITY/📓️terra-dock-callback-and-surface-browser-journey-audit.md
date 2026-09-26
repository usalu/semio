# Dock Callback and Surface Browser Journey Audit

This is a read-only source audit on 2026-09-26. I did not start a browser, dev server, Cargo, or Nx command, so none of the journeys below is a passing result.

## Dock callback and accepted-frame review

The current WGPU path is coherent for the Dock case.

- `DockControlName` is defined in `🧱️elements/🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:120-127`. Its callback is fired while the tab/action hit is registered at `:1739-1754`.
- `ShellState::plan_dock_windows` records each mounted body title, then `render_main_window_step` supplies the callback to the dock chrome walk (`🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:25091-25101,25324-25339`).
- `note_dock_control_name` supplies the actual localized Focus, Unfocus, Close, and interpolated Drag strings and records `tabpanel` for bodies and `tab` plus `selected` and `controls` for tab selectors (`:31158-31179`). It uses the existing `shell_chrome_string` authority; it does not add another dictionary.
- The map is cleared with the input/owner staging maps at `FrameSetup[0]` (`:24199-24207`). On successful GPU acknowledgement, `acknowledge_presented_input` first checks the witness, swaps the accepted hit authority, snapshots accessibility nodes, and only then publishes them (`:14265-14318`). Thus an unaccepted candidate cannot mutate the already published accessibility vector.
- The browser mirror makes a generic `div[role=tabpanel]` for a panel, writes `aria-selected`, and only writes `aria-controls` if the control target exists in that accepted projection (`🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:50-90,147-156`).

React's `ui.window.*` resource and WGPU's existing `common.*` resource are different keys but resolve to identical outward values: `Focus` / `Fokussieren`, `Unfocus` / `Fokus aufheben`, and `Close` / `Schließen` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2898-2901,3819-3822`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:29488-29491,29590-29593`). Both hold the same `tree.drag.sortTarget` template, `Click and hold left click to drag {{target}}` / `Linksklick gedrückt halten, um {{target}} zu ziehen` (`🖱️ui/🎯️targets/⚛️react/🟦️.tsx:2508-2514,3429-3434`). The callback substitutes exactly the literal `{{target}}` token. Therefore the current output parity is correct without a duplicate WGPU dictionary or an unnecessary translation-key bridge.

### Stack relationship correction

The first review incorrectly proposed that each inactive tab should control a per-window panel. React disproves that: every stack tab has `aria-controls={modeDockPanelDomId(stackPath)}`, while exactly one stack `tabpanel` is mounted and is labelled by its active tab (`🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx:1047-1052,1203-1214`). The WGPU callback's `panel: active` is therefore intentional and necessary; an inactive tab pointing to its own inactive window would be a dangling relationship and the mirror would omit it.

The current law now constructs a two-window stacked Dock and checks exactly this invariant: both tab nodes control the active body, only the active tab is selected, only the active body is a non-actionable `tabpanel`, and the inactive body is absent (`🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:763-835`). The shared neutral fixture records `stackedActiveId: "pane-top"` at `🖱️ui/🧫️fixtures/🪟️dock-accessible-names/🔣️.json`.

### Remaining verification boundary

No source-level compile defect was found in the callback, role/selection/controls projection, or the native `WorkerCell` map. The individual Dock action hits are `HitKind::Button`, so their unoverridden fallback role is correctly `button`; the current Dock law asserts their names but does not separately pin that fallback role (`🛰️Dock/🎯️targets/🧊️wgpu/🦀️.rs:1748-1762`; `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:31225-31235`).

There are two adjacent, but separate, lifecycle laws. `wgpu-frame-job-unit::superseded_frame_build_returns_its_exact_presented_input_candidate` seals a candidate, closes its `ActiveFrameBuild`, and proves a fresh successor can seal and acknowledge (`🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs:135-202`). The new Shell capacity law fills the name map, executes `FrameSetup`, and proves a successor label can publish (`🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:840-875`). Neither law carries a populated chrome-name map through that renderer cancellation boundary. The map itself is staging-global rather than stored inside `PresentedInputCandidateWitness`; correctness therefore relies on the renderer's one-witness progression checks plus the later frame setup clear.

Before treating the **label** portion of the asynchronous property as runtime-proven, add or run one bridge law that populates candidate A's chrome labels and hits, drives its owning renderer presentation through cancellation or supersession, then runs B's real `FrameSetup` and acknowledgement. It must prove A cannot publish stale labels and B has fresh labels without inheriting A's capacity. The individual witness-retirement and map-clear laws are valuable, but neither exercises their junction.

## Minimal ordered browser acceptance checklist

For every available row, first activate the exact paired target defined by `🧑‍💻dev/♻️activation/🏃️execution/🟦️.ts:148-159`:

```text
@semio-tech/framework-os-dev:activate-<variant>-react-dev
@semio-tech/framework-os-dev:activate-<variant>-wgpu-dev
```

Then use the registered playground fixture shown below, wait for an accepted frame, assert the expected surface identifier in both renderer introspections, perform the listed user-visible action in both, and compare the named receipt or visible postcondition. Capture a screenshot and the before/after introspection with each result. A boot failure, a missing surface id, or a non-changing claimed receipt is a failure or blocked result, not coverage.

| Order | Contract kind | Registered variant, app/producer, fixture | Expected source surface | User action and required observable |
|---:|---|---|---|---|
| 1 | Canvas2d | `draw`; `s.draw.drawing@1/*#editor`; `🎬️demo` | `drawing.play.composite` | Make one calibrated freehand stroke, then press Escape during a second stroke. The first produces one visible/document consequence; the second has no committed consequence. |
| 2 | World3d | `puzzle3d`; `s.puzzle.puzzle3d@1/*#editor`; `🌲️concrete-forest` | Puzzle3d's accepted World3d main scene | Orbit, pan, wheel zoom, pick one instance, and open its context menu. The existing World3d probe is appropriate only for this row. |
| 3 | NodeGraph | `flow`; producer app `flow-play`; `🎬️demo` | `flow.play.main` | Drag a visible node or complete one connection attempt; the graph geometry or connection state must change. |
| 4 | TextEditor | `imperative`; `s.imperative.procedure@1/*#editor`; `🎬️demo` | `imperative.play.script` | Focus the script, insert a known sentinel, select/copy it, and Escape. The text projection and selection/cancel state must agree. |
| 5 | Table | `imperative`; `s.imperative.procedure@1/*#editor`; `🎬️demo` | `imperative.play.main` | Run the sample, then select the `log.print` row. The source fixture's result table contains `log.print` and `run-output`; selection must become observable. |
| 6 | Paint2d | `raster`; `s.raster.raster@1/*#editor`; `🎬️demo` | `raster.play.composite` | Make one stroke or tool gesture and cancel a second. Assert the raster/model consequence and cancellation. |
| 7 | VirtualFileSystem | `s`; producer surface fixture `🎬️demo` | `s.play.media-vfs` | Assert the root row mounts and accepts focus. Current source deliberately exposes only root through a no-op media-VFS flattening path, so recursive file-operation claims are out of scope. |
| 8 | TiledMap | `gis2d`; `s.gis.gismap@1/*#editor`; `🎬️demo` | `gis2d.play.composite` | Wait for a nonempty map/feature scene, then pan or wheel zoom and select/hover a feature. A prior report records a WGPU blank/hang for this exact specimen; classify a repeat as boot/paint blocked, not as passing map coverage. |
| 9 | Board2d | `puzzle2d`; `s.puzzle.puzzle2d@1/*#editor`; `🌲️concrete-forest` | `puzzle2d.play.composite.2d-overview` | Select or brush in Overview. A selection/domain receipt or visible board mutation must follow. |
| 10 | IconRender | `shooting`; `s.shooting.shooting@1/*#editor`; `🎬️demo` | `shooting.play.icon` | Assert a nonempty icon frame and ready/pending/error status after the fixture mounts. React has no richer pointer handler here, so this is intentionally visual/state acceptance. |
| 11 | InkCanvas | `note`; `s.note.note@1/*#editor`; `🎬️demo` | `note.play.composite` | Exercise the existing copy/paste path and trusted-pointer cancellation path; assert clipboard/document consequence and cancellation. |
| 12 | GraphTimeline | `vcs`; producer app `vcs-play`; `🎬️demo` | `vcs.play.history` | Select one history/checkpoint row and navigate one step by wheel or keyboard. The selected checkout/state must update. |
| 13 | BlockList | `forms`; route app `s.forms.forms@1/*#editor`, producer `forms-play`; `🎬️demo` | `forms.play.blueprint` | Select a named builder block and run one add/remove/move action. The visible block order/content must change. |
| 14 | DiffView | No registered producer fixture | — | Blocked. Do not count renderer-unit painting as a product/browser journey. Register one app fixture before adding an acceptance path. |
| 15 | EventFeed | No registered producer fixture | — | Blocked. Register one app fixture before browser acceptance. Its locale/timezone contract is separately unresolved and must not be masked by a synthetic UTC assertion. |

The registry itself verifies the named variants, paired ports, and fixtures in `🔌️plugin/📇️registry/🤖️generated/🎠️playgrounds.json`; individual surface ids are emitted by their producer files, including Draw, Flow, Imperative, Raster, Note, GIS, Puzzle2d, Shooting, VCS, Forms, and Space. The contract's complete 15-kind authority is `🖱️ui/🧬️contract/🗺️surface/🦀️.rs:81-129`.

## Browser-suite boundary

The current parity runner launches paired Chromium pages and can compare boot, structure, pixels, and generic probe suites (`🧑‍💻dev/⚖️parity/🏃️execution/🟦️.ts:146-194`). Its existing interaction probe is specifically World3d-shaped. The ordered rows above are a test-design checklist for dedicated per-family adapters; they are not implied coverage from a green structure or screenshot result.
