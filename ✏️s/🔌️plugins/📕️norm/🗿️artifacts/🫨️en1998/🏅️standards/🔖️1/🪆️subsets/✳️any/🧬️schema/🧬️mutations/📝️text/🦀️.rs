//! ⚡️ EN 1998 — hand-rolled OpText/OpBinary for `En1998Mutation` (B2 JSON-atom field codec).

pub use crate::artifact_schema::mutations::En1998Mutation;

use crate::artifact_schema::mutations::{
    change_annex::ChangeAnnex, update_site::UpdateSite, insert_building::InsertBuilding, remove_building::RemoveBuilding,
    change_system_base_shear_resistance_n::ChangeSystemBaseShearResistanceN, change_storey_permanent_gk_n::ChangeStoreyPermanentGkN,
    change_storey_stiffness_x::ChangeStoreyStiffnessX, change_storey_drift_xm::ChangeStoreyDriftXM,
    change_building_plan_regular::ChangeBuildingPlanRegular, change_building_elevation_regular::ChangeBuildingElevationRegular,
    change_member_detailing_compatible::ChangeMemberDetailingCompatible,
    change_building_masonry_wall_area_ratio::ChangeBuildingMasonryWallAreaRatio, insert_bridge::InsertBridge,
    change_bridge_v_rd_n::ChangeBridgeVRdN, insert_assessment::InsertAssessment, change_assessment_rkn::ChangeAssessmentRKN,
    insert_silo::InsertSilo, insert_tank::InsertTank, insert_foundation::InsertFoundation,
    insert_retaining_wall::InsertRetainingWall, insert_tower::InsertTower, change_tower_m_rd_nm::ChangeTowerMRdNm,
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
fn enc_json<T: dsl::ToValue>(value: &T) -> String {
    enc_str(&pack::json::to_json_string(value))
}
fn dec_json<T: dsl::FromValue>(s: &str) -> Result<T, String> {
    pack::json::from_json_str(&dec_str(s)?).map_err(|e| e.to_string())
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
        En1998Mutation::ChangeSystemBaseShearResistanceN(p) => format!("change-system-base-shear-resistance-n building-index={} system-index={} new-base-shear-resistance-n={}", enc_json(&p.building_index), enc_json(&p.system_index), enc_json(&p.new_base_shear_resistance_n)),
        En1998Mutation::ChangeStoreyPermanentGkN(p) => format!("change-storey-permanent-gk-n building-index={} storey-index={} new-permanent-gk-n={}", enc_json(&p.building_index), enc_json(&p.storey_index), enc_json(&p.new_permanent_gk_n)),
        En1998Mutation::ChangeStoreyStiffnessX(p) => format!("change-storey-stiffness-x building-index={} storey-index={} new-stiffness-x={}", enc_json(&p.building_index), enc_json(&p.storey_index), enc_json(&p.new_stiffness_x)),
        En1998Mutation::ChangeStoreyDriftXM(p) => format!("change-storey-drift-xm building-index={} storey-index={} new-drift-x-m={}", enc_json(&p.building_index), enc_json(&p.storey_index), enc_json(&p.new_drift_x_m)),
        En1998Mutation::ChangeBuildingPlanRegular(p) => format!("change-building-plan-regular building-index={} new-plan-regular={}", enc_json(&p.building_index), enc_json(&p.new_plan_regular)),
        En1998Mutation::ChangeBuildingElevationRegular(p) => format!("change-building-elevation-regular building-index={} new-elevation-regular={}", enc_json(&p.building_index), enc_json(&p.new_elevation_regular)),
        En1998Mutation::ChangeMemberDetailingCompatible(p) => format!("change-member-detailing-compatible building-index={} member-index={} new-detailing-compatible-with-q={}", enc_json(&p.building_index), enc_json(&p.member_index), enc_json(&p.new_detailing_compatible_with_q)),
        En1998Mutation::ChangeBuildingMasonryWallAreaRatio(p) => format!("change-building-masonry-wall-area-ratio building-index={} new-masonry-wall-area-ratio={}", enc_json(&p.building_index), enc_json(&p.new_masonry_wall_area_ratio)),
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
    }
}

