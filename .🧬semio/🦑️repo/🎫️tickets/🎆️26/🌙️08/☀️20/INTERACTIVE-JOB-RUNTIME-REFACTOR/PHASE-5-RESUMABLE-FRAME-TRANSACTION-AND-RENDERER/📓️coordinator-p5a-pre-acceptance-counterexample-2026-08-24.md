# Coordinator P5a Pre-acceptance Counterexample

Date: 2026-08-24  
Verdict: **RED — source handoff is not accepted.**

## Evidence

The isolated `interactivityMountedFrameTransactionSelfTests` invocation passes and rejects its 20
mutations, but the verifier scopes most forbidden-work checks to the `FrameTransaction` definition.
The mounted `Build` opportunity delegates to `AppRuntime::frame_before_input` as one opaque call.
That callee still performs complete or allocation-bearing work before control returns:

- `draw.clear()` and `overlay.clear()` retire whole dynamic collections;
- the icon-atlas branch clones the complete pixel owner;
- `render_chrome` builds the complete chrome tree/resource set in one call;
- `take_packets()` drains the complete engine-packet collection;
- `world_resources.append_to` appends the complete world-resource set; and
- `PreparedRenderInput::new` plus upload `push` construct candidate-owned dynamic collections without
  a P5a retained microcursor or pre-transfer operation/item/byte credit.

The mounted `Finish` path similarly delegates to `frame_after_input`, which takes complete draw,
overlay, engine-packet, and deferred-action owner graphs and may immediately call
`drive_pending_frame_deferred`. These are the exact opaque complete-work and bulk-retirement defects
called out by the P5a repair contract; stage labels around the calls do not make them bounded.

## Verifier Gap

A faithful mutation that restores or enlarges the opaque work inside `frame_before_input` or
`frame_after_input` is not rejected because the permanent verifier does not inspect those callee
bodies for bulk clear/take/clone/render/drain/append or require retained subcursor entry points.
The verifier therefore proves the wrapper shape, not the mounted call graph.

## Required Remediation

Replace both opaque calls with retained, fixed/page subphases owned by `FrameTransaction`. One worker
grant may perform at most one scalar, owner, page, node, packet, upload, or child opportunity and must
recheck freshness/deadline immediately around every bounded domain/platform call. Admission must
precede transfer, rejection must return the identical owner, and supersession/terminal close must
retire the candidate one actual owner/page per grant. Extend the verifier with live-call-graph and
faithful callee-body mutations so this counterexample cannot be reintroduced.

No Cargo, Nx, Wasm, browser, or timing gate was run while overlapping Rust packets were active.
