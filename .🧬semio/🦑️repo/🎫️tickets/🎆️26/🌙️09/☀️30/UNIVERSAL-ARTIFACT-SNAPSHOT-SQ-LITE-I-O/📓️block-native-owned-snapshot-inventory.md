# Block Native Owned Snapshot Inventory And Relational Design

This is a read-only source inventory and handwritten semantic design. No Block source, provider, declaration, package router, launch entry or Source authority was modified. No Block Native assertion executed during this inventory. Applicable Block AGENTS says each document edits exactly one kind definition; assemblies belong to Puzzle. Source5d's current fifteen-table handwritten DDL is the existing schema authority and already matches all actual Native persisted fields.

## Actual persisted roots

All three concrete Snapshots derive DslRecord, ToValue, FromValue and ArtifactSchema and hand-implement ArtifactDsl/ArtifactPack. Their authored Native envelope IDs are block.block2d, block.block3d, block.block5d. Their cameras are explicitly #[state(artifact)]; they are persisted, not host materialization. Window view/transient/config/presence records elsewhere are distinct owners and must not be invented in these Snapshot tables. No separate BlockNdPlaySnapshot was found: the app names include Play, but their document codecs retain the actual BlockNdSnapshot.

|Owner|Exact root fields|Ownership source|
|---|---|---|
|Block2dSnapshot|schema; node_kind; presentation; handle_kinds; handles; compatibility; attributes; authors; camera2d; meta|◻️2d/standards1/any/schema/snapshot/🦀️.rs|
|Block3dSnapshot|schema; object_kind; representations; catalog; vortex_kind_extra; vortices; compatibility; attributes; authors; camera3d; meta|🧊️3d/standards1/any/schema/snapshot/🦀️.rs|
|Block5dSnapshot|schema; part_kind; part_2d; part_3d; representations; grip_kinds; grips; compatibility; attributes; authors; camera2d; camera3d; meta|🖐️5d/standards1/any/schema/snapshot/🦀️.rs|

The absolute artifact directory for the relative paths above is `/Users/ueli/Documents/semio/✏️s/🔌️plugins/🧱️block/🗿️artifacts`. All actual standard roots spell `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot`.

## Concrete component fields

The real shared component authority is Block2d `🧬️schema/🧱️shared/🦀️.rs`. Block3d and Block5d explicitly reexport these types from the already-declared Block2d crate; there is no need for a new shared DTO.

|Concrete type|Every persisted field|
|---|---|
|BlockKindIdentity|id/name/label/description String; variant/icon/unit Option<String>|
|BlockAttribute|key/value String; definition Option<String>|
|BlockAuthor|id/name String; email Option<String>|
|BlockCompatibilityRule|id/source/target String; bidirectional bool|
|BlockRepresentation|id/name/description String; mesh_url/lod Option<String>; ordered tags Vec<String>; ordered attributes Vec<BlockAttribute>|
|BlockCamera2d|x/y/zoom f64|
|BlockCamera3d|position [f64;3]; target [f64;3]; zoom f64|
|BlockMeta|description String|
|Block2dPresentation|shape/color/icon_kind Option<String>; radius/width/height Option<f64>|
|Block2dHandleKind|id/name/label/color/default_wire_kind String|
|Block2dHandleTemplate|id/handle_kind String; angle/radius f64|
|Block3dVortexKindExtra|id/name/label/color/default_cable_kind String|
|Block3dVortexTemplate|id/vortex_kind String; position/direction [f64;3]; radius f64; label Option<String>|
|Block5dPart2d|shape/color/icon_kind Option<String>; radius/width/height Option<f64>|
|Block5dPart3d|orientation Option<[f64;4]>; scale Option<[f64;3]>|
|Block5dGripKind|id/name/label/color/default_rope_kind String|
|Block5dGripTemplate|id/grip_kind String; angle/radius_2d/radius_3d f64; position/direction [f64;3]|

Actual types have no finite-domain constructor for these plain f64 fields. Preserve signed zero, both infinities, signaling and quiet NaN payloads, subnormals and ordinary words. Name each query REAL field explicitly with signed INTEGER binary64 bits and TEXT numeric class companions. Optional scalar absence requires all three NULL; optional vector/quaternion absence requires every component triple NULL, with partial presence rejected. Preserve native array order directly; do not apply geometric reinterpretation or normalization. Native2d has eight authored scalar positions, Native3d fourteen, Native5d twenty-nine, before repeated entity rows.

