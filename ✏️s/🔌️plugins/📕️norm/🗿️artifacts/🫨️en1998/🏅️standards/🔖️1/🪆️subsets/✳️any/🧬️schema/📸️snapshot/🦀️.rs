//! 🌋️ EN 1998 snapshot — complete scoped seismic subject (buildings + parts 2–6).

use crate::{
    DeGroundCombo, DeSeismicZone, En1998Assessment, En1998Bridge, En1998Building, En1998Foundation, En1998RetainingWall, En1998Site, En1998Silo,
    En1998Storey, En1998VariableAction, En1998System, En1998Tank, En1998Tower, En1998Member,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1998 project snapshot.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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

crate::impl_norm_artifact_record!(En1998Snapshot, extension = "en1998", envelope_id = "norm.en1998");

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
        s.bridges = vec![crate::En1998Bridge {
            id: "br-1".into(),
            period_ratio: 2.0,
            fundamental_period_s: 1.2,
            v_rd_n: 2.0e6,
            bearing_d_rd_m: 0.40,
            permanent_gk_n: 8.0e6,
            correlated_occupancy: true,
            variables: vec![crate::En1998VariableAction { id: "br-q1".into(), category: "F".into(), qk_n: 1.5e6 }],
        }];
        s.assessments = vec![crate::En1998Assessment {
            id: "as-1".into(),
            knowledge_level: "kl2".into(),
            limit_state: "sd".into(),
            supported_building_id: "bldg-office".into(),
            r_k_n: 5.0e5,
            gamma_el: 1.0,
        }];
        s.silos = vec![crate::En1998Silo {
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
        s.tanks = vec![crate::En1998Tank {
            id: "tk-1".into(),
            height_m: 10.0,
            radius_m: 5.0,
            permanent_gk_n: 8.0e5,
            content_qk_n: 5.0e6,
            content_category: "E".into(),
            filling_ratio: 0.90,
            v_rd_n: 3.0e6,
        }];
        s.foundations = vec![crate::En1998Foundation {
            id: "fd-1".into(),
            supported_building_id: "bldg-office".into(),
            area_m2: 200.0,
            p_rd_pa: 4.0e5,
            h_rd_n: 5.0e5,
            k_foundation: 8.0e5,
            k_soil: 3.0e5,
        }];
        s.retaining_walls = vec![crate::En1998RetainingWall {
            id: "rw-1".into(),
            height_m: 4.5,
            phi_deg: 32.0,
            soil_gamma: 18000.0,
            r: 1.5,
            h_rd_n_per_m: 2.5e5,
        }];
        s.towers = vec![crate::En1998Tower {
            id: "tw-1".into(),
            height_m: 45.0,
            m_rd_nm: 8.0e6,
            is_chimney: false,
            q_nominal: 2.5,
            permanent_gk_n: 1.0e6,
            correlated_occupancy: true,
            variables: vec![crate::En1998VariableAction { id: "tw-q1".into(), category: "E".into(), qk_n: 5.0e4 }],
        }];
        s
    }

    /// ⚠️ Non-compliant multi-part DE subject (weak capacities across parts 2–6).
    pub fn noncompliant_de_multipart() -> Self {
        let mut s = Self::noncompliant_de_office();
        s.bridges = vec![crate::En1998Bridge {
            id: "br-weak".into(),
            period_ratio: 1.2,
            fundamental_period_s: 1.5,
            v_rd_n: 5.0e4,
            bearing_d_rd_m: 0.02,
            permanent_gk_n: 8.0e6,
            correlated_occupancy: true,
            variables: vec![crate::En1998VariableAction { id: "br-q1".into(), category: "F".into(), qk_n: 1.5e6 }],
        }];
        s.assessments = vec![crate::En1998Assessment {
            id: "as-weak".into(),
            knowledge_level: "kl1".into(),
            limit_state: "nc".into(),
            supported_building_id: "bldg-weak".into(),
            r_k_n: 2.0e4,
            gamma_el: 1.2,
        }];
        s.silos = vec![crate::En1998Silo {
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
        s.tanks = vec![crate::En1998Tank {
            id: "tk-weak".into(),
            height_m: 12.0,
            radius_m: 4.0,
            permanent_gk_n: 8.0e5,
            content_qk_n: 6.0e6,
            content_category: "E".into(),
            filling_ratio: 1.0,
            v_rd_n: 5.0e4,
        }];
        s.foundations = vec![crate::En1998Foundation {
            id: "fd-weak".into(),
            supported_building_id: "bldg-weak".into(),
            area_m2: 40.0,
            p_rd_pa: 8.0e4,
            h_rd_n: 2.0e4,
            k_foundation: 2.0e5,
            k_soil: 1.0e5,
        }];
        s.retaining_walls = vec![crate::En1998RetainingWall {
            id: "rw-weak".into(),
            height_m: 6.0,
            phi_deg: 25.0,
            soil_gamma: 19000.0,
            r: 1.0,
            h_rd_n_per_m: 2.0e4,
        }];
        s.towers = vec![crate::En1998Tower {
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
//#region 🌉️ExternalCodecBridge
pub fn encode_en1998_snapshot_json(snapshot: &En1998Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}

pub fn decode_en1998_snapshot_json(text: &str) -> Result<En1998Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}

pub fn decode_en1998_dsl(text: &str) -> Result<En1998Snapshot, String> {
    <En1998Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}

pub fn encode_en1998_dsl(snapshot: &En1998Snapshot) -> String {
    <En1998Snapshot as store::ArtifactDsl>::print_dsl(snapshot)
}

pub fn encode_en1998_pack(snapshot: &En1998Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}

pub fn decode_en1998_pack(bytes: &[u8]) -> Result<En1998Snapshot, String> {
    <En1998Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
//#endregion 🌉️ExternalCodecBridge
