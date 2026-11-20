# Boot failure: "original outcome acknowledgement requires its live receiving place"

## Reproduction
`boot-probe.cjs <url> <out.json>` (chromium headless, PLAYWRIGHT_BROWSERS_PATH as in verify-static.sh). `#generator` on http://127.0.0.1:8766/ reproduces: the guest logs `poll semantic failure ... InvariantViolated "original outcome acknowledgement requires its live receiving place"` and then `Framework OS boot failed` with message `[object Object] (see error.payload) payload=[object Object]`.

## Where it comes from
- `OriginalJobReturnReceiver::acknowledge_outcome` (`🧵️job/📬️outcome/⏸️continuation/🎟️origin/🦀️.rs`) at HEAD refused with this invariant when `closing || continuation.is_some()` BEFORE looking at the outcome.
- The only caller on the plugin poll path is the cleanup pump (`⚛️reactor/🏃️runtime/📦️instances/🔁️cleanup-turns/🦀️.rs`, `pump.outcome_pending` -> `session.acknowledge_outcome(cx)?`). It was written to proceed afterwards and inspect `checked_out_continuation()` (callback / qualification / receipt_qualification) when the job body is unreachable.
- When the job body ends as a Continuation (a failed body), the receiver holds a continuation and no outcome. HEAD turned that into the misleading "live receiving place" refusal, so the real failure inside the continuation was masked. The error is therefore a SYMPTOM; the true root failure is the continuation's cause (the body of a boot job failed).

## Source state
The other session's uncommitted diff (mtime 06:21) fixes the mask: outcome absent -> `Ok(true)` first, the refusal remains only for `closing`. This is correct and matches the pump's contract. The staged procedural wasm (built 2026-11-18 09:52, receipt sourceTreeDigest c7c2230e...) predates it, and also predates the `[DEBUG] cleanup job body unreachable: <cause>` print (absent from the wasm; `origin_admit_false`, `table_false`, `turn phase` are present). Not a revision mismatch between hub and framework: one stale wasm built from HEAD behaviour.

## Changes made
1. New Rust test `acknowledging_a_continuation_only_receiver_reports_its_cause_instead_of_a_receiving_place_refusal` in `🧵️job/📬️outcome/⏸️continuation/🧪️tests/📦️capsule/🦀️.rs` (not compiled/run by me).
2. Boot error surfacing: the shard worker's `replyError` (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🧵️shard-worker-source/🔌️actor-loading-and-replies/🟦️.ts`) did `JSON.stringify(payload)` which throws on bigint and fell back to `String(payload)` = `[object Object]`. It now serializes bigint (`123n`) and Uint8Array (`<n bytes>`). The payload then appears in the `Error.message` the UI logs.

## Must rebuild (not done, per instructions)
- `procedural` (hub component `🌀️procedural`, the only pane-booting guest observed failing) to pick up the origin.rs change and the cleanup-pump debug print, which then reports the real continuation cause on the next boot.
- The browser bundle/shard worker source embedding `replyError` (vite site / browser bundle that carries the `shard-worker-source`; no component rebuild needed for it).
- Other guest components using the same framework crates (`playbook-module-procedural`, `puzzle`, `flow`, ...) only if they must also carry the fix; the 7 failing panes all go through `procedural`.
- After rebuild, read the `[DEBUG] cleanup job body unreachable: ...` line to find the real cause; that is the actual blocker for redeploy.
