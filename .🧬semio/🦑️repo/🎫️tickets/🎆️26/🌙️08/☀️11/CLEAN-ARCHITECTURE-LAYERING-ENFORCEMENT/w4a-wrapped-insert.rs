
//#region 🔖️MediaVocabulary
// 🔀️ Relocated verbatim from 🔺️mesh/🦀️component.rs (ticket 26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT
// wave 4a) — manifest-vocabulary types, not codec material; mesh keeps only MeshData/Primitives/
// generic obj-glb-stl codecs plus the still-required MediaFormat enum (see that file's own note).
//#region ArtifactKind
/// 🧬️ Which geometry backend a resource kind's media exporters/importers target — the manifest-level
/// counterpart threaded onto `AppDefinition.artifact_kinds` (see `ArtifactKindSpec`). Canonical home for
/// what used to be duplicated verbatim in `framework/plugin/rs` and `framework/product/os/core/rs`; both
/// now re-export this definition instead of declaring their own.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum OsMediaCapability {
    MeshOnly,
    Brep,
}

/// 🗂️ An app-declared OS resource kind (e.g. a 3D mesh format, a raster format) — the manifest-level
/// counterpart to `AppBuilder::artifact_kind(...)` (`framework/plugin/rs`), letting `framework/product/os/core`
/// build its artifact catalog from `AppDefinition.artifact_kinds` at plugin registration time instead of
/// hardcoding a per-app match on kind-id strings. Carries the manifest-level media-kind fields
/// (`media_type`/`schema`/`export_formats`/`import_formats`) directly
/// so one spec carries both the OS-catalog presentation shape and the `MediaType` a wire actually negotiates
/// — see `crate::media_types_compatible`. `OsArtifactDescriptor` (`framework/product/os/core`) threads
/// `media_type` through so registry lookups return it alongside the rest of the descriptor.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ArtifactKindSpec {
    pub id: String,
    pub name: String,
    pub source_format: String,
    pub component_kind: String,
    pub dimension: String,
    pub media_capability: OsMediaCapability,
    pub media_type: MediaType,
    pub schema: String,
    pub export_formats: Vec<MediaFormat>,
    pub import_formats: Vec<MediaFormat>,
    /// 🗄️ Stdio export target kind ids (e.g. `stdio.json`) — additive peer of `export_formats`.
    #[serde(default, skip_serializing_if = "Vec::is_empty", skip_deserializing)]
    pub export_stdio_kinds: Vec<&'static str>,
    /// 🗄️ Stdio import source kind ids — additive peer of `import_formats`.
    #[serde(default, skip_serializing_if = "Vec::is_empty", skip_deserializing)]
    pub import_stdio_kinds: Vec<&'static str>,
}
//#endregion ArtifactKind

//#region MediaType
/// 🧬️ Typed-media lattice: every port/wire in the workflow carries a `MediaType` (`class` × `form`) instead of the legacy string `artifact_kind`. This is separate from `MediaFormat` above — `MediaType` is what a wire negotiates, `MediaFormat` is only how bytes are encoded once they actually cross a process boundary (see `MediaWireFormat`). Dependent tickets retire `OsMediaCapability` (see the `ArtifactKind` region above) onto `MediaForm::{Brep,Mesh}`, which already covers what `OsMediaCapability::{Brep,MeshOnly}` expresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MediaClass {
    TwoD,
    ThreeD,
    Text,
    Data,
    Graph,
    Kit,
    Computation,
    Presentation,
}

/// 🧬️ The shape/representation a `MediaClass` payload takes, orthogonal to `class` — e.g. `ThreeD` × `Brep` vs `ThreeD` × `Mesh`. `Any` only ever appears on the accepting side of a port (see `media_types_compatible`).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MediaForm {
    Any,
    Vector,
    Raster,
    Brep,
    Mesh,
    Document,
    Value,
    Dag,
    Trinity,
    Type,
    Design,
    Kit,
    Flow,
    Sequence,
    Imperative,
    Deck,
}

