# W3-INFRA report

## 1. Nx daemon / watcher (failure mode a)

Framework findings (read-only):
- Nx `useDaemonProcess: true` is in `/home/deck/semio/nx.json`; it is a workspace setting shared with humans, left untouched.
- The framework has a supported switch: the daemon admission rule (`decideNxDaemonV1`, `nxDaemonAdmissionV1` in `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🕸️graph/🏁️startup/🚏️admission/🟦️.ts`) says "an explicit `NX_DAEMON` wins". `NX_DAEMON=false` means graph from the generation receipt and plain hashing (documented in `🧰️framework/🛍️products/🦑️repo/README.md`, "Nx (live)"). The patched Nx (`nx@23.2.0.patch`) also returns no daemon client when `NX_DAEMON === 'false'`. Alternative: `SEMIO_NX_DAEMONS=0` (finite daemons off) only applies when `NX_DAEMON` is unset.
- `semio-repo-watch` (change journal, `SEMIO_REPO_CHANGES=on|off`, `SEMIO_REPO_WATCH_BINARY`) already runs with `--exclude .git .cache .🧬semio/🦑️repo/⚡️cache .🧬semio/🦑️repo/🎫️tickets bazel-* --exclude-name node_modules target __pycache__ .venv`. These excludes are built in the code (`🕸️graph/🔏️seal/📥️membership`), not configurable by env. Tickets, caches and `target` are therefore already ignored. A failing watch there only makes the journal refuse (harmless full stat walk, documented in `📓️build-guide.md`). The Nx daemon's own watcher (the one that hit the limit) honours `.nxignore`, which already has `/.🧬semio/**`; editing `.nxignore` would change shared hashing, so not done.

Implemented (runner only, build invocation env only): `runner-next.sh` runs `NX_DAEMON=${RUNNER_NX_DAEMON:-false} ... repo run`. `nx-reclaim.sh` still stops stray idle daemons between builds. No user desktop service touched.

## 2. Graph-plan memory margin (failure mode b)

- New `build-runner/graph-gate.sh`: needs available RAM >= 5700 MB and RAM + free swap >= 7000 MB (peak 4500 + margin 2500), max wait 3600 s, then waits up to 240 s for a running heavy command. Called by `pre-build-gate.sh` (its old 5700 MB / swap > 512 loop was replaced; with swap permanently full that loop could never release) and before every attempt in `run_one`, so integrate, mechanics and fixture-sweep are gated too.
- Pause of heavy leases: gate writes `graph-pending/<dir>`; `heavy.sh` now waits unconditionally (not only when memory is low) while `freeze.txt` is non-empty or a live marker exists. Previously `freeze.txt` only made a heavy command wait if memory was also low, and freeze appeared up to 30 s after graph-plan began; the marker closes that gap. Verified by reading: `heavy.sh` wraps any command (`python3 ... sweep`, `scripts`), so Python sweeps are covered when started through it; commands not started through `heavy.sh` (agent's direct python, other tools) are not. Already-running leases are not suspended (SIGSTOP would not free RSS); the gate waits for the current one to finish.
- Marker removal: `update_freeze` (runner main loop) removes it at `nx:received-graph` or dead pid; `finish_run` and the retry loop also remove it.

## Activation and caveats

- `heavy.sh`, `pre-build-gate.sh`, `graph-gate.sh` are live immediately (fresh process per call; replaced atomically).
- `runner-next.sh` was replaced atomically; the running instance (pid 961335 tree, 3 processes) keeps the old copy. The NX_DAEMON change and the `graph-pending` handling take effect after a runner restart. I did not restart it (it owns live job subshells; restart is a coordinator decision at an idle moment, `adopt()` handles orphans).
- Syntax checked with `bash -n`; nothing was executed. Swap is at 8941/8947 MB used right now, so the new gate will hold the next graph-plan until available RAM >= 7000 MB.
- Without the daemon, tasks hash without the daemon file watcher: expect slower cold hashing, not different results.
- Documented in: `build-runner.md` (section Infra v9) and `fleet-wave3-preamble.md` (brief). `audits/infra.md` does not exist, so the infra section went into `build-runner.md`.
