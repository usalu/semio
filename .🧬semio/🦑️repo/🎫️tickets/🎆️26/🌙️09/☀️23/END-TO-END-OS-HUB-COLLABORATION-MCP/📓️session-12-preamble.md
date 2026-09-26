# Session 12 Preamble

Session 12 of the repo goal (2026-09-25 22:50, Claude Code fleet). Coordinator = main Claude Code chat (Opus 5.5).
Executors = Opus 5.5 agents, one slice each. Auditors = Sonnet 5 agents, read-only. Ticket folder (ASCII entry
`/Users/ueli/Documents/semio/.tmp-ticket/`); canonical history `/Users/ueli/Documents/semio/.tmp-ticket-0918/`.
Session-11 rules (`📓️session-11-preamble.md`) apply unless overridden here. Fleet handles: `📓️fleet-12-agents.md`.

## The Goal (four outcomes, all end to end)

1. Working os `s` frontend with all plugins and artifacts.
2. Working hub server backend (db, presence, auth, observability).
3. Working collaboration between users over the hub (React shell and wgpu shell).
4. Working AI integration for users over the **semio** MCP (`semio-framework-os-mcp`, `mcp__semio__*`), never the repo MCP.

## Situation at 22:50

- Every process of session 11 died at ~19:30 (desktop app restart): hub 7800, all slice hubs (8010–8079), serves, holds, the
  disk guard. Nothing is running. Nothing holds `/tmp/semio-wasm-build.lock`. The coordinator restarted the disk guard
  (log `.🧬semio/🌐hub/s12-coord-logs/disk-guard.txt`). Disk 161 GiB free, load ~4, Docker 29.5.3 answers.
- W2's **restage4 finished 18:56: describe-all → materialize-all → generate → check → activate-s → verify, `consistent=60
  diverged=0`**. No Rust/WIT/TOML source changed after 18:16 (only generated registry TS/RS + repo-library TS), so the staged
  `s` guests match the current tree. T12's S15 guest fixes, U5's space fix and every session-11 host fix are in it.
- Catalog B (`.🧬semio/🌐hub/w2-catalog-b`, generation `e8167ce8…`, 9 packages) is from the 11:44 tree and does not match the
  current hub-native codecs. W2's plan stands: release B packages from THIS tree → **catalog B2** → restart 7800 once on the
  current-tree `os-hub` with a fresh data root → rest (25 packages) → `--packages all` → restart 7800 onto it.
- Peer session "End-to-end repo completion" (ticket 26/09/18) is idle; it was told about this fleet. It owns WG6, N2, M5b, S3.

## Rules (override session 11)

1. **ABI freeze until W2 announces the `--packages all` publish.** No change to WIT, owned guest exports, pack-schema
   identity, codec schema hashes, the closed-actor format, or the hub-native codec crates (stdio, gis, vcs). A fix that needs
   one of them is prepared as a patch set under `wp-<slice>/` (script + clean dry run) and reported to the coordinator; it lands
   in the landing window after the publish. Every other edit lands compile-atomic (edit → immediate `cargo check -p <touched
   crate>`, plus `--target wasm32-wasip2` when cfg(wasm) code changed → fix or revert). Mixed catalogs (packages built
   before/after an ABI-compatible edit) are fine (session-11 decision 07:5x).
2. **One all-plugin wasm owner = W2.** Only W2 runs describe-all, restage, `plugin-registry:generate`, catalog publishes and
   restarts hub 7800. Others append `crate + reason` to `.tmp-ticket/wp-w1/requests/<slice>.txt` and keep working.
   Single-crate wasm32 `cargo check` is allowed for anyone, through the wasm mutex.
3. **Mutexes:** `zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh wasm|hub <slice> -- <cmd…>`; lock order
   wasm → hub; never hold hub around a wasm build.
4. **Cargo:** `-p <crate>` only; `CARGO_INCREMENTAL=0`; one cargo from you at a time; binary-producing commands with
   `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-<slice>/target` (build-dir stays shared; never change it);
   `--no-fail-fast` BEFORE `--`. Before a build check `ps -axo pid,pcpu,command | /usr/bin/grep -c '[r]ustc'`; above 12 rustc
   wait in ONE blocking loop (`until …; do sleep 30; done` in a single call with `timeout: 600000`), never many short polls.
5. **Foreground, no helpers.** Never `run_in_background`, never `Monitor`, never sub-agents (no Agent tool), never
   worktrees. Anything longer than 10 min: `nohup … > <capture> 2>&1 & disown` (record the pid) and wait on the capture in one
   blocking call per 10 min. Long-lived servers you need across turns: nohup + disown, pid recorded in your report; stop them
   when done. Never end your turn while a build of yours is still running unless it is detached and recorded.
6. **Kills:** only pids you started (verify with `ps -o pid,ppid,command`). Never `pkill`/`killall`/pattern kills.
7. **Ports:** canonical hub **7800** = W2. Slice ranges (hubs / serves): W2 8000–8009 / 6500–6509; H9 8010–8019 / 6510–6519;
   C10 8020–8029 / 6520–6529; G10 8030–8039 / 6530–6539; S15 8040–8049 / 6540–6549; WG7 8050–8059 / 6550–6559;
   T12 8060–8069 / 6560–6569; R8 8070–8079 / 6570–6579; U5 8080–8089 / 6580–6589; WG8 8090–8099 / 6590–6599;
   Z2 8100–8109 / 6600–6609. `lsof -nP -iTCP:<port> -sTCP:LISTEN` before binding.
8. **Durable data** (hub data roots, catalog copies, credentials, capability files, hold state, logs of detached chains) lives
   under `.🧬semio/🌐hub/s12-<slice>-<name>/` (gitignored, outside every sweep). `wp-<slice>/generated/` holds only expendable
   captures (an external low-disk cleanup deletes ticket `generated` folders near 96 % disk). Never create data in tracked paths
   (`git check-ignore -q <path>/x` first); the repo auto-stager stages them.
9. **Report:** continue your own `📓️wp-<slice>.md`: add a `## Session 12` section right under the status table (a short
   status table of this session's items FIRST, then a timestamped log), and update the main status table rows you change.
   Update after every landed item. Measured results only; "written, not run" is an honest status; never claim a pass you did
   not run. Chat answer ≤ 10 lines pointing at the report.
