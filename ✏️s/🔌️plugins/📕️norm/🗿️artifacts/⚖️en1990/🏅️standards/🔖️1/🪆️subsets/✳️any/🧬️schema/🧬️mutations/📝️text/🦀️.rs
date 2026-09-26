//! ⚡️ EN 1990 — hand-rolled `OpText`/`OpBinary` for `En1990Mutation`.

pub use crate::artifact_schema::mutations::En1990Mutation;

use crate::artifact_schema::mutations::{
    change_accidentals, change_annex, change_beta_computed, change_consequence_class, change_design_working_life_category, change_design_working_life_years, change_effects,
    change_inspection_level, change_members, change_bridge_sls, change_permanents, change_project_id, change_altitude_m, change_reference_period_years, change_reliability_class, change_seismics, change_supervision_level,
    change_variables, insert_accidental, insert_effect, insert_member, insert_permanent, insert_seismic, insert_variable, remove_accidental, remove_effect, remove_member, remove_permanent,
    remove_seismic, remove_variable,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
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
fn enc_json<T: dsl::ToValue>(v: &T) -> String {
    enc_str(&pack::json::to_json_string(v))
}
fn dec_json<T: dsl::FromValue>(s: &str) -> Result<T, String> {
    let raw = dec_str(s)?;
    pack::json::from_json_str(&raw).map_err(|e| e.to_string())
}
fn tokenize_args(rest: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    let mut chars = rest.chars().peekable();
    while chars.peek().is_some() {
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }
        if chars.peek().is_none() {
            break;
        }
        let mut key = String::new();
        while let Some(c) = chars.peek().copied() {
            if c == '=' || c.is_whitespace() {
                break;
            }
            key.push(c);
            chars.next();
        }
        if key.is_empty() {
            return Err("empty key".into());
        }
        if chars.next() != Some('=') {
            return Err(format!("expected '=' after {key}"));
        }
        while matches!(chars.peek(), Some(c) if c.is_whitespace()) {
            chars.next();
        }
        let mut val = String::new();
        if chars.peek() == Some(&'"') {
            val.push(chars.next().unwrap());
            loop {
                match chars.next() {
                    Some('\\') => {
                        val.push('\\');
                        if let Some(n) = chars.next() {
                            val.push(n);
                        }
                    }
                    Some('"') => {
                        val.push('"');
                        break;
                    }
                    Some(c) => val.push(c),
                    None => return Err("unterminated string".into()),
                }
            }
        } else {
            while let Some(c) = chars.peek().copied() {
                if c.is_whitespace() {
                    break;
                }
                val.push(c);
                chars.next();
            }
        }
        out.push((key, val));
    }
    Ok(out)
}
//#endregion 🔖️ScalarCodec

