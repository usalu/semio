# Staffage Audit — 2026-11-05

**Final status:** verification dependencies were recovered and production CLI video roundtrips passed. All earlier missing-tool and pending-run entries below are historical, superseded by the final verification at the end. No ffmpeg, ffprobe or rsvg blocker remains for this completed run.

## Current Sources

The canonical archive is 🎓️teaching/🏛️architecture/🧍️staffage, separate from the launch-kit instructions. No nested AGENTS.md was found in the archive or its teaching/architecture ancestors. The root AGENTS.md applies.

The catalog manifest currently has 152 entries; the course cast manifest has 97 casts: 87 thermal/elevation/50, 4 thermal/front/50, 3 dark-line/elevation/50, 2 dark-line/plan/50 and 1 thermal/elevation/100.

## Scale Conventions

The evaluator uses metres; drawing paper size is mm = metres × 1000 / denominator. Authored representation, physical dimensions and viewport zoom are independent. Course placement maps exported paper coordinates back through mmPerMetre, then applies Manim units per metre. Transport casts use figureBoundsM for fitting, preventing plume extent from shrinking or shifting the person. Window crops are in cast metres. The bridge exposes staffage_height and staffage width using physical figure measures.

## Existing Verification

Archive config: 🎓️teaching/🏛️architecture/🧍️staffage/🧪️tests/🎚️config/🟦️.ts. It discovers every test case folder and excludes config. Relevant suites include cast-export, print-size, paper-dimensions, physical-invariance, frame-timing, playback-clock, foot-plant, course-coverage and breathing/transport suites.

The course Python course_staffage suite validates catalog references, scale availability, views, states, scenarios, source cast references and orphan casts against language-agnostic fixture vectors; a Vitest twin consumes the same vectors. Bridge tests use numpy and Manim as independent verification of placement and animation behavior.

Expected command: repo tools exec --light bun -- <vitest executable> run --config <archive config> --maxWorkers 1. The normal repo run target imports framework routing outside the present package layout, so direct configured Vitest is the useful baseline.

## Observed Runtime Defects

No functional source defect established yet: test execution is presently unavailable because node_modules/vitest is absent and bun x --no-install vitest also finds no binary. Registered python resolution fails because python3 is absent. Registered rg is absent from the tool registry; repo tools exec --light node can perform targeted source reads. Normal node execution was capacity-queued; --light completed targeted reads.

An isolated disposable test package is being installed under this ticket/🗑️generated/t. No persistent runtime dependency or source change has been added. Results will be appended after tests execute.

## Focused Baseline Results

Vitest 4.0.17 ran with ticket test-runtime.config.ts aliases, maxWorkers=1, pool=threads. Course-coverage: 49 passed, 2 failed. The course cast manifest/catalog check reports row-mismatch; source scan reports orphan cooling-heating-comparison-table. The coverage implementation owner has these concrete failures for correction. Cast-export failed suite import because jsdom was missing from disposable runtime; dependency recovery continues.

## Disposable Runtime Recovery

The isolated Bun install succeeded; Vitest and external oracle dependencies are ticket-local under 🗑️generated/t. @napi-rs/canvas is available for representative SVG raster QA. The persistent test-runtime.config.ts imports the source config through a relative path and resolves aliases from import.meta.url, so it is portable across workspace paths and operating systems. No persistent product dependency changed. The initial broken manifest rows and orphan were handed to the coverage owner; that owner is rerunning their corrected coverage suite.

## Scale and Clock Verification

Portable configured run: print-size 4/4 passed and physical-invariance 5/5 passed. This verifies physical chain, loads, thermal inputs, respiration, kinematics and transport remain identical across drawing selectors. Frame-timing failed precondition because ffmpeg is not yet provisioned. The full cast-export fixture and selected subset were stopped after 100–140 seconds without completion to bound resource cost; their beforeAll exports the whole fixture manifest irrespective of test name filtering. Direct six-still-cast export and raster proof are in progress using proof-casts.json and proof-sheet.cjs.

## Direct Export and Visual QA

