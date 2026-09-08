# Renderer Tutorial Interaction State

## Outcome

Tutorial selection recording and playback now use the actor-owned local interaction contract rather than an empty capture, a Shell-only mirror, or the retired opaque `selectionJson` path.

- `PluginWasmHandle.readLocalInteraction` drains the retained native query page by page, enforces the 1 MiB bound, decodes the exact `LocalInteractionCapture`, and rejects an identity mismatch.
- The decoder owns all returned arrays/maps, accepts exact lossless u64 and revision encodings, rejects duplicate selection ids and unknown fields, and preserves special domain keys without prototype mutation.
- Shell retains the last actor observation. Session establishment and successful framework interaction actions schedule a new retained capture; hover remains ephemeral and is preserved locally.
- Recording waits for a fresh actor capture before constructing its base UI snapshot.
- Tutorial playback projects selection onto ordinary `clearSelection`, `setSelectionMode`, and `interactionSelect` action descriptors. The Shell reducer never forges the actor result; its interaction state changes only through `INTERACTION_STATE_OBSERVED`.
- The language-neutral fixture and schema cover actor capture, comma-bearing ids, special domain keys, sparse replacement, and explicit clearing. AJV and `fast-json-patch` provide independent schema/state-transition oracles.

## Evidence

- `bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:tutorial-interaction-check --skip-nx-cache` — GREEN, 7 passed / 402 skipped.
- `bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:typecheck --skip-nx-cache` — GREEN, no diagnostics.
- `bun ./📜️script.ts nx run @semio-tech/framework-renderer-react:test-long --skip-nx-cache` — GREEN, 583 passed across 11 files.
- The first full run found one stale channel-v14 Invocation fixture with absent `mutations`/`inverse_group`; the exact fixture now publishes the required empty fields and the full rerun is GREEN.
- Scoped `git diff --check` — GREEN.

This qualifies the renderer actor-query/CQRS integration and controlled runtime laws. It does not claim a browser-driven native/Wasm tutorial recording journey.
