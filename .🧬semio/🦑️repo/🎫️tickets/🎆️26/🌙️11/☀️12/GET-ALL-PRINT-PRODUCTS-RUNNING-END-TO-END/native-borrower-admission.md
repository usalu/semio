# Native borrower admission failure after a many-task run

## Root cause
`withNativeImageBorrowerOperationV1` (borrower module) refused every outermost operation synchronously once the borrower had
`maximumPendingOperations` (32, `💾️storage/🧹️cleanup/📥️operation/🧬️schema/🔣️.json`) operations pending. Callers that issue
one operation per task (cache entry verify/restore for ~86 tasks, near-simultaneously, each costing 2 pending entries
through journal + nested recheck) overran the table. The refusal rejected the caller's own async operation; Nx treated that as a
recoverable cache miss, but every rejected accepted operation is recorded in `scope.failures`, and `closeBorrower` rethrows
them as an `AggregateError` after the confirmed release. Hence all tasks succeed and the invocation fails in teardown, only for
large, mostly cache-hit plans (intermittent, plan-size dependent). The bound is a correct finite schema limit; the defect was
refusing instead of waiting at an outermost call.

## Fix (borrower module only)
- Outermost calls (no enclosing frame) that find the pending table full now queue in FIFO order and are admitted when a slot
  frees (`pump` on every operation settle). Peak concurrency stays <= 32, so the finite custody law is unchanged.
- Nested calls keep synchronous refusal at the nested bound (queueing them could deadlock a parent on its own child).
- Closing/stopping admission (`open=false`) flushes the queue; each queued call is then refused by the unchanged closed-borrower
  law ("admission closed or full"), and a released owner still throws synchronously. Genuinely closed borrowers still refuse.
- The refusal of an operation now fails exactly the caller that issued it (rejection of its promise); with queueing, an open
  borrower no longer produces the refusal at all, so nothing is left to surface only at close.

## Files touched (exact)
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🟦️.ts` (implementation)
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🧪️tests/🟦️.ts` (new test)
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🧪️tests/🔣️.json` (language-agnostic `admission` fixture: 100 operations, cap 32, all complete, closed refuses)
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📥️borrower/🧪️tests/🧬️schema/🔣️.json` (schema for the fixture)

## Tests
`repo tools exec --light bun -- test` borrower tests: 6 pass, 0 fail (new: fan-out of 100 > 32 operations all complete in order with
peak <= 32; queued call rejected when the borrower closes; closed borrower refuses). Operation-admission tests
(`🧹️cleanup/📥️operation/🧪️tests`): 11 pass, 0 fail.

## Real run
`repo run @semio-tech/print:test-viz-full --lang=en --output=stream --skip-remote-cache` started in the background after the edit
(log `🗑️generated/borrower-fix-viz-full.log`, exit `🗑️generated/borrower-fix-viz-full.exit`). It was still re-planning and
waiting for memory headroom when this note was written; result below if recorded.

Result of the real run: exit 1 before any task started. The partition `/` is at 97.2% used (limit 97%, 6.38 GiB free); the repo refused to start a build or cache write and asks for `repo cache cas gc`. Not related to the borrower. The run must be repeated after freeing space (not done by me, since gc evicts shared caches).
