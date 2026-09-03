# Hub Lag Rebootstrap

## Outcome

P2-C replaces both silent broadcast-lag branches with a typed, scope-authorized `rebootstrap-required` control followed by WebSocket close code 1013 and reason `rebootstrap-required`. The control is derived only from the current durable descriptor, active public checkpoint, and matching verified private checkpoint. It exposes structural space/document scope, checkpoint ID, descriptor digest, and baseline frontier; it never serializes the checkpoint's private BLAKE3 storage locator or DB `.spk` bytes.

If authorization is absent, the verified checkpoint is absent, the public/private projection differs, the checkpoint identity is invalid, or the descriptor digest rotated, the hub emits no control. It still terminates the lagged stream, so no receiver can resume an already discontinuous tail.

## Contract

The directory schema now owns `RebootstrapRequired` and the tagged `DirectoryStreamMessage` member `rebootstrap-required` in Rust, TypeScript, and JSON Schema. Replication wire v2 owns the corresponding `ServerFrame` tag 12 in Rust and TypeScript. Both codecs require:

- non-empty space and document IDs capped at 256 UTF-8 bytes;
- nonzero 32-byte checkpoint and descriptor identities;
- a non-empty frontier head edit ID;
- frontier document identity equal to the structural document scope.

The shared JSON fixture fixes close code/reason and the 256-byte scope, 4 KiB inline/chunk, 16,384-chunk, and 64 MiB aggregate ceilings. AJV validates the public DTO and rejects an injected `storageKey`; Node's `crypto` and a separate manual varint/UTF-8 encoder reproduce the exact framework bytes and SHA-256.

`VerifiedRebootstrapSource` is a hub-owned port over the completed directory/checkpoint and immutable-CAS seams. Metadata selection and pair reads have an absolute 15-second deadline, cancellation checks, monotonic progress stages, fixed aggregate/chunk limits, exact pack/SPR/aggregate SHA-256 verification, and deterministic 4 KiB chunk production. Native, wasm, and browser clients treat the typed control as a hard discontinuity: they validate the bound scope/frontier, preserve the last installed pair, reject partial bootstrap assembly, deduplicate the retained outbox, and reconnect instead of advancing the tail.

The native WGPU shell handles both new public variants explicitly. `BootstrapProgress` is retained in shell state and rendered in the existing sync status surface. Directory `RebootstrapRequired` cancels the stale stream and starts a fresh `stream(0)` session through the existing directory client, forcing canonical replay and membership revalidation instead of continuing the discontinuous receiver.

The document live loop reauthorizes before emitting the binary control. The directory live loop requires an authenticated current space member before emitting the JSON control. Both close 1013 regardless of whether disclosure is allowed. Test-only zero-permit admission gates and capacity-one broadcast channels make the lag condition deterministic while the transport still runs over real loopback WebSockets.

## TDD Evidence

- First TypeScript/neutral-oracle run: RED because `encodeServerFrame` existed in replication but was absent from the OS package's explicit exports. After the export was added, `bun nx run os-hub-ts:test --skip-nx-cache --verbose` passed 5 tests with the opt-in live scenario skipped.
- First focused Rust run: RED because the new directory DTO was not re-exported from `os_directory`. After the re-export, the next run reached only concurrent trusted-loader compile diagnostics; their owner fixed those.
- `CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-lag-rebootstrap-target" bun nx run os-hub:test-quick --skip-nx-cache --verbose -- lag_rebootstrap`: passed 2/2 focused fixture/boundary tests, 60 filtered.
- The first two real-socket runs exposed stack exhaustion while constructing the existing large fixed DB test owner on the default test stack. The socket cases execute inside an explicit 32 MiB owned test thread, and the existing DB worker pool additionally requires the established external `RUST_MIN_STACK=33554432` test-harness setting; neither changes production allocation or the wire contract.
- `RUST_MIN_STACK=33554432 CARGO_TARGET_DIR="$PWD/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️02/COMPLETE-SEMIO-END-TO-END/🗑️generated/hub-lag-rebootstrap-target" bun nx run os-hub:test-long --skip-nx-cache --verbose -- socket_forced_lag`: passed both real loopback WebSocket tests, 63 filtered. The document test proves verified control then close 1013 under actual broadcast lag. The directory test proves the same and proves a non-member of another space receives no checkpoint disclosure.
- Final `bun nx run os-hub-ts:test --skip-nx-cache --verbose`: passed 6 tests with the opt-in live scenario skipped. This includes the AJV schema oracle, Node SHA-256 oracle, independent wire encoder, private-field rejection, and scope/cap assertions.
- `git diff --check` over the lag source, hub Rust/TypeScript packages, replication wire, and WGPU shell passed.
- The focused native command `RUST_MIN_STACK=33554432 bun nx run @semio-tech/framework-renderer-wgpu:test-native --skip-nx-cache --verbose -- sync_pill_text_covers` reached `cargo test --no-run --package semio-framework-os-renderer-wgpu` and remained in the broad shared dependency build for more than three minutes without a source diagnostic. It was bounded and stopped as infrastructure contention. The two new WGPU match arms are explicit, and no known non-exhaustive match remains, but this report does not claim a completed native package compile.
- The dedicated 4.6 GiB `hub-lag-rebootstrap-target` test output was deleted after all hub gates completed; no shared target or peer output was removed.

## Honest Residual Seam

This packet does not claim checkpoint publication is scheduled in production. Startup can load the new opt-in trusted catalog, but the audited deployment is still missing a complete 59-root package/native-provider bundle and instantiated-WASM-handle attestation. Until the authority scheduler produces checkpoints, lag correctly fails closed with 1013 and no control.

The verified public `(pack, spr)` loader and chunk cursor are implemented, but document `Hello` still uses the DB engine's private bootstrap path. The DB engine keys a document as an encoded structural scope while the artifact frontier carries the logical document ID, and there is no committed checkpoint-barrier-to-exact-DB-tail cursor contract. Therefore this packet does not claim canonical pair installation plus exact no-gap tail resumption. The next composition must add an authenticated checkpoint request/selection to `Hello`, pin a checkpoint barrier, stream `VerifiedArtifactBootstrap`, then replay the exact post-barrier tail without translating or trusting client identities. It must also resolve the authority's 64 MiB pair ceiling against the current immutable adapter's 496 KiB per-blob ceiling.

Physical cleanup of unreferenced immutable CAS blobs remains the P2-D retention/sweep responsibility.
