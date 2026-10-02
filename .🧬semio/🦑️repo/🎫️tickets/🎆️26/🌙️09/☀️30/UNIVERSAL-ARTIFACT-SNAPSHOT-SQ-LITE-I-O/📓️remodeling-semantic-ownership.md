# Remodeling Semantic Snapshot Ownership

The Remodeling snapshot has nine persisted root fields: schema, id, streams, assets, durable artifacts, calibration, parameters, ground control points, and results. The current document schema is `remodeling.scene`, the native envelope is `remodeling.remodeling`, and the declaration is `s.remodel.remodeling` under version 1, any subset. The Rust owner has no SQLite snapshot opt-in or semantic SQL module at this inspection. Existing native Text and Pack use the declared DSL records; Text still accepts a bare body.

## Confirmed Entities And Relationships

Streams carry literal ids, names, media kind, optional camera id, two binary64 values, ordered frames, and optional video provenance. Frames have an unsigned 32-bit index, binary64 timestamp, and literal asset id. Video provenance has name, container, codec, binary64 duration, and three unsigned 32-bit counts.

Assets are a map from literal names to full composed image child handles. Each handle must retain child id and the four independent reference fields. The durable artifact map has literal names, kind, optional MIME, unsigned 32-bit width and height, and ordered string chunks. No runtime child owner or cache is persisted.

Calibration has ordered camera and rig entities. A camera has id, label, model, five binary64 intrinsic values, five binary32 distortion values, optional binary32 reprojection RMS, and a boolean lock. A rig has camera id, four binary32 quaternion components and three binary32 translation components. Ground control points have id, name, three binary64 coordinates, and ordered observations containing stream id, unsigned 32-bit frame index, and two binary32 pixel components.

The eight parameter entities are ingest, feature, matching, structure from motion, dense reconstruction, meshing, motion, and geography. Every integer count is unsigned 32-bit. Their floating scalars are binary32 except geography's three optional binary64 origins. Their six enum domains are explicitly declared by the Rust owner; SQL must retain those literal variants and reject others.

Results include optional sparse and dense clouds, mandatory mesh relationship and provenance, optional camera trajectory, ordered motion summaries, optional geographic product ids, and optional quality report. The mesh child carries all five literal child/reference values. Trajectory poses have literal camera ids plus seven binary32 components. Track summaries have id, unsigned 32-bit length, class, and binary32 speed. Quality has two binary64 values (one optional), three binary32 ratios, ordered warning strings, and optional watertight report. Both watertight occurrences have seven unsigned 32-bit counts, signed 64-bit Euler characteristic and optional genus, binary64 signed volume, optional unsigned 32-bit self-intersection count, and five boolean flags.

## Semantic Cloud Obligation

`PackedF32` and `PackedU8` are currently transparent String wrappers. Their helpers interpret base64 little-endian bytes; the floating wrapper can also refer to durable content chunks. The model accepts arbitrary strings and silently decodes malformed input as an empty buffer. Storing only these encoded strings would require codec knowledge to query cloud coordinates, colors, confidence, and classification. A complete semantic implementation therefore needs an explicit domain decision before activation: typed cloud components and ordered octets must be independently queryable, and accepted malformed spelling or durable references must have an explicit representation rather than disappearing during reconstruction. This inspection does not classify an encoded carrier as a completed provider.

## Verification State

## Canonical Cloud Model Work Planned On 2026-10-02

The universal objective requires replacing the transparent encoded cloud String wrappers with owned semantic buffers. Inline float buffers must contain exact binary32 samples; inline color/classification buffers must contain owned octets. Durable sparse content must become a tagged literal content reference with an independent content identifier and unsigned64 chunk count, rather than a parseable handle string. The corresponding durable chunks need an explicit byte interpretation at their file boundary. Existing native and Source consumers, fixtures and public declarations must move together; no old packed String model, alias, decoder fallback or provider-only alternate model will remain.

