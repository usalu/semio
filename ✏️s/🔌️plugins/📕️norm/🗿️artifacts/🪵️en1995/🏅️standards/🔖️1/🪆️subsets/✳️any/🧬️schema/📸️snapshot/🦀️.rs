//! 🪵 EN 1995 snapshot — timber structure subject (members + connections).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

use semio_s_plugin_norm_compliance::AnnexChoice;

use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1995 timber-structure document.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.en1995", layout = "lines")]
#[artifact_schema(id = "s.norm.en1995")]
pub struct En1995Snapshot {
    #[state(artifact)]
    pub annex: AnnexChoice,
    #[dsl(table)]
    #[state(artifact)]
    pub members: Vec<TimberMember>,
    #[dsl(table)]
    #[state(artifact)]
    pub connections: Vec<TimberConnection>,
}
//#endregion 🔖️Snapshot

impl Default for En1995Snapshot {
    fn default() -> Self {
        Self::compliant_building_beam()
    }
}

impl En1995Snapshot {
    /// ✅️ Compliant DE glulam building beam (GL28h, Floor role) — default subject.
    pub fn compliant_building_beam() -> Self {
        Self {
            annex: AnnexChoice::De,
            members: vec![TimberMember {
                id: "beam-B1".into(),
                label_en: "Main glulam floor beam B1".into(),
                label_de: "Hauptträger BSH Decke B1".into(),
                role: MemberRole::Floor,
                strength_class: "GL28h".into(),
                service_class: 1,
                support: SupportType::SimplySupported,
                b_m: 0.20,
                h_m: 0.40,
                span_m: 5.0,
                support_length_m: 0.15,
                bearing_length_m: 0.15,
                buckling_length_y_m: 5.0,
                buckling_length_z_m: 1.5,
                lateral_restraint_spacing_m: 1.5,
                notch_depth_m: 0.0,
                notch_distance_m: 0.0,
                m_crit_nm: 180_000.0,
                mass_kg_per_m: 120.0,
                mass_kg_per_m2: 450.0,
                damping_xi: 0.04,
                fire_duration_s: 0.0,
                bridge_n_obs: 0.0,
                bridge_t_l_years: 0.0,
                bridge_beta: 0.0,
                bridge_a: 0.0,
                bridge_b: 0.0,
                bridge_crowd_per_m2: 0.0,
                actions: vec![
                    CharacteristicAction {
                        id: "g".into(),
                        kind: "permanent".into(),
                        category: "".into(),
                        load_duration: "permanent".into(),
                        q_line_n_per_m: 2_500.0,
                        f_point_n: 0.0,
                        m_k_nm: 0.0,
                        v_k_n: 0.0,
                        n_k_n: 0.0,
                        n_t_k_n: 0.0,
                        f_c90_k_n: 0.0,
                    },
                    CharacteristicAction {
                        id: "q".into(),
                        kind: "imposed".into(),
                        category: "A".into(),
                        load_duration: "medium".into(),
                        q_line_n_per_m: 3_000.0,
                        f_point_n: 0.0,
                        m_k_nm: 0.0,
                        v_k_n: 0.0,
                        n_k_n: 0.0,
                        n_t_k_n: 0.0,
                        f_c90_k_n: 0.0,
                    },
                ],
            }],
            connections: vec![TimberConnection {
                id: "conn-C1".into(),
                label_en: "Bolt group at support".into(),
                label_de: "Schraubverbund am Auflager".into(),
                fastener_type: "bolt".into(),
                strength_class: "GL28h".into(),
                service_class: 1,
                diameter_m: 0.012,
                number: 8,
                rows: 2,
                spacing_m: 0.090,
                edge_distance_m: 0.048,
                end_distance_m: 0.096,
                t1_m: 0.20,
                t2_m: 0.20,
                steel_plate: false,
                steel_plate_thickness_m: 0.0,
                shear_planes: 1,
                f_u_k: 400_000_000.0,
                actions: vec![
                    ConnectionAction {
                        id: "g".into(),
                        kind: "permanent".into(),
                        load_duration: "permanent".into(),
                        f_k_n: 8_000.0,
                    },
                    ConnectionAction {
                        id: "q".into(),
                        kind: "imposed".into(),
                        load_duration: "medium".into(),
                        f_k_n: 10_000.0,
                    },
                ],
            }],
        }
    }

