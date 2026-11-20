# Graph Lease Wait Listener Retention

The graph exchange's private pause left its abort callback attached after a normal timer completion. Calling the public graph decision repeatedly with one long-lived signal retained a callback and timer closure after each busy-peer skip or lapsed-claim retry. A signal cancelled before it entered the pause also registered a listener and waited for the timer.

Both completion paths now remove the exact registered callback and clear the timer. A signal that has already been cancelled skips the pause immediately. The public decision still skips a busy peer and lets the lapsed-claim path reach its existing cancellation check.

A language-neutral fixture exercises six conditions across both timed paths through the public decision. It observes the real signal's listeners, preserves an unrelated subscriber, repeats normal pauses on one controller and checks that a pre-cancelled pause settles before the next event-loop turn. The old code failed four cases, and all six now match an independent SQLite model. The existing oldest-holder test used an expired claim with its advanced clock. Its fixture now holds the lease live across the configured takeover time so it exercises the intended branch. The lease-wait and exchange-refusal suites passed 22 tests with 184 assertions using mocked clients and no network requests.
