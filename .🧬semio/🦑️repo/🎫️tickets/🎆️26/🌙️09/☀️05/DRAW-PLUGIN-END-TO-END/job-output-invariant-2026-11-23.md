# Job Output Invariant (2026-11-23)

## Root cause (confirmed by a per-crossing trace)
The guest keeps one job-output slot per actor (`OriginalActorJobState`: `identity`, `output`, `ack`). `step_on_source` (`retained-turn/💼️job/🏠️state/🦀️.rs`) refuses any step while that slot holds another job's output.

The WIT/owned `step-job` crossing (`step_job_original` and `step_job_owned` in `⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`) only reported the outcome of a held output on the next call of the same job. The first call produced the output, stored it in the slot and answered `outcome: none`, which the host reads as "running, nothing to collect". The scheduler then stepped the next job. Trace of the failing run (job ids shortened):

```
stepJob ..530 -> outcome none      (output now held by the guest, unreported)
stepJob ..529 -> InvariantViolated (slot holds ..530)
```

Earlier rounds repeated `stepJob ..530 none, stepJob ..530 running 8, read.., ack` and only worked while a single job was live; the second job exposed it. Every later turn failed because the fault fences the producer.

## Fix
Guest (`⚛️reactor/🎟️context/🌉️call/🎭️exports/🦀️.rs`): new `held_outcome` reports the held output's metadata (`state`, `byte_length`) in the same crossing that produced it, for both `step_job_original` and `step_job_owned`. The host then collects and acknowledges before it steps another job. No wire or schema change.

Host (`retained-turn/🟦️.ts`, `OriginalJobFunds`), a robustness fix for the same single-slot law: the output hold records the step outcome, `step` resumes an interrupted collection or acknowledgement instead of stepping a job over its own held output, and `cancel` drains an unfinished collection before crossing `cancelJob`. Tests in `retained-turn/🧪️tests/💼️job-funds/🟦️.ts` use a strict mock guest that enforces the Rust invariant.

## Verification
- TS: `repo test-file` on `job-funds` 15/15 and `wit-lowering` 5/5 pass.
- Rust: `repo cargo --store=isolated --configuration=prototype -- check -p semio-plugin-reactor-core` passes. No native unit test of the interleaving was added (it needs the full lane harness).
- Browser: after rebuilding the component (`component-a6`), prepare, activate and a server restart, both journeys (`journey-a6b-{en,de}`) show 0 `InvariantViolated` errors and 0 `Draw guest fault`, where before there were 23 errors, the fault chain and `input #2 setCamera refused`.
- The journeys still end in an `AssertionError` (`9 !== 0`) on console errors. The only console errors are `[DEBUG] scope body ...`, `[DEBUG] canonical source zero progress ...` and `[DEBUG] sealer blocked: zero progress ...` lines from other agents' temporary Rust logs, so that is a separate issue.

## Notes
- `prepare` and `activate` do not regenerate `🌉️bridge.js`; the `component` stage does (it embeds `retained-turn/🟦️.ts`). For host-only TS iteration, regenerate it with `pluginComponentBridgeSource(...)` and copy it over `.cache/b/p/m/browser/prototype/modules/🖍️draw/🌉️bridge.js`; the server serves it immediately.
- All `[DEBUG]` job logs added for the trace are removed again.
