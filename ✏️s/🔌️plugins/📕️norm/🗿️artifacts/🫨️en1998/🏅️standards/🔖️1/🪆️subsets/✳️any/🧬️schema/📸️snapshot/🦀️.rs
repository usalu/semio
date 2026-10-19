//! 🌋️ EN 1998 snapshot — complete scoped seismic subject (buildings + parts 2–6).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1998 project snapshot.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1998", layout = "lines")]
#[artifact_schema(id = "s.norm.en1998")]
pub struct En1998Snapshot {
    #[state(artifact)]
    pub annex: String,
    #[state(artifact)]
    pub site: En1998Site,
    #[dsl(table)]
    #[state(artifact)]
    pub buildings: Vec<En1998Building>,
    #[dsl(table)]
    #[state(artifact)]
    pub bridges: Vec<En1998Bridge>,
    #[dsl(table)]
    #[state(artifact)]
    pub assessments: Vec<En1998Assessment>,
    #[dsl(table)]
    #[state(artifact)]
    pub silos: Vec<En1998Silo>,
    #[dsl(table)]
    #[state(artifact)]
    pub tanks: Vec<En1998Tank>,
    #[dsl(table)]
    #[state(artifact)]
    pub foundations: Vec<En1998Foundation>,
    #[dsl(table)]
    #[state(artifact)]
    pub retaining_walls: Vec<En1998RetainingWall>,
    #[dsl(table)]
    #[state(artifact)]
    pub towers: Vec<En1998Tower>,
}
//#endregion 🔖️Snapshot

impl Default for En1998Snapshot {
    fn default() -> Self {
        Self::compliant_de_office()
    }
}

