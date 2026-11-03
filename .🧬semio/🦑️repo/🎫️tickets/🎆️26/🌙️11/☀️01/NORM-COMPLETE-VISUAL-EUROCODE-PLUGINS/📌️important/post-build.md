# Post-Build Pipeline (wave 2)

Runs after an artifact's `:build` is green. Nobody improvises it. All `repo` commands go through the central runner (memory gated, `--output=static`, `SEMIO_LANGUAGE=en`, `SEMIO_NX_GRAPH_CAPACITY_WAIT_MS=5400000`). Never run cargo/nx directly (see `build-loop.md`). Codes: `en1990`..`en1999`; project `@semio-tech/norm-<code>-rs`. Scripts live in `🗑️generated/build-runner/`: `post-build.sh` (stage driver), `latency-report.py` (renders `latency.md`).

## Verified commands (names checked in `repo --help`, `repo run --help`, each `📋️project.json`, `📜️script.ts`; none executed)

| Need | Command |
| --- | --- |
| (a) test | `repo run '@semio-tech/norm-<code>-rs:test' --output=static`. Targets of the project: `build`, `check`, `test`, `test-snapshot-sqlite`, `test-snapshot-sqlite-native`, `test-snapshot-sqlite-source`. Extra args after `--` reach nextest (`partitionNextestExecutionFilters`): `-- -E 'test(name)'`. Level via `SEMIO_TEST_LEVEL=fundamental|quick|long|exhaustive`. |
| (b) bless scene goldens | `SEMIO_UPDATE_GOLDEN=1 repo run '<proj>:test' -- -E 'test(/golden/)'`. Golden test: `golden_scene_matches_committed_bytes` (en1990/91/92/93/95/96/97/99/98), en1994 `golden_scenes_match_the_committed_bytes`, en1998 also `🧩️example` pack twin. Files: `SUBSET/🧪️tests/🧫️fixtures/🏛️scene/🔣️.json` (en1994 also `🌉️bridge`, `🎪️showcase`). `SUBSET` = `NORM/🗿️artifacts/<emoji><code>/🏅️standards/🔖️1/🪆️subsets/✳️any`. |
| (c) regenerate assets | One switch for all ten Eurocode artifacts: `SEMIO_UPDATE_GOLDEN=1 repo run '<proj>:test' -- -E 'test(/regen/)'`. Helper: `semio_s_plugin_norm_compliance::regen` (`NORM/⚖️compliance/🧪️tests/🧬️regen/🦀️.rs`). Pack and DSL: test `regen_dsl_and_pack_assets_match_the_codecs_or_bless` (en1990/92/93/94/96 in `✏️editor/🧪️tests/📚️assets/🦀️.rs`, en1991 in `💡️inferences/🧪️tests/🔬️compliance-report`, en1997 in `📸️snapshot/🧪️tests/🔬️regen-assets`): every `🗣️.dsl.semio` is rewritten canonical through the artifact codec and every `🎒️.pack.semio` beside it from that DSL; without the switch the same test asserts byte equality; a pack not named `🎒️.pack.semio` always fails. Semantic contract: test `sqlite_snapshot_en<code>_regen_semantic_contract_from_the_independent_reader_or_bless` (en1990/91/93/94/95/96/97/99; en1992 and en1998 have no `🎛️semantic.json` contract) measures the SQLite extent with the independent bun reader and rewrites `io/🪶️sqlite/📸️snapshot/🧪️tests/🧫️fixtures/🎛️semantic.json` (schemaBytes, tableWidths, rows, valueBytes) under the switch, else asserts. Unchanged own switches: en1995 `EN1995_REGEN_PACKS=1` (`regen_example_pack_assets`) and `EN1995_REGEN_MUTATION_VECTORS=1` (`regen_mutation_vectors`), en1999 `REGEN_EN1999_ASSETS=1`, en1998 `SEMIO_UPDATE_GOLDEN=1` (example pack twin). `.op/.spr.semio` are protocol fixtures found by the fixture sweep (`@semio-tech/s-fixture-sweep-rs:test`), not generated here. After a bless pass the blessed `semantic.json` needs a second run before the existing `include_str!` tests see it. |
| (d) latency + acceptance (release) | `SEMIO_LATENCY_REPORT_DIR=<build-dir>/latency repo run '<proj>:test' -c release --output=static -- -E 'test(latency_bench) \| test(live_acceptance)'`. Macros: `norm_latency_bench!` (test `latency_bench::the_realtime_gates_hold_on_a_scaled_document_and_the_report_is_written`), `norm_live_acceptance!` (module `live_acceptance`). Budgets assert only when `debug_assertions` are off, hence `-c release` (a second full compile, so it runs after the debug test is green and needs > 8 GiB available). |
| (e) oracle and feature dry runs | Python syntax: `.venv/bin/python -I` + `ast.parse` on every `🔮️oracles/**/🐍️.py` and case `🐍️.py` (no execution). Feature dry run: `cd TICKET && PYTHONPATH=SUBSET/🔮️oracles/🏃️execution .venv/bin/python 🧰️scripts/wave2-features-dryrun.py <case dir containing 🥒️.feature and 🐍️.py>` (needs node and `🗑️generated/features/dump.ts`). Rust side: `repo run '<proj>:test' -- -E 'test(oracle) \| test(third_party)'` (`evaluators_reproduce_the_blessed_oracle_cases`, `third_party_libraries_reproduce_the_blessed_oracle_cases`). Libraries are in `.venv` (structuralcodes, concreteproperties, sectionproperties, groundhog, sympy, scipy). |

