# R2 Slice V-1: Battle Tests

## Final confirmation (2026-10-08 12:54, newest): ALL GREEN, no remaining failure

Debug `semio` rebuilt from the current tree (private build dir, 11:39). R1, R2, R3 verified fixed. Every suite run directly and once through its Nx target.

| Suite | Directly | Via Nx (`@semio-tech/repo-dashboard-rs:test-<suite>`) |
| --- | --- | --- |
| Coverage | **9 pass, 0 fail** (16,309 ids; 3,097 dry-run, 0 differences; graph errors empty; no fixture project) | **9 pass, 0 fail** (21 min 56 s) |
| CLI journeys | **11 pass, 0 fail** | 11 pass, 0 fail |
| PTY journeys | **9 pass, 0 fail** | 9 pass, 0 fail (4 min 3 s with the CLI part) |
| Load, CLI | **4 pass, 0 fail** | 4 pass, 0 fail |
| Load, PTY | **3 pass, 0 fail** (+1 ignored exploration aid): sixteen views, 17th view handled, 50k launcher typing, 10 MiB burst | 3 pass, 0 fail (4 min 55 s with the CLI part) |
| Smoke (`SEMIO_DASHBOARD_INSTANCE=v1-smoke`) | **2 pass, 0 fail**: finite real Nx test exits 0; quiz dev server `--detach --wait-ready` → HTTP 200 on the first GET → stop → exited → port closed | 2 pass, 0 fail (6 min 55 s) |

Smoke instance stopped by the suite; verified afterwards: no listener on 6061/8791, no `semio`, `vite`, quiz, proctor or idle process of mine remains.

---

(Previous round below.)


## Final round (2026-10-08, newest): rebuilt debug binary (10:41 tree), every suite re-run end to end, Nx targets proven

Run on the debug `semio` rebuilt from the current tree. F1 is fixed: suites now run directly from a normal shell (no WMI helper needed). The fixture project is named `journey-fixture` (never `workspace`).

| Suite | Result |
| --- | --- |
| Coverage (`test-coverage`, also via Nx: 21 min) | **8 pass, 1 fail**. 16,291 ids; 3,108 dry-run (2,696 declared + 412 plain sample): **0 differences**; ports unique/documented; `commands --check` 0 problems; graph `errors` empty. |
| CLI journeys | **11 pass, 0 fail** |
| PTY journeys (direct) | **9 pass, 0 fail** (also run via `test-journeys` Nx: 7 pass / 2 fail on a binary 1 hour older and two test bugs of mine since fixed: `stop 1` place selection, spinner glyph volatility) |
| Load, CLI (`test-load` Nx) | **4 pass, 0 fail**: 128 sessions in 7 s, 129th handled, no idle process left; 50,027-command check 2.6 s, searches 1.8-2.4 s; 10.2 MiB burst 5.5 s with other clients answered in 73-107 ms |
| Load, PTY (`test-load` Nx) | **2 pass, 1 fail**: launcher over 50k repaints per key in about 10-45 ms; view stays connected through a 10 MiB burst; **sixteen views fail** (see R1) |
| Smoke via `SEMIO_DASHBOARD_INSTANCE=v1-smoke` (`test-smoke` Nx) | **1 pass, 1 fail**: finite real Nx target (`repo-dashboard-rs:test -- execution`) exits 0 with passing tests; dev server `@teaching/architecture-quiz:dev` `--detach --wait-ready` → HTTP 200 → stop → exited → port closed all pass, but see R2. Instance daemon stopped; verified: no semio/vite/idle process and no listener on 6061/8791 remain. |

### Remaining failures

