# Current Product Eleven Original Whole Failure

The original defaultlong Product build succeeded in15m00s, then the original failfast owning whole selected1675 tests and executed631:630 passed,1 failed,11 skipped,1044 unrun. No complete owning success is claimed. The sole failure is native_smoke_without_a_selected_application_returns_failure: its current host channel assertion reads input21 against canonical23. The actual failure body below is preserved; no root cause correction or fixture write is inferred.

```json
{
  "owningExitCode": 1,
  "owningRefusal": "Error: /Users/ueli/.bun/bin/bun exited with status 1",
  "sourcePostUnavailable": false,
  "owningPostRefusal": null,
  "cancelled": false,
  "dispatcherExitCode": 1,
  "dispatcherRefusal": null,
  "dispatcherPostRefusal": null,
  "summaries": [
    "    Finished `test` profile [unoptimized] target(s) in 15m 00s",
    "        FAIL [   0.055s] ( 623/1675) semio-framework-os-renderer-wgpu renderer_io_retained_tests::native_smoke_without_a_selected_application_returns_failure",
    "    test renderer_io_retained_tests::native_smoke_without_a_selected_application_returns_failure ... FAILED",
    "     Summary [ 107.388s] 631/1675 tests run: 630 passed, 1 failed, 11 skipped",
    "        FAIL [   0.055s] ( 623/1675) semio-framework-os-renderer-wgpu renderer_io_retained_tests::native_smoke_without_a_selected_application_returns_failure"
  ],
  "actualFailure": [
    "    thread 'renderer_io_retained_tests::native_smoke_without_a_selected_application_returns_failure' (1532892) panicked at 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust/📚️library/../../../🧊️renderer/../../../🧪️tests/🔬️wgpu-renderer-renderer-io-retained/🦀️.rs:209:5:",
    "    assertion `left == right` failed: the smoke input uses the current host channel",
    "      left: Number(21)",
    "     right: Number(23)",
    "    note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace",
    ""
  ]
}
```

Owning and dispatcher both closed1 with available/exact source and runtime postguards, no cancellation, and the actual child failure refusal retained. Current UI787 and Board546 positive results do not admit joint publication. All26 models remain held; no applicability or live application proof was emitted. Raw whole log and compiled/test artifacts remain under epoch11 for read-only diagnosis.
