# Original work scope drop panic (5D v1)

## Root cause (proved by owscope-2.log)
`OriginalWorkPerformedReceipt::performed(delta)` set `performed=true` even for `delta==0`. 5D's `prepare_child_effect` records `effect.performed(0)` on its Pending, ready and idle branches. The claim then created a zero-delta partial receipt and set `partial_seen`, so `StepContext::with_original_effect_at` (performed==0 branch) called `cancel_for_action`. The issuer refuses to cancel an action with `partial_seen`, so the cancel failed with "original work action requires its same live scope and issuer". That error left the action open, and `OriginalWorkScope::drop` asserted. Debug run: all 9 failures reported `stage 4 performed 0` (the cancel step).

## Fix
1. `🧰️framework/🔨️modules/🌱️value/🎟️control/🧬️contract/🎟️work-action/⚙️operation/🦀️.rs`: `performed` now records `self.performed|=delta!=0`, so a zero-unit report is no work and the action cancels cleanly.
2. `🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs` (`with_original_effect_at`): receipt and partial slots moved out of the exchange closure. Every error exit now claims the receipt, settles a pending partial, commits and finishes (or cancels) the action, so the scope settles on every exit path. The Drop assertion is untouched. (Another agent concurrently changed the cancel guard in that function to `!work.performed_seen_for_action(action)?`; it is compatible with the fix.)
All `[DEBUG] owscope` logging removed.

## Test status
- owscope-1.log: scope panic gone (error path now settles); still 9 failures with the underlying refusal.
- owscope-2.log: debug run, identified stage 4 / performed 0.
- owscope-3 to owscope-16: the fix is not yet verified. Builds fail in other agents' in-flight work (pdf-lift/xref_stream imports, pdf-tests-2 Source boundary 4030/4000 lines, tool-machine borrow errors, mesh-reconstruction/brep-tessellation imports). The last retry loop was killed for low system memory.
- `semio-hub-puzzle --lib surface` not re-run.

## Next
Re-run `repo cargo ... test -p semio-s-artifact-puzzle-5d-v1 --lib -- puzzle5d` once the pdf/mesh crates compile, then the hub surface test.
