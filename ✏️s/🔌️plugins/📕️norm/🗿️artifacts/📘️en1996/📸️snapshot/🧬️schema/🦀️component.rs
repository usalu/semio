//! 🧱️ EN 1996 snapshot schema — persistent fields only.

use crate::artifacts::en1996::{MasonryClass, part_2};
use crate::document::{AnnexChoice, DesignSituation};
use schema::ArtifactSchema;
use serde::{Deserialize, Serialize};

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1996 document snapshot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord, ArtifactSchema)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1996", layout = "lines")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Snapshot {
    #[state(persistent)]
    pub m_ed_knm: f64,
    #[state(persistent)]
    pub n_ed_kn: f64,
    #[state(persistent)]
    pub v_ed_kn: f64,
    #[state(persistent)]
    pub h_ed_kn: f64,
    #[state(persistent)]
    pub z_mm3: f64,
    #[state(persistent)]
    pub area_mm2: f64,
    #[state(persistent)]
    pub shear_area_mm2: f64,
    #[state(persistent)]
    pub f_k_mpa: f64,
    #[state(persistent)]
    pub f_vk_mpa: f64,
    #[state(persistent)]
    pub annex: crate::document::AnnexChoice,
    #[state(persistent)]
    pub masonry_class: crate::document::MasonryClass,
    #[state(persistent)]
    pub design_situation: crate::document::DesignSituation,
    #[state(persistent)]
    pub mu: f64,
    #[state(persistent)]
    pub wall_thickness_mm: f64,
    #[state(persistent)]
    pub fire_resistance_min: u32,
    #[state(persistent)]
    pub unit: String,
    #[state(persistent)]
    pub exposure: crate::artifacts::en1996::part_2::ExposureClass,
    #[state(persistent)]
    pub mortar: crate::artifacts::en1996::part_2::MortarClass,
    #[state(persistent)]
    pub bed_joint_thickness_mm: f64,
    #[state(persistent)]
    pub storeys: u32,
    #[state(persistent)]
    pub h_ef_mm: f64,
    #[state(persistent)]
    pub t_ef_mm: f64,
}
//#endregion 🔖️Snapshot

//#region 🔖️HandcraftedEn1996SnapshotCodecs
/// ✉️ P6 handcrafted En1996SnapshotDsl/En1996SnapshotPack (derive no longer emits these traits).
impl store::En1996SnapshotDsl for En1996Snapshot {
    const EXTENSION: &'static str = "en1996";
    fn envelope_id() -> &'static str { "norm.en1996" }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = match store::semio_format::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = dsl::parse(
            body,
            &Self::__dsl_spec(),
            &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::En1996Snapshot },
        )?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::En1996Snapshot);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::En1996SnapshotDsl>::envelope_id(),
            store::semio_format::Component::Dsl,
            1,
        ).expect("valid envelope_id");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::En1996SnapshotPack for En1996Snapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let inner = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(
            <Self as store::En1996SnapshotDsl>::envelope_id(),
            store::semio_format::Component::Pack,
            1,
        ).map_err(|e| store::PackError::Schema(e.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, inner) = store::semio_format::unwrap_binary(bytes)
            .map_err(|e| store::PackError::Schema(e.to_string()))?;
        if envelope.envelope_id() != <Self as store::En1996SnapshotDsl>::envelope_id() {
            return Err(store::PackError::Schema(format!(
                "pack envelope mismatch: expected {}, got {}",
                <Self as store::En1996SnapshotDsl>::envelope_id(),
                envelope.envelope_id()
            )));
        }
        let (record, _report) = store::pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> { Some(Self::__dsl_spec()) }
}
//#endregion 🔖️HandcraftedEn1996SnapshotCodecs


impl Default for En1996Snapshot {
    fn default() -> Self {
        Self {
            m_ed_knm: 8.0,
            n_ed_kn: 200.0,
            v_ed_kn: 35.0,
            h_ed_kn: 20.0,
            z_mm3: 8_000_000.0,
            area_mm2: 500_000.0,
            shear_area_mm2: 300_000.0,
            f_k_mpa: 5.0,
            f_vk_mpa: 0.15,
            annex: AnnexChoice::De,
            masonry_class: MasonryClass::default(),
            design_situation: DesignSituation::Persistent,
            mu: 0.4,
            wall_thickness_mm: 240.0,
            fire_resistance_min: 60,
            unit: "clay".into(),
            exposure: part_2::ExposureClass::Mx1,
            mortar: part_2::MortarClass::M5,
            bed_joint_thickness_mm: 12.0,
            storeys: 2,
            h_ef_mm: 2500.0,
            t_ef_mm: 240.0,
        }
    }
}
