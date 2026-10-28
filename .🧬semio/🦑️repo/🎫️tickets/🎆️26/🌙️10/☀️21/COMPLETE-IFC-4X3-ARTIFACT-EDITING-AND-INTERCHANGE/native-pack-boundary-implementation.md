# Native IFC Pack Boundary Review Point

This is source implementation ready for independent review. Native execution, typechecking and complete host interoperability are unverified. The current cargo attempt stops before rustc with `Original compiler ingress requires the repository launcher and authentic enforcing store`; evidence: `🗑️generated/native-domain-pack-cargo.txt`. No compiler/build enforcement was bypassed. Repo MCP/CLI limitations and necessary direct command fallback remain documented in earlier reports.

## Original Receiving

The actual shared IFC model now implements `OriginalArtifactPackReceiving` and `ArtifactPackReceiving`, with opaque public `IfcOriginalPackReceiving` reexported by its snapshot facade. The receiver binds source pointer/length, original decoder ledger and original retirement recipient. Explicit steps and closure reject a changed source/owner. The literal flat Pack grammar preserves ADD2 header data; no IFC4 strict STEP parser is involved.

`OriginalSnapshotRecordReceiving<Frame>` performs the original envelope/Pack framing, parsing and typed authored binding. `GraphReceiving` moves original strings and binary64 values into retained graph slots. It checks active field shapes, forward-only child ownership, unique consumption, missing roots, orphan values, canonical binary content and value depth 64. Graph admission is at most 2,000,000 values and 1,000,000 entities, after the parser's separate source/Pack/physical wallet limits. Binary scans use at most 4096 bytes per turn. Source, parser, partially bound records and output remain retained on refusal.

The framework seam adds same-owner `RetireOwned` support to `OriginalSnapshotRecordReceiving<T>`. Both explicit and generic closure use its same physical close routine. Explicit closure still checks the authentic original decoder/recipient and charges its allocation port; generic cleanup is funded by the actual original retained-frame recipient. No synthetic control or grant is created.

## Original Publication

The public model-owned API is `IfcSnapshot::encode_exchange_pack(&self, &mut NativeSnapshotEncodeOwner) -> Result<Vec<u8>, ValueError>`. A funded canonical DFS index stores node/group/edge locators, borrowing all text and exact numeric payloads. A value lookup follows at most 64 parent locators. Header/entity/complex argument roots and node children use direct indexed metadata. Allocation, traversal and metadata assignments debit original authority with cancellation checks.

New shared `OriginalPackWriting` takes a `FieldProjectionSource`, `BorrowedRecordSpec` and declared envelope. The existing canonical `BorrowedPackDocumentCursor` writes directly into one final envelope buffer. It does not construct a complete RecordValue, raw Pack body or second wrapped wire. Scratch and output backing are admitted through the original encoder. Successful output enters an explicit draining phase until every codec scratch owner is terminal empty; final transfer has its own demand. Denied turns preserve owners without invoking the native work callback. Cancellation has separate scratch/final-prefix retirement.

Actual FEM publication now uses the same writer. Its `Fem3dSnapshot::encode_exchange_pack` API still borrows the actual persistent snapshot. The original successful-drain regression moved with the shared writer to avoid duplicate lifecycle logic.

## Normal Route Bindings And Remaining Boundaries

New IFC-specific `native_exchange::original_pack` wrappers replace the cumulative IFC Pack boundaries for all seven existing geometry/BIM/FEM IoEntry run functions. Export receives the actual source inside the original paired run, invokes the existing serializer and closes source custody before publication. Import retains its returned IFC graph/diagnostics, encodes through the new model API and admits the final outcome header before transfer. Typed whole IoError results remain retained during wrapper cleanup. Parent native direction and retirement custody stay scoped; no ledger/default grant is manufactured.

Normal declaration registration and central composer identity claims remain unchanged. No route or fidelity capability is added. Direct native IFC Packs are supported here. Embedded document archive carriers are outside this receiver subset and refuse their envelope. Conformance hooks require a funded producer; the wrapper explicitly refuses a nonempty hook instead of silently omitting it. Existing converters have no such hook.

