# Runtime Fixed Backing and Inline Fields R65–R66

R65 canonical exhaustive `surface_output_pool_`: exit 0, 6 passed, 109 skipped, 0.173s. Raw `🧪️member-runtime-output-pool-resident-r65-native-2026-08-27.txt`.

```text
[DEBUG] output-pool static-ledger contract=390800 runtime=143568 total=534368 additional-root-slots=0 final-release-retains-static=true
Summary [0.173s] 6 tests run: 6 passed, 109 skipped
```

The runtime registers both its existing handback registry and the shared output pool before admitting a runtime reservation. Two queues charge this fixed domain once, not twice; closing every entry/queue leaves static storage charged and consumes no dynamic root slot.

R66 canonical exhaustive original `surface_ownership_inline_fields_do_not_allocate_a_second_owner`: exit 0, 1 passed, 114 skipped, 0.047s. Raw `🧪️member-runtime-inline-census-r66-native-2026-08-27.txt`.

```text
[DEBUG] surface-inline-footprint name="tree-item-icon" before=19368 after=19368 delta=0 items-before=14 items-after=15
[DEBUG] surface-inline-footprint name="reserved-binding" before=218280 after=218280 delta=0 items-before=12 items-after=14
Summary [0.047s] 1 test run: 1 passed, 114 skipped
```

The original assertions and neutral expected deltas stayed unchanged. A typed `UiText` census operation distinguishes inline storage already held by the enclosing record from an extra heap owner; traversing an additional inline field still counts an item. All 51 such fields now use that typed path. Actual heap owners were not zeroed or replaced with initialized length.

This corrects the original duplicate-inline-charge defect only. The old three-copy estimate, logical-capacity list estimates, upfront full-array construction, parent copy/move work, and full dynamic source/candidate/output/retired overlap still require their own exact admission/census cutover. No Process fit or fully bounded-runtime claim follows from R66. A full runtime suite without exclusions is running as R67.
