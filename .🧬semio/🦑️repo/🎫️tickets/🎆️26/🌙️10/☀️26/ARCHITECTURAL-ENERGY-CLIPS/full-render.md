> Historical evidence: this report covers the revision before the uniform section-material follow-up. See [sections.md](sections.md) for the current correction and its verification.

# Full Curriculum Render

All five requested curriculum movies were rendered through the canonical authoring runtime at 640 × 360, nominal 6 fps, low quality. Complete PyAV decoding verified 46,187 final-movie frames. Every frame retained the requested dimensions and nondecreasing presentation timestamps. All 110 unique canonical sources have current render provenance and complete decode evidence (44,931 source frames); reused semantic excerpts also have current measured narration traces bound to their exact source fingerprints.

| Video | Duration (s) | Decoded Frames | Measured FPS | Codec | Audio Streams |
|---|---:|---:|---:|---|---:|
| physical_fundamentals | 1024.821 | 6149 | 6.00007 | h264 | 0 |
| ventilation | 1656.647 | 9940 | 6.00007 | h264 | 0 |
| heating | 1767.987 | 10608 | 6.00005 | h264 | 0 |
| cooling | 1047.323 | 6284 | 6.00006 | h264 | 0 |
| energy_balance | 2200.972 | 13206 | 6.00008 | h264 | 0 |

## Final Media SHA-256

| Video | SHA-256 |
|---|---|
| physical_fundamentals | 5564a953ba58b6846ee73028142d61e68bb4ebdfca171f1f53404d21f8a3d2a8 |
| ventilation | 6e6e3314a79f7f982803c113a37fd810b65f812886c5f1fac563f695cc1802bc |
| heating | 366b243ca0f1c6fc204642d0a8fd1a2abf04126606decbfe3c5ebf2f15f18950 |
| cooling | 22b50ab73f2e75bf4c5f58ef54c9f3df09182ef4c649e65d48a2d9a9d1ab3f95 |
| energy_balance | d774dc9dc35b5755053838ab5adb500ccc1b1f8107c56a76c6f9781e5f17bde6 |

The final activity repair refreshed exactly five canonical source jobs (73–77: OfficeRoom, HumanFactor, DevicesLighting, CumulativeLoad and Mitigation) in Cooling_02_InternalGains, then reassembled Cooling through the normal canonical cache. All four other complete-movie hashes remain exactly unchanged from their accepted preceding renders. Source 74 now has media SHA-256 4c45a259a06912d02c2fdbff45cd3e7497938ca76de851caf9e6ffc360003441. No source edits remain pending.

## Visual Acceptance

Independent review is accepted. All 110 unique manifest sources have sampled visual coverage: 708 selected source-frame reviews and 48 movie-sample reviews, including the superseded Cooling comparison. The final corrected source-74 comparison was inspected in 12 frames; the updated assembled Cooling comparison was accepted at 209.5 seconds. The known lamp, sash, solar-readout and activity-pose findings were resolved and refreshed before acceptance. No actionable sampled visual finding remains. Detailed evidence is in full-render-audit.md and activity-poses.md.

Complete decoding establishes media integrity; sampled visual review does not mean every frame was manually inspected. Current canonical narration assets are absent, so these authored movies have no audio streams. No speech generation was requested or invoked. The separate real-source audio integration test validates attachment and excerpt preservation when clause recordings exist.

## Validation and Reproduction

Initial rendering used two source workers; later normal resumes used four capacity-managed workers. Published caches and atomic provenance guards preserved valid completed jobs. Source failures, intentional caption fixes and the bounded owned-process restart are recorded in full-render-events.md.

Run all commands through the repo binary from C:/git/semio. Set UV_PROJECT_ENVIRONMENT and SEMIO_TEACHING_ENV to C:/git/semio/.🧬semio/🎓️teaching/⚡️cache/python; set PYTHONUTF8=1, PYTHONDONTWRITEBYTECODE=1, and ticket-local SEMIO_TEACHING_OUTPUT_ROOT/SEMIO_TEACHING_CACHE_ROOT.

1. repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts render --quality l --resolution 640,360 --fps 6 --jobs 4 --no-play --log-output --output-root <ticket>/🗑️generated/full
2. For the final local refresh, add --video cooling --chapter Cooling_02_InternalGains; then rerun with --video cooling to assemble its full movie. Both completed with exit code 0.
3. repo tools exec --light uv -- run --project 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📦️packages/🐍️python --python 3.12 --group test python -B <ticket>/verify-full.py
4. Run the same managed command with <ticket>/verify-sources.py. Both final verification commands completed with exit code 0. Unchanged files reuse prior complete-decode evidence only after exact binary SHA agreement; source evidence additionally requires the current provenance digest. Updated Cooling and five refreshed sources were decoded anew.
5. repo tools exec bun -- 🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/📜️script.ts test runs the canonical aggregate, including the newly registered local activity module. Its final result is recorded below.

The activity-specific suite passed three tests in 0.299 s after reproducing the original failures. It validates portable cases with real Manim geometry and SciPy raster topology; seated pose and metabolic values are preserved. Full implementation paths are in files-reuse.md. Per-source durable provenance and decode evidence are in source-verification.md.

Generated movie binaries, attempt logs, contact sheets and JSON evidence were removed from the ticket 🗑️generated folder after final acceptance, as required before ticket close. Retained Markdown reports, exact hashes and reproduction scripts preserve the result.

## Final Aggregate Test Result

The canonical authoring test command completed with exit code 0: 90 tests in 529.889 seconds, OK with one existing Windows symlink skip. This includes the three new activity tests through explicit canonical registration, real semantic audio/video render-and-excerpt coverage, shared geometry/style/font checks, actual scene integration and transform raster regressions. No test failure remains.