The canonical cast CLI exported all six proof-casts.json stills successfully (exit 0, 6/6 frames, peak resident memory 284 MB): person-elevation, person-front, person-plan, car-plan, thermal-person, thermal-heat-pump. Every output published an SVG and cast index. proof-sheet.cjs rasterized the SVGs through ticket-local @napi-rs/canvas at high resolution, preserving source viewBox ratios. The generated proof-casts.png was visually inspected: three person projections are distinct and coherent, car plan has readable chassis/cabin detail, and thermal heat-pump shows a cooler exchanger/grille against warmer casing. These are inspection previews with each asset fitted separately, not a common print-scale sheet. Product physical-scale correctness is separately covered by the passing print-size and physical-invariance tests. Generated proof images and runtime remain disposable; source proof manifest and script are retained.

## Frame Timing Follow-up

Provisioned ticket-local ffprobe-static3.1.0 and used roots imageio-ffmpeg7.1; disposable generated/bin contains actual ffprobe.exe/ffmpeg.exe copies for direct independent oracle calls. The baseline uncovered two stale CLI test invocations missing mandatory viewpoint. Updated frame-timing test CLI fixture to explicit elevation at authored1:50 (walking-commuter has authored50/100/200/500). No product behavior changed.

After correction, 3/5 frame-timing tests pass: exact loop/non-loop frame schedules, first-party H.264 I_PCM MP4 independently decoded and inspected by ffmpeg/ffprobe at exact5frames10fps, honest codec capabilities. Both production CLI MP4/WebM roundtrips reach rasterisation and fail because rsvg-convert is absent from PATH. This is an explicit missing external tool, not a skipped test or successful video completion. Full CLI video validation remains pending real raster tool provisioning.

## Real Raster Tool Provisioning

Provisioned the actual Windows x64 static rsvg-convert release from charls-data/static-rsvg-convert v1.0.0: https://github.com/charls-data/static-rsvg-convert/releases/download/v1.0.0/win-static-x64.zip (4,413,686 bytes). Extracted using registered tar with ticket bin cwd. The real rsvg-convert.exe is on disposable PATH alongside actual ffmpeg.exe/ffprobe.exe. Full production CLI frame-timing roundtrips are rerunning with all actual external tools, without a raster shim.

## Recreating the Disposable JavaScript Runtime

Retain test-runtime.package.json as the dependency input. To recreate verification dependencies, create ticket/🗑️generated/t, copy test-runtime.package.json there as package.json, and run repo tools exec --light bun -- install --cwd <ticket/🗑️generated/t> --ignore-scripts. test-runtime.config.ts resolves those aliases portably and creates ticket scratch environment variables. Tool binaries are separate downloaded verification dependencies, not product runtime dependencies.

## Final Verification — Authoritative Result

The final focused frame-timing run completed successfully: **5 tests passed, 1 file passed, exit 0**, duration 130.34 seconds (tests 122.61 seconds). It used actual rsvg-convert, ffmpeg and ffprobe executables; the test-local ToolPort called the rasterizer directly and admitted the two tiny encoder jobs through repo tools exec --light asynchronously with cancellation forwarding. CLI parseArguments and runExport were exercised throughout. Scratch paths were ticket-local via the portable runtime config.

Validated results:

- Exact loop/non-loop schedules at n/fps.
- Production CLI MP4: H.264, yuv420p, exactly 14 frames at 12 fps, correct duration and packet spacing.
- Production CLI WebM: VP9, alpha metadata, exactly 14 frames at 12 fps, packet spacing within tolerance. Independent libvpx-vp9 decode through ffmpeg produced the expected RGBA byte count and both fully transparent and opaque pixels.
- First-party browser MP4 writer: independently decoded by ffmpeg/ffprobe with exactly 5 frames at 10 fps.
- Codec capabilities accurately distinguish the available first-party writer from unavailable MediaRecorder codecs.

The corrected test selects explicit elevation and authored 1:50; the prior ffprobe decoder-override command was replaced with actual ffmpeg RGBA decoding because ffprobe does not expose that option. Earlier runtime import/tool failures and stale fixture failures were resolved; no failures remain in this focused final run. Together with the 9 passing print-size/physical-invariance cases and successful six-cast visual QA, this completes this audit scope. Generated binaries, runtime modules, locks, scratch outputs and preview images can now be removed by root cleanup. Preserve test-runtime.package.json, test-runtime.config.ts, proof-casts.json, proof-sheet.cjs and this audit as reviewable inputs.
