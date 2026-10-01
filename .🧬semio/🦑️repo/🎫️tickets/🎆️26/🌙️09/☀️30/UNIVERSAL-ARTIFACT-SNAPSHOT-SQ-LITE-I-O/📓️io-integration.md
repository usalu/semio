# Native Snapshot I/O Integration

## Updated Goal: Handcrafted Semantic SQLite Schemas

The coordinator relayed a new user requirement after initial integration implementation: every artifact snapshot must expose its entities and relationships through an individually handcrafted SQLite schema, interpretable without a native codec. A generic binary/text payload BLOB, hidden JSON, or universal JSON-to-SQLite flattening does not satisfy the updated goal. The earlier container implementation below is an initial implementation record, not a completion claim for the revised objective.

Reusable work comprises exact dialect registration, shared atomic store/I/O publication, 58 explicit headless coordinates, 34 explicit imperative bootstrap coordinates, universal endpoint-only routing, control callbacks, and an 88-editor shipped catalog test roster. The roster test now verifies SQLite file output through I/O and compares independently decoded semantic snapshots; it does not assert the old BLOB-table schema.

Recommended provider contract: each concrete snapshot declares a SQLite schema and typed relational exporter/importer. A shared physical SQLite layer supplies database pages, typed SQL/table records, limits, progress and cancellation. Each provider owns its actual domain tables, keys, entity fields, relationships, enum cases and ordered collections. The declaration tree and older codec specification publish the provider with the exact dialect, and route execution dispatches that provider. Missing provider declarations fail preflight. Guest plugins execute the same provider through their existing I/O export; dynamically hosted artifacts need an exact dialect provider bridge to their authorized component. Native byte-preserving wrappers cannot replace these semantic providers.

## Initial Container Integration Record

Framework I/O publishes native snapshot registrations keyed by the exact artifact/standard/subset dialect. Each registration retains its typed snapshot validator from `ArtifactCodec::of`; no native snapshot encoding is translated through JSON or document history. Binary and text payloads are preserved verbatim, with native decode validation before export and after import.

Universal descriptors expose direct Exact transfers between each native dialect and `s.framework.sqlite-snapshot@1/*`. SQLite is a reserved framework endpoint: plugins cannot register competing converters for it, routing never traverses it as an intermediate, and manually authored intermediate/disconnected/inconsistent routes are rejected. `io_identify` validates the database and identifies its SQLite snapshot dialect. Callers inspect container metadata to select the native target, so ordinary file converters retain their existing routes.

No binary-carrier ingress descriptor is added: `CARRIER_BINARY` is the same coordinate as the actual headless binary artifact, and such a descriptor would conflate raw file identification with `BinarySnapshot` export.

`io_run_with_snapshot_control` supplies caller resource limits and cancellable page progress. Its first checkpoint happens before materializing a typed snapshot; native payload size and coordinate ceilings are checked before decode. Ordinary `io_run` uses default limits and continues automatically.

Both registration systems publish exact domain identity. New declaration trees use every subset dialect; older app codecs use `ArtifactApp::DIALECT`, and 58 headless codec declarations now supply explicit dialect arguments. Native codec registration is part of the older atomic store/I/O assembly plan, and the new declaration tree now commits codec/format/native snapshots through that same atomic plan. A separate inventory records every changed headless coordinate. Foreign-only app codec declarations also publish `ArtifactApp::DIALECT`. Multiple schema names for the same typed snapshot deduplicate by validator, extension and pack schema hash; a different native owner is rejected. The direct `register_document_codec_for_app` API publishes codec and exact dialect together atomically.

Host guest-backed mirror codecs have no linked snapshot decoder, so their optional validation thunk is absent. Native snapshot registration rejects absent validators. Typed guest plugin assemblies still construct `ArtifactCodec::of` with an actual validator and receive universal SQLite routes.

## Tests Added

The language-neutral registration corpus is at framework I/O `🧫️fixtures/🪶️sqlite-snapshot-registration/🔣️.json`. The plugin fixture test `sqlite_snapshot_covers_every_declared_subset` checks all three declaration dialects, both Binary/Text encodings, discoverable Exact import/export, database identity, mismatched targets, invalid native packs, cancellation, zero hop ceilings, intermediate SQLite routes, disconnected routes, unavailable validators, and conflicting native owners. The fixture uses independent serde_json native encoders/decoders; SQLite file interoperability is covered by the shared codec oracle. The second test `sqlite_snapshot_covers_headless_atomic_assembly` checks an app-less assembly roundtrip and proves an unavailable-validator failure leaves both store codec and native route registries unchanged.

