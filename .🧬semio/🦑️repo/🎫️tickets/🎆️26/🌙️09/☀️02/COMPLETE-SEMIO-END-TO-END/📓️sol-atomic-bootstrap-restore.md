# Atomic Artifact-Bootstrap Restore

## Scope and result

Implemented the P3 client-side restore boundary for the public `ArtifactBootstrap` protocol in the native store actor, wasm store actor, and browser fallback worker. The implementation consumes the shared P1 `ArtifactBootstrapAssembler`; it does not reinterpret the database-private `Snapshot`/`.spk` vocabulary and does not change the replication wire or fixture.

The native and wasm actors now bind bootstrap identity to the selected document and registered document codec, stage the canonical `(pack, spr)` pair until the assembler validates it, and install it through the store `Snapshot` vocabulary before publishing `SnapshotReplaced` or advancing the frontier. The browser fallback implements the same envelope state machine and uses one existing atomic folder-envelope `PUT` for the pair when folder persistence is configured.

## State-machine invariants

- The descriptor/version, document/schema, nonzero registered pack-schema hash, welcome/required-tail identity, chunk ordering/counts, per-part and aggregate hashes, byte/chunk budgets, deadline, cancellation, and monotonic bounded progress are checked before install.
- Native and wasm perform typed semantic validation through the registered document codec's `print_mirror` seam, plus SPR operation-id decoding, before store replacement.
- `current_pack`, `current_spr`, `SnapshotReplaced`, baseline frontier, pending resume token, and tail requirement change only after replacement succeeds. Live/resume is committed only when the received frontier exactly equals the authenticated required frontier, including document, ordinal, head edit id, commit sequence, and chain hash.
- Presence remains observable during catch-up but cannot advertise native Live or overwrite the Live peer count before the bootstrap/tail gate clears.
- A failed wasm tail-store push does not advance frontier or Live state.
- Pending local operations are deduplicated and retained across failure/disconnect. After a native baseline succeeds, a local replay failure does not roll the baseline back: it emits a conflict, preserves the outbox, reconnects, and replays once on the fresh transfer.
- Cancellation, malformed transport bytes, descriptor/chunk mismatch, premature tail/ack, disconnect, and replacement failure discard only staging and retain the last committed pair/frontier. A later Welcome starts a fresh assembler.
- Native, wasm, and browser reject database-private `Bootstrap::Snapshot`, `SnapshotChunk`, and `SnapshotDone`; none decodes those bytes as an artifact pack.

## Browser semantic boundary

The TypeScript fallback has no registered artifact-codec seam. It therefore validates only the authenticated bootstrap envelope: protocol/descriptor identity, configured pack-schema hash, exact frontier identity, lengths, chunk ordering/counts, hashes, budgets, deadline, and cancellation. Those checks are not equivalent to decoding the artifact through a registered typed codec, so this report makes no browser typed-semantic-validation claim. The fallback does use one atomic pair-envelope `PUT`; its negative test proves a failed `PUT` leaves the prior pair and frontier untouched and cannot reach the `snapshotReplaced` emission path. Rust native and wasm are the implementations that perform registered-codec semantic validation.

## Focused evidence

| Gate | Result |
|---|---|
| Browser worker: `bun nx run '@semio-tech/framework-os:test-quick' --skip-nx-cache -- --run '🟦️backbone-worker.ts'` | PASS — 1 file, 28/28 tests. Includes exact neutral-fixture inline/chunked equality, monotonic progress, exact authenticated frontier, wrong-head/wrong-chain rejection, malformed/cancel/disconnect no-commit and restart, private snapshot-frame rejection, pending-local dedup/preservation, and failed atomic folder PUT. |
| Native production MCP build: `RUSTFLAGS='-Awarnings' CARGO_TERM_COLOR=never CARGO_TARGET_DIR='<ticket>/🗑️generated/atomic-bootstrap-restore/mcp-target' bun nx run @semio-tech/framework-os-mcp-rs:build --skip-nx-cache` | PASS — current-tree `semio-os-mcp` emitted in 6m 50s. This compiles the native store bootstrap path and clears the previous two non-exhaustive store-frame blockers. |
| MCP executable-backed quick: `SEMIO_OS_MCP_BIN='<emitted binary>' bun nx run @semio-tech/framework-os-mcp:test-quick --skip-nx-cache` | PASS — 5 files, 41/41 tests in 17.07s. Exactly 30 process-backed cases executed: 12 end-to-end + 4 process hygiene (beside 1 AJV-only case) + 8 legacy + 6 modern. No absence skip/`skipIf` remains. |
| Native all-features kernel library check: `RUSTFLAGS='-Awarnings' ... bun nx run '@semio-tech/framework-os-kernel:check' --skip-nx-cache -- --lib -p semio-framework-os-kernel --all-features --message-format short` | PASS — final current tree, 6.97s cached rerun. |
| Wasm sync kernel library check: `RUSTFLAGS='-Awarnings' ... bun nx run '@semio-tech/framework-os-kernel:check' --skip-nx-cache -- --lib --target wasm32-unknown-unknown --no-default-features --features sync -p semio-framework-os-kernel --message-format short` | PASS — final current tree, 35.23s cached rerun. Three derive warnings still print despite `RUSTFLAGS`; none belongs to the owned files. |
| Focused native bootstrap tests | BUILD-BLOCKED, 0 executed. The broad lib-test dependency graph fails outside this packet at `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust/../../📇️registry/🦀️.rs:204`: `ProgramContributionEntry: serde::Serialize` is not satisfied. The source contains three focused tests, but no pass claim is made. |
| Wasm `sync,worker` check | BLOCKED outside this packet after this packet's own match/move diagnostics were cleared: `store/👷️worker/🦀️.rs` uses async `store_worker_open`/host calls without awaiting them at lines 41, 48, 52, 55, and 80. The wasm actor itself passes the `sync` library check; no worker-runtime test claim is made. |

The executable quick matrix is real subprocess coverage of the MCP binary. It is not evidence of a real hub reconnect, hub bootstrap authority/retention, or a typed browser codec. The browser tests call the fallback state machine with the committed neutral wire fixture and fake transport/storage boundaries.

## Tests added

- Rust shared negative: same ordinals/commit with a different authenticated head id or chain hash cannot satisfy catch-up.
- Native actor: a committed baseline followed by injected local replay failure preserves one outbox owner, Presence cannot bypass catch-up, a fresh bootstrap replays once, a divergent same-ordinal frontier stays non-Live, and the exact frontier reaches Live.
- Native actor: inline and chunked fixtures install byte-identical typed pairs; cancellation and pre-Done chunks are invisible; a fresh transfer succeeds.
- Browser fallback: inline/chunked fixture equality and frontier equality; wrong identity; malformed/private/cancel/disconnect no-commit with restart; pending local edit preservation; failed atomic pair PUT.

## Owned files

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🟦️backbone-worker.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/📓️sol-atomic-bootstrap-restore.md`

No runtime dependency, replication fixture, hub/DB, plugin/MCP codec, actor-return, WGPU, root manifest, AGENTS, goal, or ticket lifecycle file was changed by this packet.

The packet-owned `🗑️generated/atomic-bootstrap-restore` build outputs were removed from the ticket after verification by moving the directory to the macOS Trash. Other agents' generated outputs were not touched.
