//! OpBinary for En1993Mutation — tag + full JSON body.
pub use crate::artifact_schema::mutations::En1993Mutation;
pub const COMPONENT_PROTOCOL_SEMIO: &str = include_str!("📡️.protocol.semio");
pub const COMPONENT_PROTOCOL_PATH: &str = concat!(module_path!(), "::📡️.protocol.semio");
fn write_str_bin(out: &mut Vec<u8>, s: &str) { store::pack_rt::write_varint_u64(out, s.len() as u64); out.extend_from_slice(s.as_bytes()); }
fn read_str_bin(reader: &mut store::ByteReader<'_>) -> Result<String, String> {
    let len = reader.read_varint_u64().map_err(|e| e.to_string())? as usize;
    let bytes = reader.read_bytes(len).map_err(|e| e.to_string())?;
    String::from_utf8(bytes.to_vec()).map_err(|e| e.to_string())
}
const WIRE_PROTOCOL: &str = COMPONENT_PROTOCOL_SEMIO;
const TAG_CHANGE_ANNEX: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "change-annex");
const TAG_UPDATE_MEMBER_PROPERTIES: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-member-properties");
const TAG_UPDATE_FIRE_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-fire-inputs");
const TAG_UPDATE_COLD_FORMED_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-cold-formed-inputs");
const TAG_UPDATE_STAINLESS_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-stainless-inputs");
const TAG_UPDATE_PLATED_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-plated-inputs");
const TAG_UPDATE_SILO_SHELL_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-silo-shell-inputs");
const TAG_UPDATE_BOLT_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-bolt-inputs");
const TAG_UPDATE_WELD_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-weld-inputs");
const TAG_UPDATE_FATIGUE_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-fatigue-inputs");
const TAG_UPDATE_THROUGH_THICKNESS_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-through-thickness-inputs");
const TAG_UPDATE_TENSION_COMPONENT_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-tension-component-inputs");
const TAG_UPDATE_HSS_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-hss-inputs");
const TAG_UPDATE_BRIDGE_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-bridge-inputs");
const TAG_UPDATE_TOWER_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-tower-inputs");
const TAG_UPDATE_PILE_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-pile-inputs");
const TAG_UPDATE_CRANE_INPUTS: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "update-crane-inputs");
const TAG_INSERT_MATERIAL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-material");
const TAG_REMOVE_MATERIAL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-material");
const TAG_INSERT_SECTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-section");
const TAG_REMOVE_SECTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-section");
const TAG_INSERT_MEMBER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-member");
const TAG_REMOVE_MEMBER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-member");
const TAG_INSERT_LOAD_CASE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-load-case");
const TAG_REMOVE_LOAD_CASE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-load-case");
const TAG_INSERT_MEMBER_ACTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-member-action");
const TAG_REMOVE_MEMBER_ACTION: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-member-action");
const TAG_INSERT_JOINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-joint");
const TAG_REMOVE_JOINT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-joint");
const TAG_INSERT_FATIGUE_DETAIL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-fatigue-detail");
const TAG_REMOVE_FATIGUE_DETAIL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-fatigue-detail");
const TAG_INSERT_FIRE_EXPOSURE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-fire-exposure");
const TAG_REMOVE_FIRE_EXPOSURE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-fire-exposure");
const TAG_INSERT_COLD_FORMED_MEMBER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-cold-formed-member");
const TAG_REMOVE_COLD_FORMED_MEMBER: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-cold-formed-member");
const TAG_INSERT_PLATED_PANEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-plated-panel");
const TAG_REMOVE_PLATED_PANEL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-plated-panel");
const TAG_INSERT_SILO_SHELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-silo-shell");
const TAG_REMOVE_SILO_SHELL: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-silo-shell");
const TAG_INSERT_TENSION_COMPONENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-tension-component");
const TAG_REMOVE_TENSION_COMPONENT: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-tension-component");
const TAG_INSERT_BRIDGE_FATIGUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-bridge-fatigue");
const TAG_REMOVE_BRIDGE_FATIGUE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-bridge-fatigue");
const TAG_INSERT_TOWER_LEG: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-tower-leg");
const TAG_REMOVE_TOWER_LEG: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-tower-leg");
const TAG_INSERT_PILE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-pile");
const TAG_REMOVE_PILE: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-pile");
const TAG_INSERT_CRANE_RUNWAY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "insert-crane-runway");
const TAG_REMOVE_CRANE_RUNWAY: u8 = dsl::protocol_record::tag_u8(WIRE_PROTOCOL, "remove-crane-runway");
impl protocol::OpBinary for En1993Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1993Mutation::ChangeAnnex(_) => TAG_CHANGE_ANNEX,
            En1993Mutation::UpdateMemberProperties(_) => TAG_UPDATE_MEMBER_PROPERTIES,
            En1993Mutation::UpdateFireInputs(_) => TAG_UPDATE_FIRE_INPUTS,
            En1993Mutation::UpdateColdFormedInputs(_) => TAG_UPDATE_COLD_FORMED_INPUTS,
            En1993Mutation::UpdateStainlessInputs(_) => TAG_UPDATE_STAINLESS_INPUTS,
            En1993Mutation::UpdatePlatedInputs(_) => TAG_UPDATE_PLATED_INPUTS,
            En1993Mutation::UpdateSiloShellInputs(_) => TAG_UPDATE_SILO_SHELL_INPUTS,
            En1993Mutation::UpdateBoltInputs(_) => TAG_UPDATE_BOLT_INPUTS,
            En1993Mutation::UpdateWeldInputs(_) => TAG_UPDATE_WELD_INPUTS,
            En1993Mutation::UpdateFatigueInputs(_) => TAG_UPDATE_FATIGUE_INPUTS,
            En1993Mutation::UpdateThroughThicknessInputs(_) => TAG_UPDATE_THROUGH_THICKNESS_INPUTS,
            En1993Mutation::UpdateTensionComponentInputs(_) => TAG_UPDATE_TENSION_COMPONENT_INPUTS,
            En1993Mutation::UpdateHssInputs(_) => TAG_UPDATE_HSS_INPUTS,
            En1993Mutation::UpdateBridgeInputs(_) => TAG_UPDATE_BRIDGE_INPUTS,
            En1993Mutation::UpdateTowerInputs(_) => TAG_UPDATE_TOWER_INPUTS,
            En1993Mutation::UpdatePileInputs(_) => TAG_UPDATE_PILE_INPUTS,
            En1993Mutation::UpdateCraneInputs(_) => TAG_UPDATE_CRANE_INPUTS,
            En1993Mutation::InsertMaterial(_) => TAG_INSERT_MATERIAL,
            En1993Mutation::RemoveMaterial(_) => TAG_REMOVE_MATERIAL,
            En1993Mutation::InsertSection(_) => TAG_INSERT_SECTION,
            En1993Mutation::RemoveSection(_) => TAG_REMOVE_SECTION,
            En1993Mutation::InsertMember(_) => TAG_INSERT_MEMBER,
            En1993Mutation::RemoveMember(_) => TAG_REMOVE_MEMBER,
            En1993Mutation::InsertLoadCase(_) => TAG_INSERT_LOAD_CASE,
            En1993Mutation::RemoveLoadCase(_) => TAG_REMOVE_LOAD_CASE,
            En1993Mutation::InsertMemberAction(_) => TAG_INSERT_MEMBER_ACTION,
            En1993Mutation::RemoveMemberAction(_) => TAG_REMOVE_MEMBER_ACTION,
            En1993Mutation::InsertJoint(_) => TAG_INSERT_JOINT,
            En1993Mutation::RemoveJoint(_) => TAG_REMOVE_JOINT,
            En1993Mutation::InsertFatigueDetail(_) => TAG_INSERT_FATIGUE_DETAIL,
            En1993Mutation::RemoveFatigueDetail(_) => TAG_REMOVE_FATIGUE_DETAIL,
            En1993Mutation::InsertFireExposure(_) => TAG_INSERT_FIRE_EXPOSURE,
            En1993Mutation::RemoveFireExposure(_) => TAG_REMOVE_FIRE_EXPOSURE,
            En1993Mutation::InsertColdFormedMember(_) => TAG_INSERT_COLD_FORMED_MEMBER,
            En1993Mutation::RemoveColdFormedMember(_) => TAG_REMOVE_COLD_FORMED_MEMBER,
            En1993Mutation::InsertPlatedPanel(_) => TAG_INSERT_PLATED_PANEL,
            En1993Mutation::RemovePlatedPanel(_) => TAG_REMOVE_PLATED_PANEL,
            En1993Mutation::InsertSiloShell(_) => TAG_INSERT_SILO_SHELL,
            En1993Mutation::RemoveSiloShell(_) => TAG_REMOVE_SILO_SHELL,
            En1993Mutation::InsertTensionComponent(_) => TAG_INSERT_TENSION_COMPONENT,
            En1993Mutation::RemoveTensionComponent(_) => TAG_REMOVE_TENSION_COMPONENT,
            En1993Mutation::InsertBridgeFatigue(_) => TAG_INSERT_BRIDGE_FATIGUE,
            En1993Mutation::RemoveBridgeFatigue(_) => TAG_REMOVE_BRIDGE_FATIGUE,
            En1993Mutation::InsertTowerLeg(_) => TAG_INSERT_TOWER_LEG,
            En1993Mutation::RemoveTowerLeg(_) => TAG_REMOVE_TOWER_LEG,
            En1993Mutation::InsertPile(_) => TAG_INSERT_PILE,
            En1993Mutation::RemovePile(_) => TAG_REMOVE_PILE,
            En1993Mutation::InsertCraneRunway(_) => TAG_INSERT_CRANE_RUNWAY,
            En1993Mutation::RemoveCraneRunway(_) => TAG_REMOVE_CRANE_RUNWAY,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        write_str_bin(&mut out, &pack::json::to_json_string(self));
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|e| protocol::ProtocolError::Io(e.to_string()))?;
        if format != store::pack_rt::OP_BINARY_FORMAT { return Err(protocol::ProtocolError::Io(format!("bad op format {format}"))); }
        let _tag = reader.read_u8().map_err(|e| protocol::ProtocolError::Io(e.to_string()))?;
        let body = read_str_bin(&mut reader).map_err(|e| protocol::ProtocolError::Io(e))?;
        pack::json::from_json_str(&body).map_err(|e| protocol::ProtocolError::Io(e.to_string()))
    }
}