Block3d `catalog` is a required `ArtifactChild<SemioKitSnapshot>` with the declared #[child(kind="s.stdio.semio")] edge. Persist the five independent literal fields: child_id, target.artifact_id, artifact_kind, standard, subset. A local alias is independent of target identity; preserve empty/reserved/NUL/Unicode literals and unresolved target identity. The snapshot stores no inline Kit payload. `catalog_child_handle` computes a default logical content ID, while `vortex_kinds_of_parts` reconstructs its working display from vortex_kind_extra; neither permits replacing the actual stored child handle or dropping any extra row. Do not serialize host resolution/materialization into this snapshot, and do not enforce a join against fetched Kit types. The declared kind edge may be validated at the owned subset boundary; the source attribute declares kind, not an invented fixed standard/subset constraint.

## Handwritten relational mapping

Each table owns a positive SQL alias INTEGER PRIMARY KEY independent of all native identifiers. Only real ownership uses FOREIGN KEY. Every repeated collection uses `(owner_id, ordinal)` with contiguous zero-based integer ordinals. Native IDs, semantic kind references, source/target pairs and strings remain duplicate/empty/unresolved literals; do not impose UNIQUE or semantic-ID foreign keys that the owned state never declared. Required singletons must reconstruct exactly once. Optional scalar/vector/child state is not replaced by defaults.

Block2d proposed ten tables, adjacent to its actual Snapshot owner:

|Table|Hand-authored cells after alias|Width|
|---|---|---|
|block2_document|schema|2|
|block2_kind|document_id; native_id; name; label; variant; description; icon; unit|9|
|block2_presentation|document_id; shape; radius triple; width triple; height triple; color; icon_kind|14|
|block2_handle_kind|document_id; ordinal; native_id; name; label; color; default_wire_kind|8|
|block2_handle|document_id; ordinal; native_id; handle_kind; angle triple; radius triple|11|
|block2_compatibility|document_id; ordinal; native_id; source; target; bidirectional|7|
|block2_attribute|document_id; ordinal; key; value; definition|6|
|block2_author|document_id; ordinal; native_id; name; email|6|
|block2_camera2d|document_id; x triple; y triple; zoom triple|11|
|block2_meta|document_id; description|3|

Document/kind/presentation/camera/meta are required singletons. HandleKind/template collections remain independent; no resolution or implicit filtering.

Block3d proposed thirteen tables:

|Table|Hand-authored cells after alias|Width|
|---|---|---|
|block3_document|schema|2|
|block3_kind|document_id; native_id; name; label; variant; description; icon; unit|9|
|block3_catalog_child|document_id; child_id; target_artifact_id; artifact_kind; standard; subset|7|
|block3_representation|document_id; ordinal; native_id; name; mesh_url; lod; description|8|
|block3_representation_tag|representation_id; ordinal; value|4|
|block3_representation_attribute|representation_id; ordinal; key; value; definition|6|
|block3_vortex_kind_extra|document_id; ordinal; native_id; name; label; color; default_cable_kind|8|
|block3_vortex|document_id; ordinal; native_id; vortex_kind; position_xyz triples; direction_xyz triples; radius triple; label|27|
|block3_compatibility|document_id; ordinal; native_id; source; target; bidirectional|7|
|block3_attribute|document_id; ordinal; key; value; definition|6|
|block3_author|document_id; ordinal; native_id; name; email|6|
|block3_camera3d|document_id; position_xyz triples; target_xyz triples; zoom triple|23|
|block3_meta|document_id; description|3|

Document/kind/catalog child/camera/meta are required singletons. All representations, tags and nested attributes retain authored order even when IDs repeat. No inline Kit catalog table.

Block5d must consume its existing fifteen handwritten tables verbatim rather than infer/regenerate SQL. I executed this actual DDL with independent Bun SQLite and queried every PRAGMA table_info; measured widths are:

|Table|Width|Actual Native fields|
|---|---|---|
|block5_document|2|schema|
|block5_kind|9|part_kind|
|block5_part2d|14|all six part_2d fields|
|block5_part3d|23|optional orientation four triples and scale three triples|
|block5_representation|8|representation named text/options|
|block5_representation_tag|4|ordered tags|
|block5_representation_attribute|6|nested ordered attributes|
|block5_grip_kind|8|all five grip-kind strings|
|block5_grip|32|both string identities plus nine binary64 positions|
|block5_compatibility|7|all compatibility fields|
|block5_attribute|6|root ordered attributes|
|block5_author|6|root ordered authors|
|block5_camera2d|11|three authored binary64 fields|
|block5_camera3d|23|seven authored binary64 fields|
|block5_meta|3|meta.description|

