# Virtual Lesson Window Verification

The five-window presentation is covered by portable layout/timing fixtures, independent geometry and image oracles, real encoded movies with audio, and runtime publication tests. The tests were authored before the presentation implementation. No production implementation or AGENTS.md file was edited by this test agent.

## Final Results

The latest component runs cover **63 test methods: 57 passed and six documented skips**. These are component results, not one combined command.

| Suite | Result | Elapsed |
| --- | --- | --- |
| Presentation contract and runtime publication | 8 passed | 3.213 s |
| Presentation native image/movie/audio | 2 passed within the nine-test full run | 67.562 s for that full run |
| Existing runtime regression | 25 passed, 6 skipped | 13.607 s |
| Semio style and compact tabs | 9 passed | 10.047 s |
| Existing native font regression | 13 passed | 14.699 s |

After the final native render, the same containment assertion adapter was strengthened and executed read-only against all five saved movies. It proves that **all eight original frames survive in order** at focused body frames 8–15, rather than checking only one representative frame. No additional presentation render was needed.

The runtime skips are five opt-in real-Manim integration checks and one Windows symbolic-link privilege check. The new presentation movie tests themselves use real Manim geometry, FFmpeg, PyAV and Pillow with no skips.

## Portable Contract and Independent Oracles

The presentation fixtures are in `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🔣️.cases.json`. They independently describe:

- The exact physical fundamentals → ventilation → heating → cooling → energy balance order.
- English and German window titles, with explicit locale selection.
- The five overview rectangles in a 1600×900 viewport, navbar/footer/title/inset metrics, and focused content viewport.
- Full-source containment with no discarded chapter/context strip.
- Overview hold, zoom and focused hold durations.
- Geometry at 1600×900, 320×180 and 1920×1080.
- Decoded RGB boundary identity within and between lessons.

NumPy independently computes every scaled window/body rectangle. Pillow compares deterministic overview pixels exactly and resizes independently decoded source frames to verify focused containment. The source frame nearest-match oracle rejects lost, repeated or reordered body frames. The representative midpoint mean RGB difference is 7.9724 on the 0–255 scale, below the fixture tolerance of 10; differences reflect independent resampling and YUV chroma rounding.

## Native Five-Movie Evidence

The tests generate two seconds of canonical H.264/AAC MP4 test-pattern content with a 48 kHz 440 Hz narration surrogate. Each lesson wraps that content at 320×180 and four frames per second.

All five outputs independently decode to:

- Exactly **24 video frames and 6.0 seconds of audio**.
- Two-second silent transition padding on each side.
- Retained 440 Hz tone with RMS **0.0883208** in the content interval.
- Prefix and suffix RMS **0.0**.
- All eight source frames preserved in order in the focused viewport.
- Identical first and last decoded RGB overview frames within each movie and across all five movies.

The ten boundary frames share SHA-256:

`d50af7e47a8d32096b2191f77c544cfb4272b19161a4cf2618b6c840e69b5129`

Generated evidence is under `🗑️generated/virtual-ui-tests/native-evidence`: five movies, overview/zoom/focused/return PNGs for each, regenerated deterministic source content, and `audit.json` containing frame counts, audio metrics, frame-order matches and boundary hashes. The independent visual review agent was given these refreshed samples.

## Runtime and Publication Safety

The tests require an explicit language and preserve the concurrently introduced explicit `audience="speaker"` interface. The curriculum fixtures now contain 113 authored segment occurrences, 26 logical chapters and 105 unique source renders, with no institute intro records. The obsolete physics composition entry is no longer used by the curriculum checks.

A runtime integration test proves raw content is concatenated before one presentation envelope and before final certification. Presentation cache identity includes locale and every source returned by the presentation source closure. The existing segment/excerpt tests isolate their established raw-content behavior behind a copied test envelope; the native suite independently validates the real envelope.

Publication regression cases prove:

- Changed presentation sources preserve the prior certified movie and certificate.
- A stale content certificate preserves the prior pair.
- A real nested authored source hash mutation is rejected even when the supplied content state has not changed.
- A certificate write failure after valid movie replacement cannot leave the previous certificate attached to the new movie.

The compact tab style cases prove chapter and section tabs share the left edge, use the prescribed spacing, retain neutral chapter text and primary section emphasis, and use body/label type scales. All 13 native font tests still pass, including exact source-outline comparison, .notdef rejection, cmap parity, cache identity, German/math glyphs and native text canvas restoration.

## Failing Baselines and Corrections Verified

Before implementation, four presentation contract tests failed because the module was absent, in 0.024 seconds. Native testing then caught and drove corrections for a Manim/Pillow image mode error, an inferred million-frame-per-second concat output rate, unbounded AAC stream composition, and consistent one-frame/audio-duration truncation. Canonical MP4/AAC input reproduced the audio stall, so the final passing evidence exercises the actual narration path.

Publication tests separately demonstrated two initial failures when source mutations replaced a certified movie too early, and a certificate I/O failure that left stale certification. Their corrected behavior now passes. Timed test-owned cancellation stopped stalled encoder attempts. Scoped process inspection was used only to stop inspected descendants of this agent's own capacity grants; other agents' renders were untouched.

## Owned Files

Created:

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🧪️tests/🔣️.cases.json`
- Ticket `virtual-ui-tests.md`
- Ticket input `run-virtual-ui-tests.py`
- Ticket input `audit-native-evidence.py`
- Ticket input `inspect-native-processes.py`
- Ticket input `stop-native-processes.py`

Updated:

- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🧪️tests/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🧪️tests/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🐍️.py`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧪️tests/🎨️semio-style/🔣️.cases.json`
- `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts` presentation test registration.
- Ticket `accent-tests.md` latest compact tab/font verification.

No owned file was removed. Process input scripts contain no hardcoded PID defaults. Inspection requires an explicit root PID; stopping additionally requires explicit child PIDs and verifies their current parent-tree membership and Python/FFmpeg executable identities.

All test/validation processes have exited. Generated logs and movies remain for root review and required ticket cleanup. Permanent input scripts and Markdown audits must be retained.
