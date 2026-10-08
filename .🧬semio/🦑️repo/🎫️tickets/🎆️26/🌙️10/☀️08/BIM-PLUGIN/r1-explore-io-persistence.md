# r1 — Artifact IO, persistence, replication and examples

Read-only exploration. All paths are relative to `C:\git\semio`. The reference artifact is `s.draw.drawing@1/*` (the drawing plugin), with framework modules under `🧰️framework/🔨️modules` and the `os` store under `🧰️framework/🛍️products/💻️os/🔨️modules`.

## 0. Short answers

- **Persistence is two things.** The document snapshot is stored as a binary pack or DSL text, or projected into SQLite (a full-state projection, 32 tables, not a log). The event log is the append-only `.spr` record stream (`REC_EDIT` and hash-chained `REC_COMMIT` frames).
- **Lanes** map to four store types: `Artifact` = persisted shared, `Config` = persisted local-only, `Presence` = ephemeral shared, `Transient` = ephemeral local-only. `Draft` is a fifth lane for drafts that `PruneDrafts` removes.
- **io** is declared once per subset by `io() -> IoDeclaration`, with typed `Serializer<S>` / `Deserializer<S>` leaves registered through `serializer_entry` / `deserializer_entry`.
- **stdio** (`✏️s/🔌️plugins/🗄️stdio`) is the dictionary of foreign formats (about 38 kinds, including gltf, step, ifc, pdf, svg, dwg, dxf). Other plugins reference its dialects (`s.stdio.gltf@2.0/*`) as export or import targets.
- **Source conflict:** the TS mirror `🚪️io/🟦️.ts` says pdf, png, dwg, dxf export and svg import are "honest not-yet-implemented stubs". The Rust `io()` registers them and their Rust headers describe real implementations (pdf in `📖️pdf/🔖️1.4/✳️any/🦀️.rs` paints real PDF content). Treat the Rust as authoritative and verify before relying on either.

## 1. Drawing `io/`: serialization, SQLite, import/export declaration

Subset root: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/`

Layout of `🚪️io/` (112 files):

- `🚪️io/🦀️.rs`: `io() -> IoDeclaration` (the registry rows), plus the SemioBridge functions.
- `🚪️io/💾️binary/` (`📸️snapshot`, `🔺️diff`, `🧬️mutations`, `💡️inferences`)
- `🚪️io/📝️text/` (same four facets)
- `🚪️io/🪶️sqlite/📸️snapshot/` (`🗄️.sql`, `🦀️.rs`, tests, fixtures)
- `🚪️io/📤️export/🧵️serializers/🗿️artifacts/<format>/<standard>/✳️any/` (svg 1.1, pdf 1.4, png 1.2, json rfc8259, dwg ac1018, dxf r12)
- `🚪️io/📥️import/🧩️deserializers/🗿️artifacts/<format>/<standard>/✳️any/` (json rfc8259, svg 1.1 with `📄️document`, `🛤️path`, `↗️transform`)
- `🚪️io/🧪️tests/🔬️unit/`

### 1.1 Snapshot serialization

Native codec declared in `io()` (`🚪️io/🦀️.rs`, `io()` body):

```rust
native: NativeCodecs {
    snapshot: LanguagePair { text: None, binary: None },
    diff: LanguagePair { text: None, binary: None },
    mutations: LanguagePair { text: None, binary: None },
    inferences: None,
    codec: store::ArtifactCodec::bare::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA.to_string()),
},
```

The `text`/`binary` slots are `None` on purpose (the comment says so). The real codecs are the trait impls on `DrawingSnapshot`, below.

**Binary** (`💾️binary/📸️snapshot/🦀️.rs`): `store::ArtifactPack` is implemented by hand. Encoding wraps a record in a `SemioEnvelope`:

```rust
let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1)...;
Ok(store::semio_format::wrap_binary(&envelope, &inner))
```

Framing is normative in `💾️binary/📸️snapshot/📡️.protocol.semio`: magic `0x8953f83f7d340d0a`, fixed 32-byte header, varint payload segments, 64-byte footer with `body_crc32`.

**Text** (`📝️text/📸️snapshot/🦀️.rs`): `store::ArtifactDsl` with `envelope_id() = "drawing.drawing"`:

```rust
fn print_dsl(&self) -> String {
    let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
    ...wrap_text(&envelope, &body)
}
```

The grammar `📖️.grammar.semio` is generated from `#[dsl(id = "drawing.drawing", layout = "lines")]` on `DrawingSnapshot`. Example document: `🖼️assets/🎬️demo/🗣️.dsl.semio` (header `semio drawing.drawing.dsl v1`, then `schema=drawing.document id=semio title="Semio Emblem"` and a `layers { ... }` block).

