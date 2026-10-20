# result-e2e-orch: one orchestrated, detachable e2e driver for course, play and demo on Bazel (wave 1020e, executor e2e-orch)

**READY.** Built and verified statically plus `--phase preflight` only. No cold, warm or e2e run was started (they refuse without `--go` / `E2E_GO=1`). Nothing outside `C/` was written; no repo file touched, no git write, no sub-agent.

`C` = `/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`. Tool folder `T` = `C/e2e-orch-tools/`.

## 1. Files (`T`)

| file | role |
|---|---|
| `run.sh` | POSIX launcher: `nice -n 10 python3 orch.py "$@"` (children inherit the niceness). On Windows call `python orch.py ...` directly |
| `orch.py` | the orchestrator (Python 3.9+, standard library only): preflight, per-product pipelines in parallel, monitor, snapshots, report, `status`, `cancel` |
| `plan.json` + `plan.schema.json` | schema-first plan: thresholds, pins (Playwright 1248/1013, Tectonic 0.16.9), per product ports, cargo lane, applied-set markers, and the step lists for cold, warm (= cold steps with a cache assertion), e2e, probe |
| `vectors.json` | language-agnostic vectors (Bazel summary lines, verdicts, slot/wait lines, slot formula, counter delta) |
| `selftest.py` | 102 assertions: vectors; an independent awk implementation of the summary parser; Ajv (third party, via bun) on the schema; a fake-plan engine run (cold/warm verdicts, group skipping, resume, report, SIGTERM cancel, lock release, no orphan child) |

Built on the existing drivers: the e2e phase runs `C/e2e-tools/run-e2e.sh <product> --tool=bazel --skip-activate` (course: `--topology=dev`), probes run `play-bzl-tools/cold-run.sh probe`, `demo-bzl-tools/cold-run.sh probe`. Cold/warm call `bun repo --tool=bazel run <target>` with exactly the targets and flags of `play-bzl-tools/cold-run.sh`, `demo-bzl-tools/cold-run.sh` (activate, report, deck) and `course-bzl-tools/course-run.sh` (the `list`). They are not invoked as scripts for cold/warm because their preflights refuse while any other cargo/bazel/vite process runs, which would serialise the three products.

## 2. What it does

`run.sh <course|play|demo|all> [--phase preflight|cold|warm|e2e|probe|report|all[,..]] [--go] [--run-id ID]`

