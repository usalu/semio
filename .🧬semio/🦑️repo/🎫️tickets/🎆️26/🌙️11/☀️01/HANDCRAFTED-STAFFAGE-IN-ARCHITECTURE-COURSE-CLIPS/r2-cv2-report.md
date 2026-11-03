# r2-cv2: verification of the open items of r2-cv, and a two-direction audit of the cast cache key

Scripts: `T/🧰️cv2/` (run Python ones with `T/🧰️cv/run.sh <abs path>`; the TS one with `repo tools exec bun -- <file>`). Generated output was deleted.
V = `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals`, S = `🎓️teaching/🏛️architecture/🧍️staffage`.

## Verdicts

| Item from r2-cv | Verdict | Evidence / action |
|---|---|---|
| Cache race on the same cast id | Disproved | `race_real.py`: 3 processes run the real exporter on the same two cast ids at once against one empty cache. All exit 0, no leftovers, 2 key folders, each cast written once and byte-identical. |
| `rename` without retry on transient Windows locks | **Confirmed, fixed** | `rename_lock.py`: a handle held on a file in the rendered folder for 0.4 s makes `ensure_casts` fail with `PermissionError` and discards the whole batch. Now `_publish` retries 8 times with a growing pause (V/🧍️staffage/🐍️.py), and it still keeps a cast that another renderer won. Regression: `Cache.test_a_transient_lock_on_the_rendered_folder_does_not_lose_the_batch`, JSON case `publish`. `negative.py` shows it fails with the old code. |
| Pose not checked before writing in `exportCast` | **Confirmed, fixed** | The doc comment promised "throws before writing anything". With a good cast followed by a bad pose, the good cast stayed in `out` (vitest failed with `['fine']`). `framesOf` now resolves the pose through `poseOf` (S/🔨️modules/📤️export/🎭️cast/🟦️.ts:61), which runs in the planning pass. Unknown clip and unknown preset already refused correctly. Regression: three vitest cases (`refused` vectors in `🧫️fixtures/🎭️cast-export/🔣️.json`, test in `🧪️tests/🎭️cast-export/🟦️.ts`). |
| Transient thermal under a timeline environment | **Confirmed, fixed** | `thermal_env.ts`, kettle (`boil`, evaporating vent) under a two-key timeline: the track's shared `thermalSession` returns parcels that differ from the session-free solve at all three times (60, 120, 180 s). With a constant environment they are equal. Cause: the session instants and chains hold the environment of the first call, and the track calls with a new sampled environment per time (`createTrack` → `thermal(state, ambientAt(...).environment)`), which breaks the session contract of ONE environment. Fix: `transientThermal` keeps one session per environment (`environmentKey`, at most 8, S/🔨️modules/💨️transport/🟦️.ts). A constant environment keeps full sharing. Regression: `💨️transport-assets` "solves the region temperatures of an evaporating vent in the environment of each instant"; it fails with a single shared key (checked). Latent in the course (no cast combines a timeline with a region-following vent) and now closed. |
| Cache directories never pruned | **Confirmed, fixed** | `prune_stats.py` before the fix: 1265 key folders, 4.48 GB, 3 live keys, 1262 stale, 4 leftover `.tmp-*`. Now `_prune` (called after every render that added casts) removes key folders and `.tmp-*` folders that no manifest cast maps to and that were not written for 24 h. Live keys are never touched, so a running scene never loses frames. Regression: `Cache.test_only_keys_without_a_cast_and_older_than_a_day_are_pruned`, JSON case `prune`. The first prune happens on the next render miss; nothing is younger than a day today, so the 4.5 GB go over the next days. |
| Windows job assignment race (`_Tree._assign`) | Disproved | `job_race.py`: 150 runs of `cmd /c ping` (cmd starts its child within milliseconds) killed through the job leave 0 grandchildren; the control without a job (plain kill of the parent) leaves 10 of 10. |
| Transport solve ignores the cancel signal | Disproved as a problem | `cancel_latency.py`: CTRL_BREAK to exit takes 0.04 to 0.05 s for the kettle (first frame and mid-run), vacuum, cook and lamp casts. On Windows the break kills the process outright, on POSIX a frame lasts milliseconds, and `_Tree.stop` kills after 3 s anyway. No change. |
| Looping clips with non-integer `durationS × fps` | Disproved as a problem | `loop_period.py`: 1 of 53 clip casts qualifies (`cooling-internal-gains-jogger`, 0.72 s at 15 fps, 11 frames, period 0.7333 s, seam 13 ms). Same on both sides by design (`frameSchedule`, `seek`). No change. |

