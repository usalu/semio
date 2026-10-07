//! ⚖️ En1998 app — binary command protocol surface + laws (constitutional: protocol).

//#region 📡️SemioProtocol
/// 📡️ Normative handcrafted binary protocol for this facet (`dialect protocol`).
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
//#endregion 📡️SemioProtocol

use crate::artifact_schema::mutations::En1998Mutation;
use protocol::OpBinary;

/// 📦️ Encodes a document mutation to its binary op form.
pub fn encode_op(mutation: &En1998Mutation) -> Result<Vec<u8>, protocol::ProtocolError> {
    mutation.encode_op()
}

/// 📖️ Decodes a document mutation from its binary op form.
pub fn decode_op(bytes: &[u8]) -> Result<En1998Mutation, protocol::ProtocolError> {
    En1998Mutation::decode_op(bytes)
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

mod native_codec {
use super::*;
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
fn write_json_bin<T: semio_framework_value::ToValue>(out: &mut Vec<u8>, value: &T) {
    let bytes = semio_framework_pack_json::to_json_string(value);
    store::pack_rt::write_varint_u64(out, bytes.len() as u64);
    out.extend_from_slice(bytes.as_bytes());
}

fn read_json_bin<T: semio_framework_value::FromValue>(reader: &mut store::ByteReader<'_>) -> Result<T, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    let text = std::str::from_utf8(bytes).map_err(|e| e.to_string())?;
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}

const TAG_CHANGE_ANNEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-annex");

const TAG_UPDATE_SITE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-site");

const TAG_INSERT_BUILDING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-building");

const TAG_REMOVE_BUILDING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-building");

const TAG_CHANGE_SYSTEM_V_RD_N: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-system-v-rd-n");

const TAG_CHANGE_STOREY_PERMANENT_GK_N: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-storey-permanent-gk-n");

const TAG_CHANGE_STOREY_STIFFNESS_X: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-storey-stiffness-x");

const TAG_CHANGE_STOREY_DRIFT_X_M: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-storey-drift-xm");

const TAG_CHANGE_BUILDING_PLAN_REGULAR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-building-plan-regular");

const TAG_CHANGE_ELEVATION_REGULAR: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-elevation-regular");

const TAG_CHANGE_MEMBER_DETAILING: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-member-detailing");

const TAG_CHANGE_MASONRY_WALL_RATIO: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-masonry-wall-ratio");

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

const TAG_REMOVE_BRIDGE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-bridge");

const TAG_REMOVE_ASSESSMENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-assessment");

const TAG_REMOVE_SILO: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-silo");

const TAG_REMOVE_TANK: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-tank");

const TAG_REMOVE_FOUNDATION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-foundation");

const TAG_REMOVE_RETAINING_WALL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-retaining-wall");

const TAG_REMOVE_TOWER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-tower");

const WIRE_PROTOCOL: &str = include_str!("📡️.protocol.semio");
impl protocol::OpBinary for En1998Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1998Mutation::ChangeAnnex(_) => TAG_CHANGE_ANNEX,
            En1998Mutation::UpdateSite(_) => TAG_UPDATE_SITE,
            En1998Mutation::InsertBuilding(_) => TAG_INSERT_BUILDING,
            En1998Mutation::RemoveBuilding(_) => TAG_REMOVE_BUILDING,
            En1998Mutation::ChangeSystemVRdN(_) => TAG_CHANGE_SYSTEM_V_RD_N,
            En1998Mutation::ChangeStoreyPermanentGkN(_) => TAG_CHANGE_STOREY_PERMANENT_GK_N,
            En1998Mutation::ChangeStoreyStiffnessX(_) => TAG_CHANGE_STOREY_STIFFNESS_X,
            En1998Mutation::ChangeStoreyDriftXM(_) => TAG_CHANGE_STOREY_DRIFT_X_M,
            En1998Mutation::ChangeBuildingPlanRegular(_) => TAG_CHANGE_BUILDING_PLAN_REGULAR,
            En1998Mutation::ChangeElevationRegular(_) => TAG_CHANGE_ELEVATION_REGULAR,
            En1998Mutation::ChangeMemberDetailing(_) => TAG_CHANGE_MEMBER_DETAILING,
            En1998Mutation::ChangeMasonryWallRatio(_) => TAG_CHANGE_MASONRY_WALL_RATIO,
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
            En1998Mutation::RemoveBridge(_) => TAG_REMOVE_BRIDGE,
            En1998Mutation::RemoveAssessment(_) => TAG_REMOVE_ASSESSMENT,
            En1998Mutation::RemoveSilo(_) => TAG_REMOVE_SILO,
            En1998Mutation::RemoveTank(_) => TAG_REMOVE_TANK,
            En1998Mutation::RemoveFoundation(_) => TAG_REMOVE_FOUNDATION,
            En1998Mutation::RemoveRetainingWall(_) => TAG_REMOVE_RETAINING_WALL,
            En1998Mutation::RemoveTower(_) => TAG_REMOVE_TOWER,
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
            En1998Mutation::ChangeSystemVRdN(p) => {
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
            En1998Mutation::ChangeElevationRegular(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.new_elevation_regular);
            }
            En1998Mutation::ChangeMemberDetailing(p) => {
                write_json_bin(&mut out, &p.building_index);
                write_json_bin(&mut out, &p.member_index);
                write_json_bin(&mut out, &p.new_detailing_compatible_with_q);
            }
            En1998Mutation::ChangeMasonryWallRatio(p) => {
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
            En1998Mutation::RemoveBridge(p) => write_json_bin(&mut out, &p.index),
            En1998Mutation::RemoveAssessment(p) => write_json_bin(&mut out, &p.index),
            En1998Mutation::RemoveSilo(p) => write_json_bin(&mut out, &p.index),
            En1998Mutation::RemoveTank(p) => write_json_bin(&mut out, &p.index),
            En1998Mutation::RemoveFoundation(p) => write_json_bin(&mut out, &p.index),
            En1998Mutation::RemoveRetainingWall(p) => write_json_bin(&mut out, &p.index),
            En1998Mutation::RemoveTower(p) => write_json_bin(&mut out, &p.index),
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
            TAG_CHANGE_SYSTEM_V_RD_N => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let system_index = read_json_bin(&mut reader).map_err(|e| malformed("system_index", reader.position(), e))?;
                let new_base_shear_resistance_n = read_json_bin(&mut reader).map_err(|e| malformed("new_base_shear_resistance_n", reader.position(), e))?;
                Ok(En1998Mutation::ChangeSystemVRdN(ChangeSystemVRdN { building_index, system_index, new_base_shear_resistance_n }))
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
            TAG_CHANGE_ELEVATION_REGULAR => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let new_elevation_regular = read_json_bin(&mut reader).map_err(|e| malformed("new_elevation_regular", reader.position(), e))?;
                Ok(En1998Mutation::ChangeElevationRegular(ChangeElevationRegular { building_index, new_elevation_regular }))
            }
            TAG_CHANGE_MEMBER_DETAILING => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let member_index = read_json_bin(&mut reader).map_err(|e| malformed("member_index", reader.position(), e))?;
                let new_detailing_compatible_with_q = read_json_bin(&mut reader).map_err(|e| malformed("new_detailing_compatible_with_q", reader.position(), e))?;
                Ok(En1998Mutation::ChangeMemberDetailing(ChangeMemberDetailing { building_index, member_index, new_detailing_compatible_with_q }))
            }
            TAG_CHANGE_MASONRY_WALL_RATIO => {
                let building_index = read_json_bin(&mut reader).map_err(|e| malformed("building_index", reader.position(), e))?;
                let new_masonry_wall_area_ratio = read_json_bin(&mut reader).map_err(|e| malformed("new_masonry_wall_area_ratio", reader.position(), e))?;
                Ok(En1998Mutation::ChangeMasonryWallRatio(ChangeMasonryWallRatio { building_index, new_masonry_wall_area_ratio }))
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
            TAG_REMOVE_BRIDGE => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveBridge(RemoveBridge { index }))
            }
            TAG_REMOVE_ASSESSMENT => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveAssessment(RemoveAssessment { index }))
            }
            TAG_REMOVE_SILO => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveSilo(RemoveSilo { index }))
            }
            TAG_REMOVE_TANK => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveTank(RemoveTank { index }))
            }
            TAG_REMOVE_FOUNDATION => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveFoundation(RemoveFoundation { index }))
            }
            TAG_REMOVE_RETAINING_WALL => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveRetainingWall(RemoveRetainingWall { index }))
            }
            TAG_REMOVE_TOWER => {
                let index = read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?;
                Ok(En1998Mutation::RemoveTower(RemoveTower { index }))
            }
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
}