1. **preflight** (light, no GO needed, about 5 s): python3, bun (+ packageManager mismatch warning), node, git; pinned Bazel binary (`bazel --version`); Playwright browsers read from the installed `playwright-core/browsers.json`, must equal the plan pins 1248/1013 and be `INSTALLATION_COMPLETE` (a missing browser is a FAIL that names the download needing the user's go; it never downloads); Tectonic 0.16.9: pin file vs plan, archive hash recorded vs pinned, extracted binary sha256 re-computed, populated TeX bundle; ports 6033/6029/6061/6063/8791/8793 free; disk (60 GB cold, 15 GB warm/e2e); available memory; load (WARN above 2 per core, FAIL above 10); capacity budget (Bazel build slots from the adapter's own formula vs the number of products, `bun repo capacity status` line); foreign heavy processes (INFO only, the governor and the slots arbitrate); `SEMIO_HOST_CAPACITY` override and the legacy wasm mutex as WARN; markers that the staged sets the warm assertions need are in the tree (play E1, demo activate cached, pg-chain component-cache test, course x-site build cached). Runs implicitly before any heavy phase unless `--skip-preflight`; a FAIL stops the run before it starts anything.
2. **cold, warm, e2e (+ opt-in probe)**: one pipeline thread per product (`--parallel`, default min(products, derived build slots) = 3 here; start staggered by 20 s x order so three Bazel servers do not start at once; longest product first). Steps are child process groups with per-step timeout, run deadline (14 h) and cancellation. Each product gets a private cargo lane (`SEMIO_CARGO_LANE` = `eplay`/`edemo`/`ecourse`, valid 1 to 12 lowercase chars), English output (`SEMIO_LANGUAGE=en`), and a stable `E2E_STAMP`. Bazel build slots are claimed by the sessions themselves (`s0..sN`, fair queue, service slots `d0..` for serve); the orchestrator records the slot and every waiting line per step.
3. **Warm assertion**: after cold, the same targets again. The Bazel summary lines of the log are summed (`INFO: 1 process: 1 internal.`; runners `disk cache hit`/`remote cache hit`/`internal` are not spawns, everything else is). Verdict: exit code 0 and spawns <= allowed. Allowed: course 0 per step; play and demo activate 2 (the two documented uncached probes `deps-wasm-opt`, `generator-inputs`); demo report/deck 3 (`deps-tectonic`, `deps-tex`, `fonts` probes); `--warm-slack N` loosens. No summary line is a WARN (not provable), exceeding the step's expected time is a WARN (course/print 600 s, activate 900 s). Cargo `Compiling` and Vite `Bundling` lines in a warm log are listed as evidence. A cold step that failed makes its warm step SKIP; a failed group skips its dependants (e2e needs `activate` or `core`); independent groups (demo `print`) continue.
4. **e2e**: `run-e2e.sh` per product (serve detached, port polled, Playwright, serve stopped); afterwards the product's ports are re-probed and a still-answering port turns the step into WARN with the port named (never killed: the orchestrator does not own it). Playwright `N passed/failed` is parsed.
5. **Monitor** (thread): every 60 s a heartbeat line (load, per-step elapsed time and last log line), resource extremes, `CANCEL` file check, abort if free disk < 20 GB, deadline, snapshots of `bun repo capacity status`, `leases status`, `--tool=bazel cache status --tasks` at start, every 30 min and at end (background, raw files under the run directory, counters diffed in the report).
6. **Report**: `C/report-e2e-orch-<run-id>.md`, rewritten at every heartbeat (60 s) and at the end of an invocation: prerequisites table, product x phase matrix with times, per-product step tables (rc, time, processes, spawns, cache hits, internal, slot, waits), warm assertion table, queueing lines, counter deltas, resource extremes, failures and warnings with the last 25 log lines and log path, re-run hint. Raw logs: `C/🗑️generated/e2e-orch/<run-id>/<product>/<phase>-<n>-<step>.log` (tool output, delete at ticket close); `state.json` is the machine record.

Detachable and safe: no tty, `SIGHUP` ignored, own session when launched through `detach.py`, one mutating orchestrator at a time (`ORCH.lock` with pid), `SIGTERM`/`SIGINT`/`CANCEL` file stops every child group (TERM, 30 s grace, KILL), lock released, report written. No process owned by an agent: launch through `detach.py` and only poll files.

## 3. Verification done

- `python3 selftest.py`: **102 assertions, 0 failures**, about 15 s, no `bun repo`/Bazel/cargo (fake plan runs `sh -c printf` and `sleep`). Found and fixed two real defects on the way (singular `1 process:` was not parsed by Python or awk; the log header line echoing the command was parsed as a summary).
- `sh -n run.sh`, `py_compile`, `--dry-run all --phase all` prints the 25 planned commands (above) with timeouts and warm limits.
- **Real `--phase preflight`** (twice, once through `detach.py`: the orchestrator survived the launcher and exited with its report): verdict WARN only for bun 1.3.14 vs packageManager 1.4.2 and load 48.9 on 10 cores. OK: bazel 8.4.2 binary, Playwright chromium + headless shell 1248 and ffmpeg 1013, Tectonic 0.16.9 aarch64-apple-darwin binary hash matches the record, 2 TeX bundles, 231 GB free, 13.3 GB available of 32 GB, 3 build slots for 3 products, all six ports free, all four applied markers present, `bun repo --help` exit 0, `bun repo capacity status` answers (2 running, 3 waiting, load 48).
- Not verified (needs a real run): the exact slot/wait text printed by `bun repo --tool=bazel run` (regexes `slot [sd]N` and `slots are in use`/`Waiting for access`/`[capacity]...waiting` follow `course-run.sh` and the conc-bazel text; if they miss, only the slot column is empty); that Bazel actions inherit `SEMIO_CARGO_LANE` (otherwise the unit-cache lane resolver falls back to the default lane with overflow lanes `d1..d31`, still correct); `cache status --tasks` output format and cost (plain `cache status` took over 2 min here: snapshots run in the background with a 900 s timeout, the end snapshot is waited for at most 10 min, the counter diff is best effort on raw text or JSON); the warm allowances (2/3) are the documented probe counts and may need `--warm-slack` on first contact; Windows/Linux (stdlib only, `ps`/`os.getloadavg` guarded, never run).

