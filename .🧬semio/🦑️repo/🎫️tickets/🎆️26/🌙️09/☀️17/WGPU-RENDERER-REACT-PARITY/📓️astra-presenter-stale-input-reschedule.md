# Presenter Stale Input Reschedule

## Actual browser failure

The repaired WGPU19 build booted and remained live for several minutes. Physical canvas clicks closed and reopened Settings, the App section showed the same four numeric controls as React, and General opened. A physical click on the Driver heading then quarantined the renderer at `2026-09-21T16:34:08.439Z` with:

```text
worker-present-failed: prepared frame input authority was stale before submit
```

Receipt: `🗑️generated/astra-runtime/checkpoint19-iab/wgpu-drivers-fault.json`.

This is a normal pre-submit supersession boundary. The frame had staged a presenter packet and an input witness, but a newer interaction epoch invalidated the UI candidate before any GPU submit. The prior implementation correctly moved the cursor to `Aborted`, but returned `Err`; the browser worker interpreted that error as a surface fault and quarantined an otherwise healthy renderer.

## Repair

Both pre-submit stale observations now:

1. abort the unsubmitted presenter packet;
2. clear its presenter witness;
3. retain the exact Shell/UI candidate on the frame for the existing bounded `Aborted` phase to discard;
4. return `AppPresentStep::Pending`.

The abort phase still closes the active GPU candidate, returns the exact input witness, releases raster ownership, and retires the frame before the next admission. No stale authority is acknowledged or revived. Staleness after GPU submit remains a separate consistency failure; this packet changes only the pre-submit boundary demonstrated by the browser receipt.

## Regression law

`a_pre_submit_input_epoch_change_retires_and_reschedules_without_faulting_the_surface` in `🧪️tests/🔬️wgpu-renderer-async-boundary/🦀️.rs` checks both stale branches return the packet and presenter witness, enter bounded abort, and answer `Pending` without the former fault string. It also pins the exact input-witness discard in the abort phase.

The production and law sources parse under `rustfmt --edition 2021 --emit stdout`. Root owns native execution and the fresh WGPU activation.

## Fresh browser acceptance

The rebuilt WGPU worker repeated the physical Driver interaction at `2026-09-21T16:45:15.611Z`. The click at `(1343, 623)` opened Driver and published the seven axes plus the name and save controls without a new renderer fault. Receipts: `🗑️generated/astra-runtime/checkpoint19-iab/wgpu-driver-open-green.json` and `🗑️generated/astra-runtime/checkpoint19-iab/wgpu-driver-open-green.jpg`.

This confirms the demonstrated pre-submit stale path now retires and reschedules in the browser. The source-string law remains only a structural guard, so a native A-accepted → B-stale → C-accepted owner-lifecycle law is still required.
