# Unit grant growth: cancelled at once, then "broker closed"

Failure: `🗑️generated/terrain-all9-6.log` (gisterrain tests, `-j 6`, 3.8 h, exit 101). A unit that needs more than the 1536 MB cargo grant (`semio_plugin_actor_input_decoder`, 2191 MB) asked for 655 MB more (at most 15 min), the wait ended "cancelled after 0 s", the request was sent again four times, then `compiler broker admission failed ... after up to 5 attempts: broker closed`.

## Findings

1. Why "broker closed". The client (`🎟️admission/🦀️.rs`) only reports `broker closed` when the broker side ends the socket. In `startAdmissionBrokerV1` (`🎟️admission/🟦️.ts`) the growth outcome `cancelled` ended the request with `socket.destroy()` whatever the reason. The wrapper treats an EOF as a restarted broker, waits 2, 4, 8, 15 s and sends the request again (5 attempts, matching the five ask/cancel pairs in the log). Every attempt met the same outcome, so the client gave up although it was alive and waiting.
2. Why the wait was cancelled after 0 s. `createGrantGrowthPortV1.acquire` (`🦑️repo/🏗️build/🌡️host-capacity/🟦️.ts`) mapped every way the gate call ends without a grant to `reason: "cancelled"` unless its own timer had fired: the gate promise rejecting for any reason (sample unavailable, queue full, ancestor or starvation refusal, a transient file fault) took the same branch as a vanished client. A rejection is immediate, hence "0 s", and it repeated on every retry. The broker did not check that its client was gone either; the only legitimate cancel is the client socket closing (`gone.signal`).
   Candidates ruled out by reading and by live probing of the gate with the real host directory: an already aborted signal (the relay is only armed on a live signal, a closed client does abort `gone.signal` and then destroying is right), a wrongly computed deadline (the wait is `waitMilliseconds` 900000 ms and a timer fires as `timeout`, never `cancelled`), the gate refusing the kind (a probe `cargo-growth` 655 MB request is queued and admitted fair-queue ordered, with or without a grant-holding ancestor), the retained reservation ordering (it only delays: "an earlier request is still waiting and holds the next reservation", never rejects).
   The exact gate error of the 3.8 h run was swallowed by the old code and the process is gone, so it is not recoverable; the fix makes any such error visible and recoverable.
3. A timeout that did arrive ended as the refusal `parent-memory`, and for a unit with history (not `cold`) a refusal makes the wrapper fail the compile. So even a healthy-host timeout failed the build.

## Fix

- Port (`createGrantGrowthPortV1`): `cancelled` only when the broker's signal aborted, `timeout` only when the declared wait ran out. A gate error is written once per message to the progress log and the gate is asked again (fresh request id, pause `wait.progressMilliseconds`) until the wait is spent; the answer then is `timeout`.
- Broker (`growParent`): `cancelled` counts only while the client signal is aborted; otherwise it is treated like a timeout and never closes the socket of a live client. A timed out growth (`refused`, `waited: true`) now takes the existing serial path (run alone, within the grant, when nothing else runs, announced in English and German) instead of refusing the unit. `beyond-host` and `no-host-fits` still refuse (no machine can hold the unit).

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🌡️host-capacity/🟦️.ts`
- `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🎟️admission/🟦️.ts`
- Tests: `🏗️build/🌡️host-capacity/🧪️tests/📈️unit-growth/🟦️.ts` (2 new), `🎟️admission/🧪️tests/📈️growth/🟦️.ts` (2 new, 1 changed).

## Tests

- Port: a growth waits for room that appears after 400 ms and is granted (wait lines seen, booking released); a gate that fails three times is asked again, the errors are logged and the answer is `granted`, never `cancelled`. `unit-growth` 8 pass.
- Broker: a growth wait that stays open keeps the client connected with `admitted:false` heartbeats, the signal never aborts, then ends granted and the booking is returned on `done`; a gate answering `cancelled` with the client alive ends with `admitted:true` (alone) and no closed socket; a timeout runs alone instead of the refusal. `growth` 9 pass (run with the ajv import stubbed: this Mac has no `node_modules` mount, `ajv` is missing for the files that import it); `waits`, `cold-class`, `deadlines`, `watchdog` pass through `repo test-file`.

## Live

See the end of this file.

## Live (new code, `unit-grant-growth-live.sh`, build of `semio-plugin-actor-input-decoder` through the gisterrain manifest, `-j 4`)

- The host was saturated by four other agents (graph plans of 4.5 to 5 GB, other cargo grants); the run waited 66 min at the capacity door for its own 1536 MB grant, then compiled 288 of the units.
- Growth waits that really wait: 4 units asked "at most 1 min" and ended after the full minute with "got no grant growth within 1 min ... runs alone with the rest of the grant" (the new serial path), and 0 lines "cancelled after 0 s" in 6.9 MB of log. Before the fix the same bound ended in a refusal.
- Not reached: the 2191 MB decoder unit itself. The build stopped at `semio-os-store-replay-operation` (a dependency in the decoder's chain): `error[E0599]: no method named issuer found for struct OriginalSourceTurnIdentity` in `🧬️native/👁️post/🦀️.rs:51`, a half-finished edit of another agent in the shared tree. I stopped the run after that (the log was deleted as generated output). Re-run `unit-grant-growth-live.sh` once that crate compiles.
