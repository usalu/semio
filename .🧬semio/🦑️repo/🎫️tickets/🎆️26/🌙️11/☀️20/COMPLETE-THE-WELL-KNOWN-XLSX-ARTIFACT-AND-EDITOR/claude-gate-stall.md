# Gated cargo runs stall at `Compiling proc-macro2`

## Cause
The rustc wrapper (`unit-cache/script.ts rustc`, sealed copy under `.cache/b/r/m/u/sealed`) takes a host-wide compile slot (`acquireSlotV1`, `🔒️lock/🟦️.ts`) before it starts the inner compiler (`semio-build-scheduler rustc`). Grants are FIFO: a waiter writes a ticket in `.cache/b/r/m/u/q/`, and only the oldest live ticket may claim a free slot in `.cache/b/r/m/u/s/`. Holders are judged dead with `processAliveV1`, which used `kill(pid,0)`. A zombie (exited, not reaped by its parent) still answers `kill(pid,0)`, so it counted as alive.

Observed state (2026-11-23 17:47):
- `s/0.lock` held by pid 36253 and the head ticket `q/001795417947893-ddd25e45-37001` owned by pid 37001. Both are `<defunct>` (state Z), as are four other early tickets (37002, 37301, 37096, 37669).
- Their parent is pid 32937, an orphaned (ppid 1) `cargo test` for `semio-s-artifact-stdio-xlsx-base`. It has run 10h46m in state R and never reaps its children.
- About 35 tickets queue behind the zombie head. Every live `rustc` wrapper sleeps (state SN) waiting for the head ticket, so no rustc child is ever spawned. Cargo's jobserver helper is the only other live process.
- The capacity gate admission ("1 of 10 cores") is not the problem: the gate admitted the run, and the stall happens one level down in the slot queue.

A stale-holder rule that only checks that the pid exists cannot recover from this, however long it waits. The heartbeat file ages hours, but `stale()` kept it because the pid answered.

## Fix
`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🔒️lock/🟦️.ts`
- `processExistsV1` (old `kill(pid,0)` semantics), new `processZombieV1` (Linux `/proc/<pid>/stat`, other POSIX `ps -o stat= -p`, Windows never), and `processAliveV1` = exists and not zombie (also fixes its other callers: lane liveness, headroom, offload admission).
- `stale()` now treats a holder as dead when its pid is gone, or when it is a zombie and its heartbeat is overdue (older than 2 heartbeats). A healthy holder never triggers the platform query, so there is no `ps` cost on the hot path, and a live holder with a stalled event loop is still kept.

Not changed: live slots held by processes that are really running; other agents' builds are untouched. Zombie slot/ticket files are reclaimed by the waiters on their next fallback poll, but only once the wrapper is re-sealed with this fix (running wrappers use the old sealed copy, so already stuck runs must be restarted). Zombie parent 32937 was not killed (not started by this session).

Remaining duplicates of the old `kill(pid,0)` check (not on this stall path, left for a later consolidation): `graph/startup/admission`, `storage/.../scan`, `storage/.../funding`, `remote-execution/worker`.

## Test
`unit-cache/🧪️tests/🧟️zombie/🟦️.ts` (bun test): injected-state laws per platform, a real zombie (perl fork without wait) detected as not alive, a slot and queue head held by zombies reclaimed within 5 s, a live holder kept however old, a zombie with a fresh heartbeat not judged yet. 7 tests pass together with `🪶️light-slot`; `🪟️windows`, `🔁️churn`, `🧳️portable` still pass (53 pass, 4 skip).

## Not done
A waiter diagnostic ("waiting on ticket pid N, slot holders ...") is not added; `slot-wait` events carry only the wait time.
