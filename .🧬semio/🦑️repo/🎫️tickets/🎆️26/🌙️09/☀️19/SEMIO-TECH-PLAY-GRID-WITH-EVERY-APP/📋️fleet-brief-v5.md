# 📋️ Fleet Brief v5 (session 6, 2026-09-22 11:00) — addendum to briefs v2/v3/v4

Read `AGENTS.md`, then `📋️fleet-brief-v2.md` (hard rules, fixing rules, stale-test buckets), `📋️fleet-brief-v3.md`
(machine + dev-server rules), `📋️fleet-brief-v4.md` (durability, peer constraints, native mutex), then this file.
`$T` = this ticket folder. You are a SUCCESSOR: read `$T/🗑️generated/<topic>/STATUS.md` and the tracked
`$T/📓️<topic>.md` first, verify the predecessor's on-disk edits by `git diff`, then continue. Do not redo done work.

## State at 11:00
- :6033 serves the 03:04 activation (28 lanes). Strict acceptance on it: 66/70 (reds: wfc2d, wfc3d, grid3d shell
  error at app registration; playbook = peer S10). Visual audit 2 (`📓️audit-visual-2.md`): raster blank, architect
  graph mis-framed, reasoning-wires empty, animate figure missing, puzzle5d loads the wrong document.
- The coordinator queued ONE wasm mutex job (`📜️describe-serial-then-activate.sh`, log
  `$T/🗑️generated/activation/describe-activate-*.txt`): describes 30 plugins one hold each (peer-owned
  norm/space/procedural/playbook excluded; puzzle + stdio last), then `activate-dev` (28 lanes), then recycles :6033.
  Expect :6033 to serve the NEW activation ~2–3 h from now; the log's `activate-dev rc=` line is the signal.
- Last auto-commit 2026-09-21 21:41; everything since is uncommitted on disk (~420 files). Never run git write
  commands. Never revert a peer's in-flight edit.
- Peer session `End-to-end repo completion` (ticket 26/09/18/OS-HUB-COLLABORATION-AI-END-TO-END) restarted 10:50.
  Its no-touch list from brief v4 still applies (hub, `🔌️plugin/🦀️.rs`, browser-bundle, `🏪️store/👷️worker`,
  ShellHost mount/presence, mcp, manifest ExampleDefinition/ExampleSource, guest code of space/flow/sequence/
  procedural/norm/playbook). Guest-code changes there = PROPOSED DIFF in your report; test-side and
  framework-general fixes are yours.

## Rules that bit the fleet last night (obey exactly)
1. EVERY native cargo goes through the play native mutex, ONE invocation for all your crates:
   `zsh "$T/📜️native-test-mutex.sh" <topic> -- env CARGO_TARGET_DIR="$T/🗑️generated/<topic>/target" CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=4 RUST_MIN_STACK=33554432 DEVELOPER_DIR=/Library/Developer/CommandLineTools cargo test --no-fail-fast -p … -- --test-threads=4`
   Note: `--no-fail-fast` is a CARGO flag and goes BEFORE the `--` (engineering's chain11–13 passed it after the
   `--` and produced no results). Run it with `nohup … > "$T/🗑️generated/<topic>/runN.txt" 2>&1 < /dev/null &`,
   then wait with ONE blocking Bash call (`while kill -0 $pid; do sleep 60; done`, timeout up to 600000 ms) and
   re-attach by tailing the log. Do NOT poll every 30 s from separate tool calls (burns the usage window).
2. NEVER run a wasm32 cargo, `describe`, `activate-*`, `materialize-*` or `component-*` target yourself. Ask:
   `touch "$T/🗑️generated/activate.request/<lane>"` and `touch "$T/🗑️generated/describe.request/<plugin>"`.
   `cargo check --target wasm32-wasip2` counts as wasm32 — forbidden.
3. Never kill a process you did not start. Never sweep/delete `🗑️generated`, never `git clean`.
4. A cargo of yours parked > 20 min at 0 % CPU with no rustc child while holding the native mutex: kill YOUR cargo
   and requeue; write the observation to STATUS.md.
