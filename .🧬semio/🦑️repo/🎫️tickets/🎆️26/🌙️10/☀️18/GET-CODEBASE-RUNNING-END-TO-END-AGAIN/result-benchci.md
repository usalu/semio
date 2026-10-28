# result-benchci (lane benchci `bc`, wave 1027b, 10-27 ~19:00-19:55): the acceptance targets as a permanent regression check owned by the repo

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`; run everything from `/Users/ueli/Documents/semio`. Nothing under tracked source was edited, no official product run was started, no sub-agent, no modifying git command. One mirror (`bc`, mine) carried every law; it is dropped at the end (`sh C/benchci-tools/rebase.sh` recreates it).

## FOR coordinator (ready lines)

FOR coordinator: batch `acceptance` ready, 14 paths (9 new, 5 edited), base = live of 19:54 (`stage.py check` on live: 14 x `base`).
- Apply: `python3 -I C/benchci-tools/staged/acceptance/stage.py check --root /Users/ueli/Documents/semio && python3 -I C/benchci-tools/staged/acceptance/stage.py apply --root /Users/ueli/Documents/semio` (keeps file modes; `python3 -I C/xplat5-tools/tests/test_stage_mode.py C/benchci-tools/staged/acceptance/stage.py` OK).
- Revert: `python3 -I C/benchci-tools/staged/acceptance/stage.py revert --root /Users/ueli/Documents/semio`.
- Drift (exit 1/2, hot files: bench command, bench tests, operations catalogue, README, help snapshot): `sh C/benchci-tools/rebase.sh` rebuilds the batch on the live bytes in one go (anchored edits, a moved anchor stops with exit 2 and names it; it creates mirror `bc` when missing, applies, regenerates the help snapshot with the real renderer there, rebuilds with it). The help snapshot is a 600 KB rendered fixture: never hand-merge it, rebase.
- Laws (mirror only, 28 s; mirror `bc` is dropped, `sh C/benchci-tools/rebase.sh` recreates it and prints the line): `sh C/benchci-tools/staged/acceptance/verify.sh $TMPDIR/sm/bc` -> `acceptance VERIFIED` (acceptance 23/0, bench 43/1, operations 14/1, workflow law 7/0; the two single failures are mirror-only and identical on the base files, see "Evidence"). Negative controls: `python3 -I C/benchci-tools/negative.py <mirror>` -> 7 x `RED-AS-REQUIRED`.
- Product proof (the batch changes no build behaviour, but the command must be seen working once on the real route; sequential, gate window, one run at a time):
  1. no edits, safe on the live tree: `bun repo bench acceptance --scenario=kind:noop --lang=en --file=C/🗑️generated/benchci/noop.json --markdown=C/🗑️generated/benchci/noop.md` (3 products x [2 prime runs + 3 hot runs]; 5-25 min depending on warmth). PASS = last block `ACCEPTANCE PASS`, exit 0, rows `course-noop-wall/footprint/executed`, `dashboard-noop-*`, `print-noop-*` and the 6 hot records with `Footprint` in MB (no `<=` prefix on macOS = physical footprint, `<=` = resident fallback). Today's expected picture (gate logs g41-g43): course wall <= 10 s on a quiet host, footprint 0.7-1.1 GB, 0 executed; dashboard 1 executed (the freshness worker), print wall 10-25 s -> `fail` of the ADVISORY `print-noop-wall` only.
  2. edit + peer stages edit and restore tracked live files, so only in the gate's quiet window with nobody editing `🎓️teaching/…/📚️course/📚️course/🟦️.ts`, `CITATION.cff`, the procedural plugin `📜️script.ts` and the capacity `consumption/🟦️.ts`: `bun repo bench acceptance --in-place --scenario=product:course --file=C/🗑️generated/benchci/course.json --markdown=C/🗑️generated/benchci/course.md` (~8 min). PASS = `restored: yes`, `landed: yes`, all `-executed` rows `pass`, `course-edit-wall <= 60 s`; `git status --porcelain` on the 4 files empty afterwards. A kill leaves `.cache/acceptance/journal.json`; the next run restores from it first.
- After landing: README (EN `### 🏎️ Performance objectives`, DE twin) carries the new bullet; the docs lane may mirror one line into `📓️build-guide.md` next to the bench baseline paragraph (not done, not mine).