## Cache key audit

`key_audit.py` copies the exporter sources, the schema, the zip writer the closure reaches and a catalog subset into a scratch tree, changes one input at a time, runs the real exporter from that tree on two casts (a line pose and a thermal pose with environment `office-neutral`), and compares the Python key and the SHA-256 of all output files with the unchanged tree. Rendering the unchanged tree twice gave identical bytes, so output comparison is sound. All variants are built from one snapshot, because other agents edit the live sources while the audit runs.

Legend: ok = key changes exactly when the output changes. Over = key changes, output does not (a harmless re-render). Stale = output changes, key does not (a wrong cast; none found).

| Changed input | line cast | thermal cast |
|---|---|---|
| Asset JSON, drawn geometry | ok | ok |
| Asset JSON, thermal region area | over | ok |
| Asset JSON, summary text | over | over |
| Asset JSON, the asset's own unused environment | over | over |
| Another asset's JSON | ok (same) | ok (same) |
| Environment preset used by the cast | ok (same) | ok |
| Environment preset not used by the cast | ok (same) | ok (same), after the fix below |
| Catalog manifest (category title) | ok (same) | ok (same) |
| Print profiles: default profile switched | ok | ok |
| Print profiles: a number of the default profile | over | over |
| Manifest row field (for example `thermalSpanK`) | ok (`entry` is hashed whole) | ok |
| Exporter behaviour (margin 1 to 2 mm) | ok | ok |
| Exporter comment only | over | over |
| Schema twin comment only | over | over |
| Module outside the import closure (pdf, dwg rust) | over | over |
| Stray editor file in the modules | over | over |

Findings:
- No stale case. Every input that changed the output also changed the key.
- Fixed one over-invalidation: before, the key hashed every environment preset, so editing any preset re-rendered every thermal cast, and for casts without an environment `environment` and `timeline` are absent so they were already exempt. Now only the presets a cast names (`environment` and the `timeline` keys) enter its key (`_presets`, `_environment_bytes`). Regression: `Cache.test_only_the_environment_presets_a_cast_names_enter_its_key`, JSON case `environments`. Unused presets: key same, output same, confirmed with real renders.
- Left as over-invalidation on purpose: a comment-level or irrelevant-field edit cannot be told from a behavioural edit without a semantic digest. The import closure covers 46 of 60 files under the hashed folders, so restricting to the closure would gain little and would let a new runtime read of a file slip through; the whole folder hash stays the safe default. The zip writer under `🎯️targets/⚛️react/🔨️modules/📤️downloads/🗜️zip` is reached by the closure but outside the digest; the cast path never calls it (output-neutral), so it stays out to avoid needless re-renders.
- Not keyed and accepted: the bun version and the host. The exporter is deterministic on one host (two renders identical).

## Tests

- Python: `m9_tests.sh` on the staffage suite plus `🐍️icons` (`SEMIO_TEACHING_LANGUAGE=en`): 50 tests, OK, 25 s. One transient `Fit` failure in the first run came from another agent's in-progress edit of `_Figure` and passed on the next runs.
- vitest (`q2-vitest-alias.config.ts`): `🎭️cast-export`, `💨️transport` (assets, field, view), `🫕️phase-change`, `⏳️thermal-transient`, `🌡️thermal-span`: 7 files, 160 passed, 1 skipped (the SIGTERM test on Windows).

## Files changed

- V/🧍️staffage/🐍️.py: `_publish`, `_prune`, `_presets`, `_environment_bytes`, constants.
- V/🧍️staffage/🧪️tests/🐍️.py and `🔣️.json`: three tests; JSON cases `publish`, `prune`, `environments`.
- S/🔨️modules/📤️export/🎭️cast/🟦️.ts: pose check.
- S/🔨️modules/💨️transport/🟦️.ts: session per environment.
- S/🧪️tests/🎭️cast-export/🟦️.ts, S/🧫️fixtures/🎭️cast-export/🔣️.json, S/🧪️tests/💨️transport-assets/🟦️.ts: regression tests.

## Still open

- Bun version is not part of the key.
- The Python side keeps `_source_digest` and `_catalog_bytes` for the life of the process, so a long-lived process that outlives a source edit keeps the old key (renders are one process each).
