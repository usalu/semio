# 1028 Fleet Brief — Demonstrator End To End

Ticket: `26/08/28/DEMONSTRATOR-END-TO-END-ALL-APPS` (folder = C). Coordinator = main chat. NEVER close/reopen the ticket.

## Goal
`@semio-tech/mit-bestand-demonstrator:activate-prototype` (Bazel) green → `serve` on 6029 → `test-e2e` green, all 8 panes (generator/koordinator/aggregator/aussuchen/bearbeiten/verfolgen/energy/fem3d) render their fixtures.
The activation closure is 32 components (`♻️mit-bestand/🧺️demonstrator/🔨️modules/🧩️runtime/🧫️cases.json`): plugins cad, demonstrator, energy, fem, flow, gis, playbook, procedural, process, puzzle, sourcing + 21 extensions/modules. Plus framework wasm targets `framework-surface-rs:wasm`, `framework-editor-rs:wasm`, `framework-os-node-graph-rs:wasm`, `puzzle-plugin:wasm` (`semio-hub-puzzle`), `semio-framework-os-flow-browser-package:wasm`.
Guest components are built for `wasm32-wasip2`. Last history: `C/../../../🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/result-demo-c.md` (75/77 on 10-24; many puzzle3d/generation3d/stdio/flow compile errors then).

## The coordinator owns the official run
The coordinator runs `activate-prototype` detached (`C/1028-tools/act.sh`, logs `C/🗑️generated/1028/act*.log`). Agents NEVER run `repo run …:activate-*`, `…:prepare-*`, `…:serve`, `…:dev`, `…:test-e2e`, or `repo dependencies sync|lock`.

## Your route: the compile frontier of your slice
- `cd /Users/ueli/Documents/semio && SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path <Cargo.toml> --target wasm32-wasip2 --keep-going -p <crate> 2>&1 | tail -n 300 > C/🗑️generated/1028/<you>-chkN.txt` (foreground; it may queue on compiler leases — that is normal, wait). Never set RUSTC/RUSTC_WRAPPER, never call raw `cargo`. Native-only checks miss `cfg(target_arch="wasm32")` code. Also check the native host side (no `--target`) when your crate has host/native code paths used by tests.
- Fix the errors at the root, clean long-term, no compat layers, no re-export shims, no `#[allow]` to hide errors. Match surrounding code style; docstrings start with a unique emoji; no comments inside definitions.
- Other agents (peer fleets, Codex) edit the same files concurrently. Re-read a file right before editing; keep edits small and compile-atomic (never leave a file half-migrated). If a peer is mid-sweep in a file, work with the current state; do not revert foreign changes.
- Shared crates (framework value/dsl/mutation/stdio contract, etc.): before changing a shared API, grep all users; prefer fixing the consumer to the current API. If you must change a shared crate, note it in your result file under `SHARED CHANGE` with the reason.

## Hard rules
- No git modifying commands (commit/stash/checkout/restore/reset). No worktrees. No `repo dashboard`, `repo clean`, `repo ticket purge-artifacts`, `pkill`/`killall`/pattern kills (kill only pids you recorded yourself).
- No sub-agents. Run commands in the foreground (background children die with your turn). macOS has no `timeout` command.
- Temp files/logs only under `C/🗑️generated/1028/` with your name prefix; never delete other files there. Scripts you write go to `C/1028-tools/<you>/`.
- Use `/usr/bin/grep` or `rg` (the shell grep alias silently misses matches on emoji paths). `cd` explicitly in every Bash call. Quote emoji paths.
- `[DEBUG] ` prefix for temporary logs; remove them before you finish.
- Disk: if `df -h /Users/ueli/Documents/semio` shows < 40 GiB free, stop compiling and report.

## Result
Write `C/1028-result-<you>.md`: status line first (GREEN / RED + frontier), files changed (absolute or repo-relative paths), errors fixed (crate: cause → fix), remaining errors with your diagnosis, SHARED CHANGE list. Keep it updated as you go (the usage window may cut you off). Final message: 5 lines + result path.

## Addendum 17:15 — waiting on the compiler queue
Machine admission serializes cargo (3584 MB per check; ~2 granted at once), so a check may queue 20-60 min. Bash moves a command past 600 s into the background. Pattern that survives:
1. Start the check yourself detached, writing a done marker: `cd /Users/ueli/Documents/semio && nohup bash -c 'SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check … > C/🗑️generated/1028/<you>-chkN.txt 2>&1; echo $? > C/🗑️generated/1028/<you>-chkN.rc' >/dev/null 2>&1 &` (record the pid).
2. Wait in the FOREGROUND with bounded loops (each Bash call < 590 s): `for i in $(seq 1 55); do [ -f C/🗑️generated/1028/<you>-chkN.rc ] && break; sleep 10; done; ls C/🗑️generated/1028/<you>-chkN.rc 2>/dev/null; tail -3 C/🗑️generated/1028/<you>-chkN.txt`. Repeat the loop call until the .rc exists. Never end your turn while waiting; never start a second check for the same crates while one is queued.
3. While waiting you MAY read code to pre-diagnose likely errors, but edit only after you have the compiler output.
Batch your crates into ONE check (`-p a -p b …`) per workspace/target to use one queue slot.

## Addendum 18:35 — the source-boundary gate is policy
Never raise `[package.metadata.semio.rust-source-boundary]` limits or the policy/schema to get past the gate. Split crates (or move generated data out of source) instead. Measure with `bun C/1028-tools/gatecensus/census.ts <roots file>` (real gate, no compile). Owners of the 23 violators: C/1028-coordination.md entry 18:30.

## Addendum 19:55 — free-fix mode (user: maximum throughput)
Ownership is now advisory. When your check fails in a crate you don't own: if no file of that crate changed in the last 10 minutes (`find <crate dir> -newermt "$(date -v-10M '+%Y-%m-%d %H:%M:%S')" -type f`), fix it yourself (clean, current API, no shims, no limit raises) and note it in your result; otherwise work on the next failing crate. Re-read every file right before editing; keep edits compile-atomic. Keep iterating checks until your slice is GREEN — don't stop at the first foreign error.

## CHECKPOINT 22:55 — weekly usage 96 %
Finish only the edit you are in (leave every file compile-atomic), then: update your `C/1028-result-<you>.md` with STATUS, files changed, exact next step + command, and stop. Start no new edits or checks. Kill only your own still-queued check pids (recorded) unless they are the one check that verifies your last edit.