| Direction | Boundary repaired | Converter scope and remaining ownership |
| --- | --- | --- |
| IFC → FEM | Original IFC receive; actual persistent FEM receive; original FEM Pack writer | Unloaded SI nodes, bars/zero-roll frames, shared materials/sections, boolean supports. Unsupported loads, releases, placements, solids and other engineering semantics refuse. |
| FEM → IFC | Actual FEM receiver/funded graph builder; original final IFC Pack | Same supported analytical subset; new IFC identities/axes, omitted solver/display settings remain diagnosed. |
| IFC → BIM | Original IFC receive | Existing spatial/storey mapping; BIM target encoding remains cumulative native. |
| BIM → IFC | Original final IFC Pack | Existing BIM source decoding/spatial graph projection remain cumulative/callback paths. |
| IFC → glTF | Original IFC receive | Existing bounded static triangulation; foreign target encoding remains cumulative native. |
| glTF/GLB → IFC | Original final IFC Pack | Existing bounded static triangles; canonical GLB padding guards; foreign decoding/projection remain cumulative/callback. |
| IFC → natural GLB | Original IFC receive | Existing geometry-only GLB encoding remains cumulative. |

Thus shared IFC source/final Pack custody and the actual FEM owner path are implemented for review. Physical ownership of every BIM/glTF conversion temporary, foreign codec and diagnostic allocation is not established by this work. No complete geometry, MVD, full-schema semantic, host-runtime or native lexical proof is claimed. Other workers own native STEP and EXPRESS/editor changes.

## Tests And Evidence

- New shared IFC `owned.feature`: complete literal/header/complex preservation, denied/cancelled prefixes, invalid forest refusal.
- Shared IFC `owned-tests.rs`: actual original receiving, exact NaN bits/i64/u64/binary/nested/complex values, original cancellation, denied receiving at 0/1/8/64/512/4096 turns with original recipient drain, invalid reused/orphan/backward/deep forests, and canonical publication byte equality against the existing independent reference codec. Written Rust tests, unexecuted.
- Shared store `writing-tests.rs`: actual Complete → Closing drain across multiple turns, zero-item/zero-release denial, wire pointer preservation and handoff. FEM `write-pack-tests.rs` exercises the actual persistent source API against the canonical reference document. Unexecuted.
- Registered FEM roundtrip now writes the graph decoded from the actual returned Pack to `native-fem-owned.ifc` for IfcOpenShell, instead of writing the pre-route source graph. The TS oracle requires genuine `SEMIO_IFC_NATIVE_ORACLE_OUTPUT`; absent native output it skips. No substitute reference output is presented as native.
- `🗑️generated/native-pack-syntax.json`: all 12 listed changed Rust leaves parse through rustfmt; `typechecked:false`, `nativeExecuted:false`. Syntax only.
- `🗑️generated/native-index-oracle.json`: SQLite 3.53.1 independently checks neutral 512/1024/10000-node lookup, duplicate and missing-reference decisions; reference decisions only, no Rust work measurements. Existing IfcOpenShell 0.8.3 source witnesses and official fixture provenance/checksums remain retained. No newly generated native output is oracle-verified.

## Touched Source

- Shared IFC4 literal native owner: new `owned.rs`, `owned-graph.rs`, `pack-index.rs`, `write-pack.rs`, `owned-tests.rs`, `owned.feature`; native mounts and receiving derives; snapshot facade public receiver reexport.
- Shared store native snapshot: same-owner retirement seam in `🫴️receiving/🦀️.rs` and prefix test; new `writing.rs`/`writing-tests.rs`; native snapshot/store reexports.
- FEM snapshot `write-pack.rs`/`write-pack-tests.rs` consume the shared writer. Prior sorted identity/receiving changes are listed in `native-domain-reaudit-fixes.md`.
- IFC4x3 native IO: new `native_pack.rs`; normal wrapper bindings in `native_exchange.rs`/`native_domains.rs`; registered FEM roundtrip oracle source in `🧪️tests/native_fem_owned.rs`.

No TS IO/editor/schema hooks, native STEP lexical source, build infrastructure, AGENTS, git state, or generated inputs were changed or cleaned.
