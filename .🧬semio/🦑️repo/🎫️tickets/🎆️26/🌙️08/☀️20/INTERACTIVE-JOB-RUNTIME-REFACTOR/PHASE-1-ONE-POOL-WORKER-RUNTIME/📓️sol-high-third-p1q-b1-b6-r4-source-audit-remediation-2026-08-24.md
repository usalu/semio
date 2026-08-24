# Third P1q B1–B6 and R4 Source-Audit Remediation

Date: 2026-08-24  
Agent: Sol High  
Disposition: source-audit-ready

## Counterexample-to-Fix Mapping

| Third-audit counterexample | Bounded repair | Exact evidence |
| --- | --- | --- |
| Saturated retirement abandoned an owner | Removed every scoped production mem::forget. Aggregate and R4 composite retirement now checks primary, fixed overflow, and fixed quarantine, raises a pressure fault, and returns the exact owner if all fixed authorities are occupied. Composite recovery returns leaf authorities without overwrite. Page/platform ledgers retain exact generation-qualified handles. | DB_IO_LOST_OWNER_OVERFLOW, DB_IO_LOST_OWNER_QUARANTINE, and the four R4 quarantine ledgers |
| Storage bulk copy/close helpers completed a whole owner behind Ready-only pseudo-yields | Replaced writer sealing, observed byte copy, list transfer, platform close, and platform-slice copy with retained Futures that perform one scalar/page/close transition, arm the waker, and return Pending. Replaced equivalent owned R4 Ready-only pseudo-yields in artifact/query/engine/Neo with yield_once. | DbIoPageWriterSeal, DbIoObservedBytesWrite, DbIoListTransfer, DbIoPlatformClose, DbIoPlatformSlicesCopy |
| Lost PostgreSQL close could poll pool.close from maintenance with a noop waker | Registered and rejected backend close now persists generation-qualified scheduling state and submits only typed Lane::Io jobs. Only those jobs pass their task Context to close_backend_step. Registry locks are released before the external future poll, including synchronous wake. Facade, Drop, and maintenance only request scheduling. | db_io_poll_backend_close_on_lane_io, db_io_poll_rejected_backend_on_lane_io, db_io_request_backend_close |
| Hostile laws were names or tautologies | Laws now assert task-driver and backend-close worker role/thread, max+1 and max+2 owner identity, overflow promotion/reclaim, accepted/cancel/deadline artifact exits, Ready/Pending/fault storage interruption, CLI Pending-to-Ready record/batch exits, and real replay/snapshot/migration/corrupt command paths. | Laws listed below |
| Verifier was false-green | The isolated verifier now rejects forget/ManuallyDrop, noop external-close wakers, direct maintenance close polling, Ready-only cursor/pseudo-yield bodies, missing checked overflow/quarantine returns, and missing exact hostile-law body evidence. Its mutations alter bodies and prohibited production patterns, not only law names. | interactivityDbIoB1B6Failures and interactivityP1qR4Failures self-tests |

## Hostile-Law Body Evidence

- db_io_actual_async_driver_future_is_polled_by_the_shared_io_worker records the actual async driver and backend-close thread IDs and worker role, asserting both differ from the caller.
- db_io_lost_owner_fixed_ring_max_plus_one_returns_the_exact_candidate fills the primary ledger, installs two named candidates, asserts exact overflow identity, promotes both, and reaches empty.
- db_io_storage_ready_and_pending_close_interruption_recover_the_same_owner_and_ledger manually polls Pending and Ready platform closes, drops a Pending owner, exercises observed-copy interruption and observed-capacity fault, then asserts the exact ledger witness.
- The artifact law executes accepted close, cancelled refusal, deadline refusal, Drop interruption, primary saturation, two exact overflow source pointers, promotion, and reclaim.
- The compaction law executes fixed max refusal, two exact page-operation overflow owners, promotion, mounted close, and terminal credit return.
- Query and engine interruption laws fill their primary retirement ledgers, retain two exact max+1/max+2 identities, promote, and reclaim.
- The CLI law manually polls retained record and batch futures through Pending to exact Ready witnesses; verifier-bound CLI full-cycle, torn-WAL, and migration laws cover replay, snapshot, fault, repair, and migration production exits.

## Validation

- bun ./📜️script.ts verify interactivity p1q-b1-b6 — passed: live-source and hostile mutations clean.
- Scoped rustfmt --edition 2021 --check over storage, PostgreSQL, Neo4j, SQLite, artifact, compact, query, engine, and CLI — passed.
- Scoped git diff --check over the P1q verifier and the same Rust sources — passed.
- Scoped source gate found no production mem::forget, retirement-capacity expect, permanent-retained fault marker, backend production noop waker, or Ready-only pseudo-yield.

No Cargo, Nx, Wasm, browser, database, or runtime command was run. These are source-only validation claims.
