# Draw, Flow, and Forms Independent Audit Repair

## Outcome

The P8j findings and the two subsequent P8o P0 rejections are repaired, and the Draw, Flow, and Forms lane is source-audit-ready. This is a source/static claim only: no Cargo command ran and the authored Rust tests were not executed in this lane.

## Shared Public Dispatch Boundary

`ArtifactStore` retains concrete shared ownership privately and exposes repository-owned `SnapshotRead<T>` / `ErasedSnapshotRead` capabilities. `SpaceMember::snapshot_read_erased` and `ChildContentView::typed_read` preserve O(1) immutable reads without exporting `Arc`. The canonical child/store revision is an incrementally maintained repository-owned SHA-256 accumulator over domain-separated, length-prefixed initial/applied/redo/cursor/checkpoint records; load/reset reconstruct it outside interactive reach, while mutation-time work only reconciles the changed tail.

A maximum-child framework regression seeds a 4 MiB child, resets clone/encode counters, dispatches the real public `probeChild` action through the app command job to its first `DispatchAction` continuation, asserts `<8 ms`, and asserts zero maximum-snapshot clones and zero pack encodes on that dispatch boundary.

## Forms

- Public scalar and bulk payloads are chunk-addressable at the command/action codec boundary; every admitted leaf is at most 4,096 bytes and pathological `inputCount` is rejected before registry retention.
- Input staging is keyed by actual app instance, parent document, initiating operation, generation, canonical base revision, semantic command, and input id. Multi-action uploads retain the initiating operation; explicit reset/cancel removes only the addressed app/document's incomplete scalar and bulk sequences, and the 65th live input/session returns `Busy` without eviction.
- Scalar staging, existing option-array editing, existing object-map editing, vector rewriting, bulk object scanning, collision verification, and persistent-root commit are continuation-driven. Each step scans/copies one 4 KiB slice or at most 64 vector components. Bulk commit performs one compact root swap and preserves atomic single-undo behavior.
- Committed values own their bounded chunks in serialized config/DSL state; registry-clear serialized reopen retains completed content. Source-authored tests also cover same-document/two-app cancellation, malformed/order/identity conflict, abandoned expiry, pathological counts, and 65th `Busy`.

## Draw

- Gesture authority is no longer thread-local. Gesture and trace registries are keyed by app, document, operation, generation, and base revision/content identity, with explicit admission and bounded cleanup rather than eviction.
- Trace paths are fixed-capacity depth handles and every traversal advances at most the declared node/depth budget; no continuation clones a growing ancestor path.
- Source-authored public tests overlap two documents, cancel only the addressed document, reject a foreign/stale checkpoint, reconstruct after both trace registries are cleared, cap adversarial depth, return `Busy` on the 65th trace, exercise operation/event-log/diff/apply routing, and assert each measured maximum step `<8 ms`.

## Flow

- The production Flow editor is `VcsArtifactApp<EditorApp<FlowPlayApp>, SemioMembers>` through the composition-aware builder. The testkit uses the same shape and registers the real `SemioFlowSnapshot` child; no `compose/` source was changed.
- Duplicate-widget has no live process-map authority and no silent eviction. Its durable checkpoint is keyed by app, document, operation, child, and generation; every continuation verifies the captured fixed child revision before taking the O(1) child snapshot handle.
- The obsolete whole-child pack digest phases were removed. Search examines at most 64 nodes/edges per step, terminal node/parameter envelopes are bounded, and the final child edit is one compact composed emit. Supersession emits `CancelDuplicateWidget { generation }`, so one app/document cannot cancel another.
- Source-authored public tests cover command encode/decode, real composed child diff/apply, config-operation log replay into a fresh app after clearing process scratch, two simultaneous apps/documents, exact supersession cancellation, stale content, oversized `Busy`, and per-segment `<8 ms` assertions.

## Executed Non-Cargo Evidence

- `bun ./📜️script.ts verify interactivity tool-jobs`: 775 command rows, all 775 bounded, zero batch-only rows, zero forbidden rows, zero deleted rows, zero failures; one production factory, registration, and dispatch.
- Focused `rustfmt --edition 2021 --check`: clean for owned Forms, Draw, Flow, framework store/plugin, and Flow builder Rust sources after formatting the one reported Flow config layout.
- Static census: zero former `snapshot_arc`, `snapshot_any`, `typed_arc`, Forms public `arc`, removed committed-content registry accessors, or play-app constants used as continuation authority.
- Pre-serde action and command admission share exact addressed-envelope classification and have exact-boundary fixtures for Forms 16 KiB and Draw/Flow 8 KiB, plus maximum-plus-one, malformed structure, hostile string, escaped-key/value bypass, and wrong-path decoy coverage.

## P8o Final-Audit Repair

- Public plugin command dispatch now validates the raw command envelope before manifest-command deserialization. The action and command validators share one allocation-free classifier for the exact address action/command identifier location and one DFF command-name limit table. Escaped semantic keys/identifiers cannot bypass classification.
- Source-authored endpoint fixtures send exact-maximum, maximum-plus-one, malformed, and hostile command bodies through public command dispatch; the matching public action path is covered in the same fixture. The exact-maximum cases are required to reach post-decode ownership validation, while maximum-plus-one is required to stop at the DFF wire bound.
- Forms scalar and bulk public-action fixtures no longer treat the content-ID accessor as raw JSON. They assert the bounded 85-byte content ID, materialize the owned chunk leaves, serialize completed config, clear staging/input/session registries, cold-reopen, and reassert both the stable ID and raw content. The obsolete no-op completed-content registry test seam was removed; completed payload authority remains the serialized chunk leaves.
- Focused Rust formatting is clean for the framework plugin and the three owned Forms files. The static interactivity verifier exits 0 with 775/775 bounded rows, zero batch-only/forbidden/deleted rows, one production factory/registration/dispatch, and zero failures. Focused static scans find one public command handler, its DFF validator before serde, no stale raw-value Forms assertions, no obsolete completed-content replay helper, and no added debug output.

## Deferred Runtime Gates

Cargo/native/Wasm tests, compilation, clippy, allocation measurement, and measured runtime execution remain owned by the separate Cargo lane. The `<8 ms` assertions above are source-authored coverage, not reported runtime passes. No runtime test is claimed passing, and no runtime behavior is claimed confirmed by console logs. No remaining source/static gap was found in the repaired P8j/P8o scope.
