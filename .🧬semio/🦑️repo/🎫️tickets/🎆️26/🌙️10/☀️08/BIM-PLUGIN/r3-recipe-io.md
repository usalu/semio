# 🚪 BIM IO implementation recipe (r3)

Read-only exploration. Paths are relative to `C:\git\semio` unless absolute. Each claim cites the file it came from. Code blocks are verbatim unless marked **(proposed)**.

Inputs: `r2-design.md` §7 (IO target: pack, DSL, sqlite, `s.stdio.ifc@2x3/*` export with geometry, `s.stdio.gltf@2.0/*`, `s.stdio.svg@1.1/*` plan per storey, IFC 2x3 import), `r1-explore-io-persistence.md`, `r1-explore-aec-geometry.md` §5-6.

---

## 0. Decisions this recipe rests on

1. **io is declared once per subset** by `pub fn io() -> IoDeclaration`, built from `serializer_entry` / `deserializer_entry` (pack-native snapshot) or the `_text` twins (DSL-native snapshot). Reference: drawing subset.
2. **`Serializer::serialize` and `Deserializer::deserialize` receive only the snapshot** (and `ArchiveChildren`). They never see inferences. Derived geometry (solids, meshes, plan linework) must be recomputed inside the leaf by the same pure functions the inference leaves call. Drawing does this: its SVG leaf calls `flatten_drawing_document_to_scene_nodes(doc)` on the snapshot.
3. **The IFC 2x3 snapshot is a generic Part-21 graph with no entity whitelist.** `IFCEXTRUDEDAREASOLID` and `IFCFACETEDBREP` are accepted as-is. BIM can emit geometry without touching the stdio IFC artifact.
4. **Producing IFC from BIM:** call `encode_ifc2x3(&Ifc2x3Snapshot)` inside the BIM `Serializer` impl. Build the `Part21Document` with `Part21Builder`. Do not go through a composer.
5. **glTF:** the stdio `GltfSnapshot` + `encode_glb` gives named nodes, materials and buffers. Framework `mesh_to_glb` emits one mesh with no nodes or materials.
6. **SVG:** `SvgElement` tree → `typed_to_svg_document` → `xml_document_to_text_checked`. One `IoPayload` per serializer entry, so "plan per storey" needs a decision (§4.4).
7. **SQLite is optional at the type level** (§1.8). Repo convention (drawing, shooting) implements it.

---

## 1. Declaring io on an artifact subset (reference: `s.draw.drawing@1/*`)

### 1.1 Layout (verified)

Subset root: `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/`

```
🚪️io/🦀️.rs                      io() + DRAWING_DIALECT consts + derived_* + mod decls
🚪️io/💾️binary/📸️snapshot/🦀️.rs  impl store::ArtifactPack for DrawingSnapshot
🚪️io/📝️text/📸️snapshot/🦀️.rs    impl store::ArtifactDsl for DrawingSnapshot
🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs  impl store::ArtifactSqliteSnapshot for DrawingSnapshot (+ 🗄️.sql)
🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs
🚪️io/📥️import/🧩️deserializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs
```

Mirror this exact tree for BIM under `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/`, with format leaves `export/…/🧊️gltf/🔖️2.0/✳️any`, `export/…/🏗️ifc/🔖️2x3/✳️any`, `export/…/🎨️svg/🔖️1.1/✳️any`, `import/…/🏗️ifc/🔖️2x3/✳️any`. Use the same emoji spelling as the stdio artifact folder names (`🏗️ifc`, `🧊️gltf`, `🎨️svg`).

### 1.2 `io()` (verbatim from drawing `🚪️io/🦀️.rs` starting at line 148; comments trimmed)

```rust
pub fn io() -> semio_framework_plugin::app::declarations::IoDeclaration {
    use crate::standards::v1::subsets::any::io::export::serializers::artifacts as export;
    use crate::standards::v1::subsets::any::io::import::deserializers::artifacts as import;
    use crate::{DrawingMutation, DrawingSnapshot, DRAWING_DIALECT, DRAWING_DOCUMENT_SCHEMA};
    use semio_framework::io::io_mechanism::{deserializer_entry, serializer_entry, IoEntry};
    use semio_framework_plugin::app::declarations::{IoDeclaration, LanguagePair, NativeCodecs};
    use std::sync::OnceLock;

    fn entries() -> &'static [IoEntry] {
        static ENTRIES: OnceLock<Vec<IoEntry>> = OnceLock::new();
        ENTRIES
            .get_or_init(|| {
                vec![
                    serializer_entry::<DrawingSnapshot, export::svg::v1_1::any::DrawingIntoSvg>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::pdf::v1_4::any::DrawingIntoPdf>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::png::v1_2::any::DrawingIntoPng>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::json::v_rfc8259::any::DrawingIntoJson>(DRAWING_DIALECT),
                    deserializer_entry::<DrawingSnapshot, import::json::v_rfc8259::any::JsonIntoDraw>(DRAWING_DIALECT),
                    deserializer_entry::<DrawingSnapshot, import::svg::v1_1::any::SvgIntoDraw>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::dwg::v_ac1018::any::DrawingIntoDwg>(DRAWING_DIALECT),
                    serializer_entry::<DrawingSnapshot, export::dxf::v_r12::any::DrawingIntoDxf>(DRAWING_DIALECT),
                ]
            })
            .as_slice()
    }

    IoDeclaration {
        native: NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::bare::<DrawingSnapshot, DrawingMutation>(DRAWING_DOCUMENT_SCHEMA.to_string()),
        },
        entries: entries(),
    }
}
```

Note: `LanguagePair { text: None, binary: None }` is the documented scope-narrowing the drawing uses. The `ArtifactPack`/`ArtifactDsl` impls exist and are tested without being declared in `NativeCodecs`.

### 1.3 Subset root mounts it (verified, `🪆️subsets/✳️any/🦀️.rs`)

```rust
pub fn subset<A: crate::DrawingApplication>() -> SubsetDeclaration<A> {
    SubsetDeclaration {
        // … dialect, schema …
        io: io::io(),
        // … viewer, editor, examples …
    }
}
```

Imports: `use semio_framework_plugin::app::declarations::{editor_surface, viewer_surface, SchemaDeclaration, SubsetDeclaration};`

### 1.4 Dialect constants (verified)

