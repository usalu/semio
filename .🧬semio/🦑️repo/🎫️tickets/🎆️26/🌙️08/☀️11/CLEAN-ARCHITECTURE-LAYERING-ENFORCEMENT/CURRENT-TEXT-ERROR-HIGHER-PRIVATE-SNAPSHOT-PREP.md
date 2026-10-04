# Higher Private Snapshot Constructors Retain Value Authority

Three actual Energy, Playbook and Procedure private into_snapshot owners return ValueError directly; old own Value/PackJSON ValueError display projections are retired. Their actual TextError source boundary uses from_value_error with the original authored span. Pack consumption requires canonical From<ValueError> retention at the actual PackError owner. These sources are authored unmounted until that downstream package floor and direct provider bindings are coherent; this is not native GREEN or source-ready. Original bodies/laws remain retained. No String reconstruction/default cause is introduced.

Complete immediate source text, inverse, authored full text, byte counts, hashes and exact per-site decisions are retained in generated/value-refusal/text-error-higher-private-snapshot-authored-1.json. Native admission is pending.

## ✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ EnergyModel snapshot schema — artifact-lane fields only.

use crate::{energy_snapshot_with_state, EnergyStructureChild, EnergyZonesChild, ENERGY_MODEL_DOCUMENT_SCHEMA};
use framework_schema::ArtifactSchema;
use semio_framework_os_kernel::{DslValue, FromValue, ToValue, ValueError};
#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;
#[path="🛬️native/🦀️.rs"]
mod native;

//#region 🔖️Snapshot
/// 📸️ Persisted energy-model document snapshot (persistent fields of the artifact). Ticket
/// 26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM (`energy→C:value,table R:model`): the old
/// `model_json: String` opaque-JSON field is replaced by two fixed composed CHILD slots — this
/// artifact no longer defines its own persisted-value/table content model, it composes stdio's
/// `value`/`table` subsets instead (see the artifact root's `🔖️Composition` region for the full
/// before/after and the honest exception carve-out for `Surface.vertices_m`). `referenced_model` is
/// a new forward `ArtifactLink` slot. `#[child(...)]`/`#[link_slot(...)]` drive
/// `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written. Text and pack both encode
/// the derived `EnergyModelPackRecord` below.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.energy.model")]
pub struct EnergyModelSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub model: crate::model::Model,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub structure: EnergyStructureChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub zones: EnergyZonesChild,
    #[state(artifact)]
    #[link_slot(roles("model"))]
    pub referenced_model: Option<store::ArtifactLink>,
    /// 🌦️ Forward link to the `🌦️epw` stdio artifact this model is simulated against — a link slot
    /// exactly like `referenced_model`, never an inlined `WeatherData` (ticket
    /// 26/09/06/ENERGY-PLUGIN-END-TO-END).
    #[state(artifact)]
    #[link_slot(roles("weather"))]
    pub weather_link: Option<store::ArtifactLink>,
}

impl Default for EnergyModelSnapshot {
    fn default() -> Self {
        energy_snapshot_with_state(ENERGY_MODEL_DOCUMENT_SCHEMA, &crate::model::Model::default(), None)
    }
}

