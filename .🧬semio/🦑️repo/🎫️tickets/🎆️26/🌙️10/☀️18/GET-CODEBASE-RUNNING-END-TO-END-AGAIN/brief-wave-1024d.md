# Wave 1024d brief (10-24 15:55, weekly + 5-h usage reset to 0 %; 5-h window resets 20:50 CEST)

User (10-24 15:50): get the codebase completely running again. Focus ALL resources on three products running end to end:
1. the dashboard (`@semio-tech/repo-dashboard-rs`, binary `semio`, `repo dashboard`),
2. the architecture course (`@teaching/architecture-course`),
3. print (`@semio-tech/print`, `@semio-tech/print-rs`, `@semio-tech/presentation(-react)`, the bericht documents, `praesentation-projektetage`).
Other agents (Claude, Codex, Cursor) work on the same files at the same time: do not stop for them, keep going on your own task. Play / demonstrator / other apps are OUT of scope this wave unless your item is shared infrastructure those three need.

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.

Start state: `C/coordination.md` section "## 15:55 (10/24" (this wave) and "## 13:05 (10/24" before it, then the `C/result-*.md` files named in your own brief (the previous executor of your area: continue from its open items, do not redo verified work; re-verify on the CURRENT tree only what you depend on). ALL rules of `C/brief-wave-1023b.md` (read it fully, incl. its addenda) and the addenda of `C/brief-wave-1023c.md` (07:30 / 08:45: launch detached runs as `env -u SEMIO_SCRIPT_STALL_MS python3 C/chain-tools/detach.py <log> <cmd…>`) hold unchanged. Your result file is `C/result-<prefix>.md`, tools in `C/<prefix>-tools/`, output in `C/🗑️generated/<prefix>/`.

## New facts at 15:55
- No process of the previous wave is running (ps shows no `bun repo`, bazel, cargo or orchestrator processes); swap 0.5 GB; disk 86 GiB free. Nothing to kill, nothing to wait for. Whatever was "running" in a result file is dead: relaunch with a new run id.
- Last known verdicts: dashboard `bun repo --tool=bazel run @semio-tech/repo-dashboard-rs:build` GREEN 07:45 (cread-c); dashboard runs real tasks (dash-c 06:00); print `print:test` on Bazel GREEN (ptestbz1x rc 0); print set GREEN via `--tool=semio exec` (print-c 05:45); course full e2e 71/76 (pets project only; fix written, not verified: course-c 14:50); Nx lane: first daemon graph publication, open fault K (nx-c 14:50).
- `♻️mit-bestand/bun.lock` was regenerated 15:15 (course-c); a workspace member added without regenerating its lock breaks every run that syncs dependencies ("lockfile had changes, but lockfile is frozen").
- cargo builds now take ONE exclusive custody per build dir (lockc-c 14:21): correct but serialising; processes started 13:54-14:21 carried a custody bug (all dead now).

## Rules added this wave
- One heavy process per agent, `nice -n 10`. Check `sysctl vm.swapusage` before any heavy launch: above 16 GB used, do not launch.
- Never wait in a single tool call longer than ~9 min; no `run_in_background`, no Monitor. Static work (next known error, the test, the result file) while you wait.
- If you are blocked > 30 min by somebody else's process, write it at the TOP of your result file (pid + lstart + command + lease line); the coordinator reads result files every ~20-30 min. Kill only pids you started (verify lstart).
- A fix in shared infrastructure that another lane needs: write it at the top of your result file as `FOR <prefix>: …` so the coordinator routes it.
- Update your result file at least every 20 min (newest first). Final message ≤ 8 lines pointing to your result file. Do NOT close/reopen the ticket, do NOT spawn sub-agents, never a modifying git command.
- Before you finish: remove your `[DEBUG]` lines, kill every process you started (child first, verify lstart) unless it is a declared handover, and list every changed file in your result file.

