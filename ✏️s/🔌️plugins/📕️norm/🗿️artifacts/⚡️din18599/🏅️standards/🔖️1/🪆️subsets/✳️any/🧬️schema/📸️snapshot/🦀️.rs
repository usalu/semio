//! 🧬️ Din18599 snapshot schema — complete building subject (zones, envelope, systems).

use crate::{
    Adjacency, Attachment, AutomationClass, BuildingCategory, CalculationMethod, CoolingSystem, DhwSystem, Din18599ClimateChild, ElementKind, EnvelopeElement, HeatingSystem, LightingSystem, MonthlyClimate, Renewables, ThermalZone, UseClass, VentilationSystem,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot

/// 📸️ Persisted Din18599 building energy subject. Climate remains a composed `s.stdio.semio`/`table`
/// child (Potsdam / part-10 monthly means); envelope and zones are id-keyed lists; plant systems are
/// nested records. Derived H_T / H_V / Q_P are never stored as free inputs.
#[derive(Clone, Debug, PartialEq, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(extension = "din18599")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Snapshot {
    #[state(artifact)]
    pub building_category: BuildingCategory,
    #[state(artifact)]
    pub attachment: Attachment,
    #[state(artifact)]
    pub use_class: UseClass,
    #[state(artifact)]
    pub method: CalculationMethod,
    #[state(artifact)]
    pub net_floor_area_m2: f64,
    #[state(artifact)]
    pub heated_volume_m3: f64,
    #[state(artifact)]
    pub geg_qp_factor: f64,
    #[state(artifact)]
    pub delta_u_wb_w_m2k: f64,
    #[state(artifact)]
    pub automation_class: AutomationClass,
    #[dsl(table)]
    #[state(artifact)]
    pub zones: Vec<ThermalZone>,
    #[dsl(table)]
    #[state(artifact)]
    pub elements: Vec<EnvelopeElement>,
    #[state(artifact)]
    pub heating: HeatingSystem,
    #[state(artifact)]
    pub dhw: DhwSystem,
    #[state(artifact)]
    pub ventilation: VentilationSystem,
    #[state(artifact)]
    pub cooling: CoolingSystem,
    #[state(artifact)]
    pub lighting: LightingSystem,
    #[state(artifact)]
    pub renewables: Renewables,
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    #[cfg_attr(test, serde(with = "crate::document::child_identity_oracle"))]
    pub climate: Din18599ClimateChild,
}

//#region 🔖️HandcraftedArtifactCodecs
/// ✉️ `ArtifactDsl` and `ArtifactPack` over the one derived record spec, composed child included.
impl store::ArtifactDsl for Din18599Snapshot {
    const EXTENSION: &'static str = "din18599";
    fn envelope_id() -> &'static str {
        "norm.din18599"
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for Din18599Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes).map_err(|e| store::PackError::Schema(e.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema(format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as store::ArtifactDsl>::envelope_id(), envelope.binary_token())));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
//#endregion 🔖️HandcraftedArtifactCodecs

impl Default for Din18599Snapshot {
    fn default() -> Self {
        crate::subjects::compliant_detached_house()
    }
}
//#endregion 🔖️Snapshot

//#region 🌉️ExternalCodecBridge
/// 📤️ The canonical JSON projection of a [`Din18599Snapshot`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_snapshot_json(snapshot: &Din18599Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

/// 📥️ The `serde_json` inverse of [`encode_din18599_snapshot_json`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_snapshot_json(text: &str) -> Result<Din18599Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

/// 📖️ Parses the committed `.dsl.semio` artifact into a [`Din18599Snapshot`].
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_dsl(text: &str) -> Result<Din18599Snapshot, String> {
    <Din18599Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

/// 🖨️ Prints a [`Din18599Snapshot`] back to its canonical `.dsl.semio` body.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_dsl(snapshot: &Din18599Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}

/// 📦️ Decodes a [`Din18599Snapshot`] from the binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_din18599_pack(bytes: &[u8]) -> Result<Din18599Snapshot, String> {
    <Din18599Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}

/// 📦️ Encodes a [`Din18599Snapshot`] to its binary `.pack.semio` envelope.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn encode_din18599_pack(snapshot: &Din18599Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