//#region 🔖️OpTextCodec
fn print_en1990_mutation(mutation: &En1990Mutation) -> String {
    match mutation {
        En1990Mutation::ChangeAnnex(p) => format!("change-annex new-annex={}", enc_json(&p.new_annex)),
        En1990Mutation::ChangeProjectId(p) => format!("change-project-id new-project-id={}", enc_json(&p.new_project_id)),
        En1990Mutation::ChangeAltitudeM(p) => format!("change-altitude-m new-altitude-m={}", enc_json(&p.new_altitude_m)),
        En1990Mutation::ChangeConsequenceClass(p) => format!("change-consequence-class new-consequence-class={}", enc_json(&p.new_consequence_class)),
        En1990Mutation::ChangeReliabilityClass(p) => format!("change-reliability-class new-reliability-class={}", enc_json(&p.new_reliability_class)),
        En1990Mutation::ChangeDesignWorkingLifeCategory(p) => format!("change-design-working-life-category new-design-working-life-category={}", enc_json(&p.new_design_working_life_category)),
        En1990Mutation::ChangeDesignWorkingLifeYears(p) => format!("change-design-working-life-years new-design-working-life-years={}", enc_json(&p.new_design_working_life_years)),
        En1990Mutation::ChangeReferencePeriodYears(p) => format!("change-reference-period-years new-reference-period-years={}", enc_json(&p.new_reference_period_years)),
        En1990Mutation::ChangeSupervisionLevel(p) => format!("change-supervision-level new-supervision-level={}", enc_json(&p.new_supervision_level)),
        En1990Mutation::ChangeInspectionLevel(p) => format!("change-inspection-level new-inspection-level={}", enc_json(&p.new_inspection_level)),
        En1990Mutation::ChangeBetaComputed(p) => format!("change-beta-computed new-beta-computed={}", enc_json(&p.new_beta_computed)),
        En1990Mutation::ChangePermanents(p) => format!("change-permanents new-permanents={}", enc_json(&p.new_permanents)),
        En1990Mutation::ChangeVariables(p) => format!("change-variables new-variables={}", enc_json(&p.new_variables)),
        En1990Mutation::ChangeAccidentals(p) => format!("change-accidentals new-accidentals={}", enc_json(&p.new_accidentals)),
        En1990Mutation::ChangeSeismics(p) => format!("change-seismics new-seismics={}", enc_json(&p.new_seismics)),
        En1990Mutation::ChangeMembers(p) => format!("change-members new-members={}", enc_json(&p.new_members)),
        En1990Mutation::ChangeBridgeSls(p) => format!("change-bridge-sls new-bridge-sls={}", enc_json(&p.new_bridge_sls)),
        En1990Mutation::ChangeEffects(p) => format!("change-effects new-effects={}", enc_json(&p.new_effects)),
        En1990Mutation::InsertPermanent(p) => format!("insert-permanent index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemovePermanent(p) => format!("remove-permanent index={}", enc_json(&p.index)),
        En1990Mutation::InsertVariable(p) => format!("insert-variable index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveVariable(p) => format!("remove-variable index={}", enc_json(&p.index)),
        En1990Mutation::InsertAccidental(p) => format!("insert-accidental index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveAccidental(p) => format!("remove-accidental index={}", enc_json(&p.index)),
        En1990Mutation::InsertSeismic(p) => format!("insert-seismic index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveSeismic(p) => format!("remove-seismic index={}", enc_json(&p.index)),
        En1990Mutation::InsertMember(p) => format!("insert-member index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveMember(p) => format!("remove-member index={}", enc_json(&p.index)),
        En1990Mutation::InsertEffect(p) => format!("insert-effect index={} item={}", enc_json(&p.index), enc_json(&p.item)),
        En1990Mutation::RemoveEffect(p) => format!("remove-effect index={}", enc_json(&p.index)),
    }
}

fn parse_en1990_mutation(text: &str) -> Result<En1990Mutation, String> {
    let text = text.trim();
    let (keyword, rest) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
    let args = tokenize_args(rest)?;
    let arg = |k: &str| -> Result<&str, String> { args.iter().find(|(a, _)| a == k).map(|(_, v)| v.as_str()).ok_or_else(|| format!("missing {k}")) };
    match keyword {
        "change-annex" => Ok(En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: dec_json(arg("new-annex")?)? })),
        "change-project-id" => Ok(En1990Mutation::ChangeProjectId(change_project_id::ChangeProjectId { new_project_id: dec_json(arg("new-project-id")?)? })),
        "change-altitude-m" => Ok(En1990Mutation::ChangeAltitudeM(change_altitude_m::ChangeAltitudeM { new_altitude_m: dec_json(arg("new-altitude-m")?)? })),
        "change-consequence-class" => Ok(En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: dec_json(arg("new-consequence-class")?)? })),
        "change-reliability-class" => Ok(En1990Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: dec_json(arg("new-reliability-class")?)? })),
        "change-design-working-life-category" => Ok(En1990Mutation::ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory { new_design_working_life_category: dec_json(arg("new-design-working-life-category")?)? })),
        "change-design-working-life-years" => Ok(En1990Mutation::ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears { new_design_working_life_years: dec_json(arg("new-design-working-life-years")?)? })),
        "change-reference-period-years" => Ok(En1990Mutation::ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears { new_reference_period_years: dec_json(arg("new-reference-period-years")?)? })),
        "change-supervision-level" => Ok(En1990Mutation::ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel { new_supervision_level: dec_json(arg("new-supervision-level")?)? })),
        "change-inspection-level" => Ok(En1990Mutation::ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel { new_inspection_level: dec_json(arg("new-inspection-level")?)? })),
        "change-beta-computed" => Ok(En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: dec_json(arg("new-beta-computed")?)? })),
        "change-permanents" => Ok(En1990Mutation::ChangePermanents(change_permanents::ChangePermanents { new_permanents: dec_json(arg("new-permanents")?)? })),
        "change-variables" => Ok(En1990Mutation::ChangeVariables(change_variables::ChangeVariables { new_variables: dec_json(arg("new-variables")?)? })),
        "change-accidentals" => Ok(En1990Mutation::ChangeAccidentals(change_accidentals::ChangeAccidentals { new_accidentals: dec_json(arg("new-accidentals")?)? })),
        "change-seismics" => Ok(En1990Mutation::ChangeSeismics(change_seismics::ChangeSeismics { new_seismics: dec_json(arg("new-seismics")?)? })),
        "change-members" => Ok(En1990Mutation::ChangeMembers(change_members::ChangeMembers { new_members: dec_json(arg("new-members")?)? })),
        "change-bridge-sls" => Ok(En1990Mutation::ChangeBridgeSls(change_bridge_sls::ChangeBridgeSls { new_bridge_sls: dec_json(arg("new-bridge-sls")?)? })),
        "change-effects" => Ok(En1990Mutation::ChangeEffects(change_effects::ChangeEffects { new_effects: dec_json(arg("new-effects")?)? })),
        "insert-permanent" => Ok(En1990Mutation::InsertPermanent(insert_permanent::InsertPermanent { index: dec_json(arg("index")?)?, item: dec_json(arg("item")?)? })),
        "remove-permanent" => Ok(En1990Mutation::RemovePermanent(remove_permanent::RemovePermanent { index: dec_json(arg("index")?)? })),
        "insert-variable" => Ok(En1990Mutation::InsertVariable(insert_variable::InsertVariable { index: dec_json(arg("index")?)?, item: dec_json(arg("item")?)? })),
        "remove-variable" => Ok(En1990Mutation::RemoveVariable(remove_variable::RemoveVariable { index: dec_json(arg("index")?)? })),
        "insert-accidental" => Ok(En1990Mutation::InsertAccidental(insert_accidental::InsertAccidental { index: dec_json(arg("index")?)?, item: dec_json(arg("item")?)? })),
        "remove-accidental" => Ok(En1990Mutation::RemoveAccidental(remove_accidental::RemoveAccidental { index: dec_json(arg("index")?)? })),
        "insert-seismic" => Ok(En1990Mutation::InsertSeismic(insert_seismic::InsertSeismic { index: dec_json(arg("index")?)?, item: dec_json(arg("item")?)? })),
        "remove-seismic" => Ok(En1990Mutation::RemoveSeismic(remove_seismic::RemoveSeismic { index: dec_json(arg("index")?)? })),
        "insert-member" => Ok(En1990Mutation::InsertMember(insert_member::InsertMember { index: dec_json(arg("index")?)?, item: dec_json(arg("item")?)? })),
        "remove-member" => Ok(En1990Mutation::RemoveMember(remove_member::RemoveMember { index: dec_json(arg("index")?)? })),
        "insert-effect" => Ok(En1990Mutation::InsertEffect(insert_effect::InsertEffect { index: dec_json(arg("index")?)?, item: dec_json(arg("item")?)? })),
        "remove-effect" => Ok(En1990Mutation::RemoveEffect(remove_effect::RemoveEffect { index: dec_json(arg("index")?)? })),
        other => Err(format!("unknown keyword {other}")),
    }
}