**Mutation wire ops**: `💾️binary/🧬️mutations/🦀️.rs` implements `protocol::OpBinary` as `dsl::variants_binary::encode_op(self)`. `📝️text/🧬️mutations/🦀️.rs` provides the `OpText` text forms. Both are on the same `DrawingMutation` enum (`🧬️schema/🧬️mutations/🦀️.rs`, `#[mutations(snapshot = DrawingSnapshot, diff = crate::diff::DrawingDiff, schema = "drawing.drawing")]`).

### 1.2 SQLite persistence

Schema `🪶️sqlite/📸️snapshot/🗄️.sql` has 32 `CREATE TABLE` statements. It is a relational projection of the snapshot, not an event log:

```sql
CREATE TABLE draw_document(id INTEGER PRIMARY KEY, schema TEXT NOT NULL, artifact_id TEXT NOT NULL, title TEXT);
CREATE TABLE draw_scalar(id INTEGER PRIMARY KEY, value REAL, bits INTEGER NOT NULL, class TEXT NOT NULL);
```

Every float goes through `draw_scalar` with its IEEE-754 bits (`insert_ieee754(p, "draw_scalar", ..., FloatColumn::Binary64(1))`). Tables cover layers, transforms, shapes, paths and segments, text, images, groups, booleans, traces, fills and gradient stops, strokes and dashes.

`🪶️sqlite/📸️snapshot/🦀️.rs` implements the codec hooks on `DrawingSnapshot`: `to_sqlite_database` / `from_sqlite_database` through `project` / `reconstruct`, `validate_sqlite_snapshot_subset` (dialect must be `s.draw.drawing` / `1` / `*`), and progress and cancellation through `SqliteSnapshotControl`. It is exposed as `ArtifactSqliteSnapshotCodec` and checked by `🪶️sqlite/📸️snapshot/🧪️tests/🦀️.rs` (`sqlite_snapshot_draw_actual_io_declaration_capability`).

### 1.3 How import and export are declared

`io()` returns `IoDeclaration { native, entries }`. The eight entries (`🚪️io/🦀️.rs`):

```rust
serializer_entry::<DrawingSnapshot, export::svg::v1_1::any::DrawingIntoSvg>(DRAWING_DIALECT),
serializer_entry::<DrawingSnapshot, export::pdf::v1_4::any::DrawingIntoPdf>(DRAWING_DIALECT),
serializer_entry::<DrawingSnapshot, export::png::v1_2::any::DrawingIntoPng>(DRAWING_DIALECT),
serializer_entry::<DrawingSnapshot, export::json::v_rfc8259::any::DrawingIntoJson>(DRAWING_DIALECT),
deserializer_entry::<DrawingSnapshot, import::json::v_rfc8259::any::JsonIntoDraw>(DRAWING_DIALECT),
deserializer_entry::<DrawingSnapshot, import::svg::v1_1::any::SvgIntoDraw>(DRAWING_DIALECT),
serializer_entry::<DrawingSnapshot, export::dwg::v_ac1018::any::DrawingIntoDwg>(DRAWING_DIALECT),
serializer_entry::<DrawingSnapshot, export::dxf::v_r12::any::DrawingIntoDxf>(DRAWING_DIALECT),
```

