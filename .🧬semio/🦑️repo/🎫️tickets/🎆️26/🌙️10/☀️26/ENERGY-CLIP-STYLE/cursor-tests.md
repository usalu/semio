# Cursor Interaction Verification

The cursor feature is covered by **60 distinct test methods across the latest component runs: 54 passed and six documented runtime skips**. The full 18-method cursor run passed in **12.109 seconds**; the additional fullscreen SVG viewport regression and targeted final native checks pass after the final fidelity correction.

| Component | Result | Elapsed |
| --- | --- | --- |
| Cursor model, intervals, native states and authored Scene helper | 19 distinct methods passed | 12.109 s full 18-run; final helper 3-run 2.573 s and endpoint 6.870 s |
| Presentation contract and publication guards | 8 passed | 3.421 s |
| Five native encoded lesson movies and overview images | 2 passed | 39.726 s |
| Existing clip runtime regression | 25 passed, 6 skipped | 10.248 s |

These are component runs, not one combined command. The runtime skips are five explicitly gated real-Manim checks and one unavailable Windows symbolic-link privilege check. The new native cursor and movie tests have no skips. No production implementation or AGENTS.md was edited by this test agent.

## Portable Contract and Independent Oracles

The new fixtures are `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🔣️.cases.json`, with the Python adapter alongside them.

The fixtures specify half-open target bounds, complete navbar/footer/window/tab/fullscreen/tree ancestor paths, disabled ancestors, stacking order, transparent WindowSilhouette cap gaps, gesture traces and state effects. Pillow independently rasterizes target masks and intersects all ancestor masks. It agrees with the production hit path for every case.

Press and release must occur on the same enabled target before a click can commit. Dragging out, releasing over another control, clicking without a press, or disabling an armed control cannot activate fullscreen or a toggle. Reducer outputs preserve earlier snapshots and the input tree. Fullscreen and toggle state changes occur only after a committed click, and zoom cannot start without that click.

The canonical overview park is inside the application/canvas. Following the final interaction decision, both shared overview endpoints have the same actual hovered path `[app, canvas]`; they do not suppress real parent hover. The separate idle park is outside the application and has no hover.

NumPy independently reproduces the canonical 80% primary / 20% light sRGB mix, yielding `#fd5a6d`. A separate NumPy polynomial-root oracle inverts CSS `cubic-bezier(.25,.1,.25,1)` and checks the 120 ms border transition at clamped endpoints and interior samples. Invalid color strings and nonfinite elapsed times are rejected.

Native Manim SVG parsing independently verifies both transformed polygon paths and the fill/stroke colors of the real default, pointer and selectable cursor assets. The source closure test requires all cursor assets, fullscreen icons, schemas and interaction code to affect presentation cache identity.

## Native UI and Authored Click Behavior

The native snapshot tests verify actual rendered control fill pixels for hover and press, real ancestor styles, and the ancestor window's border pixels. They prove the click commits before the window rectangle moves, and compare the focus-start and restore-end overview RGB pixels exactly.

Direct pointer positions exercise navbar and footer controls. A real navigation tree menu verifies that descendant hover emphasizes its owner rows and guide stroke while unrelated sibling rows retain their normal styles. Hover samples wait beyond the 120 ms border transition; direct pointer overrides represent settled canonical states.

The authored Scene helper test uses real Manim targets and a Scene spy with an advancing virtual clock. It verifies:

- Press → release → click → activation callback order.
- Release and committed click have the same event time.
- Exactly one callback for an enabled control.
- No activation for disabled controls.
- Transparent normal frames and hollow fullscreen SVG paths remain unfilled.
- Hover and active controls retain their canonical paint.
- Disabled opacity preserves the original relative fill/stroke alpha, including repeated disabling and restoration without cumulative fading.

The final native control opacity test caught a real `VGroup.set_opacity()` issue that had filled transparent frames and open SVG paths. Both renderer and authored helper corrections now pass.

## Cursor Activity Through Video Composition

Portable interval cases specify sorting, overlap/adjacent merging, half-open excerpt intersection, rebasing, invalid ranges and concatenation offsets. NumPy independently checks interval clipping.

The tests call the real `concat_videos` and `cut_clip` entry points over the abstract command interface, then verify their cursor sidecars. They also prove that all-empty activity does not probe media duration and that activity extending beyond its source duration is rejected.

