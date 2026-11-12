# Playback surfaces for teaching clips (playback owner)

Scope: every place a teaching clip (a Manim-rendered mp4, a chapter, a course preview) is played, scrubbed, thumbnailed or composed. Verified by reading source on 2026-11-11; browser pixel checks are blocked by FD-playback-1 (FUNDAMENTAL-FIXES.md).

## Findings

| Surface | Path | What it plays | Seek / scrub / reverse / pause / loop | Media choice | Poster |
| --- | --- | --- | --- | --- | --- |
| Presentation browser renderer | `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🌐️browser/🟨️render.mjs` (+ `🟦️.tsx`) | Not a player: an offline batch renderer. Real shared semio React/CSS (LayeredOverview, WindowChrome) driven by a Playwright paused clock, one PNG per requested state. | None interactive. Frames are composed by ffmpeg into the chapter mp4 (`presentation/🐍️.py::_render_chapter`). | Reads the already composed content clip given by the caller; the render settings carry the profile. | The first and last body frame are snapshots for the open/restore transitions. |
| Hierarchy/interaction/collection | `🪟️presentation/🥞️hierarchy`, `🖱️interaction`, `📚️collection` | Choreography is a pure function of the phase fraction `t` (`choreography`), plus chapter clock arithmetic. | `t` is the canonical timestamp; any `t` can be requested in any order. | n/a | n/a |
| Course site | `🎓️teaching/🏛️architecture/📚️course` and `🧰️framework/🛍️products/📚️course` | Quiz/assignment product. No `<video>`, no mp4, no poster references (grep over ts/tsx/html/json). | n/a | n/a | n/a |
| Generic React video elements | `🧰️framework/🛍️products/🎤️presentation/.../🔍️figure-zoom`, `🧭️arrangement-section`, `🎭️morph-views`; `🧰️framework/🔨️modules/🖱️ui/.../🎥️tutorial-ui` | Product demo media of other products (`scenes/<hash>/scene.mp4`, tutorial overlays). They do not consume teaching clip output. | Native `<video>` controls (not teaching related). | Path by scene hash, not by render profile. | `poster` attribute passthrough. |
| `🧰️interop/🎬️scene` | does not exist in this repository | n/a | n/a | n/a | n/a |
| Storybook | `🧰️framework/🔨️modules/🖱️ui/📖️stories/🎭️tutorial-video-overlay` | A logo animation, not teaching media. | n/a | n/a | n/a |
| Export side (exports owner) | `🎬️clip/📤️export` | Provenance (`.export.json`), posters and thumbnails from the canonical frame schedule. | n/a | `require_record_profile`. | `stills` in provenance. |

Conclusion: there is no interactive player of teaching clips in the repository today, and nothing evaluates morph state live in a browser. The only live-evaluation path is the paused-clock browser renderer, which is a deterministic offline function of declarative frame state. The player-facing work is therefore (1) the contract a future player must follow and the oracle that proves it, and (2) making the one surface that exists (the presentation renderer) a pure function of the canonical timestamp.

## Defects found and fixed in the presentation renderer

1. Batch dependence of frame state (coordinator note, verified):
   - `render.mjs` accumulated `animation.currentTime` across a batch (`animationTimes` WeakMap).
   - `hierarchy/🖌️render/🐍️.py` derived `transition_elapsed` from the previous request.
   - The mouse stayed pressed or hovering from earlier frames; the React tree kept earlier state (only canonical overview frames were remounted).
   - Fix: every frame resets (unmount, mouse release, pointer parked at 1610,910), mounts with `reset=true`, uses `transition_elapsed` from a constant or the request itself, and assigns the animation clock. The remaining real `requestAnimationFrame` use (`paintBarrier`) is a paint barrier inside a loop terminated by the `settled()` predicate, advanced by a fixed 34 ms of fake clock per attempt (max 60); it is not a clock source, so it cannot change which state is reached, only whether it is reached (a timeout raises, never a different image).
2. Mixed `round`/`ceil` frame counts: `presentation/🐍️.py` (transition `round`, body `ceil(frames * fps / source_fps)`), `collection/🐍️.py` (`round` twice, `ceil`). Now one rule, `presentation.frame_count`, which is `timeline.count(Window(0, duration), Schedule(fps))` with an explicit floor (3 for transitions: first, middle, last). The duration is rounded to nine decimals first to neutralise float noise.
3. Presentation `_settings` guessed `-ql` when no quality was given. It now requires `settings["profile"]` and takes size/fps from the profile registry (`🎚️profile`); explicit `resolution`/`fps` overrides remain.
4. Presentation cache identity (`source_files`) now includes the timeline module, because frame counts depend on it.

## Evidence

- `🪟️presentation/🧪️tests/⏱️purity/🐍️.py`: frame counts against the NumPy `arange` oracle (JSON cases), body clock cases, chapter clock uses one rule, and the browser packet of a frame is identical alone, in a batch, and in a reversed batch (forward/focus/restore phases). 6 tests, 1 skipped (browser pixels, needs `SEMIO_TEST_BROWSER=1`).
- `🎬️clip/🎞️playback/🧪️tests/🐍️.py`: PyAV decodes a lossless movie whose frame colour is the shared evaluator's owner. At f1-1, f1, f1+1 (and f0, first, last) forward, reverse (seek per frame descending), seeded-random order and a fresh seek give bit-identical frames; identity equals `timeline.owner`; skipped delivery and pause/resume land on the target. Cases: 15/30/25/60 fps, ten-frame and one-frame transitions, keyframe interval 5 so seeks really cross GOPs. Profile selection: only the matching profile opens, other profile/modified file/missing provenance/unknown profile fail, badge labels en/de, no badge language default.
- Incidental finding: `numpy.arange(0, 10/24, 1/24)` has 11 elements (float noise), so a ten-frame transition at 24 fps is eleven frames in Manim and in `timeline.count`. Authoring should declare durations as frame counts divided by an exact fps or accept the extra frame; the timeline owner may want a noise tolerance (then it would differ from Manim, which is the contract's reference).

## Open

- FD-playback-1 blocks the browser-pixel purity test and the existing browser tests; no browser result is claimed.
- No interactive player exists to exercise pause/resume/loop in a browser. When one is added it must follow the `INTERFACES.md` playback section; `🎞️playback` already provides the decode oracle.
