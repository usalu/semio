# Agent lane 5D v1 (lane5d)

## Root cause (by reading, unconfirmed by a run)
After worker admission the operation keeps an empty `RefusedWorkerSource` header. `worker_retirement_demands` quoted its retirement as `size_of_val(rejected)` copy bytes, but `retire_empty` posts only one item. The source holds the 5D job inline, probably larger than the 32768-byte fixture preparation copy grant. `original_lifecycle_step` returns zero progress silently on an unfunded demand, so the `setActiveExample` boot job never advanced (stall5d-14.log, `s=true rej=true/false`).

## Fix
`🧰️framework/.../🔌️plugin/🧵️retained-command/🔄️full-operation/🦀️.rs` `worker_retirement_demands`: empty rejected header with a live session quotes depth 1 only.

## Runs
- lane5d-1.log: started, still building at handback (result unknown).

## Pending
Remove the `[DEBUG] lane5d` probe in `🧵️instance/🔌️plugin-app-impl/🦀️.rs` `typed_operation_diagnostic` after the run.
