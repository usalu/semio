//! 🧱️ EN 1996 app — document entities (constitutional: general).

use crate::document::{AnnexChoice, DesignSituation};
use serde::{Deserialize, Serialize};

// #region 🔖️Types
/// 🧱️ Masonry manufacturing-control class underlying the EN-recommended γ_M table (EN 1996-1-1 Table 2.1-style).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
pub enum MasonryClass {
    Class1,
    Class2,
    #[default]
    Class3,
    Class4,
    Class5,
}

impl MasonryClass {
    pub fn gamma_m_en(self) -> f64 {
        match self {
            Self::Class1 => 1.5,
            Self::Class2 => 1.7,
            Self::Class3 => 2.0,
            Self::Class4 => 2.2,
            Self::Class5 => 2.5,
        }
    }
}

pub mod part_2 {
    use serde::{Deserialize, Serialize};

    /// 🌦️ Masonry durability exposure class (EN 1996-1-1 Annex B-style categorisation MX1–MX5).
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum ExposureClass {
        Mx1,
        Mx2,
        Mx3,
        Mx4,
        Mx5,
    }

    /// 🧪️ General-purpose mortar compressive-strength class per EN 998-2.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, dsl::DslScalar)]
    pub enum MortarClass {
        M1,
        /// 🔡️ `M2_5` auto-kebabs to `m2-5` (digit-underscore-digit), but the standard's own class
        /// label is `M2.5`/`M2_5` with no internal dash — kept as a genuine rename.
        #[dsl(key = "m2_5")]
        M2_5,
        M5,
        M10,
        M20,
    }

    impl MortarClass {
        pub fn compressive_strength_mpa(self) -> f64 {
            match self {
                Self::M1 => 1.0,
                Self::M2_5 => 2.5,
                Self::M5 => 5.0,
                Self::M10 => 10.0,
                Self::M20 => 20.0,
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, dsl::DslRecord)]
#[serde(rename_all = "camelCase")]
#[dsl(id = "norm.en1996", layout = "lines")]
pub struct Document {
    pub m_ed_knm: f64,
    #[dsl(unit = "kN")]
    pub n_ed_kn: f64,
    #[dsl(unit = "kN")]
    pub v_ed_kn: f64,
    #[dsl(unit = "kN")]
    pub h_ed_kn: f64,
    pub z_mm3: f64,
    #[dsl(unit = "mm2")]
    pub area_mm2: f64,
    #[dsl(unit = "mm2")]
    pub shear_area_mm2: f64,
    #[dsl(unit = "MPa")]
    pub f_k_mpa: f64,
    #[dsl(unit = "MPa")]
    pub f_vk_mpa: f64,
    pub annex: AnnexChoice,
    pub masonry_class: MasonryClass,
    pub design_situation: DesignSituation,
    pub mu: f64,
    #[dsl(unit = "mm")]
    pub wall_thickness_mm: f64,
    pub fire_resistance_min: u32,
    pub unit: String,
    pub exposure: part_2::ExposureClass,
    pub mortar: part_2::MortarClass,
    #[dsl(unit = "mm")]
    pub bed_joint_thickness_mm: f64,
    pub storeys: u32,
    #[dsl(unit = "mm")]
    pub h_ef_mm: f64,
    #[dsl(unit = "mm")]
    pub t_ef_mm: f64,
}
//#region 🔖️HandcraftedDocumentCodecs
/// ✉️ P6 handcrafted DocumentDsl/DocumentPack (derive no longer emits these traits).
impl store::DocumentDsl for Document {
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

impl store::DocumentPack for Document {
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




impl Default for Document {
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
// #endregion 🔖️Types

// `)` so the
/// artifact node, not the app, owns its own kind declaration.
pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    crate::app_surface::artifact_kind_spec("en1996", "EN 1996")
}
//#endregion 🔖️ArtifactKind
