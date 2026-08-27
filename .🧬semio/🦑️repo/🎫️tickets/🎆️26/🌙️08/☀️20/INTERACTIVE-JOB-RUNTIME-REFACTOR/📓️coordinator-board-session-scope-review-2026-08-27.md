# Board Session Scope Review

## Verified Source Repair Checkpoint

The executor implemented per-provider BoardPeerScope with stable shell bindings, exact peer-identity guards for removal and gesture state, and exact failed-load eviction through the shared module loader. The coordinator reviewed this source and independently ran the full React suite at 484/484 in four files. Tests include simultaneous equal-ID shells, stale old attachment rejection after a successor mount, all three attachment-cancellation phases, and module retry/concurrent deduplication. The separate full typecheck now has seven unrelated-to-Board diagnostics for the still-pending real local-interaction tutorial join. Actual Puzzle Wasm output and fresh all-app browser tests remain unverified. Full records: `📓️coordinator-renderer-react-full-r5-2026-08-27.md`, `📓️coordinator-renderer-typecheck-r3-2026-08-27.md`, and `📓️coordinator-linked-session-engines-test-2026-08-27.md`.

## Source Findings

The new product-owned Board factory imports the real Puzzle wasm-bindgen output and registers editor/viewer app IDs explicitly. The shared renderer no longer needs to pretend the generic surface module exports BoardSession. The coordinator verified that the Puzzle `pkg` directory and its JavaScript/declaration/Wasm outputs do not yet exist. The new canonical build and product-composition typecheck remain required.

The coordinator read the current Board host lifecycle and cross-pane mirror implementation while its executor was testing asynchronous construction/attachment cancellation. Two further source defects are in the same ownership boundary:

1. The module-wide peer and gesture Maps are keyed only by `controllerId` and `surfaceId`; their comment explicitly assumes one triptych. The new per-instance constructor context does not scope those Maps. Simultaneous shells with identical IDs can overwrite or mirror each other's peers.
2. Cleanup unregisters by those IDs alone. If an old attachment rejects after a replacement session has registered at the same IDs, the old `fail` callback can unregister the new session. The guarded session-ref release does not protect the separate registry.

Required tests include simultaneous shells with equal string IDs and an old failed attachment settling after a new mount. The registry must be owned by a stable shell/app scope, and removal must match the exact peer/lease rather than merely its key. The existing construction/attachment/ready cancellation cases only settle successful attachment promises and do not establish this stale-failure law.

The new Puzzle and shared-surface module caches also retain a rejected initialization promise indefinitely. A transient load failure therefore cannot recover on a subsequent creation call. Failed in-flight entries need exact-identity eviction while successful/in-progress loads remain deduplicated, with a retry/concurrent-caller test.

These are source findings, not observed deployed-browser failures. They were assigned to the renderer executor before its next coherent lifecycle gate.

## Build Boundary

The existing `runWasmPackWebBuild` helper currently returns `void` and performs synchronous budgeted process calls. The new Puzzle WasmScript's synchronous call matches that actual signature; no missing-await defect was inferred from other scripts' asynchronous wrappers.

Canonical registry generation succeeds when the already-supported `NX_DAEMON=false NX_ISOLATE_PLUGINS=false NX_CACHE_PROJECT_GRAPH=false` environment is used. The executor diagnosed the cached/isolated graph issue without editing metadata or deleting global caches. Actual Puzzle Wasm compilation stays behind the active native compiler lease.
