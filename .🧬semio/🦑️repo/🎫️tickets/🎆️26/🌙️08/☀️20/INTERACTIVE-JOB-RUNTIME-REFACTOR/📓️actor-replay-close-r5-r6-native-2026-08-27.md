# Actor Replay Close R5–R6

R5 exited0 but selected0 tests/112 skipped because the target defaulted to fundamental and the exact law is in quick. No law credit from R5.

R6 explicitly selected quick:

```text
bun x nx run @semio-tech/framework-actor-rs:test --skip-nx-cache --args='quick --lib mounted_replay_cancel_deadline_and_stale_refuse_the_exact_publication_owner_unchanged -- --nocapture'
Summary [0.036s] 1 test run: 1 passed, 111 skipped
NX Successfully ran target test for project @semio-tech/framework-actor-rs
```

Actual exit0. Dag added only the empty replay log's required begin_close/terminal assertion after the original three exact publication-refusal laws. Guard and production behavior unchanged.

Raw: `🧪️member-actor-replay-close-green-r6-native-2026-08-27.txt`. Explicit exhaustive rerun follows separately.
