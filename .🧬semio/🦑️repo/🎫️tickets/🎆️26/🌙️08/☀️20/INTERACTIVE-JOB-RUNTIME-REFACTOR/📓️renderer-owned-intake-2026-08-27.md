# Owned Native Patch Intake

## Concrete Integration API

`UiDocumentStore/📥️intake/🟦️component.ts` now exports `OwnedUiPatchIntake(owner, source)` with the exact API specified in `📓️renderer-live-owned-source-contract-2026-08-27.md`: `advance`, `failure`, `peekAcknowledgement`, `acceptAcknowledgement`, `takeSurface`, `beginClose`, `closeStep`, and `terminalIsEmpty`.

The constructor validates `OwnedNativeUiPatchAuthority.matchesOwner` before touching source metadata or transferring input. No actor-name lookup, operation array copy, BuiltNode or parallel content tree is introduced. `takeSurface` returns the original frozen facade once, only after actual successful native publication receipt and local patch retirement. It does not close the aggregate. The original native source remains owned by the lifecycle output participant; this cursor only retires its own alias.

## Retained Phases and Accounting

Each advance drives one child step or one fixed state transition. Lookup creation, lookup traversal, facade transfer, lookup close, patch construction, one-operation offer, decoder/application advance, exact input release, sealing, publication, ACK waiting and patch close are separate phases. The cursor neither loops over all operations nor accumulates fictional byte credits.

Child steps forward their actual `items` and `bytes` unchanged. Fixed transfer records account 32–256 bytes. Patch/known-input construction each use a dedicated 2048-byte bounded-metadata step. The child-ID constructor additionally reserves at most 128 numeric slots (1024 logical bytes), with the remaining 1024 bytes covering the fixed ownership/cursor fields; its native schema already caps this field. Packed node/value constructors retain fixed cursor metadata and transfer the buffer without traversing its payload. Actual variable payload decoding remains with the existing byte cursor. The lookup-validation step additionally charges `5 * source.surface.length`: up to two input UTF-16 bytes plus three output UTF-8 bytes per code unit, plus 256 fixed bytes. `beginSurfaceLookup` rejects strings longer than 512 UTF-16 code units before encoding, so this successful step is at most 2816 bytes. A failed fixed transition consumes the entire admitted 4096-byte opportunity conservatively; it never raises the grant or retries by accumulating allowance.

These are explicit logical traversal/copy bounds, not measurements of JavaScript VM allocation size or a complete outer-poll latency certificate. The lifecycle scheduler must still apply its real deadline before offering another step. Whole WIT lifting/structured-clone/raw output retirement remains a separately named producer boundary.

## Cancellation and Failure

Cancellation before input acceptance transfers nothing. Accepted input is closed and discharged only by the existing private source/ordinal/original-operation token. Late cancellation after paired publication continues the committed notification/receipt obligation. The caller uses its dedicated old-lifecycle ACK authority; transport refusal leaves the same token available. Aggregate close and intake close can alternate without reusing a retired child handle.

The coordinator identified that initial close forwarding overwrote a child's `rejected` with `pending`. A neutral fault fixture and real post-publication child-close injection reproduced this exactly. The corrected forwarding preserves `rejected`, phase and byte counts and records a persistent diagnostic while retaining the original owner. It preserves `blocked` as well. No ACK or terminal witness is fabricated to bypass a fault. The test's injected failure is restored explicitly after checking retention; real permanent committed faults remain retained failures, not silently recoverable successes.

Failed consumer notifications stay owned by the underlying Surface. A narrow facade `retryNotification(exactSubscription)` forwards the existing explicit retry authority and queues the same aggregate's maintenance; it does not allocate another consumer or replace a snapshot. The new callback-failure law verifies retry before a subsequent publication and rejects a duplicate retry.

## Evidence

Canonical command: `bun x nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache --args='--run -t OwnedIntake'`.

| Run | Actual result |
| --- | --- |
| R1 | Four collection failures, zero executed test bodies, one skipped; 4.29s. New module did not exist. This is import-stage TDD, not behavioral rejection evidence. |
| R2 | Two passing, 618 skipped, 620 discovered; 9.65s. Exact source/lookup/ACK and thirteen cancellation prefixes. |
| R3 | Three passing, 618 skipped, 621 discovered; 7.51s. Added actual cancellation after root publication but before notification/ACK, with alternating intake/aggregate close and refused old-activation ACK retry. |
| R4 | Three passing, one failing, 618 skipped, 622 discovered; 7.54s. Injected committed close rejection was incorrectly returned as pending. |
| R5 | Four passing, 618 skipped, 622 discovered; 8.92s. Close rejection and diagnostic preserved. |
| R6 | Four passing, one failing, 618 skipped, 623 discovered; 9.60s. Actual failed consumer reached missing facade retry method. |
| R7 | Five passing, 618 skipped, 623 discovered; 5.21s. Exact facade notification retry and a later successful publication both executed. |
| R8 | Five passing, 618 skipped, 623 discovered; 7.88s. Repeated after conservative fixed construction accounting was raised to include the bounded child-slot reservation. |

Canonical `@semio-tech/framework-renderer-react:typecheck` strict R1 exited one with exactly seven diagnostics in the complete captured output: Demonstrator brand missing the pending tutorial field, one tutorial fixture, and five ShellHelpers old tutorial-selection joins. No intake, root hook, fixture self-reference or owned Surface diagnostic remains. These seven joins are assigned to Dag and were not suppressed or replaced with defaults. Full output is `🧪️renderer-owned-intake-strict-r1-2026-08-27.txt`.

The fixtures use strict Ajv and Immer/JSON expected results. Tests instantiate actual ShardClient lifecycle/native patch authorities with a controlled worker transport; no fabricated source token or structural retirement witness is accepted. Full outputs remain in `🧪️renderer-owned-intake-rN-2026-08-27.txt`.

## Remaining Joins

The producer's new `ActorUiPatchReceipt` must be captured from the actual native authority when its canonical source API lands; no local patch sequence is invented. Native `PatchRejected` recovery needs that producer handshake and is not implied by `failure`. The production per-turn scheduler, response projection, ShellHost/Interpreter consumer cutover and TypeScript WGPU-web consumer remain unwired. All fifteen host behaviors and nested JSON/generic-pack preparation remain in scope. No production create/destroy/scheduler region, Rust source, ticket cleanup or modifying git operation was changed here.