fn parse_en1998_mutation(line: &str) -> Result<En1998Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("en1998 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-annex" => Ok(En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex: dec_json(&arg("new-annex")?)? })),
        "update-site" => Ok(En1998Mutation::UpdateSite(UpdateSite { site: dec_json(&arg("site")?)? })),
        "insert-building" => Ok(En1998Mutation::InsertBuilding(InsertBuilding { index: dec_json(&arg("index")?)?, building: dec_json(&arg("building")?)? })),
        "remove-building" => Ok(En1998Mutation::RemoveBuilding(RemoveBuilding { index: dec_json(&arg("index")?)? })),
        "change-system-base-shear-resistance-n" => Ok(En1998Mutation::ChangeSystemBaseShearResistanceN(ChangeSystemBaseShearResistanceN { building_index: dec_json(&arg("building-index")?)?, system_index: dec_json(&arg("system-index")?)?, new_base_shear_resistance_n: dec_json(&arg("new-base-shear-resistance-n")?)? })),
        "change-storey-permanent-gk-n" => Ok(En1998Mutation::ChangeStoreyPermanentGkN(ChangeStoreyPermanentGkN { building_index: dec_json(&arg("building-index")?)?, storey_index: dec_json(&arg("storey-index")?)?, new_permanent_gk_n: dec_json(&arg("new-permanent-gk-n")?)? })),
        "change-storey-stiffness-x" => Ok(En1998Mutation::ChangeStoreyStiffnessX(ChangeStoreyStiffnessX { building_index: dec_json(&arg("building-index")?)?, storey_index: dec_json(&arg("storey-index")?)?, new_stiffness_x: dec_json(&arg("new-stiffness-x")?)? })),
        "change-storey-drift-xm" => Ok(En1998Mutation::ChangeStoreyDriftXM(ChangeStoreyDriftXM { building_index: dec_json(&arg("building-index")?)?, storey_index: dec_json(&arg("storey-index")?)?, new_drift_x_m: dec_json(&arg("new-drift-x-m")?)? })),
        "change-building-plan-regular" => Ok(En1998Mutation::ChangeBuildingPlanRegular(ChangeBuildingPlanRegular { building_index: dec_json(&arg("building-index")?)?, new_plan_regular: dec_json(&arg("new-plan-regular")?)? })),
        "change-building-elevation-regular" => Ok(En1998Mutation::ChangeBuildingElevationRegular(ChangeBuildingElevationRegular { building_index: dec_json(&arg("building-index")?)?, new_elevation_regular: dec_json(&arg("new-elevation-regular")?)? })),
        "change-member-detailing-compatible" => Ok(En1998Mutation::ChangeMemberDetailingCompatible(ChangeMemberDetailingCompatible { building_index: dec_json(&arg("building-index")?)?, member_index: dec_json(&arg("member-index")?)?, new_detailing_compatible_with_q: dec_json(&arg("new-detailing-compatible-with-q")?)? })),
        "change-building-masonry-wall-area-ratio" => Ok(En1998Mutation::ChangeBuildingMasonryWallAreaRatio(ChangeBuildingMasonryWallAreaRatio { building_index: dec_json(&arg("building-index")?)?, new_masonry_wall_area_ratio: dec_json(&arg("new-masonry-wall-area-ratio")?)? })),
        "insert-bridge" => Ok(En1998Mutation::InsertBridge(InsertBridge { index: dec_json(&arg("index")?)?, bridge: dec_json(&arg("bridge")?)? })),
        "change-bridge-v-rd-n" => Ok(En1998Mutation::ChangeBridgeVRdN(ChangeBridgeVRdN { index: dec_json(&arg("index")?)?, new_v_rd_n: dec_json(&arg("new-v-rd-n")?)? })),
        "insert-assessment" => Ok(En1998Mutation::InsertAssessment(InsertAssessment { index: dec_json(&arg("index")?)?, assessment: dec_json(&arg("assessment")?)? })),
        "change-assessment-rkn" => Ok(En1998Mutation::ChangeAssessmentRKN(ChangeAssessmentRKN { index: dec_json(&arg("index")?)?, new_r_k_n: dec_json(&arg("new-r-k-n")?)? })),
        "insert-silo" => Ok(En1998Mutation::InsertSilo(InsertSilo { index: dec_json(&arg("index")?)?, silo: dec_json(&arg("silo")?)? })),
        "insert-tank" => Ok(En1998Mutation::InsertTank(InsertTank { index: dec_json(&arg("index")?)?, tank: dec_json(&arg("tank")?)? })),
        "insert-foundation" => Ok(En1998Mutation::InsertFoundation(InsertFoundation { index: dec_json(&arg("index")?)?, foundation: dec_json(&arg("foundation")?)? })),
        "insert-retaining-wall" => Ok(En1998Mutation::InsertRetainingWall(InsertRetainingWall { index: dec_json(&arg("index")?)?, wall: dec_json(&arg("wall")?)? })),
        "insert-tower" => Ok(En1998Mutation::InsertTower(InsertTower { index: dec_json(&arg("index")?)?, tower: dec_json(&arg("tower")?)? })),
        "change-tower-m-rd-nm" => Ok(En1998Mutation::ChangeTowerMRdNm(ChangeTowerMRdNm { index: dec_json(&arg("index")?)?, new_m_rd_nm: dec_json(&arg("new-m-rd-nm")?)? })),
        other => Err(format!("en1998 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for En1998Mutation {
    fn print_op(&self) -> String {
        print_en1998_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_en1998_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}

fn write_json_bin<T: dsl::ToValue>(out: &mut Vec<u8>, value: &T) {
    let bytes = pack::json::to_json_string(value);
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes.as_bytes());
}
fn read_json_bin<T: dsl::FromValue>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    pack::json::from_json_str(text).map_err(|e| e.to_string())
}