## Actual capability and executable gaps

Read-only `rg` found no Native ArtifactSqliteSnapshot implementation, Pack sqlite_snapshot_codec hook, controlled SQLite Native hook or mounted sqlite_snapshot_ Native law in these three roots. Thus token inspection suggests absent capability, but the mandatory genuine bare/actual-declaration assertion baseline has not executed and must establish the runtime RED before mounting providers. The Source5d router already registers snapshotSqliteTests with its owned SQLite/foreign-I/O Source suites; Native routers for2d/3d currently only call runArtifactRustPackageMain. All three need explicit selected Native route registration and launch commands if not already added by concurrent Source ownership.

Each `🚪️io/🦀️.rs` owns a real `io()` declaration whose Native codec uses `ArtifactCodec::bare::<BlockNdSnapshot,BlockNdMutation>`, not a JSON document codec. This is the exact registration route to inspect/use in tests. The existing ordinary Native envelope/Text/Pack code already prints typed DslRecord, so derived controlled metadata/binding/output can forward through the genuine stable store helpers once feature baselines execute. Do not wrap ordinary printing/parsing with before/after-only callbacks. The snapshot roots retain no special Drop/host cursor; pure owned retirement is appropriate. Derived default functions and required catalog handle admission still need actual control assertions.

Minimal meaningful baseline: one bare capability assertion and one actual declared Native codec assertion per owner; complete retained-field Text/Pack fidelity with every binary64 word, all optional None/Some/empty states and literal IDs; Block3d five-field child identity including independent local/target IDs; genuinely controlled schema/input/output construction. After RED, add independent SQLite count/query/edit/renumber oracles, malformed ownership/cardinality/ordinal/IEEE laws, exact borrowed/parsed/owned row ceilings, schema/value limits and all four actual long-copy cancellation phases. Native full-payload I/O and Source/public success are distinct evidence.

## Mounted Native baseline and independent oracle stage

Following the read-only inventory, twenty-two meaningful Native laws are mounted in the actual Snapshot test facets: Block2d seven, Block3d eight, Block5d seven. They exercise bare capability, each actual `io().native_codec` declaration, every persisted field through ordinary Text/Pack with all eight neutral binary64 words, explicit optional absence versus present empty values, genuinely controlled record schema/output/input, and the actual declared JSON leaf. Block3d additionally retains every independent literal Child address field, including distinct local alias and target identity. There are no mounted production SQLite providers or newly supplied capability hooks yet. No Block Native assertion has executed: the current shared Replication↔DSL Cargo cycle is a prerequisite, and an unchanged Cargo retry would establish no feature result.

The adjacent neutral JSON fixtures for Block2d/3d are authored inputs, and the existing Block5d fixture received only the agreed named `nativeCase`; its Source word, malformed SQL, UTF-16 and count corpus remain untouched. Handwritten ten-table Block2d and thirteen-table Block3d SQL now exist next to their actual Snapshot owners; existing Source5d's fifteen-table SQL is unchanged.

I actually executed the exact embedded Bun SQLite oracle from each mounted Rust test, independently of Cargo. Each created its real handwritten DDL, queried every table's PRAGMA width (10/13/15 tables), and roundtripped all eight neutral words through signed INTEGER storage and unsigned reconstruction. All three passed. An initial probe used Bun Statement.safeIntegers as if it returned the statement; its real return is void. The owned test now uses Database safeIntegers mode and plain all(), and the corrected exact scripts were executed again successfully. This validates the independent oracle and SQL, not the unexecuted Rust provider or declaration.

Actual package scripts and project targets now register Native-only baseline commands for Block2d/3d; Block5d uses its existing owned snapshot command with explicit no-app Native test features. All existing Block5d Source suites and verification trees are preserved. Both launch catalogs contain the three commands at 408.694–408.696. New unmounted Rust mapping drafts are the next implementation stage; production mounting awaits a genuine capability/fidelity assertion RED.

Files authored at this stage: each Snapshot's `🧪️tests/🪶️sqlite/🦀️.rs`, its cfg(test) module mount, Block2d/3d `🧫️fixtures/🪶️sqlite/🔣️.json` and `🪶️sqlite/🗄️.sql`, Block5d fixture nativeCase, all three package `📜️script.ts` and `📋️project.json`, plus the current and seed VS Code launch catalogs. All paths are beneath the actual owners above; no Source model/provider/facade was altered.