10. **AGENTS.md applies:** schema-first; no legacy/compat/shims/fallbacks/deprecations; no migration scripts in the repo
    (one-off codemods live in `wp-<slice>/`); emoji-first docstrings; no comments inside definitions; no `[DEBUG]` leftovers;
    bun + nx; permanent scripts only in `📜️script.ts`; new runnable commands in `.vscode/launch.json` and
    `.vscode/🧩️launch.seed.jsonc` (existing order/grouping/naming); en + de for every user-facing string; language-agnostic
    test + third-party oracle for every feature; progress + cancellation for expensive operations.
11. **Git:** never any modifying git command (`commit`, `stash`, `checkout`, `restore`, `reset`, …). Read-only git is fine.
12. **Peers edit the same files:** re-read before editing, keep to your scope, diff before assuming a vanished symbol is yours
    to restore. Fix a peer's compile break only when it blocks you and the fix is one obvious line; note it in your report.
13. **Grep:** use `/usr/bin/grep` (the shell `grep` alias is ugrep and misses matches on big emoji files). Quote every emoji
    path; `cd` explicitly in every Bash call. macOS has no `timeout` command.
14. **Usage limits / cuts:** the coordinator resumes you by SendMessage; write the report early and often.
15. **Do not** close/reopen the ticket, edit `🎫️ticket.json`, touch `📌️important.md`, or delete `🗑️generated`/other slices' files.
16. **Blocked on another slice?** Do not wait idle and do not poll. Write the exact blocker (what, whose, which evidence) into
    your report, message the coordinator (`SendMessage` to `main`) in ≤ 5 lines, and continue with your next unblocked item.
17. **Detached launches run at full priority (added 00:3x, H9 finding).** zsh's `BG_NICE` puts every `nohup … & disown`
    at nice +5 (ps `NI=5`, STAT `N`), so detached hubs/serves/chains lose to foreground checks and hubs stall in catalog
    load under load. Always launch as `setopt no_bg_nice; nohup … > <capture> 2>&1 & disown`. A running job cannot be
    reniced back to 0 on macOS; restart it only if it is not mid-way through a long build.
18. **Critical-path priority (01:2x, until the coordinator lifts it):** W2's catalog lanes run niced (+5); every other slice
    prefixes EVERY cargo/nx build or test command with `nice -n 15` (e.g. `nice -n 15 cargo test -p …`) so the catalog is
    not starved. The coordinator reniced running non-W2 cargo/rustc to ≥ +12 at 01:25.
19. **Short wasm lane (04:5x, until W2's REST/`--packages all` hold ends):** W2's catalog lanes hold the `wasm` mutex for hours.
    Short wasm32 checks/builds (≤ 20 min, coordinator-approved slices only: WG7, F1; others ask) use the separate lock
    `zsh .tmp-ticket/📜️fleet-mutex.sh wasmshort <slice> -- nice -n 15 <cmd…>` (serialized among themselves, parallel to W2).
    The coordinator runs a cargo lock-cycle watcher; on a cycle the short-lane build is killed, never W2's.
20. **HARD guest freeze (06:0x, until W2 announces the `--packages all` publish is DONE):** no edits at all to any
    guest-linked crate — the plugin SDK (`🔌️plugin/🦀️.rs` and its modules), every framework crate a guest links (kernel,
    editor, ui, dsl, hash, …), every `✏️s/🔌️plugins/**` crate, root `Cargo.toml`/`Cargo.lock`. Each such edit makes the
    bootstrap recompile every later package's closure (stdio alone 20+ min). Native-only crates (hub, os-mcp, native
    renderer shell code, test-only files, TS host code) are fine. Keep guest-side changes as prepared, dry-run-clean patches
    under `wp-<slice>/`; they land in the landing window right after the publish.
21. **Freeze extension (11:2x, W2 finding):** the guest-linked kernel's `🗣️dsl/✨️derive` proc macro reads
    `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json`, root `nx.json` and every `📋️project.json` at compile
    time, and root `Cargo.toml` / `.cargo/config.toml` profile edits change every unit's identity. Until W2 reports the
    `--packages all` publish DONE, none of these files may be edited (nor anything under `📇️directory/🧬️schema/` or any other
    `🔣️.json` the kernel includes). Prepare such changes as patches under `wp-<slice>/`.
