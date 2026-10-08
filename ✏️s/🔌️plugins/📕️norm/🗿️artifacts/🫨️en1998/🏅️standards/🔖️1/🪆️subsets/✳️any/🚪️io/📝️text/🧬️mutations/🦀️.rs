//! ⚡️ EN 1998 — hand-rolled OpText/OpBinary for `En1998Mutation` (B2 JSON-atom field codec).

use crate::artifact_schema::mutations::En1998Mutation;

use crate::artifact_schema::mutations::{
    change_annex::ChangeAnnex, update_site::UpdateSite, insert_building::InsertBuilding, remove_building::RemoveBuilding,
    change_system_v_rd_n::ChangeSystemVRdN, change_storey_permanent_gk_n::ChangeStoreyPermanentGkN,
    change_storey_stiffness_x::ChangeStoreyStiffnessX, change_storey_drift_xm::ChangeStoreyDriftXM,
    change_building_plan_regular::ChangeBuildingPlanRegular, change_elevation_regular::ChangeElevationRegular,
    change_member_detailing::ChangeMemberDetailing,
    change_masonry_wall_ratio::ChangeMasonryWallRatio, insert_bridge::InsertBridge,
    change_bridge_v_rd_n::ChangeBridgeVRdN, insert_assessment::InsertAssessment, change_assessment_rkn::ChangeAssessmentRKN,
    insert_silo::InsertSilo, insert_tank::InsertTank, insert_foundation::InsertFoundation,
    insert_retaining_wall::InsertRetainingWall, insert_tower::InsertTower, change_tower_m_rd_nm::ChangeTowerMRdNm,
    remove_bridge::RemoveBridge, remove_assessment::RemoveAssessment, remove_silo::RemoveSilo, remove_tank::RemoveTank, remove_foundation::RemoveFoundation, remove_retaining_wall::RemoveRetainingWall, remove_tower::RemoveTower,
};

pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");

fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
fn enc_json<T: semio_framework_value::ToValue>(value: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(value))
}
fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

/// 📍 An insert's optional `index=` argument: absent appends.
fn opt_index(args: &std::collections::BTreeMap<String, String>) -> Result<Option<usize>, String> {
    args.get("index").map_or(Ok(None), |raw| dec_json::<Option<usize>>(raw))
}
fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}

