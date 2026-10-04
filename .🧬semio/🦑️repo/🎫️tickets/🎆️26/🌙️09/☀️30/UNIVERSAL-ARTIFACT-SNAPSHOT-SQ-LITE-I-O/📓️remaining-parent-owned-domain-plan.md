# Remaining Parent Owned Domain Plan

Read-only declaration and codec audit, 2026-10-02. Repository, s, Process, Trinity and DAG AGENTS.md read. Existing ticket reused by parent association. No Cargo, Nx, tests or runtime commands run; no production, tests, script, launch or Git edits. This report is the sole authored file. Proposed tables below are a design derived from actual declarations, not an implemented or admitted schema.

## Verified Missing Authority

The seven concrete persisted owners below still have no explicit `ArtifactSqliteSnapshot` implementation in live Rust source. Their artifact roots were searched for SQLite/provider/DDL declarations, then their actual snapshot and ordinary native codec declarations were read. Repository-wide exact-owner implementation searches including hidden/generated source found no relocated implementation. Root-local Rust/SQL searches found no parent DDL; Trinity Jack has SQLite query-executor test tables, but those contain nodes/edges/query cursor state rather than the Jack snapshot fields and do not provide snapshot reconstruction. This distinction prevents a keyword count from being mistaken for coverage.

All snapshot locations below are relative to the corresponding artifact root, with the common suffix `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`.

