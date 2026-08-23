# Terra Re-Audit: P1n ShardExecutor Bounded-Turn Handoff — 2026-08-23

## Verdict

**REJECT** for the remediated source packet.

The remediation genuinely closes several findings from the preceding rejection:
the retained-drive waker is no longer a no-op, quiet-ingress rejection has a
finite generation-keyed timer retry, registration is one fixed FIFO owner per
closure, and terminal successor handoff exposes take/resume methods. The
required static checks pass.

Two live failure paths still violate the stated no-hot-resubmit,
one-owner-close, and fixed-authority guarantees:

1. A malformed or permanently over-capacity received frame becomes
   ShardDrive::Fault. The executor never advances consumed_epoch for Fault, then
   immediately schedules again because consumed_epoch remains behind epoch. This
   is a hot resubmit loop after precisely the terminal failure the packet says
   it retains.
2. closed only stops schedule. send_frame still accepts and sends every later
   frame into the unbounded ThreadTransport channel, then schedule returns.
   Those owners have neither a rejection result nor a close/retrieval path.

The 16 MiB fixed-owner claim also does not bound the complete frame path:
preflight covers queued Event, JobStep, and Cancel authorities but not the raw
frame length or Suspend/Resume. A single Grant can therefore carry arbitrary
Suspend/Resume envelopes, each processed in its unbounded per-envelope loop
before the one actor-turn/job-step selection.

This audit is source-only. It makes no claim about compilation, execution,
timing, actual timer firing, or Phase 1 acceptance.

## Scope

Read AGENTS.md, the prior Terra rejection audit, updated
📓️p1n-shard-executor-bounded-turn-handoff-2026-08-23.md, current production
source, static fixture source, and working/staged diffs. No production source,
script, manifest, lock, coordinator, checklist, ticket metadata, or lifecycle
state was changed. Cargo, Nx, Wasm, browser, network, and root lint were not
run.

The live packet is working-tree source plus an already-staged subset of
📜️script.ts. All findings are against current working source; no ownership is
assigned to concurrent changes.

## Requested Static Gates

| Gate                                          | Result            | Evidence                                                                                                                                                                                                                                                             |
| --------------------------------------------- | ----------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Scoped formatting                             | **PASS**          | rustfmt --edition 2021 --check on async pool, actor transport, shard component, and executor exited 0.                                                                                                                                                               |
| Root interactivity scan                       | **PASS**          | bun ./📜️script.ts verify interactivity exited 0 in deny mode. Its only finding is the existing expected test-only allowlist record.                                                                                                                                  |
| Adversarial source fixtures                   | **PASS, limited** | bun ./📜️script.ts verify interactivity --self-test exited 0. The verifier runs its twelve shard fixtures before scanning.                                                                                                                                            |
| Targeted production scan                      | **PASS, narrow**  | Executor run contains no block_on, blocking receive, byte-drain loop, epoch loop, or Blocked successor branch. Its sole production .recv().await occurrence is SharedThreadTransport's trait forwarding; concrete ThreadTransport::recv is non-parking try_recv_now. |
| No added executor thread/runtime              | **PASS**          | The production executor invokes only WorkerPool callback_at; its std::thread::spawn match is inside #[cfg(test)].                                                                                                                                                    |
| Working, staged, HEAD, and scoped diff checks | **PASS**          | git diff --check, git diff --cached --check, git diff HEAD --check, and the scoped diff check all exited 0.                                                                                                                                                          |

Clean scans are not ownership proof. The root verifier is lexical and its
successful fixtures do not exercise the failure/terminal sequences below.

## Prior-Blocker Closure Review

| Previous finding                                     | Result                               | Source evidence                                                                                                                                                                                                                                                    |
| ---------------------------------------------------- | ------------------------------------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| No-op retained-drive waker                           | **CLOSED structurally**              | ShardDriveWake holds Weak<ShardExecutor> plus a drive generation and invokes request_drive_wake in wake and wake_by_ref (executor.rs:196-212). The wake claim validates generation and one-shots the queued transition (:451-458).                                 |
| Pending drive immediate resubmit                     | **CLOSED on the Pending branch**     | When a retained future polls Pending, run clears scheduled, marks drive_waiting, and parks unless a raced wake has already claimed the one schedule (:506-512). It does not manufacture ShardDrive::Blocked.                                                       |
| Quiet-ingress rejection liveness                     | **CLOSED structurally**              | Contended/Saturated preserves rejected.into_job(), arms one callback_at timer, generation-checks it, and bounds retries to eight before terminalizing (:392-443). callback_at is part of the existing process WorkerPool timer wheel, not a new runtime or thread. |
| Shutdown/Poisoned/exhausted successor owner          | **CLOSED for the successor closure** | terminal_handoff retains kind, lane, and exact PoolJob; take_terminal_handoff and resume_terminal_handoff transfer one owner (:317-333, :403-449).                                                                                                                 |
| Dynamic deferred registrations and all-at-once drain | **CLOSED structurally**              | registrations is FixedOwnerRing with 256 fixed slots, and run pops at most one before creating the drive (:185-189, :482-504).                                                                                                                                     |
| Dynamic event/job/cancel queues                      | **CLOSED at the ring primitive**     | Four deferred rings use FixedOwnerRing with 256 slots and 16 MiB byte capacity. try_push returns the exact rejected owner and OwnerKey has a per-slot generation (:328-428).                                                                                       |

The structural closures above are not enough for acceptance because the new
terminal paths do not reach a quiescent, owned state.

## Rejection Evidence

