# Heavy-job queue (owner: validation/benchmark worker)

Heavy job = Manim render, browser session, benchmark, full test suite. Unit tests at test quality are not heavy.

## Policy

- One heavy job at a time. A second runs only with a recorded CPU/RAM sample showing headroom AND an independent output root.
- Host: AMD Ryzen 7 5800X (8 cores / 16 threads), 64 GB RAM, Windows 11. Other sessions run python renders here; they are competing load. Never kill them.
- Every measurement records the competing load (system CPU % and free RAM, and the python/manim processes visible) at start.
- Output roots are isolated: `$T/🗑️generated/<job>`; no two renders for the same language share a root.
- Protocol: append `REQUEST <worker> <command> <why>`; owner answers `RUN <worker> <id>` then `DONE <id> <result path>`. Run only after RUN.

## Competing load at queue creation (2026-11-11)

System CPU 62-77 % over three 1 s samples, free RAM 31 of 64 GB. A foreign Manim render (pid 5692, `speaker` preview, about 9.3 GB working set) and a `unittest` morph run (pid 65044) were active. Headroom for a second concurrent heavy job: none for timing-sensitive benchmarks. Benchmark numbers taken now are inflated and noisy; they are valid for before/after comparison only when taken under comparable load, and every row says what the load was.

## Queue

