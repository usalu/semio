# 🧯️ wgpu job-retirement failures: handoff

## Root cause
The 21 mounted-layout and 88 paint-laws failures (`caller/🦀️.rs:146`, receipt `(0,0)` vs heap `(5376,2688)`) were not a retirement-contract
violation of the UI job. A temporary `[DEBUG]` `eprintln!` in the job framework (`🧵️job/🛬️receiving/🎟️work/🦀️.rs`, `admits`, guarded by
`additional>1000`) fired inside `OriginalRetirementPump::with_effect`, which runs inside `observe_retirement_allocations`.
Under the libtest output capture the print grows a thread-local `Vec<u8>` (realloc 2688 -> 5376 etc.), which the allocator observer
counted as an unreceipted allocation of the retirement step. Removing the print fixed all of them.

## Files changed
- `🧰️framework/🔨️modules/🧵️job/🛬️receiving/🎟️work/🦀️.rs`: removed the temporary `[DEBUG]` print in `admits` (one-line, other agent's debug).
- `🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-mounted-layout-unit/🦀️.rs`: removed a leftover `[DEBUG] admission turn` print.
- `🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🧪️tests/🔬️wgpu-unit/🦀️.rs`: test peer Ada had role Author, whose word is
  "Editing"; React/TS `PresenceBar` (qualifiers = agent word, role word, activity text) and the Rust implementation both include it, so the
  test peer is now role-less (as the agent and Bob are) and the expected string stays without the role word.
- The caller `🧪️testing/🎟️caller/🦀️.rs` carried temporary debug during investigation and is back at its committed content.

## Results
- `semio-ui-wgpu-paint-laws`: 110 passed, 0 failed (no frame/stack/chip law needed an implementation change).
- `semio-ui-wgpu-light`: 29 passed, 0 failed.
- `semio-ui-wgpu-mounted-layout`: 23 passed, 1 failed. `mounted_layout_cancel_before_and_after_owned_text_call_is_typed_and_retained` now passes.

## Left
`mounted_layout_worker_runs_on_the_original_caller_turn` fails in `🧵️job/👷️worker/🧪️testing/🦀️.rs:21` (`OriginalWorkerFixtureCaller::close_owner`,
`assert_eq!(self.receipt(), step.progress())`): the turn receipt is doubled (2 items / 236 bytes vs 1 / 118). The owner returns its alias
through `with_original_effect` and then `receive_original_retained_work` charges the same step again; this is the job framework's
in-flight retirement/receipt accounting (`StepContext` Drop and loan changes), not UI code. Needs the job agent.
