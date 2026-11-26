# Sourcing boot fault: live cleanup `zero-progress` (elapsed 0us, ceiling 8000us)

## Cause
Not a clock/deadline defect and not a boot rung. The hub built 2026-11-26 10:12 carries two working-tree changes of that window:
1. `drive_turn_retirement` (`plugin/⚛️reactor/🔄️turn/🪜️entry-lifecycle-and-retirement`, 04:05) repeats the live maintenance pump up to
   `LIVE_CLEANUP_PRESSURE_STEPS_PER_TURN` (32) times per ingress-free turn while a funded document archive is runnable (the sourcing boot loads
   its document archive).
2. The LIVE verdict `runtime_live_cleanup_nonterminal_status` still used the count-only rule (256 consecutive empty `Progress` or `Blocked`
   readings fault). 32 repeats per turn reach 256 fruitless readings within microseconds; the message quotes the last turn cost (0us), not a stall.
The 22:58 build predates (1), hence it booted.

## Fix (landed in the tree 11:53 by the XLSX ticket, root cause shared: `COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR/claude-cleanup-pump-stall.md`)
The live verdict is priced in two currencies like the close verdict: 256 fruitless readings AND `RUNTIME_CLOSE_STALL_CREDIT_US` (8 ms) of real
time since the pump last moved; no clock never faults; a fault quotes the credit spent. File:
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs` (+ `…/📥️staging/♻️reactor-lifecycle/🛎️app-cell/🦀️.rs` fields).
No framework edit by the sourcing coordinator was needed.

## Evidence
See the "Evidence" section below (hub rebuild with the fix and the 17-step journey).
