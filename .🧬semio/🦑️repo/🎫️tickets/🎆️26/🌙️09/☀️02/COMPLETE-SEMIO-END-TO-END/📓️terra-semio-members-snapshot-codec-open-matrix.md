# SemioMembers Retained-Open Codec Matrix

Status: source-audited only. No source was changed and no build or native law was run.

## Scope and result

`SemioMembers` has exactly 18 closed factory arms. Flow is the sole arm with a real request-owned, byte-at-a-time snapshot decoder. The remaining 17 have legitimate offline `ArtifactDsl` text and `ArtifactPack` binary codecs, and all have typed store-retirement factories, but none is an incremental retained-open decoder.

They must not be routed through the legacy `MemberFactory::open` during the public open migration. That method borrows a whole `&[u8]`, copies both document components, fully decodes SPR/history, decodes a complete snapshot, replays all operations, and only then constructs a store. It is not cancellation-safe or bounded by the request/grant lifetime.

## Shared evidence

- [`SemioMembers`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:1121) is the single 18-arm closed declaration: every arm has kind `s.stdio.semio`, standard `v1`, its subset, and schema `stdio.semio`.
- The root’s [`semio_subset_table!`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🦀️.rs:82) drives the exact per-snapshot `MemberStoreOwner` implementations at line 822–1117. Each arm installs a snapshot-retirement factory, owned-value/mutation retirement, and `SemioStoreOwnedDisposer`; none may return an `ArtifactStore` without calling `P::member_store_owners()`.
- The generated [`MemberFactory::open`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:18345) delegates every arm to [`open_member_store`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:3180).
- That legacy helper calls [`decode_document_pack_bytes`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:9375), which creates two unbounded `Vec`s; then fully `decode_history`, validates identity, and calls [`parse_decoded_document_spr`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:11037). The latter invokes `P::decode_pack`, decodes both operation directions for every edit, applies all forwards, and creates ledgers. It is a correct legacy replay path, not a bounded public-open primitive.
- Every non-Flow pack decoder calls [`semio_format::unwrap_binary`](../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs:127), which copies the entire payload to `Vec<u8>`. Its `PackDecodeOptions` argument is explicitly unused. Thus ordinary codec round trips do not prove streamed size/fuel/cancel/retirement behaviour.
- Flow has a materially different component: [`SemioFlowSnapshotDecode`](../../../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️flow/🧬️schema/📸️snapshot/💾️binary/🦀️.rs:25) owns `MemberOpenRequest`, copies one byte per request step, checks authority before/after each byte, applies explicit node/edge/string caps, and owns bounded close. Its old `ArtifactPack` implementation remains whole-buffer and is not the new public open path.

## Arm matrix