Drawing: `pub const DRAWING_DIALECT` is `Dialect { artifact_kind: "s.draw.drawing", standard: StandardId("1"), subset: SubsetId::ANY }` (`🦀️.rs` ~line 490, per r1; re-check before copying). Foreign targets are declared in the leaf:

```rust
pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };
```

(drawing `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs` line 8)

Other verified foreign dialect constants:

| Target | Constant | Source |
|---|---|---|
| IFC 2x3 base | `Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") }` | `🏗️ifc/…/🧱️base/🚪️io/🦀️.rs` (`DIALECT` in `derived_composition`) |
| glTF 2.0 | `Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") }` | gis `🗿️artifacts/🏔️gisterrain/…/🚪️io/🦀️.rs` line 148 (`EXPORT_GLTF_DIALECT`) |
| SVG 1.1 | `Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY }` | drawing svg leaf |

### 1.5 Framework contract (verbatim, `🧰️framework/🔨️modules/🚪️io/🦀️.rs`, `pub mod io_mechanism`)

```rust
pub trait Serializer<S> {
    const INTO: Dialect;
    const FIDELITY: IoFidelity;
    fn serialize(from: &S, children: &ArchiveChildren) -> impl std::future::Future<Output = IoResult<IoPayload>> + Send;
}

pub trait Deserializer<S> {
    const FROM: Dialect;
    const FIDELITY: IoFidelity;
    const CONFORMANCE: Option<fn(&S) -> Vec<Diagnostic>> = None;
    fn sniff(_payload: &IoPayload) -> impl std::future::Future<Output = Confidence> + Send { std::future::ready(Confidence::None) }
    fn deserialize(payload: &IoPayload) -> impl std::future::Future<Output = IoResult<S>> + Send;
}
```

Entry constructors:

```rust
pub fn serializer_entry<S: store::ArtifactPack, T: Serializer<S>>(own: Dialect) -> IoEntry
pub fn serializer_entry_text<S: store::ArtifactDsl, T: Serializer<S>>(own: Dialect) -> IoEntry
pub fn deserializer_entry<S: store::ArtifactPack, T: Deserializer<S>>(own: Dialect) -> IoEntry
pub fn deserializer_entry_text<S: store::ArtifactDsl, T: Deserializer<S>>(own: Dialect) -> IoEntry
```

The built `IoEntry` (verbatim from the constructors):

```rust
pub struct IoEntry {
    pub from: Dialect,
    pub into: Dialect,
    pub fidelity: IoFidelity,
    pub direction: IoEntryDirection,      // Export | Import
    pub sniff: Option<fn(&IoPayload) -> Confidence>,
    pub run: fn(&IoPayload) -> IoResult<IoPayload>,
}
// serializer_entry:    IoEntry { from: own, into: T::INTO, fidelity: T::FIDELITY, direction: IoEntryDirection::Export, sniff: None, run: run::<S, T> }
// deserializer_entry:  IoEntry { from: T::FROM, into: own, fidelity: T::FIDELITY, direction: IoEntryDirection::Import, sniff: Some(deserializer_sniff::<S, T>), run: run::<S, T> }
```

`deserializer_entry` also runs `T::CONFORMANCE` after a successful deserialize and folds its diagnostics into the outcome. The pack path re-encodes the result with `outcome.value.encode_pack()`. The `_text` twin uses `print_dsl()`.

`IoFidelity` (`🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs` line 65): `Exact | Canonical | Semantic | Lossy`. Lossy needs a non-empty drops set (`IoFidelityDeclaration::validate`).

`IoOutcome::clean(payload)` wraps the value with no diagnostics (drawing leaves use it).

### 1.6 A leaf, verbatim (drawing SVG export)

```rust
pub const SVG_DIALECT: Dialect = Dialect { artifact_kind: "s.stdio.svg", standard: StandardId("1.1"), subset: SubsetId::ANY };

pub struct DrawingIntoSvg;

impl Serializer<DrawingSnapshot> for DrawingIntoSvg {
    const INTO: Dialect = SVG_DIALECT;
    const FIDELITY: IoFidelity = IoFidelity::Lossy;
    async fn serialize(from: &DrawingSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> {
        let (svg_text, _width, _height) = drawing_document_to_svg(from).map_err(|message| IoError::from_value_error(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, format!("DrawingIntoSvg: {message}"))))?;
        Ok(IoOutcome::clean(IoPayload::Text(svg_text)))
    }
}
```

Import leaf (verbatim, drawing `📥️…/🎨️svg/🔖️1.1/✳️any/🦀️.rs`):

```rust
pub struct SvgIntoDraw;
impl Deserializer<DrawingSnapshot> for SvgIntoDraw {
    const FROM:Dialect=SVG_DIALECT;
    const FIDELITY:IoFidelity=IoFidelity::Semantic;
    async fn deserialize(payload:&IoPayload)->IoResult<DrawingSnapshot> {
        let source=match payload {IoPayload::Text(text)=>text.as_str(),IoPayload::Binary(bytes)=>std::str::from_utf8(bytes).map_err(|value|error(value.to_string()))?};
        let mut job=document::SvgImportJob::new(source,&crate::standards::v1::subsets::any::schema::create_drawing_id("svg",source.as_bytes())).map_err(error)?;
        while !job.step(64).map_err(error)?.done {}
        job.take().map(IoOutcome::clean).map_err(error)
    }
}
```

The import is incremental (`SvgImportJob::step(64)`), which is how drawing meets the progress and cancellation rule. BIM IFC import on a large file should use the same job pattern.

### 1.7 Older `ComposerEntry` pattern (still in use in stdio, shooting, gis)

Stdio IFC, gis and shooting still register through `ComposerEntry`. Framework definition (`🧰️framework/🔨️modules/🚪️io/🦀️.rs` ~line 866):

```rust
pub struct ComposerEntry {
    pub writes: Dialect,
    pub reads: &'static [Dialect],
    pub compose: AsyncComposeFn,
}
```

Shooting export entries (verbatim, `🚪️io/🦀️.rs` lines 120-129):

```rust
pub fn entries() -> &'static [ComposerEntry] {
    ENTRIES
        .get_or_init(|| {
            vec![
                composer_entry_of::<ShootingAnyComposer>(),
                ComposerEntry { writes: EXPORT_JSON_DIALECT, reads: &[SHOOTING_DIALECT], compose: compose_export_json },
            ]
        })
        .as_slice()
}
```