## Verification Status

The first registered Bun/Nx integration run exposed an incorrect root-module name in the new typed store validator (`crate::os_io_schema`); kernel mounts the vocabulary as `crate::io_schema`. Corrected all four new references after inspecting both kernel mounting and framework reexport. Integration rerun is pending; no integration test pass is claimed yet.

## Owned Files

- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📓️io-integration.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/UNIVERSAL-ARTIFACT-SNAPSHOT-SQ-LITE-I-O/📓️snapshot-registration-inventory.md`
- `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/☁️las/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🏅️standards/🔖️5/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌐️html/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🏅️standards/🔖️energyplus/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🌦️epw/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/7️⃣87a/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🏅️standards/9️⃣89a/🪆️subsets/🧱️base/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎞️gif/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎥️mp4/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🏅️standards/🔖️mpeg1-layer3/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎵️mp3/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/4️⃣4/🪆️subsets/✳️any/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💬️bcf/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📊️csv/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🏅️standards/🔖️iana/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📑️tsv/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📕️xlsx/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📝️md/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📷️png/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📸️jpg/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🏅️standards/🔖️1.0/🪆️subsets/🎛️hdrl/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📼️avi/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📽️pptx/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🏅️standards/🔖️riff-pcm/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔊️wav/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🏅️standards/🔖️utf-8/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔤️txt/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🔺️stl/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖊️dwg/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖋️dxf/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🖼️tiff/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗜️deflate/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🗽️obj/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧱️ply/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧾️json/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🌊️flow/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎞️animation/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🎬️video/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🏛️model/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📊️table/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📐️cad/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📑️document/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📦️object/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/📽️presentation/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔊️audio/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔢️value/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔤️text/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🔺️mesh/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🕸️graph/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖼️image/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧊️brep/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🧰️kit/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🏅️standards/🔖️v3/🪆️subsets/✳️any/🚪️io/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🪟️bmp/🦀️.rs`
- `✏️s/🔌️plugins/🗄️stdio/🧪️tests/✏️editor-catalog/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🧪️tests/🔬️unit/🦀️.rs`
- `🌎️hub/🗿️artifact-authority/🧪️tests/🔬️unit/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🧫️fixtures/🪶️sqlite-snapshot-registration/🔣️.json`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🟦️.ts`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🏠️workspace/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-declarations-fixture/🦀️.rs`

## Mandatory Semantic Provider Contract

The updated user scope supersedes the initial byte-container design. The I/O route now executes each registered snapshot's explicit `ArtifactSqliteSnapshot` implementation. Its handwritten SQL lives with that snapshot; native decode, relational projection, relational reconstruction and native encode are distinct stages. No universal native snapshot BLOB is stored. An absent provider is rejected rather than completed by a generic fallback.

The required trait exposes `SQLITE_SCHEMA`, `to_sqlite_database`, and `from_sqlite_database`, each conversion taking a shared `SqliteSnapshotControl`. The erased codec carries its SQL declaration and receives exact artifact dialect plus native schema, allowing native and guest ownership to remain explicit. The shared SQLite module is mounted once in the kernel and reexported by framework/I/O so every provider uses the same nominal database types. Only persistent artifact Snapshot bounds require the trait; Draft, Presence and Transient do not.

The single reserved `semio_snapshot` table contains id, artifact_kind, standard, subset, schema_version and native_encoding. Typed domain schemas cannot claim it. Import requires precisely the provider's table names and declared DDL before typed reconstruction. SQL comparison preserves text inside literals. Metadata rejects invalid coordinates, wrong version/encoding, missing singleton identity, and wrong target dialect. Native snapshot pairs remain endpoint-only Exact routes; SQLite cannot become a graph intermediate between unrelated artifacts.

The framework format catalog owns a built-in `.sqlite`/`application/vnd.sqlite3` descriptor under `s.framework.sqlite-snapshot`, accessible before plugin activation. Existing native formats retain their ordinary routes.

Cancellation is checked before native decode and after it, during provider projection/reconstruction, and before/after native encoding. Physical page processing shares the same callback. Explicit fixture providers checkpoint scalar stages; child relation loops checkpoint each 256 rows. Native decoders/encoders themselves remain synchronous operations with before/after cancellation boundaries; expensive native codec streaming remains a practical limitation.

### Additional Owned Sources

- `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs` shared engine mount
- `🧰️framework/📦️packages/🦀️rust/🦀️.rs` shared engine reexport
- `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧬️schema/🧷️metadata.sql`
- `🧰️framework/🔨️modules/🚪️io/🧫️fixtures/🪶️sqlite-snapshot-registration/🗄️.sql`
- Plugin fixtures `🖥️test-app-mutations-document`, `🧬️mutation-fixtures-dummy`, `🧬️mutation-fixtures-transaction`, `🧬️mutation-fixtures-surface`: their existing `🦀️.rs` plus individual `🗄️.sql`
- Plugin composition fixture `🧩️composition/🦀️.rs`, `🪴️parent.sql`, `🌿️branch.sql`
- Store unit fixture `🏪️store/🧪️tests/🔬️unit/🦀️.rs` and `🗄️.sql`
- Store sync unit fixture `🏪️store/🔄️sync/🧪️tests/🔬️unit/🦀️.rs` and `🗄️.sql`
- Native renderer diagnostic `📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs`, `🪶️sqlite/🗄️.sql`
- MCP workspace diagnostic `🌉️mcp/🏠️workspace/🦀️.rs`, `🪶️sqlite/🦀️.rs`, `🪶️sqlite/🗄️.sql`

MCP ProbeSnapshot's intrinsic domain is arbitrary JSON syntax: its explicit `probe_node` table models syntax types and scalar columns with parent/member/array-position relationships, rather than a serialized JSON field. This specialized diagnostic provider is not a generic provider for other artifacts.

### Verification Status and Remaining Work

The parent owns execution of registered Nx gates. No green semantic integration result has yet been received. Required providers intentionally make unimplemented shipped snapshot types fail compilation; the catalog test roster remains 88 typed shipping editors, and will test binary/text semantic roundtrips once those individual providers exist. Dynamic guest host codecs currently carry no relational provider and are rejected by native snapshot registration. Their declared per-artifact semantic execution bridge still requires implementation. Completing the universal route does not complete the 149 individual semantic schemas.

### Guest Semantic Execution Seam

Read-only follow-up found existing `semio.io-run` cold jobs and `IoPluginProxy::io_run` in plugin host, plus component codec WIT exports. The guest already owns the same universal endpoint and individually declared snapshot providers once native registration activates. A semantic host bridge can reuse guest native↔SQLite execution while selecting the exact verified component and `lease.parent_dialect`; it must not try arbitrary schema candidates. Schema discovery should return that exact provider's handwritten SQL declaration explicitly, e.g. a declared `codec.sqlite-schema` export or well-known cold schema query. Do not infer SQL from a native pack, JSON field shape, or a synthesized zero snapshot.

The host's erased provider can convert the guest's physical SQLite result through the shared physical database engine, remove validated reserved metadata, and verify the explicit DDL. Import applies the reverse physical step before invoking the exact guest route. This retains one schema authored beside each snapshot and no opaque native payload. Guest execution must forward operation budgets and cancellation/progress to the cold job. Existing `register_guest_document_codec` currently registers by schema only and drops the full lease dialect; it must receive the exact dialect and publish native registry coverage atomically after successful schema resolution.

## Exact Guest Semantic Provider Bridge (2026-10-01)

The codec WIT now declares schema lookup and semantic SQLite export/import with explicit exact dialect, native encoding, and seven resource limits. Guest results retain success warnings and rejection diagnostics through the framework diagnostic pack grammar outside the SQLite file. Both the owned interpreter exports and Wasmtime host implement these operations. Guest execution uses existing budget/fuel controls and cancellation observation; host progress reports a heartbeat without inventing row counts.

The MCP host binds semantic routes to the hub lease’s exact parent dialect, schema, and verified compiled component hash. Registration queries the guest’s authored SQL and atomically publishes store/native registry entries before making its route visible. Export strips validated metadata from the guest-produced SQLite database before the outer framework adds its own metadata. Import attaches exact metadata to the domain database consumed by the guest provider. Both directions preserve diagnostics and reject encoding/identity disagreements. Specialized subset validation belongs explicitly to the guest component’s exact registered route.

Additional owned paths: plugin/schema/📜️.wit; plugin/schema/🪶️sqlite/🦀️.rs; plugin/🧠️interpreter/🦀️.rs; plugin/🖥️host/🦀️.rs; MCP/workspace/🦀️.rs. The existing physical TS ownership seam now asserts an owned database import thunk, avoiding a guest domain database clone.

Verification remains pending: registered framework I/O tests and plugin host checks launched. No guest execution or whole catalog completion is claimed yet. All individually authored snapshot providers and missing specialized validators remain required.

## Owned Typed Snapshot I/O Decision

Some intrinsic format codecs normalize schema strings or omit snapshot properties that do not exist in that file format. Full typed SQLite I/O therefore treats the owned snapshot and its authored relational provider as authority. `io_export_sqlite_snapshot<P>` and `io_import_sqlite_snapshot<P>` select the exact registered dialect, verify the native provider's `TypeId` and SQL, preserve metadata and callback resource controls, and return owned typed snapshots directly. They never decode/encode native wire for the wildcard profile. Specialized subsets use the existing registered native validator as a separate conformance fence; its encoded validation payload is never the reconstructed snapshot authority. Remote native providers carry no host-native type identity and cannot pretend to implement a host `P`.

This supplements native payload routes. Native payload export/import retains the existing exact dialect and subset enforcement. Intrinsic native format losses must be audited separately; a semantic typed roundtrip must not be weakened to match codec normalization. The core law records absence of DecodeNative/EncodeNative events on a wildcard typed roundtrip and rejects a different snapshot type even with identical SQL. Actual CSV schema-string and BMP RGBA regressions are being coordinated with their provider owners.

## Verification Update

Registered physical gate passed 25 TypeScript unit tests (155 assertions), 14 Rust unit tests, and 3 interoperability tests (232 assertions). Registered plugin-host check passed with the new WIT, owned interpreter exports, and Wasmtime implementations. Registered MCP check passed with exact guest route registration and conversion callbacks. Registered I/O gate passed three tests before the owned typed API addition. Those results prove the reusable integration seams, not all149 individually authored provider implementations.

## Note Extension

`s.note.note` is an actual plugin artifact beyond the initial stdio roster, not a framework-only fixture. Its authored model contains six recursive block variants, composed text child records with durable paragraphs/runs, table rows/cells including empty and ragged rows, intrinsic image assets, a link target, and all link pin variants. The SQLite facet contains fourteen declared entity tables with containment, order and domain foreign keys. Snapshot blob sizes preserve all u64 values as explicit high/low unsigned32 words. The shared owned TypeScript BlobRef.size domain was changed to bigint with no number compatibility union. Note native/provider checks and component roundtrips remain pending; its component also depends on DXF/DWG/PDF/PNG/semio/SVG.

## Typed Fidelity and Domain Provider Continuation (October 1)

The owned typed I/O APIs were verified by the registered framework `test-snapshot-sqlite-io` target: four tests passed (937 filtered). Export/import retain the actual typed snapshot and do not pass it through a potentially normalizing native file codec. The root executor separately verified the BMP32/alpha/noncanonical-schema field-loss regression using these APIs. Provider-owned subset validation now also skips native encoding before the validator fence; exact type ownership and schema checks remain mandatory.

The shared owned TypeScript BlobRef schema now requires bigint for the full unsigned64 size domain, with no number compatibility union. Its actual consumers and fixture boundaries still require the later full mirror validation gate. The Note relational schema explicitly stores two unsigned32 size words and reconstructs the complete Rust u64 domain.

The explicit row projection helper is at `io/🪶️sqlite-snapshot/🧩️artifact/🦀️.rs`: borrowed Cell arrays, before-copy row/value budgets, cancellation, and optional early resource prediction. It does not walk or reflect over any artifact type. The native SQLite worker mounted it; its fresh helper regression has not yet been rerun in our lane.

Native DXF has twelve individually authored tables and reconstruction for all actual snapshot fields: typed headers, layers, styles, line types, raw table tags, block containment, all eight entity variants, ordered geometry and vertices, and the four typed group-code value kinds attached to each of seven explicit owners. Its neutral fixture covers nine entities and thirteen extra/Other group codes. Initial test execution compiled both new tests and failed only authored count/selection expectations; these were corrected. The corrected rerun failed before compilation because the simultaneous hub split removed the inherited workspace `semio-s-plugin-stdio` key. No corrected DXF green claim.

Native PDF1.4 has its own document/page schema, complete typed projection/reconstruction (including empty documents), neutral fixture and two laws. PDF1.7 is being implemented in authored domain modules: COS values, ordered arrays/dictionaries, filters/predictors/CCITT, sampled/exponential/stitching/PostScript functions, colour spaces, and font descriptors/programs/CMaps/CID metrics. These partial modules do not substitute an incomplete implementation for the required complete Snapshot trait. Content operations, resources, navigation, annotations, forms and metadata remain required before the full trait is wired. The attempted PDF Nx target failed graph processing with `Unknown runtime component stdio` before compilation, due to the simultaneous package split.

The Note provider preflight now includes exact numeric storage bytes alongside all strings before row ownership copies; reconstruction checks the full input aggregate budget before index/allocation. Its full typed graph, assets, text child identity, optional flags, ragged/empty tables, ink points and all link pin variants have authored laws, but its actual crate and guest component remain blocked by missing provider dependencies. Real Owned/Wasmtime guest semantic SQLite runtime laws remain required.

Parent reassigned native DWG to the TypeScript executor. No DWG source/provider edits were made in the I/O lane. Retained handoff is [DWG Provider Handoff](📓️dwg-provider-handoff.md), with exact sources and the complete public-type inventory. PDF1.7 field inventory is [PDF1.7 Native Fields](📓️pdf17-native-fields.md). Native PDF, DXF and Note mirror/third-party relational oracle checks remain outstanding; authored code is not completion evidence.

## October 1: Complete PDF Provider Authored

PDF 1.7 now has a handwritten relational schema and explicit Rust encoder/reconstructor for all persistent native fields: COS values and retained indirect objects, ten stream filters, all color/function variants, content operators and text operands, fonts/programs/CMaps, image/form/state/shading/pattern resources, document/navigation metadata, all 27 annotation kinds, appearance states, borders, markup, all five form-field kinds and recursive children/widgets, optional content, page fields, and ordered document collections. No typed projection uses reflective JSON or native pack as its relational representation. Intrinsic byte strings remain individual byte-valued columns. Native `usize` annotation references use checked high/low u32 words; native full-width COS integers remain SQLite INTEGER.

The canonical SQL and rich neutral fixture reside beside the PDF 1.7 snapshot in `🪶️sqlite`. Full mandatory trait stitching and eight SQLite tests are authored, including full snapshot equality, independent Bun SQLite integrity/FK/query/edit checks, typed I/O with no native encode/decode phases, malformed relations/schema, empty snapshots and controls. PDF 1.4 has its separate two-table schema, three-page neutral fixture and two runtime laws. These new tests are **not yet green**: the first raw registered PDF check stopped before compilation on locked Cargo.lock; the fresh check is now compiling shared dependencies after other native lanes refreshed the lock.

Exact profile discovery was audited: the PDF root had only 1.4/* and 1.7/* document-codec declarations, and the current shared registry performs exact lookup with no wildcard expansion. Eight explicit native profile codec declarations were therefore added for 1.4 a/x and 1.7 a/x/e/ua/vt/h, preserving the existing exact validators.

The shared `Projection` checks cancellation before a row larger than 64 KiB is copied, in addition to row intervals. PDF reader text/blob copies and Note reader text copies now delegate to the shared persistent `Reconstruction` accounting helper before ownership allocation. Fixed depth 512 remains on recursive PDF domains and Note block reconstruction and is an explicit outstanding restriction; this is not unrestricted snapshot coverage. Actual Note Owned/Wasmtime guest roundtrip is still pending complete dependency providers. DWG ownership was handed to the TypeScript fleet worker; this agent authored only its retained field inventory, not DWG provider files.

## Actual Declaration Runtime Exposed Duplicate I/O Ownership

The registered PDF native lane compiled and ran 13 SQLite laws: **11 passed**, including complete PDF 1.7 snapshot equality, SQL edits, independent Bun SQLite query/integrity/FK checks, and PDF 1.4 laws. Two actual declaration-driven I/O laws failed because the framework facade still mounted the full I/O source a second time, giving plugin assembly and kernel consumers separate static registries. This was not a missing PDF provider.

The framework root now reexports the canonical kernel-owned I/O module and its three thunk macros. Its source dependencies were verified to be kernel-owned vocabulary, diagnostics, store and physical SQLite types. Native codecs, subset validators, ordinary entries and supplemental snapshot entries therefore publish and read one registry across both public facades. The PDF runtime lane is rerunning after this correction. The framework I/O test target must now execute the kernel's libtests, since these tests are no longer duplicated in the framework crate.

PDF's actual declaration law checks all ten exact wildcard/profile routes and a conforming PDF/A typed roundtrip. It rejects a nonconforming PDF/A export and both mismatched import metadata and a forged exact PDF/A coordinate whose reconstructed snapshot fails the registered validator. The native owner exposes its relational capability through the new explicit ArtifactPack hook; Note/DXF/PDF 1.4 owners already expose equivalent concrete hooks.

Two actual Note component laws are authored in the existing host `owned-instance-open` tests, one per Owned/Wasmtime runtime. They use the actual typed Note fixture through a test-only artifact dependency, exercise binary and text snapshots, compare native and guest relational reconstructions, require exact owner schema and metadata, and reject mismatched targets, tiny budgets and cancellation. These laws intentionally require a freshly built component; they do not silently skip when the deliverable is absent. They remain unexecuted pending component dependencies/build.


## October 1 — Owned Strict Subset Validation

Owned typed SQLite I/O now calls `ArtifactSqliteSnapshot::validate_sqlite_snapshot_subset(&self, exact_dialect, borrowed_database, control) -> IoResult<()>` after projection or reconstruction. Named subsets reject unless the artifact owner explicitly supplies semantic validation; the wildcard remains unconstrained. The interface receives the already-built authored relational model, avoiding a second projection and preserving all fields that native wire codecs may normalize. Native-payload routes retain their exact registered wire validators. Provider-mode remote codecs do not bypass typed validation.

PDF1.4 supplies its existing scope-limited a/x warnings. PDF1.7 supplies all six actual declared profile checks, augmented with explicit typed action, every authored COS dictionary entry, encryption, and x/vt page-box checks. Owned I/O rejects Error/Fatal diagnostics and retains warnings. The existing generic owned I/O law now also exercises strict rejection and identical warning preservation across import/export, without EncodeNative/DecodeNative phases. The actual PDF/A declaration law uses a custom snapshot schema string that native PDF decoding canonicalizes; full typed SQLite import must preserve it.

The preceding PDF run had12 of13 laws passing. The genuine failure was an inline JavaScript OpenAction missed by a validator scanning only top-level indirect dictionaries. PDF/A retained-object scans now walk nested arrays/dictionaries/stream dictionaries iteratively; the new owned profile validator additionally examines semantic action rows independently of native lowering. Fresh verification is pending. No completed claim covers the remaining recursive512 limits or native IEEE NaN semantics; both are outstanding provider work.


## October 1 — PDF Strict Runtime Evidence And Note Depth

Fresh registered `@semio-tech/stdio-pdf-rs:test -- sqlite_snapshot_` passed all13 laws (521 unrelated tests skipped). This is actual runtime evidence for both PDF versions, full owned snapshots, every declared profile route, full typed PDF/A custom-schema preservation without native Encode/Decode phases, strict inline-JavaScript export and forged SQLite metadata import rejection, and independent Bun SQLite integrity/FK/query/edit interoperability. The earlier E0277 in the new diagnostic-budget expression was fixed before this green run.

DXF's expanded lane ran four laws: three passed, including actual declaration full owned I/O. The independent SQL oracle failed because its expectation incorrectly counted four line-point rows; the manually authored fixture has one line, hence two rows. Corrected rerun is pending.

Note reconstruction now assembles group children iteratively and removed the fixed512 nesting ceiling from preflight. Added a language-neutral1024-group plan fixture and an independent SQLite recursive-CTE traversal law. This has not run: its registered artifact lane failed before compilation on concurrent plugin discovery ENOENT for the removed writer Cargo manifest. Host guest test compilation separately reached Semio BRep dependencies and failed on external Body inherent impl ownership/from_snapshot errors. Neither failure is reported as a passing Note test.

Owned public export/import now require only ArtifactSqliteSnapshot; native pack traits remain solely in the erased native-payload path. Pending broad requirements include per-provider IEEE NaN companions, PDF recursive stack removal, TypeScript mirrors for Note/DXF/PDF, and actual Owned/Wasmtime Note guest component runtime verification.


## October 1 — Follow-up Scalar And Deep COS Work

PDF1.4 now declares width/height INTEGER bit and TEXT numeric-class companions. PDF1.7 explicitly declares binary64 companions for every scalar position in38 authored entity tables; the research inventory is retained in 📓️pdf17-ieee-fields.md. Its domain projection uses explicit table/column constants and its reader uses borrowed FloatRow views, preserving optional NaN presence, signaling/quiet payloads, infinities, subnormals and signed zero. Added neutral fixtures plus independent SQLite scalar queries and full owned I/O bit laws. These changes are newer than the13-test green and are not yet compiled or runtime-confirmed.

Note now has explicit binary64 companions for document settings, asset sizes, block geometry, text size, ink/color and point fields. Its reader remains iterative for block containment. PDF COS array/dictionary/stream traversal and reconstruction are now iterative and no longer carry an arbitrary512 ceiling; recursive PDF colors/functions/actions/outlines/form fields are still outstanding. Both providers need fresh runtime gates before their new scalar/depth behavior can be claimed.

Concurrent ownership moved the Note Owned/Wasmtime laws into the Note artifact's own sqlite/🧪️tests/🌐️guest module, mounted by its artifact root. The Note dev-dependency was repaired to the actual canonical existing host package via relative path, avoiding a removed workspace alias. Native checks advanced past that missing-key error and currently fail on multiple Cargo workspace roots (the standalone repo test package plus the root). The previous Note plugin package/component producer has also been removed, so the guest test must be retargeted to the active new component owner rather than silently finding a stale old component. No guest completion is claimed.


## Floating-Point and Iterative Containment Follow-Up (2026-10-01)

Every owned PDF binary64 semantic field now has an explicitly declared logical REAL plus exact bits/class companions. These are handwritten positions per table, shared physical helpers only preserve the numeric representation. All 38 PDF 1.7 tables containing binary64 fields and the separate PDF 1.4 page dimensions are covered. Note adds six table mappings (document settings, asset dimensions, geometry, typography, ink stroke/color and points). DXF adds six mappings (header/group-code values, block base point, entity dimensions, geometry points and vertices). NaN remains queryable as SQL NULL plus class/bits; optional absence is all three NULL, preserving Some(NaN) distinctly. Independent SQLite edits to numeric fields update their declared companions and are validated strictly.

DXF actual runtime evidence: registered `@semio-tech/stdio-dxf-rs:test sqlite_snapshot_` ran five tests successfully, run `7b22bc6c-c53a-45a6-816b-223a358466f0`. This includes the corrected independent two-point line join, SQL circle-radius edit, full actual declaration owned I/O, all eight entity kinds, all four typed group-code kinds, controls and seven IEEE payload cases. A sixth explicit owned coordinate/projection guard law was added after this green and awaits the current rerun.

PDF library compilation passed with the numeric mappings and iterative COS implementation. New runtime fixtures cover seven IEEE values and depth1,024 containment using independent Bun SQLite queries. All fixed512 semantic traversal limits are removed from COS, functions, color bases/alternates, action.next, outline children and form-field children. Projection/reconstruction use explicit domain stack frames, borrow native children until copied, reject consumed identities/cycles, and retain resource/cancellation checkpoints. Native nested Rust model drop remains owned by its type; depth1,024 runtime laws assert fields through iterative borrowed walks instead of recursive equality. No unrestricted-depth completion claim is made before runtime evidence.

Current Note guest source follows the canonical `🌎️hub/🧩️compositions/🗒️note` component, `semio_hub_note.wasm`; the registered inferred producer is `@semio-tech/note-plugin:component-dev`. The previous Note plugin package is not a fallback. The Note artifact test host dependency uses the explicit canonical host Cargo path because the root workspace alias was removed. Note unit and guest runtime tests still await successful component/dependency builds; no guest runtime green is claimed.

Authored host fixture deployment metadata is `neutral-host-fixture`, matching its component package `semio:neutral-host-fixture`. This follows the owned catalog declaration policy and repairs the relevant component graph seam, without a retired alias. Registered Note raw check `--tests` is prohibited by the new native input contract; actual Note tests use the dedicated test target. No raw Cargo bypass was introduced.


### Headless Fixture Publication Boundary

The concurrent catalog owner completed optional deployment authority during validation: component metadata without a deployment directory is a supported headless component, while `findPluginCargoFiles` publishes only explicitly deployed components. The temporary host fixture deployment field was consequently removed after its publication caused a genuine registry failure (no public directory for neutral-host-fixture). The final fixture remains a headless test component with explicit package/kind, and its dedicated build target owns the test output. No public catalog row or fixture alias was introduced.


## Final Native Domain Gates and Host Ownership Follow-Up

Fresh PDF gate `@semio-tech/stdio-pdf-rs:test sqlite_snapshot_` is GREEN18/18 (521 unrelated tests filtered), run `450795df-1e95-4740-b346-e89bbd778680`. The new laws prove PDF1.4/1.7 IEEE binary64 bit state, optional presence, all handwritten domains, all exact profiles/owned schema fields, and depth1,024 COS/functions/color/action/outline/form trees with independent SQLite integrity, foreign-key and recursive/count queries. The first cancellation test failure came from unwrapping an expected cancellation at Reader creation; its final assertion accepts that bounded early cancellation as intended. Fresh DXF is GREEN6/6 (36 unrelated tests filtered), run `00ae0b1c-1547-42e7-904f-a350d0bd831f`, now including its exact owned coordinate/projection guard.

The public TypeScript native-host module lacked a declaration for its owned JavaScript implementation. Added `🟨️.d.mts` with exact five runtime function contracts and owned identity/source/facts types, reexported those types from the TS wrapper and removed its duplicated literal contracts/cast. Registered native-host contract checks passed all11 vectors with independent Ajv/TOML oracles, and actual OBJ package typecheck passed suites2. No compatibility module was introduced.

Note native/component prerequisites advanced to WGPU frame-worker generation and rejected the new shared IEEE browser import because it was absent from the exact authored browser source roster. Added only the actual runtime IEEE module path beside the existing SQLite core in both the canonical browser profile and generator input list. Artifact helper imports in the framework root are type-only, so they are not added as an unused browser runtime module. The narrow generator gate is running; Note runtime remains unverified pending it.

## Shared I/O Green And Canonical Note TypeScript Ownership

The canonical WGPU frame-worker target passed after the exact IEEE runtime path was added. The later canonical I/O JSON-schema runtime import required its own exact authored path; the concurrent catalog owner supplied that path in both roster and inputs. The latest Note component attempt now passes generator/catalog/browser prerequisites and is compiling. The previous owned Note match-arm syntax error was repaired. No Note component runtime result is claimed yet.

Fresh `@semio-tech/framework-rs:test-snapshot-sqlite-io` is GREEN four runtime laws, with941 unrelated tests filtered. It exercises actual declaration registration, complete subset discovery, headless atomic publication, owned type/dialect mismatch rejection, strict semantic rejection and warning preservation without native Encode/Decode phases, native Fatal validator rejection, and guest wire diagnostic/resource contracts. The new fixture hooks use the canonical `store` module; the final FaultCode assertion reads its actual owned tuple field.

The Note TypeScript mirror is handwritten over the same fourteen adjacent SQL tables. Its four neutral/independent SQLite laws are GREEN with1054 assertions: all six block domains and composed text/asset/link relations, ragged/empty children and rows, SQL text edits, full unsigned64 blob pins, all seven binary64 bit fixtures including NaN optional presence, hostile ownership/ordering/cycles, budgets/cancellation, and1,024-group containment with independent recursive SQL. The first missing-provider run was red; the first implemented run had three laws green and one true authored SQL CHECK typo, then the corrected schema-equal run passed all four.

No separate NoteSqliteSnapshot model is retained. The existing canonical NoteSnapshot, NoteArtifact, NoteBlockNode, NoteImageAsset and NoteDiff numeric fields now own Binary64 words. Native JSON syntax still admits finite numbers through its declared JSON schema and converts them into owned words at that explicit boundary. Exact SQLite reconstruction uses words directly and never routes through JSON. Existing native wire facades are unwired and are not claimed to support NaN JSON transport. The Note native JSON contract oracle is being verified with explicit authored numeric output projection, keeping finite JSON interoperability separate from complete owned IEEE state. New registered Node targets are `@semio-tech/note-js:test-snapshot-sqlite`, `:test-document-contract`, and `:check`; source/schema inputs include the shared physical and IEEE implementation. Typecheck and native JSON oracle outcomes are pending.

### Canonical Note TypeScript Final Checks

Fresh Note `:check` is GREEN (6.7s). The fresh SQLite rerun is GREEN four tests,1054 assertions,0 failures (4.8s). The native JSON document-contract target is GREEN with independent Ajv validation for66 committed native snapshots and33 committed diffs plus the owner/child rejection vectors. Its test-only numeric output projection explicitly covers Note settings, frame geometry, font size, stroke color/points, assets and sparse diff changes. The shared document-contract oracle optionally accepts this explicit native output projection; other owners retain their existing direct output comparison. Its production admission still rejects unknown fields/variants and invalid native numbers. SQL reconstruction preserves owned binary64 words independently of native JSON representability.

Additional TypeScript files owned in this follow-up: Note canonical schema/🟦️.ts, snapshot/🟦️.ts and diff/🟦️.ts; snapshot/sqlite/🟦️.ts and sqlite/tests/🟦️.ts; Note native JSON contract test; Note TypeScript package script and project; shared document-contract oracle. Existing native Note Owned/Wasmtime laws and the rebuilt component are still being verified. No successful guest runtime claim is inferred from TypeScript/native physical engine tests.