    /// ❌️ Non-compliant building subject (overloaded C24, column, weak nails, fire).
    pub fn noncompliant_building() -> Self {
        Self {
            annex: AnnexChoice::De,
            members: vec![
                TimberMember {
                    id: "beam-B2".into(),
                    label_en: "Overloaded beam B2".into(),
                    label_de: "Überlasteter Träger B2".into(),
                    role: MemberRole::Beam,
                    strength_class: "C24".into(),
                    service_class: 1,
                    support: SupportType::SimplySupported,
                    b_m: 0.12,
                    h_m: 0.24,
                    span_m: 5.0,
                    support_length_m: 0.08,
                    bearing_length_m: 0.08,
                    buckling_length_y_m: 5.0,
                    buckling_length_z_m: 5.0,
                    lateral_restraint_spacing_m: 2.5,
                    notch_depth_m: 0.04,
                    notch_distance_m: 0.10,
                    m_crit_nm: 250_000.0,
                    mass_kg_per_m: 0.0,
                    mass_kg_per_m2: 0.0,
                    damping_xi: 0.01,
                    fire_duration_s: 3600.0,
                    bridge_n_obs: 0.0,
                    bridge_t_l_years: 0.0,
                    bridge_beta: 0.0,
                    bridge_a: 0.0,
                    bridge_b: 0.0,
                    bridge_crowd_per_m2: 0.0,
                    actions: vec![
                        CharacteristicAction {
                            id: "g".into(),
                            kind: "permanent".into(),
                            category: "".into(),
                            load_duration: "permanent".into(),
                            q_line_n_per_m: 4_000.0,
                            f_point_n: 0.0,
                            m_k_nm: 0.0,
                            v_k_n: 0.0,
                            n_k_n: 0.0,
                            n_t_k_n: 0.0,
                            f_c90_k_n: 0.0,
                        },
                        CharacteristicAction {
                            id: "q".into(),
                            kind: "imposed".into(),
                            category: "C".into(),
                            load_duration: "medium".into(),
                            q_line_n_per_m: 8_000.0,
                            f_point_n: 5_000.0,
                            m_k_nm: 0.0,
                            v_k_n: 0.0,
                            n_k_n: 0.0,
                            n_t_k_n: 0.0,
                            f_c90_k_n: 0.0,
                        },
                    ],
                },
                TimberMember {
                    id: "col-C1".into(),
                    label_en: "Slender column C1".into(),
                    label_de: "Schlanker Stützpfosten C1".into(),
                    role: MemberRole::Column,
                    strength_class: "C24".into(),
                    service_class: 2,
                    support: SupportType::SimplySupported,
                    b_m: 0.10,
                    h_m: 0.10,
                    span_m: 3.0,
                    support_length_m: 0.10,
                    bearing_length_m: 0.10,
                    buckling_length_y_m: 3.0,
                    buckling_length_z_m: 3.0,
                    lateral_restraint_spacing_m: 3.0,
                    notch_depth_m: 0.0,
                    notch_distance_m: 0.0,
                    m_crit_nm: 50_000.0,
                    mass_kg_per_m: 0.0,
                    mass_kg_per_m2: 0.0,
                    damping_xi: 0.01,
                    fire_duration_s: 0.0,
                    bridge_n_obs: 0.0,
                    bridge_t_l_years: 0.0,
                    bridge_beta: 0.0,
                    bridge_a: 0.0,
                    bridge_b: 0.0,
                    bridge_crowd_per_m2: 0.0,
                    actions: vec![CharacteristicAction {
                        id: "g".into(),
                        kind: "permanent".into(),
                        category: "".into(),
                        load_duration: "medium".into(),
                        q_line_n_per_m: 0.0,
                        f_point_n: 80_000.0,
                        m_k_nm: 0.0,
                        v_k_n: 0.0,
                        n_k_n: 0.0,
                        n_t_k_n: 0.0,
                        f_c90_k_n: 0.0,
                    }],
                },
            ],
            connections: vec![TimberConnection {
                id: "conn-C2".into(),
                label_en: "Underdesigned nail group".into(),
                label_de: "Unterbemessene Nagelgruppe".into(),
                fastener_type: "nail".into(),
                strength_class: "C24".into(),
                service_class: 1,
                diameter_m: 0.0031,
                number: 4,
                rows: 1,
                spacing_m: 0.012,
                edge_distance_m: 0.010,
                end_distance_m: 0.030,
                t1_m: 0.04,
                t2_m: 0.04,
                steel_plate: false,
                steel_plate_thickness_m: 0.0,
                shear_planes: 1,
                f_u_k: 600_000_000.0,
                actions: vec![ConnectionAction {
                    id: "q".into(),
                    kind: "imposed".into(),
                    load_duration: "medium".into(),
                    f_k_n: 8_000.0,
                }],
            }],
        }
    }