impl protocol::OpText for En1990Mutation {
    fn print_op(&self) -> String {
        print_en1990_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        parse_en1990_mutation(line).map_err(|e| store::TextError::new(e, store::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpTextCodec

//#region 🔖️OpBinaryCodec
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

const TAG_CHANGE_ANNEX: u8 = 0;
const TAG_CHANGE_PROJECT_ID: u8 = 1;
const TAG_CHANGE_CONSEQUENCE_CLASS: u8 = 2;
const TAG_CHANGE_RELIABILITY_CLASS: u8 = 3;
const TAG_CHANGE_DWL_CATEGORY: u8 = 4;
const TAG_CHANGE_DWL_YEARS: u8 = 5;
const TAG_CHANGE_REFERENCE_PERIOD: u8 = 6;
const TAG_CHANGE_SUPERVISION: u8 = 7;
const TAG_CHANGE_INSPECTION: u8 = 8;
const TAG_CHANGE_BETA: u8 = 9;
const TAG_CHANGE_PERMANENTS: u8 = 10;
const TAG_CHANGE_VARIABLES: u8 = 11;
const TAG_CHANGE_ACCIDENTALS: u8 = 12;
const TAG_CHANGE_SEISMICS: u8 = 13;
const TAG_CHANGE_MEMBERS: u8 = 14;
const TAG_CHANGE_EFFECTS: u8 = 15;
const TAG_CHANGE_ALTITUDE: u8 = 28;
const TAG_CHANGE_BRIDGE_SLS: u8 = 29;
const TAG_INSERT_PERMANENT: u8 = 16;
const TAG_REMOVE_PERMANENT: u8 = 17;
const TAG_INSERT_VARIABLE: u8 = 18;
const TAG_REMOVE_VARIABLE: u8 = 19;
const TAG_INSERT_ACCIDENTAL: u8 = 20;
const TAG_REMOVE_ACCIDENTAL: u8 = 21;
const TAG_INSERT_SEISMIC: u8 = 22;
const TAG_REMOVE_SEISMIC: u8 = 23;
const TAG_INSERT_MEMBER: u8 = 24;
const TAG_REMOVE_MEMBER: u8 = 25;
const TAG_INSERT_EFFECT: u8 = 26;
const TAG_REMOVE_EFFECT: u8 = 27;

impl protocol::OpBinary for En1990Mutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let tag: u8 = match self {
            En1990Mutation::ChangeAnnex(_) => TAG_CHANGE_ANNEX,
            En1990Mutation::ChangeProjectId(_) => TAG_CHANGE_PROJECT_ID,
            En1990Mutation::ChangeAltitudeM(_) => TAG_CHANGE_ALTITUDE,
            En1990Mutation::ChangeConsequenceClass(_) => TAG_CHANGE_CONSEQUENCE_CLASS,
            En1990Mutation::ChangeReliabilityClass(_) => TAG_CHANGE_RELIABILITY_CLASS,
            En1990Mutation::ChangeDesignWorkingLifeCategory(_) => TAG_CHANGE_DWL_CATEGORY,
            En1990Mutation::ChangeDesignWorkingLifeYears(_) => TAG_CHANGE_DWL_YEARS,
            En1990Mutation::ChangeReferencePeriodYears(_) => TAG_CHANGE_REFERENCE_PERIOD,
            En1990Mutation::ChangeSupervisionLevel(_) => TAG_CHANGE_SUPERVISION,
            En1990Mutation::ChangeInspectionLevel(_) => TAG_CHANGE_INSPECTION,
            En1990Mutation::ChangeBetaComputed(_) => TAG_CHANGE_BETA,
            En1990Mutation::ChangePermanents(_) => TAG_CHANGE_PERMANENTS,
            En1990Mutation::ChangeVariables(_) => TAG_CHANGE_VARIABLES,
            En1990Mutation::ChangeAccidentals(_) => TAG_CHANGE_ACCIDENTALS,
            En1990Mutation::ChangeSeismics(_) => TAG_CHANGE_SEISMICS,
            En1990Mutation::ChangeMembers(_) => TAG_CHANGE_MEMBERS,
            En1990Mutation::ChangeBridgeSls(_) => TAG_CHANGE_BRIDGE_SLS,
            En1990Mutation::ChangeEffects(_) => TAG_CHANGE_EFFECTS,
            En1990Mutation::InsertPermanent(_) => TAG_INSERT_PERMANENT,
            En1990Mutation::RemovePermanent(_) => TAG_REMOVE_PERMANENT,
            En1990Mutation::InsertVariable(_) => TAG_INSERT_VARIABLE,
            En1990Mutation::RemoveVariable(_) => TAG_REMOVE_VARIABLE,
            En1990Mutation::InsertAccidental(_) => TAG_INSERT_ACCIDENTAL,
            En1990Mutation::RemoveAccidental(_) => TAG_REMOVE_ACCIDENTAL,
            En1990Mutation::InsertSeismic(_) => TAG_INSERT_SEISMIC,
            En1990Mutation::RemoveSeismic(_) => TAG_REMOVE_SEISMIC,
            En1990Mutation::InsertMember(_) => TAG_INSERT_MEMBER,
            En1990Mutation::RemoveMember(_) => TAG_REMOVE_MEMBER,
            En1990Mutation::InsertEffect(_) => TAG_INSERT_EFFECT,
            En1990Mutation::RemoveEffect(_) => TAG_REMOVE_EFFECT,
        };
        let mut out = vec![store::pack_rt::OP_BINARY_FORMAT, tag];
        match self {
            En1990Mutation::ChangeAnnex(p) => write_json_bin(&mut out, &p.new_annex),
            En1990Mutation::ChangeProjectId(p) => write_json_bin(&mut out, &p.new_project_id),
            En1990Mutation::ChangeAltitudeM(p) => write_json_bin(&mut out, &p.new_altitude_m),
            En1990Mutation::ChangeConsequenceClass(p) => write_json_bin(&mut out, &p.new_consequence_class),
            En1990Mutation::ChangeReliabilityClass(p) => write_json_bin(&mut out, &p.new_reliability_class),
            En1990Mutation::ChangeDesignWorkingLifeCategory(p) => write_json_bin(&mut out, &p.new_design_working_life_category),
            En1990Mutation::ChangeDesignWorkingLifeYears(p) => write_json_bin(&mut out, &p.new_design_working_life_years),
            En1990Mutation::ChangeReferencePeriodYears(p) => write_json_bin(&mut out, &p.new_reference_period_years),
            En1990Mutation::ChangeSupervisionLevel(p) => write_json_bin(&mut out, &p.new_supervision_level),
            En1990Mutation::ChangeInspectionLevel(p) => write_json_bin(&mut out, &p.new_inspection_level),
            En1990Mutation::ChangeBetaComputed(p) => write_json_bin(&mut out, &p.new_beta_computed),
            En1990Mutation::ChangePermanents(p) => write_json_bin(&mut out, &p.new_permanents),
            En1990Mutation::ChangeVariables(p) => write_json_bin(&mut out, &p.new_variables),
            En1990Mutation::ChangeAccidentals(p) => write_json_bin(&mut out, &p.new_accidentals),
            En1990Mutation::ChangeSeismics(p) => write_json_bin(&mut out, &p.new_seismics),
            En1990Mutation::ChangeMembers(p) => write_json_bin(&mut out, &p.new_members),
            En1990Mutation::ChangeBridgeSls(p) => write_json_bin(&mut out, &p.new_bridge_sls),
            En1990Mutation::ChangeEffects(p) => write_json_bin(&mut out, &p.new_effects),
            En1990Mutation::InsertPermanent(p) => { write_json_bin(&mut out, &p.index); write_json_bin(&mut out, &p.item); }
            En1990Mutation::RemovePermanent(p) => write_json_bin(&mut out, &p.index),
            En1990Mutation::InsertVariable(p) => { write_json_bin(&mut out, &p.index); write_json_bin(&mut out, &p.item); }
            En1990Mutation::RemoveVariable(p) => write_json_bin(&mut out, &p.index),
            En1990Mutation::InsertAccidental(p) => { write_json_bin(&mut out, &p.index); write_json_bin(&mut out, &p.item); }
            En1990Mutation::RemoveAccidental(p) => write_json_bin(&mut out, &p.index),
            En1990Mutation::InsertSeismic(p) => { write_json_bin(&mut out, &p.index); write_json_bin(&mut out, &p.item); }
            En1990Mutation::RemoveSeismic(p) => write_json_bin(&mut out, &p.index),
            En1990Mutation::InsertMember(p) => { write_json_bin(&mut out, &p.index); write_json_bin(&mut out, &p.item); }
            En1990Mutation::RemoveMember(p) => write_json_bin(&mut out, &p.index),
            En1990Mutation::InsertEffect(p) => { write_json_bin(&mut out, &p.index); write_json_bin(&mut out, &p.item); }
            En1990Mutation::RemoveEffect(p) => write_json_bin(&mut out, &p.index),
        }
        Ok(out)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        let mut reader = store::ByteReader::new(bytes);
        let malformed = |what: &'static str, offset: usize, detail: String| protocol::ProtocolError::Malformed { what, offset: offset as u64, detail };
        let _format = reader.read_u8().map_err(|e| malformed("op format", 0, e.to_string()))?;
        let tag = reader.read_u8().map_err(|e| malformed("op tag", 1, e.to_string()))?;
        match tag {
            TAG_CHANGE_ANNEX => Ok(En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: read_json_bin(&mut reader).map_err(|e| malformed("new_annex", reader.position(), e))? })),
            TAG_CHANGE_PROJECT_ID => Ok(En1990Mutation::ChangeProjectId(change_project_id::ChangeProjectId { new_project_id: read_json_bin(&mut reader).map_err(|e| malformed("new_project_id", reader.position(), e))? })),
            TAG_CHANGE_ALTITUDE => Ok(En1990Mutation::ChangeAltitudeM(change_altitude_m::ChangeAltitudeM { new_altitude_m: read_json_bin(&mut reader).map_err(|e| malformed("altitude_m", reader.position(), e))? })),
            TAG_CHANGE_CONSEQUENCE_CLASS => Ok(En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: read_json_bin(&mut reader).map_err(|e| malformed("cc", reader.position(), e))? })),
            TAG_CHANGE_RELIABILITY_CLASS => Ok(En1990Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: read_json_bin(&mut reader).map_err(|e| malformed("rc", reader.position(), e))? })),
            TAG_CHANGE_DWL_CATEGORY => Ok(En1990Mutation::ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory { new_design_working_life_category: read_json_bin(&mut reader).map_err(|e| malformed("dwl_cat", reader.position(), e))? })),
            TAG_CHANGE_DWL_YEARS => Ok(En1990Mutation::ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears { new_design_working_life_years: read_json_bin(&mut reader).map_err(|e| malformed("dwl_y", reader.position(), e))? })),
            TAG_CHANGE_REFERENCE_PERIOD => Ok(En1990Mutation::ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears { new_reference_period_years: read_json_bin(&mut reader).map_err(|e| malformed("ref", reader.position(), e))? })),
            TAG_CHANGE_SUPERVISION => Ok(En1990Mutation::ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel { new_supervision_level: read_json_bin(&mut reader).map_err(|e| malformed("sup", reader.position(), e))? })),
            TAG_CHANGE_INSPECTION => Ok(En1990Mutation::ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel { new_inspection_level: read_json_bin(&mut reader).map_err(|e| malformed("insp", reader.position(), e))? })),
            TAG_CHANGE_BETA => Ok(En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: read_json_bin(&mut reader).map_err(|e| malformed("beta", reader.position(), e))? })),
            TAG_CHANGE_PERMANENTS => Ok(En1990Mutation::ChangePermanents(change_permanents::ChangePermanents { new_permanents: read_json_bin(&mut reader).map_err(|e| malformed("permanents", reader.position(), e))? })),
            TAG_CHANGE_VARIABLES => Ok(En1990Mutation::ChangeVariables(change_variables::ChangeVariables { new_variables: read_json_bin(&mut reader).map_err(|e| malformed("variables", reader.position(), e))? })),
            TAG_CHANGE_ACCIDENTALS => Ok(En1990Mutation::ChangeAccidentals(change_accidentals::ChangeAccidentals { new_accidentals: read_json_bin(&mut reader).map_err(|e| malformed("accidentals", reader.position(), e))? })),
            TAG_CHANGE_SEISMICS => Ok(En1990Mutation::ChangeSeismics(change_seismics::ChangeSeismics { new_seismics: read_json_bin(&mut reader).map_err(|e| malformed("seismics", reader.position(), e))? })),
            TAG_CHANGE_MEMBERS => Ok(En1990Mutation::ChangeMembers(change_members::ChangeMembers { new_members: read_json_bin(&mut reader).map_err(|e| malformed("members", reader.position(), e))? })),
            TAG_CHANGE_BRIDGE_SLS => Ok(En1990Mutation::ChangeBridgeSls(change_bridge_sls::ChangeBridgeSls { new_bridge_sls: read_json_bin(&mut reader).map_err(|e| malformed("bridge_sls", reader.position(), e))? })),
            TAG_CHANGE_EFFECTS => Ok(En1990Mutation::ChangeEffects(change_effects::ChangeEffects { new_effects: read_json_bin(&mut reader).map_err(|e| malformed("effects", reader.position(), e))? })),
            TAG_INSERT_PERMANENT => Ok(En1990Mutation::InsertPermanent(insert_permanent::InsertPermanent { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?, item: read_json_bin(&mut reader).map_err(|e| malformed("item", reader.position(), e))? })),
            TAG_REMOVE_PERMANENT => Ok(En1990Mutation::RemovePermanent(remove_permanent::RemovePermanent { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))? })),
            TAG_INSERT_VARIABLE => Ok(En1990Mutation::InsertVariable(insert_variable::InsertVariable { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?, item: read_json_bin(&mut reader).map_err(|e| malformed("item", reader.position(), e))? })),
            TAG_REMOVE_VARIABLE => Ok(En1990Mutation::RemoveVariable(remove_variable::RemoveVariable { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))? })),
            TAG_INSERT_ACCIDENTAL => Ok(En1990Mutation::InsertAccidental(insert_accidental::InsertAccidental { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?, item: read_json_bin(&mut reader).map_err(|e| malformed("item", reader.position(), e))? })),
            TAG_REMOVE_ACCIDENTAL => Ok(En1990Mutation::RemoveAccidental(remove_accidental::RemoveAccidental { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))? })),
            TAG_INSERT_SEISMIC => Ok(En1990Mutation::InsertSeismic(insert_seismic::InsertSeismic { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?, item: read_json_bin(&mut reader).map_err(|e| malformed("item", reader.position(), e))? })),
            TAG_REMOVE_SEISMIC => Ok(En1990Mutation::RemoveSeismic(remove_seismic::RemoveSeismic { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))? })),
            TAG_INSERT_MEMBER => Ok(En1990Mutation::InsertMember(insert_member::InsertMember { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?, item: read_json_bin(&mut reader).map_err(|e| malformed("item", reader.position(), e))? })),
            TAG_REMOVE_MEMBER => Ok(En1990Mutation::RemoveMember(remove_member::RemoveMember { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))? })),
            TAG_INSERT_EFFECT => Ok(En1990Mutation::InsertEffect(insert_effect::InsertEffect { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))?, item: read_json_bin(&mut reader).map_err(|e| malformed("item", reader.position(), e))? })),
            TAG_REMOVE_EFFECT => Ok(En1990Mutation::RemoveEffect(remove_effect::RemoveEffect { index: read_json_bin(&mut reader).map_err(|e| malformed("index", reader.position(), e))? })),
            other => Err(malformed("op tag", 1, format!("unknown tag {other}"))),
        }
    }
}
//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<En1990Mutation> {
    let base = crate::En1990Snapshot::default();
    vec![
        En1990Mutation::ChangeAnnex(change_annex::ChangeAnnex { new_annex: crate::document::AnnexChoice::En }),
        En1990Mutation::ChangeProjectId(change_project_id::ChangeProjectId { new_project_id: "x".into() }),
        En1990Mutation::ChangeAltitudeM(change_altitude_m::ChangeAltitudeM { new_altitude_m: 1200.0 }),
        En1990Mutation::ChangeConsequenceClass(change_consequence_class::ChangeConsequenceClass { new_consequence_class: 3 }),
        En1990Mutation::ChangeReliabilityClass(change_reliability_class::ChangeReliabilityClass { new_reliability_class: 3 }),
        En1990Mutation::ChangeDesignWorkingLifeCategory(change_design_working_life_category::ChangeDesignWorkingLifeCategory { new_design_working_life_category: 5 }),
        En1990Mutation::ChangeDesignWorkingLifeYears(change_design_working_life_years::ChangeDesignWorkingLifeYears { new_design_working_life_years: 100.0 }),
        En1990Mutation::ChangeReferencePeriodYears(change_reference_period_years::ChangeReferencePeriodYears { new_reference_period_years: 100.0 }),
        En1990Mutation::ChangeSupervisionLevel(change_supervision_level::ChangeSupervisionLevel { new_supervision_level: "DSL".into() }),
        En1990Mutation::ChangeInspectionLevel(change_inspection_level::ChangeInspectionLevel { new_inspection_level: "IL3".into() }),
        En1990Mutation::ChangeBetaComputed(change_beta_computed::ChangeBetaComputed { new_beta_computed: 4.3 }),
        En1990Mutation::ChangePermanents(change_permanents::ChangePermanents { new_permanents: base.permanents.clone() }),
        En1990Mutation::ChangeVariables(change_variables::ChangeVariables { new_variables: base.variables.clone() }),
        En1990Mutation::ChangeAccidentals(change_accidentals::ChangeAccidentals { new_accidentals: vec![] }),
        En1990Mutation::ChangeSeismics(change_seismics::ChangeSeismics { new_seismics: vec![] }),
        En1990Mutation::ChangeMembers(change_members::ChangeMembers { new_members: base.members.clone() }),
        En1990Mutation::ChangeBridgeSls(change_bridge_sls::ChangeBridgeSls { new_bridge_sls: base.bridge_sls.clone() }),
        En1990Mutation::ChangeEffects(change_effects::ChangeEffects { new_effects: base.effects.clone() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
