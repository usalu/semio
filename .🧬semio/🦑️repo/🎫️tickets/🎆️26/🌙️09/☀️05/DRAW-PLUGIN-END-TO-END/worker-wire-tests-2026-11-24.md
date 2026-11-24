# Worker wire tests: three failures, causes and fixes (2026-11-24)

Command: `repo test-file "🧰️framework/🛍️products/💻️os/🧪️tests/🪪️document-backbone-worker-wire/🟦️.ts"`. Before: 8 pass, 3 fail. After: 11 pass, 0 fail (three consecutive runs, 1.5 s, 1.1 s, 1.3 s).

## (a) and (c): the tests were wrong, the product was right

Cause: the suite runs under bun (`bun test v1.4.2`), where `expect(promise).rejects.toThrow(...)` blocks synchronously until the promise settles. Both tests created that expectation before the action that settles the promise (`mailbox.settle(...)` or `mailbox.close(...)`), so the call waited for the mailbox timer instead: 30 s `inference-opening: admission unconfirmed` for the inference mailbox, 1000 ms `browser-actor-action: completion unconfirmed` for the action mailbox. A probe test (created and deleted, `🧪️tests/🩺️probe-rejects`) confirmed it: `expect(p).rejects.toThrow("manual")` returned only after the 1500 ms timer rejected, with message "timer".

Product check: `InferencePortOpeningMailboxV1.settle`/`close` and `BrowserActorActionMailboxV1.close` reject synchronously with the exact reason, in agreement with the fixtures (`inference/opening` fixture `refused.code = inference.capacity`).

Fix (assertions kept, only moved after the action): keep the promise, act, then `await expect(promise).rejects.toThrow(reason)`.
- `🔶️admits-inference-opening-only/🟦️.ts`: `refused` and `closed`.
- `🔸️settles-only-exact-live/🟦️.ts`: `retired`, `firstClosed`/`secondClosed` (now `Promise.allSettled` after `close`, both must be rejected with "owner retired"), `refused` in the retry block, `expired` in the timer block.
- The timer block used `vi.useFakeTimers` and `vi.advanceTimersByTimeAsync`, which bun's `vi` does not provide (`is not a function`). It now uses real timers with a 200 ms mailbox timeout and waits of 50, 200, 50 and 250 ms (at least 50 ms margin on every boundary). Same assertions (no early send, expiry of the first request, queued request sent after expiry, settle before its own timer, no pending after settle).

## (b): the product was wrong (the test and the contract agree)

Cause: `projectWireEffect` and `projection` in `🧰️framework/🔨️modules/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts` had no `clipboard-write` case, although the contract has it: WIT `effects.wit` `clipboard-write-effect { fragment: pack }`, kernel `Effect::ClipboardWrite { fragment: ClipboardFragment }`, Rust bridge `E::ClipboardWrite` (JSON `ClipboardFragment` decode), wgpu bridge `{ clipboardWrite: { fragment: decodePackWire(...) } }`, retained-ui-patch `{ clipboardWrite: { fragment: packField("fragment") } }`, TS kernel `Effect` type `{ clipboardWrite: { fragment } }`, and the fixture `publication.clipboardFragment`. The test (added in commit 752) was ahead of the product.

Fix: added `clipboard-write` to `projectWireEffect` (exact fields, canonical pack of at most `BROWSER_ACTOR_ACTION_PACK_MAXIMUM_BYTES`, so trailing bytes fail), added `clipboardWrite` to `BrowserActorHostEffectKeyV1`, and `clipboardFragment` validation in `projection` (exact `schema`, `mediaType {class, form}`, `dslText`, `sourceApp`, `label`, optional `packBytes` of bytes 0..255; the `{ label: "incomplete" }` fragment is refused). Encoded form is `pack({ clipboardWrite: { fragment } })`, decoded canonically on the Shell side.

## Effect on real copy and paste (plugins in the browser)

Yes, before the fix. Path: a guest running in the document browser actor (copy or cut, for example the clipboard job of the Puzzle editors or Draw) emits WIT `clipboard-write`. `ReservationTurnEffects.routeTurnEffects` (`🏪️store/👷️worker/🧷️actor-reservation/📤️turn-effects/🟦️.ts`) calls `encodeBrowserActorHostEffectV1` for every non-`send-message` effect of an action or command turn. It threw `unsupported host effect clipboard-write`, so the copy turn failed and no fragment reached the Shell. On success the effect travels in `result.hostEffects`, is decoded by `decodeBrowserActorHostEffectsV1` in `🏛️ShellHost/🛫️spawned-session/🟢️request-service-operation/🟦️.ts`, passed to `applyHostEffects` (`🏛️ShellHost/⚡️host-effects/🟦️.ts:99`) which stores `clipboardFragmentRef`; `pasteActionWithRetainedFragment` injects it on the next `paste`. With the projection, that browser-actor path is whole. The wgpu/native paths and the retained-ui-patch lane were not affected (they have their own `clipboard-write` handling).

## Files

- Changed: `🧰️framework/🔨️modules/🌐️browser-bundle/🎯️action-handoff/📤️publication/🟦️.ts`; `🧰️framework/🛍️products/💻️os/🧪️tests/🪪️document-backbone-worker-wire/🔶️admits-inference-opening-only/🟦️.ts`; `.../🔸️settles-only-exact-live/🟦️.ts`.
- Not mine, untouched: `.../🪪️document-backbone-worker-wire/🟦️.ts` (registration of `registerAcknowledgedStatusTests`) and the untracked folder `.../🔹️acknowledged-status/` are another agent's work in progress; they pass with the rest.

## Results

- Worker wire suite: 11 pass, 0 fail, three runs.
- Direct: `repo test-file` on `📤️publication/🧪️tests/🟦️.test.ts` (3 pass) and the action-handoff schema test `🧪️browser-actor-action-handoff-validates-the-neutral-schema-and-exact-owne/🟦️.ts` (1 pass).

## Not verified

- No browser run, no Rust build: the Rust twin was read, not executed. A clipboard fragment produced by a real guest was not round-tripped through the browser.
- `routeTurnEffects` still throws `document browser actor: unsupported effect` for any non-`send-message` effect in the string modes `"ordinary"` and `"control"`. If a reserved copy job completes on such a turn, its `clipboard-write` is still refused there; this was not changed (no evidence yet which mode real copy completions use).
- The pack to JSON decode of `packBytes` is checked only through the fixture (`[0, 127, 255]`).