/// 🧬️ A port or wire's declared media type — the pair a producer offers or a consumer accepts.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct MediaType {
    pub class: MediaClass,
    pub form: MediaForm,
}

/// 🔌️ How a `MediaType` is actually encoded once it crosses a process boundary — binary payloads reuse `MediaFormat`, structured payloads carry a schema id instead (see `ArtifactKindSpec::schema`).
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum MediaWireFormat {
    Binary { format: MediaFormat },
    Document { schema: String }
}

/// 🔀️ Which side of a wire a `MediaPortSpec` sits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum MediaPortDirection {
    In,
    Out,
}

/// 🔢️ Whether a `MediaPortSpec` accepts/produces exactly one media value or a stream/collection of them — e.g. a mesh-array input that fans in from several upstream producers.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub enum PortMultiplicity {
    One,
    Many,
}

/// 🔌️ A single port an app exposes on the workflow — `kind_id` optionally pins it to one `ArtifactKindSpec.id` when the port is more specific than its `media_type` alone conveys.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct MediaPortSpec {
    pub id: String,
    pub label: String,
    pub direction: MediaPortDirection,
    pub media_type: MediaType,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional))]
    pub kind_id: Option<String>,
    pub required: bool,
    pub multiplicity: PortMultiplicity,
}

/// ⚖️ Result of checking whether a producer's `MediaType` can feed a consumer's accepted `MediaType`: exact match, a known lossy-but-allowed conversion, or outright rejection.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MediaCompat {
    Direct,
    Convert { from: MediaForm, to: MediaForm },
    Reject,
}

/// 🔀️ One-way `MediaForm` conversions the workflow is allowed to insert implicitly (e.g. a B-Rep producer feeding a mesh-only consumer). `media_types_compatible` looks up `(produced, accepted)` directly, so add the reverse pair too if a conversion should also hold the other way.
const MEDIA_FORM_CONVERSIONS: &[(MediaForm, MediaForm)] = &[
    (MediaForm::Brep, MediaForm::Mesh),
    (MediaForm::Vector, MediaForm::Raster),
    (MediaForm::Design, MediaForm::Kit),
    (MediaForm::Type, MediaForm::Kit),
];

/// ⚖️ The single source of truth for wire compatibility: classes must match exactly, `MediaForm::Any` on the accepting side takes anything within the class, equal forms are always direct, and everything else falls through to the explicit `MEDIA_FORM_CONVERSIONS` table.
pub fn media_types_compatible(produced: &MediaType, accepted: &MediaType) -> MediaCompat {
    if produced.class != accepted.class {
        return MediaCompat::Reject;
    }
    if matches!(accepted.form, MediaForm::Any) || produced.form == accepted.form {
        return MediaCompat::Direct;
    }
    for (from, to) in MEDIA_FORM_CONVERSIONS {
        if *from == produced.form && *to == accepted.form {
            return MediaCompat::Convert { from: *from, to: *to };
        }
    }
    MediaCompat::Reject
}
//#endregion MediaType

//#region 🔖️AppIo
/// 🧷️ The non-format fields of `ArtifactKindSpec` (see `ArtifactKind` region above) that describe how
/// a resource presents in the OS catalog — split out so `AppIo` can carry its own `export_formats`/
/// `import_formats` lists without duplicating `ArtifactKindSpec`'s full shape (which stays alive
/// unchanged for now; later waves retire it onto `AppIo`).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ArtifactPresentation {
    pub id: String,
    pub name: String,
    pub dimension: String,
    pub component_kind: String,
}

/// 🔌️ An app's full media I/O surface — the document schema/type every app carries implicitly (see
/// `document_in_port`/`document_out_port`) plus whatever additional workflow ports, catalog
/// export/import formats, and OS presentation it declares itself. Scaffolding for the typed manifest
/// surface (`AppDefinition.io`); apps don't populate this yet — later waves migrate `media_inputs`/
/// `media_outputs`/`artifact_kinds` onto it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct AppIo {
    pub document_schema: String,
    pub document_media_type: MediaType,
    /// 🔌️ App-specific ports only — the implicit document ports are auto-injected by `all_ports`.
    pub ports: Vec<MediaPortSpec>,
    pub export_formats: Vec<MediaFormat>,
    pub import_formats: Vec<MediaFormat>,
    pub artifact: ArtifactPresentation,
}

