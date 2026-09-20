# 📋️ Fleet Brief v3 (session 4, 2026-09-20 01:30) — RECOVERY addendum to `📋️fleet-brief-v2.md`

Read `AGENTS.md`, then `📋️fleet-brief-v2.md` (ALL of its hard rules, fixing rules and stale-test buckets still
apply verbatim), then this file. `$T` = this ticket folder.

## What happened
The whole v2 fleet (and its coordinator) was killed by a process restart at ~01:18 on 2026-09-20, mid-flight.
No agent wrote a final REPORT.md. Every edit the dead agent of your topic made is ON DISK (nothing is lost, nothing
is committed by you — an auto-committer owns git). You are the successor of the dead agent with your topic name.

## Recovery protocol (do this first, in order)
1. Read everything the predecessor left in `$T/🗑️generated/<your-topic>/` — `STATUS.md`, `RECIPES.md`, scripts, and
   the TAIL of its newest logs (`ls -lt`). Logs whose last lines are mid-compile were cut by the restart.
2. Read the cross-cutting owners' notes: `$T/🗑️generated/xcut-toolproof/STATUS.md`,
   `$T/🗑️generated/xcut-dict/STATUS.md`, and sibling `RECIPES.md` files (`norm-a`, `norm-b`, `stdio-a2`, `stdio-b2`).
3. Orphaned cargo runs of the dead fleet may still be alive (ppid 1) and still writing into your topic folder;
   let them finish (they are your first-run result), never kill cargo/rustc you did not start.
4. Re-run your crates to learn the CURRENT truth before editing, then continue where the predecessor stopped.
5. Keep `$T/🗑️generated/<your-topic>/STATUS.md` current (append a dated line after every finished step: what
   landed, what is running, what is next) so the next restart costs nothing. Write `REPORT.md` at the end.

## Machine
10 cores / 32 GB, shared with two other coordinator sessions and a Codex fleet: load average ~100, swap nearly
full, ~46 GB disk free. `CARGO_INCREMENTAL=0 CARGO_BUILD_JOBS=2 RUST_MIN_STACK=33554432`
`DEVELOPER_DIR=/Library/Developer/CommandLineTools`. Batch several `-p` crates into ONE cargo invocation rather
than one invocation per crate (each invocation re-queues on the shared build-dir lock). Wait for cargo in the
FOREGROUND of a single Bash call with a long timeout (up to 600000 ms) and re-attach by tailing the log — do not
burn turns polling every few seconds. Check `df -g /System/Volumes/Data` before big builds (prune rule in v2).

## Dev servers
`:6033` (play) is kept alive by `$T/🔁️serve-supervisor.sh` (owned by the coordinator). Do not start/stop/restart
any dev server. Read-only probing of `http://127.0.0.1:6033/` with headless playwright is allowed
(pass `--use-angle=metal`; one page at a time; the server may need 15–60 s per answer under load).
Only the play-* topics may run `bun nx run @semio-tech/framework-os-dev:activate-<variant>-react-dev` lanes and
`bun ./🔨️modules/🧩️runtime/📜️script.ts activate` in `🏢️semio-tech/🎡️play`.

## Definition of done for the ticket
`semio-tech play` shows EVERY plugin (every editor app of every plugin descriptor, incl. stdio, space and the
imperative/flow extensions in the activation union) in its grid; every pane boots to `data-shell-ready` with its
curated default example and VISIBLE, correct content; no page errors, console errors or refused inputs; the play
unit suite and the strict acceptance suite (`bun nx run @semio-tech/semio-tech-play:test-e2e`) are green; every
plugin crate's native test suite is green.

## Dev server addendum (02:15)
The supervised play server on :6033 now runs FROZEN (`SEMIO_TECH_PLAY_FROZEN=true` → Vite file watching off): the
fleet's edit storm restarted/wedged Vite every few minutes. Consequence: the server does NOT pick up source edits
or new activation receipts. When you need your play/host/TypeScript edits or a new activation served, request a
recycle: `touch "$T/🗑️generated/serve-restart.request"` — the supervisor restarts the server within ~30 s (cold
start ~1–2 min under load; wait for HTTP 200). Request a recycle only when you are about to verify in the browser,
and never while you can see from `$T/🗑️generated/serve-6033-supervised.txt.events.txt` that it was recycled less
than 5 minutes ago (the audits are probing pane by pane and resume, but every recycle costs them a pane).
