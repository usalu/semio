# F71: why the broker never answered, and why no request may now go unanswered

## Root cause
- The stuck `syn` (F69, pid 46584) belonged to the very broker F64 diagnosed live (pid 38325): the unit was in the broker queue, `waiting.push` done and the early "queued" verdict sent, and the client then reads for ever. Admission nets the driver's own memory out of the grant (`parentBudget = grant - nonCompiler`, 0 for a big cargo), `admitWaitersV1` skips any waiter above that budget even with nothing running, and nothing re-evaluated it. That is a stalled queue, not a lost verdict: F64's growth/rescue code fixes the cause, but the broker process F69 saw was started before that code landed (a broker is the driver's own process, it keeps the code it started with).
- Candidates checked in code: F60 growth wait is bounded (15 min) but depended on the gate honouring its signal (no guard if it never answers); client reconnect/restart: the broker lives inside the driver (cargo dies with it), a restarted broker gave the client EOF, which is an error, not silence, but a re-send was missing; no lease deadlock found (the broker takes no lease on this path). The remaining silent paths were: no heartbeat, no client read deadline, no stall/queue bound, unbounded `predict` and growth `acquire`.
- Slot ordering: the wrapper took the single compile slot and only then ran the scheduler that waits for admission, so one stuck admission froze every unit.

## Fix (all under `Policy.bounds` of the admission schema, en/de messages)
- Broker: heartbeat verdict (`admitted:false`, a valid wire line) every 30 s to every waiting unit and from the growth ticker; a unit that waits with nothing running for 90 s runs alone (`forced`, still behind the host-available check); a unit older than 45 min is refused with `parent-memory`; estimating a unit is bounded to 120 s (connection dropped, client asks again); a grant growth is guarded by its wait plus 30 s grace (waiter withdrawn, a late grant given back); the watchdog runs on the existing 10 s tick whether or not a log/waits port exists.
- Scheduler (Rust): reads verdicts through a reader thread with a 180 s silence deadline (`TimedOut`), closes its socket on drop so the broker forgets the dropped request, and `admit_with_retries` sends the admit again (5 attempts, pause 2-15 s) on silence, EOF, refused or missing endpoint: a request survives a broker restart; refusals are never repeated.
- Wrapper: `awaitBrokerHeadroomV1` polls the broker's `probe` (never joins its queue) before `acquireSlotV1`, bounded to 60 s, so a wrapper waits for headroom first and takes the slot after. A fleet-only pass answers `fleetOnly:true` in its headroom reply (`HeadroomV1.fleetOnly`) so nothing waits for a refusal.

## Proof
- Admission `🧪️tests/⌛️deadlines` 6/0: Rust constants equal the policy; stalled-idle unit hears heartbeats then runs alone (both-language log); heartbeat behind a running unit; queue bound refusal; growth gate that ignores its signal ends at its bound and the unit still gets a verdict; fleet-only headroom reply.
- Unit cache `🧪️tests/⌛️headroom-first` 5/0 (fake broker: polls until admit, bounded timeout, no broker / silent / fleet-only never blocks, source order headroom before slot).
- Rust `rustc --edition 2024 --test` 14/0 incl. `silent_broker_is_dropped_and_asked_again` (silent listener, resend answered on the second attempt, refusal not repeated, exhausted attempts); non-test build has no warnings.
- Neighbours green: waits 6/0, growth 8/0, probe 3/0, parent 5/0, sibling 2/0, beyond-host 2/0, replay 5/0, tokens 4/0, charged-peak 2/0, slot-deadline 3/0. dangling-imports exit 0.
- Live: `repo cargo --store=shared --configuration=prototype -- check -p semio-ui-contract --lib` (rebuilt scheduler with the new Rust) exit 0, Finished in 8m41s (wall 10m34s, load 33-47); the units were held by the warden (danger) with the 30 s en/de lines and then compiled.

## Not verified / notes
- `🧯️available/🎫️broker` 12/1: `sourceKlocV1` fallback-line expectation (5 vs 2), not caused here (code untouched).
- `repo typecheck`: no error in any touched file; 5 errors in untouched importer test files (growth `47,89`, `🧯️available`, native-build `🦀️cargo` test, unit-cache `🔭️cutoff`), JSON literal widening and a missing export, pre-existing.
- No real broker restart or 77-min stall was reproduced live; the restart case is proven by a silent-listener unit test (Rust) and the stall by the broker test with the driver holding the whole grant. Windows: the reader thread cannot be cancelled by closing a named pipe, so a dropped attempt leaves its broker-side request until the process exits (it exits with the failure otherwise).
- The scheduler binary is rebuilt on every machine by the source receipt (Rust changed).