Framework contract, `🧰️framework/🔨️modules/🚪️io/🦀️.rs`, `pub mod io_mechanism`:

```rust
pub trait Serializer<S> {
    const INTO: Dialect;
    const FIDELITY: IoFidelity;
    fn serialize(from: &S, children: &ArchiveChildren) -> impl Future<Output = IoResult<IoPayload>> + Send;
}
pub trait Deserializer<S> {
    const FROM: Dialect;
    const FIDELITY: IoFidelity;
    const CONFORMANCE: Option<fn(&S) -> Vec<Diagnostic>> = None;
    fn sniff(_payload: &IoPayload) -> ... { Confidence::None }
    fn deserialize(payload: &IoPayload) -> impl Future<Output = IoResult<S>> + Send;
}
```

`serializer_entry` and `deserializer_entry` build a type-erased `IoEntry { from, into, fidelity, direction, sniff, run }`. Each leaf is one file: a `Serializer` impl whose `INTO` is the foreign dialect. For example `📤️export/.../🎨️svg/🔖️1.1/✳️any/🦀️.rs`:

```rust
pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };
impl Serializer<DrawingSnapshot> for DrawingIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &DrawingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let (svg_text, _width, _height) = drawing_document_to_svg(from)...;
        Ok(IoOutcome::clean(IoPayload::Text(svg_text)))
    }
}
```

The subset root `🪆️subsets/✳️any/🦀️.rs` mounts it: `SubsetDeclaration { dialect: DRAWING_DIALECT, schema, io: io::io(), viewer, editor, examples }`.

Status per entry, by the Rust bodies:

- svg export, json export and json import: real.
- pdf export: real (vector content, shading, SMask images) per its header.
- png, dwg and dxf export: real. They project through `drawing_document_to_semio_drawing` into `s.stdio.semio` and call the shared writer. All three are `IoFidelity::Lossy`.
- svg import: real, incremental with progress and cancellation (`📄️document/🦀️.rs`).

The TS mirror (`🚪️io/🟦️.ts`, `ioEntries`) says otherwise (see section 0).

Fixture-generated JSON carrier (`🏭️generator/`, `🔬️probes/`): `🏭️generator/🧩️json/🏗️generate/🦀️.rs` writes fixtures for the layer-metadata kinds, which SVG cannot carry (`locked`, `blendMode`, `name`). `🔬️probes/📖️reader/🦀️.rs` is a `quick-xml` reader for exported SVG, and `🔮️oracles/🔣️.json` says there is no third-party oracle for drawing semantics.

## 2. Artifact identity and versioning

Identity types (`🧰️framework/🔨️modules/🧬️schema/🗿️artifact-reference/🦀️.rs`):

```rust
pub struct StandardId(pub &'static str);
pub struct SubsetId(pub &'static str);
impl SubsetId { pub const ANY:Self=Self("*"); }
pub struct Dialect { pub artifact_kind:&'static str,pub standard:StandardId,pub subset:SubsetId }
pub struct ArtifactDialect { pub artifact_kind:String,pub standard:String,pub subset:String }
```

Canonical coordinate text is `kind@standard/subset`, for example `s.draw.drawing@1/*` (`DialectCoordinateText::to_coordinate`, `🚪️io/📝️text/🗿️artifact-reference/🦀️.rs`). An artifact kind id is three lowercase kebab segments, `<domain>.<plugin>.<artifact>`.

So `🏅️standards/🔖️1/🪆️subsets/✳️any` means:

- `🏅️standards/🔖️1` is **standard "1"** of the artifact kind `s.draw.drawing`. It is the version of that artifact's schema contract, with its own media declaration. Its root `🏅️standards/🔖️1/🦀️.rs`:

  ```rust
  pub fn standard<A: crate::DrawingApplication>() -> StandardDeclaration<A> {
      StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.drawing+json"], extensions: &["drawing"] }, subsets: vec![subsets::any::subset()] }
  }
  ```