impl En1998Snapshot {
    /// 🏢 Realistic compliant DE office (zone 2, B-R, RC DCH frame, 4 storeys).
    pub fn compliant_de_office() -> Self {
        Self {
            annex: "de".into(),
            site: En1998Site {
                seismic_zone: DeSeismicZone::Zone2,
                a_gr: 0.6,
                de_ground_combo: DeGroundCombo::BR,
                en_ground_type: String::new(),
                en_spectrum_type: String::new(),
                importance_class: "II".into(),
            },
            buildings: vec![En1998Building {
                id: "bldg-office".into(),
                name: "Office RC frame".into(),
                plan_width_m: 24.0,
                plan_length_m: 16.0,
                systems: vec![
                    En1998System {
                        id: "sys-x".into(),
                        direction: "x".into(),
                        system_type: "frame".into(),
                        material: "rc".into(),
                        ductility_class: "dch".into(),
                        q0: 4.5,
                        alpha_u_over_alpha_1: 1.3,
                        k_w: 1.0,
                        base_shear_resistance_n: 1_200_000.0,
                    },
                    En1998System {
                        id: "sys-y".into(),
                        direction: "y".into(),
                        system_type: "frame".into(),
                        material: "rc".into(),
                        ductility_class: "dch".into(),
                        q0: 4.5,
                        alpha_u_over_alpha_1: 1.3,
                        k_w: 1.0,
                        base_shear_resistance_n: 1_200_000.0,
                    },
                ],
                storeys: vec![
                    En1998Storey { id: "s1".into(), height_m: 3.5, permanent_gk_n: 2334780.000000, correlated_occupancy: true, variables: vec![En1998VariableAction { id: "s1-q".into(), category: "B".into(), qk_n: 1716750.000000 }], stiffness_x: 180000000.0, stiffness_y: 180000000.0, centre_of_mass_x_m: 12.0, centre_of_mass_y_m: 8.0, centre_of_stiffness_x_m: 12.0, centre_of_stiffness_y_m: 8.0, drift_x_m: 0.008, drift_y_m: 0.008, shear_resistance_n: 400000.0 },
                    En1998Storey { id: "s2".into(), height_m: 3.5, permanent_gk_n: 2251395.000000, correlated_occupancy: true, variables: vec![En1998VariableAction { id: "s2-q".into(), category: "B".into(), qk_n: 1655437.500000 }], stiffness_x: 170000000.0, stiffness_y: 170000000.0, centre_of_mass_x_m: 12.0, centre_of_mass_y_m: 8.0, centre_of_stiffness_x_m: 12.0, centre_of_stiffness_y_m: 8.0, drift_x_m: 0.009, drift_y_m: 0.009, shear_resistance_n: 400000.0 },
                    En1998Storey { id: "s3".into(), height_m: 3.5, permanent_gk_n: 2168010.000000, correlated_occupancy: true, variables: vec![En1998VariableAction { id: "s3-q".into(), category: "B".into(), qk_n: 1594125.000000 }], stiffness_x: 160000000.0, stiffness_y: 160000000.0, centre_of_mass_x_m: 12.0, centre_of_mass_y_m: 8.0, centre_of_stiffness_x_m: 12.0, centre_of_stiffness_y_m: 8.0, drift_x_m: 0.01, drift_y_m: 0.01, shear_resistance_n: 400000.0 },
                    En1998Storey { id: "s4".into(), height_m: 3.5, permanent_gk_n: 2001240.000000, correlated_occupancy: true, variables: vec![En1998VariableAction { id: "s4-q".into(), category: "B".into(), qk_n: 1177200.000000 }], stiffness_x: 150000000.0, stiffness_y: 150000000.0, centre_of_mass_x_m: 12.0, centre_of_mass_y_m: 8.0, centre_of_stiffness_x_m: 12.0, centre_of_stiffness_y_m: 8.0, drift_x_m: 0.011, drift_y_m: 0.011, shear_resistance_n: 400000.0 },
                ],
                members: vec![
                    En1998Member { id: "col-c1".into(), material: "rc".into(), role: "column".into(), detailing_compatible_with_q: true, min_dimension_m: 0.35, rho: 0.012, rho_prime: 0.0, omega_wd: 0.12, steel_section_class: 0 },
                    En1998Member { id: "beam-b1".into(), material: "rc".into(), role: "beam".into(), detailing_compatible_with_q: true, min_dimension_m: 0.30, rho: 0.010, rho_prime: 0.005, omega_wd: 0.08, steel_section_class: 0 },
                ],
                plan_regular: true,
                elevation_regular: true,
                t1_method: "ct".into(),
                t1_given_s: 0.0,
                ct: 0.075,
                drift_limit_class: "ductile".into(),
                nu: 0.5,
                multiple_resisting_systems: false,
                claims_simple_masonry: false,
                masonry_wall_area_ratio: 0.0,
                accidental_eccentricity_ratio: 0.05,
            }],
            bridges: vec![],
            assessments: vec![],
            silos: vec![],
            tanks: vec![],
            foundations: vec![],
            retaining_walls: vec![],
            towers: vec![],
        }
    }

    /// ⚠️ Non-compliant multi-failure DE office (weak capacity, large drift, irregular).
    pub fn noncompliant_de_office() -> Self {
        let mut s = Self::compliant_de_office();
        let b = &mut s.buildings[0];
        b.id = "bldg-weak".into();
        b.name = "Weak RC frame".into();
        b.plan_regular = false;
        b.elevation_regular = false;
        b.multiple_resisting_systems = false;
        for sys in &mut b.systems {
            sys.base_shear_resistance_n = 80_000.0;
            sys.ductility_class = "dcm".into();
            sys.q0 = 3.0;
        }
        for st in &mut b.storeys {
            st.drift_x_m = 0.045;
            st.drift_y_m = 0.045;
            st.stiffness_x *= 0.25;
            st.stiffness_y *= 0.25;
        }
        for m in &mut b.members {
            m.detailing_compatible_with_q = false;
            m.min_dimension_m = 0.20;
            m.rho = 0.002;
            m.rho_prime = 0.0;
            m.omega_wd = 0.02;
        }
        for st in &mut b.storeys {
            st.shear_resistance_n = 50_000.0;
        }
        s.site.a_gr = 0.6; // matches zone 2
        s
    }

