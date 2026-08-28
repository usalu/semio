# Kernel Return Framing R1 — Actual Compile RED

Canonical command: `bun x nx run @semio-tech/framework-rs:test-wire-retirement-native --skip-nx-cache --args='--lib return_content_framing_ -- --nocapture'`, using the existing single-job master Cargo target and unchanged build environment.

The proper common `semio-framework` host crate failed before tests with the intended missing `return_content` import. No native framing test executed. The snapshot includes the final partial-header `finish()` rejection assertions. Runtime comparison source was not part of this dependency graph.

Actual captured output:

```text
error[E0432]: unresolved import `super::return_content`
error: could not compile `semio-framework` (lib test) due to 1 previous error; 18 warnings emitted
NX Running target test-wire-retirement-native for project @semio-tech/framework-rs failed
```

Process session 69343 exited 1. Full retained output: `🧪️member-kernel-return-framing-red-r1-native-2026-08-27.txt`. Actor/Kernel production source hold released to the owning lane for implementation. No cleanup or generated-output publication was performed.
