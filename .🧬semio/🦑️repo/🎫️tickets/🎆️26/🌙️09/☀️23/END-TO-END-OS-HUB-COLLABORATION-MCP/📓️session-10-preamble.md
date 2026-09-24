# Session 10 Preamble

Session 10 of the repo goal (Claude Code fleet, coordinator = main Claude Code chat). Ticket folder (ASCII entry
`/Users/ueli/Documents/semio/.tmp-ticket/`) is this lane; the canonical history lives in
`/Users/ueli/Documents/semio/.tmp-ticket-0918/` (`📓️worker-preamble.md` rules 1–33 still apply except where this file
overrides them).

## Situation at 22:50

The machine rebooted at ~19:48. Every session-9 worker, hub, serve, mutex hold and queued build is dead; `/tmp` was
wiped (no mutex queue/lock). Peer "semio-tech play" processes (vite `play:dev`, `play:build`, their wasm `cargo rustc`
demonstrator/sequence) are running now — NEVER kill them. Uncommitted tree: ~430 changed paths; auto-commit is not
running, so nobody loses work, but never run modifying git commands.

## Rules (override session 9)

1. You are a Claude Code agent: emoji in paths and file contents are fine. Quote paths. Prefer the Read/Edit tools with
   absolute paths; the ASCII symlinks `.tmp-ticket`, `.tmp-ticket-0918` are available for shell convenience.
2. Foreground only for anything you must see finish inside your turn. Long builds: run ONE Bash call with
   `timeout: 600000`; if it needs longer, launch it detached (`nohup … > <capture> 2>&1 & disown`, record the pid)
   and wait on the capture with `until grep -q … ; do sleep 20; done` in a single `timeout: 600000` Bash call, repeated.
   Detached processes survive your turn; plain background ones do not.
3. No sub-agents, no worktrees, no `isolation`. Do not close/reopen the ticket, do not edit `🎫️ticket.json`, do not
   delete `🗑️generated` or anything in it you did not create, do not touch `📌️important.md`.
4. Every wasm32 plugin build, `describe`, `plugin-registry:generate`, `component dev/release`, `materialize`,
   `activate`, `trusted-catalog-bootstrap`: through the fleet mutex in one call:
   `zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm <slice> -- <command …>`.
   Only slice **W1** issues full-catalog/all-plugin wasm work; other slices that need a rebuilt guest write the request
   (crate + reason) into `/Users/ueli/Documents/semio/.tmp-ticket/wp-w1/requests/<slice>.txt` and continue with other
   work; W1 appends the result path to the same file.
5. Every `cargo test|nextest|build` of `-p semio-hub` and every `os-hub:test*` target:
   `zsh …/📜️fleet-mutex.sh hub <slice> -- <command …>`. `cargo check -p semio-hub` is free. Lock order wasm → hub.
6. Every cargo: `-p <crate>` only, `CARGO_INCREMENTAL=0`, one cargo from you at a time, binary-producing commands
   with `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-<slice>/target` (never change `build.build-dir`).
   `--no-fail-fast` goes BEFORE `--`.
7. Never `pkill`/`killall`/pattern kills. Kill only pids you started. Deadlock test: your cargo at 0 % CPU, no child,
   10 min, `sample <pid> 1 | grep -E 'prebuild_lock|open_rw_exclusive'` hit → kill your pid, rerun once.
8. Ports: hubs 7800–7899, serves 6300–6399, each slice its own ten (listed in its brief). Record pids in the report.
9. Report `📓️wp-<slice>.md` in the ticket folder, skeleton FIRST, updated after every landed item; captures under
   `wp-<slice>/generated/*.txt` (≤ 1 MB). Measured results only; never claim a pass you did not run.
10. AGENTS.md applies: schema-first, no legacy/compat/shims/fallbacks, no migration scripts, emoji-first docstrings, no
    comments in definitions, no `[DEBUG]` leftovers, bun + nx, permanent scripts only in `📜️script.ts`, new runnable
    commands registered in `.vscode/launch.json` + `.vscode/🧩️launch.seed.jsonc`.
11. Peers edit the same files: re-read before editing, keep scope, diff before assuming a vanished symbol is yours to
    restore. Fix a peer's compile break only when it blocks you and it is a one-line obvious fix; note it in the report.
12. In chat return ≤ 10 lines pointing at your report.
13. **Compile-atomic edits (added 12:15).** W1's catalog publishes (45–110 min each) died three times on peers'
    transient half-edits in crates every guest links (kernel store/pack, framework plugin, stdio). In any crate under
    `🧰️framework/**` or `✏️s/**`, land a change as a set that compiles: make the multi-file edit, then run
    `cargo check -p <crate>` (plus `--target wasm32-wasip2` when you touched cfg(wasm)/wasi code) IMMEDIATELY and fix or
    revert before doing anything else. Never leave a crate non-compiling while you go off to run tests, wait on
    builds or research.
14. **Disk (added 12:50).** At 12:43 an external disk-space cleanup deleted the whole shared cargo cache
    (`⚡️cache/cargo/target` + `build`, ~460 GiB; the disk was at 96 %) and SIGKILLed every process holding files there.
    Every guest/crate is cold again. The coordinator runs `📜️disk-guard.sh` (pid 54365): below 80 GiB free it prunes
    cargo incremental sessions idle > 60 min. Delete your own `wp-<slice>/target` when your slice is done.
