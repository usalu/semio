# Pressed Projection and Candidate Cancellation Audit

Read-only source review on 2026-09-26. This report records the current tree; it makes no completion claim and ran no tests. The reported `13/13` production mirror/React interaction result came from the implementation owner and was not independently rerun here.

## Current Mobile Panel Publication

The current implementation resolves the previously identified mobile duplicate correctly.

- The real mobile painter, `paint_mobile_tab_bar` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:25954-25964`, stages the actual visible hit key, `shell.panel.tab.mobile.{tabId}`, its painted label, `role=button`, and `pressed=is_active` through `note_chrome_group_item`.
- The desktop semantic catalogue is now expressly omitted for a mobile viewport in the same file at `:31500`. This matters because a visible desktop anchor can coexist in state while only the mobile panel is painted. It prevents the bare desktop `tab.id` catalogue node from duplicating the actual mobile node.
- The native regression `mobile_panel_buttons_publish_their_painted_names_and_pressed_state_once` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:736-773` uses width 390, an open desktop anchor, and the real mobile paint routine. For each painted tab it asserts one prefixed node, its label, `button` role, true or false `pressed`, absent `checked`/`selected`, and absent bare `tab.id`.

This key difference from React is intentional transport identity rather than a dangling DOM identity: React's `PanelTabButton` uses `id={tab.id}` and `aria-pressed={isActive}` at `🧰️framework/🔨️modules/🖱️ui/🧱️elements/🧭️PanelTabBar/🟦️.tsx:430-435`; WGPU's mobile hit router requires a prefixed ID to resolve the flattened panel. The regression verifies that each visible WGPU key is unique and carries the same user-visible name and state.

## Accepted-Frame Boundary

`CHROME_CONTROL_NAMES` is cleared at actual `FrameSetup` step zero in Shell WGPU `:24298-24319`. Names therefore become candidate data for the next walk. The accessibility projection is built only after all candidate authority is accepted in `acknowledge_presented_input` at `:14385-14416`: it first validates the witness, then swaps input authority, then calls `chrome_accessibility_nodes`.

`discard_presented_input_candidate` at `:14418-14427` clears the witness but deliberately does not publish chrome data. A late acknowledgment then fails `presented_input_candidate_matches` before it can read the candidate name map. That ordering is sound.

Existing coverage is complementary but split:

- `presentation_acceptance_and_abort_are_generation_bound` in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🧪️tests/🎯️presented-input-authority/🦀️.rs:306-344` proves candidate hit/owner authority: an aborted witness cannot be acknowledged later and the accepted owner/hit generation remains.
- `a_discarded_chrome_walk_cannot_exhaust_successor_accessible_names` in `.../🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:881-916` proves frame-setup clears a full name map before a successor walk and the successor can publish a label.

The selected native regression now joins those assertions in `a_discarded_chrome_walk_cannot_exhaust_successor_accessible_names` (`.../🎨️wgpu-theme-editor-and-accessibility/🦀️.rs:881-926`). It accepts A, snapshots the projection and generation, fills/stages/seals B, discards B, rejects B's late acknowledgement, checks A's projection/generation and hit remain, drives real `ShellChromeFramePhase::FrameSetup` setup zero, accepts C, and rejects B again without changing C. This is the correct level: the test calls the Shell acknowledgment entry point, which owns chrome projection promotion, rather than the interpreter directly.

The important transitions are B's discard followed by its delayed GPU ACK, then C's new walk. This audit did not run the regression.

## Frame-Gate Audit: Can a Later Walk Mutate B Before B Is Acknowledged?

No, not in the current production schedule. `render_chrome_step` and `seal_presented_input_candidate` run together in the frame build at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:16618-16629`. The witness then travels in the one frame presentation. The host invokes frame construction only as the producer passed to `AppPresenter::admit_next_frame` in `.../🪟️winit-app/🦀️.rs:265-277`.

`AppPresenter::admit_next_frame` refuses to invoke that producer when `has_pending_presentation()` is true (`.../🧊️renderer/🦀️.rs:15549-15580`). The pending predicate includes the pending cursor, retirement, and prepared-gate acknowledgement. Winit's immediate follow-on comment at `:279-283` states the consequence explicitly: while presentation proceeds, the next frame does not build. `FrameBuildHandle` also permits a single live session (`.../🧵️frame-job/🦀️.rs:125-140,278-280`).

The mutable names map is written only by chrome paint paths, and clearing it belongs to `FrameSetup`; neither runs during the pending presentation. The presenter merely progresses and validates the same candidate before rendering and acknowledgement (`.../🧊️renderer/🦀️.rs:15935-15995,16108-16137`). Therefore B cannot see a successor map, layout state, or chrome paint before its acknowledge or explicit discard. The guard owner is the presenter's admission gate, rather than the candidate structure.

