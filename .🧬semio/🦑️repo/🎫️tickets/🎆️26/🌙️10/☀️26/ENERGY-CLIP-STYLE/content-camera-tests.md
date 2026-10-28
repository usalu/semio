# Content Camera Tests

The portable presentation fixture now specifies a 0.6 second camera movement after the focus navigation and before the restore navigation. It requires the complete source image across the body output and continuous source geometry at both handoffs. Native focus viewport geometry remains the measured UI window body.

The previous five lesson encode loop is reduced to one representative lesson. Its independent Pillow, NumPy and PyAV oracle compares every full decoded body frame, all four output edges, source frame order, focus/body and body/restore seams, exact first/last overview pixels, tone frequency and silent transition padding. A second fast silent-source test isolates the real media compositor using two synthetic transition frames, checking full-frame output and finite silent audio.

## Red Evidence

- The new portable timing check failed with `KeyError: contentCamera` before the production contract changed.
- The silent source media test failed against the previous shell compositor: full-frame mean RGB error was 49.869 against a maximum of 10. This verifies the oracle detects the incorrect inset body independently of the production geometry calculation.

The retained `run-content-camera-tests.py` runner owns temporary media under `🗑️generated/content-camera-tests`. It runs through `repo capacity run` using the existing teaching Python environment.

## Green Results

- All nine presentation contract tests passed, including explicit language, unchanged native UI geometry, source identities and publication safety.
- The native browser camera test passed eight portable samples across focus and restore. It verifies independent source pixels fill the complete frame at the focus endpoint and restore start, the measured source rectangle maps to the complete canonical viewport, intermediate frames apply a real camera transform, the virtual pointer stays hidden during movement, restore pointer down/up/click events occur after the camera returns, and the restore control remains fused to the right cap.
- One representative production lesson encoded successfully: 14 focus frames, eight authored frames and 14 restore frames at four fps. Pillow, NumPy and PyAV verified every full-frame body image and all four edges, original frame order, both camera handoffs, exact first/last root pixels, 440 Hz narration and silence throughout both transition pads. This browser and encoded run passed two tests in 244.348 seconds. The five lesson encode loop was intentionally removed.
- The no-audio source test passed with the actual compositor, full-frame source order and finite silent audio. Its transition images are synthetic so that it isolates media behavior without repeating browser rendering.

## Frame Rate Conversion Regression

An additional portable fixture specifies an eight-frame, four fps source converted to two fps. Pillow writes independently distinguishable input frames, PyAV decodes both movies, and NumPy identifies the selected source sequence `[0, 2, 4, 6]`. Synthetic transition images use the actual snapshots supplied by production, so the same oracle checks both camera/body handoffs without browser overhead.

This test exposed a second production defect before its fix: the final body frame selected source frame six, while the restore snapshot used raw source frame seven. Their mean RGB discontinuity was 18.6926, exceeding the allowed two levels of RGB conversion rounding. Production then normalized snapshots with the same frame rate and fitting filter as the body. The conversion regression and no-audio regression both passed after that final change, in 2.676 seconds. The obsolete shell mock was removed with the unused production shell API.

The final media-only run covered both equal and differing source/output frame rates. The final normalization change leaves the browser camera route and audio padding logic unchanged, so another expensive browser encode was unnecessary.

## Edited MP4 Frame Count Regression

A portable edit-list fixture reproduces the real captured input's distinction between copied samples and displayed frames. The test generates a two-second, eight fps H.264 source, seeks to 0.125 seconds, and copies a 1.5-second selection into a new MP4. PyAV independently observes 13 nonempty packets and 12 decoded frames. FFmpeg stream-copy probing reports 13, while FFmpeg decoding and PyAV both report 12. The copied preroll sample therefore cannot be counted as an authored body frame.

The production test failed before the correction: its final normalized snapshot selected nonexistent decoded frame 12, leaving `last.png` absent. Production now probes decoded video frames. The same fixture produces precisely 12 body frames and four synthetic transition frames, preserves decoded source order across the entire frame, and creates an audio stream matching the two-second visible duration.

After this last production correction, the edited MP4, frame rate conversion and silent source regressions all passed together in 9.935 seconds. These tests use actual FFmpeg media processing and independent Pillow, NumPy and PyAV assertions without another browser render. The input experiment and its measured table are retained as `probe-content-count.py` and `content-frame-count-fixture.md`.

## Registered Native Interaction Timing

The existing native interaction tests retained their previous three-second normalized sample positions. Their portable fixture now stores explicit sample seconds: overview zero, topic hover one, topic opened 1.44, video hover 2.4, and focused 3.6. The test normalizes those independent times using the public `transition_duration` function. The four fps production transition test uses that same phase duration, producing 14 samples instead of 12.

Both targeted real-browser methods passed in 330.493 seconds. Every native assertion remains: preview panes stay inert until opening, the first and last overview pixels match exactly, both levels in both transition directions record pointer down/up/click, the last three restore samples park the cursor and close the hierarchy, and the final root pane geometry agrees with the preceding settled sample. No additional encoded lesson was rendered.

## Owned Files

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧪️tests/🔣️.cases.json`
- Ticket inputs `run-content-camera-tests.py` and `probe-content-count.py`, this retained report, and `content-frame-count-fixture.md`.

Generated test media and browser work remain under the ticket generated directory for root cleanup at completion.
