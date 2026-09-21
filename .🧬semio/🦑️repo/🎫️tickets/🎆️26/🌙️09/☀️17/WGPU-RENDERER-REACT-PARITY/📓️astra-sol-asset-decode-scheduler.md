# Asset Decode Scheduler Contract

## Authority

The neutral `semio.frame-turn-scheduling.v3` fixture gives `frame` and `assetDecode` separate retained owners under one single-credit Worker task queue. It requires exact alternating dispatch, input admission between callbacks, one presentation wake only when decode publication changes visible state, no public frame message from decode, and close of a yielded decode without publication into its successor. A failed scheduling mutex acquisition is not a retained `Blocked` state; an owner must remain independently wakeable.

The fixture now also pins `expected.frameDecodeUnits` to `0`. This is the neutral counterpart of the native behavior law that one actual `FrameTransaction::step` cannot advance a retained catalogue decoder.

## Schema and third-party oracle

`📐️schema.json` is Draft 2020-12, closed at every object and tuple boundary. All `prefixItems` arrays have matching exact `minItems` and `maxItems`. Vitest validates the fixture with strict `ajv/dist/2020` before exercising the scheduler.

## Behavioral laws

- Concurrent frame and asset-decode requests share one scheduled callback credit and dispatch `frame, assetDecode, frame, assetDecode, assetDecode`.
- Ingress is observed after every callback and before the next retained callback.
- An asset-only request never reaches the frame owner.
- Closing after a yielded asset turn cancels the retired token, publishes no token, and cannot arm a successor callback.
- Decode work has a private Worker turn, posts one `wake` on presentation change, and never calls the frame tick or posts a public `frame` message.
- A sealed response whose first private decode turn observes the interaction owner unavailable returns `idle`. The later ordinary runtime handback wake marks both retained owners, runs the empty frame owner without a public message, retries decode, and publishes exactly one public wake.
- The scheduler has no un-wakeable `Blocked` mutex state.

## Fail-first receipt

Command:

```text
NX_DAEMON=false NX_FORCE_REUSE_CACHED_GRAPH=true bun nx run @semio-tech/framework-renderer-wgpu:test-browser-worker -- '🧪️tests/⏱️wgpu-worker-step-budget/🟦️.ts'
```

Receipt: `🗑️generated/astra-runtime/browser-asset-decode-red/run.log`.

Result: 11 files, 147 tests; 144 passed and 3 intended failures. The strict Ajv schema oracle passes. The failures identify the missing production boundary precisely:

1. Current scheduler executes only `[frame, frame]` rather than alternating with the three retained asset turns.
2. An asset-only request reaches the frame owner.
3. `frame-worker` has no private `runAssetDecodeTurn` publication path.

Production scheduler and frame-worker code remain unchanged pending this fail-first evidence.

## Worker implementation receipt

The shared `WorkerTurnTaskQueue` remains the sole MessageChannel owner. `FrameTurnScheduler` now retains independent frame and asset-decode pending bits, alternates them via `lastDispatched` when both are runnable, and consumes one selected owner per callback. Close disarms both owner kinds before the existing close step.

`frame-worker` uses the agreed closed ABI `assetDecodeStep(): string` with `{ kind: "pending" | "published" | "cancelled" | "idle" | "fault", pending, detail? }`. Its private `runAssetDecodeTurn` does not call `tick` or post `frame`; only `published` posts the existing `wake`. A sealed asset response now requests the private asset-decode owner.

Canonical browser-worker receipt: `🗑️generated/astra-runtime/browser-asset-decode-green/run.log`. Result: 11 files and 147/147 tests passed; tests 2.28 s, Vitest 7.62 s, Nx 14.5 s. Rust still needs to supply the agreed wasm export and typed result before whole-renderer acceptance.

## Runtime handback wake

The shared runtime wake callback now calls `FrameTurnScheduler::requestRuntimeWake`, which atomically marks both retained owner bits and arms the same single-credit task queue once. The v3 neutral fixture pins the unavailable-before-handback `idle`, exact post-handback owner order `[frame, assetDecode]`, `published`, one public `wake`, and zero public frame messages. The behavioral test executes that state machine; the strict Ajv oracle validates the new closed fixture member.

Canonical focused receipt after this change: `🗑️generated/astra-runtime/browser-asset-runtime-wake-green/run.log`; 11 files and 148/148 tests passed, tests 2.08 s and Vitest 8.10 s. The first execution took 22.2 s with one of five Nx tasks cached; the retained receipt replay was a full Nx cache hit. Command unchanged from the fail-first receipt. Rust native decode ownership and the browser transport's exact fetch cancellation remain separate acceptance boundaries.
