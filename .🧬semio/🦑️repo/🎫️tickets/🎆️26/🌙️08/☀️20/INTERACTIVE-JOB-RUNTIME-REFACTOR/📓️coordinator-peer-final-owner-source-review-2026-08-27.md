# Peer Final-Owner Source Review

## Native Confirmation

The coordinator read `🧪️member-presence-all-r2-native-2026-08-27.txt`: all ten retained-Presence tests pass, 881 filtered, 0.00 s rounded test time after 24.72 s compilation. The exact tests below are no longer source-only: overlapping old rosters in held-reader and two-worker modes retire eight actor bytes and two peer payloads; local replacements/read returns retain Store authority; seven rejection/adoption vectors preserve 65,536-byte backing owners; and the rejected owner uses only its minting factory after the source closes (original 1, foreign 0). This scoped ownership result does not certify global allocation leases, real CAD adoption or maximum all-app step latency.

## Rejected Actor and Original Factory Review

The coordinator reread the complete new rejected-admission module. The rejected owner now retains both the original String (including excess backing capacity) and P, together with a privately minted Arc to the exact original retirement factory. Its consuming into_retirement method takes no replacement factory, transfers String into Vec without copying, retires initialized bytes under the grant, separately frees the empty allocation, and then delegates P to that captured factory. All terminal guards preserve ownership on incomplete close. The seven admission fixture vectors and the source-close/original-versus-foreign factory law are native tests queued with the ten Presence laws; they are not yet passing runtime evidence. This source repair supersedes the rejected-actor defect recorded below without claiming the separate global allocation-credit seam is solved.

## Reviewed Checkpoint

The coordinator read the complete peer roster, publication, commit and retirement definitions in the shared Store component, together with the new overlapping-roster law in its Presence retirement module. This is source review only; the changed native test has not executed.

Every successful peer publication now transfers the previous immutable roster root into a retirement cursor. The cursor retains that root while captured root readers exist, then moves its fixed array into owned retirement. Each individual shared entry is consumed with `Arc::into_inner` rather than retaining competing clones with `try_unwrap`. This allows overlapping old rosters to surrender their aliases without mutually blocking each other's final owner. The one winning entry owner retires its actor string by UTF-8 scalar and hands its payload to the installed typed disposer.

The new test covers both a held old reader and two concurrently closing overlapping rosters, checks the captured reader's exact values, counts two peer payload retirements and eight actor bytes before Store close, then verifies the separate local root closes. Its debug output and assertions remain unobserved until the native gate runs.

## Remaining Scope

The later local-Presence source checkpoint is now coherent: local captures and preparation bases are opaque reads, returned-read maintenance also runs on the idle host path, and detach transfers the same registry plus an active returned cursor. The coordinator independently reran the canonical source self-test target and observed 834 passing self-tests (exit 0), with complete output in `📓️coordinator-tool-job-selftests-r12-2026-08-27.md`. The current native registered-dispatch r11 build is active from a cold target; no changed native law has executed yet. This supersedes the mid-edit status below, not its runtime caveat.

- The local Presence capability integration is currently being edited. Bare local-root aliases and publication-preparation bases need the opaque read/return protocol and a live Store maintenance owner; no CAD adoption credit follows from the peer-only source repair.
- Peer actor admission bounds string length, but the reviewed `adopt` API accepts an owned `String` with independently variable backing capacity. Its rejected-actor paths also return the payload without returning the actor owner. The maximum-envelope byte-capacity and rejected-owner contract still needs explicit verification or repair before a bounded-allocation claim. A short string is not proof of a small backing allocation.
- The public retirement/publication ownership protocol, abort paths, Store-close guard, and actual mounted cancellation must all execute natively. Fixed-array loops and typed cursor names are not timing measurements.
- Snapshot-reader return ordering has separate native race and contention laws queued; the peer overlap law does not substitute for them.

The workspace advanced to peer commit `a8d1caf41f` during this checkpoint. No git-modifying operation was performed by the coordinator, and peer changes are preserved.