    /// 🏗️ Compliant DE subject with populated EN 1998 parts 2–6 (for scope-aware leaf tests).
    pub fn compliant_de_multipart() -> Self {
        let mut s = Self::compliant_de_office();
        s.bridges = vec![En1998Bridge {
            id: "br-1".into(),
            period_ratio: 2.0,
            fundamental_period_s: 1.2,
            v_rd_n: 2.0e6,
            bearing_d_rd_m: 0.40,
            permanent_gk_n: 8.0e6,
            correlated_occupancy: true,
            variables: vec![En1998VariableAction { id: "br-q1".into(), category: "F".into(), qk_n: 1.5e6 }],
        }];
        s.assessments = vec![En1998Assessment {
            id: "as-1".into(),
            knowledge_level: "kl2".into(),
            limit_state: "sd".into(),
            supported_building_id: "bldg-office".into(),
            r_k_n: 5.0e5,
            gamma_el: 1.0,
        }];
        s.silos = vec![En1998Silo {
            id: "si-1".into(),
            height_m: 12.0,
            radius_m: 4.0,
            permanent_gk_n: 1.2e6,
            content_qk_n: 4.0e6,
            content_category: "E".into(),
            filling_ratio: 0.85,
            n_rd_n: 2.0e6,
            v_rd_n: 1.5e6,
            q_nominal: 1.2,
        }];
        s.tanks = vec![En1998Tank {
            id: "tk-1".into(),
            height_m: 10.0,
            radius_m: 5.0,
            permanent_gk_n: 8.0e5,
            content_qk_n: 5.0e6,
            content_category: "E".into(),
            filling_ratio: 0.90,
            v_rd_n: 3.0e6,
        }];
        s.foundations = vec![En1998Foundation {
            id: "fd-1".into(),
            supported_building_id: "bldg-office".into(),
            area_m2: 200.0,
            p_rd_pa: 4.0e5,
            h_rd_n: 5.0e5,
            k_foundation: 8.0e5,
            k_soil: 3.0e5,
        }];
        s.retaining_walls = vec![En1998RetainingWall {
            id: "rw-1".into(),
            height_m: 4.5,
            phi_deg: 32.0,
            soil_gamma: 18000.0,
            r: 1.5,
            h_rd_n_per_m: 2.5e5,
        }];
        s.towers = vec![En1998Tower {
            id: "tw-1".into(),
            height_m: 45.0,
            m_rd_nm: 8.0e6,
            is_chimney: false,
            q_nominal: 2.5,
            permanent_gk_n: 1.0e6,
            correlated_occupancy: true,
            variables: vec![En1998VariableAction { id: "tw-q1".into(), category: "E".into(), qk_n: 5.0e4 }],
        }];
        s
    }