Not verified: `--` forwarding syntax of `repo run` itself and the nextest `-E` filterset through `forwardAllArgs`. First use is the check; if `repo` rejects it, try `repo run <proj>:test --args=...` per `repo run --help` and fix `post-build.sh` once (single place).

## Pipeline

```
:build green (runner) -> :test (runner, decide())
  test green  -> stage acceptance (release) -> stage oracle -> done
  test red    -> classify failures in build-<code>/<n>.txt
                 only golden|pack|asset|regen|semantic names -> stage bless -> "ready for test" -> runner re-tests
                 anything else / no FAIL lines               -> logged to latest.md, no bless
```

`post-build.sh <code> [auto|bless|acceptance|oracle|status]` is called by `runner-next.sh` after every finished run of an artifact (hook requested from INFRA v4 in `requests.md`; until it lands run it by hand). State: `build-runner/post-<code>.env` (`BLESS_COUNT`, `BLESS_FOR`, `ACCEPT_FOR`, `ORACLE_FOR`), idempotent per build/test run id.

### Bless pass rules
1. At most 2 bless passes per code (`BLESS_COUNT`); more means a human looks.
2. Before the pass, all golden-like files (`🧪️tests/🧫️fixtures/**`, `🖼️assets/**`: json, `*.semio`) are archived to `build-<code>/golden-before-<n>.tar` (tar, because of path length).
3. The pass runs the golden/regen/pack/asset/semantic tests with the code's env (`SEMIO_UPDATE_GOLDEN=1` plus its regen switch).
4. Every changed file is judged: kept only if (a) it was empty or absent before, (b) it is listed in `📌️important/bless-allow.txt` (`<code|*> <path-substring>` per line), or (c) it is a `*.pack.semio` whose sibling DSL is unchanged (pack is a pure function of the DSL, asset-laws enforces it). Kept files are listed in `bless-kept-<n>.txt`.
5. Every other changed file is **restored** from the tar and its diff (unified, first 400 lines; binary: size and md5) goes to `🗑️generated/build-<code>/golden-diff-<n>.txt`, with a `needs review` line in `latest.md`. Review the diff, add the path to `bless-allow.txt`, run `post-build.sh <code> bless`.
6. If at least one file was kept, `- ready for test: post-bless ...` is appended to `latest.md`; the runner re-tests (urgent queue).

