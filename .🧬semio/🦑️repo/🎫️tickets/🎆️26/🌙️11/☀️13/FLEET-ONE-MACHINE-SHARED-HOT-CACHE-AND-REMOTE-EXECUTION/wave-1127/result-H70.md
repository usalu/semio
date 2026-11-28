# H70: who sends the SIGTERM that ends a stage after adoption

## Verdict
The sender of the 11 historical SIGTERMs is NOT proven: the evidence is gone (failed log is overwritten per attempt, warden ring keeps 2-8 alarms, `kills.log` is machine-local and not mirrored). What the code and the records exclude or show:
- Adopting supervisor's own stop (stall, stale-waiting, restart, cancel, timeout): its kill sets local flags, so the end is `stalled`/`cancelled`/`timedOut`, never "sender unknown". H10's stale-waiting landed 09:00 and its clock starts at adoption; 8 of the 11 SIGTERMs predate it. Not the sender.
- Previous supervisor's shutdown / sidecar hand-over / agent restart: launcher and agent signal only their own role child (`Bun.spawn` role processes); stages are `detached` wrappers (own session) and survive, which is why adoption works at all. Not a stage sender.
- Warden: a real sender for some runs (m06 alarms 11:00:36 and 11:02:29: `cancel` of kind `stage`, 68/70 MB, danger = exactly the m06 13 s and 74 s runs). For m07/m02/m15 the retained alarms are slice-worker cancels only and the warden cancel counters are tiny (m07 4, m02 1, m15 1), so it cannot explain m07's 6+ SIGTERMs.
- Real defects found in our own code (fixed):
  1. A stage adopted WITHOUT watchdog history (record absent or rejected by the current schema after a landing) got its stall clock from the stage's START (or log mtime): a quiet stage older than `stallStopMilliseconds` was SIGTERM-stopped seconds after adoption "for lack of history".
  2. A stop sent by another supervisor than the one that watches the end (old process draining while the successor adopts) was recorded in the stage record but ignored at the end: the watcher saw SIGTERM with no local flag = "unknown sender".
  3. Stop reasons were written into the stage record only when a launch record existed.
  4. Our own signals were visible only in machine-local `kills.log`, never in the mirrored failure log.

## Instrumentation (every stage end by a signal now names its sender, in the mirrored failed log, EN/DE)
- Stage record gets `stopBy {supervisor, reason, atMs}` written BEFORE every supervisor stop (also without launch record). Schema `StopBy`.
- Wrapper exit file gets `received` (termination signals the wrapper itself received: SIGTERM/SIGINT/SIGHUP, time) and `sent` (its own deadline signals, written before sending). SIGHUP is now recorded and forwarded.
- `readyStopSenderV1` (process-tree) decides from records only, within 60 s of the end: `supervisor` (this / another supervisor, pid, reason, time), `deadline`, `warden` (cancel row names the stage pid), `runner` (kill record `kills.log` names the stage pid; `readyKillRowsV1` parses it, tail 512 KB), else `outside-wrapper` (the wrapper itself was signalled, nothing of ours explains it) or `outside-child` (wrapper got no signal: tree signalled directly, i.e. code inside the stage or an outside program). Result field `sender`, line appended to `tail` and `failure` (so `ready-log-<stage>-failed.txt` carries it).
- macOS gives no sender pid unprivileged: an unrecorded sender is named by exclusion and the line says so.
- Classification: a stop recorded by another supervisor classifies the end (stalled/cancelled/timed out) instead of unknown SIGTERM.
- Adoption never kills for lack of history: `readyAdoptionClockV1` starts the stall clock at the adoption when no history exists.

## What to watch
Worker `ready-log-workspace-failed.txt` lines "Ended by ..." / "SIGTERM reached the stage wrapper ..." / "The stage wrapper received no signal ...". `outside-child` points into the stage (repo/Nx own cancellation, e.g. repo-mcp build or capacity code), `outside-wrapper` to an outside program/OS; both mean no supervisor, warden or deadline did it. Also worker `ready.json` field via result `sender`.

## Verification
- Landed 12:00 slot, lock `landing.lock.ready` (released), `T/🧰️scripts/H70/apply.py` (atomic, anchor-checked). New tests `🧪️tests/🛑️stop-sender` 10/0: sender truth table vs Python oracle (12 cases), EN/DE texts, kill-record roundtrip with the real recorder, adoption clock vs oracle, wrapper receipt / child-only / deadline with the real wrapper, supervisor integration (outside signal, other-supervisor stop, own cancel).
- Regression on the real tree: ready `🧪️tests/🟦️.ts` 59/0, containment 16/0; scratch copy before landing: external-stop, stale-waiting, holder-custody, stage-observers, warden-cancel, failure-log, code-stop, stale-stage, disk-guard, step-overdue, mirror, source-transient, wrapper deadline, process-tree folders all green.
- `repo typecheck` of the 4 touched files + 61 importers: 0 errors (after a one-line test typing fix); dangling-imports on the ready folder exit 0.
## Unverified
No worker runs the new code yet; the historical sender stays unproven. Wrapper change only affects stages started after workers restart on this code.

## Typecheck follow-up
The one TS2769 error (test line 33, `kind` literal union vs string) was fixed at 12:05 with `kind as string`; `repo typecheck` of the test file now reports 0 errors, and the test still passes 10/0.