Import composition (verbatim trait shape, stdio IFC base `🚪️io/🦀️.rs` `derived_composition`):

```rust
impl ArtifactComposition for Ifc2x3ComposerComposition {
    type Snapshot = Ifc2x3Snapshot;
    const WRITES: Dialect = DIALECT;
    fn reads() -> &'static [Dialect] { &[DIALECT, DEP_TXT] }
    fn compose(sources: &[ComposeSource<'_>]) -> Result<Composition<Self::Snapshot>, ComposeError> { /* analyze → snapshot */ }
}
```

`composer_entry_of::<C: ArtifactComposer>()` requires `C::Snapshot: ArtifactPack` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` line 1043).

**Recommendation:** use `io()` (drawing style) for BIM. Use `ComposerEntry` only if BIM must also expose a foreign-to-native compose route, as the shooting and stdio artifacts do.

### 1.8 Pack (binary), DSL text and SQLite

**Snapshot type bounds (verbatim, `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` line 39120):**

```rust
type Snapshot: Clone + PartialEq + semio_framework_value::ToValue + semio_framework_value::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + semio_framework_schema_composition::ArtifactCompositionFields + 'static;
```

So **DSL text and pack are mandatory** for an `ArtifactEditor::Snapshot`. `ArtifactSqliteSnapshot` is **not** in that bound.

Pack (drawing `🚪️io/💾️binary/📸️snapshot/🦀️.rs` line 16):

```rust
impl store::ArtifactPack for DrawingSnapshot {
    // encode_pack_with(&self, &store::PackEncodeOptions) -> Result<Vec<u8>, PackError>
    // decode_pack_with(bytes: &[u8], &store::PackDecodeOptions) -> Result<Self, PackError>
}
```

Pack framing (r1): `💾️binary/📸️snapshot/📡️.protocol.semio`, magic `0x8953f83f7d340d0a`, 32-byte header, 64-byte footer with `body_crc32`.

DSL text (drawing `🧬️schema/📸️snapshot/🦀️.rs` lines 16-17):

```rust
#[dsl(id = "drawing.drawing", layout = "lines")]
#[artifact_schema(id = "s.draw.drawing")]
pub struct DrawingSnapshot { /* … #[dsl(statements, block)] … */ }
```

Shooting (`🎥️shooting/…/🧬️schema/📸️snapshot/🦀️.rs` lines 7-10):

```rust
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, ArtifactSchema, semio_framework_dsl_record_derive::DslRecord)]
#[value(rename_all="camelCase")]
#[artifact_schema(id="s.shooting.shooting")]
#[dsl(extension="shooting")]
#[dsl(layout="lines")]
pub struct ShootingSnapshot{ /* #[dsl(table)] assets, #[dsl(block)] scene, … */ }
```

Hand-written `ArtifactDsl`/`ArtifactPack` impls are the repo pattern (`HandcraftedArtifactPack`, `HandcraftedArtifactDsl` regions). The derive does not emit them.

**SQLite (optional):** The store trait `ArtifactSqliteSnapshot` (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs` ~line 11880-11930) has default bodies that return `ValueRefusalKind::UnsupportedOwner`. Condensed excerpt (signature shortened, message verbatim):

```rust
fn encode_sqlite_snapshot_native(&self, /* encoding, */ control: &mut SqliteSnapshotControl<'_>) -> Result<IoPayload, ValueError> {
    Err(ValueError::new(ValueRefusalKind::UnsupportedOwner, "snapshot owner has no controlled native encoding implementation"))
}
```

`sqlite_codec()` builds the relational codec "only when the snapshot actually implements it". The plugin resolves `plugin_snapshot_sqlite_schema(dialect)` through `native_snapshot_sqlite_schema`, which errors when none is registered (`🛍️os/…/🔌️plugin/🦀️.rs` ~line 44526).

Drawing's implementation (`🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` line 76-77, 79):

```rust
impl store::ArtifactSqliteSnapshot for DrawingSnapshot {
    const SQLITE_SCHEMA: &'static str = SCHEMA;   // include_str!("🗄️.sql")
    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, ValueError> { project(self, control) }
    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> { reconstruct(database, control) }
    fn retire_sqlite_snapshot(self) { retire_decoded_drawing_snapshot(self) }
}
```

**Answer to "mandatory for a subset?":** not enforced by the framework. Pack and DSL are enforced by the `Snapshot` bound. SQLite is opt-in, and both drawing and shooting implement it by convention (`🪶️sqlite` folder). For BIM, follow the convention: implement `ArtifactSqliteSnapshot` with `SQLITE_SCHEMA` (`🗄️.sql`), `to_sqlite_database` and `from_sqlite_database` using `SqliteSnapshotControl` checkpoints for progress and cancellation. Use `🗄️.d.ts` as in shooting for the TS mirror.

### 1.9 Export media (from `📜️artifact-definition.json` of the stdio artifacts; verified)

| Format | mimes | extensions |
|---|---|---|
| glTF | `model/gltf+json` (json), `model/gltf-binary` (binary) | `.gltf`, `.glb` |
| SVG | `image/svg+xml` | `.svg` |
| IFC | `application/x-ifc` (`is_binary: true` in the definition) | `.ifc` |

Note: the IFC format descriptor says `is_binary: true`, but the 2x3 txt leaf produces text. Decide the BIM IFC payload kind (see §6.3).

Standard-level media for BIM follows drawing's `standard()` (`🏅️standards/🔖️1/🦀️.rs`): `MediaDeclaration { mimes: &["application/vnd.semio.<…>+json"], extensions: &["<…>"] }`.

---

## 2. IFC (`s.stdio.ifc`) — document model, writer, reader, and how BIM produces it

### 2.1 Identity and standards (verified)

Root `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🦀️.rs`:

- `pub const STDIO_IFC_DOCUMENT_SCHEMA: &str = "stdio.ifc";` (standard 4)
- `pub const IFC_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc";`
- Standards: `pub mod standards { pub mod v4 {…} pub mod v2x3 {…} }` (`🏅️standards/4️⃣4`, `🏅️standards/🔖️2x3`)
- 2x3 subsets: `🧱️base`, `🤝️cv20`, `🧮️sav`, `🏢️cobie`. `🧬️mvd` holds shared MVD mechanics.

Declaration (verbatim, root `declaration()`):

