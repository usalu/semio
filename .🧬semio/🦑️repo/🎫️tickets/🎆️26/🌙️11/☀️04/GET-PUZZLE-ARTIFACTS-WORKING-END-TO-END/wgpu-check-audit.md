# WGPU Check Failure Audit

The compiler output in `clipboard-wgpu-check-sequence.log` identifies independent API drift outside Puzzle clipboard:

- Kernel replay client needs `open_create_app_source` and `CreateAppBorrowed` from `semio_renderer_wgpu_kernel_protocol`. Pool app lifecycle needs the `semio_plugin_host::GuestRuntime` trait in scope for `compile`. `ShardIdentityPolicy` derives Serde `Deserialize`, not `FromValue`; the existing OS run bootstrap reads the same authored policy with `serde_json::from_str`.
- Canvas 2D packet image decoding needs the `base64::Engine` trait in scope for `STANDARD.decode`.
- Graph selection's edge iterator cannot clone because `NumericSetIter` is not `Clone`; recreate the iterator for the credit pass. Flow and DAG cameras are distinct types, so project each match arm to `[x, y, zoom]` before forming the tuple.
- Icon export uses an obsolete job-return and retirement interface. `InteractiveJob::step` requires an actual original return claim and yields `InteractiveJobStep`; the owner needs a paid return receiver and `drive_step`/`OriginalJobReturn` handling, including pending, continuation, acknowledgement and retirement. See `plugin/🧪️testing/🧪️artifact-app-laws/🎟️fixture-dispatch-steps-and-document-loading/🦀️.rs` `fixture_step`, and `🧵️job/🚦️clock-stride-and-job-protocol/▶️drive/🦀️.rs`. Do not fabricate a claim.
- The icon export `close_prepared` helper accepts an old grant-to-`Result` closure; prepared packet, input and rejection owners now require `&mut RetainedAllocationEffect` and return `RetainedWorkPoll`. Adapt the helper and its callers/tests to `close_original_effect_step`, preserving returned work, progress and refusal; `🌊️flow/🖥️host/📥️evaluation-source/🦀️.rs` has a paid effect example. `JobOutcomeDescriptor::acknowledge` already returns a `RetainedWorkOutcome`; passing it directly removes the erroneous outer `Ok`.

These are source-level observations from the existing log; no build was run in this audit.