fn print_en1998_mutation(mutation: &En1998Mutation) -> String {
    match mutation {
        En1998Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1998Mutation::UpdateSite(p) => format!("update-site site={}", enc_json(&p.site)),
        En1998Mutation::InsertBuilding(p) => format!("insert-building index={} building={}", enc_json(&p.index), enc_json(&p.building)),
        En1998Mutation::RemoveBuilding(p) => format!("remove-building index={}", enc_json(&p.index)),
        En1998Mutation::ChangeSystemVRdN(p) => format!("change-system-v-rd-n building-index={} system-index={} new-base-shear-resistance-n={}", enc_json(&p.building_index), enc_json(&p.system_index), enc_json(&p.new_base_shear_resistance_n)),
        En1998Mutation::ChangeStoreyPermanentGkN(p) => format!("change-storey-permanent-gk-n building-index={} storey-index={} new-permanent-gk-n={}", enc_json(&p.building_index), enc_json(&p.storey_index), enc_json(&p.new_permanent_gk_n)),
        En1998Mutation::ChangeStoreyStiffnessX(p) => format!("change-storey-stiffness-x building-index={} storey-index={} new-stiffness-x={}", enc_json(&p.building_index), enc_json(&p.storey_index), enc_json(&p.new_stiffness_x)),
        En1998Mutation::ChangeStoreyDriftXM(p) => format!("change-storey-drift-xm building-index={} storey-index={} new-drift-x-m={}", enc_json(&p.building_index), enc_json(&p.storey_index), enc_json(&p.new_drift_x_m)),
        En1998Mutation::ChangeBuildingPlanRegular(p) => format!("change-building-plan-regular building-index={} new-plan-regular={}", enc_json(&p.building_index), enc_json(&p.new_plan_regular)),
        En1998Mutation::ChangeElevationRegular(p) => format!("change-elevation-regular building-index={} new-elevation-regular={}", enc_json(&p.building_index), enc_json(&p.new_elevation_regular)),
        En1998Mutation::ChangeMemberDetailing(p) => format!("change-member-detailing building-index={} member-index={} new-detailing-compatible-with-q={}", enc_json(&p.building_index), enc_json(&p.member_index), enc_json(&p.new_detailing_compatible_with_q)),
        En1998Mutation::ChangeMasonryWallRatio(p) => format!("change-masonry-wall-ratio building-index={} new-masonry-wall-area-ratio={}", enc_json(&p.building_index), enc_json(&p.new_masonry_wall_area_ratio)),
        En1998Mutation::InsertBridge(p) => format!("insert-bridge index={} bridge={}", enc_json(&p.index), enc_json(&p.bridge)),
        En1998Mutation::ChangeBridgeVRdN(p) => format!("change-bridge-v-rd-n index={} new-v-rd-n={}", enc_json(&p.index), enc_json(&p.new_v_rd_n)),
        En1998Mutation::InsertAssessment(p) => format!("insert-assessment index={} assessment={}", enc_json(&p.index), enc_json(&p.assessment)),
        En1998Mutation::ChangeAssessmentRKN(p) => format!("change-assessment-rkn index={} new-r-k-n={}", enc_json(&p.index), enc_json(&p.new_r_k_n)),
        En1998Mutation::InsertSilo(p) => format!("insert-silo index={} silo={}", enc_json(&p.index), enc_json(&p.silo)),
        En1998Mutation::InsertTank(p) => format!("insert-tank index={} tank={}", enc_json(&p.index), enc_json(&p.tank)),
        En1998Mutation::InsertFoundation(p) => format!("insert-foundation index={} foundation={}", enc_json(&p.index), enc_json(&p.foundation)),
        En1998Mutation::InsertRetainingWall(p) => format!("insert-retaining-wall index={} wall={}", enc_json(&p.index), enc_json(&p.wall)),
        En1998Mutation::InsertTower(p) => format!("insert-tower index={} tower={}", enc_json(&p.index), enc_json(&p.tower)),
        En1998Mutation::ChangeTowerMRdNm(p) => format!("change-tower-m-rd-nm index={} new-m-rd-nm={}", enc_json(&p.index), enc_json(&p.new_m_rd_nm)),
        En1998Mutation::RemoveBridge(p) => format!("remove-bridge index={}", enc_json(&p.index)),
        En1998Mutation::RemoveAssessment(p) => format!("remove-assessment index={}", enc_json(&p.index)),
        En1998Mutation::RemoveSilo(p) => format!("remove-silo index={}", enc_json(&p.index)),
        En1998Mutation::RemoveTank(p) => format!("remove-tank index={}", enc_json(&p.index)),
        En1998Mutation::RemoveFoundation(p) => format!("remove-foundation index={}", enc_json(&p.index)),
        En1998Mutation::RemoveRetainingWall(p) => format!("remove-retaining-wall index={}", enc_json(&p.index)),
        En1998Mutation::RemoveTower(p) => format!("remove-tower index={}", enc_json(&p.index)),
    }
}

