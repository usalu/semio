# F69: unit-cache wrapper wedge (cargo 69285, port 6094)

## Root cause (evidence, not the cache server)
- Cache server answered in 0.25 ms; its claim table has lifetime caps. Not the cause.
- All ~28 wrappers of cargo 69285 queued FIFO in `.cache/b/r/m/u/q` for the single compile slot (`SEMIO_UNIT_CACHE_SLOTS=1`, `u/s/0.lock`). The 6094 connections were idle keep-alives.
- Slot holder: wrapper pid 46543 of another cargo (46056, F60's `check -p semio-ui-contract`, parent chain 38325 broker bun, 37807). Its inner `semio-build-scheduler rustc syn` (pid 46584) sat at 0 % CPU for 68+ min, blocked in `recvfrom` in `Broker::verdict` (admission `read_line`, no read timeout). `repo capacity status` showed no unit wait, so the broker never queued the unit: it stalled before `waiting.push` (predict/growth chain; its log file was an unlinked F60 generated file, so the exact await could not be read).
- A live holder is never stale, and the slot wait had no deadline, so one stuck admission froze every other lane.

## Fix
- Slot wait is bounded (`slot.maximumWaitMilliseconds` = 600000): after it the wrapper logs slot-wait "timeout", counts fallback slot-timeout and compiles without the slot (the memory broker still admits it). Ticket removed on timeout.
- Fleet requests already carry deadlines (AbortSignal.timeout per call, breaker down file, retries); proven by a test against a server that never answers: claim returns unavailable in under 5 s.
- Test: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧪️tests/⌛️slot-deadline/🟦️.ts` 3/0.

## Killed
- pid 46584 (semio-build-scheduler rustc syn, repo tooling, 0 % CPU 77 min, proven blocked in recvfrom). The slot passed to the next waiter at once.

## Open
- Why the broker 38325 never answered the admit request for syn (needs its log; consider a broker-side watchdog that refuses an admit whose predict/growth chain exceeds a bound).

## Latency measurement (coordinator request), live server :6094, signed client, 04:5x
- getResult 0-7 ms, slotKeys (F40 scan) 6-15 ms, claim 1-6 ms, release 0-2 ms; 30 parallel getResult all done within 14 ms. Server queueing, F40 scans, ledger and resolver are not slow. The stall was the compile-slot queue behind a wedged admission, not request latency. Keep-alive connections on 6094 were idle. The `down` breaker file showed intermittent faults from other clients (uploads), but lookups were fast.

## Proof
- `repo cargo --store=shared --configuration=prototype -- check -p semio-ui-contract --lib` (same wrapper path as the stuck run) finished: exit 0, 3m12s, after the stuck holder was removed.
- `-p semio-ui-tui-laws --tests` cannot be selected now: the root Cargo.toml (last written 03:17) has no tui member, cargo says the spec matches no package (peer workspace drift, not this wedge). Not proven on that exact crate.
- Probe script: scratchpad/F69/lat.ts (not kept in the ticket).