This remains a future design constraint. If frame construction is ever pipelined while presentation is pending, capture the entire `BTreeMap<String, ChromeControlPresentation>` with the witness at seal, or form the chrome projection at seal, before relaxing `admit_next_frame`. A compact source/behavior law should assert that the producer closure is not invoked for a candidate-bearing pending presentation.

## Pressed-State Taxonomy and Ownership

`AccessibilityProjectionNode.pressed` now reaches the browser mirror's `aria-pressed` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/♿️accessibility-mirror/🟦️.ts:85-86`. The source does not apply a global `active -> pressed` rule: `note_chrome_group_item` limits it to `HitKind::PanelTab` in Shell WGPU `:31277-31286`. That narrow scope matches the React contracts below.

| Surface | React evidence | WGPU publication boundary | Required semantic state |
| --- | --- | --- | --- |
| Anchor and mobile panel tabs | `PanelTabBar` native button, `aria-pressed={isActive}` (`🧭️PanelTabBar/🟦️.tsx:430-435`) | PanelTab paint paths | `button`, `pressed` true/false |
| Pane chrome chip | `Tree` renders an ordinary native button without `aria-pressed` (`🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx:4691-4730`) | explicit pane-chip presentation | `button`, label only; no inferred state |
| Fullscreen | `NavbarFullscreenToggle` passes `pressed={isFullscreen}` to `<Toggle>` (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx:9572-9584`) | Shell fullscreen paint, exact key `ui.fullscreen.toggle` | `button`, localized current label, `pressed=isFullscreen` |
| Utility toggle | utility model has `pressed: Option<bool>` in `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/🧩️component/🦀️.rs:1566-1569` | actual utility hit, not generic painter placeholder | `button`, painted label, `pressed.unwrap_or(false)` |
| Utility button | ordinary action | actual utility hit | `button`, painted label, no `pressed` |
| Utility collection | expandable action group | actual utility hit | `button`, painted label; establish React's expanded contract before publishing `expanded`, never infer `pressed` from expansion |
| Retained contract toggle | actual Interpreter maps `checkbox` to `TreeCheckbox checked`; every other appearance to `<Toggle pressed>` (`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx:1328-1332`) | shared Rust/TS contract and WGPU live stamp | checkbox = `checkbox` + `checked`; Button/default = `button` + `pressed` |

The last row is an unresolved shared-contract defect, assigned to the Tree owner. Current contract projection treats all retained toggles as switch/checked (`🧰️framework/🔨️modules/🖱️ui/🧬️contract/♿️accessibility/🦀️.rs:75-91,265-270` and `.../🟦️.ts:73-95,201-207`), while the actual React Interpreter uses button/pressed for the default/Button branch. The WGPU live state stamp also writes `checked` for all toggle nodes (`🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/♿️accessibility/🦀️.rs:113-118`). Correct it at the shared schema/fixture level with paired Rust and TypeScript projection tests, then the WGPU projection test; it is separate from the dock/mobile slice.

## Utility and Fullscreen Implementation Packet

This packet was sent to the assigned utility/fullscreen owner.

`render_utility_node_step` at Shell WGPU `:18403-18459` paints placeholder IDs `framework.utility.button`, `framework.utility.toggle`, and `framework.utility.collection`, but registers different actual keys: `format!("{WINDOW_UTILITY_RAIL_PARENT}button.{id}")`, `...toggle.{id}`, and `...collection.{id}`. A label/pressed presentation written by the generic painter cannot match that hit. Stage the final presentation after each painter completes under the exact registered key. Use `utility_node_label(label, text, title, id)` as the canonical painted label. For a toggle, write role `button` and the model's `pressed.unwrap_or(false)`; for button/collection write role `button` and no pressed state.

Add schema-first fullscreen and utility records to `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/♿️wgpu-accessibility-interaction/🔣️.json` and its `🧬️schema` sibling. Verify the browser mirror emits `aria-pressed="false"` as well as true; the current fixture contains true pressed nodes but a false node is necessary to catch omission of an explicit false state. The independent React oracle in `🧪️tests/♿️wgpu-accessibility-interaction/🟦️.tsx:194-218` already establishes tab true/false behavior and is the appropriate pattern.

## Validation Status

No command, browser journey, native compilation, or test was run during this audit. The executable validation slices are the native presented-input cancellation extension, the native mobile-paint regression already present, paired Rust/TS shared projection tests for retained toggles, and browser mirror assertions for both pressed values.
