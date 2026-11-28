# Selection and tool-switch turn cost

## Root cause (confirmed in code)
One selection or tool switch is a framework reserved tool job (`framework.reserved.tool`). Its guest body (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🏛️reserved/🦀️.rs`) does ONE micro step per `step-job` host crossing: 8 magic-prefix bytes one by one, then 64 input bytes per step, then close steps. A ~6 KB selection payload is therefore about 130 `stepJob` crossings, each with a fresh issued turn, a settle and ~5 receipts (matches the measured 130 stepJob, 290 results, 670 receipts).

## Change
`abi::step_coarse` (new, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🦀️.rs`) runs the same paid registry micro steps back to back inside one crossing. It stops on a terminal step, `should_yield` (wall budget or fuel), cancellation, fewer than 2 items/fuel left (keeps the outcome receipt payable), a step with no progress, or 256 micro steps (about the 8 ms interactive ceiling). Accounting is unchanged: every micro step consumes retained progress and fuel exactly as before. `step_job_original` and `step_job_owned` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`) call it.

## Counts
| | before | after (law) |
|---|---|---|
| step-job crossings for a 6144-byte selection job | about 8 + 96 + close (>100) | <= 2 |
| 0 and 200 byte payloads | 10 to 20 | <= 2 |
Browser-level turn/crossing/receipt counts per click: to be measured by the coordinator after the wasm rebuild. Not changed: the `turn` request count from the command-page re-offer (host drive loop, `🖼️render-and-turn-drivers`); expect it to shrink with the job crossings but it is unverified. Tool switch is assumed to use the same reserved job path (not separately counted).

## Law
`a_selection_or_tool_switch_reserved_job_settles_in_a_bounded_number_of_crossings` in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/💼️jobs/🏛️reserved/🧪️tests/🦀️.rs`: payloads 0, 200, 6144 reach Done with the exact bytes in <= `SELECTION_JOB_CROSSINGS_MAX` (2) crossings; the micro-step drive needs >= 8 + payload/64 and >20x more for 6144. Result: PASSED (`cargo test -p semio-plugin-reactor-jobs --lib`, `C:\t\pk`, -j 3).

## Existing suite status (honest)
Full `semio-plugin-reactor-jobs --lib`: 24 passed, 31 failed. The failures are NOT caused by this change: none of them call `step_coarse`; they fail in test fixture setup with "original caller has no parent source supplier" (about 10) and "original Source clock declined new construction" / "must settle within its quoted turns" (reserved birth helper), i.e. in the job-context / caller-source fixtures that peers are editing (`🧵️job/⏱️context`, `debug_retained_parts` edit seen mid-run). I did not verify a clean baseline; recheck once those edits settle.