const WIRE_PROTOCOL: &str = include_str!("../💾️binary/📡️.protocol.semio");
const TAG_CHANGE_ANNEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-annex");
const TAG_UPDATE_SITE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-site");
const TAG_INSERT_BUILDING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-building");
const TAG_REMOVE_BUILDING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-building");
const TAG_CHANGE_SYSTEM_BASE_SHEAR_RESISTANCE_N: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-system-base-shear-resistance-n");
const TAG_CHANGE_STOREY_PERMANENT_GK_N: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-storey-permanent-gk-n");
const TAG_CHANGE_STOREY_STIFFNESS_X: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-storey-stiffness-x");
const TAG_CHANGE_STOREY_DRIFT_X_M: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-storey-drift-xm");
const TAG_CHANGE_BUILDING_PLAN_REGULAR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-building-plan-regular");
const TAG_CHANGE_BUILDING_ELEVATION_REGULAR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-building-elevation-regular");
const TAG_CHANGE_MEMBER_DETAILING_COMPATIBLE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-member-detailing-compatible");
const TAG_CHANGE_BUILDING_MASONRY_WALL_AREA_RATIO: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-building-masonry-wall-area-ratio");
const TAG_INSERT_BRIDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-bridge");
const TAG_CHANGE_BRIDGE_V_RD_N: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-bridge-v-rd-n");
const TAG_INSERT_ASSESSMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-assessment");
const TAG_CHANGE_ASSESSMENT_R_K_N: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-assessment-rkn");
const TAG_INSERT_SILO: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-silo");
const TAG_INSERT_TANK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-tank");
const TAG_INSERT_FOUNDATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-foundation");
const TAG_INSERT_RETAINING_WALL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-retaining-wall");
const TAG_INSERT_TOWER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-tower");
const TAG_CHANGE_TOWER_M_RD_NM: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-tower-m-rd-nm");

