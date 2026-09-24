# Session 11 Preamble

Session 11 of the repo goal (2026-09-25, Claude Code fleet). Coordinator = main Claude Code chat (Opus 5.5). Executors =
Opus 5.5 agents, one slice each. Auditors = Sonnet 5 agents, read-only. Ticket folder (ASCII entry
`/Users/ueli/Documents/semio/.tmp-ticket/`) is this lane; canonical history in `/Users/ueli/Documents/semio/.tmp-ticket-0918/`
(`📓️status.md`, `📓️worker-preamble.md`). Session-10 rules (`📓️session-10-preamble.md`) apply unless overridden here.

## The Goal (four outcomes, all end to end)

1. Working os `s` frontend with all plugins and artifacts.
2. Working hub server backend (db, presence, auth, observability).
3. Working collaboration between users over the hub (React shell and wgpu shell).
4. Working AI integration for users over the **semio** MCP (`semio-framework-os-mcp`, `mcp__semio__*`), never the repo MCP.

## Situation at 00:30

- Session 10 ended 2026-09-24 ~20:46. W1 published **trusted catalog A** (stdio, gis, note, draw, writer, puzzle), generation
  `ee491213…`, root `/Users/ueli/Documents/semio/.🧬semio/🌐hub/w1-catalog-a` (`generated/catalog-a.txt`, `publish-w1-catalog-a.txt`).
- The session-10 **freeze is lifted**. Prepared-but-unapplied patch sets land first, compile-atomic, in the landing window (below).
- Live processes you must NEVER kill: the semio-tech play `nx run @semio-tech/semio-tech-play:dev` tree (pids 6195/6199/6215/37968),
  the vitest-vscode workers, the semio MCP stdio servers of the coordinator session (`dev mcp stdio client|os`), the disk guard
  (`📜️disk-guard.sh`, pid 54365). Hub 7900 (`c8-hub-hold.ts`, pid 74207, data `/private/tmp/g7w-hub-data-7900`) belongs to the
  WG7 slice's lineage: only WG7 may stop or replace it.
- Machine: 10 cores, 32 GiB RAM, ~270 GiB free. Builds are the bottleneck; the fleet is ~8 compiling slices.

## Rules (override session 10)

1. **Landing window (first).** These prepared sets land before any full guest rebuild, each by its owner, compile-atomic (edit →
   immediate `cargo check -p <every touched crate>` incl. `--target wasm32-wasip2` where cfg(wasm) code changed → fix or revert):
   T12 = T10 outcome switch (`wp-t10/switch.py`, `manifest-align.py`) + T11 frozen follow-ups; G10 = G9 commit binding
   (`wp-g9/g9-apply-commit-binding.py`); WG7 = N2 relay (`.tmp-ticket-0918/🐍️n2-relay-edits.py`) + kernel `ureq` native-only move.
   Scripts were prepared on 2026-09-24; re-run their dry run first and re-derive any hunk that no longer applies (never force).
   Record each landing as one row in `📓️landing.md` (slice, what, crates checked, result, time). W2 starts the full rebuild
   when the coordinator says so.
2. **One all-plugin wasm owner.** Only W2 runs describe-all, guest restage, `plugin-registry:generate`, catalog publishes. Other
   slices that need a rebuilt guest append `crate + reason` to `.tmp-ticket/wp-w1/requests/<slice>.txt` and keep working.
   Single-crate wasm32 `cargo check` is allowed for anyone (through the wasm mutex).
3. **Mutexes** unchanged: `zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm|hub <slice> -- <cmd…>`; lock order
   wasm → hub; never hold hub around a wasm build.
4. **Cargo:** `-p <crate>` only; `CARGO_INCREMENTAL=0`; one cargo from you at a time; binary-producing commands with
   `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-<slice>/target` (build-dir stays shared; never change it);
   `--no-fail-fast` before `--`. Before a build, check `ps -axo pid,pcpu,command | grep -c '[r]ustc'`; above 12 rustc wait on
   ONE blocking loop (`until …; do sleep 30; done` in a single `timeout: 600000` call), never poll in many short calls.
5. **Foreground.** Never `run_in_background`, never `Monitor`, never sub-agents, never worktrees. Anything longer than 10 min:
   `nohup … > <capture> 2>&1 & disown` (record the pid) and wait on the capture in one blocking call per 10 min.
   Long-lived servers (hubs, serves) you need across turns: nohup + disown, pid recorded in your report; stop them when done.
6. **Kills:** only pids you started. Never `pkill`/`killall`/pattern kills.
7. **Ports (session 11):** canonical hub **7800** = W2. Slice ranges: W2 hubs 8000–8009 serves 6500–6509; H9 8010–8019/6510–6519;
   C10 8020–8029/6520–6529; G10 8030–8039/6530–6539; S15 8040–8049/6540–6549; WG7 8050–8059/6550–6559 (+ its inherited 7900);
   T12 8060–8069/6560–6569; R8 8070–8079/6570–6579. `lsof -nP -iTCP:<port> -sTCP:LISTEN` before binding.
8. **Report** `📓️wp-<slice>.md` in the ticket folder: skeleton with a status table FIRST, updated after every landed item;
   captures under `wp-<slice>/generated/*.txt` (≤ 1 MB each). Measured results only; "written, not run" is an acceptable,
   honest status; never claim a pass you did not run. Chat answer ≤ 10 lines pointing at the report.
9. **AGENTS.md applies:** schema-first; no legacy/compat/shims/fallbacks/deprecations; no migration scripts left in the repo
   (one-off codemods live in `wp-<slice>/`); emoji-first docstrings; no comments inside definitions; no `[DEBUG]` leftovers;
   bun + nx; permanent scripts only in `📜️script.ts`; new runnable commands registered in `.vscode/launch.json` and
   `.vscode/🧩️launch.seed.jsonc` following the existing order/grouping/naming; en + de for every user-facing string;
   language-agnostic test + third-party oracle for every feature; progress + cancellation for expensive operations.
10. **Git:** never any modifying git command (`commit`, `stash`, `checkout`, `checkout -p`, `restore`, `reset`, …). Read-only git
    (`status`, `diff`, `log`, `show`) is fine.
11. **Peers edit the same files:** re-read before editing, keep to your scope, diff before assuming a vanished symbol is yours to
    restore. Fix a peer's compile break only when it blocks you and the fix is one obvious line; note it in your report.
12. **Grep:** use `/usr/bin/grep` (the shell `grep` alias is ugrep and silently misses matches on large emoji files). Quote every
    emoji path; `cd` explicitly in every Bash call.
13. **Usage limits:** if the account limit cuts you, your transcript is resumed by the coordinator; write the report early and
    often so nothing is lost.
14. **Do not** close/reopen the ticket, edit `🎫️ticket.json`, touch `📌️important.md`, or delete `🗑️generated`/other slices' files.
15. **Durable data lives under `.🧬semio/🌐hub/s11-<slice>-<name>/` (revised 12:4x).** Hub data roots, catalog copies,
    credentials, capability files, hold state and logs of long-running detached chains go there (gitignored, outside every
    sweep). `wp-<slice>/generated/` holds only expendable captures: an external low-disk cleanup (fires near 96 % disk use)
    deleted every ticket `generated` folder and SIGKILLed processes holding files there at ~12:16–12:35 today (and on 09-24).
    Never create data in tracked paths (`git check-ignore -q <path>/x`); the repo's auto-stager stages them. The coordinator
    keeps the disk above 100 GiB free (disk guard); if you see < 80 GiB, stop starting builds and tell the coordinator.