- `🪆️subsets/✳️any` is **subset `*`** (`SubsetId::ANY`). It is the shared substrate for the whole standard (editor, viewer, io, schema core, examples). Its manifest `🪆️subsets/🔣️.json` lists the domain subsets `structure`, `style`, `transform` and `metadata`, which own the 14 mutations (folder `🧱️structure`, `🎨️style`, `🔀️transform`, `🏷️metadata`). The `any` note says it "Owns no mutation any more". The JSON key is `any`, but the folder and `SubsetId` are `✳️any` / `*`.

Declaration chain:

- Artifact root `🦀️.rs` line 546: `ArtifactDeclaration { kind: ArtifactKindId::parse("s.draw.drawing")..., standards: vec![standards::v1::standard()] }`.
- `pub mod standards { pub mod v1 { #[path = "🏅️standards/🔖️1/🦀️.rs"] ... } }` at line 567.
- The `DRAWING_DIALECT` constant (`🦀️.rs` line 490) is `Dialect { artifact_kind: "s.draw.drawing", standard: StandardId("1"), subset: SubsetId::ANY }`.

**Adding a second standard** (from the existing pattern; nothing written): create `🏅️standards/🔖️2/🦀️.rs` with its own `standard()` and `StandardId("2")`, mount `pub mod v2` beside `v1`, and push `standards::v2::standard()` into `ArtifactDeclaration.standards`. Each subset under it carries its own `🚪️io` (own `DRAWING_DIALECT`-like constant), `🧬️schema`, `🔮️oracles` and `🧪️tests`. The stdio plugin already uses this shape: `🗿️artifacts/🎞️gif/🏅️standards/` has `7️⃣87a` and `9️⃣89a`, and `🔖️2.0` for gltf. Caveat: `9️⃣89a/🧬️migrations/🦀️.rs` exists there, but AGENTS.md forbids migrations in this repo, so do not copy that part.

## 3. Event sourcing, replication and offline behaviour (author API)

### 3.1 Required author surface

The artifact author implements `ArtifactEditor` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, around line 39088, `pub trait ArtifactEditor: Default + Send + 'static`). The trait items without a default body that the author must supply include:

```rust
type Snapshot: ... + store::ArtifactDsl + ArtifactPack + ...;
type Mutation: protocol::SemanticMutation<Self::Snapshot> + ::protocol::OpText + ::protocol::OpBinary + ...;
type Config / ConfigMutation / Draft / DraftMutation / Presence / PresenceMutation / Transient / TransientMutation / Command;
const DIALECT: Dialect;
const DOCUMENT_SCHEMA: &'static str;
fn initial_snapshot() -> Self::Snapshot;
```

Optional items with default bodies include `initial_config`, `initial_draft`, `config_schema`, `child_restore_projection`, `build_*_store_owners`, `build_*_disposer`, `build_*_one_item_preparation_factory` and `build_*_retirement_factory`. The drawing editor (`✏️editor/🦀️.rs` around line 1590) sets the lanes explicitly:

```rust
type Snapshot = DrawingSnapshot;
type Mutation = DrawingMutation;
type Config = NoConfig;
type ConfigMutation = NoConfigMutation;
type Draft = NoDraft;
type DraftMutation = NoDraftMutation;
type Presence = DrawingPresence;
type PresenceMutation = DrawingPresenceMutation;
type Transient = semio_framework_plugin::NoTransient;
type TransientMutation = semio_framework_plugin::NoTransientMutation;
```

Store owners are supplied by the host module: `build_document_store_owners` returns `drawing_document_store_owners()` (`🔨️modules/🏠️host/🧰️owned/🦀️.rs` line 4289) and the envelope decoder is `drawing_envelope_decode_owner_bundle()` (line 338). Both live in `crate::spr`, re-exported from the subset root.

