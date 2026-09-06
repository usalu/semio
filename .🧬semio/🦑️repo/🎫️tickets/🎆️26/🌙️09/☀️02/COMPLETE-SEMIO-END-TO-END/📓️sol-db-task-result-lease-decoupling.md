# DB Task and Result Lease Decoupling

Status: source/oracle and current 38-law native writer group green.

## Contract

- A terminal task slot returns after backend/task cleanup even while its caller retains the result's aggregate-operation lease.
- Result handback returns only its exact operation lease. It checks the complete `(slot, generation, operation)` identity before requesting task close, so an old result cannot affect a reused slot.
- `DbIoTaskOperation::finish` transfers the result before waiting for task retirement. Fault conversion drains its retained result handback rather than parking an otherwise invisible owner.
- Dropped result owners request mounted DB-I/O maintenance. The fixed backend maintenance controller drains lost page/list/fault/result owners without requiring unrelated task ingress.

## Neutral and Native Boundary

The memory-backing fixture now requires:

- 128 later operations while an earlier result remains retained;
- 44 simultaneously retained one-page `DbIoPages` results;
- exact task-slot reuse before the oldest page result closes;
- generation/operation/phase invariance of that reused live slot across the stale handback;
- full page, shell, item, control, and result-lease credit return to the pre-retention ledger baseline.

The Bun schema/source gate `@semio-tech/framework-os-kernel:wal-writer-authority-check` is green with `AJV=6`, nine backend-pool-use vectors, and the new fixed page-result values. The Rust selector is `db_io_retained_page_results_survive_same_task_slot_reuse_and_return_exact_credit` and is registered in the existing writer native group.

## Evidence

- Source/oracle gate: green on 2026-09-06.
- `exact-cargo-laws-Ni5nPn/00` passed the first 15 laws and then rejected the prior list-credit assertion. The task/result split had returned all transient list credit with the task even though one fixed output backing remained caller-owned.
- The retained handback now carries its exact retained resource credit. A list transfers one fixed backing/item partition from its task aggregate to the result handback; task retirement returns only task/transient credit, and result close returns backing plus result-lease credit. Page owners keep their existing page/shell operation credit.
- `exact-cargo-laws-lNzCZB/00` reached the stale-controller law and exposed a transient first `try_submit` contention in the fixture. The fixture now retries admission with the exact returned `Job` under its existing bound.
- `exact-cargo-laws-MtqsJR/00` passed the new page law and first 33 laws, then exposed a rollback terminal-ACK race: the rejected-backend generation became absent just before its local `WorkerPoolUse` was dropped. Terminal cleanup now fences the slot as scheduled, drops that use, then clears the exact generation.
- Final receipt `wal-writer-authority-exact/exact-cargo-laws-aKflPX/00`: 38/38 exact native laws green. Executable SHA-256: `2885e0fc2d788ea28c8c9fdef86242cfa090f133049365fd8693c56f5cca62e0`.
