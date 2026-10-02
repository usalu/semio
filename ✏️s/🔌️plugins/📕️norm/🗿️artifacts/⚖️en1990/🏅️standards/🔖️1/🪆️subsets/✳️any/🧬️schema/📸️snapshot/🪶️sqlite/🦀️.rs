//! ⚖️ Handwritten EN1990 actions, verification members and influence relationships.
use super::En1990Snapshot;
use crate::{AccidentalAction, BridgeSls, ImportanceClass, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction, document::AnnexChoice};
use std::collections::BTreeMap;
use store::{ArtifactSqliteSnapshot, sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, validate_sqlite_database_schema, artifact::{Cell, FloatColumn, FloatRow, NativeEncodingBound, Projection, insert_ieee754, insert_key_ieee754, reconstruct_text}}};

const DOCUMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(12), FloatColumn::Binary64(13)];
const PERMANENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(2)];
const VARIABLE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(2)];
const ACCIDENTAL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(1)];
const SEISMIC_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(1)];
const MEMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(12), FloatColumn::Binary64(13), FloatColumn::Binary64(14), FloatColumn::Binary64(15)];
const BRIDGE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11)];
const EFFECT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7)];
type Entities<'a> = BTreeMap<i64, FloatRow<'a>>;
type References<'a> = BTreeMap<&'a str, Option<i64>>;

