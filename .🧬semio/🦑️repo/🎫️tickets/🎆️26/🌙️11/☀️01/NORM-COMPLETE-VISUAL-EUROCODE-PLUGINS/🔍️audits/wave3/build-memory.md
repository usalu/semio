# W3-BUILD-MEMORY report

## Evidence (runner logs, live host; no build started, no process killed)
- Host: 15.8 GB RAM, 8.9 GB swap 100% used (0 free), 12 cores, load ~28, memory PSI full avg60 0.77. RSS of all processes sums to ~7 GB; the rest of "used" is swapped pages of idle desktop/agent processes (plasma-discover 976 MB swapped, claude 3 x ~0.4 GB, codex, spectacle). Builds do not fill swap; they wait for it.
- Runner log: 85 finished builds, 39.5 h total, 59 longer than 30 min, median 1312 s, max 5646 s. 14 pre-build memory-guard waits = 4.0 h (3 hit the 3600 s timeout, i.e. the old "swap free > 512" loop could never release). Mean at pause: 8.0 GB available, 318 MB swap free: RAM was plentiful, swap was not.
- Dominant wait is NOT the runner but repo's own capacity gate (build-guide.md line 34: admit only while swap < 25 %, load < 2/core, floor 2048 MB on 16 GB). In build logs: 24317 `[capacity] waiting` lines, sum 22.7 Ms of printed seconds (cumulative over concurrent waiters), reasons by count: swap above ceiling ~10000, "learned reservations still growing" ~5700, earlier request waiting ~4900, pressure critical ~1300. Example: build-en1996 run 16 (2157 s): admitted "1 of 8 jobs" because "the host cannot carry more right now". There is no env/override for the 25 % swap rule (grep of SEMIO_*SWAP*/PRESSURE: none), and swap can not be changed by me, so this wait persists while swap is full. Only lever: free swap (stop the idle swapped-out processes, user decision) or run fewer agents.
- Graph-plan memory: a stray Nx daemon (`nx/dist/src/daemon/server/start.js`, pid 3105397, parent systemd, 35 min old, started by a `repo run` with NX_DAEMON unset, env had SEMIO_HOST_CAPACITY_PLAN jobs=1) has VmHWM 4450 MB (this is the origin of the "graph-plan peak 4500") and VmRSS 2450 MB now, the single biggest process on the box. nx-reclaim.sh will SIGTERM it only when no build/`repo run` is live and <= 4 client sockets; with 20 agents some `repo` is nearly always live so it survives. The runner's own env (NX_DAEMON=false, W3-INFRA) prevents the runner from spawning one; the leaked daemon comes from peers or from the pre-restart runner. The repo graph memo (`SEMIO_BUILD_GRAPH_MEMO`, default on, journal-validated) is already the "graph cache"; build-guide says it refuses when peers change directories the slice reads, which happens constantly in a 20-agent tree. No separate Nx graph cache is worth adding.
- Cargo/rustc: repo drives cargo via `rustc = repo script`, `rustc-wrapper = repo`, jobs come from repo's capacity grant (1 of 8 jobs under pressure, `maximumParallelism` from compileUnitsShare) and a jobserver, `incremental` is applied 0 (cache guard log), linker is already the pinned toolchain's rust-lld (no mold/lld on PATH; no better cross-platform linker). So CARGO_BUILD_JOBS must NOT be set (it would fight the jobserver). No `[profile]` exists in Cargo.toml or .cargo/config.toml, so dev/test use cargo defaults: debug=2 (full DWARF), codegen-units=256. `rustflags = -Z threads=0` makes every rustc frontend use all 12 cores. Rustc/linker peak RSS per crate is not in any log (`/usr/bin/time` is not installed; repo learns footprints in its own estimates.json), so no number is claimed.

## Implemented (runner scope only, atomic replace, `bash -n` checked)
1. `build-runner/graph-gate.sh`: GRAPH_PEAK_MB 4500 -> 3000 (daemon-less graph child; repo sizes it at 39-194 % of 5120 MB, the 4450 MB mark was the leaked daemon), GRAPH_MARGIN_MB 2500 -> 1000, GRAPH_MIN_AVAIL_MB 5700 -> 3500, GRAPH_MAX_WAIT 3600 -> 1800. New condition: available >= 3500 MB and available + free swap >= 4000 MB (was 5700 / 7000). With swap full that means ~4 GB available instead of ~7 GB, which the host shows regularly (mean 8 GB at pause). The repo capacity gate still enforces its own floor/swap rules afterwards.
2. `build-runner/runner-next.sh`: the pre-run `avail_mb <= 4096` wait is now `RUNNER_MIN_AVAIL_MB` default 3072 (repo floor is 2048). Takes effect on the next runner restart; the gate script change is live now.
3. NX_DAEMON=false in the runner was already in place (W3-INFRA); verified `runner-next.sh` line 173.

## Proposed, deliberately NOT applied (needs a coordinator window)
Both are in `.cargo/config.toml` (repo-owned, cross-platform) but they change rustc arguments, so every unit in the content-addressed shared cache (`m/u`) is invalidated once, fleet-wide: a cold rebuild of all deps in the middle of the wave would cost more than it saves now. Apply together at a wave boundary:
```
[profile.dev]
debug = "line-tables-only"   # biggest rustc + lld RSS and link-time cut; backtraces keep line numbers
[profile.dev.build-override]
debug = false                # build scripts / proc macros
[profile.test]
debug = "line-tables-only"
```
and replace `-Z threads=0` by `-Z threads=4` in the three rustflags arrays (host, wasm32-unknown-unknown, wasm32-wasip2) to cap per-rustc frontend memory under load 28. Expected effect (typical rustc behaviour, unmeasured here): 20-40 % lower rustc/lld peak and smaller artifacts. Re-measure by reading `estimates.json` footprints (`repo capacity status`) before/after instead of /usr/bin/time. Test suites that assert config content (cargo-provenance, profile, ingress configuration tests) must be re-run when applying.

## Recommendations (system level, not mine to do)
- The 25 % swap rule is the actual starvation. Free swap by ending idle swapped-out processes (plasma-discover 976 MB, stale claude/codex sessions) or restarting the pid 3105397 Nx daemon at an idle moment (frees 2.4 GB RSS); do not run `repo` with NX_DAEMON unset.
- Restart the runner at an idle moment so item 2 and the NX_DAEMON/graph-pending changes are all live (the 31-min-old instance already has the W3-INFRA copy).
