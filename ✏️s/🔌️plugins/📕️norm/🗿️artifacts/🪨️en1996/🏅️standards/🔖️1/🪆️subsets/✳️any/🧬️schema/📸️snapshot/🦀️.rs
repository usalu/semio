//! 🧱 EN 1996 snapshot — masonry building subject (walls with units, mortar, actions).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::{AnnexChoice, DesignSituation};

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1996 masonry-building document.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1996", layout = "lines")]
#[artifact_schema(id = "s.norm.en1996")]
pub struct En1996Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[state(artifact)]
    pub masonry_class: MasonryClass,
    #[state(artifact)]
    pub design_situation: DesignSituation,
    #[state(artifact)]
    pub storeys: u32,
    #[dsl(table)]
    #[state(artifact)]
    pub walls: Vec<MasonryWall>,
}
//#endregion 🔖️Snapshot

impl Default for En1996Snapshot {
    fn default() -> Self {
        Self::compliant_clay_wall()
    }
}

impl En1996Snapshot {
    /// ✅️ Realistic compliant DE clay Group-1 wall (simplified path applicable).
    pub fn compliant_clay_wall() -> Self {
        Self {
            annex: AnnexChoice::De,
            masonry_class: MasonryClass::Class1,
            design_situation: DesignSituation::Persistent,
            storeys: 2,
            walls: vec![MasonryWall {
                id: "wall-north".into(),
                label_en: "North load-bearing wall".into(),
                label_de: "Tragende Nordwand".into(),
                wall_type: WallType::LoadBearing,
                thickness_m: 0.365,
                height_m: 2.75,
                length_m: 5.0,
                support_sides: 4,
                openings: vec![],
                slab_bearing_depth_m: 0.120,
                eccentricity_top_m: 0.020,
                eccentricity_bottom_m: 0.015,
                unit_group: UnitGroup::Group1,
                unit_material: UnitMaterial::Clay,
                f_b_pa: 20e6,
                unit_length_m: 0.240,
                unit_width_m: 0.365,
                unit_height_m: 0.113,
                mortar_type: MortarType::GeneralPurpose,
                mortar_class: MortarClass::M10,
                mortar_strength_pa: 10e6,
                bed_joint_thickness_m: 0.012,
                reinforced: false,
                as_vertical_m2: 0.0,
                as_horizontal_m2: 0.0,
                f_yd_pa: 0.0,
                fire_rei_min: 90,
                exposure: ExposureClass::Mx1,
                mu: 0.40,
                density_kg_m3: 1800.0,
                phi_infinity: 1.5,
                is_basement: false,
                load_cases: vec![WallLoadCase {
                    id: "ulsinz".into(),
                    design_situation: "persistent".into(),
                    imposed_category: "A".into(),
                    g_k_slab_n: 120_000.0,
                    q_k_imposed_pa: 2_000.0,
                    tributary_area_m2: 12.5,
                    slab_span_m: 4.5,
                    q_k_snow_pa: 0.0,
                    q_p_wind_pa: 800.0,
                    c_pe: 0.8,
                    h_k_earth_n: 0.0,
                    concentrated: vec![],
                }],
            }],
        }
    }

