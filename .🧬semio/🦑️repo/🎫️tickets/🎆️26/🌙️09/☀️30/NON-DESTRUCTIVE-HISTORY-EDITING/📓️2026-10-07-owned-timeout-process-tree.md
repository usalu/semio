# Owned Timeout Process Tree

## Demonstrated Existing Runtime Defect

Original Puzzle controller65503 started10:54:15.214Z and requested its unchanged1800000ms timeout. Its direct child exited, but recorded descendant65533 had created a separate POSIX group, became parent1 and held inherited output pipes. Controller did not settle until11:31:25.138Z after identity-fenced SIGTERM to that exact recorded process. The controller therefore exceeded its nominal30minute budget by7m10. No selected Puzzle assertions executed. PDF65565 independently settled timeout11:28:31.195Z with no selected assertions.

The before-stop identity receipt is generated/tools-execution/puzzle-orphan-65533-identity-before-stop.json. A fresh ps inspection confirms65503/65533/65565 absent. Existing live71729/77522/77604 remain untouched.

## Schema-First Regression

A strict language-neutral ownership corpus specifies timeout, cancellation and stop results: owned root and detached descendant terminate, inherited pipes close, unrelated sibling survives, and the original reason is preserved. The third-party tree-kill oracle exercises the same real Node-created process tree. Our runOwnedCommand currently kills only its direct child group on timeout/stop, whereas cancellation already traverses descendants. Production remains unchanged pending executed RED.

## First Harness Capture

Managed tools-owned-timeout-tree-red80124 executed0/1 with5 expectations. The tree-kill oracle settled correctly; the owned500ms test timed out before its READY row arrived (recorded zero child IDs), so this is a harness readiness failure, not feature RED. The neutral timeout is now5000ms and settle ceiling8000ms, allowing actual child startup before validating tree ownership. Production timeout budgets remain unchanged.

## Preserved Current Fleet

Recorded Puzzle/PDF old ancestry now has15 capturedIDs and zero current matches in generated/tools-execution/puzzle-pdf-original-terminal-absence-1138.json. The first readiness harness failure remains separate from corrected tools-owned-timeout-tree-red-b81906, still queued at Nx graph construction. Both selected source tests use canonical Nx execution. Timeout production remains unchanged until the corrected regression executes. Live strict30-b77522 and35-r77604 have current ancestry saved in current-owned-descendants-1148.json.

## Actual Feature RED and Producer Repair

Corrected81906 executed0/1,7expectations,20.95s. Both childIDs were observed. The tree-kill oracle had zero survivors and settled its pipe; current owned timeout retained one detached descendant, did not settle and had no terminal reason. Unrelated sibling remained alive in both implementations. This is actual feature RED, distinct from the earlier readiness failure. Test cleanup terminated only its own recorded ephemeral processes.

runOwnedCommand now delegates timeout, SIGINT, SIGTERM and cancellation to the existing first-party terminateOwnedChildTree. That authority snapshots descendant-created groups before ancestry is lost and refuses an already-exited child, rather than signaling only the direct root group. The delayed root-only force kill is removed. All external timeout values, cancellation reasons, progress and stream framing remain unchanged. The full existing process ownership suite and independent termination law are pending.

## Actual Repaired Runtime Boundary

Owned-execution84049 ran3PASS/1FAIL,36expectations. The new detached-tree test itself passed24.207s, with all six DEBUG results (tree-kill/owned × timeout/abort/stop) settled, zero owned survivors, unrelated sibling alive and correct original reason. Independent termination84087 passed4/4,13expectations9.76s, including the exited-child no-signal guard.

The full suite's single failure was the existing Node UTF8/cancellation adapter: its2000ms startup deadline expired before the ready-line cancellation request, so actual timeout correctly won over cancellation. The neutral report harness now declares10000ms for Node startup, retaining its same text/cancellation/error expectations. Its separate2000ms bounded-descendant test, new5000ms timeout witness and all production budgets remain unchanged. The full suite successor is pending.

## Repeated Old-Loaded Orphan

Strict35-r77604 is terminal11:57:48.510Z with zero assertions. Old-loaded strict30-b77522 again exceeded its nominal deadline with recorded Node77617 parent1 retaining descendants. After exact PID/parent/group/full-command matching against the pre-deadline snapshot, SIGTERM was sent only to that owned Node. Its native-matrix78642 remains a recorded orphan after Node exit; no peer was signaled. Current descendants remain under observation before any successor.

## Old Fleet Terminal Proof

Old-loaded strict30-b77522 finally settled12:02:21.593Z after the identity-fenced orphan request, again zero selected assertions. Strict35-r77604 settled11:57:48.510Z. Fresh14-ID ancestry receipt broad-b-r-terminal-absence-1204.json contains no matches. No other signal or peer action was needed.

The repaired detached-tree test and independent termination suite establish the observed alive-root timeout/stop topology. Pre-deadline1120 explicitly records direct child65528 alive at26m13. This does not claim discovery of a tree naturally orphaned before admission; the exited-child guard intentionally refuses stale rootIDs. The unchanged original Puzzle scope now has an original-route successor, preserving100000 remote/1million snapshot limits and all prerequisites, using only the existing optional preparation timings. Its native verdict remains pending.

## Complete Current Regression GREEN

Managed tools-owned-timeout-tree-green-b89299 completed successfully:4/4 tests,zero failures,39expectations,39.87s (Nx41s). UTF8 line reports, pre-cancel no-spawn, live cancellation, nonzero status and progress assertions all remain. Every tree-kill/owned × timeout/abort/stop result has zero owned survivors, unrelated sibling alive, settled inherited pipes and original reason. Independent process-tree termination84087 is separately4/4,13expectations9.76s, including the exited-child no-signal law. Source is now held stable; no operational timeout, native predicate, preparation recipe, cache authority or acceptance fence changed.

## Actual First Fresh Full Wrapper Deadline Closure

Current original domain/remote90557, strict35/103scope91426 and correctedStdio30scope91454 all reached their original1800000ms deadlines at12:35:09.449Z,12:36:25.995Z and12:36:29.976Z. No selected native assertions or primary compiler diagnostics executed. The saved predeadline16-node Puzzle and14-node strict35/Stdio ownership trees each have zero surviving recorded PIDs after terminal; no manual signal was required. The repaired process-tree timeout behavior therefore closed these concrete Nx/native preparation hierarchies within normal cutoff settlement, unlike prior old-loaded orphan cases. This confirms lifecycle only, not current domain/editor runtime proof. DWG93422 remains live with its original later cutoff12:42:38; no new Cargo successor through UIwindow.