- **R1 (A-2 / A-3):** the 17th view is not refused and is misleading: it connects, shows `0 running` (the running ticker is invisible) and the status line says `the daemon holds as many tasks as it allows` (a task-limit text for a view limit). `sixteen_views_attach_to_one_workspace_and_see_the_same_output`.
- **R2 (M-1a quiz declaration / A-2 ready matcher):** `--wait-ready` returns `http://127.0.0.1:6061` minutes before the server listens: the quiz stack script prints `[stack] waiting for the site at http://127.0.0.1:6061` before Vite starts and the matcher takes that line. First GET after `--wait-ready` is refused for up to ~4 minutes (build). Declare `ready` against Vite's own line (printed) or have the stack script print the URL only when the site answers.
- **R3 (N-1):** two plain `project.json` fixtures are still Nx projects: `@fixture/a` and `@fixture/b` under `📚️library/⚡️caching/🧫️fixtures/import-edges/`. `.nxignore` covers only `**/🧫️fixtures/**/📋️project.json`; add `**/🧫️fixtures/**/project.json`. Fails coverage test "published project graph has no errors and no fixture manifest is a project".
- Notes (not failures): `semio tasks` no longer prints or accepts a 1-based place (my tests select by session id now); the status-line notice of a 17th view is truncated to `the daemon …` at 120 columns.

### Corrections of earlier F-findings by this run

F1, F2, F3, F4, F5, F6, F7, F8, F9, F10 (`SEMIO_APP_ID`), F11, F12, F13, F14 (graph healthy: 1,308 projects, 0 errors), F15, F16 (instance isolation works: instance-less client sees `daemon not running` while `v1-smoke` runs) verified fixed on this binary, except where R1-R3 say otherwise. Two oracle gaps fixed on my side (`{port}` token, `{workspace}` expansion). Process hygiene: four `conhost --headless` processes of my own earliest PTY experiments (03:39-03:44) had spun at 100% CPU for six hours and slowed every measurement; I ended them. Other agents' hung `python3 -` processes (4, parents alive) were left alone.

---

(Earlier round below.)


Date 2026-10-08. Newest results on top. Binary under test: debug `semio` built 05:0x from the working tree
(`target-fleet-v1`, includes A-1, A-2, A-3 as of then). Host: Windows 11, ConPTY, 16 cores, about 24 concurrent cargo processes of other slices.
`D` = `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`.

## 1. Result table (exact counts)

| Suite | Where | Result |
| --- | --- | --- |
| Coverage, real monorepo | `D/🧪️tests/🗺️coverage/🟦️.ts` (+ oracle `🔮️oracle/🟦️.ts`, Ajv, feature `🥒️.feature`) | **6 pass, 2 fail** (8 tests) |
| CLI journeys, fixture workspace, real daemon | `D/🧪️tests/🧭️journeys/🟦️.ts` | **11 pass, 0 fail** (9 journeys + 2 traceability) |
| PTY journeys (portable-pty + vt100, second oracle pyte) | `D/🧪️tests/🧭️journeys/🦀️.rs` | **6 pass, 3 fail** (9 tests incl. 1 fixture sanity test) |
| Load, CLI | `D/🧪️tests/🧭️journeys/🏋️load/🟦️.ts` | **4 pass, 0 fail** (128 sessions, 50k search, 10 MiB burst, traceability) |
| Load, PTY | `D/🧪️tests/🧭️journeys/🏋️load/🦀️.rs` | **2 pass, 1 fail** (+1 ignored exploration aid) |
| Real-workspace smoke | `D/🧪️tests/🧭️journeys/💨️smoke/🟦️.ts` | **0 pass, 2 fail by design**: blocked, see F16 |
| One-time launch proof | `T/r2-v1-launch-coverage.ts` | selections 994 of 1073 resolve as the launch rows did; M-2 mapping 89 of 101 commands resolve |