    /// ⚠️ Non-compliant multi-part DE subject (weak capacities across parts 2–6).
    pub fn noncompliant_de_multipart() -> Self {
        let mut s = Self::noncompliant_de_office();
        s.bridges = vec![En1998Bridge {
            id: "br-weak".into(),
            period_ratio: 1.2,
            fundamental_period_s: 1.5,
            v_rd_n: 5.0e4,
            bearing_d_rd_m: 0.02,
            permanent_gk_n: 8.0e6,
            correlated_occupancy: true,
            variables: vec![En1998VariableAction { id: "br-q1".into(), category: "F".into(), qk_n: 1.5e6 }],
        }];
        s.assessments = vec![En1998Assessment {
            id: "as-weak".into(),
            knowledge_level: "kl1".into(),
            limit_state: "nc".into(),
            supported_building_id: "bldg-weak".into(),
            r_k_n: 2.0e4,
            gamma_el: 1.2,
        }];
        s.silos = vec![En1998Silo {
            id: "si-weak".into(),
            height_m: 14.0,
            radius_m: 3.5,
            permanent_gk_n: 1.2e6,
            content_qk_n: 5.0e6,
            content_category: "E".into(),
            filling_ratio: 1.0,
            n_rd_n: 5.0e4,
            v_rd_n: 4.0e4,
            q_nominal: 1.5,
        }];
        s.tanks = vec![En1998Tank {
            id: "tk-weak".into(),
            height_m: 12.0,
            radius_m: 4.0,
            permanent_gk_n: 8.0e5,
            content_qk_n: 6.0e6,
            content_category: "E".into(),
            filling_ratio: 1.0,
            v_rd_n: 5.0e4,
        }];
        s.foundations = vec![En1998Foundation {
            id: "fd-weak".into(),
            supported_building_id: "bldg-weak".into(),
            area_m2: 40.0,
            p_rd_pa: 8.0e4,
            h_rd_n: 2.0e4,
            k_foundation: 2.0e5,
            k_soil: 1.0e5,
        }];
        s.retaining_walls = vec![En1998RetainingWall {
            id: "rw-weak".into(),
            height_m: 6.0,
            phi_deg: 25.0,
            soil_gamma: 19000.0,
            r: 1.0,
            h_rd_n_per_m: 2.0e4,
        }];
        s.towers = vec![En1998Tower {
            id: "tw-weak".into(),
            height_m: 55.0,
            m_rd_nm: 1.0e5,
            is_chimney: true,
            q_nominal: 1.5,
            permanent_gk_n: 1.5e6,
            correlated_occupancy: true,
            variables: vec![],
        }];
        s
    }

    /// 🏢 Compliant EN annex office (ground B, type 1) — exercises enGroundType / enSpectrumType.
    pub fn compliant_en_office() -> Self {
        let mut s = Self::compliant_de_office();
        s.annex = "en".into();
        s.site.a_gr = 0.6;
        s.site.en_ground_type = "B".into();
        s.site.en_spectrum_type = "type1".into();
        s
    }

    /// ⚠️ Failing EN annex office (soft spectrum / weak resistance).
    pub fn noncompliant_en_office() -> Self {
        let mut s = Self::noncompliant_de_office();
        s.annex = "en".into();
        s.site.a_gr = 0.8;
        s.site.en_ground_type = "D".into();
        s.site.en_spectrum_type = "type1".into();
        s
    }

    /// 🌀 Compliant DE office with plan-regular torsion path (small e₀).
    pub fn compliant_de_torsion_regular() -> Self {
        let mut s = Self::compliant_de_office();
        for st in &mut s.buildings[0].storeys {
            st.centre_of_mass_x_m = 12.0;
            st.centre_of_mass_y_m = 8.0;
            st.centre_of_stiffness_x_m = 12.0;
            st.centre_of_stiffness_y_m = 8.0;
            st.stiffness_x = 8.0e7;
            st.stiffness_y = 8.0e7;
        }
        s.buildings[0].accidental_eccentricity_ratio = 0.05;
        s.buildings[0].plan_regular = true;
        s
    }

    /// ⚠️ Failing DE office — large CM/CS eccentricity (torsion / plan-irregularity governed).
    pub fn noncompliant_de_torsion_irregular() -> Self {
        let mut s = Self::compliant_de_office();
        s.buildings[0].plan_regular = false;
        s.buildings[0].accidental_eccentricity_ratio = 0.12;
        for st in &mut s.buildings[0].storeys {
            st.centre_of_mass_x_m = 20.0;
            st.centre_of_mass_y_m = 14.0;
            st.centre_of_stiffness_x_m = 4.0;
            st.centre_of_stiffness_y_m = 3.0;
            st.stiffness_x = 5.0e6;
            st.stiffness_y = 5.0e6;
        }
        // Soft upper storey for elevation irregularity
        if let Some(top) = s.buildings[0].storeys.last_mut() {
            top.stiffness_x = 5.0e5;
            top.stiffness_y = 5.0e5;
        }
        s
    }

}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1998", "EN 1998", EN1998_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1998_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1998", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1998_DOCUMENT_SCHEMA: &str = "semio.norm.en1998/v1";