The SQL schema will expose ordered float samples with numeric query values, exact binary32 words and IEEE classes, plus ordered octets and explicit content-reference fields. Cloud presence, optional empty buffers and absent buffers must remain distinct. Owned buffer construction must admit rows and bytes before allocating samples, and support cancellation within large collections. Exact native Binary/Text round trips, independent DataView/SQLite word oracles, current actual editor/viewer imports and the whole package gates will be required before activating the provider.

This is a design record and verified source inventory, not a completed model change. Architect's expanded provider remains the root's active native execution lane.

## Mounted Executable Model Baselines — October 2

The actual version 1 snapshot now mounts four native SQLite-prefixed laws: actual parent codec capability, typed inline samples, raw durable octet chunks, and exact binary32 words. A shared neutral fixture and three independent Source/Ajv laws are mounted alongside them. All three Nx modes route through the existing Rust package script and have matching seed/live launch entries. The private uncached Source baseline is running; no assertion outcome is available yet. The native lane remains occupied by Architect and its next owning run will include these native baselines. The canonical model and SQLite implementation are still unchanged. Remodeling remains uncovered.

The Source baseline has now genuinely executed all three laws: zero passed, three failed, 583ms Bun runtime, 1m42s reported uncached Nx task run. The actual decoder refuses typed inline buffers and raw durable octet arrays because the old persisted fields require encoded strings, and the actual public relational function is absent. The independent Ajv buffer oracle admitted the same neutral typed inline payload before the owner refusal. Evidence: `🗑️generated/root-remodel-model-baseline-source.log`. Native counterparts remain queued and have no assertion result yet.

The hand-authored SQL contract is mounted with 35 semantic tables covering the complete nine-field root, eight parameter sections, camera calibration/rig/frame/GCP relations, independent child identities, durable raw chunks, tagged float buffers and exact binary32 sample words, byte buffers, mesh/watertight summaries, trajectory, tracks, geography and QC warnings. Movie codec domains were checked against the actual native `Avc/Hevc/Vp9/Av1/Mjpeg/Unknown` owner and corrected in the authored contract. The SQL schema is not a provider implementation and has not yet been executed by an independent engine.

Inspected owners: `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs`, its version 1 any snapshot Rust and TypeScript files, and the existing Rust package script and project targets.

## Actual Schema Oracle And Canonical Native Work — October 2

The owning registered Source run now executes four laws: the independent Bun SQLite schema oracle passes, while the three canonical model/capability laws remain genuinely failing. All 35 authored tables execute with their declared sample columns and foreign-key check. Evidence: `🗑️generated/root-remodel-authored-sql-source.log`, one passed and three failed in 344ms Bun runtime. This supersedes the earlier unexecuted schema status.

Native baseline runs stopped before owner assertions in the concurrent schema registry/state refactor; their compile failures are not model behavior RED. The already executed language-agnostic Source/Ajv model RED now drives the shared canonical native repair. Native production model uses tagged `Float32Buffer::Inline { values: Vec<f32> }` and `Content { content_id: String, chunk_count: u64 }`, and transparent `ByteBuffer(Vec<u8>)`. Durable artifact and append-content chunks own literal bytes. Native controlled DSL projection/construction uses the declared tagged variant methods and Bytes64 raw-byte physical syntax, charging each wrapper and collection before allocation. No encoded String buffer or compatibility alias remains in the native tree.

Native reconstruction, commit validation, imported mesh chunks, I/O consumers, serde unit laws and editor/viewer presentation consumers have been aligned. Base64 conversion remains at declared image and UI file/presentation boundaries, not the persisted cloud or durable-chunk model. Source/JSON/fixture counterparts are assigned to the Source worker; remaining authored native schemas and SQLite provider remain root work. None of these newly changed native behaviors is reported passing before execution.

