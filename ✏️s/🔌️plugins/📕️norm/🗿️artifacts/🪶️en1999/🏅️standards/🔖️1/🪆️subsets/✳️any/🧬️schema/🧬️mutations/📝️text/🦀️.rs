//! 🔧️ En1999 artifact — OpText/OpBinary codecs for `En1999Mutation`.

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

use crate::document::AnnexChoice;
pub use crate::artifact_schema::mutations::En1999Mutation;
use crate::artifact_schema::mutations::{
    change_annex, change_bolt_count, change_material_designation, change_member_buckling_length, change_member_m_y_ed, change_member_n_ed, change_plate_thickness,
    change_weld_throat, add_member, remove_member, change_cold_formed, change_connections, change_fatigue_details, change_fire_scenarios, change_materials, change_members,
    change_sections, change_shells,
};
use crate::snapshot::{AluminiumConnection, AluminiumMaterial, AluminiumMember, AluminiumSection, AluminiumShell, ColdFormedSheet, FatigueDetail, FireScenario};
use protocol::OpText;

#[derive(Clone, Debug, PartialEq, dsl::DslEnum)]
enum En1999MutationDsl {
    ChangeAnnex { new_annex: AnnexChoice },
    ChangeMaterials { payload_json: String },
    ChangeSections { payload_json: String },
    ChangeMembers { payload_json: String },
    ChangeConnections { payload_json: String },
    ChangeFireScenarios { payload_json: String },
    ChangeFatigueDetails { payload_json: String },
    ChangeColdFormed { payload_json: String },
    ChangeShells { payload_json: String },
    AddMember { index: u32, payload_json: String },
    RemoveMember { id: String },
    ChangeMemberNEd { member_id: String, action_id: String, new_n_k: f64 },
    ChangeMemberMYEd { member_id: String, action_id: String, new_m_y_k: f64 },
    ChangeMemberBucklingLength { member_id: String, axis: String, new_length: f64 },
    ChangeMaterialDesignation { material_id: String, new_designation: String },
    ChangePlateThickness { section_id: String, element_id: String, new_thickness: f64 },
    ChangeWeldThroat { connection_id: String, new_throat: f64 },
    ChangeBoltCount { connection_id: String, new_rows: u32, new_bolts_per_row: u32 },
}