/// 🌩 DE seismic zone per DIN EN 1998-1/NA (zones 0–3 only; schema-restricted).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum DeSeismicZone {
    #[dsl(key = "zone0")]
    Zone0,
    #[dsl(key = "zone1")]
    Zone1,
    #[dsl(key = "zone2")]
    Zone2,
    #[dsl(key = "zone3")]
    Zone3,
}

impl DeSeismicZone {
    /// 📐️ Reference PGA a_gR [m/s²] per DIN EN 1998-1/NA Table NA.1 (2011).
    pub fn a_gr(self) -> f64 {
        match self {
            Self::Zone0 => 0.0,
            Self::Zone1 => 0.4,
            Self::Zone2 => 0.6,
            Self::Zone3 => 0.8,
        }
    }

    pub fn as_u8(self) -> u8 {
        match self {
            Self::Zone0 => 0,
            Self::Zone1 => 1,
            Self::Zone2 => 2,
            Self::Zone3 => 3,
        }
    }
}

/// 🪨 DE-NA ground/subsoil combination (DIN EN 1998-1/NA Table NA.4) — sole DE ground subject field.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum DeGroundCombo {
    #[dsl(key = "A-R")]
    #[value(rename = "A-R")]
    #[cfg_attr(test, serde(rename = "A-R"))]
    AR,
    #[dsl(key = "B-R")]
    #[value(rename = "B-R")]
    #[cfg_attr(test, serde(rename = "B-R"))]
    BR,
    #[dsl(key = "C-R")]
    #[value(rename = "C-R")]
    #[cfg_attr(test, serde(rename = "C-R"))]
    CR,
    #[dsl(key = "B-T")]
    #[value(rename = "B-T")]
    #[cfg_attr(test, serde(rename = "B-T"))]
    BT,
    #[dsl(key = "C-T")]
    #[value(rename = "C-T")]
    #[cfg_attr(test, serde(rename = "C-T"))]
    CT,
    #[dsl(key = "C-S")]
    #[value(rename = "C-S")]
    #[cfg_attr(test, serde(rename = "C-S"))]
    CS,
}

impl DeGroundCombo {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AR => "A-R",
            Self::BR => "B-R",
            Self::CR => "C-R",
            Self::BT => "B-T",
            Self::CT => "C-T",
            Self::CS => "C-S",
        }
    }
}

/// 🌚️ Seismic site parameters (DE zones 0–3 + DE-NA ground/subsoil, or EN A–E spectrum).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Site {
    pub seismic_zone: DeSeismicZone,
    /// 📏️ EN annex reference PGA [m/s²]. For DE annex, evaluate derives a_gR from `seismic_zone` (Table NA.1).
    #[dsl(unit = "m/s2")]
    pub a_gr: f64,
    pub de_ground_combo: DeGroundCombo,
    /// 🏛️ EN annex ground type A–E (EN 1998-1 Table 3.1). Empty under DE annex.
    #[cfg_attr(test, serde(default, skip_serializing_if = "String::is_empty"))]
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub en_ground_type: String,
    /// 📜️ EN annex spectrum type (`type1` | `type2`). Empty under DE annex.
    #[cfg_attr(test, serde(default, skip_serializing_if = "String::is_empty"))]
    #[value(default, skip_serializing_if = "String::is_empty")]
    pub en_spectrum_type: String,
    pub importance_class: String,
}

