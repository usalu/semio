# Typed Result ACK Watchdog Audit

> Superseded for current source by [the runnable/maintenance audit](📓️terra-plugin-ack-runnable-maintenance-audit.md). The poll-count retransmission described below was removed; retain this document only as pre-fix evidence.

Status: **production RED.** This was a current-source audit only: no build or runtime test was run.

## Decisive finding

A normal renderer ACK that arrives after the next reactor pass is already stale; after a third unacknowledged continuation poll, the operation is converted into a `Fault` page. This is not elapsed-time retry logic.

`MountedTypedCommandFullOperation::take_result_page` treats every repeated read as a retransmission: it sets attempt 1 on initial output, changes it to 2 on the next read, and changes the same retained page to `Fault` on the third (`attempt >= 2`). [`plugin/🦀️.rs:16365`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16365) [`plugin/🦀️.rs:13072`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:13072) The ACK validator requires equality of the complete original token, so the first retransmission makes a normally delayed ACK for attempt 1 return `false`; it does not merely tolerate a duplicate. [`plugin/🦀️.rs:16383`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16383)

The reactor makes those reads without a wall-clock wait. Every poll first schedules live cleanup, then consumes a shell ACK if one was delivered, then calls typed continuation and routes a page as `SendMessage`. [`reactor/🦀️.rs:1369`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1369) [`reactor/🦀️.rs:1496`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1496) [`reactor/🦀️.rs:1842`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1842) `has_pending_typed_operations` remains true while any result page is ACK-owned, and its value directly sets `TurnStatus::MoreWork`; therefore the result page itself drives immediate reactor re-polling. [`plugin/🦀️.rs:24565`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24565) [`reactor/🦀️.rs:1945`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:1945)

This establishes the exact bad sequence:

1. First continuation routes Child/Terminal page `(…, attempt: 1)` and marks it presented.
2. Browser/native has not delivered its shell `Message` ACK by the next poll; continuation rereads the same page as attempt 2.
3. A subsequently delivered attempt-1 ACK is rejected as stale.
4. One more poll converts that retained page to `Fault`, again without a timer or a transport failure observation.

The present test fixture encodes this same pull-count behaviour as expected retry exhaustion; it is not a valid delayed-renderer acceptance law. [`plugin/🦀️.rs:17613`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:17613)

## Independent watchdog fault

The live-maintenance watchdog can also fault an otherwise live ACK wait. Stage 0 calls `drive_worker_step` for every non-retiring operation, including `AwaitingAck`; that method returns `Blocked`. [`plugin/🦀️.rs:24155`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24155) [`plugin/🦀️.rs:16259`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16259) The runtime classifies both `Blocked` and a zero-release `Pending` as structural zero progress and enters `RUNTIME_MAINTENANCE_FAULT` at 256 callbacks. [`plugin/🦀️.rs:29462`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29462) [`plugin/🦀️.rs:28548`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28548)

This is callback-count based, not duration based. The 21-stage app maintenance round-robin creates zero-release callbacks in its empty stages, so the threshold is reached in approximately twelve complete rotations plus four callbacks, not only after 256 operation visits. [`plugin/🦀️.rs:24155`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:24155) The next `plugin_step_live_cleanup` turns that state into an application error. [`plugin/🦀️.rs:30121`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:30121)

The current unit law intentionally preserves a fault for a truly permanent blocked cleanup owner. It must remain; a blanket exemption for `Blocked` would hide the original leak detector. [`plugin/🦀️.rs:29922`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:29922)

## Smallest correct repair

Do **not** credit a zero-release ACK wait as `Pending { released_items: 1, … }`, exempt every `Blocked`, or merely make `plugin_step_live_cleanup` return `READY`. Those choices respectively lie about retirement, mask permanent blocked ownership, or preserve a busy poll.

