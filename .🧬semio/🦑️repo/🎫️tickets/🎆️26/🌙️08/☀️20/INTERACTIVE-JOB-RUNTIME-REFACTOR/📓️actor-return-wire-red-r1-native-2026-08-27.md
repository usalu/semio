# Actor Return Wire RED R1

Canonical command: `bun x nx run @semio-tech/framework-actor-rs:test --skip-nx-cache --args='--lib actor_return_wire_ -- --nocapture'`, using the existing shared native target and one compiler.

Actual exit 1 before test execution. Exact captured diagnostic:

```text
error[E0432]: unresolved import `crate::return_page`
error: could not compile `semio-framework-actor` (lib test) due to 1 previous error; 2 warnings emitted
NX Running target test for project @semio-tech/framework-actor-rs failed
```

Raw: `🧪️member-actor-return-wire-red-r1-native-2026-08-27.txt`. This is the intended missing-module compile RED, not a semantic/runtime execution. Actor source is released to its owner for implementation. No compiler remains active.
