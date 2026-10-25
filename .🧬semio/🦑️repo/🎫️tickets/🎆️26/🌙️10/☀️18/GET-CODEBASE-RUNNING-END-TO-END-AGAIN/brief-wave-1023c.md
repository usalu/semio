# Wave 1023c brief (10-23 22:15, weekly + 5-h usage reset to 0 %; 5-h window resets 01:00 CEST)

User (10-23 22:00, same request again): get the codebase completely running again. The dashboard, the architecture course (`@teaching/architecture-course`), semio-tech play (`@semio-tech/semio-tech-play`), the mit-bestand demonstrator (`@semio-tech/mit-bestand-demonstrator`) etc. must run end to end. Other agents (Claude, Codex, Cursor) work on the same files: do not stop for them, keep going on your own task.

`C` = `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.

Start state: `C/coordination.md` section "## 02:10 (10/23" to the end, then the `C/result-*.md` files named in your own brief (the previous executor of your area; continue from its open items, do not redo verified work). ALL rules of `C/brief-wave-1023b.md` (incl. its addenda 04:55, 07:50, 10:20) hold unchanged: read it fully before you start. Your result file is `C/result-<your prefix>.md`, tools in `C/<prefix>-tools/`, output in `C/🗑️generated/<prefix>/`.

## New facts at 22:15
- The previous coordinator session and all its agents are gone. Live leftovers: play activation loop (sh 52516 → `bun repo --tool=bazel --workers=off run @semio-tech/semio-tech-play:activate-prototype` 49774, lstart 11:28:51, OLD code, holds `bazel-slot-s0` + images via loader 82896; owner: play-c) and an orphan rcore-b chain (sh 82303 `rcore-b-tools/run.sh`, lstart 21:58:44 → cargo test `semio-framework-os-kernel-db-io-registry`…; owner: rcore-c). Nothing else of ours runs. Capacity: 15.9 GB available, swap 1.6 GB.
- FLEET P0 (infra-c): the 10-min no-progress watchdog kills runs that wait for a Bazel server slot ("all 3 Bazel server slots are in use; waiting for one" → "made no progress for 10 min and has no live child" → rc 124). Course run e2e-1023-course10 died this way after 1h23. Until infra-c reports the fix live, export `SEMIO_SCRIPT_STALL_MS=14400000` for any run that may queue for a Bazel slot.
- FLEET P0 (cap-c): every plugin/hub component build takes the EXCLUSIVE whole-host `semio.compiler.host-capacity/v1`, so component builds and every cargo run serialize (play activation 67/201 after 9 h, ~3 actions/h, load 1.5-6 on 10 cores). Until cap-c reports counted capacity live, Rust sweeps use short per-crate runs (`-p <crate>`), never `--workspace`.
- Nested shared self-deadlock (process waits behind its own shared ticket) fixed by nx-b 20:30 in `🔒️leases/🟦️.ts` `awaitQueuedTurn` (`ownShared`): processes started before 20:30 still carry the bug.

## Addendum (coordinator 22:15)
- One heavy process per agent (`nice -n 10`), still. If you are blocked > 30 min by somebody else's process, write it at the top of your result file with pid + lstart + command + lease line; the coordinator reads result files every ~20 min and decides kills. You may kill only pids you started (verify lstart).
- Static work while you wait for capacity is encouraged: fix the next error you already know, write the test, update the result file.
- Never wait for a build in a `sleep` loop longer than ~9 min per tool call; never use `run_in_background` or Monitor.
- Final message ≤ 8 lines pointing to your result file. Do NOT close/reopen the ticket, do NOT spawn sub-agents.

## Addendum 23:45 (graph-image deadlock)
- A `bun repo run|exec` loader may request EXCLUSIVE `native-fixed-image:…/.cache/b/g` while long activations hold it shared; with writer preference this blocks the whole fleet (twice 23:00-23:40, killed by the coordinator). Until gimg-c reports the fix live in `C/result-gimg-c.md`: before launching a graph-needing run check `bun repo leases status`; if YOUR run shows `WAITING exclusive native-fixed-image:…/.cache/b/g` for > 3 min, stop it yourself (verify lstart) and use graph-free paths (`bun repo cargo …`, explicit `bun test ./<file>`) meanwhile.

## Addendum 00:45 (10/24): graph-exclusive rule LIFTED
- gimg-c's fix is live for processes started after ~23:50: exclusive tickets yield to shared requests after 120 s; the nx loader no longer queues an exclusive headroom recovery above the 256 MiB hard floor (live proof: nx course typecheck passed graph preparation at 92 % full). Graph-needing `repo run|exec` for tests are allowed again. The graph image `.cache/b/g` is 92 % full: gimg-c works on need-sized recreation.
- Memory: activation graph loaders currently keep 9-13 GB after planning (graph-c P0). New activations need the coordinator's go (write a line at the top of your result file and wait); play-c and demo-c activations keep running.

## Addendum 03:45 (10/24): graph loader memory
- Every graph loader (`repo run|exec` that loads the graph: activations, `:test`, `:build`) currently grows to 5-20 GB (graph-c P0, isolated child planning in progress). Until graph-c reports it LIVE as default in `C/result-graph-c.md`: no NEW graph-needing runs except the ones already running for play-c, demo-c, course-c and graph-c's own probes; everyone else uses graph-free paths (`bun repo cargo …`, explicit `bun test ./<file>`). Check `sysctl vm.swapusage` before any heavy launch; above 16 GB used, do not launch.

## Addendum 04:30 (10/24): graph loader memory FIXED, activation stop LIFTED
- graph-c: isolated planning is the default since 04:09 (child peak ~2 GB, loader +0.5 GB; `graph: N projects planned by a child process that has exited`). New graph-needing runs and activations are allowed again, one heavy process per agent; still check `sysctl vm.swapusage` (do not launch above 16 GB used).

## Addendum 07:30 (10/24): drop SEMIO_SCRIPT_STALL_MS
- Slot waits, queue waits, capacity waits and compiler-capacity waits are declared waits now (infra-c, cap-c), so the 10-min no-progress watchdog only kills genuinely idle processes. REMOVE `SEMIO_SCRIPT_STALL_MS=14400000` from every launch script: it is inherited by build actions and lets idle component drivers (no child, no wait, holding cpu units + writer) sit for hours (play-c 07:25).

## Addendum 08:45 (10/24): `env -u SEMIO_SCRIPT_STALL_MS`
- Your shell or an old loop script may still EXPORT `SEMIO_SCRIPT_STALL_MS` (seen in play/demo action envs at 08:50). Launch every detached run as `env -u SEMIO_SCRIPT_STALL_MS python3 C/chain-tools/detach.py …`, and restart loops after editing their script (a running `sh loop.sh` keeps the old text).
