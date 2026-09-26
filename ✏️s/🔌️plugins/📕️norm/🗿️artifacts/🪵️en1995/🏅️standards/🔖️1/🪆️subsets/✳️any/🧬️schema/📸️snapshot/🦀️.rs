//! 🪵 EN 1995 snapshot — timber structure subject (members + connections).

use crate::document::AnnexChoice;
use crate::{
    CharacteristicAction, ConnectionAction, MemberRole, SupportType, TimberConnection, TimberMember,
};
use framework_schema::ArtifactSchema;

//#region 🔖️Snapshot
/// 📸️ Persisted EN 1995 timber-structure document.
#[derive(Clone, Debug, PartialEq, dsl::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
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

//#region 🔖️HandcraftedArtifactCodecs
crate::impl_norm_artifact_record!(En1995Snapshot, extension = "en1995", envelope_id = "norm.en1995");
//#endregion 🔖️HandcraftedArtifactCodecs

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

//#region 🌉️ExternalCodecBridge
pub fn encode_en1995_snapshot_json(snapshot: &En1995Snapshot) -> String {
    pack::json::to_json_string(snapshot)
}
pub fn decode_en1995_snapshot_json(text: &str) -> Result<En1995Snapshot, String> {
    pack::json::from_json_str(text).map_err(|error| error.to_string())
}
pub fn decode_en1995_dsl(text: &str) -> Result<En1995Snapshot, String> {
    <En1995Snapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1995_dsl(snapshot: &En1995Snapshot) -> String {
    store::ArtifactDsl::print_dsl(snapshot)
}
pub fn decode_en1995_pack(bytes: &[u8]) -> Result<En1995Snapshot, String> {
    <En1995Snapshot as store::ArtifactPack>::decode_pack(bytes).map_err(|error| format!("{error:?}"))
}
pub fn encode_en1995_pack(snapshot: &En1995Snapshot) -> Vec<u8> {
    store::ArtifactPack::encode_pack(snapshot)
}
//#endregion 🌉️ExternalCodecBridge