    /// ❌️ Non-compliant multi-failure subject (compression, shear, fire, durability, joint, slenderness).
    pub fn noncompliant_multi_fail() -> Self {
        Self {
            annex: AnnexChoice::De,
            masonry_class: MasonryClass::Class3,
            design_situation: DesignSituation::Persistent,
            storeys: 4,
            walls: vec![MasonryWall {
                id: "wall-weak".into(),
                label_en: "Undersized masonry wall".into(),
                label_de: "Unterbemessene Mauerwerkswand".into(),
                wall_type: WallType::Shear,
                thickness_m: 0.090,
                height_m: 3.20,
                length_m: 2.50,
                support_sides: 2,
                openings: vec![],
                slab_bearing_depth_m: 0.050,
                eccentricity_top_m: 0.040,
                eccentricity_bottom_m: 0.030,
                unit_group: UnitGroup::Group2,
                unit_material: UnitMaterial::Aerated,
                f_b_pa: 4e6,
                unit_length_m: 0.500,
                unit_width_m: 0.090,
                unit_height_m: 0.250,
                mortar_type: MortarType::GeneralPurpose,
                mortar_class: MortarClass::M2_5,
                mortar_strength_pa: 2.5e6,
                bed_joint_thickness_m: 0.004,
                reinforced: false,
                as_vertical_m2: 0.0,
                as_horizontal_m2: 0.0,
                f_yd_pa: 0.0,
                fire_rei_min: 90,
                exposure: ExposureClass::Mx4,
                mu: 0.40,
                density_kg_m3: 600.0,
                phi_infinity: 1.5,
                is_basement: false,
                load_cases: vec![WallLoadCase {
                    id: "uls-bad".into(),
                    design_situation: "persistent".into(),
                    imposed_category: "C".into(),
                    g_k_slab_n: 280_000.0,
                    q_k_imposed_pa: 5_000.0,
                    tributary_area_m2: 20.0,
                    slab_span_m: 7.0,
                    q_k_snow_pa: 1_500.0,
                    q_p_wind_pa: 1_200.0,
                    c_pe: 1.0,
                    h_k_earth_n: 0.0,
                    concentrated: vec![crate::ConcentratedLoad {
                        id: "beam-A".into(),
                        force_n: 80_000.0,
                        bearing_area_m2: 0.01,
                        bearing_length_m: 0.10,
                    }],
                }],
            }],
        }
    }

    /// 🪟 Scope example: partial window + full-height pier opening (§5.5.1.4).
    pub fn opening_wall_example() -> Self {
        let mut doc = Self::compliant_clay_wall();
        doc.walls[0].id = "wall-openings".into();
        doc.walls[0].label_en = "Wall with window and door pier".into();
        doc.walls[0].label_de = "Wand mit Fenster und Türöffnung".into();
        doc.walls[0].openings = vec![
            crate::WallOpening {
                id: "win-1".into(),
                width_m: 1.20,
                height_m: 1.40,
                sill_height_m: 0.90,
            },
            crate::WallOpening {
                id: "door-1".into(),
                width_m: 1.00,
                height_m: 2.60,
                sill_height_m: 0.0,
            },
        ];
        doc
    }

    /// 🏗️ Scope example: basement wall with earth pressure (EN 1996-3 §4.5).
    pub fn basement_wall_example() -> Self {
        let mut doc = Self::compliant_clay_wall();
        doc.walls[0].id = "wall-basement".into();
        doc.walls[0].label_en = "Basement retaining wall".into();
        doc.walls[0].label_de = "Kelleraußenwand".into();
        doc.walls[0].is_basement = true;
        doc.walls[0].height_m = 2.50;
        doc.walls[0].load_cases[0].id = "uls-earth".into();
        doc.walls[0].load_cases[0].h_k_earth_n = 45_000.0;
        doc
    }

    /// 🏋️ Scope example: load case with concentrated beam bearing (§6.1.3).
    pub fn concentrated_load_example() -> Self {
        let mut doc = Self::compliant_clay_wall();
        doc.walls[0].id = "wall-concentrated".into();
        doc.walls[0].label_en = "Wall with concentrated beam load".into();
        doc.walls[0].label_de = "Wand mit Einzellast aus Träger".into();
        doc.walls[0].load_cases[0].concentrated = vec![crate::ConcentratedLoad {
            id: "beam-B".into(),
            force_n: 35_000.0,
            bearing_area_m2: 0.04,
            bearing_length_m: 0.20,
        }];
        doc
    }

    /// 🔩 Scope example: reinforced masonry panel (§6.6).
    pub fn reinforced_wall_example() -> Self {
        let mut doc = Self::compliant_clay_wall();
        doc.walls[0].id = "wall-reinforced".into();
        doc.walls[0].label_en = "Reinforced masonry wall".into();
        doc.walls[0].label_de = "Bewehrte Mauerwerkswand".into();
        doc.walls[0].reinforced = true;
        doc.walls[0].as_vertical_m2 = 6.0e-4;
        doc.walls[0].as_horizontal_m2 = 4.0e-4;
        doc.walls[0].f_yd_pa = 435e6;
        doc
    }

}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

