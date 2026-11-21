# Graph Read Handoff Quiet Clock

Current source has a concrete conditional watchdog handoff gap. `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🕸️graph/🔭️read/🟦️.ts` lines 59–74 creates a new watch on each follower and, on adoption, replays at most the last 4 MiB of untimestamped log. Lines 91–103 detach without terminating the owned read and let the next follower adopt it. `🏗️build/🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts` lines 21–24 initializes `progressAt` to the new follower’s current time, and lines 45–70 also timestamp qualifying replayed history at the current time. No previous quiet timestamp or progress high-water state is supplied.

Consequently, if normal handoffs recur before the 1,500,000 ms stall window expires, the quiet deadline is renewed without physical progress. The 43,200,000 ms absolute deadline remains anchored to the retained `first.startedMs` (watch verdict lines 85–87). This is a source-supported conditional, not proof that handoffs caused this live read’s current delay. Replaying the retained tail can reconstruct bounded counters, but its old rows carry no event timestamps to recover elapsed quiet.

At 00:31:59 UTC on 2026-11-21 the existing read record still named runner 91085, read 91086, start 1795189402549 and fresh beat 1795221117570. `read-watch` was an empty file with a fresh mtime, used as follower custody in `🔭️read/🟦️.ts` lines 96–99, not a quiet checkpoint. `serve-state.json` reported `reading:none` and contained no watch clock; its independent snapshot does not by itself prove which follower currently owns the watch. These artifacts do not establish actual handoff times or how long the current follower has observed silence.

Existing `🌐️remote-execution/🛰️sidecar/🧪️tests/🕸️graph/🔭️read/🟦️.ts` lines 31–54 verifies adoption and detached-read custody, while the `🪪️graph-state/🙋️wanted/🧪️tests/🛠️server/🩺️read-watch` suite covers a single watch. A deterministic handoff regression needs a controlled clock, prior progress and repeated adoption, preserving the original absolute deadline and verifying that replay does not manufacture progress. No service, process, production source or network state was changed.

## Physical handoffs on November 21

The primary sidecar log `.cache/f/sidecars/build-agent.log` records the same read PID 91086 across the following natural server handoffs. The read record retained runner 91085, read 91086 and the same original start. The log explicitly reports attachment to the already running read.

| Detached UTC | Reattached UTC | Next reported quiet age |
| --- | --- | --- |
| 00:10:55.363 | 00:11:15.533 | 121 seconds at 00:13:42.179 |
| 00:15:46.082 | 00:16:07.567 | 120 seconds at 00:18:32.390 |
| 00:19:03.706 | 00:19:23.717 | 121 seconds at 00:21:44.303 |
| 00:31:58.257 | 00:32:22.753 | 121 seconds at 00:34:57.225 |

At 00:07:15.457 the previous observer already reported 421 seconds without qualifying progress. The accepted owner tally remained 3,913 of 4,808, and the last scheduler inference count was 3,937. These are different output boundaries. The old scheduler emitted no further inference record before the handoffs. The quiet resets therefore occurred on the existing physical read, with no corresponding counted progress. The latest graph sidecar transition follows its ordinary recorded quiet-period reload after code changed. None of these actions replaced the read.

At 00:27:13.674 the repo's explicit physical diagnosis recorded twelve processes, eight paging, 979 MiB resident and 12,452 MiB physical footprint under the read's 14,336 MiB grant. Machine pressure was warn with 6,513 MiB available of 32,768. The visible read output was only its separate command heartbeat. This establishes pressure and missing publication, while leaving the exact parent callsite or owner unresolved.