// 🌱️ Hand-written, not derived — `structure`/`zones` are `store::ArtifactChild<S>` and
// `referenced_model` is `Option<store::ArtifactLink>`, neither of which has a
// `#[derive(ToValue, FromValue)]`-reachable impl (fan-out playbook trap #3; `ArtifactLink` mirrors
// the same framework-exempt shape). `model: crate::model::Model` goes through `ToValue`/`FromValue`
// directly — `Model` now derives both.
impl ToValue for EnergyModelSnapshot {
    fn to_value(&self) -> DslValue {
        DslValue::object([
            ("schema".to_string(), self.schema.to_value()),
            ("model".to_string(), self.model.to_value()),
            ("structure".to_string(), semio_framework_value::ToValue::to_value(&self.structure)),
            ("zones".to_string(), semio_framework_value::ToValue::to_value(&self.zones)),
            ("referencedModel".to_string(), semio_framework_value::ToValue::to_value(&self.referenced_model)),
            ("weatherLink".to_string(), semio_framework_value::ToValue::to_value(&self.weather_link)),
        ])
    }
}
impl FromValue for EnergyModelSnapshot {
    fn from_value(value: DslValue) -> Result<Self, ValueError> {
        let entries = DslValue::into_object(value)?;
        let field = |key: &str| entries.iter().find(|(k, _)| k == key).map_or(DslValue::Null, |(_, v)| v.clone());
        Ok(Self {
            schema: String::from_value(field("schema"))?,
            model: crate::model::Model::from_value(field("model"))?,
            structure: semio_framework_value::FromValue::from_value(field("structure"))?,
            zones: semio_framework_value::FromValue::from_value(field("zones"))?,
            referenced_model: semio_framework_value::FromValue::from_value(field("referencedModel"))?,
            weather_link: semio_framework_value::FromValue::from_value(field("weatherLink"))?,
        })
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️PackRecord
/// 🔋️ Derived pack record of an `EnergyModelSnapshot` — every field as persisted, with the typed
/// `model` carried as its first-party value.
#[derive(dsl::DslRecord)]
#[dsl(extension = "energy")]
struct EnergyModelPackRecord {
    schema: String,
    model: DslValue,
    structure: EnergyStructureChild,
    zones: EnergyZonesChild,
    referenced_model: Option<store::ArtifactLink>,
    weather_link: Option<store::ArtifactLink>,
}

impl EnergyModelPackRecord {
    fn from_snapshot(snapshot: &EnergyModelSnapshot) -> Self {
        Self { schema: snapshot.schema.clone(), model: snapshot.model.to_value(), structure: snapshot.structure.clone(), zones: snapshot.zones.clone(), referenced_model: snapshot.referenced_model.clone(), weather_link: snapshot.weather_link.clone() }
    }

    fn into_snapshot(self) -> Result<EnergyModelSnapshot, String> {
        let model = crate::model::Model::from_value(self.model).map_err(|error| error.to_string())?;
        Ok(EnergyModelSnapshot { schema: self.schema, model, structure: self.structure, zones: self.zones, referenced_model: self.referenced_model, weather_link: self.weather_link })
    }
}

/// 🖨️ The derived text body: the same `EnergyModelPackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &EnergyModelSnapshot) -> String {
    dsl::print(&EnergyModelPackRecord::from_snapshot(snapshot).__dsl_to_record(), &EnergyModelPackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `EnergyModelPackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<EnergyModelSnapshot, semio_framework_diagnostic::TextError> {
    let record = dsl::parse(body, &EnergyModelPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
    EnergyModelPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for EnergyModelSnapshot {
    const EXTENSION: &'static str = "energy";
    fn envelope_id() -> &'static str {
        "energy.model"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for EnergyModelSnapshot {
    fn sqlite_snapshot_codec()->Option<store::ArtifactSqliteSnapshotCodec>{Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())}
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&EnergyModelPackRecord::__dsl_spec(), &EnergyModelPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &EnergyModelPackRecord::__dsl_spec(), options)?;
        EnergyModelPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(EnergyModelPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️round-trip/🦀️.rs"]
mod round_trip_tests;
#[cfg(test)]
#[path = "🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;
//#endregion 🧪️Tests

//#region 🌉️IdentityBridge
/// 🔁️ One JSON report of carrying `dsl_text` through this subset's own codecs, for a
/// language-neutral test adapter. Same reachability wall as `energy_model_mutation_report_json`:
/// `store::ArtifactDsl`/`store::ArtifactPack` and their error types are unnameable outside this
/// crate, so the identity law's evidence has to be produced here and handed over as text.
///
/// `canonicalText` is `print_dsl` of the parsed document and `canonicalTextAgain` is `print_dsl` of
/// re-parsing that — [`store::ArtifactDsl`]'s own documented LAW is that canonical output is a
/// `parse_dsl` fixpoint (hand-written text may normalize on the way in), so the two must be
/// byte-identical while neither is required to equal the committed file. `packDecoded` comes back
/// through a SEPARATE binary codec, so agreeing on one snapshot cannot be achieved by carrying text
/// bytes across.
pub fn energy_model_identity_report_json(dsl_text: &str) -> Result<String, String> {
    let parsed = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(dsl_text).map_err(|error| error.to_string())?;
    let canonical = <EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(&parsed);
    let reparsed = <EnergyModelSnapshot as store::ArtifactDsl>::parse_dsl(&canonical).map_err(|error| error.to_string())?;
    let canonical_again = <EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(&reparsed);
    let packed = <EnergyModelSnapshot as store::ArtifactPack>::encode_pack(&reparsed);
    let unpacked = <EnergyModelSnapshot as store::ArtifactPack>::decode_pack(&packed).map_err(|error| error.to_string())?;
    let report = semio_framework_pack_json::object([
        ("parsed".to_string(), semio_framework_pack_json::from_dsl_value(&parsed.to_value())),
        ("reparsed".to_string(), semio_framework_pack_json::from_dsl_value(&reparsed.to_value())),
        ("packDecoded".to_string(), semio_framework_pack_json::from_dsl_value(&unpacked.to_value())),
        ("canonicalText".to_string(), semio_framework_pack_json::Value::String(canonical)),
        ("canonicalTextAgain".to_string(), semio_framework_pack_json::Value::String(canonical_again)),
    ]);
    Ok(semio_framework_pack_json::to_string(&report))
}
//#endregion 🌉️IdentityBridge

```

## ✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ Playbook snapshot schema — artifact-lane fields only.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`playbook→C:document,flow`): `steps:
//! Vec<PlaybookStep>` is replaced by the composed `document`/`flow` child slots (see the artifact
//! root's `🔖️ContentBridge` region). This struct no longer maps 1:1 onto the kernel `PlaybookSpec`:
//! `store::ArtifactDsl` and `ArtifactPack` are the derived text and pack of `PlaybookPackRecord`.
//! Both carry the steps the `flow` handle's local owner holds, because the
//! parent is the state every composed child is derived from (`genesis_playbook_child_pack`).

use crate::PlaybookStep;
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted playbook document snapshot (persistent fields of the artifact). `#[child(...)]`
/// drives `#[derive(ArtifactSchema)]`'s slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, ArtifactSchema)]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    pub id: String,
    #[state(artifact)]
    pub version: String,
    #[state(artifact)]
    pub title: Option<String>,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub document: crate::PlaybookDocumentChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: crate::PlaybookFlowChild,
}

impl Default for PlaybookSnapshot {
    fn default() -> Self {
        let kernel = crate::playbook::empty_playbook_snapshot();
        Self::from_kernel(crate::playbook::PlaybookSpec { schema: kernel.schema, id: kernel.id, version: kernel.version, title: kernel.title, steps: kernel.steps })
    }
}

impl PlaybookSnapshot {
    /// 🌉️ Builds a plugin snapshot from the shared kernel `PlaybookSpec`, minting/caching the
    /// composed `document`/`flow` children from its `steps`.
    pub fn from_kernel(spec: crate::playbook::PlaybookSpec) -> Self {
        crate::playbook_snapshot_with_steps(&spec.schema, &spec.id, &spec.version, spec.title, spec.steps)
    }

    /// 🌉️ Lowers this snapshot into the kernel `PlaybookSpec` for shared domain helpers — reads
    /// steps off the `flow` child's working-scene cache (see the artifact root's `🔖️WorkingScene`).
    pub fn to_kernel(self) -> crate::playbook::PlaybookSpec {
        self.as_kernel()
    }

    /// 🌉️ Borrows as kernel spec without consuming `self`.
    pub fn as_kernel(&self) -> crate::playbook::PlaybookSpec {
        crate::playbook::PlaybookSpec { schema: self.schema.clone(), id: self.id.clone(), version: self.version.clone(), title: self.title.clone(), steps: crate::playbook_steps(self) }
    }

    /// 🔎️ The current steps, read through the composed `flow` child's working-scene cache — the
    /// single call site every render/inference/export path in this plugin uses instead of the old
    /// `.steps` field access.
    pub fn steps(&self) -> Vec<PlaybookStep> {
        crate::playbook_steps(self)
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived — mirrors the sibling `PlaybookArtifact` impl one region up
/// (`../🦀️.rs`'s `🔖️ValueCodec`): `document`/`flow` are `store::ArtifactChild<S>` composed-artifact
/// handles, bridged per-field through the pre-existing `to_dsl_value`/`from_dsl_value` seam instead
/// of widening the derive macro to understand child-slot handles.
impl ::semio_framework_os_kernel::ToValue for PlaybookSnapshot {
    fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
        ::semio_framework_os_kernel::DslValue::object([
            ("schema".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.schema)),
            ("id".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.id)),
            ("version".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.version)),
            ("title".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.title)),
            ("document".to_string(), semio_framework_value::ToValue::to_value(&self.document)),
            ("flow".to_string(), semio_framework_value::ToValue::to_value(&self.flow)),
        ])
    }
}
impl ::semio_framework_os_kernel::FromValue for PlaybookSnapshot {
    fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let field = |key: &str| get(key).ok_or_else(|| ::semio_framework_os_kernel::ValueError::new(format!("missing field `{key}`")));
        Ok(Self {
            schema: ::semio_framework_os_kernel::FromValue::from_value(field("schema")?)?,
            id: ::semio_framework_os_kernel::FromValue::from_value(field("id")?)?,
            version: ::semio_framework_os_kernel::FromValue::from_value(field("version")?)?,
            title: ::semio_framework_os_kernel::FromValue::from_value(field("title")?)?,
            document: semio_framework_value::FromValue::from_value(field("document")?)?,
            flow: semio_framework_value::FromValue::from_value(field("flow")?)?,
        })
    }
}
//#endregion 🔖️ValueCodec