```rust
let builder = semio_framework_plugin::ArtifactDeclaration::builder(definition)
    .schema(standards::v4::subsets::any::schema::ifc_artifact_schema_descriptor())
    .schemas([standards::v2x3::subsets::base::schema::ifc2x3_artifact_schema_descriptor()])
    .formats(formats()?)
    .inferences([/* v4 + v2x3 inference descriptors */])
    .composers(standards::v4::engine::io_registry::entries())
    .composers(standards::v2x3::engine::io_registry::entries())
    .document_codec_bare::<standards::v2x3::subsets::base::schema::snapshot::Ifc2x3Snapshot, standards::v2x3::subsets::base::schema::mutations::Ifc2x3Mutation>(standards::v2x3::subsets::base::schema::snapshot::STDIO_IFC2X3_DOCUMENT_SCHEMA, semio_framework_artifact_reference::Dialect { artifact_kind: "s.stdio.ifc", standard: semio_framework_artifact_reference::StandardId("2x3"), subset: semio_framework_artifact_reference::SubsetId("*") });
```

### 2.2 Snapshot and document model (verified)

File `🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs`:

```rust
pub const STDIO_IFC2X3_DOCUMENT_SCHEMA: &str = "stdio.ifc.2x3";
pub const IFC2X3_ARTIFACT_SCHEMA_ID: &str = "s.stdio.ifc.2x3";

pub struct Ifc2x3Snapshot {
    pub schema: String,                                  // = "stdio.ifc.2x3"
    pub document: Part21Document,                        // the whole Part-21 graph
    pub edm_preamble: Option<Ifc2x3EdmPreamble>,         // EDM StepFileFactory banner (16 fields)
}
```

`validate_ifc2x3_snapshot` (verified) requires:

- `snapshot.schema == "stdio.ifc.2x3"`
- `FILE_SCHEMA` contains the string `IFC2X3`, checked as `value.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some("IFC2X3")))`
- instance ids unique
- every instance has at least one entity

The 2x3 mutation vocabulary is generic (`🧬️mutations/🦀️.rs`):

```rust
pub enum Ifc2x3Mutation {
    SetSnapshot(set_snapshot::SetSnapshot),
    PatchSnapshot(patch_snapshot::PatchSnapshot),
    UpsertInstance(upsert_instance::UpsertInstance),
    RemoveInstance(remove_instance::RemoveInstance),
    SetHeader(set_header::SetHeader),
}
```

The 4 standard's `IfcMutation` (about 280 references, `add_product`, `add_space`, …) is typed. The 2x3 base is not.

### 2.3 Can the IFC document hold arbitrary entities? Yes.

Evidence: the Part-21 instance type stores type name and argument list as strings and values, and nothing checks EXPRESS membership.

```rust
pub struct Part21Instance {
    pub id: u64,
    pub entities: Vec<(String, Vec<Part21Value>)>,   // complex instances: every (type, args) kept
}
pub enum Part21Value {
    Ref(u64), Str(String), Enum(String), Int(i64), Real(Part21Decimal), List(Vec<Part21Value>),
    Typed { name: String, items: Vec<Part21Value> },   // IFCLENGTHMEASURE(3000.)
    Unset, Derived,
}
```

A grep of `🧬️schema` for `IFCWALL…`, `IFCSLAB`, `IFCBUILDINGSTOREY`, `IFCSPACE` finds only the `🔺️diff` unit-test fixtures and one `IFCBUILDINGSTOREY` in `🔺️diff/🦀️.rs` line 312. No whitelist. The upper-case stored name is matched case-insensitively by `Part21Instance::is_type` (`eq_ignore_ascii_case`).

Entity-name examples that need no code change: `IFCEXTRUDEDAREASOLID`, `IFCSHAPEREPRESENTATION`, `IFCPRODUCTDEFINITIONSHAPE`, `IFCLOCALPLACEMENT`, `IFCFACETEDBREP`, `IFCPROPERTYSET`, `IFCPROPERTYSINGLEVALUE`, `IFCELEMENTQUANTITY`, `IFCQUANTITYLENGTH`, `IFCRELAGGREGATES`, `IFCRELCONTAINEDINSPATIALSTRUCTURE`, `IFCRELVOIDSELEMENT`, `IFCRELFILLSELEMENT`, `IFCRELDEFINESBYPROPERTIES`, `IFCRELASSOCIATESMATERIAL`.

### 2.4 Part-21 codec API (verbatim signatures, `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/📐️part21/🦀️.rs`)

Re-exported by the IFC crate: `pub use semio_s_artifact_stdio_contract::part21;` (ifc root). BIM should import `semio_s_artifact_stdio_ifc::part21::…` and not depend on the contract crate directly.

```rust
pub struct Part21Header { pub file_description: Vec<Part21Value>, pub file_name: Vec<Part21Value>, pub file_schema: Vec<Part21Value> }
pub struct Part21Document { pub header: Part21Header, pub instances: Vec<Part21Instance> }
impl Part21Header { pub fn iso_10303_21_minimum() -> Self }       // all three records, every LIST[1:?] populated
impl Part21Document {
    pub fn instance(&self, id: u64) -> Option<&Part21Instance>
    pub fn resolve(&self, value: &Part21Value) -> Option<&Part21Instance>
    pub fn by_type<'a>(&'a self, type_name: &'a str) -> impl Iterator<Item = &'a Part21Instance> + 'a
    pub fn next_id(&self) -> u64
}
pub struct Part21Builder { pub instances: Vec<Part21Instance>, next_id: u64 }   // next_id starts at 1
impl Part21Builder {
    pub fn new() -> Self
    pub fn alloc(&mut self, type_name: &str, args: Vec<Part21Value>) -> u64
    pub fn build(self, header: Part21Header) -> Part21Document
}
pub fn parse_part21(text: &str) -> Result<Part21Document, Part21Error>
pub fn write_part21(doc: &Part21Document) -> String
pub fn write_part21_with<P: Part21Preamble>(doc: &Part21Document, options: Part21WriteOptions, preamble: Option<&P>) -> String
pub struct Part21WriteOptions { pub line_ending: &'static str, pub blank_after_header: bool, pub blank_before_data: bool, pub blank_before_terminator: bool, pub space_after_instance_equals: bool }
pub trait Part21Preamble { fn write_preamble(&self, out: &mut String, line_ending: &str); }
```

`Part21Decimal::from_f64(value: f64) -> Part21Decimal` exists (line 49) and `to_f64` (line 56).