5. Wall-clock/timing laws (8 ms worker steps etc.) that fail only under load: re-measure once alone, then report
   as load flakes with the numbers; do not loosen bounds.
6. The test-binary watchdog kills any test binary older than 30 min. A SIGKILL mid-run is the watchdog, not a hang.
7. No sub-agents. Foreground builds only (rule 1). Do not close or reopen tickets. Do not edit `📓️status.md`
   (coordinator-owned); write `$T/🗑️generated/<topic>/STATUS.md` continuously and the tracked `$T/📓️<topic>.md`
   at the end (append a dated section; keep earlier sections).
8. Browser probes against :6033 are allowed (headless Playwright/Chromium via the existing probe scripts under
   `$T/🧪️probe-*.ts|.mjs` or your topic's `probe-*.mjs`); one page at a time; never restart :6033 yourself
   (`touch "$T/🗑️generated/serve-restart.request"` asks the supervisor). Until the new activation lands, a pane
   probe measures the 03:04 build — say so in your report.
9. Schema-first: any wire/format/mutation change goes through the schema/grammar/fixture chain the crate already
   has (see v2). Fixture regeneration uses the crate's own `zzz_write_*` writer, never hand edits.

## Definition of done (unchanged)
Play shows EVERY plugin; every pane boots to `data-shell-ready` with its curated default example and VISIBLE correct
content; no page/console errors, no refused inputs; play unit suite + strict acceptance suite green; every plugin
crate's native test suite green (real defects fixed in production code, stale tests restated per the v2 buckets).

## 12:05 addendum — native mutex now has TWO slots
`📜️native-test-mutex.sh` admits the first two queued tickets concurrently (`/tmp/semio-play-native-test.lock` and
`.lock.2`; `PLAY_NATIVE_SLOTS` to change). Waiters started before 12:05 keep the old one-slot logic until they
requeue. Nothing else changes: ONE invocation per topic, private CARGO_TARGET_DIR, JOBS=4.
`📓️play-runtime.md`: the strict acceptance suite now also asserts the curated example LABEL in the pane chrome,
a non-uniform canvas / non-empty window body, and no `text/html` answer for media-extension paths — probe your
panes against those three before declaring them green.

## 13:05 addendum — back to ONE native slot, JOBS=2
Peer request (machine at load 50–90, disk 25 GiB free, their stdio+gis release build starved): the native mutex is
back to ONE holder (slot 2 parked by the coordinator). Use `CARGO_BUILD_JOBS=2` from now on, keep
`CARGO_INCREMENTAL=0` and your private target dir. Do source work while queued; do not add a second cargo.

## 16:40 addendum — 🗑️generated is NOT durable: fleet state moves to ⚡️cache/play-fleet
At ~16:05–16:25 the repo's workspace-cleanup (`clean`; `🧼️workspace-cleanup` removes `ticket-generated` dirs even
for OPEN tickets) deleted most of `$T/🗑️generated/` (activation, e2e, raster, media incl. its live target dir,
xcut-dict, audits, serve logs). From now on: `$G = /Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/play-fleet/<topic>`
holds your STATUS.md, run logs, probes and `CARGO_TARGET_DIR="$G/target"`. Re-create STATUS.md from your transcript
if it was lost; copy anything that still exists under `$T/🗑️generated/<topic>/`. Requests stay in
`$T/🗑️generated/activate.request|describe.request` (recreated). Never run `clean`/workspace cleanup yourselves.
Tracked `$T/📓️<topic>.md` reports remain the durable record — append to them at every milestone, not only at the end.

## 17:20 addendum — two native slots again (load < 25)
`📜️native-test-mutex.sh` admits TWO holders again. A wrapper queued before 17:20 still runs the one-slot loop: if your
ticket is rank 2 and slot 2 (`/tmp/semio-play-native-test.lock.2`) is free, kill YOUR waiting wrapper + cargo and
relaunch the same command once. JOBS=2 stays. If the coordinator announces load > 100 again, back to one.
