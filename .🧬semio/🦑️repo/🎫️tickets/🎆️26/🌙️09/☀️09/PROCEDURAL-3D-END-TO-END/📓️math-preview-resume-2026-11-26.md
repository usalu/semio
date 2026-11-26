# Math progress and the Edit preview continuation

The ordinary Chromium B4 journey submitted a real `math.vector` call for Mushroom’s extrusion axis, received its 87-byte Pack reply, and published an Edit-preview `flowEvalTick` effect. At the end of the baseline window, the viewport still showed 4 of 7 units and one outstanding call. The trace contains no subsequent provider request.

| Witness | Observed value |
| --- | --- |
| First requested operation | `flow-extension-math/evaluate`, request 1 |
| Evaluation identity | `extrusion-axis`, `math.vector`, hash `17647890481751977919` |
| Reply | Unfinished, cancellable, `input-retire`, 256 of 257 units, null output |
| Reply SHA-256 | `b1ddfe8470f5147ecf7c3a919efe7f4a09d87038e6df60c583ca6082cf22b396` |
| Served B4 requester SHA-256 | `c2824a5ee59995d51d080e90443344f54fc26a1a220d8d61c2bc09c8f998c60f` |
| B4 Chromium delivery and Edit rearm | 588.167 s and 588.169 s |
| B4 Chromium cutoff | 762.207 s, no accepted mesh |

The optional Node JSPI replay consumes the staged requester and browser-captured open, surface and first Edit tick. It verifies the exact 87 bytes and preserves the captured 64-bit evaluation identity. It releases the captured progress once the running requester emits the same fresh Math call. It waits for the emitted Edit rearm and requires a new request ID and compact resume with the same identity. The browser journey is documented in [the B4 React report](./%F0%9F%93%93%EF%B8%8Freact-fresh-requester-2026-11-26.md).

The first Node run reached the inherited setup bound at crossing 697 while contribution installation still had retained work. A longer run with baseline Wasm compilation admitted the real 425,209-character contribution pack, then emitted lane 11 `job-session.terminal-fault` for operation 128 at crossing 629. It subsequently reported idle. No replayed Math bytes entered either run.

The focused native law passed in 0.03 seconds. It decoded the captured Pack through the actual OS codec, folded the progress under the retained original caller, settled the Edit preview’s outstanding call and armed exactly one continuation. The original dispatch producer published the same full-width node hash with the compact resume flag and empty input and dependency payloads. The renderer TypeScript codec independently decoded and re-encoded all 87 bytes and verified the complete captured request without rounding the node identity.

The later stock Rectangle journey observed two extension effects from one Edit tick at 536.511 seconds and a Math response at 537.614 seconds. The React shell serializes both provider execution and reply delivery by requesting instance. Any extension effect queued behind that response waits for the requester’s completion turn. The viewport rose from 0 of 1 to 4 of 7 by 691.323 seconds with no mesh and zero in flight. An Edit release and two window ticks were also issued around the first request. The progress change alone cannot identify the pending Math delivery or a second request.