Header check (`FILE_SCHEMA` attribute is `LIST[1:?] OF STRING`): the 2x3 validator needs `Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])` in `file_schema`.

### 2.5 The 2x3 writer and reader (verbatim, `🧱️base/🚪️io/🦀️.rs`)

```rust
pub const IFC2X3_SCHEMA_NAME: &str = "IFC2X3";

pub fn decode_ifc2x3(bytes: &[u8]) -> Result<Ifc2x3Snapshot, String> {
    let text = std::str::from_utf8(bytes).map_err(|e| format!("ifc2x3: not valid utf-8: {e}"))?;
    let document = parse_part21(text).map_err(|e| format!("ifc2x3 parse: {e}"))?;
    let declares_ifc2x3 = document.header.file_schema.iter().any(|v| v.as_list().is_some_and(|items| items.iter().any(|item| item.as_str() == Some(IFC2X3_SCHEMA_NAME))));
    if !declares_ifc2x3 { return Err(format!("ifc2x3: FILE_SCHEMA does not declare {IFC2X3_SCHEMA_NAME}")); }
    Ok(Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: parse_edm_preamble(text) })
}

pub fn encode_ifc2x3(snapshot: &Ifc2x3Snapshot) -> Result<Vec<u8>, String> {
    crate::standards::v2x3::subsets::base::schema::snapshot::validate_ifc2x3_snapshot(snapshot)?;
    let options = Part21WriteOptions { line_ending: "\r\n", blank_after_header: snapshot.edm_preamble.is_some(), blank_before_data: true, blank_before_terminator: true, space_after_instance_equals: true };
    Ok(write_part21_with(&snapshot.document, options, snapshot.edm_preamble.as_ref()).into_bytes())
}
```

Public path from outside the crate: `semio_s_artifact_stdio_ifc::standards::v2x3::engine::{decode_ifc2x3, encode_ifc2x3}` (the `engine` barrel re-exports `subsets::base::io::*`; `v2x3` is `pub`). The stdio txt leaf calls exactly this path: `crate::standards::v2x3::engine::encode_ifc2x3(from)`.

Snapshot path: `semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3Snapshot, Ifc2x3EdmPreamble, STDIO_IFC2X3_DOCUMENT_SCHEMA}`.

### 2.6 How BIM produces an IFC document (proposed, uses the verified API above)

Route: BIM `Serializer<ModelSnapshot>` → `Part21Builder` → `Part21Document` → `encode_ifc2x3` → payload. Do not write Part-21 text by hand, and do not invent a second writer. `encode_ifc2x3` owns the CRLF layout and validation.

**(proposed)**

```rust
use semio_s_artifact_stdio_ifc::part21::{Part21Builder, Part21Header, Part21Value};
use semio_s_artifact_stdio_ifc::standards::v2x3::engine::encode_ifc2x3;
use semio_s_artifact_stdio_ifc::standards::v2x3::subsets::base::schema::snapshot::{Ifc2x3Snapshot, STDIO_IFC2X3_DOCUMENT_SCHEMA};

fn bim_to_ifc2x3(model: &ModelSnapshot, derived: &DerivedModel) -> Result<Vec<u8>, String> {
    let mut b = Part21Builder::new();
    let project = b.alloc("IFCPROJECT", vec![/* GlobalId, $, Name, … */]);
    // one alloc per entity; reference by Part21Value::Ref(id); geometry from `derived`, not from the snapshot
    let mut header = Part21Header::iso_10303_21_minimum();
    header.file_schema = vec![Part21Value::List(vec![Part21Value::Str("IFC2X3".into())])];
    let document = b.build(header);
    encode_ifc2x3(&Ifc2x3Snapshot { schema: STDIO_IFC2X3_DOCUMENT_SCHEMA.into(), document, edm_preamble: None })
}
```

Then in the leaf: `IoPayload::Binary(bytes)` (or `Text(String::from_utf8(bytes)?)`, §6.3).

Pitfalls:

- `Part21Builder::alloc` gives sequential ids from 1. Forward references need the id before the referencing entity is created, so allocate in dependency order or reserve ids explicitly.
- `validate_ifc2x3_snapshot` rejects duplicate ids and empty entity lists.
- `IFCLENGTHMEASURE(3000.)` is `Part21Value::Typed { name: "IFCLENGTHMEASURE".into(), items: vec![Part21Value::Real(Part21Decimal::from_f64(3000.0))] }`.

### 2.7 IFC import (proposed, verified inputs)

`deserializer_entry_text::<ModelSnapshot, IfcIntoBim>(BIM_DIALECT)` with `const FROM: Dialect = Dialect { artifact_kind: "s.stdio.ifc", standard: StandardId("2x3"), subset: SubsetId("*") };`. The leaf calls `decode_ifc2x3(bytes)`, then walks `document.by_type("IFCWALLSTANDARDCASE")`, `"IFCOPENINGELEMENT"`, `"IFCSLAB"`, `"IFCCOLUMN"`, `"IFCBUILDINGSTOREY"`, `"IFCRELAGGREGATES"` and `"IFCRELCONTAINEDINSPATIALSTRUCTURE"`. `Part21Document::resolve(&Part21Value)` follows references. Use `CONFORMANCE` for the "all referenced ids exist" check. Large files need a job like `SvgImportJob` (§1.6).

### 2.8 Crates and dependency lines (verified)

