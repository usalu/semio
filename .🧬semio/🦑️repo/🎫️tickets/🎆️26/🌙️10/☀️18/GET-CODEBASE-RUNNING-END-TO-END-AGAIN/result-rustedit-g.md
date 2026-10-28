# result-rustedit-g (wave 1026g): what ONE edited line costs in `bun repo --tool=bazel run @semio-tech/print-rs:build`

Tools `C/rustedit-g-tools/` (`edit.py` byte-backup edit/revert, `run.sh` one measured run, `measure.py`+`phases.json` copy of bench2-g's harness with `marks`, `events.py` unit-cache event window, `procs.py` process-table sampler, `p0/e1.summary.json`, `e1.procs.jsonl`). Raw logs under `C/🗑️generated/rustedit-g/` (swept sometimes; numbers are here). `C` = ticket folder.

Edit: `🧰️framework/🛍️products/📓️print/🧬️schema/💡️inferences/🎨theme/🦀️.rs`, `format!("invalid colour: {value}")` -> `"invalid colour value: {value}"`, reverted byte-exact (sha `3c811062...f98d0` checked after e1: identical). Host: 32 GB / 10 cores, load 22-53, shared by ~20 agents. NO quiet host; every wall number carries that load. One void run (19:~08, host ENFILE from a peer's bun-test fd leak) is discarded.

## Verdict
The target "one-edit rebuild under 2 min for Rust" is MISSED by 20x on this host: **44 min** (2637 s) wall for the one-line edit. But the Rust compiler is NOT the main cost, and the unit cache is NOT missing: the edited crate is the only rustc that ran (0 misses of unchanged crates). 38 % of the wall is the capacity queue, 38 % is the single Bazel action of which only 51 % is rustc, the rest is fixed ingress cost per action (the nested `repo` process) and post-processing.

## Per-phase table (seconds; same command, wall clock)
| phase | p0 prime (19:35) | e1 one-line edit (20:08) | what it is |
|---|---|---|---|
| boot | 0.7 | 0.6 | |
| capacity queue | 66 | **1012** (179 `[capacity] waiting` rows, "need 7168 MB, 1-3 ahead") | run asks 7168 MB ("sized at 197-200 % of the default"), admitted "1 of 8 jobs: the host cannot carry more right now" |
| admission (source) | 11 | 22 | |
| verify (graph facts, 104k) | 265 | 72 | |
| graph | 478 (2 projects refreshed in 199 s, gen 577, caused by peer library edits) | 297 (0 refreshed; "550 projects planned by a child process") | the edit itself re-infers nothing |
| tool-setup (plan + Bazel analysis) | 26 | **250** | "this plan cannot be reused later: the planner read something that cannot be checked: spawned /usr/sbin/sysctl" -> replan every run (toolsetup-g's finding); Bazel analysis itself 7 s |
| execute = Bazel action `print-rs:build` | 811 (Bazel 820, critical path 788) | **980** (Bazel 994, critical path 967) | breakdown below |
| finish | 2 | 4 | |
| **wall** | **1659** | **2637** | |
| tree RSS peak / footprint | 3.85 GB / 3.04 GB | 3.67 GB / 1.12 GB | EXCLUDES the rustc (runs under the Bazel server, outside the tree) |
| swap used start -> max | 0.44 -> 0.44 GB | 0.43 -> 3.7 GB | rustc 4.2 GB RSS |

### Inside the one Bazel action (e1; seconds from run start, process-table samples every 2 s, `e1.procs.jsonl`)
| step | from -> to | s | share of action |
|---|---|---|---|
| nested `repo` boot + store + lane + inputs, up to `cargo fetch --locked` | 1652 -> 1784 | 132 | 13 % |
| `cargo fetch` 4 s, `cargo metadata` 9 s, `cargo build --unit-graph` 6 s (all cheap) | 1784 -> 1813 | 29 | 3 % |
| source admission, writer lease, CPU capacity, scheduler, broker (no process visible) | 1813 -> 1979 | 166 | 17 % |
| `cargo build` + **rustc `semio_framework_print` (incremental, focus crate, `-Zthreads=1`, RSS 4.24 GB)** | 1979 -> 2486 | 507 (rustc 499-500) | **52 %** |
| after rustc: consume, verify captured artifacts, record store content, Bazel finish | 2486 -> 2632 | 146 | 15 % |
Fixed ingress cost before the first compiler process = 327 s (e1) / 377 s (p0: first unit event at +1224 s, action start 847 s): 33-38 % of the action, independent of the edit.

### Unit-cache lanes (`⚡️caching/🦀️cargo/🗃️unit-cache`, lane `pea977c`, crate key `2e2c3444720d` = `semio-framework-print`)
| run | miss | hit | incremental | bypass | notes |
|---|---|---|---|---|---|
| p0 prime | 94 (310 s CPU-ish sum, one at a time) | 0 | 1 (print, 15.6 s) | 0 | the cone `libc, socket2, mio, tokio` -> `os_io`, 25 `os_store_*`, `os_kernel`, 50 `plugin_*` -> `plugin`; unchanged sources (registry mtimes 07-24/10-22, no `.rs` of the cone edited since the lane's last use at 16:11) |
| e1 edit | **0** | 0 | 1 (print, **500.4 s**) | 0 | only the edited crate compiled; no lock-wait, no slot-wait |
Requirement "cache misses of unchanged crates = 0": MET in e1, VIOLATED in p0 (94 units recompiled in the warm lane).

## Root causes above the 2-min target
1. **Admission queue (1012 s e1, 1229+ s r1).** A hot print-rs:build asks 7168 MB (200 % sizing from earlier runs) while its measured tree footprint is 1.1 GB (6.4x over-ask); the queue has "1 running, 1-3 ahead" and releases only when 7 GB are spare above the 4 GB floor. The run is then admitted with 1 job, which makes the compiler single-threaded (`-Zthreads=1`, `SEMIO_COMPILER_PARALLELISM=1`). Owners: quietfc-g (sizing; staged), ram-g. Effect on Rust: 1 front-end thread + load 30-50 on 10 cores is why print costs 500 s here; the same crate compiled non-incrementally with 10 threads took 67-87 s at 12:19 and 13:33 (events lane `p789f7a`).
2. **Fixed per-action ingress cost, 327-377 s** (nested `repo` boot with the source-closure loader, compiler store, lane acquisition, `cargo fetch/metadata/unit-graph`, admission, leases, scheduler). rust-g measured the same ingress at 27 s on an idle host for a trivial crate, so 12x of it is load/capacity-wait inside the nested run, not cargo (cargo steps total 29 s). No phase rows reach the run log: `SEMIO_CARGO_TIMING=1` (`[cargo-timing]` rows from `launchRepositoryCargoV1`) never appeared, because the action's stderr is swallowed by Bazel unless the action fails. OPEN: that blind spot is itself the reason this lane cannot split the 166 s gap (source admission / writer lease / CPU capacity / scheduler) any further.
3. **Replan every run (250 s)**: unchanged cause, sysctl read makes the Bazel plan non-reusable (toolsetup-g).
4. **Incremental focus compile is not cheap**: 500 s CPU-starved, 4.2 GB RSS for a one-string edit; the edited function embeds `owned_json_file!` (proc macro expanding the named-colors JSON), whose expansion re-runs on every incremental session and whose function is re-codegen'd. My edit was therefore NOT a cheap leaf: a pure-logic leaf would be faster. Not re-measured (gate priority forbids more runs). The previous cold-lane incremental of print was 233 s (16:11), the dep-only-changed incremental 15.6 s (p0).
5. **p0 cone of 94 recompiled unchanged crates**: cause NOT proven. Evidence: libc lib fingerprint rewritten 19:56 while its run-build-script fingerprint (16:00) is untouched; registry sources untouched; the edited tree has `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🗂️schema-document-index.json` modified 17:21 (a derive-crate `include`, plausible root of the os/plugin cone but it cannot explain `libc/mio/socket2/tokio`). Needs `CARGO_LOG=cargo::core::compiler::fingerprint=info` inside the lane on a repeat; e1 shows it does not recur without a source change.
6. **Peer-driven graph refresh (p0 199 s)**: the prime paid 2 re-inferred projects caused by peers' library edits (`host-capacity/📡️sampling/🟦️.ts`, a dashboard `project.json`), not by the edit; e1 re-inferred nothing, so a pure Rust source edit costs 0 s of inference (graph 297 s is assembling 550 projects).

## Fixes
No staged batch from this lane: no root cause above is both proven and local to a Rust file I may change. Concrete proposals, each with the law that fails today:
- quietfc-g/ram-g: size a `print-rs:build`-class run from the measured FOOTPRINT of the class (1.1 GB), not RSS or a 200 % multiplier of the largest earlier run. Law: queue wait for a 1.1 GB class on a host with 2 GB spare above the floor must be 0 (fails today: 1012 s).
- process/launcher: surface `[cargo-timing]` rows of a Bazel action in the run record (action stderr tail or a per-action timing file next to the receipt), so the 166 s gap is attributable. Law: a run with `SEMIO_CARGO_TIMING=1` prints one row per phase (fails today: 0 rows).
- process/admission: do not give a compile that sits on the critical path of the only action 1 job when other runs are idle waiting; or let the Rust action keep the grant of the run (nest-f) instead of re-queueing.
- Re-measure on a quiet host with CPU seconds (`procs.py` now records `cpu`) before touching the focus-incremental policy.

## Residual
Run r1 (rebuild after the byte-exact revert, no edit on disk) was still in the capacity queue (1234 s) at 21:15 when the coordinator's gate-priority message arrived; it is the last official run of this lane. Its numbers are appended below if it finishes while I am alive.

## r1 (revert rebuild)
CANCELLED by me (SIGINT to my own measure.py pid, 21:26) after 1848 s in the capacity queue (need 7168 MB, 1-3 ahead, load 40-47) to free the slot for gate-g; no compile ever started, no edit on disk (theme leaf sha verified identical to the backup). The revert-rebuild cost is therefore NOT measured; by construction it is an incremental recompile of the same crate plus the same fixed phases as e1.
