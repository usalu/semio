# Live Local Interaction Transport Review

## Native Wire Contract

Coordinator read the actual result of `@semio-tech/framework-replication-rs:test-local-interaction-native`: **7 passed**, zero failed, 211 filtered, 3.49 seconds compilation, test duration rounded to 0.00 seconds. The output still contains one qualification warning; this is not a warning-denial pass.

Log: `🧪️member-local-interaction-wire-r2-native-2026-08-27.txt`.

Executed laws cover lossless u64/full identity, explicit nullable fields and unknown-field rejection, neutral restore parity, three command forms, four reply forms, unsigned LEB128 parity, and oversized page rejection before payload copying.

## Source Review

Read the complete live query owner, exact document/config read-pair owner, fixed query-page cursor and transport codec, plus their current app-instance mount. The live query retains three exact Store reads, does not reuse its slot before owner retirement, and obtains a checked generation from a runtime-owned allocator. The query page remains ACK-owned, and cancellation hides it before retirement. Generation belongs above reused app instance IDs.

This source review does not prove the still-being-mounted outer AppCommand/AppFrame delivery or all tutorial/native-shell consumers.

## Assigned Integration Checks

1. The live wrapper currently discards the query cursor's emitted/retired byte counts and Blocked outcome. The partial-error path also reduces a counted encoder error to a String. Preserve honest query progress through the live boundary, including physical error-prefix bytes; do not call emitted bytes released bytes. The existing one-step scheduling bounds are not a substitute for actual progress accounting.
2. The reply extraction marks Started/page as sent before returning its value. The actual transport must retain that fixed reply losslessly until output admission or perform the transfer atomically. A saturated output channel must not drop it and permanently strand the one-slot query.
3. Verify output saturation, exact cancellation, app close, reused app IDs and late Started/page/ACK responses at the mounted channel boundary. The current query-owner fixture tests are not an actor-channel proof.
4. Keep restore authority tied to retained document/config roots and the actual topology input generation. Frozen read-only queries may legitimately describe an older snapshot; future authoritative restore must reject stale input authority.

The expanded plugin cohort r3 executed **13 passed / 3 failed**, 429 filtered, 0.08 seconds test duration. All three failures deserialize the native test InteractionState without its required `hover` field before reaching the new live behaviors. The native fixture now explicitly includes hover; the public query/restore contract still correctly excludes it. The coordinator read the exact failed output: `🧪️member-local-interaction-r3-native-2026-08-27.txt`. No 16-test green is claimed.

Dag is completing the coherent progress and outer transport API join before the next plugin compilation. Actual CAD remains queued after that join. The publication executor retains the sole fleet Rust lease and may run independent warm kernel regressions during the plugin source window.
