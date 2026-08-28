# Kernel Message Encoder R1 — Actual Compile RED

Canonical common-host command: `@semio-tech/framework-rs:test-wire-retirement-native --args='--lib return_content_message_ -- --nocapture'`, unchanged master target, explicit `SEMIO_COVERAGE=0`.

Actual output:

```text
error[E0432]: unresolved import `super::return_message`
error: could not compile `semio-framework` (lib test) due to 1 previous error; 19 warnings emitted
```

Session 28599 exited 1 before native tests. This is the intended missing-module RED for the three mounted message encoder laws; it is not message encoding runtime evidence. Raw output: `🧪️member-kernel-return-message-red-r1-native-2026-08-27.txt`. Kernel source released for implementation; no cleanup or generated publication.