Commands return `Emit<DrawingMutation, NoConfigMutation>` or `Fault` (see `✏️editor/🎮️commands/⌫️delete-selection/🦀️.rs`). `Emit` carries the mutations for each lane.

### 3.2 Mutation contract

`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs`:

```rust
pub trait Mutation<P>: Clone + ToValue + FromValue {
    type Diff: MutationDiff<P>;
    const DESCRIPTORS: &'static [MutationLeafDescriptor];
    fn diff(&self, base: &P) -> MutationOutcome<Self::Diff>;
    fn inverse(&self, base: &P) -> Result<Vec<Self>, ValueError>;
    fn state_class(&self) -> StateClass { StateClass::Artifact }
    fn undo_policy(&self) -> UndoPolicy { UndoPolicy::ExactBaseOnly }
    fn conflict_target(&self) -> Vec<String> { Vec::new() }
    ...
}
pub trait MutationDiff<P>: Clone + Default + PartialEq + ToValue + FromValue + DiffAlgebra<P> {
    fn apply(&self, base: &P, capability: ApplyCapability) -> MutationApplyResult<P>;
    fn absorb(&mut self, other: Self);
}
pub trait DiffAlgebra<P>: Sized {
    fn inverse(&self, base: &P) -> Self;
    fn between(base: &P, other: &P) -> Self;
    fn is_empty(&self) -> bool;
}
pub fn apply_diff<P, D: MutationDiff<P>>(diff: &D, base: &P) -> MutationApplyResult<P> { ... }
```

`apply_diff` is the only place that can mint `ApplyCapability`, so leaves never call `apply` directly. Laws, from the docs: `d.inverse(base).apply(d.apply(base)) == base`, and `absorb` is sequential-only (concurrent merging belongs to the authority).

Drawing defines the mutations with `#[derive(dsl::Mutations)]` (`🧬️schema/🧬️mutations/🦀️.rs`). Presence is hand-written: `impl Mutation<DrawingPresence> for DrawingPresenceMutation` (`✏️editor/👥️presence/🦀️.rs` line 148), with one variant `Set { engagement_input, camera }`. Note that `DrawingPresenceMutation` does not override `state_class()` (grep: no `state_class` in the drawing subset), so its lane comes from the store it is emitted to, not from this method.

### 3.3 Log, replay and persistence