//#region 🔖️PackRecord
/// 📦️ Derived pack record of a `PlaybookSnapshot`: both composed-child handles plus the steps the
/// `flow` handle's local owner holds, which a bare-handle pack would lose. The steps travel as their
/// first-party JSON text because block `default`/`params` are free-form `DslValue`s whose key order is
/// significant, while pack canonicalises map keys into sorted order.
#[derive(dsl::DslRecord)]
#[dsl(extension = "playbook")]
struct PlaybookPackRecord {
    schema: String,
    id: String,
    version: String,
    title: Option<String>,
    document: crate::PlaybookDocumentChild,
    flow: crate::PlaybookFlowChild,
    steps: String,
}

impl PlaybookPackRecord {
    fn snapshot_record_controlled(snapshot: &PlaybookSnapshot, control: &mut dsl::NativeEncodeControl<'_>) -> Result<dsl::RecordValue, String> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(7)?; let mut record = dsl::native_encoding::EncodedRecord::new(7, control)?;
            for (id, value) in [(0, &snapshot.schema), (1, &snapshot.id), (2, &snapshot.version)] { let value = control.scoped_stage(|control| { control.begin_stage(0)?; <String as dsl::DslField>::to_value_controlled(value, control) })?; record.insert(id, value); control.step()?; }
            record.insert(3, control.scoped_stage(|control| { control.begin_stage(0)?; match &snapshot.title { Some(title) => <String as dsl::DslField>::to_value_controlled(title, control), None => { control.step()?; Ok(dsl::FieldValue::Absent) } } })?); control.step()?;
            record.insert(4, control.scoped_stage(|control| { control.begin_stage(0)?; <crate::PlaybookDocumentChild as dsl::DslField>::to_value_controlled(&snapshot.document, control) })?); control.step()?;
            record.insert(5, control.scoped_stage(|control| { control.begin_stage(0)?; <crate::PlaybookFlowChild as dsl::DslField>::to_value_controlled(&snapshot.flow, control) })?); control.step()?;
            let steps = match snapshot.flow.local_owner::<crate::PlaybookWorkingScene>() { Some(scene) => semio_framework_pack_json::to_json_string_controlled(&scene.steps, control), None => semio_framework_pack_json::to_json_string_controlled(&Vec::<PlaybookStep>::new(), control) }?;
            record.insert(6, dsl::FieldValue::Text(steps)); control.step()?; Ok(record.take())
        }))
    }

    fn into_snapshot_controlled(mut self, control: &mut dsl::NativeDecodeControl<'_>) -> Result<PlaybookSnapshot, semio_framework_diagnostic::TextError> {
        use dsl::FromValue;
        let steps = semio_framework_pack_json::from_json_str_controlled::<Vec<PlaybookStep>>(&self.steps, semio_framework_pack_json::JsonMemberPolicy::Reject, control).map_err(|error| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1)))?.guard_decoded();
        control.charge(std::mem::size_of::<crate::PlaybookWorkingScene>() + 2 * std::mem::size_of::<usize>()).map_err(|message| semio_framework_diagnostic::TextError::from_value_error(message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        crate::attach_playbook_steps(&mut self.flow, steps.take()); Ok(PlaybookSnapshot { schema: self.schema, id: self.id, version: self.version, title: self.title, document: self.document, flow: self.flow })
    }

    fn from_snapshot(snapshot: &PlaybookSnapshot) -> Self {
        Self {
            schema: snapshot.schema.clone(),
            id: snapshot.id.clone(),
            version: snapshot.version.clone(),
            title: snapshot.title.clone(),
            document: snapshot.document.clone(),
            flow: snapshot.flow.clone(),
            steps: semio_framework_pack_json::to_json_string(&crate::playbook_steps(snapshot)),
        }
    }

    fn into_snapshot(self) -> Result<PlaybookSnapshot, String> {
        let mut flow = self.flow;
        let steps: Vec<PlaybookStep> = semio_framework_pack_json::from_json_str(&self.steps, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())?;
        crate::attach_playbook_steps(&mut flow, steps);
        Ok(PlaybookSnapshot { schema: self.schema, id: self.id, version: self.version, title: self.title, document: self.document, flow })
    }
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
impl store::ArtifactDsl for PlaybookSnapshot {
    const EXTENSION: &'static str = "playbook";
    fn envelope_id() -> &'static str {
        "playbook.playbook"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &PlaybookPackRecord::__dsl_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
        PlaybookPackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&PlaybookPackRecord::from_snapshot(self).__dsl_to_record(), &PlaybookPackRecord::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for PlaybookSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&PlaybookPackRecord::__dsl_spec(), &PlaybookPackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &PlaybookPackRecord::__dsl_spec(), options)?;
        PlaybookPackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(|error| store::PackError::Schema(error.to_string()))
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(PlaybookPackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🔖️ExternalBridges
/// 📖️ Parses `.playbook` DSL text with a plain-`String` error, reachable from OUTSIDE this crate —
/// `store` is a private `extern crate` alias (`🦀️.rs`), so `store::TextError` cannot be named
/// by the exhaustive mutation case's test adapter that has to read the committed
/// `🗣️.dsl.semio` artifact.
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn parse_playbook_dsl(text: &str) -> Result<PlaybookSnapshot, String> {
    <PlaybookSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string())
}

/// 🖨️ Prints a [`PlaybookSnapshot`] back to `.playbook` DSL text under a name an external caller can reach, paired
/// with [`parse_playbook_dsl`].
// 🚫️async: E1 pure codec helper (file verified I/O-free) — see R9
pub fn print_playbook_dsl(snapshot: &PlaybookSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🔖️ExternalBridges

#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;

```

## ✏️s/🔌️plugins/📜️imperative/🗿️artifacts/📜️procedure/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs

```rust
//! 🧬️ Imperative snapshot schema — artifact-lane fields only.

use crate::{ProcedureFlowChild, ProcedureTextChild};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted imperative document snapshot (persistent fields of the artifact). Ticket
/// `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`imperative→C:text,flow`): the inline `path:
/// Path` (the ordered/nested `Step` control-flow tree) and `seed: BTreeMap<String, Value>` (the
/// initial variable dictionary) content fields are replaced by two fixed composed CHILD slots —
/// this plugin no longer defines its own program-graph or seed-content model, it composes stdio's
/// `flow` and `text` subsets instead. `#[child(...)]` drives `#[derive(ArtifactSchema)]`'s
/// slot-table emission; never hand-written.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::FromValue, ArtifactSchema)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[artifact_schema(id = "s.imperative.procedure")]
pub struct ProcedureSnapshot {
    #[state(artifact)]
    pub schema: String,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub flow: ProcedureFlowChild,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub text: ProcedureTextChild,
}

impl Default for ProcedureSnapshot {
    fn default() -> Self {
        crate::procedure_snapshot_with_content("procedure.document", &crate::Path::new(), &std::collections::BTreeMap::new())
    }
}
//#endregion 🔖️Snapshot

//#region 🔖️PackRecord
/// 🛤️ Derived pack record of a `ProcedureSnapshot`: both composed-child handles plus the content their
/// local owners hold (the flow child's `path`, the text child's `seed`), which a bare-handle pack would
/// lose. Text and pack are both derived from this record.
#[derive(dsl::DslRecord)]
#[dsl(extension = "imperative")]
struct ProcedurePackRecord {
    schema: String,
    flow: ProcedureFlowChild,
    text: ProcedureTextChild,
    path: dsl::DslValue,
    seed: dsl::DslValue,
}

impl ProcedurePackRecord {
    fn snapshot_record_controlled(snapshot: &ProcedureSnapshot, control: &mut dsl::NativeEncodeControl<'_>) -> Result<dsl::RecordValue, String> {
        control.scoped_depth(64, |control| control.scoped_stage(|control| {
            control.begin_stage(5)?;
            let mut record = dsl::native_encoding::EncodedRecord::new(5, control)?;
            record.insert(0, control.scoped_stage(|control| { control.begin_stage(0)?; <String as dsl::DslField>::to_value_controlled(&snapshot.schema, control) })?); control.step()?;
            record.insert(1, control.scoped_stage(|control| { control.begin_stage(0)?; <ProcedureFlowChild as dsl::DslField>::to_value_controlled(&snapshot.flow, control) })?); control.step()?;
            record.insert(2, control.scoped_stage(|control| { control.begin_stage(0)?; <ProcedureTextChild as dsl::DslField>::to_value_controlled(&snapshot.text, control) })?); control.step()?;
            let path = match snapshot.flow.local_owner::<crate::ProcedureFlowWorkingData>() { Some(scene) => dsl::ToValue::to_value_controlled(&scene.path, control), None => dsl::ToValue::to_value_controlled(&crate::Path::new(), control) }.map_err(|error| error.to_string())?;
            record.insert(3, dsl::FieldValue::Value(path)); control.step()?;
            let seed = match snapshot.text.local_owner::<crate::ProcedureTextWorkingData>() { Some(scene) => dsl::ToValue::to_value_controlled(&scene.seed, control), None => dsl::ToValue::to_value_controlled(&std::collections::BTreeMap::<String, crate::Value>::new(), control) }.map_err(|error| error.to_string())?;
            record.insert(4, dsl::FieldValue::Value(seed)); control.step()?;
            Ok(record.take())
        }))
    }
    fn from_snapshot(snapshot: &ProcedureSnapshot) -> Self {
        let scene = crate::procedure_working_scene(snapshot);
        Self { schema: snapshot.schema.clone(), flow: snapshot.flow.clone(), text: snapshot.text.clone(), path: dsl::ToValue::to_value(&scene.path), seed: dsl::ToValue::to_value(&scene.seed) }
    }

    fn into_snapshot(self) -> Result<ProcedureSnapshot, String> {
        let (mut flow, mut text) = (self.flow, self.text);
        let seed = neural_engine::ColdOwner::new(<std::collections::BTreeMap<String, crate::Value> as dsl::FromValue>::from_value(self.seed).map_err(|error| error.to_string())?);
        let path: crate::Path = dsl::FromValue::from_value(self.path).map_err(|error| error.to_string())?;
        text.set_local_owner(std::sync::Arc::new(crate::ProcedureTextWorkingData { seed: seed.into_inner() }));
        flow.set_local_owner(std::sync::Arc::new(crate::ProcedureFlowWorkingData { path }));
        Ok(ProcedureSnapshot { schema: self.schema, flow, text })
    }
    fn into_snapshot_controlled(self, control: &mut dsl::NativeDecodeControl<'_>) -> Result<ProcedureSnapshot, semio_framework_diagnostic::TextError> {
        let error = |error: dsl::ValueError| semio_framework_diagnostic::TextError::from_value_error(error, semio_framework_diagnostic::TextSpan::at(1, 1));
        let seed = neural_engine::ColdOwner::new(<std::collections::BTreeMap<String, crate::Value> as dsl::FromValue>::from_value_controlled(&self.seed, control).map_err(error)?);
        let path = <crate::Path as dsl::FromValue>::from_value_controlled(&self.path, control).map_err(error)?;
        control.charge(std::mem::size_of::<crate::ProcedureFlowWorkingData>() + std::mem::size_of::<crate::ProcedureTextWorkingData>() + 4 * std::mem::size_of::<usize>()).map_err(|message| semio_framework_diagnostic::TextError::from_value_error(message, semio_framework_diagnostic::TextSpan::at(1, 1)))?;
        let (mut flow, mut text) = (self.flow, self.text);
        flow.set_local_owner(std::sync::Arc::new(crate::ProcedureFlowWorkingData { path }));
        text.set_local_owner(std::sync::Arc::new(crate::ProcedureTextWorkingData { seed: seed.into_inner() }));
        Ok(ProcedureSnapshot { schema: self.schema, flow, text })
    }
}

/// 🖨️ The derived text body: the same `ProcedurePackRecord` the pack encodes, printed by the spec-driven engine.
pub(crate) fn print_pack_record_text(snapshot: &ProcedureSnapshot) -> String {
    dsl::print(&ProcedurePackRecord::from_snapshot(snapshot).__dsl_to_record(), &ProcedurePackRecord::__dsl_spec(), dsl::JoinMode::Document)
}

/// 📖️ Parses a derived text body back through `ProcedurePackRecord`, with the same decode steps as the pack.
pub(crate) fn parse_pack_record_text(body: &str) -> Result<ProcedureSnapshot, semio_framework_diagnostic::TextError> {
    let record = dsl::parse(body, &ProcedurePackRecord::__dsl_spec(), &dsl::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: dsl::SourceMode::Document })?;
    ProcedurePackRecord::__dsl_from_record(&record)?.into_snapshot().map_err(|error| semio_framework_diagnostic::TextError::new(error, semio_framework_diagnostic::TextSpan::at(1, 1)))
}
//#endregion 🔖️PackRecord

//#region 🔖️HandcraftedArtifactCodecs
/// 🎁 `ArtifactDsl` and `ArtifactPack` are the derived text and pack of `ProcedurePackRecord`.
impl store::ArtifactDsl for ProcedureSnapshot {
    const EXTENSION: &'static str = "imperative";
    fn envelope_id() -> &'static str {
        "imperative.imperative"
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        parse_pack_record_text(body)
    }
    fn print_dsl(&self) -> String {
        let body = print_pack_record_text(self);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for ProcedureSnapshot {
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> { Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec()) }
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&ProcedurePackRecord::__dsl_spec(), &ProcedurePackRecord::from_snapshot(self).__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &ProcedurePackRecord::__dsl_spec(), options)?;
        ProcedurePackRecord::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)?.into_snapshot().map_err(store::PackError::Schema)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(ProcedurePackRecord::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

//#region 🌉️ExternalCodecBridge
/// 📤️ Renders an [`ProcedureSnapshot`] as this facet's own camelCase JSON projection — the
/// comparison surface `🛟️mutate-procedure-1`'s scenarios are measured through, and the shape the
/// committed `../🧫️fixtures/🧬️mutations/<slug>/<fixture>/📸️snapshot/{⬅️before,➡️after}/🔣️.json`
/// specification vectors are written in. It carries `flow` and `text` as content-addressed HANDLES,
/// never as content, which is what makes it a usable observability surface here: the `flow` handle
/// moves if and only if the program moved.
///
/// A thin `dsl::os_pack::json` wrapper over `ProcedureSnapshot`'s own `ToValue` impl — first-party,
/// infallible (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS).
pub fn encode_procedure_snapshot_json(snapshot: &ProcedureSnapshot) -> String {
    semio_framework_pack_json::to_json_string(snapshot)
}

/// 📥️ The inverse of [`encode_procedure_snapshot_json`] — decodes those committed specification
/// vectors into real [`ProcedureSnapshot`] values, so `🛟️mutate-procedure-1`'s adapter reads the
/// committed fixture rather than re-declaring it as a Rust literal beside it.
pub fn decode_procedure_snapshot_json(text: &str) -> Result<ProcedureSnapshot, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}

/// 📝️ Parses `.imperative.dsl.semio` text into an [`ProcedureSnapshot`] — a named, non-async
/// pass-through of this type's own `store::ArtifactDsl` impl above, whose trait and error type are
/// both unnameable outside this crate, so `🛟️mutate-procedure-1`'s `identity-round-trip` scenario
/// reaches the real committed artifact (`../../🖼️assets/🎬️demo/🗣️.dsl.semio`)
/// through this instead.
pub fn parse_procedure_dsl(text: &str) -> Result<ProcedureSnapshot, String> {
    <ProcedureSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 📝️ Renders an [`ProcedureSnapshot`] back as `.imperative.dsl.semio` text — the inverse of
/// [`parse_procedure_dsl`], preamble and both composed child handles included.
pub fn print_procedure_dsl(snapshot: &ProcedureSnapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
//#endregion 🌉️ExternalCodecBridge


#[cfg(test)]
#[path="🧪️tests/🪶️sqlite/🦀️.rs"]
mod sqlite_snapshot_tests;

#[path="🪶️sqlite/🦀️.rs"]
pub mod sqlite;

```