/// 🏗️ Lateral-force-resisting system for one horizontal direction.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998System {
    pub id: String,
    pub direction: String,
    pub system_type: String,
    pub material: String,
    pub ductility_class: String,
    pub q0: f64,
    pub alpha_u_over_alpha_1: f64,
    pub k_w: f64,
    #[dsl(unit = "N")]
    pub base_shear_resistance_n: f64,
}

/// 🏋️ Variable action Q_k,i — category selects ψ₂ (EN 1990 DE NA); φ from EN 1998-1 Table 4.2.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998VariableAction {
    pub id: String,
    /// 🚦️ Imposed-load category (A–H / snow / wind) for ψ₂.
    pub category: String,
    #[dsl(unit = "N")]
    pub qk_n: f64,
}

/// 🏬 Storey — seismic mass from ΣG_k + Σψ_E,i·Q_k,i (EN 1998-1 §3.2.4 / EN 1990 6.12b).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Storey {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    /// 🎮️ Characteristic permanent action ΣG_k [N] on the storey.
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// 🧑️ EN 1998-1 Table 4.2 correlated occupancies (φ = 0.8 storeys / 1.0 roof).
    pub correlated_occupancy: bool,
    #[dsl(table)]
    pub variables: Vec<En1998VariableAction>,
    pub stiffness_x: f64,
    pub stiffness_y: f64,
    #[dsl(unit = "m")]
    pub centre_of_mass_x_m: f64,
    #[dsl(unit = "m")]
    pub centre_of_mass_y_m: f64,
    #[dsl(unit = "m")]
    pub centre_of_stiffness_x_m: f64,
    #[dsl(unit = "m")]
    pub centre_of_stiffness_y_m: f64,
    #[dsl(unit = "m")]
    pub drift_x_m: f64,
    #[dsl(unit = "m")]
    pub drift_y_m: f64,
    /// 🪢️ Per-storey shear resistance V_Rd,i [N] for storey-force verification.
    #[dsl(unit = "N")]
    pub shear_resistance_n: f64,
}

/// 🧱 Member summary where material ductility rules apply.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Member {
    pub id: String,
    pub material: String,
    pub role: String,
    pub detailing_compatible_with_q: bool,
    /// ↔️ RC column min cross-section dimension b [m] (§5.4.3 / §5.5.3).
    #[dsl(unit = "m")]
    pub min_dimension_m: f64,
    /// 🔍️ RC longitudinal reinforcement ratio ρ.
    pub rho: f64,
    /// 🗜️ RC compression reinforcement ratio ρ′ (beams).
    pub rho_prime: f64,
    /// ⚙️ RC confinement mechanical volumetric ratio ω_wd.
    pub omega_wd: f64,
    /// 🔩️ Steel cross-section class (1–4) for q-compatibility (§6.5 / §6.6).
    pub steel_section_class: u8,
}

/// 🏢 Building subject for EN 1998-1.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Building {
    pub id: String,
    pub name: String,
    /// 🗓️ Plan dimension Lx [m] for accidental eccentricity (§4.3.2).
    #[dsl(unit = "m")]
    pub plan_width_m: f64,
    /// 📋️ Plan dimension Ly [m] for accidental eccentricity (§4.3.2).
    #[dsl(unit = "m")]
    pub plan_length_m: f64,
    #[dsl(table)]
    pub systems: Vec<En1998System>,
    #[dsl(table)]
    pub storeys: Vec<En1998Storey>,
    #[dsl(table)]
    pub members: Vec<En1998Member>,
    pub plan_regular: bool,
    pub elevation_regular: bool,
    pub t1_method: String,
    pub t1_given_s: f64,
    pub ct: f64,
    pub drift_limit_class: String,
    pub nu: f64,
    pub multiple_resisting_systems: bool,
    pub claims_simple_masonry: bool,
    pub masonry_wall_area_ratio: f64,
    pub accidental_eccentricity_ratio: f64,
}