    /// ✅️ Compliant glulam footbridge (EN 1995-2 role Bridge).
    pub fn compliant_bridge() -> Self {
        Self {
            annex: AnnexChoice::De,
            members: vec![TimberMember {
                id: "bridge-G1".into(),
                label_en: "Glulam footbridge girder G1".into(),
                label_de: "BSH-Fußgängersteg Träger G1".into(),
                role: MemberRole::Bridge,
                strength_class: "GL28h".into(),
                service_class: 2,
                support: SupportType::SimplySupported,
                b_m: 0.24,
                h_m: 0.72,
                span_m: 12.0,
                support_length_m: 0.30,
                bearing_length_m: 0.30,
                buckling_length_y_m: 12.0,
                buckling_length_z_m: 3.0,
                lateral_restraint_spacing_m: 3.0,
                notch_depth_m: 0.0,
                notch_distance_m: 0.0,
                m_crit_nm: 800_000.0,
                mass_kg_per_m: 180.0,
                mass_kg_per_m2: 0.0,
                damping_xi: 0.015,
                fire_duration_s: 0.0,
                bridge_n_obs: 2.0e5,
                bridge_t_l_years: 50.0,
                bridge_beta: 5.0,
                bridge_a: 15.0,
                bridge_b: 4.0,
                bridge_crowd_per_m2: 1.0,
                actions: vec![],
            }],
            connections: vec![],
        }
    }

    /// ❌️ Non-compliant bridge — high crowd and high N_obs (fatigue / vibration fail).
    pub fn noncompliant_bridge() -> Self {
        Self {
            annex: AnnexChoice::De,
            members: vec![TimberMember {
                id: "bridge-G2".into(),
                label_en: "Overloaded footbridge girder G2".into(),
                label_de: "Überlasteter Fußgängersteg G2".into(),
                role: MemberRole::Bridge,
                strength_class: "GL24h".into(),
                service_class: 2,
                support: SupportType::SimplySupported,
                b_m: 0.16,
                h_m: 0.40,
                span_m: 14.0,
                support_length_m: 0.20,
                bearing_length_m: 0.20,
                buckling_length_y_m: 14.0,
                buckling_length_z_m: 7.0,
                lateral_restraint_spacing_m: 7.0,
                notch_depth_m: 0.0,
                notch_distance_m: 0.0,
                m_crit_nm: 80_000.0,
                mass_kg_per_m: 60.0,
                mass_kg_per_m2: 0.0,
                damping_xi: 0.005,
                fire_duration_s: 0.0,
                bridge_n_obs: 2.0e7,
                bridge_t_l_years: 100.0,
                bridge_beta: 5.0,
                bridge_a: 9.0,
                bridge_b: 5.5,
                bridge_crowd_per_m2: 5.0,
                actions: vec![],
            }],
            connections: vec![],
        }
    }
}

//#region 🍃️FacetLeaves
/// 🍃️ The five handcrafted leaves of the `snapshot` schema facet.
pub const SNAPSHOT_LEAVES: semio_framework_schema_registry::FacetLeaves = semio_framework_schema_registry::FacetLeaves { rust: include_str!("🦀️.rs"), typescript: include_str!("🟦️.ts"), graphql: include_str!("🔗️.graphql"), json_schema: include_str!("🔣️.json"), proto: include_str!("🛰️.proto") };
//#endregion 🍃️FacetLeaves

use semio_s_plugin_norm_app_surface as app_surface;

/// 🪵 Timber strength properties in SI (Pa, kg/m³) from EN 338 / EN 14080 tables.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TimberProperties {
    pub f_m_k: f64,
    pub f_t_0_k: f64,
    pub f_t_90_k: f64,
    pub f_c_0_k: f64,
    pub f_c_90_k: f64,
    pub f_v_k: f64,
    pub e_0_mean: f64,
    pub e_0_05: f64,
    pub e_90_mean: f64,
    pub g_mean: f64,
    pub rho_k: f64,
    pub product: TimberProduct,
}

/// 🪵 Product family for γ_M routing (EN 1995-1-1 Table 2.3).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TimberProduct {
    Solid,
    Glulam,
    Lvl,
    Clt,
}

/// 🎯 Member structural role — gates floor §7.3 vs bridge EN 1995-2 checks.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum MemberRole {
    Beam,
    Column,
    Floor,
    Bridge,
}

/// 📍 Support idealisation for characteristic → internal force derivation.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslScalar, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub enum SupportType {
    SimplySupported,
    Cantilever,
    ContinuousTwoSpan,
}