All paths below are rooted at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets` and name the actual `schema/snapshot/🦀️.rs` source file. “Text” is the arm’s `ArtifactDsl` implementation; “pack” is its binary `ArtifactPack` decoder.

| Arm and concrete store snapshot | Text / pack decoder | Retained status | Typed owner and replay requirement | Smallest future decoder batch |
| --- | --- | --- | --- | --- |
| Animation — `SemioAnimationSnapshot` | `✳️animation`: `ArtifactDsl` 610; `decode_animation_snapshot_binary` 589; pack 650 | Whole slice only | Root macro instantiates exact owners. Must replay animation mutations after snapshot. | Nested timeline/channel/keyframe reader, after scalar collection primitive. |
| Audio — `SemioAudioSnapshot` | `✳️audio`: text 366; binary 332; pack 406 | Whole slice only | Exact owners exist; channels/samples/tags require per-field and aggregate byte caps before history replay. | Media batch with Image/Video; do not accumulate sample payload in one `Vec`. |
| Brep — `SemioBrepSnapshot` | `✳️brep`: text 1274; binary 1222; pack 1295 | Whole slice only | Exact owners exist; vertices/edges/loops/faces need bounded cross-reference decoding before replay. | Geometry batch with CAD/Mesh/Model. |
| Cad — `SemioCadSnapshot` | `✳️cad`: text 579; binary 548; pack 600 | Whole slice only | Exact owners exist; layer/block/entity collections and variant geometry precede replay. | Geometry batch with Brep/Mesh/Model. |
| Document — `SemioDocumentSnapshot` | `✳️document`: text 570; binary 534; pack 591 | Whole slice only | Exact owners exist; styles/images/block tree needs depth and aggregate counts. | Structured tree batch with Presentation/Drawing/Animation. |
| Drawing — `SemioDrawingSnapshot` | `✳️drawing`: text 785; binary 758; pack 806 | Whole slice only | Exact owners exist; styles/layers/path-node variants need fixed depth/count limits. | Structured tree batch with Document/Presentation/Animation. |
| Flow — `SemioFlowSnapshot` | `✳️flow`: text 321; old binary 277; pack 342 | **Actual retained snapshot component** at `snapshot/💾️binary/🦀️.rs` | Exact owners exist. Still lacks selected-factory → retained history/replay/store bridge; current `MemberFactory::open` remains legacy whole-history. | First concrete factory arm; no PluginApp publication in this packet. |
| Graph — `SemioGraphSnapshot` | `✳️graph`: text 490; binary 466; pack 511 | Whole slice only | Exact owners exist; node/edge ids and endpoints need counts, uniqueness and reference checks before replay. | Graph/collection batch with Table and Value, after Flow proves the bridge. |
| Image — `SemioImageSnapshot` | `✳️image`: text 406; binary 368; pack 427 | Whole slice only | Exact owners exist; dimensions/colour/sample data need byte and dimension product limits. | Media batch with Audio/Video. |
| Kit — `SemioKitSnapshot` | `✳️kit`: text 645; binary 626; pack 665 | Whole slice only | Exact owners exist. Contains `ArtifactChild<Object/Model/Value>` refs. Decode creates unmaterialized handles only; later child loading must go through declared parent projection, never default/local-owner substitution. | Composition-reference batch with Object, after public selected child-open bridge. |
| Mesh — `SemioMeshSnapshot` | `✳️mesh`: text 582; binary 514; pack 603 | Whole slice only | Exact owners exist; mesh/material/texture cardinality and byte caps precede replay. | Geometry batch with Brep/Cad/Model. |
| Model — `SemioModelSnapshot` | `✳️model`: text 853; binary 822; pack 874 | Whole slice only | Exact owners exist; spatial/elements/relations need a bounded reference graph. | Geometry batch with Brep/Cad/Mesh. |
| Object — `SemioObjectSnapshot` | `✳️object`: text 304; binary 287; pack 324 | Whole slice only | Exact owners exist. `brep`/`mesh`/`properties` are child handles. `ArtifactChild::new` is identity-only, so a completed decoder must not pretend their payloads are loaded. | Composition-reference batch with Kit. |
| Presentation — `SemioPresentationSnapshot` | `✳️presentation`: text 531; binary 494; pack 552 | Whole slice only | Exact owners exist; masters/layouts/slides and nested content require bounded tree decode. | Structured tree batch with Document/Drawing/Animation. |
| Table — `SemioTableSnapshot` | `✳️table`: text 278; binary 254; pack 299 | Whole slice only | Exact owners exist; columns/rows/cells require row/column/cell and total-text bounds. | Collection batch with Graph/Value. |
| Text — `SemioTextSnapshot` | `✳️text`: text 298; binary 279; pack 319 | Whole slice only | Exact owners exist; run counts and UTF-8 marks need a retained string/list decoder. | First non-Flow leaf arm; best minimal proof of the general per-byte decoder trait. |
| Value — `SemioValueSnapshot` | `✳️value`: text 157; pack 178 directly parses UTF-8 recursive value text | Whole slice only | Exact owners exist; ordered nodes and recursive value graph need recursion-depth, node and reference caps. | Collection/graph batch with Graph/Table. |
| Video — `SemioVideoSnapshot` | `✳️video`: text 336; binary 297; pack 376 | Whole slice only | Exact owners exist; streams and codec/media fields need byte/capacity bounds. | Media batch with Audio/Image. |

The matrix deliberately excludes `✳️base::SemioSnapshot`: it is the 18-variant outer union, not an arm in `SemioMembers`.

## Strict-owner and default-substitution hazards

1. `ArtifactStore` has a strict terminal `Drop`; a failed async decode/replay cannot let a partly initialized store or typed candidate fall out of scope. The selected operation must retain it and use the arm’s `MemberStoreOwner` disposer until terminal.
2. `ArtifactChild` is a serialised identity handle; `Object` and `Kit` decode it with `local_owner: None`. That is acceptable only as an unmaterialized reference. A decoder must reject an undeclared child on later load and must not make a default snapshot, clone a local owner, or hydrate from a surface cache.
3. Snapshot `Default` implementations are construction conveniences, not an empty-input fallback. The existing generic genesis helper already rejects an empty initial pack. Every new incremental decoder may build a partial default internally but may hand out its snapshot only after exact EOF, schema/dialect checks, and full required-field validation.
4. The package `ArtifactPack` options are ignored by all 17 legacy arms. Do not call those implementations from the retained operation merely after obtaining verified input; that would reintroduce whole-input copying and bypass request fuel/cancel checks.
5. SPR semantic verification is arm-independent but typed replay is not: each mutation enum must decode only after the selected schema/dialect arm is fixed. A generic `DslValue` or default mutation fallback would let a foreign kind reach a typed store.

## Dependency-ordered implementation batches

1. **Shared factory/open core:** keep request, verified history witness, selected declaration, and typed candidate private; add closed per-arm `begin_open` routing in `space_members!`; a byte/fuel source is sealed. It may return one selected child candidate but must not publish to `PluginApp`.
2. **Flow only:** adapt the existing `SemioFlowSnapshotDecode` into that source, then add retained history/operation replay/init and its arm-specific `MemberStoreOwner` installation. This is the only current decoder that can enter an actual bounded-operation proof.
3. **Text only:** add a retained run/UTF-8 decoder, with explicit run/string totals. It proves a second arm without claiming a shared full-buffer decoder.
4. **Collection/reference readers:** Value, Graph, Table. Share scalar/varint/UTF-8 *primitives* only; retain arm-local recursion, uniqueness, row, node, and reference limits.
5. **Structured documents:** Document, Drawing, Presentation, Animation. Add arm-local tree/variant state machines and fixed depth/count/budget rules.
6. **Geometry:** Brep, Cad, Mesh, Model. Add bounds and cross-reference admission before typed history replay.
7. **Media:** Audio, Image, Video. Define media data limits and page-retained payload ownership; no single large body accumulator.
8. **Composed references:** Object and Kit. Only after the selected-child factory/open contract and parent child-projection admission are mounted. These decoders may yield reference-only parent snapshots; child materialization remains a separate retained transaction.

Each batch needs a schema-first neutral corpus with normal, split-grant, cancellation-at-every-field, malformed/trailing/nonminimal, cap, wrong declaration, and exact-retirement rows, then a Rust law using the real member owners. A generic all-18 positive law is legitimate only after every arm has its own retained decoder and replay owner; until then all unimplemented arms must fail closed.

## Current acceptance boundary

Current source proves offline text/binary round trips and typed store disposal factories for the 18 arms. It does not prove a public retained `MemberFactory::open` for any arm, including Flow, because no selected arm is yet threaded through verified history, bounded replay, typed store initialization, and private handoff. No app map, graph, socket, or browser activation is implied.
