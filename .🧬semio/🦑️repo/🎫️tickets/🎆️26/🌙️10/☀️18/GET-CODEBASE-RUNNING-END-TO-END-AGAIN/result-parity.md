# Result: parity (nx vs Bazel output parity verifier), wave 1020d

READY (staged, not applied; waiting for GO). `python3 C/parity-tools/apply.py` (dry run, no drift: 24 files) then `--write`; `--undo` restores.

## What it is
`bun repo verify build-parity [--only=<ids|area:x>] [--plans-only] [--samples=<file>] [--out=<dir>] [--keep] [--json] [--recover=<dir>] [--lang=en|de]`
- Per sample: dry-run task graph of both tools, closure (Bazel cquery vs planner), then `repo --tool=nx|bazel run <target>` fresh (`--skip-cache`) and cache-restored, declared outputs captured per tool into separate roots and compared byte for byte (kind, mode, link, sha256), exit codes / failed tasks / test status, Nx `inputs` vs the inputs+env of Bazel's spawns (execution log), restore check (cold vs cache-restored per tool).
- Only `rules.json` normalizations (each with EN/DE justification + evidence, counted in the report) may explain a difference; everything else is a divergence and exits non-zero.
- Progress per sample/tool/phase and cancellation (`advanceScriptInvocation`/`checkScriptInvocation`, owned processes); declared output roots are set aside before each run and restored in `finally` (also on Ctrl-C); `--recover` restores from the journal after a crash. Tracked roots are copied (never moved), untracked ones moved.
- Report: `report.json`, `report.en.md`, `report.de.md` in the evidence directory; help and report fully EN+DE (schema-first: `parity/🧬️schema`, authored `samples.json` 27 samples/6 areas, `rules.json` 8 rules, `help.json` 35 divergence codes).
- Neutral interface additions: `repo inputs <project:target[:cfg]>` (Nx adapter -> `nx show target inputs --json`; Bazel `missing` by design, catalog + schema + help snapshots), Bazel evidence hook (`SEMIO_BUILD_EVIDENCE_DIR`: execution log flag, build events copy, closure JSON; no-op when unset), root target `verify-build-parity` + dashboard tool entry.

## Staged set (`C/parity-tools/staged/`)
- New (13): `build/🪞️parity/{🟦️,evidence,collect,report,command,tests,fixtures,schema,samples,rules,help}`, `build/🔧️tools/🏗️bazel/👁️evidence/{🟦️.ts,tests}`.
- Anchored edits of live files (11, hashes in `staged/bases.json`): `verify.ts`, `nx-operations.ts` (+test, fixture), `neutral.ts`, `bazel-adapter.ts`, `bazel-operations.ts`, `catalog.json`, `operations-schema.json`, `help-snapshots.json` (regenerated from the catalog), root `📋️project.json`. Rebuild after drift: `python3 C/parity-tools/build-staged.py`.
- `apply.py`: content-hash drift check, refuses on differing new files, over-255-char paths and running dependency sync/lock, all-or-nothing with backups under `C/🗑️generated/parity-apply`.

## Evidence (overlay copy `/private/tmp/parity/ov`, quiet tree respected)
- Parity tests 13/13 (702 expects): Ajv vs schema/data/fixtures, Bun.Glob + Python for path patterns, independent Python comparator for every authored divergence, `diff`/`shasum` for captured bytes, interruption recovery, fake-`repo` live runner leaving the checkout untouched, closed CLI grammar EN/DE. Bazel evidence tests 2 pass, 1 skipped (real Bazel, needs `SEMIO_TEST_BAZEL_BINARY`).
- tsc: new files clean; modified files clean except pre-existing unresolved workspace imports in the overlay and `📥️caller/📤️child` TS2345 (not touched).
- This resume: added localized `Usage/Options/Examples` headings (DE: Aufruf/Optionen/Beispiele), validated `C/parity-tools/post-go-samples.json` (course build+publish, play build, demonstrator release build, compact report print build) against the schema.

## After GO (plan)
1. `apply.py` (dry run, then `--write`); `repo verify build-parity --help`; `--plans-only` over all samples (cheap).
2. Per area with `--only=area:<x>` (one process at a time, memory governor respected), then `--samples=C/parity-tools/post-go-samples.json` for course, play and demonstrator builds.
3. Every divergence: decide fix (adapter bug / output declaration) or a documented rule; record here.

## Known risks
- Real-tool answers untested until GO (dry-run JSON shape verified against `bazelDryRunPlanV1`); `store` receipts of play/demonstrator `build` may need a rule if they carry timestamps.
- Heavy builds (demonstrator release) can exceed the 3600 s sample limit.