- **Write path:** all public writes go through `ArtifactStore::dispatch(ArtifactCommand)`. The enum (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` around line 3565) is:

  ```rust
  pub enum ArtifactCommand<Mutation> {
      Apply { mutations: Vec<Mutation>, transaction: Option<TransactionRef> },
      Undo, Redo, UndoWithPolicy { .. }, ApplyInLane { .. }, UndoInLane { .. }, RedoInLane { .. },
      CommitCheckpoint { message, authors }, CreateAlternative { name }, SwitchAlternative { .. }, CheckoutCheckpoint { .. },
      IngestRemote { envelope: MutationEnvelope }, PruneDrafts, SetMergePolicy { .. },
  }
  ```

- **Wire envelope:** `MutationEnvelope { mutation_id, document_id, actor, dependencies, observed, target, diff, inverse, timestamp, transaction }` (`📡️replication/🔗️causal/🦀️.rs` line 52). Edits are `Edit<Op> { id, actor, line, forwards, inverse, mutation_meta, verb }` (`📡️replication/🎮️mutation/🦀️.rs` line 1682).
- **Durable log:** `.spr` is an append-only record stream. Its magic is `0x89 'S' 'P' 'R' 0x0D 0x0A 0x1A 0x0A`, with a 32-byte header. Frame kinds include `REC_DOC 0x01`, `REC_EDIT 0x04`, `REC_COMMIT 0x0C` (hash-chained), `REC_EPHEMERAL 0x10`, `REC_COMPACTION 0x12` (`📡️replication/📐️format/🦀️.rs`, `🧾️wire/🦀️.rs`). `.spk` is the write-once footer-rooted pack (`🎒️pack`). Replication helpers: `extract_range` and `verify_slice` (`📡️replication/🦀️.rs`).
- **Replay:** `fold_history` and `fold_history_for` (`📡️replication/🔗️causal/🔀️transition/🦀️.rs` line 568) fold edits into a head. The `.spr` decoder is `artifact_owned_spr_edit_history_decoder` (store, around line 8768). Snapshots are materialized from pack or SQLite.
- **Shape of persistence:** the snapshot (pack, DSL or SQLite) is the materialized head. `.spr` is the log of edits behind it.

### 3.4 Time travel

`⏪️time-travel/🦀️.rs` is a pure reducer for one **ephemeral, local-only** history edit: `TimeTravelSession` with events `Begin`, `BaseMoved`, `Withdraw`, `Restore`, `Rerun`, `Exit`, and effects. The host forwards every store content revision as `BaseMoved`. A mutation that is applied inside time travel is not persisted until it is finalized.

### 3.5 Offline and short connection shortage

- Shortage policy (`🏪️store/🔄️sync/🦀️.rs` around lines 1211-1240): `DOCUMENT_LINK_SHORTAGE_POLICY` uses `reconnect_min_ms`, `reconnect_max_ms` (`HUB_RECONNECT_*`) and `shortage_bound_ms = 2 * HUB_RECONNECT_MAX_MS`. `retry_at` never schedules past the bound. `ceiling_at` ends the link at bound + one backoff. `DOCUMENT_LINK_ACCESS_REFUSED_STATUSES = [401, 403, 404, 410]` are never retried.
- Actor behaviour: during `RebootstrapRequired` the actor "keeps its unacked work", and `Reseed` hands unacked local operations back to the guest. `Detach` "flushes any pending outbound operations". Outbound operations are a store concern (`ArtifactActorMsg`, `🔄️sync/🦀️.rs` around line 225).
- The local store is optimistic: `ArtifactStore` is described as "Local-first, non-blocking, client-side, in-memory document store" with backbones `temp://`, `file://`, `folder://`, `remote://`.
- Durability classes (`🛢️db/💾️durability/🦀️.rs`): `Memory`, `Os`, `Fsync`, and `Quorum(n)` replica acknowledgement. Read consistency (`🛢️db/🔍️query/🦀️.rs`): `Canonical`, `AtLeast(Frontier)`, `Exact`, `Historical`, `Speculative`, `PreviewAugmented`.
- Gap: I did not find a durable client-side outbox file. Unacked operations appear to be held by the actor in memory, and I did not confirm whether they survive a restart. Needs a follow-up check.

### 3.6 Lanes: persisted versus ephemeral

`🧰️framework/🔨️modules/🧬️schema/📶️state/🦀️.rs`:

```rust
pub enum StateClass { Artifact, Config, Presence, Transient }
// Artifact = persisted shared, Config = persisted local-only,
// Presence = ephemeral shared, Transient = ephemeral local-only.
```

Store types (`🏪️store/🦀️.rs`):

| Lane | Store | Behaviour |
|---|---|---|
| Artifact (persisted shared) | `ArtifactStore<Snapshot, Mutation>` | Full history, undo, checkpoints, merge policy, `.spr` |
| Config (persisted local-only) | `ConfigStore = ArtifactStore<C, CM>` | Same algebra, not shared |
| Draft | `DraftStore` | Same algebra; `PruneDrafts` never enters a change |
| Presence (ephemeral shared) | `PresenceStore<P, M>` | No history, undo or merge. A later frame from a peer supersedes its earlier one. `generation` counts local changes for heartbeat throttling |
| Transient (ephemeral local-only) | `TransientStore<P, M>` | Never shared, persisted, packed, checkpointed or undone |

The doc comment on `TransientStore` says: "If a value must survive a reload it belongs in `config`; if a peer must see it, in `presence`; if it is document content, in the artifact (or its draft)."

Presence transport frames: `ClientFrame::Presence { peer }` and `ServerFrame::Presence { peers }`. Peer overlay derivation: `📡️replication/👕️peer-overlay/🦀️.rs` ("Ephemeral shared only — never persisted").

Liveness for plugin shards (not user connections): `🎭️actor/📮️shard-client/🫀️liveness/🟦️.ts`, `SHARD_LIVENESS_POLICY` (heartbeat 5000 ms, missed limit 3).

## 4. The `🗄️stdio` plugin

Path: `✏️s/🔌️plugins/🗄️stdio/`. Its `AGENTS.md`: "Zero-app library plugin whose artifacts are well-known file formats. Codecs are real."

Formats (folders under `🗿️artifacts/`, about 38): ☁️las, 🌐️html, 🌦️epw, 🎒️zip, 🎞️gif, 🎥️mp4, 🎨️svg, 🎵️mp3, 🏗️ifc, 💬️bcf, 💾️binary, 📇️inventory, 📊️csv, 📐️step, 📑️tsv, 📕️xlsx, 📖️pdf, 📜️docx, 📝️md, 📰️xml, 📷️png, 📸️jpg, 📼️avi, 📽️pptx, 🔊️wav, 🔣️json, 🔤️txt, 🔺️stl, 🖊️dwg, 🖋️dxf, 🖼️tiff, 🗜️deflate, 🗽️obj, 🧊️gltf, 🧱️ply, 🧾️json, 🧿️semio, 🪟️bmp.

So, to your list: **gltf** (2.0), **step** (AP214, subsets cc1 to cc4), **pdf** (1.4 and 1.7), **svg** (1.1), **ifc** (2x3 with cobie and cv20 subsets; the 4x4 standard is a "universal stub" per its own source comment), and also obj, stl, ply, las, dwg, dxf, zip and others.

Each format follows the same tree: `🏅️standards/<std>/🪆️subsets/<subset>/`, with `🚪️io/{💾️binary,📝️text,📤️export,📥️import,🪶️sqlite}`, `🧬️schema`, `🔮️oracles` and `📦️packages/{🦀️rust,🟦️typescript}`.

Format descriptors: `🗿️artifacts/🧊️gltf/🦀️.rs`:

```rust
pub fn formats() -> Result<Vec<semio_framework_plugin::io::FormatDescriptor>, ...> {
    validate_definition_schema(ARTIFACT_DEFINITION_SCHEMA)...;
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
fn native_codec() -> store::ArtifactCodec {
    let mut codec = store::ArtifactCodec::bare::<GltfSnapshot, GltfMutation>(STDIO_GLTF_DOCUMENT_SCHEMA);
    codec.extension = "gltf";
    codec
}
```

Do other plugins export through stdio? Yes. Counting `s.stdio.*` references across `✏️s/🔌️plugins` (rough `rg` count, outside and inside stdio): `s.stdio.semio` 1546, `s.stdio.gltf` 791, `s.stdio.json` 289, `s.stdio.pdf` 238, `s.stdio.txt` 204, `s.stdio.step` 120, `s.stdio.ifc` 113, `s.stdio.zip` 111, `s.stdio.svg` 107, `s.stdio.dwg` 88, `s.stdio.png` 82. About 21 plugin `io` files register `serializer_entry` or `deserializer_entry`.

Export path, quoted from drawing (section 1.3), is the `serializer_entry` pattern: `INTO = s.stdio.svg@1.1/*`, `serialize` calls the SVG writer, and the writer builds the stdio SVG and XML documents. A second, older pattern is still in use by gis terrain. `🌍️gis/🗿️artifacts/🏔️gisterrain/🦀️.rs` line 174:

```rust
ComposerEntry { writes: EXPORT_GLTF_DIALECT, reads: &[GISTERRAIN_DIALECT], compose: compose_export_gltf },
```

with `EXPORT_GLTF_DIALECT = Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") }`. Drawing's own comment says `ComposerEntry` was deleted from it, so the two patterns coexist during migration.

CAD STEP export: `📐️cad/.../🚪️io/🦀️.rs` line 172, `CAD_SOLID_EXPORT_DIALECT_STEP = "s.stdio.step"`. Lowpoly: `EXPORT_GLTF_DIALECT` at `💠️lowpoly/.../🚪️io/🦀️.rs` line 208.

## 5. Declaring references to other artifacts

- Identity: `ArtifactRef { artifact_id: String, dialect: ArtifactDialect }` (`🧬️schema/🗿️artifact-reference/🦀️.rs` line 99). Its text binding is `🚪️io/📝️text/🗿️artifact-reference/🪆️binding/📡️.protocol.semio` (`record ArtifactRef { artifact-id, artifact-kind, standard, subset }`). URI form: `ArtifactReferenceText::to_uri` / `parse_uri`.
- Schema-level children: `#[child(kind = "s.stdio.semio")]` on a field. Examples: `🪵️sourcing/🗿️artifacts/🗂️curation/.../🧬️schema/🦀️.rs` line 18, `🌊️flow/.../🧬️schema/🦀️.rs` line 86. Traversal: the `ChildRefVisitor` trait gets `child(slot, ChildRefFields { child_id, artifact_id, artifact_kind, standard, subset })` (`🧰️framework/🔨️modules/🧬️schema/🧩️composition/🦀️.rs`).
- Runtime: `store::ArtifactChild<S> { child_id, target: ArtifactRef, read_owner }` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧵️owner/🦀️.rs`). `ArtifactChildRead` gives the child snapshot read-only, and children are read on load, never through the parent's owner.
- Drawing declares no child references (`rg` finds none outside tests).

## 6. Examples and tests (conventions)

- Examples: `📚️examples/🎬️demo/🦀️.rs` (`ID = "demo"`, `PRIMARY_TEXT = include_str!("../../🖼️assets/🎬️demo/🗣️.dsl.semio")`, `ExampleSource::new(ID, label(), PRIMARY_TEXT, ICON)`). The subset root registers `examples()` as the single catalogue. The example's TS twin is `📚️examples/🎬️demo/🟦️.ts`. Its manifest is `📚️examples/🎬️demo/🖼️assets/🛂️manifest.json`.
- The editor's `✏️editor/📚️examples/🎬️demo-session` is a command-replay fixture, deliberately not listed as an example.
- Language-agnostic tests: Gherkin `.feature` files plus Rust step runners under `🧪️tests/` (for example `🧪️tests/🔁️round-trips-the-committed-document/🥒️.feature`, which parses and reprints `asset://🎬️demo/🗣️.dsl.semio` byte for byte). Mutation tests are per standard subset (`mutate-drawing-1-any-style`, `-transform`, `-structure`, `-metadata`).
- Test fixtures: `🧫️fixtures/🔣️.json`, per IO facet and per codec.
- Oracles: `🔮️oracles/🔣️.json` declares that drawing has no third-party oracle for its semantics (`noOracleDecision`).

## 7. Discrepancies and follow-ups

1. TS mirror `🚪️io/🟦️.ts` describes pdf, png, dwg, dxf export and svg import as stubs. The Rust sources describe them as implemented. Verify which is true before using either as the spec.
2. Subset manifest key `any` versus folder `✳️any` and `SubsetId::ANY = "*"`.
3. Drawing's `NativeCodecs` text and binary slots are `None`, so the real codecs are reached only through the `ArtifactPack` / `ArtifactDsl` trait impls.
4. Durable client-side outbox for unacked operations: not located (section 3.5).
5. `🎞️gif/.../🧬️migrations` exists in stdio, which conflicts with the no-migrations rule in AGENTS.md. Not copied.