1. Split *live* from *runnable continuation* in `PluginApp` at [`plugin/🦀️.rs:11712`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:11712). Retain `has_pending_typed_operations` for ownership/close. Add a separate continuation predicate that is false for a presented `AwaitingAck` page, but true for Worker/Publishing/outbox work and for an unpresented result page. Make `pending_typed_operation_instance` use that predicate. [`plugin/🦀️.rs:31250`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:31250)
2. Make `take_result_page` single-delivery: after first presentation it returns `None` until an **explicit retry deadline** permits one retry. Attempt must change only at that deadline, never because reactor output was polled. This preserves exact ACK ownership and makes stale-token denial meaningful.
3. Add a private, fixed-capacity reactor-owned typed-result delivery/deadline registry, keyed by the full result token and instance lifetime. It must retain no arbitrary client input and use the already bounded page inside the app as its authority. On deadline, it requests one exact `retry_typed_operation_result_page(token)` and routes only that page; on accepted ACK, cancellation, close, or generation replacement it removes the entry. Do not reuse guest `Effect::SetTimer`/`ARMED_TIMERS`: that registry is guest-id keyed, collision checked, checkpointed, and has no namespace for host retry authority. [`reactor/🦀️.rs:447`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:447) [`reactor/🦀️.rs:2113`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs:2113)
4. Introduce a typed `AwaitingRendererAck` maintenance outcome, distinct from `PluginCloseStep::Blocked`. It is legal only when the selected page is presented and the app has no eligible non-wait maintenance unit. Map it to a new `RUNTIME_MAINTENANCE_WAITING_ACK` state rather than consume the 256 structural-stall credits. The existing status machine has only READY/QUEUED/RUNNING/FAULT; this requires one new state, not a reinterpretation of `Blocked`. [`plugin/🦀️.rs:28518`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:28518)
5. Wake that state only after the exact ACK is accepted, or its own matching retry deadline/cancellation fires. An invalid, duplicate, cross-instance, stale-attempt, or old-generation ACK must not wake it. The wake needs an epoch/fence: an ACK can be accepted while the maintenance worker is transitioning RUNNING to WAITING. Capture the epoch before the transition; after setting WAITING, re-read it, and set READY if changed. ACK itself increments the epoch and CASes WAITING to READY. Without that two-sided fence an ACK arriving between the worker’s decision and its status store is lost.
6. The reactor must expose the private deadline as an actual host wake, separate from guest timer ids, so a lost transport ACK retries after bounded elapsed time rather than spinning. The shared worker pool supports retained deadline callbacks, but the callback must re-enter the actor through the exact runtime/instance generation path; it must not mutate the app from a pool thread. [`plugin/🖥️host/🦀️.rs:239`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🦀️.rs:239)

The child page remains committed-but-unacknowledged until the matching ACK; it must not become visible as complete simply because delivery was retried. `Child` ACK’s current committed-authority check is the correct disposal boundary to retain. [`plugin/🦀️.rs:16394`](/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs:16394)

## Required schema-first proof

Add a neutral `typed-result-ack-scheduler-v1` corpus and independent Bun/AJV model before the Rust changes. It needs event traces for `workerReady`, `publish`, `poll`, `ack(fullToken)`, `deadline(fullToken)`, `cancel`, and maintenance callback. Pin token attempts, runnable/wait states, maintenance status, and released ownership separately.

Required rows:

- 1,000 normal polls before the deadline: first page stays attempt 1, no retransmission, no fault, no false released credit, and no additional live-maintenance submission once wait is entered.
- Exact current ACK: wakes once, accepts Child, then permits Terminal; terminal ACK reaches Retiring and terminal-empty.
- Retry deadline: exactly one attempt increment/re-send; old attempt ACK is denied without wake; current token ACK is accepted.
- Retry exhaustion occurs only on deadline, produces a fault page under the same bounded delivery policy, and never through poll count.
- 256 real structural `Blocked` cleanup steps still fault. The ACK wait must not weaken that law.
- ACK-vs-WAITING interleavings (before decision, between decision/store, after store), duplicate/cross-instance/stale ACKs, deadline-vs-ACK, pool contention, cancellation, close, and generation replacement preserve the exact page/owner or remove it only through terminal retirement.

Native evidence must include:

1. A real `PluginRuntime` + `RuntimeAppCell` child-operation driver. Route its page through the actual reactor `Event::Message` shell ACK path, not a private stage mutation. Pause the private monotonic deadline, perform >300 polls, and prove Child stays attempt 1 without `RUNTIME_MAINTENANCE_FAULT`; then ACK, see Terminal, ACK, and drive actual close to terminal-empty.
2. A controlled epoch race around the runtime maintenance transition, proving an accepted ACK cannot strand `WAITING_ACK`.
3. The existing permanent-blocked watchdog law unchanged, plus stale/duplicate ACK no-wake cases.
4. Browser and native shell transport tests that delay an ACK across several actor polls, then return the original wire ACK; both must accept it before the deadline. These prove the shared reactor path without claiming rendering correctness.

## Acceptance boundary

The current runtime has no valid delayed-ACK acceptance. Credit the repaired packet only after the neutral corpus, native runtime/lifecycle laws, and both shell transport delay laws pass. This does not establish browser rendering, socket delivery, or child-publication atomicity.
