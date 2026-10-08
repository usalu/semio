# Remodeling Twelve Tags, Domain Policy, Native Tests

Read-only current source audit; no tests run.

|Tag|Field|Value type|Aggregate maximum|
|---|---|---|---|
|0|positions|f32|1536|
|1|normals|f32|1536|
|2|colors|f32|2048|
|3|indices|u32|1536|
|4|uvs|f32|1024|
|5|face_ids|u32|512|
|6|vertex_ids|u32|512|
|7|edge_positions|f32|3072|
|8|edge_ids|u32|1536|
|9|edge_uvs|f32|2048|
|10|edge_is_seam|u8|1536|
|11|paint_texture_base64|UTF-8 text|24576 bytes|

Actual authority: domain root `apply_mesh_chunk` lines426–487. Tags must be nondecreasing; repeated tags are lawful and preserve distinct source boundaries. Float/u32 payloads must be divisible by4 individually, not merely after concatenating chunks. Empty tagged chunks are accepted. Empty tag11 changes None to Some(empty), so preserve a text chunk even with zero bytes. Tag10 accepts all u8 values; do not add boolean0/1 restriction. Tag11 accepts arbitrary UTF-8 text; its field name does not impose actual Base64 validation. All IEEE bit patterns accepted; do not require finite floats. Full mesh validation lines525–549 checks triples, vertex/triangle maxima, indices in vertex extent, optional normals/colors/UV/faceID/vertexID shape. It does not require edge arrays to agree with each other, positive provenance IDs, or indices/IDs uniqueness.

Snapshot storage accepts arbitrary String kind and Vec<ByteBuffer>; its schema does not establish chunk validity. Existing owned fixture lines62–63 deliberately holds kind `unresolved\0世界`, two arbitrary byte chunks (one empty), and huge width. `remodeling_content_is_complete`390 accepts bounded sparse bytes without whole-word checking; sparse resolver629 rejects any individual incomplete word by returning an empty Vec. Mesh completeness requires full valid mesh, while missing/external reference metadata is legal. Therefore raw schema acceptance must not be mislabeled semantic validity.

Recommended explicit policy: known `sparse` chunks must be individually bounded and divisible by4; expose every f32 word and boundary, including empty chunks. A sparse durable store need not currently be a full xyz cloud: confidence buffers share this same kind, so no universal multiple-of3 requirement. Known `mesh` chunks require nonempty tag, tag0–11, nondecreasing order, individually aligned numeric words/UTF8, bounded aggregates and existing mesh envelope. Reject invalid known structured content with ValueError before producing output; do not drop tail bytes, normalize tag order, coalesce boundaries, emit an opaque fallback, or silently manufacture valid geometry. Apply identical validation in native export/project and SQL reconstruction/import. Error leaves original Snapshot unchanged. This deliberately makes semantic IO a validated subset of the currently permissive storage schema; record and test that domain policy openly. Preserve unresolved external FloatBuffer/child references without imposing local existence/count consistency; only validate locally stored structured payloads. Unknown content kinds remain literal binary artifact data, as existing fixtures require; do not relabel known malformed sparse/mesh to unknown automatically. A later domain schema redesign can distinguish admitted structured content from generic binary content at construction.

Negative neutral inputs: sparse3-byte leaf; mesh missing tag; tag12; descending tags; numeric5-byte body; text invalid UTF8; positions not triples; index outside vertex extent; inconsistent normals; oversized aggregate. Validate refusal before publication and source unchanged. Positive boundaries: two same-tag positions leaves, empty numeric leaf, empty tag11, seam255, nonfinite/signed-zero f32. Distinguish empty chunks from absent chunks and optional empty texture from None.

Existing native helpers are in SQLite snapshot `🧪️tests/🧬️owned/🦀️.rs`: fixture58, database79, restored80, same81 (both native DSL and Pack comparison), oracle82 (bun:sqlite independent database reader), complete native test84, payload36. Existing public IO declaration test19–21 registers actual tree artifact using Plugin builder then `io_export_sqlite_snapshot`/`io_import_sqlite_snapshot` for both Text/Binary encodings. Extend this test's expected Snapshot with the positive neutral source witness, rather than only default scene. Independently query named field/type and numeric words, boundary row counts, NULL octets for sparse/mesh, present image/unknown octets, ownership/foreign key integrity; serialize third-party DB and import, compare same plus exact original durable chunk bytes. Current oracle84 queries raw chunks and table count from old fixture; update its expectations with the authored schema, preserving unknown-kind original fixture as binary control.

Current canonical observer readback: ErasedSnapshotRetirement contract line14 now returns Result for copy demand; handoff line56 delegates cursor error unchanged, matching capacity/release/depth lines57–59, with no demand expect/MAX sentinel. This is source evidence only. Remaining expect in handoff finish32 is a validated take invariant, not demand handling. Separate candidate demand expects remain Paged UTF8 append103 and Neural retirement103; inspect exact trait/caller context before classifying them as this migration scope.

[All twelve tags positive source input](📥️inputs/semantic-independent-oct8-remodeling-content-all-twelve-tags-witness.json) extends the eight-tag witness with colors, edge_positions, edge_ids and edge_uvs. It has twelve ordered tagged chunks and preserves the same sparse witness. No runtime qualification asserted.

Later concurrent readback: Paged append103 now propagates Result too. Its depth106 still maxes1 for any nonterminal cursor; close88–94 can merely detach source/destination references with item-only accounting, so this is an overestimate rather than exact depth. Pending owned birth requires1; live ticket delegates depth; final borrowed-reference detach requires0 under present implementation. Neural retirement103 still holds unpropagated copy Result and expects on other observers; execution peer informed, not a completed implementation judgment.