## Lanes (Sonnet 5.5 high executors; Haiku 5.5 high read-only audits)
| prefix | product | item | continue from |
|---|---|---|---|
| course-d | course | full course e2e on Bazel via `C/e2e-orch-tools` (cold, warm, e2e, probe) GREEN 76/76, deterministic, no retries; pets 200 % text fix verify; remove [DEBUG] in pets walk | result-course-c, report-e2e-orch-e2e-1023-course17 |
| cwarm-d | course | warm run = cache hits: the generate actions that re-execute on every run (ui-rs, ui-contract-rs, assets, ui-styling-tokens, framework-actor-rs, …) made hermetic at the source; prove cold→warm 0 re-executions | result-course-c (06:07 item), coordination 06:07 |
| nx-d | all 3 | Nx lane (`--tool=nx`) for course typecheck/build, dashboard build, print test: fault K (`Cannot find project 'workspace'`) and the next faults; remove nx-c's [DEBUG] hunk in `nx@23.2.0.patch` | result-nx-c |
| dash-d | dashboard | runtime e2e on the current tree: official build (Bazel, then default tool) re-verified, `installed.json` route, `repo dashboard` launcher → `daemon status` → headless pty smoke EN + DE → stop; `commands --check` for the S catalogue | result-dash-c, result-cread-c |
| dtest-d | dashboard | every test target of the dashboard closure (the 24 `repo-*-rs` projects cread-c touched + dashboard bun suites) GREEN on Bazel; the red intent test 'neutral build orchestration loaded helper identity refuses stale imports…' | result-dash-c, result-cread-c |
| print-d | print | Bazel verdicts: `print-rs:test`, `print:check`, `print:build` (flyer), presentation(-react):test, bericht builds + test, `praesentation-projektetage` build + headless dev smoke (6050) | result-print-c, result-print-b |
| snap-d | print/framework | regression of print-c's framework change (`🏪️store/📦️codec/🪶️snapshot-capability/🪆️workspace-receiving/🦀️.rs`: validate_decoding/encoding under `receiving_allocation!`): run the other handwritten sqlite-snapshot plugins' tests, fix fallout at the source | result-print-c |
| lockc-d | infra (dashboard + print-rs cargo) | lockc-c open items: contended custody proof via real `bun repo cargo`, re-run cargo-build-lease / resource-leases / overlap suites, hoist lease above the shared writer in testing/cargo + exact, lane-ify warm component drivers so cargo stops serialising on `m/b/p` | result-lockc-c |
| churn-d | infra (all 3) | builds of the three products must survive live peer edits: "Current shared Source-owner generation changed during consumption" (print:build-flyer, ts-c c3), "Stale loaded Source revision reader; restart required" (dashboard), "outdated implementation" exit 75: one snapshot + bounded re-observe policy, laws | result-churn-c, result-ts-c, result-dash-c |
| deps-d | infra (all 3) | every `bun.lock` in sync with its workspace members (the 3 lock files jack-c listed as OPEN 3 + any other); a law/preflight that names the stale lock and the command to fix it before a frozen install fails deep inside a build | result-jack-c (OPEN 3), result-dl-c 15:13 |
| audit-d (Haiku) | all 3 | `C/status-1024d.md`: per product the last verdict per target, open items, blockers, owners (graph-free, read-only) | — |
| map-d (Haiku) | all 3 | `C/e2e-map-1024d.md`: the targets that define "end to end" for each product (read the `📋️project.json` files directly, graph-free) with dependency closure and test targets | — |
| hyg-d (Haiku) | all 3 | `C/audit-hygiene-1024d.md`: `[DEBUG]` lines in tracked sources with file:line + owner lane, leftover processes, cache image fill (`.cache/b/g`, products, rust), disk | — |

Ports: course 6061/6063/8791/8793 (course-d only), projektetage 6050 (print-d only), dashboard (dash-d picks a free one and writes it in its result file).