### Latency and acceptance
Output `build-<code>/latency.md`: per variant drag preview p95 (< 12 ms), commit to final checks p95 (< 400 ms: resolve + settled report), incremental vs full scene p95, entity count (>= 200), PASS/FAIL, plus the nextest result of every `latency_bench` and `live_acceptance` test. Raw reports: `build-<code>/latency/<variant>.json`; log `accept-<n>.txt`. A `debug` build row is a FAIL (nothing asserted). Only EN 1990 was named in the matrix for a first run; run any code with `post-build.sh <code> acceptance`.

## Known risks
- Nothing here was run; first live use will likely need one fix in the filter syntax.
- Latency scaling clones rows without fixing references; timings are representative, check results are not.
- Release compile is a second full compile per artifact (about the cost of the debug test build): schedule at most one at a time.
- 2026-11-03 06:33 mechanics/3 exit 0 (883s): no post-build.sh run seen in runner.log (non-artifact target); nothing to post-process. build-fixer v4
- [coordinator 12:06] [open] post-build: en1998 full_showcase DSL asset plus palette example (round-trip verified), then pack regeneration for all new showcase assets (en1998-showcase.md).

## Wave 3 changes (W3-POSTBUILD-FIX)

- Hook: `runner-next.sh` `finish_run` calls `post-build.sh build-<code> auto` after a `:build` with rc 0 (the runner then queues `:test` itself: `decide()` returns `test` while `TEST_EXIT` is empty) and after every finished `:test` of any rc. Green test: Cargo.lock check, schema catalog/index regeneration, spr/op check, release acceptance, kernel latency, oracle. Red test: classification, bless. Takes effect when the runner process is restarted (the running copy keeps the old hook).
- `📌️important/bless-allow.txt` exists (`<code|*> <path-substring>` lines). The first bless pass of an artifact (`FIRST_DONE` empty, `BLESS_COUNT` 0) writes every golden, DSL, pack, `.spr`/`.op` diff (`kept(first-green)` in `bless-kept-<n>.txt`). Later passes keep only empty-before files, allow-listed files and packs of unchanged DSLs; real diffs are restored and written to `golden-diff-<n>.txt`. Budget stays 2 passes per code.
- Regen switches: `SEMIO_UPDATE_GOLDEN=1` plus the filter `test(/golden|regen|pack|asset|semantic/)` for every artifact; en1995 also `EN1995_REGEN_PACKS=1 EN1995_REGEN_MUTATION_VECTORS=1`; en1999 `REGEN_EN1999_ASSETS=1`. `EN1991_REGEN_ASSETS` and `EN1992_REGEN_ASSETS` do not exist.
- New stages (`post-build.sh <code> locks|spr|catalog|latency`): `lock-check.py` (tomllib check that every workspace package and dependency is in `Cargo.lock`, no cargo), `s-fixture-sweep-rs:test` (with the bless env inside the bless pass so `.spr.semio`/`.op.semio` diffs are judged by the bless rules; as a plain check on a green test), `dsl-derive-rs:generate-schema-index` (30 min cap, graph-gated), latency kernels.
- Latency kernels: mechanics `latency_tests` (released once per hour, shared) and en1993 `frame-envelope` test, appended as "Kernel latency" to `build-<code>/latency.md` by `latency-kernels.py` (targets 12 ms drag, 400 ms commit). p95 numbers appear only when the test prints `p95 <x> ms` (the current asserts print it on failure only); a PASS means the in-test budget held.
- Any artifact code works (`[a-z]*[0-9]`, project `@semio-tech/norm-<code>-rs`, build state `build-runner/build-<code>.env`). The runner only queues en1990..en1999; din/iso/vdi need entries in `PROJECTS`.