fn checkpoint(control: &mut SqliteSnapshotControl<'_>, position: usize, total: usize) -> Result<(), String> {
    if position % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, position, total)?; }
    Ok(())
}
fn entities<'a>(database: &'a SqliteDatabase, name: &str, columns: usize, floats: &'static [FloatColumn], control: &mut SqliteSnapshotControl<'_>) -> Result<Entities<'a>, String> {
    let rows = &database.table(name)?.rows;
    let mut result = BTreeMap::new();
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        let row = FloatRow::new(row, floats)?;
        if row.values.len() != columns || row.rowid <= 0 || row.integer(0)? != row.rowid || result.insert(row.rowid, row).is_some() { return Err(format!("{name} requires positive unique aliased identities and exact fields")); }
    }
    Ok(result)
}
fn ordered<'a>(rows: impl Iterator<Item = FloatRow<'a>>, ordinal: usize, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<FloatRow<'a>>, String> {
    let mut result = BTreeMap::new();
    for (position, row) in rows.enumerate() {
        checkpoint(control, position, 0)?;
        let index = usize::try_from(row.integer(ordinal)?).map_err(|error| error.to_string())?;
        if result.insert(index, row).is_some() { return Err("EN1990 ordinals must be unique".into()); }
    }
    for (expected, actual) in result.keys().copied().enumerate() { checkpoint(control, expected, result.len())?; if expected != actual { return Err("EN1990 ordinals must be dense".into()); } }
    Ok(result.into_values().collect())
}
fn remember<'a>(references: &mut References<'a>, name: &'a str, identity: i64) {
    references.entry(name).and_modify(|id| *id = None).or_insert(Some(identity));
}
fn resolved(references: &References<'_>, name: &str) -> Cell<'static> {
    references.get(name).copied().flatten().map(Cell::Integer).unwrap_or(Cell::Null)
}
fn validate_reference(row: FloatRow<'_>, column: usize, name: &str, references: &References<'_>) -> Result<(), String> {
    let expected = references.get(name).copied().flatten().map(SqliteValue::Integer).unwrap_or(SqliteValue::Null);
    if row.values.get(column) != Some(&expected) { return Err("EN1990 resolved reference disagrees with the unique logical target".into()); }
    Ok(())
}
fn importance(value: ImportanceClass) -> &'static str { match value { ImportanceClass::I => "I", ImportanceClass::II => "II", ImportanceClass::III => "III", ImportanceClass::IV => "IV" } }
fn read_importance(value: &str) -> Result<ImportanceClass, String> { match value { "I" => Ok(ImportanceClass::I), "II" => Ok(ImportanceClass::II), "III" => Ok(ImportanceClass::III), "IV" => Ok(ImportanceClass::IV), _ => Err("EN1990 importance class is not owned".into()) } }
fn ordinal(value: usize) -> Result<Cell<'static>, String> { Ok(Cell::Integer(i64::try_from(value).map_err(|error| error.to_string())?)) }

impl ArtifactSqliteSnapshot for En1990Snapshot {
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),Self::__dsl_from_record_controlled,control)}
    const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");

    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or("Native semantic row count overflow");let mut rows=1usize;for size in[self.permanents.len(),self.variables.len(),self.accidentals.len(),self.seismics.len()]{rows=add(rows,size.checked_mul(2).ok_or("EN1990 action row count overflow")?)?}for size in[self.members.len(),self.bridge_sls.len(),self.effects.len()]{rows=add(rows,size)?}control.check_rows(rows)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control)
 }

 fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), String> {
        let mut bound = NativeEncodingBound::new(control)?;
        bound.add(32768)?;
        for value in [&self.project_id, &self.structure_kind, &self.supervision_level, &self.inspection_level] { bound.repeated(value.len(), 24)?; }
        for action in &self.permanents { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; bound.repeated(action.kind.len(), 24)?; }
        for action in &self.variables { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; bound.repeated(action.category.len(), 24)?; }
        for action in &self.accidentals { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; }
        for action in &self.seismics { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; }
        for member in &self.members { bound.add(8192)?; for value in [&member.id, &member.label_en, &member.label_de] { bound.repeated(value.len(), 24)?; } }
        for bridge in &self.bridge_sls { bound.add(4096)?; bound.repeated(bridge.id.len(), 24)?; bound.repeated(bridge.member_id.len(), 24)?; }
        for effect in &self.effects { bound.add(2048)?; bound.repeated(effect.member_id.len(), 24)?; bound.repeated(effect.action_id.len(), 24)?; }
        bound.finish()
    }

    fn to_sqlite_database(&self, control: &mut SqliteSnapshotControl<'_>) -> Result<SqliteDatabase, String> {
        let mut out = Projection::new(Self::SQLITE_SCHEMA, control)?;
        insert_key_ieee754(&mut out, "en1990_document", 1, &[Cell::Text(match self.annex { AnnexChoice::En => "En", AnnexChoice::De => "De" }), Cell::Text(&self.project_id), Cell::Text(&self.structure_kind), Cell::Real(self.altitude_m), Cell::Integer(i64::from(self.consequence_class)), Cell::Integer(i64::from(self.reliability_class)), Cell::Integer(i64::from(self.design_working_life_category)), Cell::Real(self.design_working_life_years), Cell::Real(self.reference_period_years), Cell::Text(&self.supervision_level), Cell::Text(&self.inspection_level), Cell::Real(self.k_fi_declared), Cell::Real(self.beta_computed)], DOCUMENT_FLOATS)?;
        let mut actions = References::new();
        for (position, action) in self.permanents.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("permanent"), ordinal(position)?, Cell::Text(&action.id)])?;
            remember(&mut actions, &action.id, id);
            insert_key_ieee754(&mut out, "en1990_permanent_action", id, &[Cell::Text(&action.kind), Cell::Real(action.gk)], PERMANENT_FLOATS)?;
        }
        for (position, action) in self.variables.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("variable"), ordinal(position)?, Cell::Text(&action.id)])?;
            remember(&mut actions, &action.id, id);
            insert_key_ieee754(&mut out, "en1990_variable_action", id, &[Cell::Text(&action.category), Cell::Real(action.qk)], VARIABLE_FLOATS)?;
        }
        for (position, action) in self.accidentals.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("accidental"), ordinal(position)?, Cell::Text(&action.id)])?;
            remember(&mut actions, &action.id, id);
            insert_key_ieee754(&mut out, "en1990_accidental_action", id, &[Cell::Real(action.ad)], ACCIDENTAL_FLOATS)?;
        }
        for (position, action) in self.seismics.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("seismic"), ordinal(position)?, Cell::Text(&action.id)])?;
            remember(&mut actions, &action.id, id);
            insert_key_ieee754(&mut out, "en1990_seismic_action", id, &[Cell::Real(action.a_ek), Cell::Text(importance(action.importance_class))], SEISMIC_FLOATS)?;
        }
        let mut members = References::new();
        for (position, member) in self.members.iter().enumerate() {
            let id = insert_ieee754(&mut out, "en1990_member", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&member.id), Cell::Text(&member.label_en), Cell::Text(&member.label_de), Cell::Real(member.rd_str), Cell::Real(member.rd_geo), Cell::Real(member.rd_equ_stab), Cell::Real(member.rd_equ_destab), Cell::Real(member.rd_fat), Cell::Real(member.span), Cell::Real(member.deflection_w), Cell::Real(member.deflection_limit_ratio), Cell::Real(member.vibration_frequency), Cell::Real(member.vibration_frequency_min)], MEMBER_FLOATS)?;
            remember(&mut members, &member.id, id);
        }
        for (position, bridge) in self.bridge_sls.iter().enumerate() {
            insert_ieee754(&mut out, "en1990_bridge_sls", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&bridge.id), Cell::Text(&bridge.member_id), resolved(&members, &bridge.member_id), Cell::Real(bridge.deck_acceleration), Cell::Real(bridge.deck_acceleration_limit), Cell::Real(bridge.deck_twist), Cell::Real(bridge.deck_twist_limit), Cell::Real(bridge.bridge_deflection), Cell::Real(bridge.bridge_deflection_limit)], BRIDGE_FLOATS)?;
        }
        for (position, effect) in self.effects.iter().enumerate() {
            insert_ieee754(&mut out, "en1990_effect", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&effect.member_id), Cell::Text(&effect.action_id), resolved(&members, &effect.member_id), resolved(&actions, &effect.action_id), Cell::Real(effect.influence)], EFFECT_FLOATS)?;
        }
        out.finish()
    }

    fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, String> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits()).map_err(|error| error.to_string())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents = entities(database, "en1990_document", 14, DOCUMENT_FLOATS, control)?;
        if documents.len() != 1 || !documents.contains_key(&1) { return Err("EN1990 requires exactly document identity one".into()); }
        let document = documents[&1];
        let actions = entities(database, "en1990_action", 5, &[], control)?;
        let permanent_rows = entities(database, "en1990_permanent_action", 3, PERMANENT_FLOATS, control)?;
        let variable_rows = entities(database, "en1990_variable_action", 3, VARIABLE_FLOATS, control)?;
        let accidental_rows = entities(database, "en1990_accidental_action", 2, ACCIDENTAL_FLOATS, control)?;
        let seismic_rows = entities(database, "en1990_seismic_action", 3, SEISMIC_FLOATS, control)?;
        let member_rows = entities(database, "en1990_member", 16, MEMBER_FLOATS, control)?;
        let bridge_rows = entities(database, "en1990_bridge_sls", 12, BRIDGE_FLOATS, control)?;
        let effect_rows = entities(database, "en1990_effect", 8, EFFECT_FLOATS, control)?;
        let mut action_references = References::new();
        let mut permanent_order = Vec::new(); let mut variable_order = Vec::new(); let mut accidental_order = Vec::new(); let mut seismic_order = Vec::new();
        for (position, (&id, action)) in actions.iter().enumerate() {
            checkpoint(control, position, actions.len())?;
            if action.integer(1)? != 1 { return Err("EN1990 action has an unknown document".into()); }
            remember(&mut action_references, action.text(4)?, id);
            match action.text(2)? { "permanent" => permanent_order.push(*action), "variable" => variable_order.push(*action), "accidental" => accidental_order.push(*action), "seismic" => seismic_order.push(*action), _ => return Err("EN1990 action choice is not owned".into()) }
        }
        for (kind, rows, count) in [("permanent", &permanent_rows, permanent_order.len()), ("variable", &variable_rows, variable_order.len()), ("accidental", &accidental_rows, accidental_order.len()), ("seismic", &seismic_rows, seismic_order.len())] {
            if rows.len() != count { return Err("EN1990 action subtype coverage differs".into()); }
            for (position, &id) in rows.keys().enumerate() { checkpoint(control, position, rows.len())?; if actions.get(&id).ok_or("EN1990 action subtype is orphaned")?.text(2)? != kind { return Err("EN1990 action subtype disagrees with its owned choice".into()); } }
        }
        let mut permanents = Vec::new();
        for action in ordered(permanent_order.into_iter(), 3, control)? { checkpoint(control, permanents.len(), 0)?; let row = permanent_rows[&action.rowid]; permanents.push(PermanentAction { id: reconstruct_text(control, action.text(4)?)?, kind: reconstruct_text(control, row.text(1)?)?, gk: row.real(2)? }); }
        let mut variables = Vec::new();
        for action in ordered(variable_order.into_iter(), 3, control)? { checkpoint(control, variables.len(), 0)?; let row = variable_rows[&action.rowid]; variables.push(VariableAction { id: reconstruct_text(control, action.text(4)?)?, category: reconstruct_text(control, row.text(1)?)?, qk: row.real(2)? }); }
        let mut accidentals = Vec::new();
        for action in ordered(accidental_order.into_iter(), 3, control)? { checkpoint(control, accidentals.len(), 0)?; let row = accidental_rows[&action.rowid]; accidentals.push(AccidentalAction { id: reconstruct_text(control, action.text(4)?)?, ad: row.real(1)? }); }
        let mut seismics = Vec::new();
        for action in ordered(seismic_order.into_iter(), 3, control)? { checkpoint(control, seismics.len(), 0)?; let row = seismic_rows[&action.rowid]; seismics.push(SeismicAction { id: reconstruct_text(control, action.text(4)?)?, a_ek: row.real(1)?, importance_class: read_importance(row.text(2)?)? }); }
        let mut member_references = References::new();
        let mut members = Vec::new();
        for row in ordered(member_rows.values().copied(), 2, control)? {
            checkpoint(control, members.len(), 0)?;
            if row.integer(1)? != 1 { return Err("EN1990 member has an unknown document".into()); }
            remember(&mut member_references, row.text(3)?, row.rowid);
            members.push(Member { id: reconstruct_text(control, row.text(3)?)?, label_en: reconstruct_text(control, row.text(4)?)?, label_de: reconstruct_text(control, row.text(5)?)?, rd_str: row.real(6)?, rd_geo: row.real(7)?, rd_equ_stab: row.real(8)?, rd_equ_destab: row.real(9)?, rd_fat: row.real(10)?, span: row.real(11)?, deflection_w: row.real(12)?, deflection_limit_ratio: row.real(13)?, vibration_frequency: row.real(14)?, vibration_frequency_min: row.real(15)? });
        }
        let mut bridge_sls = Vec::new();
        for row in ordered(bridge_rows.values().copied(), 2, control)? {
            checkpoint(control, bridge_sls.len(), 0)?;
            if row.integer(1)? != 1 { return Err("EN1990 bridge SLS has an unknown document".into()); }
            validate_reference(row, 5, row.text(4)?, &member_references)?;
            bridge_sls.push(BridgeSls { id: reconstruct_text(control, row.text(3)?)?, member_id: reconstruct_text(control, row.text(4)?)?, deck_acceleration: row.real(6)?, deck_acceleration_limit: row.real(7)?, deck_twist: row.real(8)?, deck_twist_limit: row.real(9)?, bridge_deflection: row.real(10)?, bridge_deflection_limit: row.real(11)? });
        }
        let mut effects = Vec::new();
        for row in ordered(effect_rows.values().copied(), 2, control)? {
            checkpoint(control, effects.len(), 0)?;
            if row.integer(1)? != 1 { return Err("EN1990 influence has an unknown document".into()); }
            validate_reference(row, 5, row.text(3)?, &member_references)?; validate_reference(row, 6, row.text(4)?, &action_references)?;
            effects.push(MemberEffect { member_id: reconstruct_text(control, row.text(3)?)?, action_id: reconstruct_text(control, row.text(4)?)?, influence: row.real(7)? });
        }
        let result = Self { annex: match document.text(1)? { "En" => AnnexChoice::En, "De" => AnnexChoice::De, _ => return Err("EN1990 annex choice is not owned".into()) }, project_id: reconstruct_text(control, document.text(2)?)?, structure_kind: reconstruct_text(control, document.text(3)?)?, altitude_m: document.real(4)?, consequence_class: u8::try_from(document.integer(5)?).map_err(|error| error.to_string())?, reliability_class: u8::try_from(document.integer(6)?).map_err(|error| error.to_string())?, design_working_life_category: u8::try_from(document.integer(7)?).map_err(|error| error.to_string())?, design_working_life_years: document.real(8)?, reference_period_years: document.real(9)?, supervision_level: reconstruct_text(control, document.text(10)?)?, inspection_level: reconstruct_text(control, document.text(11)?)?, k_fi_declared: document.real(12)?, beta_computed: document.real(13)?, permanents, variables, accidentals, seismics, members, bridge_sls, effects };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(result)
    }
}

/// 🪶️ Binds the EN1990 owner to its explicit relational capability.
pub fn sqlite_codec() -> store::ArtifactSqliteSnapshotCodec { <En1990Snapshot as ArtifactSqliteSnapshot>::sqlite_codec() }