## Unmounted literal Rust provider readiness

All three adjacent `📸️snapshot/🪶️sqlite/🦀️.rs` files now contain explicit handwritten drafts. They use the ten/thirteen/fifteen authored table contracts, projecting each concrete native field directly through borrowed Cell values and reconstructing concrete owned types through controlled text/scalar copies. Native identifiers and unresolved semantic references remain unrestricted literals. Block3d stores the five catalog address fields independently; its only child semantic guard is the actually declared Semio artifact kind. Arrays preserve their stored component order, and Block5d optional quaternion/vector states require complete presence or complete absence. There is no JSON/Value/native-byte carrier, inferred SQL, reflection or provider-generating mapping.

The drafts preflight exact owned relational row counts and schema bytes, admit parsed native entity counts before derived typed construction, and forward genuine generated schema/binding/output methods through the stable store native helpers. Reconstruction validates authored SQL, each alias/type width, positive alias identity, ownership FK, singleton cardinality, dense zero-based ordinals, exact numeric class/query/word consistency, boolean domain and complete consumption of every entity. Shared Projection and Reconstruction supply chunked field-copy cancellation; every table scan/group traversal also checkpoints. Pure concrete owners retain ordinary owned retirement; no retained host materialization is manufactured.

All three draft files passed Rust parser validation through `rustfmt --edition 2021 --emit stdout` with output discarded. This is syntax evidence only. Their module paths are deliberately absent from production Snapshot mounts, and Pack capability hooks remain absent: genuine Native baseline assertion failure is required before mounting. No Cargo/Nextest compilation or provider runtime occurred in this stage. Block5d's existing Source SQL/provider and all Source suites remain unchanged.

Registered next baseline commands are `bun nx run @semio-tech/block-2d-rs:test-snapshot-sqlite-native --skip-nx-cache`, the analogous block-3d-rs command, and block-5d-rs command. Use the existing quick test level and warm paired Cargo directories. Root owns the next actual Native lane; the worker did not introduce another Cargo process during the cycle/readiness interval. After genuine RED, mount each actual provider and Pack hook, then run independent full-table edited-file reconstruction, mixed IEEE component words, row/schema/value bounds and all four actual copy-phase cancellation laws. Existing twenty-two baselines prove no production capability yet.

Puzzle3d's separate twenty-three-table Rust draft and full-entity test draft remain unmounted. Its six actual Native baselines are mounted; the retained attempts stopped before assertions on shared dependencies, most recently canonical DSL extraction. FEM Source/public evidence remains forty-five laws per combined pair (2d15, 3d30) with unchanged five-second per-law policy. Neither P3 nor Block is marked end-to-end complete.

Package router/project paths in the readiness receipt are precisely each artifact's `📦️packages/🦀️rust/{📜️script.ts,📋️project.json}`, not the artifact root. Physical rereads confirm all three Native targets and both launch entries at 408.694–408.696. Block5d's additional unmounted Rust draft also passed syntax parsing; its existing SQL was only read. The final worker stage introduced no Native Cargo run and preserves both warm caches and all earlier evidence.

## Block2d/3d Authentic Missing Capability and Empty-Schema Repair

The main eighteen-owner Native run genuinely selected Block2d seven laws (four pass/three fail, 40 ms, 223 ordinary outside selector) and Block3d eight laws (five pass/three fail, 96 ms, 301 ordinary outside selector). In each owner, two failures prove absent bare/actual declaration SQLite capability; the third shows actual JSON decoding overwrites a present empty schema string. These results authorize mounting the pre-authored ten-table/thirteen-table typed providers, owner capability hooks and genuine controlled Native input/output bridges. Removed only the JSON empty-schema overwrite, retaining every present schema string exactly; no absent-field compatibility or numeric union is added. Providers preserve individually queryable REAL/null+signed IEEE words+class, all options/relationships/literal semantic references and exact ordered entities. Rustfmt parsed both providers. No worker Cargo invocation or post-mount Native passing claim is made; broadened independent SQLite and control laws are being mounted for the coordinator’s next owning verification.

The mounted Block2d and Block3d selected suites are now eleven and twelve laws respectively. Added checks retain all original fixture assertions and independently inspect each table, hostile semantic SQL edits, exact domain row limits and real four-phase long-text cancellation. The new test harness incorrectly unwrapped ordinary `encode_pack()` (whose actual API returns Vec); both measured `.unwrap()` errors are corrected without changing production. Fresh post-mount execution remains pending.