FOR user: CI can only run what is pushed. `.github/workflows/acceptance.yml` is a NEW workflow: `schedule` fires only for the workflow file on the default branch (`⛳wip`), `workflow_dispatch` works from any branch once pushed. The workflow law and cicover's staged laws (`ci1`: runner pins, SHA pins, timeouts, artifact names) were applied to it: live law 7/0, `ciwf-tools/run-law.sh` 0 findings; `ci1` itself has 2 drifted files on live (not mine), so its law file could not be run against my workflow; its generic rules were read and met (pinned `macos-26`, SHA pins, step limits <= 60 min, valid artifact names).

## What the batch adds

`repo bench acceptance [--scenario=<ids|all>] [--in-place] [--file=<json>] [--markdown=<md>] [--json] [--lang=en|de]` (route: the bench command dispatches `bench acceptance` to the new module; nothing else of `bench` changed).

| path (under the build library `🏗️build/📏️incremental/🏎️bench/🛂️acceptance/` unless noted) | role |
|---|---|
| `🧬️schema/🔣️.json` | schema first: Policy const (roles, metrics, clock grains, reasons, options, door, journal, environment allowlist), Set (the budget table), Record, Outcome, Row, Tally, Report, Fixture; 1154 lines |
| `🎯️budgets/🔣️.json` | the declared budgets: 3 products, 10 scenarios, 27 budgets, 75 bilingual labels; each budget names its source and, where one exists, the bench SLO objective it restates (`slo`) |
| `🟦️.ts` | pure logic: table admission (`acceptanceSetIssuesV1`), selection, edit line/mutation/restore on bytes, the observer (reuses the bench observer: executed actions, phases, inferred projects, plus memo service in EN/DE), `acceptanceJudgeBudgetV1`, invariants, tally, verdict, report, EN/DE text and markdown rendering |
| `🏃️runner/🟦️.ts` | scenario engine over ports (prime x2 + hot x3 / control + peer + follow / edit + revert), crash journal + recovery, cancellation, and the real spawner `acceptanceSpawnV1` (exact monotonic wall clock to the process exit, physical footprint of the whole tree via the capacity governor's `observeTreeMegabytesV1`, tree termination on timeout/abort) |
| `⌨️command/🟦️.ts` | arguments, report emission (stdout text/JSON, `--file`, `--markdown`), exit policy, binding to the invocation (progress `bench:acceptance:<scenario>:<role>:<n>`, cancellation), child command line (door + `repo --tool=bazel run <target>`), child environment |
| `🧪️tests/🟦️.ts`, `🐍️oracle/🐍️.py`, `🧫️fixtures/🔣️.json` | 23 laws, an independent Python oracle, 12 recorded gate runs |
| `.github/workflows/acceptance.yml` | nightly (02:17 UTC) + manual, `macos-26` (or the variable `SEMIO_ACCEPTANCE_RUNNER`), one job per product, fails on a breach |
| edited: bench `⌨️command/🟦️.ts` (7 lines), bench `🧪️tests/🟦️.ts` (the subcommand law lists `acceptance`), `⚙️operations/🔣️catalog.json` (flag `--scenario` also for `bench`, new flag `--in-place`, subcommand, synopsis, summary, 2 notes/flag texts, example; EN+DE), `⚙️operations/🧪️tests/🧫️fixtures/📖️help/🔣️.json` (regenerated, 174 lines: root, `incremental*`, `conformance*`, `bench*`, new `bench acceptance`, both languages), `🦑️repo/README.md` (EN+DE bullet) | |

`--in-place` is required for every scenario that edits files; `--scenario=all` without it runs the no-ops and names the others as skipped (the report lists them `skipped`, rows `unproven: skipped`), an explicit selection of an editing scenario without it is refused (EN/DE). The default checkout is the one the command runs in; a shared working tree must never use `--in-place` (the gate and CI runners are the places).

## Scenarios and budgets (all in `🎯️budgets`, none in code)

Order: hot no-ops, then peer stages, then edits (an edit leaves the product needing a re-prime). A product that is not hot first gets TWO unjudged prime runs (the first builds, the second absorbs the memo/plan replan that the first hot run after a build is known to pay for: g34-hot1 20 s, hot2/hot3 10 s).

| budget (statistic over the runs of its role) | maximum | enforcement | source / evidence |
|---|---|---|---|
| course hot no-op wall (max of 3) | 10 s | blocking | brief 1026g; SLO `product-course-noop-wall-p50`; g41/g42 10.0 s |
| course hot no-op footprint | 1536 MB (1.5 GiB) | blocking | SLO `product-course-noop-rss-max`; g41 0.7-1.1 GB |
| course hot no-op executed actions | 0 | blocking | `INFO: 1 process: 1 internal` |
| dashboard hot no-op wall / footprint | 10 s / 1536 MB | blocking | g41-dash3 10.0 s 1.15 GB |
| dashboard hot no-op executed | 1 | blocking | the freshness worker (`prototype is current`) runs on every build; the SLO table says 0 and is stricter than the product (see decisions) |
| print hot no-op wall / footprint | 10 s / 1536 MB | advisory | print-rs:test answers in 10-45 s today (g34-g45); executed 0 is blocking |
| course edit wall, revert wall | 60 s, 60 s | blocking | g42c 50.5 s / 40.3 s |
| print (Rust) edit wall, revert wall | 120 s, 120 s | blocking | g36q 105.9 s, g43q 50.5 s |
| dashboard (Rust) edit wall, revert wall | 120 s | advisory | no green measurement of a dashboard edit exists yet |
| peer edits: `control`, `peer`, `follow` executed (course unrelated + scheduling, print unrelated, dashboard unrelated) | 0 (dashboard 1) | blocking | brief 1026g; harness3 p1/p3 files: `CITATION.cff`, procedural plugin `📜️script.ts`, capacity `consumption/🟦️.ts` |
| invariants (blocking, not budgets): every official run exits 0; edited files restored byte-exact (a foreign write is kept and reported `unproven: foreign-write`); the peer edit landed while the run executed (else `unproven: not-landed`, no follow-up run) | | | |

Verdicts: `pass` / `fail` / `unproven` with a reason. Time passes at `<=` the maximum; a recorded poll clock (5 s grain) is an upper bound, so a value within one grain of the maximum is `unproven: clock-grain`, never a false breach; a time breach on a saturated host (load/core >= 1.5) is `unproven: host-saturated`; a resident-set-only memory read above the maximum is `unproven: resident-only` (macOS RSS over-reads shared pages 3x). The run FAILS (exit 1, after `--file`, `--markdown` and the text were written) when a blocking budget or invariant failed, or when no blocking budget was proven at all (invariants do not count as proof), or on cancellation (edits restored, partial report written).

## Evidence (all on mirror `bc`, graph-free, no product build)

- Recorded logs as fixtures (`🧫️fixtures`, 238 KB, every line a tool reads kept verbatim, compiler chatter elided with a count): 12 cases from today's gate runs (g34-g46 series: g36p, g36q, g41-hot2, g41-dash3, g41-print3, g42-hot2, g42c-edit, g42c-revert, g43-hot3, g43c-edit) each with the verdicts the budgets must give, written by hand in `mkfixtures.py`:

| case (recorded run) | wall (poll) | resident | executed | verdicts |
|---|---|---|---|---|
| course hot g41 | 10.0 s | 1119 MB | 0 | wall pass, footprint pass, executed pass |
| dashboard hot g41 | 10.0 s | 1147 MB | 1 | all pass |
| print hot g41 | 10.0 s | 1491 MB | 0 | all pass |
| course hot BREACH g43-hot3 (journal lost) | 138.6 s | 2478 MB | 0 | wall FAIL, footprint unproven (resident only), executed pass |
| course hot g42 | 10.1 s | 1074 MB | 0 | wall unproven (clock grain), rest pass |
| course edit / revert g42c | 50.5 s / 40.3 s | | 1 / 1 | pass / pass |
| course edit BREACH g43c | 133.5 s | | 1 | FAIL |
| print edit g36q / BREACH g36p | 105.9 s / 161.1 s | | 2 / 2 | pass / FAIL |
| peer run g41 (read as peer run) / g42c-edit (re-executes 1) | | | 0 / 1 | pass / FAIL |

- Independent oracle (`🐍️oracle/🐍️.py`, stdlib only, no shared code): reads the recorded logs (executed = Bazel `INFO: N process(es)` cells minus `internal` and `... cache hit`, memo service EN/DE) and judges a seeded grid of 700 budget x run-set cases (metric, statistic max/min/p50, clock kinds, resident/footprint, loads around 1.5, exits, killed, outcomes, 0-4 samples): the TypeScript agrees on status, reason, value, n and bound for every case; the grid reaches all 3 statuses and >= 7 reasons. Edits: 66 random byte strings (empty, no final newline, CRLF, binary, UTF-8) checked by the oracle for "exact append" and "byte-exact restore". Third-party: `ajv` (strict) validates the schema, the table, the fixture and every report; `yaml` parses the workflow; `statistics.median` for p50.
- Budget table: closed (repository validator and ajv agree); 14 planted inconsistencies are all found; every budget that names an SLO objective restates its number exactly (13 budgets, footprint in bytes); the numbers of the brief are asserted one by one.
- Scenarios with scripted ports (recorded logs replayed): run order, two primes per product that is not hot, edit visible during the edit run and gone for the revert, peer edit lands at the first graph line (child sees it while running, not before/after), no follow-up when it never lands, foreign write survives the restore, failed run restores and skips the revert, missing file fails only its scenario, cancellation restores and reports, journal recovery of a killed run, command end to end (pass exits clean in EN and DE; breach writes `--file` and fails with the bilingual message; `all` skips editing scenarios and says so; cancellation writes a partial report).
- Real files + real process: a stub child replays a graph line, waits, reports whether the file holds the edit; peer and edit scenarios run through the real spawner with real files and a real journal: `edited=true` only in the peer/edit run, every file byte-exact afterwards, no journal left, exact clock >= 500 ms, observer reads the child's `INFO` rows.
- Exact clock + footprint: a real child allocates 300 MB, sleeps 2.2 s: wall >= 2.2 s, `peakMb >= 250` (physical footprint on macOS; resident fallback is recorded as such), measure recorded; wall of `python3 -c sleep(1.5)` agrees with Python's own `perf_counter` wrapper within 1.5 s; a timeout and an abort terminate the whole tree (`killed: timeout|abort`, exit null).
- Workflow: parsed with `yaml`: schedule + dispatch only (no pull request/push), `contents: read`, pinned runner, SHA-pinned actions, `--in-place`, per-product `--scenario`, the measuring step has no `continue-on-error` (only the advisory rows' job level), summary and upload `if: !cancelled()`, tool-setup steps limited, budgets (ms) below job timeouts. Live workflow law 7/0 with it; `ciwf-tools/run-law.sh`: 3/0 + 0 findings x 3.
- Negative controls (each planted violation turns the named law red, the batch is written back after each): loosened budget (3 failing), budget drifted from the SLO (2), exclusive instead of inclusive maximum (7), restore that leaves the edit (6), journal never cleared (3), a nightly step that cannot fail (1), a catalogue without `--in-place` (1).
- Static: parse scan of all 7 TS files OK (`gate-g-tools/scan.ts`, 0 bad); `tsc --strict --noUncheckedIndexedAccess` clean for the module (one error found and fixed in my test); no `//` comment inside any definition; 61 docstrings, each starting with its own emoji, none used elsewhere in the repo (checked against all TS/MJS/PY/RS/GO docstring leaders); longest batch path 119 characters (limit 256); stage mode law OK.
- Mirror-only failures, identical on the base files (measured with the batch reverted): bench `the product catalogue holds the brief's budgets … every edited file exists` (needs `🕸️graph/…/🦀️rust/📜️script.ts`, not in a mirror) and operations `the real CLI prints the own help …` (needs the `✏️s` tree). Door shape proven graph-free: `bun <door> repo bench acceptance --help` prints the new help (EN, 0.1 s).

## Design notes (why it is like this)

- Official route and clean children: each run is `process.execPath <door> repo --tool=bazel run <target>` in the checkout, with the incremental harness's child environment (no inherited `SEMIO_*`/`NX_*` invocation state, so a run nested in `repo bench` consumes no parent pin or grant) plus an allowlist (`SEMIO_REMOTE_CACHE_*`, `SEMIO_HOST_CAPACITY_*` - the 16 GB declaration of `bench2-g` works - and `SEMIO_BUILD_BUDGET_MS`). `bench` is routed before the lease/capacity wrappers (`requires: none`), so the parent holds nothing the child needs.
- Memory is what the capacity governor's own observer reports for the whole tree under the run's pid (Bazel's daemon JVM is outside the tree, as in the gate's numbers); the record says `footprint` or `resident`, never silently the weaker one.
- Safety of edits on a shared tree: only with `--in-place`; restore is byte-exact from an in-memory copy AND an on-disk journal (`.cache/acceptance/journal.json`, repaired first by the next run), a foreign write during a stage is never overwritten (our line is removed, theirs stays, the row is `unproven: foreign-write`).
- No duplication of numbers: budgets restate bench SLO rows by id and a law keeps them equal; the product list, files and targets are data in the table, not code.

## Decisions I took (flip with one line each) and open points

1. `print-noop` wall/footprint and `dashboard-edit` are ADVISORY: no passing measurement exists (print hot 10-45 s, dashboard edit never green); everything else of the brief is blocking. The nightly rows for dashboard and print are advisory (`continue-on-error`) until their first green run; the course row blocks.
2. Dashboard no-op allows 1 executed action (the freshness worker). The SLO table `product-dashboard-noop-executed-max` says 0 and the product never reaches it (g41-g46: `2 processes: 1 internal, 1 worker`): FOR dashhot/perf-slo lane: either make the worker not an executed action or relax the SLO row; my law does not link these two on purpose.
3. The hosted macOS runner (7 GB, 3 cores) is smaller than the Mac the budgets were measured on; print and dashboard cold builds may not fit (cicover measured print near 16 GB cold). That is why rows are separate and why `SEMIO_ACCEPTANCE_RUNNER` exists. The first nightly decides whether budgets need a host-class dimension (the report already records host class and load).
4. Poll-era records (5 s grain) can only be judged as upper bounds; live runs of this command use the exact clock, so `unproven: clock-grain` appears in fixtures only.
5. Not done: a `history`/baseline comparison of acceptance reports (budgets are absolute; `repo bench compare` is for the SLO reports); `repo bench acceptance list` (the budget table is the JSON file, rendered by `renderAcceptanceListV1`, not wired to a flag to keep the catalogue small).

## Files

`C/benchci-tools/`: `data.py` (schema + budget table source), `mkfixtures.py`, `mkbatch.py` (builder, anchored edits), `rebase.sh`, `cycle.sh`, `forceapply.py` (mirror only), `helpgen.ts` + `helpgen_run.py` (help snapshot), `negative.py`, `help-snapshot.json`, `src/` (logic, runner, command, test, oracle, workflow sources with path tokens), `staged/acceptance/` (`manifest.json`, `files/`, `base/`, `stage.py`, `verify.sh`, `expected-failures.txt`). No `🗑️generated` folder was used.