| Actual owner | Artifact root | Persisted parent fields |
| --- | --- | --- |
| `Process3dSnapshot` / `s.process.process3d@1/*` | `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d` | workshop, stock_id, stock_label, stock_pose, stock_payload, stock_solid, steps, step_payloads, tool_solids |
| `JackSnapshot` / `s.trinity.jack@1/*` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack` | schema, name, manifest_id, manifest, camera, content, root_node_id, query |
| `DagSnapshot` / `s.dag.dag@1/*` | `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag` | schema, content |
| `PlaygroundSnapshot` / `s.demonstrator.playground@1/*` | `✏️s/🔌️plugins/🎪️demonstrator/🗿️artifacts/🎪️playground` | schema only |
| `RewritingSnapshot` / `s.trinity.rewriting@1/*` | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewriting` | before_fixture_json, lhs_json, rhs_json, parameter_bindings, rule_layout |
| `DrawingSnapshot` / `s.draw.drawing@1/*` | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing` | schema, id, title, layers, assets, artboard |
| `RasterSnapshot` / `s.raster.raster@1/*` | `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster` | schema, id, title, layers, assets |

Existing child providers cannot cover these parents. In particular the framework Infinite DAG persisted projection is a separate authority, and SemioDrawingSnapshot is distinct from DrawingSnapshot. Current absent-provider findings are source observations, not executed capability REDs.

## Physical Native Boundary

`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:10873` defines `ArtifactSqliteSnapshot`; its defaults reject controlled native decode, encode and preflight. `sqlite_codec()` export calls the exact owner's native decoder before projection; import reconstructs the owner then calls its native encoder. A handwritten relational provider alone does not establish ordinary erased I/O. Every owner needs projection, reconstruction, native hooks, preflight, semantic validation and declared retirement; typed and erased laws must be separate.

No parent SQLite opt-in is visible in the three priority owners' actual ArtifactPack declarations. Process derives `DslRecord` and uses its canonical record codecs in the snapshot declaration; its `snapshot/💾️binary/🦀️.rs` contains protocol constants only. Jack's public binary wrapper is under `🧬️schema/📸️snapshot/💾️binary/🦀️.rs`, but its real ArtifactPack implementation is in `🧬️schema/📸️snapshot/📝️text/🦀️.rs:172`. DAG's real implementation is under `🚪️io/📸️snapshot/💾️binary/🦀️.rs:31`. Do not rely on stale comments pointing to `🗣️dsl` or assume all owners use `🚪️io` paths.

Jack ordinary declaration is physically `🏅️standards/🔖️1/🪆️subsets/✳️any/🦀️.rs:55`: NativeCodecs uses `ArtifactCodec::bare::<JackSnapshot, TrinityGraphMutation>` and no foreign entries. DAG subset delegates to `🚪️io/🦀️.rs`, whose line 48 uses `ArtifactCodec::bare::<DagSnapshot, DagMutation>`. Neither is a SQLite opt-in. Process root builder line 1107 registers its ordinary composer entries; its binary protocol-only leaf must not be read as an explicit SQLite capability.

### Process3d Exact Domain

All inline definitions are in `✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🦀️.rs`:

- `Workshop` (270): ordered machines.
- `WorkshopMachine` (214): id, label, icon_id, optional catalog_id, ordered capabilities.
- `Capability` (197): id, label, icon_id, MeasureRecipe, ordered parameters, ordered rules.
- `CapabilityParameter` (131): id, label, f64 value.
- `CapabilityRule` (112): Min/Max; quantity: StockQuantity, parameter: String, margin: f64.
- `MeasureRecipe` (143): DiscCut(diameter, kerf), BladeCut(kerf, length, depth), PocketCut(diameter, depth), BoreDrill(radius, depth), CylinderAttach(radius, length), BoxAttach(width, depth, height). Every recipe argument is a String parameter reference, not a numeric dimension.
- `Pose` (390): position[3], axis[3], angle; all f64.
- `Stock` (484): id, label, WorkingSolid, Pose. It is actually persisted by `stock_payload`, despite older docstrings describing it as ephemeral.
- `WorkingSolid` (437): Box(width, depth, height), Cylinder(radius, height), Sphere(radius), ImportedMesh(mesh_url), ImportedSolid(solid_handle), Reference(reference_id). Preserve imported handle text exactly; do not resolve or replace it during persistence.
- `ProcessStep` (538): id, label, enabled: bool, optional StepOrigin(machine_id, capability_id), ProcessMeasure.
- `ProcessMeasure` (503): Cut(tool WorkingSolid, Pose), Drill(radius, depth, Pose), Attach(component WorkingSolid, Pose).

`stock_id`, `stock_label`, `stock_pose` and their `stock_payload` counterparts are independent actual persisted fields. Projection must not collapse them, assume equality, or rebuild one from the other. `step_payloads` and the steps flow handle are also independent persisted fields.

Child types and canonical paths:

- stock_solid and ordered tool_solids: `store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::brep::schema::snapshot::SemioBrepSnapshot>`, physically `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🧬️schema/📸️snapshot/🦀️.rs`.
- steps: `store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::flow::schema::snapshot::SemioFlowSnapshot>`; physically `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🧬️schema/📸️snapshot/🦀️.rs`.
- Each handle has child_id and target { artifact_id, dialect { artifact_kind, standard, subset } }. Store all five coordinates independently. The process genesis child hook at root lines 819–832 reconstructs child packs from stock/step payloads; it is child archive behavior, not parent SQLite authority.

`WorkshopMachinePatch`, `ProcessStepPatch` and `ProcessWorkingScene` are mutation/runtime vocabulary, not extra persisted snapshot fields. Keep patch tri-state origin out of snapshot schema: snapshot origin is only Option<StepOrigin>.

### Jack Exact Domain and Current Native Loss

`JackContentChild` is `store::ArtifactChild<semio_s_artifact_stdio_semio::standards::v1::subsets::graph::schema::snapshot::SemioGraphSnapshot>`, declared at Jack root line 191. Its exact local runtime owner is JackWorkingScene, nodes/edges (310–312). `materialize_jack_content` attaches the scene to an existing handle (316); `jack_content_child_with_owner` instead mints content-derived coordinates (344). Parent SQLite reconstruction must choose the former when attaching resolved content to literal imported coordinates. Canonical child physical owner is `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🧬️schema/📸️snapshot/🦀️.rs`. Parent SQL need not duplicate graph nodes/ports/edges merely to prove metadata persistence.

Inline persisted `Camera` (396) owns x,y,zoom f64. `Manifest` and kind definitions physically live in Jack `🛂️manifest/🦀️.rs`: ordered node_kinds, edge_kinds, port_kinds. NodeKindDef has name, ordered properties, ordered port_kinds Strings. EdgeKindDef has name and ordered properties. PortKindDef has name, PortDirection and ordered properties.

`PropertyDef` and `PortDirection` authority is `🧰️framework/🔨️modules/🕸️graph/🛂️manifest/🦀️.rs:198,257`: name, PropertyKind(Data/Derived), ValueType, optional expr; direction In/Out. `ValueType` is `🧰️framework/🔨️modules/🌱️value/🏷️type/🦀️.rs:7`: Boolean, Integer, Decimal, Text, List(Box<ValueType>), Schema(String), Any. Recursive lists and literal schema strings must remain explicit, not serialized into a JSON cell.

Important concrete mismatch: JackPackRecord (text codec lines 100–111) excludes persisted manifest and excludes child coordinates. Encoding reads the local scene into nodes/edges; decoding (130–134) mints a new content-derived child, defaults manifest, then resolves it from manifest_id. Therefore an independently customized inline manifest and a literal non-derived child/target identity are not represented by ordinary native pack/text today. This needs an executed Native baseline before schema implementation and an owner-native correction, not a provider-only workaround or inference from a registered manifest. JackSnapshot's hand ToValue preserves all declared fields, so it does not establish the separate pack law.

### DAG Exact Domain and Current Native Loss

`DagContentChild` is the same canonical SemioGraphSnapshot typed child, declared in DAG root line 41. Local runtime owner is DagWorkingScene(nodes,edges), lines 141–143. Parent persisted fields are only schema and handle. DagCamera, DagNodeSpec, DagHostSnapshotEdge, editor state and previews are not additional parent persisted fields.

Snapshot validate() checks schema equals `dag.dag` and exact Semio graph child dialect. The root factory creates target.artifact_id equal to a content-derived child_id; that factory is a genesis choice, not a rule allowing projection to infer target from child_id.

Native pack delegates to `semio_framework_artifact_infinite_dag::DagSnapshot`, whose schema/nodes/edges are a distinct framework owner. Conversion back calls `dag_content_child_with_owner` and re-mints identity. A wire-only handle reads as an empty working scene, and ordinary encode can therefore silently replace literal identity and unresolved content with a new empty scene identity. Test custom literal child and target IDs with a valid graph dialect before mounting; test attached and unresolved owners separately. Native serialization must represent the declared parent fields directly and child materialization through its own authority. No compatibility layer or duplicate Dag DTO should become a second source of truth.

## Individually Authored Relational Designs

All designs use literal domain fields and ordinal structural coordinates. Relational row indices are traversal positions, never synthesized domain identities. Require contiguous ordinals for each ordered Vec, strict singleton cardinality, known columns/types, foreign keys, no orphan variant rows, and rejection of duplicate/unknown rows before allocating reconstructed state. Domain IDs may repeat unless the owner's actual validation prohibits it; do not introduce guessed uniqueness or referential restrictions on unresolved parameter/machine strings. Each owner authors its own DDL in its own snapshot/🪶️sqlite directory.

f64/f32 values require the existing exact IEEE754 scalar component with width and full bits. Do not cast u64 bits into lossy REAL or truncate to signed SQLite INTEGER; exact bits are an 8-byte/4-byte scalar BLOB or the established exact component representation. Queryable numeric projections can accompany exact bits where supported, but the authoritative value stays exact. Preserve negative zero, subnormal, infinity and NaN payloads in typed semantic laws with bit equality; record native format limitations as separate failing laws.

### Process Tables

1. `process3d_document`: singleton row; stock_id, stock_label. Separate stock_pose row owns seven named IEEE754 fields.
2. `process3d_stock_payload`: singleton id,label; separate stock payload pose. Its WorkingSolid is an explicit tagged solid row with subtype tables for Box/Cylinder/Sphere/ImportedMesh/ImportedSolid/Reference. Cut/Attach solids reuse this same domain decomposition by structural owner location, without reusing root stock identity.
3. `process3d_machine`: machine ordinal,id,label,icon_id,nullable catalog_id. `process3d_capability`: machine ordinal,capability ordinal,id,label,icon_id,recipe tag. Recipe subtype rows contain only the declared String references listed above, with exact one-of validation.
4. `process3d_parameter`: machine/capability/parameter ordinals,id,label,value IEEE754 scalar. `process3d_rule`: machine/capability/rule ordinals,Min/Max tag,StockQuantity tag,parameter String,margin scalar. StockQuantity actual variants are Width,Depth,Height,MaxDimension,MinDimension (root line 56); the value codec emits width,depth,height,maxDimension,minDimension.
5. `process3d_step`: ordinal,id,label,enabled. Optional `process3d_step_origin` has machine_id,capability_id. `process3d_measure` has Cut/Drill/Attach tag; measure Pose is explicit; Drill dimensions are scalars; Cut/Attach have their exact solid variant rows.
6. `process3d_stock_solid_child` and `process3d_steps_child`: independently singleton, all five handle coordinates. `process3d_tool_solid_child`: ordinal plus all five coordinates. No child native pack BLOB, no generated tool geometry, no stock-vs-payload equality CHECK.

### Jack Tables

1. `jack_document`: singleton schema,name,nullable manifest_id,nullable root_node_id,query. Null optional versus present empty string must remain distinct.
2. `jack_camera`: singleton exact x,y,zoom scalars.
3. `jack_content_child`: singleton literal child_id,artifact_id,artifact_kind,standard,subset.
4. `jack_node_kind`, `jack_edge_kind`, `jack_port_kind`: ordered separate domains; name plus port direction where applicable. `jack_node_kind_port` preserves ordered port kind references including repeated strings.
5. Kind-specific property tables or one explicitly constrained owner-kind/property ordinal relation: name,Data/Derived tag,nullable expr. `jack_property_value_type`: property structural owner coordinates, depth/ordinal, exact Boolean/Integer/Decimal/Text/List/Schema/Any tag and schema String only for Schema. Every List has one child, leaves none; reject disconnected or cyclic rows and depth/byte-budget overflow.

The relational source reconstructs actual `Manifest` directly, independent of registry/manifest_id. Graph child provider remains separate. Native child content is neither a JSON cell nor a pack carrier in the parent database.

### DAG Tables

`dag_document`: singleton schema. `dag_content_child`: singleton literal five handle coordinates, relation to singleton document, and actual subset validation. No framework DAG nodes table, camera or editor fields in this parent schema. Test graph child SQL independently; admit its composition only with explicit closure/resolver policy, not an implicit empty local owner.

### Remaining Four Owners

Playground needs one singleton schema table and its actual Native hooks; no editor/example schema fields exist on its snapshot.

Rewriting needs three exact TEXT body columns (the native domain type is String despite json-named fields), parameter binding key rows with a structural tagged PropertyValue tree, and layout key rows with exact LayoutPoint scalars. PropertyValue declaration is in framework graph manifest: Null,Bool,Number(f64),String,Array(Vec),Object(BTreeMap). Preserve ordered arrays and exact sorted object keys; do not parse or normalize the three authored String bodies. LayoutPoint is actually x,y f64, declared at Rewriting root lines 98–100.

Drawing needs parent schema/id/nullable title, ordered layer forest, asset map key + mime/data literal Strings + nullable width/height u32, optional artboard with exact width/height f64. Its seven layer variants are Shape,Path,Text,Image,Group,Boolean,Trace; actual detailed bodies, style and PathSegment domains must all be enumerated before authoring complete SQL. PathSegment is Move/Line/Quad/Cubic/Arc/Close with coordinates, controls, arc flags and rotation; preserve the actual variants. SemioDrawing component cannot substitute this forest. Drawing assets' data is String, not a presumed decoded byte BLOB.

Raster needs schema/id/nullable title, ordered Pixel/Group/Adjustment layer forest, exact f32 opacity, f64 transform(x,y,a,b,c,d), Pixel/Group optional masks, mask enabled/linked/invert + nullable width/height/image_key + transform, Pixel nullable width/height/image_key, Adjustment kind and actual `RasterOwnedMap<DslValue>` structural parameters. Asset map values are RasterAssetChild handles to SemioImageSnapshot, with all five coordinates; image payload remains child authority. RasterPackRecord currently encodes layers as JSON text and child native packs as bytes; these are existing native mechanics, explicitly unsuitable as semantic SQL cells. Retirement of asset and nested adjustment maps is mandatory through `retire_raster_snapshot`; blanket Drop is invalid. Exact dynamic number variants and ordered DslValue Object entries require their own actual-domain census before complete Raster DDL.

## Neutral Fixture and TDD Baseline Plan

No tests were executed. Planned fixtures belong beside each actual snapshot under `🧫️fixtures/🪶️sqlite`, with a strict language-neutral laws schema, and independent handwritten SQL expectations. JSON fixture specifications are permissible test inputs; they must never become snapshot SQLite carriers or production DTO authorities. Read actual snapshot schema, child schema, I/O schema and reference registry into Source schema tests; validate with the already present Ajv test library. Cross-check real emitted SQLite files with independent `bun:sqlite` engine and exact queries, as existing Sequence tests do.

Existing references read:

- `✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql`: separates singleton document and literal child coordinates.
- Its sibling `🧪️tests/🪶️sqlite/🟦️.ts`: Ajv validates native/child/I/O/reference schemas; bun:sqlite queries exported files; tests malformed SQL/column shapes, caller limits, cancellation and reconstruction.
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧩️artifact/🧪️tests/🫳️reconstruction/🦀️.rs`: independent Bun SQLite UTF-8/BLOB byte oracle for reconstructed owned fields.
- Framework generic fixture/schema/export tests establish transport properties, not these seven parent semantics.

Priority fixture matrix:

1. Process intentionally differs stock metadata from stock_payload, includes all six solid variants, all six recipes, both rule variants, enabled/disabled steps and every measure variant; includes absent and present-empty catalog/origin strings, reordered lists, repeated legal domain IDs, large multibyte text and exact IEEE754 edge bits. Tool list empty/multiple, and literal child_id differing from target.artifact_id. Do not let child handles derived from default geometry mask missing fields.
2. Jack includes an inline Manifest unrelated to a registered manifest_id, nested ValueType Lists and Schema strings, optional IDs absent versus empty, multilingual query/name, signed-zero camera, repeated ordered port refs, and literal independent content coordinates. Attached graph scene and unresolved handle require separate Native baselines.
3. DAG includes schema plus literal independent graph handle coordinates; one unresolved owner, one attached scene, and distinct framework Infinite DAG baseline. A parent test must compare every coordinate after actual native→SQLite→native→owner, not only working node count or final content digest.
4. All: empty/missing singleton, duplicate singleton, missing handle, invalid dialect, extra tables/columns, invalid variant, orphan rows, non-contiguous ordinals, exact byte limits and cancellation initially and mid-traversal. Reconstruction failure must retire partial owners; no mutation/store mounts during baseline establishment.

TDD order: language-neutral fixture/schema RED → Source semantic functions + independent SQL engine assertions → actual owner native typed RED → controlled hooks GREEN → declared ordinary erased capability/mount RED → correct opt-in GREEN. Separate failures caused by shared compile blockers from capability assertion RED. Root alone executes Cargo lane; coordinate package suites with Root and preserve Generation/Layout/Drawing/ZIP workers' files. Per-kind source and native tasks must have distinct results.

## Existing Commands and Execution Prerequisites

| Package | Physically declared Nx targets | Existing script authority |
| --- | --- | --- |
| `@semio-tech/process-process3d-rs` | build,check,test,test-diff | package `📜️script.ts` calls runArtifactRustPackageMain with `semio-s-artifact-process-process3d` |
| `@semio-tech/trinity-jack-rs` | build,check,test,verify-jack-document-contract,verify-jack-query-ownership,graph-generate,graph-wire-check | package `📜️script.ts`; build/check/test depend on graph-generate |
| `@semio-tech/dag-dag-rs` | build,check,test,verify-dag-document-contract | package `📜️script.ts` |

Their package files are `📦️packages/🦀️rust/📋️project.json` under the roots above. They currently declare no dedicated parent `test-snapshot-sqlite-native` or Source target. Existing `.vscode/launch.json` entries include verify-dag-document-contract at line 1634, verify-jack-document-contract at 1887, verify-jack-query-ownership at 1898, and process test-diff at 23497; framework snapshot SQLite I/O gate is at 24905. Generic project-target picker IDs include test-snapshot-sqlite-native and verify-snapshot-sqlite-source, but that does not instantiate a missing package target. Generation/Block/Puzzle entries at launch top are currently owned by other agents.

Before execution: Root finishes shared Severity/diagnostic repair and releases Cargo lane; Jack graph generated manifest outputs must be current through its existing registered graph-generate dependency. Author neutral parent fixture/schema first; register narrowly filtered source and native commands by extending existing `📜️script.ts`, package project.json, and launch grouping/order. Do not add alternate scripts. Begin baseline using existing package test target/filter when appropriate, then dedicated target only when the real tests exist. Actual Jack/DAG native identity and Jack inline-manifest losses must be resolved before erased SQLite can honestly be admitted. All generated receipts remain ticket/🗑️generated and are deleted when the ticket closes; this report remains.