/// 🧱️ Masonry manufacturing-control class underlying the EN-recommended γ_M table (EN 1996-1-1 Table 2.3).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum MasonryClass {
    Class1,
    Class2,
    #[default]
    Class3,
    Class4,
    Class5,
}

impl MasonryClass {
    /// ⚖️ EN-recommended γ_M by category of execution/manufacturing control.
    pub fn gamma_m_en(self) -> f64 {
        match self {
            Self::Class1 => 1.5,
            Self::Class2 => 1.7,
            Self::Class3 => 2.0,
            Self::Class4 => 2.2,
            Self::Class5 => 2.5,
        }
    }

    /// ⚖️ DIN EN 1996-1-1/NA γ_M from manufacturing category × execution class.
    /// Class1→Cat I+A, Class2→Cat I+B, Class3→Cat II+A, Class4/5→Cat II+B.
    pub fn gamma_m_de(self, accidental: bool) -> f64 {
        if accidental {
            return 1.3;
        }
        match self {
            Self::Class1 => 1.5,
            Self::Class2 => 1.7,
            Self::Class3 => 1.7,
            Self::Class4 | Self::Class5 => 2.0,
        }
    }
}

/// 🌦️ Masonry durability exposure class (EN 1996-2 / Annex B MX1–MX5).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum ExposureClass {
    Mx1,
    Mx2,
    Mx3,
    Mx4,
    Mx5,
}

/// 🧱 Masonry unit material.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum UnitMaterial {
    Clay,
    CalciumSilicate,
    Aerated,
    Concrete,
}

/// 🧱 Unit group per EN 771 / EN 1996-1-1 (Group 1–4).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum UnitGroup {
    Group1,
    Group2,
    Group3,
    Group4,
}

/// 🧈 Mortar type per EN 998-2 / EN 1996-1-1.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum MortarType {
    GeneralPurpose,
    ThinLayer,
    Lightweight,
}

/// 🧈 Mortar compressive-strength class (EN 998-2).
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum MortarClass {
    M1,
    #[dsl(key = "m2_5")]
    M2_5,
    M5,
    M10,
    M15,
    M20,
}

impl MortarClass {
    /// 🗜️ Characteristic compressive strength f_m [MPa] of the mortar class.
    pub fn f_m_mpa(self) -> f64 {
        match self {
            Self::M1 => 1.0,
            Self::M2_5 => 2.5,
            Self::M5 => 5.0,
            Self::M10 => 10.0,
            Self::M15 => 15.0,
            Self::M20 => 20.0,
        }
    }
    /// ♻️ Alias used by older durability helpers.
    pub fn compressive_strength_mpa(self) -> f64 { self.f_m_mpa() }
}

/// 🧱 Wall structural role.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
pub enum WallType {
    #[dsl(key = "load-bearing")]
    LoadBearing,
    Shear,
    #[dsl(key = "non-load-bearing")]
    NonLoadBearing,
}

/// 🪟 Opening in a masonry wall — SI base units.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WallOpening {
    pub id: String,
    #[dsl(unit = "m")]
    pub width_m: f64,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub sill_height_m: f64,
}

/// 🏋️ Characteristic concentrated action on a masonry wall — SI base units.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ConcentratedLoad {
    pub id: String,
    /// 🧲️ Characteristic concentrated force F_k [N].
    #[dsl(unit = "N")]
    pub force_n: f64,
    #[dsl(unit = "m2")]
    pub bearing_area_m2: f64,
    #[dsl(unit = "m")]
    pub bearing_length_m: f64,
}