impl AppIo {
    /// 🔌️ The implicit `"document:in"` port every app accepts, keyed by `self.document_media_type`.
    pub fn document_in_port(&self) -> MediaPortSpec {
        MediaPortSpec {
            id: "document:in".into(),
            label: "Document".into(),
            direction: MediaPortDirection::In,
            media_type: self.document_media_type,
            kind_id: None,
            required: true,
            multiplicity: PortMultiplicity::One,
        }
    }

    /// 🔌️ The implicit `"document:out"` port every app produces — see `document_in_port`.
    pub fn document_out_port(&self) -> MediaPortSpec {
        MediaPortSpec {
            id: "document:out".into(),
            label: "Document".into(),
            direction: MediaPortDirection::Out,
            media_type: self.document_media_type,
            kind_id: None,
            required: true,
            multiplicity: PortMultiplicity::One,
        }
    }

    /// 🔌️ The full port list, in stable order: the implicit document ports first, followed by every app-specific port declared in `self.ports`.
    pub fn all_ports(&self) -> Vec<MediaPortSpec> {
        let mut ports = vec![self.document_in_port(), self.document_out_port()];
        ports.extend(self.ports.clone());
        ports
    }

    /// 🏗️ Builds an `AppIo` from just its implicit document surface, with no extra ports/formats declared yet — chain `.with_ports(...)` to add app-specific ports.
    pub fn from_document(schema: impl Into<String>, media_type: MediaType, artifact: ArtifactPresentation) -> Self {
        Self {
            document_schema: schema.into(),
            document_media_type: media_type,
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact,
        }
    }

    /// 🔌️ Attaches app-specific ports (beyond the implicit document ports) to this `AppIo`.
    pub fn with_ports(mut self, ports: Vec<MediaPortSpec>) -> Self {
        self.ports = ports;
        self
    }
}

impl Default for AppIo {
    fn default() -> Self {
        Self {
            document_schema: String::new(),
            document_media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
            ports: Vec::new(),
            export_formats: Vec::new(),
            import_formats: Vec::new(),
            artifact: ArtifactPresentation {
                id: String::new(),
                name: String::new(),
                dimension: String::new(),
                component_kind: String::new(),
            },
        }
    }
}
//#endregion 🔖️AppIo

//#region 🔖️ConfigSpec
/// 🧮️ How one config field's value is edited/validated, independent of what record it belongs to.
/// Deliberately hand-rolled rather than derived from `dsl_schema::Shape` (`dsl_schema`'s `Shape` isn't
/// `Serialize`/`Deserialize` — `Shape::Record`/`Statements`/`Table` carry `fn() -> RecordSpec` pointers
/// — and `semio-framework-core` doesn't depend on `dsl`/`dsl_schema` today, so wrapping it would add a
/// new cross-crate dependency purely to reach a shape that can't round-trip over the wire anyway).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum ConfigFieldShape {
    Number {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        min: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        max: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[cfg_attr(feature = "typegen", ts(optional))]
        step: Option<f64>,
    },
    Toggle,
    Text,
    Select { options: Vec<String> },
    Record(Vec<ConfigFieldSpec>),
}

/// 🧮️ One field of an app's declared configuration record — the whole-app-settings counterpart to
/// `ActionArgDef` (which scopes to a single action's arguments instead).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ConfigFieldSpec {
    pub key: String,
    pub label: String,
    pub shape: ConfigFieldShape,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "typegen", ts(optional, type = "unknown"))]
    pub default: Option<DslValue>,
}

/// 🧮️ An app's full typed configuration record — the manifest-level declaration
/// `AppDefinition.config` carries. Empty until per-app waves populate it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct ConfigSpec {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub fields: Vec<ConfigFieldSpec>,
}