Measured (debug build): 128 detached sessions started in 8.1 s, listing 128 tasks under 2 s, 129th handled, no idle process left after `daemon stop`;
`commands --check` over 50,027 commands 2.2 s, word search 2.3 s, rare prefix 1.8 s (budget 3 s); 10.2 MiB burst through an attached client in 5.4 s with other clients answered in 60-87 ms;
launcher over 50,027 commands: key latencies 9, 44, 43, 41 ms (budget 1.5 s); a view stays `connected` while a task prints 10 MiB.
Oracle vs registry: 15,9xx oracle ids (graph targets, 155 catalog variants, tools, compounds, groups, 1,080 open ticket commands) are all listed, none phantom (repo actions excluded); 1,769 of 10,052 ids dry-run
(1,354 declared/special + stable sample of 415 plain targets; `SEMIO_COVERAGE_FULL=1` does all, hours in debug).

## 2. Failures, owner, evidence

| Id | Owner | Defect | Evidence |
| --- | --- | --- | --- |
| F1 P1 | A-2 | `semio run`, `daemon start` and the TUI cannot start the daemon when the parent process sits in a job object without breakaway: `CreateProcessW detached failed: Access is denied (os error 5)`. `spawn_detached` always passes `CREATE_BREAKAWAY_FROM_JOB` (`⌨️tui/🚇️pty/🦀️.rs:553`). Happens in this agent shell, under every `cargo test` child (cargo's job), likely CI/containers. Retry without the flag on ERROR_ACCESS_DENIED. | Python `creationflags=0x01000000` fails, `0x08` works; the same call works from a WMI-started process. All V-1 runs go through `T/r2-v1-oob.sh`. |
| F2 P1 | A-2 | `thread 'dashboard connection' panicked at 📎️connection/🦀️.rs:156:42: attempt to subtract with overflow` (`outbound.bytes -= frame.len()`) on every connection in debug builds; inside the TUI the panic text is painted over the screen; release wraps and breaks backpressure accounting. | every run of `semio tasks` etc. before the last rebuild; no longer seen in the 05:xx binary (UNVERIFIED if fixed or only not hit). |
| F3 | A-2 / A-3 | The text `workspace dashboard daemon ready at pid N` is written into the TUI pane (stale cell at pane row 3-6, overlays task output). Fails scenario "incremental screen equals a full repaint". | `the_incremental_screen_equals_a_full_repaint`: row 6 incremental `┃workspace dashboard daemon ready at pid 8056 …` vs repainted blank. |
| F4 | A-3 / T-B | Ctrl+B s on a freshly attached view leaves the Tasks window on top; the restored task output appears only after a further Tab. Detach/attach and two-view scenarios fail on it (replay itself is correct: gapless ticks 9..33 after Tab, scenario "reattached view replays a gapless run" passes with Tab). | `detaching_leaves_the_task_running…`, `two_views_of_one_workspace…` (screen dumps in the failure text). |
| F5 | T-B / A-3 | Incrementally painted screens carry stale cells from earlier frames (`No tasks┏━━━┓ne with New task`, `2027 commandsmands`, `ery`). Two independent emulators (vt100 and pyte) agree on the raw byte stream, so it is real. | transcript `semio-explore.raw` fed to pyte; failing repaint test. |
| F6 | A-2 | `Ctrl+B c` on a task started through the npm shim `bun.cmd` (first `bun` on this PATH) shows `^CTerminate batch job (Y/N)?`; the task ends only after the grace as `exit 137`. Interrupt of batch-file shims is not handled. | `a_task_is_started…` screen dump (passes only because the end shows `exit 137`). |
| F7 | A-2 | Argument quoting for `cmd /c "<string with spaces>"`: a tool `["cmd","/c","ping -n 3600 127.0.0.1 >nul"]` fails with `'"ping -n 3600 127.0.0.1 >nul' is not recognized`. | `semio logs 1` of the task in a fixture workspace (fixture now uses bun). |
| F8 | A-1 / A-2 | With 16 views attached, `semio tasks` (a 17th client) lists the running ticker as `interrupted` (journal fallback, pid null), `daemon status` says `0 active tasks`, while the journal and the process say running. The refusal at the view limit is silent. | `sixteen_views_attach…` failure text. |
| F9 | A-1 / M-2 | `--param steady` and `--param hub` (bare flag names) are rejected: `--param "steady" is not key=value`. M-2's mapping and L-2 docs use that spelling for 7 commands; `--param steady=true` works. Accept bare flags or change the docs. | `T/🗑️generated/v1-launch-proof.log`. |
| F10 | A-1 | `SEMIO_APP` is not set for any of the 144 `playground:*` launches checked; the removed launch rows set it (`s.cad.cad@1/*#editor` etc., the catalog still has `app`). Decide: drop deliberately or restore. Fails the dry-run test (144 differences, only this kind). | `[coverage] differences by kind {"env SEMIO_APP":144}`. |
| F11 | A-1 | Playground native entry differs from the launch rows: `nx run @semio-tech/framework-os-dev:run-<v>-native-dev` instead of `framework-renderer-wgpu:native -- <v>`; react/wasm use `dev-<v>-react-dev`. Accepted as equivalent entry targets in the suite (159 notes), flagged because both targets must exist (see F14). | proof log, 27 selections. |
| F12 | M-1a / M-2 | Ready port 6300 is claimed by `playground:stdio-epw` and `tool:workspace/os-mcp-http`; they cannot run together. Other shared ports are documented in `D/🧫️fixtures/🗺️coverage/🔣️.json` (6010 storybook, 6029, 6033, 6061 quiz, 6274 inspector, 8787 hub). | coverage test 8. |
| F13 | A-1 | `--env TEACHING_ARCHITECTURE_QUIZ_PORT=6199` changes the environment but the declared ready port stays 6061, so `--wait-ready` could never fire on the overridden port. | `semio run @teaching/architecture-quiz:dev --env … --dry-run`. |
| F14 | Nx graph / L-1 / env | The published graph `.nx/workspace-data/project-graph.json` shrank from 154 MB to 47 MB (680 projects) around 04:50; `framework-os-dev` has 0 targets, so the 181 inferred `dev-<v>-react-dev` entry targets (and 3 retired rows such as `…aec-building-rust:describe`) are unknown. Coverage test 6 reports "181 launches name an Nx target that is not in the project graph". Re-run when the graph is healthy (a failing Nx inference plugin is the likely cause: UNVERIFIED). | coverage log. |
| F15 | A-1 | Each `semio run … --dry-run` takes 1.8-6 s in the debug build (snapshot of 3 MB read and registry built per call); the exhaustive 15k-id proof needs a batch verb (e.g. `semio commands --resolve --json`, one process). Pets `port` parameter is now `stories-port`; the mapping/docs still say `port` (2 selections). | speed probe: 64 ids, 8-wide, 19 s. |
| F16 | A-2 | **No daemon isolation for the real workspace.** The pipe/socket and cache are derived from the workspace path; `SEMIO_DASHBOARD_RUNTIME_DIR` only affects Unix. A junction mirror cannot work (Nx inference plugins require real ancestry, the tooling patch check rejects junctions). Request: `SEMIO_DASHBOARD_INSTANCE=<name>` mixed into `workspace_key` and `dashboard_cache_dir`, documented in the usage text. The smoke suite asserts that text in `semio --help` and otherwise fails fast without touching anything. | `💨️smoke/🟦️.ts`. |

## 3. Incident (disclosed)

While verifying the migration selections, an early version of the coverage test appended `--dry-run` after `--`, so 20 selections with extra arguments really ran:
a debug `semio` daemon was started on the real workspace root and about 6 real `nx run …:test` tasks (`framework-plugin`, `framework-os-kernel`, `stdio-semio-rs`) ran for roughly 15 minutes
before I found them (`semio tasks`, process list) and ended them with `semio daemon stop`; no process of them survives (checked). Fixed by `runArguments(selection, flags)` (flags before `--`) and by a guard
in `🧰️support/🟦️.ts`: `run|stop|restart|kill|daemon` against the real repository without `--dry-run` before `--` throws. Those test runs may have written Nx/cargo caches; nothing else.

## 4. What was built (all new, in my owned paths)

- Fixture workspace `D/🧫️fixtures/🧭️journeys/🏗️workspace/` (`📋️project.json` with tools words, serve, serve-a/b, idle, ticker, burst, fail, env, title, resize, probe; compounds pair, stack; one `📜️script.ts` with the behaviours); ready ports are sentinels replaced by free ports per copy.
- TS support `🧭️journeys/🧰️support/🟦️.ts`; Rust driver `🖥️terminal/🦀️.rs` (portable-pty 0.9, vt100 0.16; answers the ConPTY `ESC[6n` startup query, without which conhost never lets a child paint), `🏗️workspace/🦀️.rs`, `🧰️steps/🦀️.rs`; standalone crate `🧭️journeys/📦️packages/🦀️rust/Cargo.toml` (own `[workspace]`, not a member of the root workspace; tests `journeys` and `load`).
- Feature files: `🗺️coverage/🥒️.feature` (7 scenarios), `🧭️journeys/🥒️.feature` (9 @cli, 10 @pty), `🏋️load/🥒️.feature` (3 @cli, 3 @pty), `💨️smoke/🥒️.feature` (2). Traceability tests assert every scenario title has a test of that name (TS title, Rust `fn slug`).
- Oracle: independent TS resolver `🗺️coverage/🔮️oracle/🟦️.ts` (graph, manifests, catalog, tickets, §2.2 semantics), Ajv 2020 on the registry schema, vt100 and pyte for screens, Bun `fetch` for servers.
- Nx: targets `test-coverage`, `test-journeys`, `test-load`, `test-smoke` in `D/📦️packages/🦀️rust/📋️project.json`; `📜️script.ts test <suite>` builds the debug binary, runs the bun suites and runs the Rust test executables directly (never under `cargo test`, see F1).
- Per coordinator note: the 1073-row launch fixture was removed from the repo; the permanent coverage suite asserts properties of the current sources only (`🧫️fixtures/🗺️coverage/🔣️.json` holds only the documented shared ports). The one-time proof lives in `T/r2-v1-launch-coverage.ts` with `T/r2-v1-launch-selections.json` (M-1a derivation) and `T/r2-m2-mapping.json`.
- Not done: `🔮️oracles/🔣️.json` of the dashboard does not list the new oracles (not my file); the Rust crate is outside the root Cargo workspace, so a root policy gate that enumerates every `Cargo.toml` may need to know it (UNVERIFIED).

## 5. How to run (Windows needs a parent without a restrictive job until F1 is fixed)

`bun nx run @semio-tech/repo-dashboard-rs:test-coverage|test-journeys|test-load|test-smoke`. Helpers in `T`: `r2-v1-oob.sh` (run outside the harness job), `r2-v1-bun.sh`, `r2-v1-jt.sh` (`V1_TARGET=journeys|load`), `r2-v1-build.sh`.
Environment: `SEMIO_TEST_CLI`, `SEMIO_COVERAGE_FULL=1`, `SEMIO_COVERAGE_SAMPLE`, `SEMIO_SEARCH_BUDGET_MS`, `SEMIO_TYPE_BUDGET_MS`, `SEMIO_KEEP_WORKSPACES`.

## 6. Launch proof detail (one time, not in the repo)

Selections: 994 of 1073 resolve with the cmd/args/env/ready the rows stood for (accepted equivalences: 27 playground entry targets, 76 extra environment variables, 1 extra ready). The 79 others are F10 (SEMIO_APP), F11 (native entry), F14 (3 retired targets), pets `port` (F15), generation3d viewer ready path and example env (the catalog fields `hub`, `dataDir`, `viewerPath` are read, `SEMIO_DEFAULT_EXAMPLE` is not set for examples).
Mapping: 108 entries (79 `.claude`, 4 compounds, 25 deviation items) give 101 distinct commands; 89 resolve; the 12 others are 7 bare-flag spellings (F9) and 5 shorthand `a|b` ids that are not real commands. 9 entries are marked intentionally dropped, 13 declared now.
