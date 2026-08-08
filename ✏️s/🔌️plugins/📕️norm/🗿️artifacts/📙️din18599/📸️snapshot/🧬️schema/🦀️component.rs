//! 🧬️ Din18599 snapshot schema — persistent fields only.

use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot


#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.din18599", layout = "lines")]
#[artifact_schema(id = "s.norm.din18599")]
pub struct Din18599Snapshot {
    pub use_class: UseClass,
    #[dsl(unit = "m2")]
    pub heated_area_m2: f64,
    pub occupants: u32,
    pub h_t: f64,
    pub h_v: f64,
    #[dsl(block)]
    pub climate: MonthlyClimate,
    pub internal_gains_w_m2: f64,
    pub solar_gains_kwh: f64,
    pub system_losses_kwh: f64,
    pub renewable_kwh: f64,
    pub annual_limit_kwh: f64,
    pub energy_carrier: String,
    pub reference_q_p_kwh: f64,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for Din18599Snapshot {
    const EXTENSION: &'static str = "din18599";
    fn envelope_id() -> &'static str { "norm.din18599" }
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

impl store::DocumentPack for Din18599Snapshot {
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

impl Default for Din18599Snapshot {
    fn default() -> Self {
        Self {
            use_class: UseClass::Residential,
            heated_area_m2: 100.0,
            occupants: 4,
            h_t: 92.12124613902822,
            h_v: 40.800000000000004,
            climate: MonthlyClimate {
                theta_e_c: [-14.0, -11.186533479473212, -3.4999999999999964, 7.000000000000001, 17.5, 25.186533479473212, 28.0, 25.186533479473212, 17.5, 7.000000000000001, -3.4999999999999964, -11.186533479473212],
                g_h_w_m2: [30.0, 60.0, 100.0, 140.0, 180.0, 200.0, 210.0, 190.0, 140.0, 90.0, 40.0, 20.0],
            },
            internal_gains_w_m2: 3.5,
            solar_gains_kwh: 84.0,
            system_losses_kwh: 800.0,
            renewable_kwh: 1500.0,
            annual_limit_kwh: 7500.0,
            energy_carrier: "natural_gas".into(),
            reference_q_p_kwh: 10000.0,
        }
    }
}
//#endregion 🔖️Snapshot
