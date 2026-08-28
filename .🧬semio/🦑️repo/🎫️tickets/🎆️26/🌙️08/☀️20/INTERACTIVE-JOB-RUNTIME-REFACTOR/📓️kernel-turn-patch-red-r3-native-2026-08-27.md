# Kernel Turn Patch Normal-Close Contention — Native RED R3

The correct common Kernel host crate executed the exact normal-close law. Production remained unchanged through this snapshot.

Command: `bun x nx run @semio-tech/framework-rs:test-wire-retirement-native --skip-nx-cache --args='--lib ui_turn_patch_owner_normal_close_does_not_wait_for_arena -- --nocapture'` with the existing master-ticket target and artifact directories.

Actual result: exit 1; 0 passed, 1 failed, 245 skipped; nextest 0.173s. The assertion at common Kernel component.rs:1587 observed `left: true`, `right: false`: normal close waited for the held arena. The test completed its owner recovery before asserting. This is semantic RED, not a compiler failure and not an OS Store/SPR dependency result.

Retained raw output: `🧪️member-kernel-turn-patch-red-r3-native-2026-08-27.txt`.

```text
manifest::kernel::ui_turn_patch_tests::ui_turn_patch_owner_normal_close_does_not_wait_for_arena
assertion `left == right` failed
  left: true
 right: false
test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 245 filtered out; finished in 0.12s
Summary [0.173s] 1 test run: 0 passed, 1 failed, 245 skipped
NX Running target test-wire-retirement-native for project @semio-tech/framework-rs failed
```