fn parse_en1998_mutation(line: &str) -> Result<En1998Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1998 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-annex" => Ok(En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),
        "update-site" => Ok(En1998Mutation::UpdateSite(UpdateSite { site: dec_json(&arg("site")?)? })),
        "insert-building" => Ok(En1998Mutation::InsertBuilding(InsertBuilding { index: opt_index(&args)?, building: dec_json(&arg("building")?)? })),
        "remove-building" => Ok(En1998Mutation::RemoveBuilding(RemoveBuilding { index: dec_json(&arg("index")?)? })),
        "change-system-v-rd-n" => Ok(En1998Mutation::ChangeSystemVRdN(ChangeSystemVRdN { building_index: dec_json(&arg("building-index")?)?, system_index: dec_json(&arg("system-index")?)?, new_base_shear_resistance_n: dec_json(&arg("new-base-shear-resistance-n")?)? })),
        "change-storey-permanent-gk-n" => Ok(En1998Mutation::ChangeStoreyPermanentGkN(ChangeStoreyPermanentGkN { building_index: dec_json(&arg("building-index")?)?, storey_index: dec_json(&arg("storey-index")?)?, new_permanent_gk_n: dec_json(&arg("new-permanent-gk-n")?)? })),
        "change-storey-stiffness-x" => Ok(En1998Mutation::ChangeStoreyStiffnessX(ChangeStoreyStiffnessX { building_index: dec_json(&arg("building-index")?)?, storey_index: dec_json(&arg("storey-index")?)?, new_stiffness_x: dec_json(&arg("new-stiffness-x")?)? })),
        "change-storey-drift-xm" => Ok(En1998Mutation::ChangeStoreyDriftXM(ChangeStoreyDriftXM { building_index: dec_json(&arg("building-index")?)?, storey_index: dec_json(&arg("storey-index")?)?, new_drift_x_m: dec_json(&arg("new-drift-x-m")?)? })),
        "change-building-plan-regular" => Ok(En1998Mutation::ChangeBuildingPlanRegular(ChangeBuildingPlanRegular { building_index: dec_json(&arg("building-index")?)?, new_plan_regular: dec_json(&arg("new-plan-regular")?)? })),
        "change-elevation-regular" => Ok(En1998Mutation::ChangeElevationRegular(ChangeElevationRegular { building_index: dec_json(&arg("building-index")?)?, new_elevation_regular: dec_json(&arg("new-elevation-regular")?)? })),
        "change-member-detailing" => Ok(En1998Mutation::ChangeMemberDetailing(ChangeMemberDetailing { building_index: dec_json(&arg("building-index")?)?, member_index: dec_json(&arg("member-index")?)?, new_detailing_compatible_with_q: dec_json(&arg("new-detailing-compatible-with-q")?)? })),
        "change-masonry-wall-ratio" => Ok(En1998Mutation::ChangeMasonryWallRatio(ChangeMasonryWallRatio { building_index: dec_json(&arg("building-index")?)?, new_masonry_wall_area_ratio: dec_json(&arg("new-masonry-wall-area-ratio")?)? })),
        "insert-bridge" => Ok(En1998Mutation::InsertBridge(InsertBridge { index: opt_index(&args)?, bridge: dec_json(&arg("bridge")?)? })),
        "change-bridge-v-rd-n" => Ok(En1998Mutation::ChangeBridgeVRdN(ChangeBridgeVRdN { index: dec_json(&arg("index")?)?, new_v_rd_n: dec_json(&arg("new-v-rd-n")?)? })),
        "insert-assessment" => Ok(En1998Mutation::InsertAssessment(InsertAssessment { index: opt_index(&args)?, assessment: dec_json(&arg("assessment")?)? })),
        "change-assessment-rkn" => Ok(En1998Mutation::ChangeAssessmentRKN(ChangeAssessmentRKN { index: dec_json(&arg("index")?)?, new_r_k_n: dec_json(&arg("new-r-k-n")?)? })),
        "insert-silo" => Ok(En1998Mutation::InsertSilo(InsertSilo { index: opt_index(&args)?, silo: dec_json(&arg("silo")?)? })),
        "insert-tank" => Ok(En1998Mutation::InsertTank(InsertTank { index: opt_index(&args)?, tank: dec_json(&arg("tank")?)? })),
        "insert-foundation" => Ok(En1998Mutation::InsertFoundation(InsertFoundation { index: opt_index(&args)?, foundation: dec_json(&arg("foundation")?)? })),
        "insert-retaining-wall" => Ok(En1998Mutation::InsertRetainingWall(InsertRetainingWall { index: opt_index(&args)?, wall: dec_json(&arg("wall")?)? })),
        "insert-tower" => Ok(En1998Mutation::InsertTower(InsertTower { index: opt_index(&args)?, tower: dec_json(&arg("tower")?)? })),
        "change-tower-m-rd-nm" => Ok(En1998Mutation::ChangeTowerMRdNm(ChangeTowerMRdNm { index: dec_json(&arg("index")?)?, new_m_rd_nm: dec_json(&arg("new-m-rd-nm")?)? })),
        "remove-bridge" => Ok(En1998Mutation::RemoveBridge(RemoveBridge { index: dec_json(&arg("index")?)? })),
        "remove-assessment" => Ok(En1998Mutation::RemoveAssessment(RemoveAssessment { index: dec_json(&arg("index")?)? })),
        "remove-silo" => Ok(En1998Mutation::RemoveSilo(RemoveSilo { index: dec_json(&arg("index")?)? })),
        "remove-tank" => Ok(En1998Mutation::RemoveTank(RemoveTank { index: dec_json(&arg("index")?)? })),
        "remove-foundation" => Ok(En1998Mutation::RemoveFoundation(RemoveFoundation { index: dec_json(&arg("index")?)? })),
        "remove-retaining-wall" => Ok(En1998Mutation::RemoveRetainingWall(RemoveRetainingWall { index: dec_json(&arg("index")?)? })),
        "remove-tower" => Ok(En1998Mutation::RemoveTower(RemoveTower { index: dec_json(&arg("index")?)? })),
        other => Err(format!("en1998 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for En1998Mutation {
    fn print_op(&self) -> String {
        print_en1998_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_en1998_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}





































#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1998Mutation> {
    let snap = crate::En1998Snapshot::default();
    let building = snap.buildings[0].clone();
    vec![
        En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex: String::from("en") }),
        En1998Mutation::UpdateSite(UpdateSite { site: snap.site.clone() }),
        En1998Mutation::InsertBuilding(InsertBuilding { index: Some(0usize), building: building.clone() }),
        En1998Mutation::RemoveBuilding(RemoveBuilding { index: 0usize }),
        En1998Mutation::ChangeSystemVRdN(ChangeSystemVRdN {
            building_index: 0usize,
            system_index: 0usize,
            new_base_shear_resistance_n: 1.0e6,
        }),
        En1998Mutation::ChangeStoreyPermanentGkN(ChangeStoreyPermanentGkN {
            building_index: 0usize,
            storey_index: 0usize,
            new_permanent_gk_n: 1.0e5,
        }),
        En1998Mutation::ChangeStoreyStiffnessX(ChangeStoreyStiffnessX {
            building_index: 0usize,
            storey_index: 0usize,
            new_stiffness_x: 2.0e7,
        }),
        En1998Mutation::ChangeStoreyDriftXM(ChangeStoreyDriftXM {
            building_index: 0usize,
            storey_index: 0usize,
            new_drift_x_m: 0.012,
        }),
        En1998Mutation::ChangeBuildingPlanRegular(ChangeBuildingPlanRegular {
            building_index: 0usize,
            new_plan_regular: false,
        }),
        En1998Mutation::ChangeElevationRegular(ChangeElevationRegular {
            building_index: 0usize,
            new_elevation_regular: false,
        }),
        En1998Mutation::ChangeMemberDetailing(ChangeMemberDetailing {
            building_index: 0usize,
            member_index: 0usize,
            new_detailing_compatible_with_q: false,
        }),
        En1998Mutation::ChangeMasonryWallRatio(ChangeMasonryWallRatio {
            building_index: 0usize,
            new_masonry_wall_area_ratio: 0.05,
        }),
        En1998Mutation::InsertBridge(InsertBridge {
            index: Some(0usize),
            bridge: crate::En1998Bridge {
                id: "br-demo".into(),
                period_ratio: 2.0,
                fundamental_period_s: 1.5,
                v_rd_n: 5.0e5,
                bearing_d_rd_m: 0.25,
                permanent_gk_n: 9.81e6,
                correlated_occupancy: true,
                variables: vec![crate::En1998VariableAction { id: "br-q".into(), category: "F".into(), qk_n: 1.0e6 }],
            },
        }),
        En1998Mutation::ChangeBridgeVRdN(ChangeBridgeVRdN { index: 0usize, new_v_rd_n: 5.0e5 }),
        En1998Mutation::InsertAssessment(InsertAssessment {
            index: Some(0usize),
            assessment: crate::En1998Assessment {
                id: "as-demo".into(),
                knowledge_level: "kl2".into(),
                limit_state: "sd".into(),
                supported_building_id: "bldg-office".into(),
                r_k_n: 4.0e5,
                gamma_el: 1.0,
            },
        }),
        En1998Mutation::ChangeAssessmentRKN(ChangeAssessmentRKN { index: 0usize, new_r_k_n: 4.0e5 }),
        En1998Mutation::InsertSilo(InsertSilo {
            index: Some(0usize),
            silo: crate::En1998Silo {
                id: "si-demo".into(),
                height_m: 10.0,
                radius_m: 5.0,
                permanent_gk_n: 9.81e5,
                content_qk_n: 3.924e6,
                content_category: "E".into(),
                filling_ratio: 0.9,
                n_rd_n: 5.0e5,
                v_rd_n: 3.0e5,
                q_nominal: 2.0,
            },
        }),
        En1998Mutation::InsertTank(InsertTank {
            index: Some(0usize),
            tank: crate::En1998Tank {
                id: "tk-demo".into(),
                height_m: 8.0,
                radius_m: 4.0,
                permanent_gk_n: 4.905e5,
                content_qk_n: 2.4525e6,
                content_category: "E".into(),
                filling_ratio: 0.85,
                v_rd_n: 4.0e5,
            },
        }),
        En1998Mutation::InsertFoundation(InsertFoundation {
            index: Some(0usize),
            foundation: crate::En1998Foundation {
                supported_building_id: String::new(),
                id: "fd-demo".into(),
                area_m2: 100.0,
                p_rd_pa: 5.0e5,
                h_rd_n: 4.0e5,
                k_foundation: 5.0e5,
                k_soil: 2.0e5,
            },
        }),
        En1998Mutation::InsertRetainingWall(InsertRetainingWall {
            index: Some(0usize),
            wall: crate::En1998RetainingWall {
                id: "rw-demo".into(),
                height_m: 4.0,
                phi_deg: 30.0,
                soil_gamma: 18000.0,
                r: 1.5,
                h_rd_n_per_m: 1.5e5,
            },
        }),
        En1998Mutation::InsertTower(InsertTower {
            index: Some(0usize),
            tower: crate::En1998Tower {
                id: "tw-demo".into(),
                height_m: 40.0,
                m_rd_nm: 2.5e6,
                is_chimney: true,
                q_nominal: 2.5,
                permanent_gk_n: 7.848e5,
                correlated_occupancy: true,
                variables: vec![],
            },
        }),
        En1998Mutation::ChangeTowerMRdNm(ChangeTowerMRdNm { index: 0usize, new_m_rd_nm: 2.5e6 }),
        En1998Mutation::RemoveBridge(RemoveBridge { index: 0usize }),
        En1998Mutation::RemoveAssessment(RemoveAssessment { index: 0usize }),
        En1998Mutation::RemoveSilo(RemoveSilo { index: 0usize }),
        En1998Mutation::RemoveTank(RemoveTank { index: 0usize }),
        En1998Mutation::RemoveFoundation(RemoveFoundation { index: 0usize }),
        En1998Mutation::RemoveRetainingWall(RemoveRetainingWall { index: 0usize }),
        En1998Mutation::RemoveTower(RemoveTower { index: 0usize }),
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::diff::En1998Diff;
use crate::En1998Snapshot;
use crate::standards::v1::subsets::any::schema::mutations::change_annex;
use crate::standards::v1::subsets::any::schema::mutations::update_site;
use crate::standards::v1::subsets::any::schema::mutations::insert_building;
use crate::standards::v1::subsets::any::schema::mutations::remove_building;
use crate::standards::v1::subsets::any::schema::mutations::change_system_v_rd_n;
use crate::standards::v1::subsets::any::schema::mutations::change_storey_permanent_gk_n;
use crate::standards::v1::subsets::any::schema::mutations::change_storey_stiffness_x;
use crate::standards::v1::subsets::any::schema::mutations::change_storey_drift_xm;
use crate::standards::v1::subsets::any::schema::mutations::change_building_plan_regular;
use crate::standards::v1::subsets::any::schema::mutations::change_elevation_regular;
use crate::standards::v1::subsets::any::schema::mutations::change_member_detailing;
use crate::standards::v1::subsets::any::schema::mutations::change_masonry_wall_ratio;
use crate::standards::v1::subsets::any::schema::mutations::insert_bridge;
use crate::standards::v1::subsets::any::schema::mutations::change_bridge_v_rd_n;
use crate::standards::v1::subsets::any::schema::mutations::insert_assessment;
use crate::standards::v1::subsets::any::schema::mutations::change_assessment_rkn;
use crate::standards::v1::subsets::any::schema::mutations::insert_silo;
use crate::standards::v1::subsets::any::schema::mutations::insert_tank;
use crate::standards::v1::subsets::any::schema::mutations::insert_foundation;
use crate::standards::v1::subsets::any::schema::mutations::insert_retaining_wall;
use crate::standards::v1::subsets::any::schema::mutations::insert_tower;
use crate::standards::v1::subsets::any::schema::mutations::change_tower_m_rd_nm;
use crate::standards::v1::subsets::any::schema::mutations::remove_bridge;
use crate::standards::v1::subsets::any::schema::mutations::remove_assessment;
use crate::standards::v1::subsets::any::schema::mutations::remove_silo;
use crate::standards::v1::subsets::any::schema::mutations::remove_tank;
use crate::standards::v1::subsets::any::schema::mutations::remove_foundation;
use crate::standards::v1::subsets::any::schema::mutations::remove_retaining_wall;
use crate::standards::v1::subsets::any::schema::mutations::remove_tower;

/// 📥️ Decodes one committed mutation JSON document into [`En1998Mutation`] — the bridge the repository test host reaches, since it links no codec of its own.
pub fn decode_en1998_mutation_json(text: &str) -> Result<En1998Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