/// 📜️ Characteristic actions for one wall load case — EN 1990 combinations formed in evaluate().
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct WallLoadCase {
    pub id: String,
    /// 🎨️ persistent | transient | accidental | seismic
    pub design_situation: String,
    /// 📌️ Imposed-floor category (A/B/C/D/E/H/…) for ψ factors (EN 1990 Table A1.1 / DE NA).
    pub imposed_category: String,
    /// 🟫️ Characteristic permanent vertical load from slabs/floors onto this wall [N] (excl. wall self-weight).
    #[dsl(unit = "N")]
    pub g_k_slab_n: f64,
    /// 🔒️ Characteristic imposed floor load intensity [Pa].
    #[dsl(unit = "Pa")]
    pub q_k_imposed_pa: f64,
    /// 📐️ Tributary floor area for imposed / snow vertical load [m²].
    #[dsl(unit = "m2")]
    pub tributary_area_m2: f64,
    /// ⬜️ Clear slab span [m] (eccentricity + EN 1996-3/NA Table NA.A.1 ≤ 6 m).
    #[dsl(unit = "m")]
    pub slab_span_m: f64,
    /// 🌬️ Characteristic snow load intensity on tributary [Pa].
    #[dsl(unit = "Pa")]
    pub q_k_snow_pa: f64,
    /// ❄️ Peak velocity pressure q_p [Pa] (EN 1991-1-4).
    #[dsl(unit = "Pa")]
    pub q_p_wind_pa: f64,
    /// 🔗️ External pressure coefficient c_pe [-] (EN 1991-1-4).
    pub c_pe: f64,
    /// 🗺️ Characteristic horizontal earth-pressure resultant on basement wall [N].
    #[dsl(unit = "N")]
    pub h_k_earth_n: f64,
    #[dsl(table)]
    pub concentrated: Vec<ConcentratedLoad>,
}

/// 🧱 Masonry wall subject entity — SI base units throughout.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct MasonryWall {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub wall_type: WallType,
    #[dsl(unit = "m")]
    pub thickness_m: f64,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub length_m: f64,
    pub support_sides: u8,
    #[dsl(table)]
    pub openings: Vec<WallOpening>,
    #[dsl(unit = "m")]
    pub slab_bearing_depth_m: f64,
    #[dsl(unit = "m")]
    pub eccentricity_top_m: f64,
    #[dsl(unit = "m")]
    pub eccentricity_bottom_m: f64,
    pub unit_group: UnitGroup,
    pub unit_material: UnitMaterial,
    #[dsl(unit = "Pa")]
    pub f_b_pa: f64,
    #[dsl(unit = "m")]
    pub unit_length_m: f64,
    #[dsl(unit = "m")]
    pub unit_width_m: f64,
    #[dsl(unit = "m")]
    pub unit_height_m: f64,
    pub mortar_type: MortarType,
    pub mortar_class: MortarClass,
    /// 🔆️ Declared mortar compressive strength (SI Pa) — must match `mortarClass` within tolerance.
    #[dsl(unit = "Pa")]
    pub mortar_strength_pa: f64,
    #[dsl(unit = "m")]
    pub bed_joint_thickness_m: f64,
    pub reinforced: bool,
    #[dsl(unit = "m2")]
    pub as_vertical_m2: f64,
    #[dsl(unit = "m2")]
    pub as_horizontal_m2: f64,
    #[dsl(unit = "Pa")]
    pub f_yd_pa: f64,
    pub fire_rei_min: u32,
    pub exposure: ExposureClass,
    /// 🔢️ Friction coefficient μ for sliding (EN 1996-1-1 §6.2 / DE NA).
    pub mu: f64,
    /// 🔋️ Masonry bulk density for self-weight [kg/m³].
    pub density_kg_m3: f64,
    /// 🏁️ Final creep coefficient φ_∞ (EN 1996-1-1 Table 3.3 / DE NA) used in e_k.
    pub phi_infinity: f64,
    /// 🚧️ Basement wall (EN 1996-3/NA earth-pressure path).
    pub is_basement: bool,
    #[dsl(table)]
    pub load_cases: Vec<WallLoadCase>,
}

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1996", "EN 1996", EN1996_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1996_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1996", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1996_DOCUMENT_SCHEMA: &str = "semio.norm.en1996/v1";
