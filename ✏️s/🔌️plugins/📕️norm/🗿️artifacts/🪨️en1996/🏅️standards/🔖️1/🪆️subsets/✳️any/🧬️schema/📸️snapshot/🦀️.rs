//! 🧱 EN 1996 snapshot — masonry building subject (walls with units, mortar, actions).

use crate::document::{AnnexChoice, DesignSituation};
use crate::{MasonryClass, MasonryWall, MortarClass, MortarType, UnitGroup, UnitMaterial, WallLoadCase, WallType, ExposureClass};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1996 masonry-building document.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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

crate::impl_norm_artifact_record!(En1996Snapshot, extension = "en1996", envelope_id = "norm.en1996");

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
        doc.walls[0].load_cases[0].q_p_wind_pa = 0.0;
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
        doc.walls[0].as_vertical_m2 = 2.5e-4;
        doc.walls[0].as_horizontal_m2 = 1.5e-4;
        doc.walls[0].f_yd_pa = 435e6;
        doc
    }

}

//#region 🌉️ExternalCodecBridge
pub fn encode_en1996_snapshot_json(snapshot: &En1996Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}
pub fn decode_en1996_snapshot_json(text: &str) -> Result<En1996Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}
pub fn decode_en1996_dsl(text: &str) -> Result<En1996Snapshot, String> {
    <En1996Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1996_dsl(snapshot: &En1996Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
pub fn decode_en1996_pack(bytes: &[u8]) -> Result<En1996Snapshot, String> {
    <En1996Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1996_pack(snapshot: &En1996Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