### Fault Does Not Acknowledge a Consumed Frame

send_frame increments epoch after passing a frame to the channel
(executor.rs:350-354). ShardLoop sets last_drive_consumed_frame when it reads
that frame (:604-618). A decode error or a permanently full frame returns
ShardDrive::Fault (:827-837), but executor only classifies consumed_frame for
Idle or MoreWork (:514-518). It then sees consumed_epoch < epoch and calls
schedule (:527-535).

A concrete source sequence is a frame containing 257 event authorities or one
event exceeding the 16 MiB deferred byte cap:

1. preflight rejects it;
2. with empty deferred rings, consume_frame saves terminal_frame then returns
   Fault (:830-837);
3. run retains a failure but neither closes nor acknowledges the frame;
4. the next worker closure finds no frame, returns Idle without consumption,
   observes the same epoch deficit, and resubmits again.

take_terminal_frame does not alter epoch, close, or the retained drive
condition. This is a production hot-resubmit loop; it is distinct from, and
not caught by, the remediated Pending/waker branch.

### Terminal Close Still Accepts Unowned Ingress

terminalize_handoff sets closed (:445-449), and schedule returns immediately
when closed (:378-381). send_frame does not test closed before
kernel_side.send, epoch increment, or lane update (:350-354). Every frame after
a terminal shutdown, poison, retry exhaustion, terminal failure, or terminal
frame is therefore accepted by the unbounded native mpsc transport but cannot
be scheduled. The void send_frame API returns no rejected frame owner.

terminal_frame and terminal_completion are each a single Option. terminal_frame
is assigned without an occupancy check (:830-835); terminal completion replaces
a prior value behind debug_assert only (:452-460). Because terminal conditions
do not close the executor, a later terminal input can overwrite and drop the
first exact owner in release builds.

### The Authority Cap Does Not Cover the Drive

FixedOwnerRing itself correctly caps stored slots and ring-accounted bytes.
The integration preflight counts only Event, JobStep, and Cancel payloads
(shard component.rs:866-897). It does not cap raw frame bytes, the number of
Suspend/Resume authorities, or a Resume checkpoint's state bytes. consume_frame
then loops every Grant envelope (:848-855); Suspend and Resume invoke
checkpoint or restore directly (:938-959) before the later one-event/one-job
selection.

Thus one accepted transport frame can cause arbitrarily many lifecycle
operations and allocations. The packet satisfies the narrow one selected actor
turn or selected job-step shape, but not the claimed bounded worker closure or
complete 16 MiB authority admission.

### ABA and Preflight Checks Are Not End-to-End Ownership Proof

The primitive returns OwnerKey and the cfg(test) test verifies slot-reuse
generation discrimination. Production owners discard every successful key and
only pop FIFO; no production operation accepts or validates a key. There is no
live stale-key addressing path to test. This makes the primitive's ABA test a
useful data-structure check, not proof that the packet's lifecycle has a
checked ABA transition.

Likewise, dispatch_envelope assumes preflight and uses debug_assert that
try_push succeeded (:926-935, :968-972). Release behavior returns no exact
owner if that assumption is violated. The omitted lifecycle/raw-frame classes
mean preflight is not a complete admission boundary.

## Fixture Assessment

The root source fixtures meaningfully mutate several real rejection shapes:
no waker callback, duplicate wake claim, no timer retry, no terminal resume,
dynamic ring, no byte check, missing generation token, all-registration drain,
and multi-cancel drain. The direct Rust fixture source also asserts item/+1,
byte/+1, and ring slot reuse.

They do not contain a Fault-with-unacknowledged-epoch fixture, terminal ingress
after closed, terminal Option replacement, or an oversized/multi-Suspend Grant
fixture. The static self-test's clean result therefore cannot establish the
failed runtime-owner sequences above. Rust test source was not executed under
this audit's command boundary.

## Required Repair Before Another Audit

1. Make every consumed frame terminally acknowledged or explicitly close its
   executor on Fault. A terminal frame/failure must not leave
   consumed_epoch behind epoch or schedule an idle successor.
2. Make terminal close ingress fail closed: do not enqueue after closed.
   Return the exact input owner through a bounded terminal/rejected interface,
   and prevent replacement of an existing terminal owner.
3. Preflight the complete frame, including raw byte size and every envelope
   kind. Defer or bound Suspend/Resume so one drive cannot run an unbounded
   lifecycle loop.
4. Replace debug-only ownership assumptions with release-effective checked
   handback/terminal behavior.
5. Either apply OwnerKey validation to a real production lifecycle transition
   or state that no external keyed transition exists and remove the ABA claim.
6. Add executable tests for fault acknowledgement/no hot resubmit, terminal
   ingress handback, no terminal overwrite, oversized and multi-lifecycle
   Grants, timer retry/exhaustion, and wake storms.

## Other Phase 1 Blockers

Outside P1n, the prior readiness audit's MCP transport extra runtime and
store-sync nested actor block_on remain. Fresh native runtime evidence for the
worker/thread census, permit reserve/release, timer retry behavior, and
plugin-host synthetic path is also still required. None was run here.

## Commands Run

```sh
rustfmt --edition 2021 --check '🧰️framework/🔨️modules/⏳️async/🦀️component.rs' '🧰️framework/🔨️modules/🎭️actor/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🦀️component.rs' '🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🏃️executor.rs'
bun ./📜️script.ts verify interactivity
bun ./📜️script.ts verify interactivity --self-test
git diff --check
git diff --cached --check
git diff HEAD --check
```

No command outside the permitted read-only/static boundary was run.
