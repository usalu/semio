# Actor Exhaustive Regression GREEN R7

Canonical command: `SEMIO_COVERAGE=0 bun x nx run @semio-tech/framework-actor-rs:test --skip-nx-cache --args='exhaustive --lib'`.

Actual exit0, **112 passed / 0 skipped / 0.618s**. Same debug target, no release/coverage graph.

```text
Nextest run with nextest profile: exhaustive
Starting 112 tests across 1 binary
Summary [0.618s] 112 tests run: 112 passed, 0 skipped
NX Successfully ran target test for project @semio-tech/framework-actor-rs
```

Raw: `🧪️member-actor-return-full-r7-native-2026-08-27.txt`. Includes the new return4, existing byte-page3, lifecycle/patch receipt codecs and repaired existing replay-close fixture. Native Actor regression only; no runtime paged return implementation or consumed Wasm inference.
