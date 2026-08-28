# Runtime Exhaustive Regression RED R49

Actual canonical exhaustive run selected106 tests and exited1: **58 passed, 2 failed, 46 not run**, 0.214s. No tests were deliberately excluded; ordinary fail-fast stopped the remainder.

Exact failures:

```text
every_large_tree_cursor_slice_stays_below_eight_milliseconds:
assertion left == right failed: left0 right31 (snapshot inspected the not-yet-sealed candidate)
fixed_runtime_owners_keep_bounded_state_off_the_stack:
assertion failed: size_of::<SurfaceReconcileCursor>() <= 48 * 1024
Summary [0.214s] 60/106 tests run: 58 passed, 2 failed, 0 skipped
46/106 tests were not run due to test failure
```

Raw: `🧪️member-runtime-canonical-regression-r49-native-2026-08-27.txt`. The first is a fixture lifecycle join: the canonical assembly now requires seal before a snapshot; the same strict8ms assertion is being applied to each seal step too. The second is a real representation regression: the initial join retained a second inline pending record. Production now reuses the existing structural record slot through `RecordSource(Option<UiNodeRecord>)`, preserving exact placement/refusal ownership without increasing48KiB or allocating a box. Native rerun pending.

Original inline physical-census RED remains open and was not reached in this fail-fast run. Transaction output join and complete physical census remain unfinished.