/// ⚖️ Characteristic action on a timber member (EN 1990 family + duration for k_mod).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct CharacteristicAction {
    pub id: String,
    /// permanent | imposed | snow | wind | accidental
    pub kind: String,
    /// Imposed category A–H when kind=imposed; else empty.
    pub category: String,
    /// permanent | long | medium | short | instantaneous (DIN EN 1995-1-1/NA Table NA.1)
    pub load_duration: String,
    /// Line load q_k (N/m) along span — used with support type when analysed forces are zero.
    pub q_line_n_per_m: f64,
    /// Point load F_k (N) at critical location.
    #[dsl(unit = "N")]
    pub f_point_n: f64,
    /// Externally analysed characteristic internals (used when both load intensities are 0).
    pub m_k_nm: f64,
    #[dsl(unit = "N")]
    pub v_k_n: f64,
    #[dsl(unit = "N")]
    pub n_k_n: f64,
    #[dsl(unit = "N")]
    pub n_t_k_n: f64,
    #[dsl(unit = "N")]
    pub f_c90_k_n: f64,
}

/// 🔗 Characteristic shear force on a connection under one action family.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ConnectionAction {
    pub id: String,
    pub kind: String,
    pub load_duration: String,
    #[dsl(unit = "N")]
    pub f_k_n: f64,
}

/// 🪵 Rectangular timber member under EN 1995 — SI base units.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TimberMember {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub role: MemberRole,
    pub strength_class: String,
    pub service_class: u8,
    pub support: SupportType,
    #[dsl(unit = "m")]
    pub b_m: f64,
    #[dsl(unit = "m")]
    pub h_m: f64,
    #[dsl(unit = "m")]
    pub span_m: f64,
    #[dsl(unit = "m")]
    pub support_length_m: f64,
    #[dsl(unit = "m")]
    pub bearing_length_m: f64,
    #[dsl(unit = "m")]
    pub buckling_length_y_m: f64,
    #[dsl(unit = "m")]
    pub buckling_length_z_m: f64,
    #[dsl(unit = "m")]
    pub lateral_restraint_spacing_m: f64,
    #[dsl(unit = "m")]
    pub notch_depth_m: f64,
    #[dsl(unit = "m")]
    pub notch_distance_m: f64,
    pub m_crit_nm: f64,
    /// Linear mass for vibration (kg/m) — joist/girder.
    pub mass_kg_per_m: f64,
    /// Area mass for floor panels (kg/m²).
    pub mass_kg_per_m2: f64,
    /// Modal damping ratio ξ (—).
    pub damping_xi: f64,
    #[dsl(unit = "s")]
    pub fire_duration_s: f64,
    /// EN 1995-2 Annex A — observed cycles per year N_obs.
    pub bridge_n_obs: f64,
    /// Design working life t_L (years) for fatigue.
    pub bridge_t_l_years: f64,
    /// Fatigue exponent β (Annex A).
    pub bridge_beta: f64,
    /// Wohler intercept a (Annex A.2).
    pub bridge_a: f64,
    /// Wohler slope b (Annex A.2).
    pub bridge_b: f64,
    /// Pedestrian crowd density for Annex B (1/m²).
    pub bridge_crowd_per_m2: f64,
    #[dsl(table)]
    pub actions: Vec<CharacteristicAction>,
}

/// 🔗 Dowel-type timber connection (nails/screws/bolts/dowels) — SI base units.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct TimberConnection {
    pub id: String,
    pub label_en: String,
    pub label_de: String,
    pub fastener_type: String,
    pub strength_class: String,
    pub service_class: u8,
    #[dsl(unit = "m")]
    pub diameter_m: f64,
    pub number: u32,
    pub rows: u32,
    #[dsl(unit = "m")]
    pub spacing_m: f64,
    #[dsl(unit = "m")]
    pub edge_distance_m: f64,
    #[dsl(unit = "m")]
    pub end_distance_m: f64,
    #[dsl(unit = "m")]
    pub t1_m: f64,
    #[dsl(unit = "m")]
    pub t2_m: f64,
    pub steel_plate: bool,
    #[dsl(unit = "m")]
    pub steel_plate_thickness_m: f64,
    pub shear_planes: u32,
    #[dsl(unit = "Pa")]
    pub f_u_k: f64,
    #[dsl(table)]
    pub actions: Vec<ConnectionAction>,
}

pub fn artifact_kind() -> semio_framework_plugin::ArtifactKindSpec {
    app_surface::artifact_kind_spec("en1995", "EN 1995", EN1995_DOCUMENT_SCHEMA)
}

/// 🪪️ This subset's canonical `(artifact_kind, standard, subset)` coordinate (ticket
/// 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET contract §1) — lives at the ARTIFACT level, not
/// under the sibling `editor` module, so a viewer file can read it without ever importing through it.
pub const EN1995_DIALECT: semio_framework_plugin::app::Dialect = semio_framework_plugin::app::Dialect { artifact_kind: "s.norm.en1995", standard: semio_framework_plugin::app::StandardId("1"), subset: semio_framework_plugin::app::SubsetId::ANY };

pub const EN1995_DOCUMENT_SCHEMA: &str = "semio.norm.en1995/v1";