impl protocol::OpBinary for En1998Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1998Mutation::ChangeAnnex(_) => TAG_CHANGE_ANNEX,
            En1998Mutation::UpdateSite(_) => TAG_UPDATE_SITE,
            En1998Mutation::InsertBuilding(_) => TAG_INSERT_BUILDING,
            En1998Mutation::RemoveBuilding(_) => TAG_REMOVE_BUILDING,
            En1998Mutation::ChangeSystemBaseShearResistanceN(_) => TAG_CHANGE_SYSTEM_BASE_SHEAR_RESISTANCE_N,
            En1998Mutation::ChangeStoreyPermanentGkN(_) => TAG_CHANGE_STOREY_PERMANENT_GK_N,
            En1998Mutation::ChangeStoreyStiffnessX(_) => TAG_CHANGE_STOREY_STIFFNESS_X,
            En1998Mutation::ChangeStoreyDriftXM(_) => TAG_CHANGE_STOREY_DRIFT_X_M,
            En1998Mutation::ChangeBuildingPlanRegular(_) => TAG_CHANGE_BUILDING_PLAN_REGULAR,
            En1998Mutation::ChangeBuildingElevationRegular(_) => TAG_CHANGE_BUILDING_ELEVATION_REGULAR,
            En1998Mutation::ChangeMemberDetailingCompatible(_) => TAG_CHANGE_MEMBER_DETAILING_COMPATIBLE,
            En1998Mutation::ChangeBuildingMasonryWallAreaRatio(_) => TAG_CHANGE_BUILDING_MASONRY_WALL_AREA_RATIO,
            En1998Mutation::InsertBridge(_) => TAG_INSERT_BRIDGE,
            En1998Mutation::ChangeBridgeVRdN(_) => TAG_CHANGE_BRIDGE_V_RD_N,
            En1998Mutation::InsertAssessment(_) => TAG_INSERT_ASSESSMENT,
            En1998Mutation::ChangeAssessmentRKN(_) => TAG_CHANGE_ASSESSMENT_R_K_N,
            En1998Mutation::InsertSilo(_) => TAG_INSERT_SILO,
            En1998Mutation::InsertTank(_) => TAG_INSERT_TANK,
            En1998Mutation::InsertFoundation(_) => TAG_INSERT_FOUNDATION,
            En1998Mutation::InsertRetainingWall(_) => TAG_INSERT_RETAINING_WALL,
            En1998Mutation::InsertTower(_) => TAG_INSERT_TOWER,
            En1998Mutation::ChangeTowerMRdNm(_) => TAG_CHANGE_TOWER_M_RD_NM,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            En1998Mutation::ChangeAnnex(p) => write_json_bin(&mut out, &p.new_annex),
            En1998Mutation::UpdateSite(p) => write_json_bin(&mut out, &p.site),
            En1998Mutation::InsertBuilding(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.building);
            }
            En1998Mutation::RemoveBuilding(p) => write_json_bin(&mut out, &p.index),
            En1998Mutation::ChangeSystemBaseShearResistanceN(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.system_index);
                write_json_bin(&mut out, &p.new_base_shear_resistance_n);
            }
            En1998Mutation::ChangeStoreyPermanentGkN(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.storey_index);
                write_json_bin(&mut out, &p.new_permanent_gk_n);
            }
            En1998Mutation::ChangeStoreyStiffnessX(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.storey_index);
                write_json_bin(&mut out, &p.new_stiffness_x);
            }
            En1998Mutation::ChangeStoreyDriftXM(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.storey_index);
                write_json_bin(&mut out, &p.new_drift_x_m);
            }
            En1998Mutation::ChangeBuildingPlanRegular(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.new_plan_regular);
            }
            En1998Mutation::ChangeBuildingElevationRegular(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.new_elevation_regular);
            }
            En1998Mutation::ChangeMemberDetailingCompatible(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.member_index);
                write_json_bin(&mut out, &p.new_detailing_compatible_with_q);
            }
            En1998Mutation::ChangeBuildingMasonryWallAreaRatio(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.new_masonry_wall_area_ratio);
            }
            En1998Mutation::InsertBridge(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.bridge);
            }
            En1998Mutation::ChangeBridgeVRdN(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.new_v_rd_n);
            }
            En1998Mutation::InsertAssessment(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.assessment);
            }
            En1998Mutation::ChangeAssessmentRKN(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.new_r_k_n);
            }
            En1998Mutation::InsertSilo(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.silo);
            }
            En1998Mutation::InsertTank(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.tank);
            }
            En1998Mutation::InsertFoundation(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.foundation);
            }
            En1998Mutation::InsertRetainingWall(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.wall);
            }
            En1998Mutation::InsertTower(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.tower);
            }
            En1998Mutation::ChangeTowerMRdNm(p) => {
                write_json_bin(&mut out, &p.index);
                write_json_bin(&mut out, &p.new_m_rd_nm);
            }
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_CHANGE_ANNEX => {
                let new_annex = read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))?;
                Ok(En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex }))
            }
            TAG_UPDATE_SITE => {
                let site = read_json_bin(&mut reader).map_err(|e| malformed("site", reader.position(), e))?;
                Ok(En1998Mutation::UpdateSite(UpdateSite { site }))
            }
            TAG_INSERT_BUILDING => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let building = read_json_bin(&mut reader).map_err(|e| malformed("building", reader.position(), e))?;
                Ok(En1998Mutation::InsertBuilding(InsertBuilding { index, building }))
            }
            TAG_REMOVE_BUILDING => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveBuilding(RemoveBuilding { index }))
            }
            TAG_CHANGE_SYSTEM_BASE_SHEAR_RESISTANCE_N => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let system_index = read_json_bin(&mut reader).map_err(|e| malformed("system_index", reader.position(), e))?;
                let new_base_shear_resistance_n = read_json_bin(&mut reader).map_err(|e| malformed("new_base_shear_resistance_n", reader.position(), e))?;
                Ok(En1998Mutation::ChangeSystemBaseShearResistanceN(ChangeSystemBaseShearResistanceN { building_index, system_index, new_base_shear_resistance_n }))
            }
            TAG_CHANGE_STOREY_PERMANENT_GK_N => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let storey_index = read_json_bin(&mut reader).map_err(|e| malformed("storey_index", reader.position(), e))?;
                let new_permanent_gk_n = read_json_bin(&mut reader).map_err(|e| malformed("new_permanent_gk_n", reader.position(), e))?;
                Ok(En1998Mutation::ChangeStoreyPermanentGkN(ChangeStoreyPermanentGkN { building_index, storey_index, new_permanent_gk_n }))
            }
            TAG_CHANGE_STOREY_STIFFNESS_X => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let storey_index = read_json_bin(&mut reader).map_err(|e| malformed("storey_index", reader.position(), e))?;
                let new_stiffness_x = read_json_bin(&mut reader).map_err(|e| malformed("new_stiffness_x", reader.position(), e))?;
                Ok(En1998Mutation::ChangeStoreyStiffnessX(ChangeStoreyStiffnessX { building_index, storey_index, new_stiffness_x }))
            }
            TAG_CHANGE_STOREY_DRIFT_X_M => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let storey_index = read_json_bin(&mut reader).map_err(|e| malformed("storey_index", reader.position(), e))?;
                let new_drift_x_m = read_json_bin(&mut reader).map_err(|e| malformed("new_drift_x_m", reader.position(), e))?;
                Ok(En1998Mutation::ChangeStoreyDriftXM(ChangeStoreyDriftXM { building_index, storey_index, new_drift_x_m }))
            }
            TAG_CHANGE_BUILDING_PLAN_REGULAR => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let new_plan_regular = read_json_bin(&mut reader).map_err(|e| malformed("new_plan_regular", reader.position(), e))?;
                Ok(En1998Mutation::ChangeBuildingPlanRegular(ChangeBuildingPlanRegular { building_index, new_plan_regular }))
            }
            TAG_CHANGE_BUILDING_ELEVATION_REGULAR => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let new_elevation_regular = read_json_bin(&mut reader).map_err(|e| malformed("new_elevation_regular", reader.position(), e))?;
                Ok(En1998Mutation::ChangeBuildingElevationRegular(ChangeBuildingElevationRegular { building_index, new_elevation_regular }))
            }
            TAG_CHANGE_MEMBER_DETAILING_COMPATIBLE => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let member_index = read_json_bin(&mut reader).map_err(|e| malformed("member_index", reader.position(), e))?;
                let new_detailing_compatible_with_q = read_json_bin(&mut reader).map_err(|e| malformed("new_detailing_compatible_with_q", reader.position(), e))?;
                Ok(En1998Mutation::ChangeMemberDetailingCompatible(ChangeMemberDetailingCompatible { building_index, member_index, new_detailing_compatible_with_q }))
            }
            TAG_CHANGE_BUILDING_MASONRY_WALL_AREA_RATIO => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let new_masonry_wall_area_ratio = read_json_bin(&mut reader).map_err(|e| malformed("new_masonry_wall_area_ratio", reader.position(), e))?;
                Ok(En1998Mutation::ChangeBuildingMasonryWallAreaRatio(ChangeBuildingMasonryWallAreaRatio { building_index, new_masonry_wall_area_ratio }))
            }
            TAG_INSERT_BRIDGE => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let bridge = read_json_bin(&mut reader).map_err(|e| malformed("bridge", reader.position(), e))?;
                Ok(En1998Mutation::InsertBridge(InsertBridge { index, bridge }))
            }
            TAG_CHANGE_BRIDGE_V_RD_N => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let new_v_rd_n = read_json_bin(&mut reader).map_err(|e| malformed("new_v_rd_n", reader.position(), e))?;
                Ok(En1998Mutation::ChangeBridgeVRdN(ChangeBridgeVRdN { index, new_v_rd_n }))
            }
            TAG_INSERT_ASSESSMENT => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let assessment = read_json_bin(&mut reader).map_err(|e| malformed("assessment", reader.position(), e))?;
                Ok(En1998Mutation::InsertAssessment(InsertAssessment { index, assessment }))
            }
            TAG_CHANGE_ASSESSMENT_R_K_N => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let new_r_k_n = read_json_bin(&mut reader).map_err(|e| malformed("new_r_k_n", reader.position(), e))?;
                Ok(En1998Mutation::ChangeAssessmentRKN(ChangeAssessmentRKN { index, new_r_k_n }))
            }
            TAG_INSERT_SILO => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let silo = read_json_bin(&mut reader).map_err(|e| malformed("silo", reader.position(), e))?;
                Ok(En1998Mutation::InsertSilo(InsertSilo { index, silo }))
            }
            TAG_INSERT_TANK => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let tank = read_json_bin(&mut reader).map_err(|e| malformed("tank", reader.position(), e))?;
                Ok(En1998Mutation::InsertTank(InsertTank { index, tank }))
            }
            TAG_INSERT_FOUNDATION => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let foundation = read_json_bin(&mut reader).map_err(|e| malformed("foundation", reader.position(), e))?;
                Ok(En1998Mutation::InsertFoundation(InsertFoundation { index, foundation }))
            }
            TAG_INSERT_RETAINING_WALL => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let wall = read_json_bin(&mut reader).map_err(|e| malformed("wall", reader.position(), e))?;
                Ok(En1998Mutation::InsertRetainingWall(InsertRetainingWall { index, wall }))
            }
            TAG_INSERT_TOWER => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let tower = read_json_bin(&mut reader).map_err(|e| malformed("tower", reader.position(), e))?;
                Ok(En1998Mutation::InsertTower(InsertTower { index, tower }))
            }
            TAG_CHANGE_TOWER_M_RD_NM => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                let new_m_rd_nm = read_json_bin(&mut reader).map_err(|e| malformed("new_m_rd_nm", reader.position(), e))?;
                Ok(En1998Mutation::ChangeTowerMRdNm(ChangeTowerMRdNm { index, new_m_rd_nm }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}

#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1998Mutation> {
    let snap = crate::En1998Snapshot::default();
    let building = snap.buildings[0].clone();
    vec![
        En1998Mutation::ChangeAnnex(ChangeAnnex { new_annex: String::from("en") }),
        En1998Mutation::UpdateSite(UpdateSite { site: snap.site.clone() }),
        En1998Mutation::InsertBuilding(InsertBuilding { index: 0usize, building: building.clone() }),
        En1998Mutation::RemoveBuilding(RemoveBuilding { index: 0usize }),
        En1998Mutation::ChangeSystemBaseShearResistanceN(ChangeSystemBaseShearResistanceN {
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
        En1998Mutation::ChangeBuildingElevationRegular(ChangeBuildingElevationRegular {
            building_index: 0usize,
            new_elevation_regular: false,
        }),
        En1998Mutation::ChangeMemberDetailingCompatible(ChangeMemberDetailingCompatible {
            building_index: 0usize,
            member_index: 0usize,
            new_detailing_compatible_with_q: false,
        }),
        En1998Mutation::ChangeBuildingMasonryWallAreaRatio(ChangeBuildingMasonryWallAreaRatio {
            building_index: 0usize,
            new_masonry_wall_area_ratio: 0.05,
        }),
        En1998Mutation::InsertBridge(InsertBridge {
            index: 0usize,
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
            index: 0usize,
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
            index: 0usize,
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
            index: 0usize,
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
            index: 0usize,
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
            index: 0usize,
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
            index: 0usize,
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
    ]
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
