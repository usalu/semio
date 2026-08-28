# Actor Exhaustive Regression R4

Command: `SEMIO_COVERAGE=0 bun x nx run @semio-tech/framework-actor-rs:test --skip-nx-cache --args='exhaustive --lib'`, with the shared warm debug target.

Actual exit1: **61 passed, 1 failed, 50 not run**, 0.548s. All112 were selected; fail-fast stopped the remainder. This is not a full regression PASS.

Exact primary failure:

```text
component::tests::quick::mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged
panicked at actor/🦀️component.rs:1531:13:
mounted replay log requires generation-qualified recovery and incremental close
stack: JobReplayLog::Drop → mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged
Summary [0.548s] 62/112 tests run: 61 passed, 1 failed, 0 skipped
50/112 tests were not run due to test failure
```

Raw: `🧪️member-actor-return-full-r4-native-2026-08-27.txt`. The exact fixture/owner close audit was sent to Dag; production strict guard remains unchanged. Return-codec R2 remains a separate4PASS scope.