impl OpText for En1999MutationDsl {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        let variants = <Self as dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown mutation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let variants = <Self as dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec");
        dsl::print(&record, &spec_fn(), dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for En1999MutationDsl {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_tagged_op(include_str!("../💾️binary/📡️.protocol.semio"), bytes)
    }
}

fn json_of<T: pack::value::ToValue>(value: &T) -> String {
    pack::json::to_json_string(value)
}

fn from_json<T: pack::value::FromValue>(text: &str) -> T {
    pack::json::from_json_str(text).expect("mutation payload json")
}

fn en1999_mutation_to_dsl(mutation: &En1999Mutation) -> En1999MutationDsl {
    match mutation {
        En1999Mutation::ChangeAnnex(p) => En1999MutationDsl::ChangeAnnex { new_annex: p.new_annex },
        En1999Mutation::ChangeMaterials(p) => En1999MutationDsl::ChangeMaterials { payload_json: json_of(&p.materials) },
        En1999Mutation::ChangeSections(p) => En1999MutationDsl::ChangeSections { payload_json: json_of(&p.sections) },
        En1999Mutation::ChangeMembers(p) => En1999MutationDsl::ChangeMembers { payload_json: json_of(&p.members) },
        En1999Mutation::ChangeConnections(p) => En1999MutationDsl::ChangeConnections { payload_json: json_of(&p.connections) },
        En1999Mutation::ChangeFireScenarios(p) => En1999MutationDsl::ChangeFireScenarios { payload_json: json_of(&p.fire_scenarios) },
        En1999Mutation::ChangeFatigueDetails(p) => En1999MutationDsl::ChangeFatigueDetails { payload_json: json_of(&p.fatigue_details) },
        En1999Mutation::ChangeColdFormed(p) => En1999MutationDsl::ChangeColdFormed { payload_json: json_of(&p.cold_formed) },
        En1999Mutation::ChangeShells(p) => En1999MutationDsl::ChangeShells { payload_json: json_of(&p.shells) },
        En1999Mutation::AddMember(p) => En1999MutationDsl::AddMember { index: p.index, payload_json: json_of(&p.member) },
        En1999Mutation::RemoveMember(p) => En1999MutationDsl::RemoveMember { id: p.id.clone() },
        En1999Mutation::ChangeMemberNEd(p) => En1999MutationDsl::ChangeMemberNEd { member_id: p.member_id.clone(), action_id: p.action_id.clone(), new_n_k: p.new_n_k },
        En1999Mutation::ChangeMemberMYEd(p) => En1999MutationDsl::ChangeMemberMYEd { member_id: p.member_id.clone(), action_id: p.action_id.clone(), new_m_y_k: p.new_m_y_k },
        En1999Mutation::ChangeMemberBucklingLength(p) => En1999MutationDsl::ChangeMemberBucklingLength { member_id: p.member_id.clone(), axis: p.axis.clone(), new_length: p.new_length },
        En1999Mutation::ChangeMaterialDesignation(p) => En1999MutationDsl::ChangeMaterialDesignation { material_id: p.material_id.clone(), new_designation: p.new_designation.clone() },
        En1999Mutation::ChangePlateThickness(p) => En1999MutationDsl::ChangePlateThickness { section_id: p.section_id.clone(), element_id: p.element_id.clone(), new_thickness: p.new_thickness },
        En1999Mutation::ChangeWeldThroat(p) => En1999MutationDsl::ChangeWeldThroat { connection_id: p.connection_id.clone(), new_throat: p.new_throat },
        En1999Mutation::ChangeBoltCount(p) => En1999MutationDsl::ChangeBoltCount { connection_id: p.connection_id.clone(), new_rows: p.new_rows, new_bolts_per_row: p.new_bolts_per_row },
    }
}

fn en1999_mutation_from_dsl(mutation: En1999MutationDsl) -> En1999Mutation {
    match mutation {
        En1999MutationDsl::ChangeAnnex { new_annex } => En1999Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex }),
        En1999MutationDsl::ChangeMaterials { payload_json } => En1999Mutation::ChangeMaterials(change_materials::ChangeMaterials { materials: from_json::<Vec<AluminiumMaterial>>(&payload_json) }),
        En1999MutationDsl::ChangeSections { payload_json } => En1999Mutation::ChangeSections(change_sections::ChangeSections { sections: from_json::<Vec<AluminiumSection>>(&payload_json) }),
        En1999MutationDsl::ChangeMembers { payload_json } => En1999Mutation::ChangeMembers(change_members::ChangeMembers { members: from_json::<Vec<AluminiumMember>>(&payload_json) }),
        En1999MutationDsl::ChangeConnections { payload_json } => En1999Mutation::ChangeConnections(change_connections::ChangeConnections { connections: from_json::<Vec<AluminiumConnection>>(&payload_json) }),
        En1999MutationDsl::ChangeFireScenarios { payload_json } => En1999Mutation::ChangeFireScenarios(change_fire_scenarios::ChangeFireScenarios { fire_scenarios: from_json::<Vec<FireScenario>>(&payload_json) }),
        En1999MutationDsl::ChangeFatigueDetails { payload_json } => En1999Mutation::ChangeFatigueDetails(change_fatigue_details::ChangeFatigueDetails { fatigue_details: from_json::<Vec<FatigueDetail>>(&payload_json) }),
        En1999MutationDsl::ChangeColdFormed { payload_json } => En1999Mutation::ChangeColdFormed(change_cold_formed::ChangeColdFormed { cold_formed: from_json::<Vec<ColdFormedSheet>>(&payload_json) }),
        En1999MutationDsl::ChangeShells { payload_json } => En1999Mutation::ChangeShells(change_shells::ChangeShells { shells: from_json::<Vec<AluminiumShell>>(&payload_json) }),
        En1999MutationDsl::AddMember { index, payload_json } => En1999Mutation::AddMember(add_member::AddMember { index, member: from_json::<AluminiumMember>(&payload_json) }),
        En1999MutationDsl::RemoveMember { id } => En1999Mutation::RemoveMember(remove_member::RemoveMember { id }),
        En1999MutationDsl::ChangeMemberNEd { member_id, action_id, new_n_k } => En1999Mutation::ChangeMemberNEd(change_member_n_ed::ChangeMemberNEd { member_id, action_id, new_n_k }),
        En1999MutationDsl::ChangeMemberMYEd { member_id, action_id, new_m_y_k } => En1999Mutation::ChangeMemberMYEd(change_member_m_y_ed::ChangeMemberMYEd { member_id, action_id, new_m_y_k }),
        En1999MutationDsl::ChangeMemberBucklingLength { member_id, axis, new_length } => En1999Mutation::ChangeMemberBucklingLength(change_member_buckling_length::ChangeMemberBucklingLength { member_id, axis, new_length }),
        En1999MutationDsl::ChangeMaterialDesignation { material_id, new_designation } => En1999Mutation::ChangeMaterialDesignation(change_material_designation::ChangeMaterialDesignation { material_id, new_designation }),
        En1999MutationDsl::ChangePlateThickness { section_id, element_id, new_thickness } => En1999Mutation::ChangePlateThickness(change_plate_thickness::ChangePlateThickness { section_id, element_id, new_thickness }),
        En1999MutationDsl::ChangeWeldThroat { connection_id, new_throat } => En1999Mutation::ChangeWeldThroat(change_weld_throat::ChangeWeldThroat { connection_id, new_throat }),
        En1999MutationDsl::ChangeBoltCount { connection_id, new_rows, new_bolts_per_row } => En1999Mutation::ChangeBoltCount(change_bolt_count::ChangeBoltCount { connection_id, new_rows, new_bolts_per_row }),
    }
}

impl OpText for En1999Mutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        Ok(en1999_mutation_from_dsl(<En1999MutationDsl as OpText>::parse_op(line)?))
    }
    fn print_op(&self) -> String {
        <En1999MutationDsl as OpText>::print_op(&en1999_mutation_to_dsl(self))
    }
}

impl protocol::OpBinary for En1999Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        en1999_mutation_to_dsl(self).encode_op()
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        Ok(en1999_mutation_from_dsl(En1999MutationDsl::decode_op(bytes)?))
    }
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