## 4. Decisions to know

- Course `publish` is NOT in the plan: `course-run.sh` includes it, but its cache class is `side-effecting/publishes-remote`; the course steps are proctor build, typecheck, test (`-- quick`), build, check, proctor test, then `test-e2e --topology dev` (`--course-topology rehearsal|both` adds `--release`). `--course-mode dev` switches the two `--configuration=release` steps.
- Course probe (`course-run.sh edit`) is left out for the same `publish` reason; play and demo probes exist and are opt-in (`--phase probe`: they edit one source file, run, revert, re-run).
- Demo includes the print chain (report compact, report all, deck) as group `print`, because Tectonic is a stated prerequisite; its failure does not skip the demonstrator e2e.
- `test-e2e` is `cache:false` by design; it is never asserted as a hit. Its in-graph re-activation is a warm hit and shows in its Bazel line.

## 5. Exact launch commands (after the coordinator's E2E GO)

```
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
ID=e2e-1021a    # any unique id; the same id continues the same run

# 0. light, any time (no GO): prerequisites + plan
sh "$C/e2e-orch-tools/run.sh" all --phase preflight --run-id "$ID-pre"
sh "$C/e2e-orch-tools/run.sh" all --dry-run

# 1. everything: preflight -> cold -> warm -> e2e -> report, three products in parallel, detached
python3 "$C/chain-tools/detach.py" "$C/🗑️generated/e2e-orch-$ID.log" sh "$C/e2e-orch-tools/run.sh" all --go --run-id "$ID"

# 2. or phase by phase (same id; steps that passed are skipped, --rerun repeats them)
python3 "$C/chain-tools/detach.py" "$C/🗑️generated/e2e-orch-$ID-cold.log" sh "$C/e2e-orch-tools/run.sh" all --go --run-id "$ID" --phase cold
python3 "$C/chain-tools/detach.py" "$C/🗑️generated/e2e-orch-$ID-warm.log" sh "$C/e2e-orch-tools/run.sh" all --go --run-id "$ID" --phase warm
python3 "$C/chain-tools/detach.py" "$C/🗑️generated/e2e-orch-$ID-e2e.log"  sh "$C/e2e-orch-tools/run.sh" all --go --run-id "$ID" --phase e2e

# one product only, e.g. the short course first as a canary
python3 "$C/chain-tools/detach.py" "$C/🗑️generated/e2e-orch-$ID-course.log" sh "$C/e2e-orch-tools/run.sh" course --go --run-id "$ID-course"

# monitor (light) and stop
sh "$C/e2e-orch-tools/run.sh" status --run-id "$ID"
tail -n 20 "$C/🗑️generated/e2e-orch-$ID.log"
sh "$C/e2e-orch-tools/run.sh" cancel --run-id "$ID"

# report (also written automatically at the end): C/report-e2e-orch-<ID>.md
sh "$C/e2e-orch-tools/run.sh" all --phase report --run-id "$ID"
```
Options: `--parallel N`, `--warm-slack N`, `--course-topology dev|rehearsal|both`, `--course-mode release|dev`, `--no-stagger`, `--no-snapshots`, `--skip-preflight`, `--force-lock`. Environment: `E2E_GO=1` instead of `--go`, `E2E_SERVE_TIMEOUT` (default 3600 s), `E2E_MIN_FREE_GB` for run-e2e, `SEMIO_BAZEL_SERVERS` to change the slot count the preflight assumes.

Expected wall time on this host: cold dominated by play (about 3 h, 68 components); demo about 45 min plus print; course 20 to 40 min plus the gate; warm minutes; e2e 5 to 30 min per product; run deadline 14 h. Keep the rest of the fleet off wasm/cargo during the cold phase (the preflight only counts them).

## 6. Before the GO (coordinator)

The preflight proves tools, pins, ports and four applied markers. It does not prove that `show projects` / `tools setup bazel` are green or that the remaining staged sets (course x-paths/x-release/x-proctor, play/demo manifests beyond the cache flag, hub) are applied; a missing set shows up as a FAIL/WARN in the matrix, not as a preflight stop. The dry-run (`--dry-run`) lists exactly what will run.