impl ConfigSpec {
    pub fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️ConfigSpec

//#region 🔖️CommandGrammar
/// 🎛️ One field of a binary command variant — reuses `ConfigFieldShape` for the value shape (see
/// `ConfigFieldShape`'s doc comment for why command grammar fields are hand-rolled rather than
/// derived from `dsl_schema`). No `List`/array shape exists yet — the manifest's existing field-typed
/// vocabulary (`ActionArgControl`: Text/Number/Slider/Toggle/Select/Vec3/IconSelect) has no array
/// control either, so `ConfigFieldShape` doesn't invent one ahead of a real need.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandFieldSpec {
    pub key: String,
    pub shape: ConfigFieldShape,
    pub optional: bool,
}

/// 🎛️ One keyword-dispatched command variant (e.g. `move x=1 y=2`) and its field grammar.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandVariantSpec {
    pub keyword: String,
    pub fields: Vec<CommandFieldSpec>,
}

/// 🎛️ An app's full typed binary command grammar — the manifest-level declaration
/// `AppDefinition.command_grammar` carries. Empty until per-app waves populate it.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct CommandGrammar {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub variants: Vec<CommandVariantSpec>,
}

impl CommandGrammar {
    pub fn empty() -> Self {
        Self::default()
    }
}
//#endregion 🔖️CommandGrammar

//#region Media
/// 🎞️ The value that actually flows over a workflow wire, produced by `ArtifactApp::export_media` and consumed by `ArtifactApp::import_media`. Kept separate from the `MediaType` lattice above (which only negotiates *compatibility*, never carries a value) so headless runners and the UI share one payload shape.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase")]
pub struct Media {
    pub media_type: MediaType,
    pub payload: MediaPayload,
}

/// 📦️ Structured payloads stay inline as canonical JSON (small, diffable); binary payloads are content-addressed through `store::BlobStore` so a `Media` value never carries megabytes across a WIT boundary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum MediaPayload {
    Structured { schema: String, json: String },
    Binary { format: MediaFormat, blob_hash: String }
}

/// 🔑️ A cheap identity for one port's current output, independent of serializing the full payload — the unit the `SpaceRunner` compares to decide whether a downstream node actually needs to see a new value.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "typegen", derive(ts_rs::TS))]
pub struct MediaFingerprint(pub String);

impl MediaFingerprint {
    /// 🔑️ Canonical fingerprint of a `Media` value: structured payloads hash their JSON text, binary payloads reuse their existing content hash directly (no re-hashing bytes already addressed by the blob store).
    pub fn of(media: &Media) -> Self {
        match &media.payload {
            MediaPayload::Structured { schema, json } => {
                MediaFingerprint(semio_framework_hash::hash_parts(&[schema.as_str(), json.as_str()]))
            }
            MediaPayload::Binary { blob_hash, .. } => MediaFingerprint(blob_hash.clone()),
        }
    }
}

/// 🚧️ Failure exporting, importing, or fingerprinting media on a declared port.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum MediaError {
    #[error("unknown media port `{0}`")]
    UnknownPort(String),
    #[error("port `{port}` produced {produced:?} but the wire accepts {accepted:?}")]
    Incompatible { port: String, produced: MediaType, accepted: MediaType },
    #[error("media payload error on port `{0}`: {1}")]
    Payload(String, String),
    #[error("media ports are not implemented for this app")]
    NotImplemented,
}

/// 🔀️ A registered one-way conversion the workflow may insert on a wire when `media_types_compatible` reports `MediaCompat::Convert`. Kept behind a trait (never a bare closure) so converters can be enumerated, tested, and swapped without touching the runner.
pub trait MediaConverter: Send + Sync {
    fn from_form(&self) -> MediaForm;
    fn to_form(&self) -> MediaForm;
    fn convert(&self, media: &Media) -> Result<Media, MediaError>;
}
//#endregion Media
//#endregion 🔖️MediaVocabulary