A new publication case mutates cursor activity during candidate rendering. The prior certified movie and certificate must survive the rejection, just as they do for changed presentation sources, stale content state and changed nested authored hashes.

Cached chapter fixtures now explicitly publish empty cursor metadata and use the current audience-specific media path and source activity hash fields. The whole-chapter reuse regression still proves that cached chapters do not require retained raw segment movies.

## Five Native Movie Regression

The existing native regression generates canonical H.264/AAC MP4 content with eight scientific surrogate frames and a 48 kHz 440 Hz narration surrogate. Each of the five lessons is wrapped independently at 320×180 and four frames per second.

All five encoded movies pass:

- Exactly 24 video frames and six seconds of decoded audio.
- Two seconds of silent transition padding on each side.
- Retained tone and amplitude.
- Every original source frame preserved in order in the focused body.
- Identical first/last overview RGB pixels within each movie and across all five lessons.

The first source additionally carries cursor activity `[0.5, 1.5]`. It exercises the finite hovered-shell PNG/AAC branch and proves that ancestor pixels change outside the scientific viewport. The remaining four movies cover the empty-activity branch.

The final opacity correction changes only native object paint; the final 18-method cursor run rechecks native appearance and exact shared endpoint pixels after that correction. The already passing five-movie media regression was not repeated because audio/frame composition was unchanged.

Generated movies and phase PNGs are under `🗑️generated/cursor-tests/presentation/native-evidence`. Settled native hover/press/click, navbar/footer and tree snapshots are under `🗑️generated/cursor-tests/native-frames`. The independent visual review agent received the samples. The root agent owns the final authored equipment/tour preview.

## Baselines and Defects Caught

Before implementation, five core tests failed missing the module in 0.030 s; the Scene helper test failed missing its module in 3.034 s; and two interval tests failed missing runtime APIs in 0.296 s. Two native snapshot tests also failed before the renderer snapshot API existed.

A later regression caught zoom progressing for an outside-app target without any committed click. Both focus and restore now stay at zero zoom in that case.

The asset closure regression initially showed eight missing interaction inputs. During broader verification, faulthandler located a recursive `segment_state` asset walk entering the canonical styling directory's `node_modules` junction. Restricting recursive discovery to Python source owners fixed the stall while preserving direct JSON/SVG fingerprints. A behavioral regression now proves that unrelated sibling SVG changes do not invalidate a segment, while an explicit cursor SVG change does.

The opacity baseline failed nine native object subcases. The advancing-clock helper baseline separately showed release at 0.03 s and click at 0.04 s. The corrected helper commits click/callback in the same turn and treats release dwell as post-activation time.

## Fullscreen SVG Viewport Fidelity Follow-up

The independent SVG geometry check confirmed that a 20 px native width had been applied to the visible 18×18 path bounds rather than the canonical 24×24 SVG viewport. Both icons therefore appeared 33⅓% too large. Portable fixtures now require a 20×20 px viewport and centered 15×15 px visible ink.

The regression compares the source viewBox, independent svgelements bounds, every oriented path vertex, native geometry and relative SVG stroke width. It also caught an initial coordinate mismatch that produced a 20×37.5 px group. Correcting the native Y translation preserves the canonical northeast/southwest orientation. Visible stroke width is 2×20/24 = 1⅔ units; the invisible viewport frame stays unpainted.

The final three helper checks pass in **2.573 s**: viewport geometry/orientation/stroke, transparent/hollow paint and relative disabled alpha, and synchronous release/click/callback semantics. The targeted native hover/press and exact shared endpoint check passes in **6.870 s** after the correction. There are **19 distinct cursor test methods** in the final source. The earlier full 18-method run plus these targeted checks covers the final implementation; no five-movie re-encode was necessary because audio, scientific content, hit regions and layout were unchanged.

## Owned Files and Cleanup

Created:

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🔣️.cases.json`
- Ticket input `run-cursor-tests.py`
- Ticket report `cursor-tests.md`

Updated:

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts` interaction test registration.

No owned source file was removed. Existing explicit-PID inspection/stop input scripts were reused only to stop this agent's inspected stalled Python test descendants. No other agents' processes were touched.

All owned test/diagnostic processes have exited and all runner file handles/timers are closed. Generated output remains for root review and required ticket cleanup. The permanent runner and Markdown audit must be retained.