(append below)
REQUEST playback single-frame presentation browser probe (1 frame, ~20 s) diagnosing a render.mjs exit 1 after the per-frame reset change; will run immediately unless another browser job is active (note: tiny)
REQUEST playback FD-playback-1 verification: browser probe, then presentation purity (SEMIO_TEST_BROWSER=1) and the 3 browser-backed presentation tests, one browser job at a time, root $T/🗑️generated/playback; no RUN marker was posted by the validation owner and no other browser (chrome) job is visible, so playback starts sequentially now
REQUEST exports (none heavy yet) a final+preview real-lesson export pair (compose_videos --profile preview then --profile final for one short lesson, root $T/🗑️generated/exports-real) to verify publish_export on real masters and a real course join; will ask again when the clips of that lesson carry transition documents
REQUEST clips-e2b `bash $T/clips-e2b/smoke.sh <profile> <clip> <Scene...>` ($T/clips-e2b/smoke.py: skip_animations dry run, no media; preview then final, every Beat scene of the 7 demand clips: recap, primary-energy, system-losses, regulations, demand-power, practice, certificate) to verify stage.transition runs in both profiles and the work counters (preview: full.* == 0; final: full.* > 0); one process at a time, root $T/🗑️generated/e2b; certificate Beat1 preview already done (97 s, mostly cold staffage cache: preview.swap=2, no full.*)
RUN-ANNOUNCE foundations full teaching suite (bun 📜️script.ts test; test-quality Manim, no render; root $T/🗑️generated/full-suite; started 05:23; long, one python process tree)
REQUEST clips-e1 `bash $T/bench/e1_smoke.sh <profile> <module> <Scene...>` (skip_animations dry run, no media, root $T/🗑️generated/e1; preview then final) for every migrated E1 scene: heating convection/internal-gains/solar-gains/conduction/calculation/fundamentals, cooling x5, physics fundamentals, optimization index/pareto/extrema/experiments/algorithms; scenes listed in $T/bench/e1_scenes.txt; verifies stage bindings, validation and preview/final dispatch of the 130+ migrated transitions
RUN clips-e2b smoke (coordinator grant: skip_animations dry runs write no media and are not benchmarks; may run concurrently with clips-e1 smoke)
RUN clips-e1 smoke (coordinator grant, same rationale)
RUN clips-e2b smoke (validation owner confirms coordinator grant; skip_animations dry runs, no media, not benchmarks) 05:26
RUN clips-e1 smoke (confirmed; may overlap clips-e2b because both are no-media dry runs; each one process at a time) 05:26
RUN playback FD-playback-1 (confirmed, sequential, single browser job; root $T/🗑️generated/playback) 05:26
HOLD exports pair render (not RUN until clips carry transition documents and playback/foundations suite finished; real renders need an isolated root and the CPU is already at 100 %) 05:26
NOTE validation owner: host CPU is 95-100 % from foreign jobs (not ours) so no benchmark-grade headroom; only timing comparisons under the same load are claimed. Baseline fixtures run from the HEAD snapshot C:\sb\head (git archive ff42158b973, read-only).
NOTE validation owner: the owner's own running heavy jobs: base trees probe (pid 11676, cold prep, slow) is the only baseline render; after it ends or is stopped, after-measurements run one at a time.
RUN-ANNOUNCE preview(references) skipped-animation dry run of Excel Beat2_References, preview then final, no media, one process at a time (coordinator grant for dry runs), root $T/🗑️generated/references
RUN-ANNOUNCE clips-e2b smoke-all.sh (granted at 05:26): 7 demand clips x 2 profiles, sequential dry runs, log $T/🗑️generated/e2b-smoke.log
REQUEST playback second purity browser run after fresh-context-per-frame change (one browser job, root $T/🗑️generated/tests-playback); proceeding sequentially, no other chrome visible
REQUEST staffage cold TS timing baseline (seconds per frame; isolated cache so nothing is reused): `cd /c/git/semio && SEMIO_TEACHING_CACHE_ROOT=<$T>/🗑️generated/staffage/cold-cache SEMIO_TEACHING_PROFILE=final PYTHONUTF8=1 PYTHONPATH=/c/git/semio .🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe <$T>/staffage-export.py <$T>/🗑️generated/staffage/cold-baseline.json --lods full,preview --casts cooling-internal-gains-walker,cooling-summer-strategy-wheel,ventilation-volume-cook --output-fps 15` (6 single-cast exporter runs, one bun process at a time, about 1 to 4 minutes; baseline for the preview budgets)
REQUEST staffage approved cold re-export of all 99 casts at full, preview and poster (coordinator decision 1; the exporter and the cache layout changed, so every key is new): same command with `--casts all --batch --lods preview,poster,full` and the default course cache (no SEMIO_TEACHING_CACHE_ROOT) writing `<$T>/🗑️generated/staffage/export-all.json`; run it after the baseline, one bun process per level, expect minutes for preview/poster and the longest for full (13,200 thermal frames); preview first so the preview-profile scenes are unblocked
REQUEST excel-b `bash $T/excel-b/smoke.sh <profile> en <module> <Scene>` ($T/excel-b/smoke.py: skip_animations dry run, no media, preview then final, scenes Excel Beat1_Anatomy, Beat3_Program, Beat4_Energy and BIM Beat1_DataToInformation; asserts preview full.* == 0 and final full.* > 0), one process at a time, root $T/🗑️generated/excel-b-smoke; why: evidence that the migrated sites run through stage.transition in both profiles. Also running now (lightweight, no render): BIM query benchmark `bench/bim-query-bench.py` (single python, pid visible as uv/python, before/after of the model query memoisation).
REQUEST clips-a smoke dry runs (skip_animations, no media, one process at a time) of GH Beat1_Anatomy / Beat3_Tower / Beat5_DataTrees in preview then final via clips-a/smoke.py, plus tower timing probe clips-a/measure_tower.py (cold builds are minutes each; profile final and preview) - already started one process of each under my own output root; tell me to stop if the host needs the CPU
REQUEST gh-capsule-casting render Beat2_Capsule and Beat4_Casting, language en, speaker, once with --profile final (jobs 1, root $T/🗑️generated/gh-final) and once with --profile preview (root $T/🗑️generated/gh-preview), then compare frame at f1 of every transition and read both work.json files (preview: full.prepare == full.evaluate == 0); validates the declarative capsule/casting migration against real output (flight/replace/fade are exercised only by the unit cases so far)
DONE baseline trees probe (stopped by validation owner at 05:40 after 41 min and no first Scene.play; HEAD snapshot, Beat5_DataTrees, 480p15) 
RUN validation baseline b2 h30 (base-vol-b2 Beat2_Luftwechselrate 1080p30 cold then warm; running since 05:39, root $T/🗑️generated/base-vol-b2-h30). Smoke owners (clips-e1/e2b): timings in this window are only comparable under equal load; please do not start NEW parallel smoke jobs until DONE.
