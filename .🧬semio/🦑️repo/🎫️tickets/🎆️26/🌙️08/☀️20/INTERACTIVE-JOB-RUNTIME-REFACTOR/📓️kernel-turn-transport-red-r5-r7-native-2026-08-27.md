# Common Kernel Transport Contention — Native RED R5–R7

Each exact law uses `@semio-tech/framework-rs:test-wire-retirement-native`, `--lib`, and `--nocapture` in a separate invocation to avoid fail-fast omission. Production transport remains unchanged until all three observations are captured.

## R5 Producer Drop

Actual exit 1: 0 passed, 1 failed, 249 skipped; nextest 0.147s. `ui_turn_patch_transport_producer_drop_hands_back_without_waiting_for_arena` observed wait `true` versus required `false` at common Kernel component.rs:1826. The fixture recovered its exact owner before asserting.

```text
assertion `left == right` failed
  left: true
 right: false
Summary [0.147s] 1 test run: 0 passed, 1 failed, 249 skipped
```

Raw: `🧪️member-kernel-turn-transport-producer-red-r5-native-2026-08-27.txt`.

## R6 Lease Drop

Actual exit 1: 0 passed, 1 failed, 249 skipped; nextest 0.157s. `ui_turn_patch_transport_lease_drop_hands_back_without_waiting_for_arena` failed its held-arena no-wait law after exact owner recovery.

```text
Summary [0.157s] 1 test run: 0 passed, 1 failed, 249 skipped
```

Raw: `🧪️member-kernel-turn-transport-lease-red-r6-native-2026-08-27.txt`.

## R7 Normal Close

Actual exit 1: 0 passed, 1 failed, 249 skipped; nextest 0.148s. `ui_turn_patch_transport_normal_close_does_not_wait_for_arena` failed its held-arena no-wait law after exact owner recovery.

```text
Summary [0.148s] 1 test run: 0 passed, 1 failed, 249 skipped
```

Raw: `🧪️member-kernel-turn-transport-close-red-r7-native-2026-08-27.txt`. All three requested semantic RED laws have now executed individually; none was omitted by fail-fast. Production transport is released to its owner for correction.