/// 🌉 Bridge subject for EN 1998-2.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Bridge {
    pub id: String,
    /// ⏱️ Isolation period ratio T_isol/T_fixed (EN 1998-2).
    pub period_ratio: f64,
    /// 🔁️ Fundamental period T [s] for displacement / shear (EN 1998-2 §4.2).
    #[dsl(unit = "s")]
    pub fundamental_period_s: f64,
    #[dsl(unit = "N")]
    pub v_rd_n: f64,
    #[dsl(unit = "m")]
    pub bearing_d_rd_m: f64,
    /// ⌨️ Characteristic permanent action ΣG_k [N] (deck + self-weight).
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// 👥️ EN 1998-1 Table 4.2 correlated occupancy flag for φ on variable actions.
    pub correlated_occupancy: bool,
    #[dsl(table)]
    pub variables: Vec<En1998VariableAction>,
}

/// 🔧 Retrofit assessment for EN 1998-3.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Assessment {
    pub id: String,
    pub knowledge_level: String,
    /// 🎚️ EN 1998-3 limit state: nc | sd | dl (Table 2.1 + DE NA).
    pub limit_state: String,
    /// 🪝️ Building id providing seismic demand (spectrum × mass model).
    pub supported_building_id: String,
    #[dsl(unit = "N")]
    pub r_k_n: f64,
    pub gamma_el: f64,
}

/// 🫙 Silo for EN 1998-4.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Silo {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub radius_m: f64,
    /// ⛓️ Structure permanent action ΣG_k [N].
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// 📄️ Characteristic content action Q_k [N] at full fill.
    #[dsl(unit = "N")]
    pub content_qk_n: f64,
    /// 📦️ Content category for ψ₂ (typically E storage).
    pub content_category: String,
    /// 🪣️ Design filling ratio 0…1 (EN 1998-4).
    pub filling_ratio: f64,
    #[dsl(unit = "N")]
    pub n_rd_n: f64,
    #[dsl(unit = "N")]
    pub v_rd_n: f64,
    pub q_nominal: f64,
}

/// 🛢️ Tank for EN 1998-4.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Tank {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    #[dsl(unit = "m")]
    pub radius_m: f64,
    /// 🕹️ Structure permanent action ΣG_k [N].
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    /// 🧾️ Characteristic content action Q_k [N] at full fill.
    #[dsl(unit = "N")]
    pub content_qk_n: f64,
    /// 📝️ Content category for ψ₂ (typically E storage / liquid).
    pub content_category: String,
    /// 🟦️ Design filling ratio 0…1 (EN 1998-4).
    pub filling_ratio: f64,
    #[dsl(unit = "N")]
    pub v_rd_n: f64,
}

/// 🪨 Foundation for EN 1998-5.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Foundation {
    pub id: String,
    /// 🤝️ Building id whose seismic base shear drives foundation demand (EN 1998-5 §5.3).
    pub supported_building_id: String,
    #[dsl(unit = "m2")]
    pub area_m2: f64,
    #[dsl(unit = "Pa")]
    pub p_rd_pa: f64,
    #[dsl(unit = "N")]
    pub h_rd_n: f64,
    pub k_foundation: f64,
    pub k_soil: f64,
}

/// 🧱 Retaining wall for EN 1998-5.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998RetainingWall {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    pub phi_deg: f64,
    pub soil_gamma: f64,
    pub r: f64,
    pub h_rd_n_per_m: f64,
}

/// 🗼 Tower / mast / chimney for EN 1998-6.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct En1998Tower {
    pub id: String,
    #[dsl(unit = "m")]
    pub height_m: f64,
    pub m_rd_nm: f64,
    pub is_chimney: bool,
    pub q_nominal: f64,
    /// 🎬️ Characteristic permanent action ΣG_k [N].
    #[dsl(unit = "N")]
    pub permanent_gk_n: f64,
    pub correlated_occupancy: bool,
    #[dsl(table)]
    pub variables: Vec<En1998VariableAction>,
}
