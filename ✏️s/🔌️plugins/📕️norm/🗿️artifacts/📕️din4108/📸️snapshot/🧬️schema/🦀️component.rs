//! 🧬️ Din4108 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use crate::document::ClimateZoneDe;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.din4108", layout = "lines")]
#[artifact_schema(id = "s.norm.din4108")]
pub struct Din4108Snapshot {
    pub category: String,
    #[dsl(table)]
    pub layers: Vec<crate::artifacts::din4108::LayerDocument>,
    pub climate: ClimateZoneDe,
    pub airtightness_n50: f64,
    pub psi_times_l_sum: f64,
    pub rh_int: f64,
    pub catalog_id: String,
    pub material_id: String,
    pub airtightness_class: String,
    pub t_int_c: f64,
    pub solar_absorptance: f64,
    pub irradiance_w_m2: f64,
    pub moisture_mu_exterior: f64,
    pub moisture_mu_interior: f64,
    #[dsl(unit = "m2")]
    pub envelope_area_m2: f64,
    pub bb2_details_conform: bool,
    pub application_type: String,
    pub declared_application_class: String,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for Din4108Snapshot {
    const EXTENSION: &'static str = "din4108";
    fn envelope_id() -> &'static str { "norm.din4108" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::DocumentPack for Din4108Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::DocumentDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::DocumentDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::DocumentDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedDocumentCodecs

impl Default for Din4108Snapshot {
    fn default() -> Self {
        Self {
            category: "residential".into(),
            layers: vec![LayerDocument { thickness_m: 0.24, lambda_w_mk: 0.81 }, LayerDocument { thickness_m: 0.14, lambda_w_mk: 0.035 }],
            climate: ClimateZoneDe::Zone2,
            airtightness_n50: 2.5,
            psi_times_l_sum: 0.02,
            rh_int: 0.5,
            catalog_id: "AW-01".into(),
            material_id: "mineral_wool".into(),
            airtightness_class: "class2".into(),
            t_int_c: 20.0,
            solar_absorptance: 0.6,
            irradiance_w_m2: 600.0,
            moisture_mu_exterior: 15.0,
            moisture_mu_interior: 1.3,
            envelope_area_m2: 100.0,
            bb2_details_conform: true,
            application_type: "DEO".into(),
            declared_application_class: "dk".into(),
        }
    }
}
//#endregion 🔖️Snapshot
