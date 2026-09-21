# Native Asset Response Handoff

## Verified Problem and Current Tests

Native transport rejected every populated retained page and every populated HTTP chunk. The language-neutral fixture is engine/fixtures/native-asset-response, with one-byte and 16384-byte exact-limit cases. Chromium Response preserves both cases in the 150-test browser-worker GREEN receipt. Native120 produced the valid production RED: zero of one expected byte transferred; the separate busy-runtime cancellation test passed. Native118 was compile-only and Native119 needed a fixture cleanup correction before this actual RED.

The helper now takes a borrowed retained page, makes one bounded copy into the World response, and explicitly releases the source only after successful admission. HTTP moves its owned page directly and yields between pages. A typed handoff slot preserves the exact request across seal Busy and return Busy, including bounded disposal of a rejected payload; it returns cancelled owners to their still-live authority. Native121 is running these changes and two fail-first native reference disposal laws. These production changes are not yet recorded as passing.

## Ownership Design

Keep the exact fetch owned across transport, seal, return, cancellation, and disposal. Native I/O returns one bounded RetainedJobPayload page; WorldAssetResponsePage owns an incompatible boxed byte buffer, so transfer requires one bounded copy. A borrowed-payload helper can retain an invalid/rejected source, with the caller responsible for its explicit bounded close. HTTP's owned Vec can move directly into a response page. Never report cancellation or a transient seal Busy as a frame fault.

The native handback slot must distinguish SealPending from ReturnPending. A successfully read response that cannot yet seal waits unchanged for the interaction-owner wake. A cancelled/failed response begins close and returns to its still-live World authority; a busy handback retains that exact owner instead of dropping a closed fetch. The current native_asset_blocked path closes and drops an owner without returning its checked-out authority token, so it must be replaced by this typed handback.

Per-request transport cancellation must carry exact World surface and request tokens plus a child cancellation token. Closing component A cancels that request only; B's authority stays reachable. The global host close may cancel the parent token. Native reference decode retirement is a separate outstanding owner and must release its large retained input/pixels through explicit grants.

## Required Verification

Native122 completed 60 tests: 59 passed, one Map fixture setup failure, 2.560s (Nx 3m2s). The native response test now includes a 16385-byte two-page rejection and confirms the entire source stays retained until explicit close. Both exact handoff laws pass: seal Busy and return Busy retain the same request token/seven bytes, and component cancellation returns A without a frame fault while B remains live. Native reference phase-zero and phase-two bounded retirement laws also pass. The separate in-flight native transport and phase-one worker-return disposal gaps remain open.

Native121 completed 58 selected tests: 55 passed, three failed, in 1.289s (Nx 3m21s). Native page transfer now passes for one byte and the exact 16384-byte limit, preserving byte equality and releasing the retained source. The two native-reference disposal laws produced valid RED: cancellation discarded 32785 encoded bytes before a close grant, and one close turn removed a phase-two Waiting output larger than two pages. Sol owns their bounded retirement repair. The third failure remains Map fixture setup, not a production failure. The four Interpreter window-close laws pass after setup uses actual presented protocol-owned surface documents.

After the page RED, implement bounded handoff and rerun byte equality/token identity. Add exact-limit-plus-one rejection with the source retained for close, native seal Busy followed by same-owner completion, cancel after one page without frame fault, and an end-to-end local catalogue GLB read through independent decode. Rust source gates do not substitute for WGPU19 activation and paired React/browser acceptance.
# Native125 Transport Integration

The real local TCP stalled-body law failed in Native125 at its intended sibling-progress assertion (2.07 seconds); its cleanup released the body and joined the server. The transport API now distinguishes interruptible Socket reads from Ureq reads waiting for the configured 15-second read deadline. Root integrated the exact surface/request lease after HTTP head receipt, retains its guard through stream termination, signals only the matching component during close, and signals all transports during renderer close. The child cancellation token is checked before and after each body await, including EOF and error outcomes, so cancellation cannot seal a partial response or fault the frame.

The source parses. The native runtime GREEN is pending. The new body lease does not cover cancellation while the HTTP head is blocked. Existing connect/read timeouts are not proof of a total cancellation bound, particularly for a slowly progressing head; a dedicated withheld-head regression is being prepared before that separate repair.
