# Mathematical Retained Command Runtime Source

## Outcome

All seven Mathematical action IDs have one exact app-owned retained factory registration and one exact bounded proof/disposition row: `setDocument`, `setAlgorithm`, `setDirected`, `nodeGraphEdit`, `nodeGraphViewport`, `setPoints`, and `setLocale`. The official static ledger accepts all seven, reports none missing, none scan-then-monolith, and no Mathematical-specific failure.

`MathematicalRetainedCommandWork` owns its graph, points, decoded operations, delete-id set, rewrite buffers, cursors, digest, and close state. It never invokes the legacy generic reducer. Node, edge, point, JSON-byte, move-node, delete-id-build, delete-node, node-order-restore, delete-edge, and edge-order-restore work advances through persistent microcursors. Admission parses at most 8,192 JSON bytes and rejects semantic maximum+1 before factory acceptance.

Checkpoint identity binds action ID, app instance, parent document, operation ID, generation, canonical base revision, and extent. A stale generation or different action cannot restore the checkpoint. Replay deterministically rebuilds the same typed mutation output through the same microsteps. Close releases decoded operations, delete IDs, rewrite buffers, points, graph edges, graph nodes, and graph metadata incrementally, and repeated terminal close is idempotent.

## Schema-First Law and Oracle

- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🧪️fixtures/mathematical-retained-command.schema.json`
- `✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️mathematical/🧪️fixtures/mathematical-retained-command-law.json`
- Ajv 2020 exit `0`: 7 exact actions, 14 hostile laws. Evidence: `🧪️sol-mathematical-retained-command-ajv-2026-08-26.txt`.

Rust coverage is source-complete for exact factory keys, proof count, nodes/edges/points/JSON/operation/delete/text/locale maximum+1, interruption/replay output through serde_json, action and generation ABA rejection, cancel-before, cancel-after, repeated close, and maximum-shaped `<8ms` work/close turns.

## Fresh Static Evidence

- `bun ./📜️script.ts verify interactivity tool-jobs --self-test`: exit `0`, 468 clean.
- Full JSON verifier: expected aggregate exit `1`; Mathematical accepted `7`, remaining `0`, scan-then-monolith `0`, failures `0`.
- `rustfmt --edition 2021 --check` and `git diff --check` on owned source: exit `0`.
- Evidence: `🧪️sol-mathematical-retained-source-gate-2026-08-26.txt`, `📊️sol-mathematical-retained-tool-jobs-2026-08-26.json`.

## Pending Compiler Gate

No Cargo or Nx command was started because the compiler lane remains reserved ahead of this cohort. Runtime acceptance is not claimed. The focused Mathematical Rust tests, native/Wasm library checks, descriptor command, and live gate remain queued for an explicit lease.
