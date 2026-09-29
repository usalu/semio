# Session 14 Preamble

Session 14 of the repo goal (2026-09-27 18:1x, Claude Code fleet). Coordinator = main Claude Code chat (Opus 5.5).
Executors = Opus 5.5 agents, one slice each. Auditors = Sonnet 5 agents, read-only. Ticket folder (ASCII entry
`/Users/ueli/Documents/semio/.tmp-ticket/`). Fleet handles + coordinator log: `📓️fleet-14-agents.md`.
**Rules of `📓️session-12-preamble.md` and `📓️session-13-preamble.md` (rules 2–37) apply unless overridden here.**

## The Goal (four outcomes, all end to end)

1. Working os `s` frontend with all plugins and artifacts.
2. Working hub server backend (db, presence, auth, observability).
3. Working collaboration between users over the hub (React shell and wgpu shells).
4. Working AI integration for users over the **semio** MCP (`semio-framework-os-mcp`, `mcp__semio__*`), never the repo MCP.

## Situation at 18:1x

- Session 13's coordinator and every agent are gone (ids cannot be resumed). Each `📓️wp-<slice>.md` is the handover;
  a session-14 slice continues its predecessor reports (named in `📓️fleet-14-agents.md`).
- **Final chain FAILED 17:03** in rebuild-all step 4/11 (descriptors): layout's untracked leaves
  `🧬️mutations/📐update-grid` + `🔒set-frame-flags` (a peer's 14:21 work) lack their `🔣️.json` authority and
  `🧬️schema/`; robotic (`🏭️process/🧩️extensions/🤖️robotic`) and flow text (`🌊️flow/🧩️extensions/📝️text`) component
  builds also failed. Log `.🧬semio/🌐hub/s13-w3-logs/final-rebuild-all.txt`. No `final-publish.rc`, no all catalog.
- **Hub 7800 is READY on catalog B3** (9 packages: stdio, gis, note, animate, block, writer, draw, puzzle, wfc;
  hold 4540 / hub 4548, root `s13-w3-hub-7800-b3`, state `.🧬semio/🌐hub/s13-w3-state-7800/`). It stays until the
  chain moves it onto ALL. Still alive from session 13: C11 hub 8022 (hold 45592 / hub 45599), H12 hub (79070), two
  stale LB `fleet-mutex.sh` waiters (31879, 42023; W4 stops them).
- No Codex peer process is running right now; human devs and other agents may still edit the tree at any time.

## Rules (override sessions 12/13)

1. **Chain = critical path.** W4 owns it: unblock → preflight → the coordinator relaunches
   `python3 .tmp-ticket/wp-w2/w2-detach.py .🧬semio/🌐hub/s14-w4-logs/chain-final.txt zsh .tmp-ticket/wp-w3/w3-chain.sh final`
   (or W4's successor script) → `--packages all` publish → 7800 onto ALL (fresh root) → readiness + open-plan probe.
2. **GUEST FREEZE from the coordinator's "CHAIN LAUNCHED" line until W4 reports `final-publish.rc` = 0.** Frozen:
   every crate or file the chain builds — `🧰️framework/**` Rust + `🔌️plugin/**` (incl. browser bundle, shard worker,
   support, plugin host), kernel/store/sync, SDK, renderer, every `✏️s/🔌️plugins/**` crate, `📚️library/🔣️taxonomy.json`,
   root `nx.json`, `📋️project.json` files, `Cargo.toml`/`Cargo.lock`, `.cargo/config.toml`, `.vscode/launch.json` +
   `🧩️launch.seed.jsonc` (launch rows are generated from project.json). **Open during the freeze:** `🌎️hub/**` crates
   (semio-hub, os-hub bin), the os-mcp host crate (code not linked into any guest), React shell TS under `💻️os` that is
   NOT under `🔌️plugin/`, ticket-folder scripts, reports. Hub/os-mcp edits must be compile-green (native lane) BEFORE
   `final-publish.rc` appears, because the chain builds os-hub + os-mcp right after it; a red there costs a relaunch.
   Guest-linked work during the freeze = **prepared patches** (idempotent script in `wp-<slice>/`, `--dry-run` clean on
   the live tree) plus narrow proofs in an overlay (rule 37 of session 13). Window 3 opens when W4 reports the publish rc;
   the coordinator announces it in `📓️fleet-14-agents.md`.
3. **Lanes (session-13 rules 29/37 kept):** wasm32 = chain only during the freeze. Native cargo =
   `zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native <slice> -- nice -n 15 <cmd…>` with
   `CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b`,
   `CARGO_INCREMENTAL=0`, private `CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-<slice>/target` for
   test/build/run, `-p <crate>` only, `--no-fail-fast` before `--`. Overlay builds = `fleet-mutex.sh overlay <slice> -- …`
   with a PRIVATE build-dir inside the overlay (never the shared build-dir: memory "Scratch Clone Poisons Shared
   Build-Dir"). The chain uses the default build-dir — nobody else touches it.
4. **Foreground only:** no `run_in_background`, no `Monitor`, no Agent tool, no worktrees. Anything > 10 min:
   `setopt no_bg_nice; nohup … > <capture> 2>&1 & disown` (record the pid) and wait on the capture in one blocking call
   per ≤ 10 min. Long-lived processes (hubs, serves, holds) launch with
   `python3 /Users/ueli/Documents/semio/.tmp-ticket/wp-w2/w2-detach.py <log> <cmd…>` so they survive your turn.
5. **Ports (session 14):** 7800 = W4 only (read-only live use by everyone). Hubs / serves per slice:
   W4 8000–8009 / 6500–6509; H13 8010–8019 / 6510–6519; C12 8020–8029 / 6520–6529 (keeps C11's 8022);
   G12 8030–8039 / 6530–6539; S18 8040–8049 / 6540–6549; WG11 8050–8059 / 6550–6559; T14 8060–8069 / 6560–6569;
   ST2 8070–8079 / 6570–6579; SH2 8080–8089 / 6580–6589; AV2 8090–8099 / 6590–6599; EN2 8100–8109 / 6600–6609;
   S19 8110–8119 / 6610–6619; R10 8120–8129 / 6620–6629; LB2 8130–8139 / 6630–6639; H14 8160–8169 / 6660–6669
   (keeps H12's hub); Z4 8190–8199 / 6690–6699. `lsof -nP -iTCP:<port> -sTCP:LISTEN` before binding.
6. **Memory (32 GiB machine):** ≤ 1 headless browser per slice, closed as soon as a run ends; stop every hub/serve you
   are not using this hour (record it); Docker (pg/neo4j) only via `os-hub-ts:backend-up` when your item needs it and
   `backend-down` after; no Docker image builds until the coordinator lifts this. Above 14 rustc
   (`ps -axo command | /usr/bin/grep -c '^[^ ]*rustc '`) wait in ONE blocking loop before starting a cargo.
7. **Data:** durable data under `.🧬semio/🌐hub/s14-<slice>-<name>/` (gitignored); `wp-<slice>/generated/` holds only
   expendable captures (≤ 1 MB each). Never delete `🗑️generated/` or other slices' files.
8. **Report:** `📓️wp-<slice>.md` (create or continue): `## Session 14` right under the title block — status table of
   this session's items FIRST, then a timestamped log. Measured results only; "written, not run" is honest; never claim
   a pass you did not run (with the command + capture path). Every landed tree edit = one row in `📓️landing.md` under
   `# Session 14` (slice, set, files, crates checked native/wasm32, result + capture, time). Chat answer ≤ 10 lines
   pointing at the report.
9. **AGENTS.md applies in full**: schema-first; no legacy/compat/shims/fallbacks/deprecations/migration scripts in the
   repo (one-off codemods live in `wp-<slice>/`); emoji-first docstrings; no comments inside definitions; no `[DEBUG]`
   leftovers; bun + nx; permanent scripts only in `📜️script.ts` + nx target + generated launch row; en + de for every
   user-facing string; language-agnostic test + third-party oracle per feature; progress + cancellation for expensive ops;
   event-sourced CQRS (no CRUD, no CRDT).
10. **Git:** never any modifying git command (no commit/stash/checkout/restore/reset/worktree). Read-only git is fine.
11. **Peers edit the same files:** re-read before editing, keep to your scope, diff before assuming a vanished symbol is
    yours to restore; fix a peer's compile break only when it blocks you and the fix is obvious — note it in your report.
12. **Shell:** `/usr/bin/grep` (the `grep` alias is ugrep and misses matches); quote every emoji path; `cd` explicitly in
    every Bash call; macOS has no `timeout`; `setopt no_bg_nice` before `nohup … &`. Kill only pids you started
    (verify `ps -o pid,ppid,command`); never `pkill`/`killall`.
13. **Do not** close/reopen tickets, edit `🎫️ticket.json`, touch `📌️important.md`, or use the repo MCP ticket tools.
14. **Blocked?** Write the exact blocker in your report, `SendMessage` to `main` in ≤ 5 lines, continue with your next
    unblocked item. Never wait idle, never poll. Usage cuts: the coordinator resumes you — write the report early and often.
15. **Credentials:** use only the local test users the hub recipes provision (`wp-w3/w3-restart-7800.sh`: user1/user2
    @semio.dev) or ones you create on your own local hub; never print them in reports or chat.
16. **Cross-slice messages (18:4x):** slices cannot address each other directly. To coordinate, `SendMessage` to `main`
    starting with `RELAY <slice>: …` (≤ 5 lines); the coordinator forwards it. Prefer writing the agreement into both
    reports so it survives cuts.
17. **Harness productization contract (R10, 18:5x) — binding for every slice that makes a harness permanent:**
    (1) port YOUR ticket-local harness as a verb of the owning module's existing `📜️script.ts` (os-dev, os-mcp-rs,
    os-hub-ts, renderer-wgpu), code in an EXISTING directory where possible — a NEW directory needs R10's taxonomy
    registration in window 3, so tell R10 (via main) the path now; (2) the harness writes its record through
    `withAcceptanceRecord` + `publishAcceptanceCheckResult`
    (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/🟦️.ts`): en + de summary, `blocked` for
    missing preconditions, flags `--hub <url> --serve <url> --locale en|de`, credentials only via env (never argv, never
    logged); (3) do NOT edit `📋️project.json` / `launch.json` / launch seed — send R10 (via main, `RELAY R10:`): project,
    target name, command, args, env names, requires (hub|serve|localServe|backends|hubAdmin), browser count, acceptance
    criteria ids, en + de title. R10 adds the nx target + generated launch row in window 3 and the check to the goal gate
    `⚖️gate🎯️repo-goal` (`@semio-tech/repo-test-domain:acceptance-goal`). Dev serves: use S18's shared `ensureDevServe`
    fixture once S18 publishes its path (don't fork one). Existing permanent harnesses — reuse, don't re-port: os-dev
    program-matrix, tool-run-matrix, hub-document-sweep, two-human, collab-e2e; os-mcp-rs plugin-coverage-check,
    user-path-check, security-check, live-agent-loop-check, hub-agent-participant-check; os-hub-ts backup-restore-drill,
    residency-watch, boot-watch, hub-freshness, backend-*.
18. **Shared dev-serve fixture LANDED (S18, 19:2x):** `ensureDevServe({repoRoot, port, variant?, locale?, hubUrl?, signal?,
    onProgress?, bootBoundMs?, logPath?}) → {url, reused, stop()}` in `🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts` (region
    🔖️DevServeFixture, + `devServePortV1(url)`); harness wrapper `withDevServe(…)` in `🧑‍💻dev/🧪️tests/🧮️program-matrix/🟦️.ts`
    (publishes `blocked` when no serve). Every harness that needs a dev serve uses it — no forks.
19. **Codex peer on stdio (19:2x):** its gate `testEditorCatalogContract` (stdio `📜️script.ts`) asserts the full 88-editor
    catalog; the tree ships lb-p3/lb-p4's 18-app subset. ST2's per-family packages are the resolution (all 88 shipped under
    the wasm size ceilings). Nobody re-adds `full-app-catalog` to one component; nobody edits that gate while the peer is
    active in it.
20. **Host TS edits are compile-atomic (19:3x):** two half-landed React-shell TS edits broke every `s` boot within 20 min
    (TEXT_EDITOR_SCENE_LANES module-level use in an import cycle; a removed `NamedLayoutStore` export with an importer left
    behind). After every host TS edit that can reach the shell bundle: `tsc` of every package that imports the changed module
    AND one `serve s react dev` boot to Home (pageerror-free console) BEFORE your next edit or any hand-off. Prefer removing an
    export only in the same edit that updates all its importers (`/usr/bin/grep -rn` the name first).
21. **LOAD/MEMORY EMERGENCY (20:3x): load 82–97 on 10 cores, swap 36/38 GB.** Until the coordinator lifts this:
    (a) lanes are back to ONE slot each (`/tmp/semio-<lane>-build.slots`; the mutex script now supports N slots — only the
    coordinator changes those files); never run cargo/tsc/vitest builds outside a lane except `tsc` of ≤ 1 package;
    (b) at most ONE dev serve and ONE hub per slice (two serves only for a two-shell run, stopped right after), ≤ 1 headless
    browser, stop everything idle NOW and record it; (c) `NX_DAEMON=false` for every `bun nx …` (two wedged workspace
    daemons sat 31 min in project-graph calculation); (d) a queued lane ticket costs nothing — keep it, don't bypass the lane.

## Session 14b — Restart (2026-09-28 12:0x)

- The whole fleet was cut ~20:45 yesterday by the account usage limit; the app then restarted: every agent, hub 7800, serves,
  holds, the disk guard and all lane waiters died. Agents cannot be resumed; every slice gets a successor agent with the SAME
  slice name that continues `📓️wp-<slice>.md` (read its `## Session 14` table + log first; reconcile your predecessor's
  in-flight tree edits and prepared patches before anything else).
- A peer (Codex, not running now) edited ~1 870 source files overnight 01:50–05:0x; nothing since. Re-diff every prepared patch
  (dry-run) against the live tree before relying on it.
- **CHAIN LAUNCHED 2026-09-28 12:02:46** (`wp-w4/w4-chain.sh final`, pid 45604, log `.🧬semio/🌐hub/s14-w4-logs/chain-final.txt`).
  **GUEST FREEZE is ON** (rule 2) until W4 reports `final-publish.rc` = 0 and the coordinator writes "WINDOW 3 OPEN".
- Hub 7800 resumed on B3 at 12:03 (hold 45800, `wp-w4/w4-hub-resume.sh s13-w3-hub-7800-b3`, rolling capture). Disk guard pid
  45433. Disk 101 GiB free (build-landing deleted). Lane slots: native 1, overlay 1, wasm = chain only.
- Rule 21 stays: one serve + one hub per slice, ≤ 1 headless browser, `NX_DAEMON=false`, no lane bypass. The machine is idle
  apart from the chain — keep it that way for the chain's sake (the chain is the critical path).
- **Usage budget:** a 20-agent fleet burned the 5-hour usage window in 2.5 h yesterday. Wait with ONE blocking call per ≤ 10 min
  (never short polls), keep reports terse, don't re-read large files you already summarized, and prefer finishing one item end to
  end over starting many.
22. **Test-only fixes during the freeze (13:1x):** fixtures, `🧪️tests/` files and `#[cfg(test)]`-only code in guest-linked
    crates MAY land during the guest freeze when no non-test code changes (they don't affect the chain's lib/wasm builds);
    compile-atomic in the native lane (`--lib --tests`), landing row. Anything touching non-test code stays prepared.
23. **Overlay lane = 1 slot, disk-bound (13:5x):** every overlay with a private build-dir grows to 40–50 GB cold (p9-build 50 GB,
    t14 42 GB); disk is at 101 GiB free with the chain still to write wasm-release units. Overlay proofs are OPTIONAL
    pre-validation — window 3 proves every set compile-atomically in the real tree anyway (native lane + wasm lane). Only queue
    an overlay build when it de-risks a large guest set (≥ 20 files or kernel/SDK wide); otherwise keep the patch dry-run clean
    and wait for window 3. Delete your own overlay build-dir as soon as its proof is recorded (record the deletion).

## Session 14c — Second Restart (2026-09-28 16:5x)

- The usage limit cut the fleet at ~14:37 and the desktop app went down with it: every agent, the chain (it had reached rebuild-all
  attempt 2 components, 12:17 → 14:37), holds and guards died. The app came back at 16:35. launchd cannot run our long jobs (macOS
  privacy blocks ~/Documents for launchd jobs), so long processes still die with the app — reports and prepared patches are the
  handover; nothing else survives.
- **CHAIN RELAUNCHED 2026-09-28 16:55:46** (pid 76013, same command/log `.🧬semio/🌐hub/s14-w4-logs/chain-final.txt`). GUEST FREEZE is
  ON (rule 2) until W4 reports `final-publish.rc` = 0 and the coordinator writes "WINDOW 3 OPEN". Hub 7800 is DOWN (not restarted:
  current-tree clients speak channel 19 and cannot use B3 anyway) — the chain restarts 7800 on ALL.
- Coordinator landed LB2's chain-critical `wp-lb2/lb2-p4-structural-classification.py` at 16:5x (stdio `structural_table_window_kind`
  stamps its 5 appended rows `Migrated`; without it stdio `plugin()` panics at describe). Proof = the chain's describe step.
- A **Cursor agent peer** (repo MCP `dev mcp stdio cursor`) is actively editing stdio pdf editors and norm (en1994/din4108) and runs
  nx tests — work alongside it; re-diff before editing those areas.
- Disk 78 GiB free (overlay build caches of s19/c13 deleted by the coordinator). Rule 23 stands.
- Successors are spawned in two waves: wave A now (slices with work before window 3), wave B at WINDOW 3 OPEN. Every successor
  FIRST reconciles its predecessor's last in-flight step (named in its spawn prompt): finish to compile/tsc-green + rule-20 boot, or
  cleanly revert its own half-applied hunks; record it.
24. **WINDOW 3 OPEN (2026-09-28 21:00):** landing runs as TRAINS per `📓️window3-plan.md` — R10 lands T0 (taxonomy/launch,
    serialized), then the integrator **L1** applies every prepared guest set in train order (T1 SDK core → T2 stdio → T3 apps/shells
    → T4 host runtime) with one combined native + wasm32 + tsc/boot proof per train and reverts only a culprit set on red. Set owners
    do NOT apply their prepared guest sets themselves; they keep them dry-run clean, answer L1's relays, fix their reverted sets for
    the next train and run their laws when asked. Host-only fixes from live verification still land directly (rules 20/22).
    Hub 7800 = catalog **s14-w4-catalog-p24** (24 packages; fem, flow, mathematical, shooting, lowpoly, forms, norm, imperative,
    sourcing + demonstrator are NOT on the hub until the next chain republishes all 34).

## 14c lane update (2026-09-28 23:3x)
- (00:2x reverted to 1 slot: load 147, swap 6.6 GB) Native lane opened to **2 slots** (`/tmp/semio-native-build.slots` = 2; memory 69 % free, swap 0 after the 22:42 reboot). L1 train
  proofs keep their priority stamps. If swap passes 8 GB the coordinator drops it back to 1. Overlay 1 slot, wasm = L1 trains/chain.

25. **KERNEL PANICS (2026-09-29 01:2x): the machine panicked 4× (22:42, 00:31, 00:41, 00:48) — `watchdog timeout: no checkins
    from watchdogd in 90 seconds`** (`/Library/Logs/DiagnosticReports/panic-base+socd-*.panic`) = userspace starved under load
    (our cargo/vite/chrome + the user's VS Code ripgrep indexing ~550 % CPU + GitKraken/VS Code git status + a 5 GB `git
    index-pack --verify-stat`). Every panic kills every agent, hub, serve and lane. Until the coordinator lifts this:
    (a) the mutex now caps `CARGO_BUILD_JOBS` at 4 for native/overlay/wasm and holds a granted slot until the 1-min load is
    < 32 (`FLEET_LOAD_GATE`); (b) ONE heavy job machine-wide at a time (native OR wasm, not both; relaxed 05:3x: a native job may run beside wasm while the 1-min load < 16), no parallel serves;
    (c) never start a serve/hub/browser while a lane job runs unless the job is yours and needs it; (d) ≤ 4 live agents (relaxed 06:4x to ≤ 8: agents are cheap, builds are gated by the lanes + load gate).
26. **SWEEP 01:14 (2026-09-29):** the recurring EXTERNAL cleanup deleted every gitignored dir inside the ticket (`wp-*/generated/`,
    `wp-*/w3-backup/`, `wp-*/target/`, `🗑️generated/`) — owners' set backups and captures are GONE. Keep backups, overlays,
    captures and logs ONLY under `.🧬semio/🌐hub/s14-<slice>-*` (survived). Landed sets' `--revert` state is lost (T1 is
    green, no revert needed); for T2+ L1's own `l1-land` records (now under the hub dir) are the revert path.
27. **WINDOW 3 CLOSED — ALL-34 CHAIN LAUNCHED 2026-09-29 07:18:33** (pid 44628, log `.🧬semio/🌐hub/s14-w4-logs/chain-final.txt`;
    catalog `s14-w4-catalog-all`, 7800 root `s14-w4-hub-7800-all`; T1–T5 + c12-splice landed, PRE-CHAIN GREEN 07:18). GUEST FREEZE
    (rule 2) until the coordinator writes "WINDOW 4 OPEN": guest sets are PREPARED only (for train T6: H14 retire-pages, WG11
    json-number + shell-footprint + renderer fixes, LB2 family hosting, U6 row target). Host-only fixes still land per rules 20/22
    (compile-atomic) — hub/kernel-db edits must keep `os-hub:build-dev` green (the chain builds the hub from the tree at its end).