Package: `semio-s-artifact-stdio-ifc` at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust/Cargo.toml`. Its `[lib]` is `path = "../../🦀️.rs"`. Its `[package]` header uses `workspace = "../../../../../.."`. Its dependencies start with `semio-framework-artifact-reference = { workspace = true }`, `semio-framework-dsl-record = { path = "../../../../../../../🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust" }`, and `pack = { workspace = true }`.

BIM dependency line, as the drawing crate writes its stdio deps (drawing uses `{ workspace = true }`, but the root `Cargo.toml` `[workspace.dependencies]` (line 238-341) has no `semio-s-artifact-stdio-*` entries, so use an explicit path):

```toml
semio-s-artifact-stdio-ifc  = { path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📦️packages/🦀️rust" }
semio-s-artifact-stdio-gltf = { path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧊️gltf/📦️packages/🦀️rust" }
semio-s-artifact-stdio-svg  = { path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎨️svg/📦️packages/🦀️rust" }
semio-s-artifact-stdio-xml  = { path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📰️xml/📦️packages/🦀️rust" }
semio-s-artifact-stdio-semio = { path = "../../../../../../../✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/📦️packages/🦀️rust" }   # only if BIM exports via s.stdio.semio mesh
```

Count of `../` = 7 from the BIM package dir `✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model/📦️packages/🦀️rust` to the repo root. The same depth is used by the stdio crates. The `semio-s-artifact-stdio-xml` crate path should be checked (it is `📰️xml`, not `📰markup`). Confirm the xml folder name before writing it.

The contract crate `semio-s-artifact-stdio-contract` (`📇️registry/🧬️contract/📦️packages/🦀️rust`) is the owner of `part21`. Depend on it only if BIM must bypass the IFC re-export.

---

## 3. glTF 2.0 (`s.stdio.gltf@2.0/*`) — document model, GLB, and mesh exporters

### 3.1 Document model (verified, `🏅️standards/🔖️2.0/🪆️subsets/♾️any/🧬️schema/📸️snapshot/🦀️.rs`)

```rust
pub struct GltfSnapshot {
    pub schema: String,              // STDIO_GLTF_DOCUMENT_SCHEMA
    pub document: GltfDocument,      // the JSON chunk
    pub buffers: Vec<Vec<u8>>,       // binary payloads, parallel to document.buffers
    pub source_form: GltfSourceForm, // Json | Glb (how it was read)
}
pub struct GltfDocument { pub asset: GltfAsset, pub scene: Option<usize>, pub scenes: Vec<GltfScene>, pub nodes: Vec<GltfNode>, pub meshes: Vec<GltfMesh>, pub accessors: Vec<GltfAccessor>, /* bufferViews, buffers, materials, textures, images, samplers, skins, animations, cameras … */ }
```

Named types exist for every glTF object (`GltfNode`, `GltfMesh`, `GltfPrimitive`, `GltfAccessor`, `GltfBufferView`, `GltfBuffer`, `GltfMaterial`, `GltfPbrMetallicRoughness`, `GltfCamera`, …).

Byte-level functions (verified, `🏅️standards/🔖️2.0/🪆️subsets/♾️any/🚪️io/🦀️.rs`), module path `semio_s_artifact_stdio_gltf::standards::v2_0::subsets::any::io`:

```rust
pub fn parse_gltf_document(bytes: &[u8]) -> Result<GltfSnapshot, String>
pub fn serialize_gltf_document(snapshot: &GltfSnapshot) -> Vec<u8>          // .gltf: embeds buffers as data URIs
pub fn encode_glb(snapshot: &GltfSnapshot) -> Result<Vec<u8>, String>      // .glb: first buffer as BIN chunk
pub fn decode_glb(bytes: &[u8]) -> Result<GltfSnapshot, String>
```

`encode_glb` calls `validate_document`, then embeds the first buffer as the BIN chunk when it has no URI, and keeps `byteLength` truthful. Use it for BIM element solids.

### 3.2 How gis exports glTF today (verbatim, `🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🦀️.rs` lines 149-155)

```rust
fn compose_export_gltf(sources: &[ErasedComposeSource]) -> semio_framework_plugin::ComposeFuture<'_> {
    Box::pin(async move {
        let snapshot = rebuild_native_snapshot(sources)?;
        let bytes = crate::standards::v1::subsets::any::io::export::serializers::artifacts::gltf::v2_0::any::serialize_bytes(&snapshot).map_err(/* … */)?;
        Ok(ComposedArtifact { dialect: EXPORT_GLTF_DIALECT, payload: IoPayload::Binary(bytes), diagnostics: Vec::new(), confidence: IoConfidenc/* … */ })
    })
}
```

Its leaf (`📤️export/🧵️serializers/🗿️artifacts/🧊️gltf/🔖️2.0/✳️any/🦀️.rs`, verbatim):

```rust
//! 🏔️ gisterrain → gltf — … written by `s.stdio.semio/v1/mesh`'s own export leaf.
//! 🔖 `IoFidelity::Lossy`: the file carries the surface geometry, not the terrain document …
use semio_s_artifact_stdio_semio::standards::v1::subsets::mesh::io::{encode_mesh, SemioMeshFormat};

pub fn serialize_bytes(snapshot: &GisTerrainSnapshot) -> Result<Vec<u8>, semio_framework_diagnostic::TextError> {
    encode_mesh(&gis_terrain_mesh_from_snapshot(snapshot), SemioMeshFormat::Gltf).map_err(/* … */)
}
```

`encode_mesh` (`🧿️semio/…/🔺️mesh/🚪️io/🦀️.rs` line 241-252) dispatches `SemioMeshFormat::Gltf => …serialize_gltf_…` (JSON with data URI). So gis goes **mesh → s.stdio.semio mesh → gltf JSON**, not through `GltfSnapshot` nodes.

### 3.3 Framework `mesh_to_glb` (verified, `🧰️framework/🔨️modules/🏗️mesh-engine/🦀️.rs` line 971)

```rust
pub fn mesh_to_glb(mesh: &MeshData) -> Vec<u8>
pub struct MeshData { pub positions: Vec<f32>, pub normals: Vec<f32>, pub colors: Vec<f32>, pub indices: Vec<u32>, pub uvs: Vec<f32>, pub face_ids: Vec<u32>, pub vertex_ids: Vec<u32>, /* … */ }
```

It writes one mesh (one primitive, mode 4), one node, one scene, accessors POSITION(VEC3, min/max), NORMAL(VEC3), indices(SCALAR u32). It computes normals when the lengths differ. There are no materials, no node hierarchy and no names. Use it only for a quick single-mesh viewer export.

### 3.4 Recommendation for BIM glTF (proposed)

One glTF node per element (named by element id, `mesh` = element solid, `extras` or `name` = id), one material per BIM material, buffers as a single `Vec<u8>` accessors/bufferViews per primitive. Build `GltfSnapshot`, then `encode_glb`. Use `mesh_to_glb` only for a fast preview.

Leaf:

```rust
pub struct ModelIntoGlb;
impl Serializer<ModelSnapshot> for ModelIntoGlb {
    const INTO: Dialect = GLTF_DIALECT;     // Dialect { artifact_kind: "s.stdio.gltf", standard: StandardId("2.0"), subset: SubsetId("*") }
    const FIDELITY: IoFidelity = IoFidelity::Lossy;   // drops: BIM psets, quantities, layers (no glTF home)
    async fn serialize(from: &ModelSnapshot, _: &ArchiveChildren) -> IoResult<IoPayload> { /* derive meshes → GltfSnapshot → encode_glb */ }
}
```

Lossy requires a non-empty drops list (`IoFidelityDeclaration::validate`).

### 3.5 Tessellation source (r1)

`semio-framework-3d` `Brep` `*_sync` APIs and the adaptive CDT tessellation (`🧰️framework/🔨️modules/🧊️3d/📐️brep/💡️queries/🧩tessellation/🦀️.rs`) yield `MeshTransfer` / `MeshData`. This is the route r1 §6 identifies. Extrusions and openings come from `🧊️3d` booleans, so solids reach glTF through `MeshData`, and IFC gets them as `IFCEXTRUDEDAREASOLID` or `IFCFACETEDBREP`, which BIM writes.

---

## 4. SVG (`s.stdio.svg@1.1/*`) — drawing writer path

### 4.1 Writer chain (verified, drawing `🚪️io/📤️export/🧵️serializers/🗿️artifacts/🎨️svg/🔖️1.1/✳️any/🦀️.rs`)

1. `drawing_document_to_svg(doc)` → `flatten_drawing_document_to_scene_nodes(doc)` → `drawing_scene_to_svg(nodes, view_box)`.
2. Root assembly (verbatim, drawing leaf lines 157-160):

```rust
let common = CommonAttrs { extra_attrs: vec![attr("version","1.1"),attr("xmlns:xlink","http://www.w3.org/1999/xlink")], ..Default::default() };
let root = SvgElement::Svg { common, view_box: Some(ViewBox { min_x:x, min_y:y, width, height }), width: Some(width.to_string()), height: Some(height.to_string()), xmlns: Some("http://www.w3.org/2000/svg".into()), children };
xml_document_to_text_checked(&typed_to_svg_document(&root, None))
```

3. Imports (verbatim):

```rust
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::io::text::snapshot::typed_to_svg_document;
use semio_s_artifact_stdio_svg::standards::v1_1::subsets::base::schema::snapshot::{CommonAttrs, PathCommand, SvgElement, TransformOp, ViewBox};
use semio_s_artifact_stdio_xml::standards::v1_0::subsets::base::io::text::snapshot::xml_document_to_text_checked;
```

`typed_to_svg_document(root: &SvgElement, doctype: Option<XmlDoctype>) -> XmlDocument` (`🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🚪️io/📝️text/📸️snapshot/🧮️attributes/🦀️.rs` line 430). `xml_document_to_text_checked(doc: &XmlDocument) -> Result<String, String>` (`📰️xml/🏅️standards/🔖️1.0/…/📝️text/📸️snapshot/🦀️.rs` line 562).

### 4.2 SvgElement variants and attributes (verified, `🏅️standards/🔖️1.1/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs`)

- Variants: `Svg { common, view_box, width, height, xmlns, children }`, `Rect { common, x, y, width, height, rx, ry }`, `Circle { common, cx, cy, r }`, `Ellipse`, `Line`, `Polyline`, `Polygon`, `Path { common, d: Vec<PathCommand> }`, `Group { common, children }`, `Text`, `Tspan`, `Defs { common, children }`, `LinearGradient`, `RadialGradient`, `Stop`, `Use`, `Unknown { name, attrs, children }`, plus text, CDATA, comment and PI nodes.
- `CommonAttrs { id, class, transform: Option<Vec<TransformOp>>, presentation: PresentationAttrs, extra_attrs: Vec<XmlAttr> }`. Use `CommonAttrs::default().with_fill("none").with_stroke("none")` for presentation.
- `ViewBox { min_x, min_y, width, height }`.

### 4.3 BIM plan linework (proposed)

Walls and openings from the derived footprints become `SvgElement::Path { common, d }` inside `Group`s with `extra_attrs: [("id", "storey-<id>")]`. Text labels become `SvgElement::Text`. Build one `Svg` root with `ViewBox`. Use the drawing leaf's root code as the template.

### 4.4 Decision: "plan per storey" versus one payload

`Serializer::serialize` returns one `IoPayload`, and the `IoEntry` is keyed by `(from, into)`. Nothing passes a storey selector. Two entries with the same `INTO` are a registry conflict (`same_io_entry` compares `from`, `into`, `fidelity`, `direction`, sniff and `run`), so one `s.stdio.svg@1.1/*` entry exports one SVG. Options:

- **A (recommended):** one SVG with every storey as a `Group` (`id="storey-<id>"`), laid out side by side or stacked, with a `ViewBox` per storey page. One payload per export.
- **B:** per-storey export as a separate command or child artifact (plan document per storey), not via io.
- **C:** a multi-file export, which the io mechanism does not support today.

Choose A for io and add B as a command later if needed.

---

## 5. Oracle evidence for exports in stdio

### 5.1 IFC (ifcopenshell 0.8.4.post1, Python) — mutations only

Existing oracle: `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/🏅️standards/🔖️2x3/🪆️subsets/🧱️base/🧪️tests/🔺️differential-ifc-2x3/🐍️.py` (also `🥒️.feature`, `🦀️.rs`). It applies each `Ifc2x3Mutation` to the real 3 464-entity building model with `ifcopenshell`, reserializes with `ifcopenshell.file.to_string`, and compares with the subject's own writer.

Quote (python `🐍️.py` lines 484-501, 683-691):

```python
def open_model(path: str):
    """📥️ `ifcopenshell.open` plus the guard a real measurement made necessary. …"""
    …
    model = ifcopenshell.open(path)
    …

def mutate(ctx: Context) -> Outcome:
    """🔮️ IfcOpenShell applies the named mutation and re-serializes; the from-scratch reader
    projects its own written bytes. …"""
    path = mutable_input(ctx)
    spec = spec_of(ctx)
    baseline = project(rewrite(path).decode("utf-8"))
    produced = apply_mutation(path, spec)
    projection = project(produced.decode("utf-8"))
```

Rust subject side (`🦀️.rs` lines 78-84):

```rust
pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
    let input = mutable_input(ctx)?;
    let operation = operation_of(&ctx.doc_json()?)?;
    outcome(encoded(&input, &applied(decode_ifc2x3(&input)?, &[operation])?)?)
}
```

Gap: there is no IFC **export-from-geometry** oracle. BIM needs a new case, for example `🪆️subsets/🧱️base/🧪️tests/🏗️export-bim-geometry-ifc-2x3/`, that reads BIM's IFC with `ifcopenshell.open`, checks entity counts and placements, and runs `ifcopenshell.geom` on the solids. `ifcopenshell` is pinned in root `pyproject.toml` (test group) but is not installed in `.venv` (r1 §5.4). Install before the test can run.

### 5.2 glTF — three.js 0.182.0 reader oracle, and the `gltf` Rust crate

Reader oracle (`🏅️standards/🔖️2.0/🪆️subsets/♾️any/🔮️oracles/🔣️.json`): `id: "three-gltf-2-0-mutate-reader"`, `kind: "third-party-library"`, `package: "three"`, `version: "0.182.0"`. Its TS adapter (`🧪️tests/🧊️mutate-gltf-2-0/🟦️.ts`):

```ts
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    mutate: { oracle: (ctx) => committedArtifact(ctx, "➡️after.glb", "expected-gltf", "model/gltf-binary") },
    inverse: { oracle: (ctx) => committedArtifact(ctx, "🧊️.glb", "expected-gltf", "model/gltf-binary") },
    "identity-round-trip": { oracle: (ctx) => committedArtifact(ctx, "🧊️.glb", "expected-gltf", "model/gltf-binary") },
  },
});
```

Three's `GLTFLoader` reads the committed `expected-gltf` and the subject's `actual-gltf` (`gltf-2-0-three-compare-v1`). The generator `🏭️generator/📜️script.ts` builds the base document with `three` `GLTFExporter` (line 10, 40).

Rust-side differential against the `gltf` crate (`🧰️framework/🔨️modules/🏗️mesh-engine/🧪️tests/🔬️gltf-oracle-differential/🦀️.rs`, `gltf = { version = "1.4.1", default-features = false, features = ["utils"] }`):

```rust
#[test]
fn differential_generated_uv_sphere_glb_matches_gltf_crate_oracle() {
    let mesh = mesh_uv_sphere(1.0, 8, 6);
    let bytes = mesh_to_glb(&mesh);
    let ours = mesh_from_glb(&bytes).expect("first-party decode");
    let oracle = oracle_mesh_from_glb(&bytes).expect("oracle decode");
    assert_structurally_equal(&ours, &oracle);
}
```

For BIM, add the same shape of test: build a `GltfSnapshot` → `encode_glb` → `gltf` crate (Rust) and three's `GLTFLoader` (TS) read it back and compare the node tree and accessors.

### 5.3 SVG

`semio-s-artifact-stdio-svg-test-oracle` (`🎨️svg/🔮️oracles/📦️packages/🦀️rust/Cargo.toml`) is the test-oracle crate. r1 §1 says there is no third-party oracle for drawing semantics (`🔮️oracles/🔣️.json`). For BIM plan SVG, the oracle is a parse back through `parse_svg_xml` (drawing import path) and a geometry comparison, not a third-party tool.

---

## 6. Recommended BIM io set and concrete order of work

### 6.1 Entries (proposed, mirror drawing's `io()`)

```rust
vec![
    serializer_entry::<ModelSnapshot, export::ifc::v2x3::any::ModelIntoIfc2x3>(BIM_DIALECT),   // Lossy
    serializer_entry::<ModelSnapshot, export::gltf::v2_0::any::ModelIntoGlb>(BIM_DIALECT),     // Lossy
    serializer_entry::<ModelSnapshot, export::svg::v1_1::any::ModelIntoSvg>(BIM_DIALECT),      // Lossy
    deserializer_entry::<ModelSnapshot, import::ifc::v2x3::any::IfcIntoModel>(BIM_DIALECT),    // Semantic
]
```

`serializer_entry` (pack) is used by drawing, so use it here as well. `ModelSnapshot` must implement `ArtifactPack` (§1.8).

### 6.2 Order of work

1. Snapshot with `#[dsl(id = "bim.model", layout = "lines")]` and `#[artifact_schema(id = "s.bim.model")]`. Hand-write `impl store::ArtifactPack` and `impl store::ArtifactDsl`.
2. Pure derivation functions for solids and plan linework, callable without a store (§0 decision 2).
3. IFC 2x3 export leaf via `Part21Builder` + `encode_ifc2x3` (§2.6). Test against `ifcopenshell` (§5.1).
4. glTF export leaf via `GltfSnapshot` + `encode_glb` (§3.4). Test with the `gltf` crate and three.js (§5.2).
5. SVG plan leaf, option A (§4.4).
6. IFC import leaf via `decode_ifc2x3` and `by_type` (§2.7). Use a progress job for large files.
7. SQLite projection (`🪶️sqlite`) following drawing (§1.8).

### 6.3 Decisions to take

- **IFC payload kind:** `IoPayload::Binary(bytes)` (matches the format descriptor's `is_binary: true`) or `IoPayload::Text(String::from_utf8(bytes))` (matches the stdio txt leaf). The recipe recommends Binary.
- **Fidelity:** Lossy for all three exports, with explicit drops lists. Semantic for IFC import.
- **Cancellation:** wrap the IFC/glTF derivation in a job with progress (the drawing SVG import pattern, `SvgImportJob::step`), since `Serializer::serialize` is a single call with no progress argument.

---

## 7. Open items and risks

- `Serializer::serialize` has no cancellation or progress argument. Expensive derivations (booleans, tessellation) inside a leaf will block. Either precompute in a job and cache (inference), or pass the derived result in. Verify before coding: the leaf signature is `serialize(from: &S, children: &ArchiveChildren)` only.
- The root `Cargo.toml` `[workspace.dependencies]` has no `semio-s-artifact-stdio-*` entries, while the drawing crate writes `{ workspace = true }` for them. Confirm before adding `workspace = true` to BIM. The recipe uses explicit paths.
- The stdio IFC format descriptor says `is_binary: true`, but the 2x3 txt leaf emits text. Confirm which the host expects.
- `ifcopenshell` is not in `.venv` (r1). The IFC export oracle cannot run until installed.
- r1 notes the TS mirror `🚪️io/🟦️.ts` describes some drawing entries as "not-yet-implemented stubs" while the Rust implements them. Read the Rust.
- Verify `DRAWING_DIALECT` line numbers and the `🏅️standards/🔖️1/🦀️.rs` media block before copying; this recipe took them from r1.