The registry now physically has one definition and its real schema-state package dependency exists, so one changed-prerequisite native Architect/Remodeling run was dispatched. Its log is `🗑️generated/root-architect-remodel-typed-model-prerequisite-change.log`. Assertion results remain pending.

## 2026-10-02 11:23 UTC: Typed Provider Staging and Source Evidence

The Native parent now owns Float32Buffer::Inline { values } and Float32Buffer::Content { content_id, chunk_count }, with Vec<ByteBuffer> raw durable leaves. The obsolete string content URI helpers and their sole CreateAsset special-case guard were removed; file transport validation remains at its declared boundary. Hand-authored relational projection/reconstruction, exact row/schema forecasts, controlled Native record traversal and nine expanded typed tests are mounted privately. Erased ArtifactPack opt-in remains absent pending an executed capability baseline.

Source semantic evidence is now nine passing laws, zero failures, all 35 tables populated and checked with an independent SQLite engine. It includes exact IEEE and full-width integer words, literal child identities, raw bytes, editing/reserialization and malformed ownership refusal. Strict public consuming compilation is still pending.

Root Native19392 ended with shared registry facade errors for Architect and undeclared canonical state crate errors for Plugin/Workflow while building Remodeling. Zero new Native owner assertions executed. Root82612 is the single replacement warm lane after canonical registry calls and declared first-party state/composition dependencies changed. Compiler success, selected test success and whole-package success will be reported separately.

## Current Source Public and Native Word Boundaries

The strict owning Source public consumer gate passed in 6.3 s uncached Nx. The new selected Native declared JSON word law is mounted: it uses the actual asynchronous declared serializer/importer and an independent serde JSON word-object oracle for every neutral binary32/binary64 word. Its production leaf is unchanged pending executed behavior. A later Source follow-up genuinely exposed nan32 Text refusal, stale mutation JSON numeric/octet declarations and native widened nan64 refusal; those repairs and full fixture alignment are in progress. These follow-up failures are distinct from the earlier nine passing SQL semantic laws.

Snapshot/mutation grammar mirrors now describe typed Inline/Content float blocks and octet leaves. The mutation DenseCloud grammar was hand-aligned as well. The ANTLR float productions include exact IEEE NaN word spellings and infinities. No separate ANTLR runtime verification is claimed.

## First Executed Expanded Native Owner — 2026-10-02 at 13:29 UTC

Nextest `c3149164-ff9c-4f55-b6f1-7975b67193f3` executed all 15 selected Remodeling laws: 10 passed and five failed in 1.033 seconds. The 1,327 ordinary owner tests were outside the selector, with no selected skips. Its direct complete relational reconstruction, independent SQL edits, exact scalar INTEGER/IEEE queries, literal byte/Child state, declared Text/Pack, exact row/schema admission and control laws passed.

Three genuine failures reached the deliberately absent capability at the real factory, erased boundary and actual declaration/I/O registration. The owning ArtifactPack now explicitly exposes its existing hand-authored typed relational codec after that measured baseline. The canonical Native JSON word test genuinely returned Number(0.0) rather than the required binary64 `{bits}` object; its repair is delegated to the Source/model owner. The old inline-cloud law also exposed serde integer-versus-float fixture spelling, and will be aligned to the new exact-word canonical contract. Fresh verification is pending. The whole owning package has not been tested by this selected run.

## Native Capability and JSON Repair Verified

Nextest `2f075b0d-913d-4bcd-b4db-4cb9d9c3723f` passed all 16 selected Remodeling laws in 1.553 seconds, with no selected skips. Its 1,327 ordinary tests were outside the selector. The actual factory, erased import/export, and real declaration-driven SQLite file I/O now pass. The canonical JSON boundary preserves all owned binary32/binary64 words and full-width integer paths, rejects malformed word objects and unsafe unsigned integers, and follows the existing hand-authored JSON transport alternatives. The old inline-cloud fixture now tests the declared canonical word boundary. Whole owner verification is active separately through the existing registered Native test target.
