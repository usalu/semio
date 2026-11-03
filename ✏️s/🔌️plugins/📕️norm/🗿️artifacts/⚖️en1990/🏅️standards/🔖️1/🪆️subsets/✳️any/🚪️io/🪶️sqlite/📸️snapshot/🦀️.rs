//! ⚖️ Handwritten EN1990 actions, verification members and influence relationships.
use pack::snapshot::ArtifactSqliteSnapshot as _;
use crate::En1990Snapshot;
use crate::{AccidentalAction, BridgeSls, FatigueSpectrum, ImportanceClass, Member, MemberEffect, PermanentAction, ReliabilityAnalysis, ReliabilityClass, SeismicAction, TestSeries, Placement, FactorOverride, LoadGroup, TrafficGroup, VariableAction, CaseRow, CaseNumber, CaseText, DistributionKind, TestDistribution, PermanentKind, StructureKind, SupervisionLevel, InspectionLevel};
use semio_s_plugin_norm_compliance::AnnexChoice;
use std::collections::BTreeMap;
use store::{os_store::serialization::ArtifactSqliteSnapshotHost, sqlite_snapshot::{SnapshotEncoding, SqliteDatabase, SqliteValue, SqliteSnapshotControl, SqliteSnapshotPhase, validate_sqlite_database_schema, artifact::{Cell, FloatColumn, FloatRow, NativeEncodingBound, RowWriter, reconstruct_text}}};
use semio_framework_value::{ValueError, ValueRefusalKind};
fn inspection_level_text(a:InspectionLevel)->&'static str{match a{InspectionLevel::Il1=>"il1",InspectionLevel::Il2=>"il2",InspectionLevel::Il3=>"il3",}}
fn read_inspection_level(a:&str)->Result<InspectionLevel, ValueError>{match a{"il1"=>Ok(InspectionLevel::Il1),"il2"=>Ok(InspectionLevel::Il2),"il3"=>Ok(InspectionLevel::Il3),_=>Err(invalid("EN1990 inspection level differs"))}}
fn supervision_level_text(a:SupervisionLevel)->&'static str{match a{SupervisionLevel::Dsl1=>"dsl1",SupervisionLevel::Dsl2=>"dsl2",SupervisionLevel::Dsl3=>"dsl3",}}
fn read_supervision_level(a:&str)->Result<SupervisionLevel, ValueError>{match a{"dsl1"=>Ok(SupervisionLevel::Dsl1),"dsl2"=>Ok(SupervisionLevel::Dsl2),"dsl3"=>Ok(SupervisionLevel::Dsl3),_=>Err(invalid("EN1990 supervision level differs"))}}
fn structure_kind_text(a:StructureKind)->&'static str{match a{StructureKind::Building=>"building",StructureKind::RoadBridge=>"roadBridge",StructureKind::Footbridge=>"footbridge",StructureKind::RailBridge=>"railBridge",}}
fn read_structure_kind(a:&str)->Result<StructureKind, ValueError>{match a{"building"=>Ok(StructureKind::Building),"roadBridge"=>Ok(StructureKind::RoadBridge),"footbridge"=>Ok(StructureKind::Footbridge),"railBridge"=>Ok(StructureKind::RailBridge),_=>Err(invalid("EN1990 structure kind differs"))}}
fn permanent_kind_text(a:PermanentKind)->&'static str{match a{PermanentKind::GSup=>"gSup",PermanentKind::GInf=>"gInf",PermanentKind::Prestress=>"prestress",}}
fn read_permanent_kind(a:&str)->Result<PermanentKind, ValueError>{match a{"gSup"=>Ok(PermanentKind::GSup),"gInf"=>Ok(PermanentKind::GInf),"prestress"=>Ok(PermanentKind::Prestress),_=>Err(invalid("EN1990 permanent kind differs"))}}
fn test_distribution_text(a:TestDistribution)->&'static str{match a{TestDistribution::Normal=>"normal",TestDistribution::Lognormal=>"lognormal",}}
fn read_test_distribution(a:&str)->Result<TestDistribution, ValueError>{match a{"normal"=>Ok(TestDistribution::Normal),"lognormal"=>Ok(TestDistribution::Lognormal),_=>Err(invalid("EN1990 test distribution differs"))}}
fn distribution_kind_text(a:DistributionKind)->&'static str{match a{DistributionKind::Normal=>"normal",DistributionKind::Lognormal=>"lognormal",DistributionKind::Gumbel=>"gumbel",DistributionKind::Weibull=>"weibull",}}
fn read_distribution_kind(a:&str)->Result<DistributionKind, ValueError>{match a{"normal"=>Ok(DistributionKind::Normal),"lognormal"=>Ok(DistributionKind::Lognormal),"gumbel"=>Ok(DistributionKind::Gumbel),"weibull"=>Ok(DistributionKind::Weibull),_=>Err(invalid("EN1990 distribution kind differs"))}}
fn invalid(message: impl Into<String>) -> ValueError { ValueError::new(ValueRefusalKind::InvalidValue, message) }

const DOCUMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(12)];
const PERMANENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(2)];
const VARIABLE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(2)];
const ACCIDENTAL_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(1)];
const SEISMIC_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(1)];
const MEMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(12), FloatColumn::Binary64(13), FloatColumn::Binary64(14), FloatColumn::Binary64(15), FloatColumn::Binary64(16), FloatColumn::Binary64(17), FloatColumn::Binary64(18), FloatColumn::Binary64(19), FloatColumn::Binary64(20), FloatColumn::Binary64(21), FloatColumn::Binary64(22), FloatColumn::Binary64(23), FloatColumn::Binary64(24), FloatColumn::Binary64(25), FloatColumn::Binary64(26), FloatColumn::Binary64(27), FloatColumn::Binary64(28), FloatColumn::Binary64(29), FloatColumn::Binary64(30), FloatColumn::Binary64(31), FloatColumn::Binary64(32), FloatColumn::Binary64(33), FloatColumn::Binary64(34), FloatColumn::Binary64(35)];
const BRIDGE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11), FloatColumn::Binary64(12), FloatColumn::Binary64(13), FloatColumn::Binary64(14), FloatColumn::Binary64(15), FloatColumn::Binary64(16), FloatColumn::Binary64(17), FloatColumn::Binary64(18), FloatColumn::Binary64(19)];
const EFFECT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7)];
const FATIGUE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8)];
const RELIABILITY_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(8), FloatColumn::Binary64(9)];
const TEST_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11),FloatColumn::Binary64(13)];
const CASE_NUMBER_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4)];
const FACTOR_OVERRIDE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6)];
const PLACEMENT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(4), FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(7)];
type Entities<'a> = BTreeMap<i64, FloatRow<'a>>;
type References<'a> = BTreeMap<&'a str, Option<i64>>;

fn checkpoint(control: &mut SqliteSnapshotControl<'_>, position: usize, total: usize) -> Result<(), ValueError> {
    if position % 256 == 0 { control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, position, total)?; }
    Ok(())
}
fn entities<'a>(database: &'a SqliteDatabase, name: &str, columns: usize, floats: &'static [FloatColumn], control: &mut SqliteSnapshotControl<'_>) -> Result<Entities<'a>, ValueError> {
    let rows = &database.table(name)?.rows;
    let mut result = BTreeMap::new();
    for (position, row) in rows.iter().enumerate() {
        checkpoint(control, position, rows.len())?;
        let row = FloatRow::new(row, floats)?;
        if row.values.len() != columns || row.rowid <= 0 || row.integer(0)? != row.rowid || result.insert(row.rowid, row).is_some() { return Err(invalid(format!("{name} requires positive unique aliased identities and exact fields"))); }
    }
    Ok(result)
}
fn ordered<'a>(rows: impl Iterator<Item = FloatRow<'a>>, ordinal: usize, control: &mut SqliteSnapshotControl<'_>) -> Result<Vec<FloatRow<'a>>, ValueError> {
    let mut result = BTreeMap::new();
    for (position, row) in rows.enumerate() {
        checkpoint(control, position, 0)?;
        let index = usize::try_from(row.integer(ordinal)?).map_err(|error| invalid(error.to_string()))?;
        if result.insert(index, row).is_some() { return Err(invalid("EN1990 ordinals must be unique")); }
    }
    for (expected, actual) in result.keys().copied().enumerate() { checkpoint(control, expected, result.len())?; if expected != actual { return Err(invalid("EN1990 ordinals must be dense")); } }
    Ok(result.into_values().collect())
}
fn remember<'a>(references: &mut References<'a>, name: &'a str, identity: i64) {
    references.entry(name).and_modify(|id| *id = None).or_insert(Some(identity));
}
fn validate_reference(row: FloatRow<'_>, column: usize, name: &str, references: &References<'_>) -> Result<(), ValueError> {
    let expected = references.get(name).copied().flatten().map(SqliteValue::Integer).unwrap_or(SqliteValue::Null);
    if row.values.get(column) != Some(&expected) { return Err(invalid("EN1990 resolved reference disagrees with the unique logical target")); }
    Ok(())
}
fn importance(value: ImportanceClass) -> &'static str { match value { ImportanceClass::I => "I", ImportanceClass::II => "II", ImportanceClass::III => "III", ImportanceClass::IV => "IV" } }
fn read_importance(value: &str) -> Result<ImportanceClass, ValueError> { match value { "I" => Ok(ImportanceClass::I), "II" => Ok(ImportanceClass::II), "III" => Ok(ImportanceClass::III), "IV" => Ok(ImportanceClass::IV), _ => Err(invalid("EN1990 importance class is not owned")) } }
fn reliability(value: ReliabilityClass) -> &'static str { match value { ReliabilityClass::RC1 => "RC1", ReliabilityClass::RC2 => "RC2", ReliabilityClass::RC3 => "RC3" } }
fn read_reliability(value: &str) -> Result<ReliabilityClass, ValueError> { match value { "RC1" => Ok(ReliabilityClass::RC1), "RC2" => Ok(ReliabilityClass::RC2), "RC3" => Ok(ReliabilityClass::RC3), _ => Err(invalid("EN1990 reliability class is not owned")) } }
fn load_group(value: LoadGroup) -> &'static str { match value { LoadGroup::GR1A=>"GR1A",LoadGroup::GR1B=>"GR1B",LoadGroup::GR2=>"GR2",LoadGroup::GR3=>"GR3",LoadGroup::GR4=>"GR4",LoadGroup::GR5=>"GR5",LoadGroup::GR6=>"GR6" } }
fn read_load_group(value: &str) -> Result<LoadGroup, ValueError> { match value { "GR1A"=>Ok(LoadGroup::GR1A),"GR1B"=>Ok(LoadGroup::GR1B),"GR2"=>Ok(LoadGroup::GR2),"GR3"=>Ok(LoadGroup::GR3),"GR4"=>Ok(LoadGroup::GR4),"GR5"=>Ok(LoadGroup::GR5),"GR6"=>Ok(LoadGroup::GR6), _ => Err(invalid("EN1990 traffic load group is not owned")) } }
fn ordinal(value: usize) -> Result<Cell<'static>, ValueError> { Ok(Cell::Integer(i64::try_from(value).map_err(|error| invalid(error.to_string()))?)) }

fn sort_references(values:&mut[(&str,i64)],out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{let phase=out.phase();out.sort_frontier(values,|left,right,control|store::sqlite_snapshot::transfer::compare_text(left.0,right.0,phase,control))}
fn resolve_index(values:&[(&str,i64)],name:&str,out:&mut RowWriter<'_,'_>)->Result<Cell<'static>,ValueError>{let(mut low,mut high)=(0,values.len());while low<high{let middle=low+(high-low)/2;match out.compare_text(values[middle].0,name)?{std::cmp::Ordering::Less=>low=middle+1,_=>high=middle}}
 if low==values.len()||!out.compare_text(values[low].0,name)?.is_eq(){return Ok(Cell::Null)}if low+1<values.len()&&out.compare_text(values[low+1].0,name)?.is_eq(){return Ok(Cell::Null)}Ok(Cell::Integer(values[low].1))}
impl En1990Snapshot{
 fn write_sqlite_rows(&self,out:&mut RowWriter<'_,'_>)->Result<(),ValueError>{
        out.insert_key_float( "en1990_document", 1, &[Cell::Text(match self.annex { AnnexChoice::En => "En", AnnexChoice::De => "De" }), Cell::Text(&self.project_id), Cell::Text(structure_kind_text(self.structure_kind)), Cell::Real(self.altitude_m), Cell::Integer(i64::from(self.consequence_class)), Cell::Text(reliability(self.reliability_class)), Cell::Integer(i64::from(self.design_working_life_category)), Cell::Real(self.design_working_life_years), Cell::Real(self.reference_period_years), Cell::Text(supervision_level_text(self.supervision_level)), Cell::Text(inspection_level_text(self.inspection_level)), Cell::Real(self.beta_computed)], DOCUMENT_FLOATS)?;
        let mut actions = out.allocate_frontier(0)?;
        for (position, action) in self.permanents.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("permanent"), ordinal(position)?, Cell::Text(&action.id)])?;
            out.push_frontier(&mut actions, (action.id.as_str(),id))?;
            out.insert_key_float( "en1990_permanent_action", id, &[Cell::Text(permanent_kind_text(action.kind)), Cell::Real(action.gk)], PERMANENT_FLOATS)?;
        }
        for (position, action) in self.variables.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("variable"), ordinal(position)?, Cell::Text(&action.id)])?;
            out.push_frontier(&mut actions, (action.id.as_str(),id))?;
            out.insert_key_float( "en1990_variable_action", id, &[Cell::Text(&action.category), Cell::Real(action.qk), Cell::Text(&action.group)], VARIABLE_FLOATS)?;
        }
        for (position, action) in self.accidentals.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("accidental"), ordinal(position)?, Cell::Text(&action.id)])?;
            out.push_frontier(&mut actions, (action.id.as_str(),id))?;
            out.insert_key_float( "en1990_accidental_action", id, &[Cell::Real(action.ad)], ACCIDENTAL_FLOATS)?;
        }
        for (position, action) in self.seismics.iter().enumerate() {
            let id = out.insert("en1990_action", &[Cell::Integer(1), Cell::Text("seismic"), ordinal(position)?, Cell::Text(&action.id)])?;
            out.push_frontier(&mut actions, (action.id.as_str(),id))?;
            out.insert_key_float( "en1990_seismic_action", id, &[Cell::Real(action.a_ek), Cell::Text(importance(action.importance_class))], SEISMIC_FLOATS)?;
        }
        sort_references(&mut actions,out)?;
        let mut members = out.allocate_frontier(0)?;
        for (position, member) in self.members.iter().enumerate() {
            let id = out.insert_float( "en1990_member", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&member.id), Cell::Text(&member.label_en), Cell::Text(&member.label_de), Cell::Real(member.rd_str), Cell::Real(member.rd_geo), Cell::Real(member.rd_equ_stab), Cell::Real(member.rd_equ_destab), Cell::Real(member.rd_fat), Cell::Real(member.span), Cell::Real(member.precamber), Cell::Real(member.deflection_w1), Cell::Real(member.deflection_w2), Cell::Real(member.deflection_w3), Cell::Real(member.deflection_limit_ratio), Cell::Real(member.deflection_limit_ratio_w3), Cell::Real(member.sls_limit_char), Cell::Real(member.sls_limit_freq), Cell::Real(member.sls_limit_qp), Cell::Real(member.vibration_frequency), Cell::Real(member.vibration_frequency_min), Cell::Real(member.frame_height), Cell::Real(member.frame_drift), Cell::Real(member.storey_height), Cell::Real(member.storey_drift), Cell::Real(member.material_strength_k), Cell::Real(member.material_conversion_factor), Cell::Real(member.material_partial_factor), Cell::Real(member.material_design_strength), Cell::Real(member.nominal_dimension), Cell::Real(member.dimension_tolerance), Cell::Real(member.design_dimension), Cell::Real(member.member_alpha_cr), Cell::Real(member.second_order_analysis)], MEMBER_FLOATS)?;
            out.push_frontier(&mut members, (member.id.as_str(),id))?;
        }
        sort_references(&mut members,out)?;
        for (position, bridge) in self.bridge_sls.iter().enumerate() {
            let member=resolve_index(&members,&bridge.member_id,out)?;
            out.insert_float( "en1990_bridge_sls", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&bridge.id), Cell::Text(&bridge.member_id), member, Cell::Real(bridge.deck_acceleration), Cell::Real(bridge.deck_acceleration_limit), Cell::Real(bridge.deck_twist), Cell::Real(bridge.deck_twist_limit), Cell::Real(bridge.bridge_deflection), Cell::Real(bridge.bridge_deflection_limit), Cell::Real(bridge.track_gauge), Cell::Real(bridge.line_speed), Cell::Real(bridge.ballast_depth), Cell::Real(bridge.deck_mass), Cell::Real(bridge.natural_frequency), Cell::Real(bridge.end_rotation), Cell::Real(bridge.basic_wind_speed), Cell::Real(bridge.damping_ratio)], BRIDGE_FLOATS)?;
        }
        for (position, effect) in self.effects.iter().enumerate() {
            let member=resolve_index(&members,&effect.member_id,out)?;let action=resolve_index(&actions,&effect.action_id,out)?;
            out.insert_float( "en1990_effect", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&effect.member_id), Cell::Text(&effect.action_id), member, action, Cell::Real(effect.influence)], EFFECT_FLOATS)?;
        }
        for (position, spectrum) in self.fatigue_spectra.iter().enumerate() {
            let member=resolve_index(&members,&spectrum.member_id,out)?;
            out.insert_float( "en1990_fatigue_spectrum", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&spectrum.id), Cell::Text(&spectrum.member_id), member, Cell::Real(spectrum.detail_category), Cell::Real(spectrum.stress_range), Cell::Real(spectrum.cycles)], FATIGUE_FLOATS)?;
        }
        for (position, analysis) in self.reliability_analyses.iter().enumerate() {
            out.insert_float( "en1990_reliability_analysis", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&analysis.id), Cell::Text(distribution_kind_text(analysis.resistance_distribution)), Cell::Real(analysis.resistance_mean), Cell::Real(analysis.resistance_sd), Cell::Text(distribution_kind_text(analysis.action_distribution)), Cell::Real(analysis.action_mean), Cell::Real(analysis.action_sd)], RELIABILITY_FLOATS)?;
        }
        for (position, series) in self.test_series.iter().enumerate() {
            out.insert_float( "en1990_test_series", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&series.id), Cell::Text(&series.quantity), Cell::Text(test_distribution_text(series.distribution)), Cell::Integer(i64::from(series.sample_size)), Cell::Real(series.mean), Cell::Real(series.std_dev), Cell::Real(series.variation_known), Cell::Real(series.conversion_factor), Cell::Real(series.required),Cell::Text(&series.pairs),Cell::Real(series.theory_variation)], TEST_FLOATS)?;
        }
        for (position, placement) in self.placements.iter().enumerate() {
            out.insert_float( "en1990_placement", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&placement.id), Cell::Real(placement.x), Cell::Real(placement.y), Cell::Real(placement.z), Cell::Real(placement.yaw)], PLACEMENT_FLOATS)?;
        }
        for (position, case) in self.cases.iter().enumerate() {
            let id = out.insert_float("en1990_case", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&case.id), Cell::Text(&case.label_en), Cell::Text(&case.label_de), Cell::Text(&case.entry_id)], &[])?;
            for (index, number) in case.numbers.iter().enumerate() { out.insert_float("en1990_case_number", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&number.id), Cell::Real(number.value)], CASE_NUMBER_FLOATS)?; }
            for (index, text) in case.texts.iter().enumerate() { out.insert_float("en1990_case_text", &[Cell::Integer(id), ordinal(index)?, Cell::Text(&text.id), Cell::Text(&text.value)], &[])?; }
        }
        for (position, row) in self.factor_overrides.iter().enumerate() {
            out.insert_float("en1990_factor_override", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(&row.factor), Cell::Text(&row.target), Cell::Real(row.value), Cell::Text(&row.justification)], FACTOR_OVERRIDE_FLOATS)?;
        }
        for (position, row) in self.traffic_groups.iter().enumerate() {
            out.insert_float("en1990_traffic_group", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&row.id), Cell::Text(load_group(row.group)), Cell::Text(&row.variable_id)], &[])?;
        }
        Ok(())
 }
 fn admit_sqlite_values(&self,control:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<(),ValueError>{semio_s_artifact_norm_contract::sqlite_native::schema(En1990Snapshot::SQLITE_SCHEMA,18,96,control)?;let mut out=RowWriter::borrowed(control,phase)?;self.write_sqlite_rows(&mut out)?;out.finish_borrowed()}
}
impl ArtifactSqliteSnapshotHost for En1990Snapshot {
 fn decode_sqlite_snapshot_native(payload:&store::io::IoPayload,control:&mut SqliteSnapshotControl<'_>,native_control: &mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{let limits=control.limits();store::decode_sqlite_snapshot_record_native(payload,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record, snapshot_output, native,_body| { let constructed: Result<_, ValueError> = (|| {admission::admit(record,native,limits)?;Self::__dsl_from_record_controlled(record,native)})(); *snapshot_output = Some(constructed?); Ok(()) },control,native_control)}
    

    fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload, ValueError>{
  control.checkpoint(SqliteSnapshotPhase::EncodeNative,0,0)?;let add=|count:usize,size:usize|count.checked_add(size).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "Native semantic row count overflow"));let mut rows=1usize;for size in[self.permanents.len(),self.variables.len(),self.accidentals.len(),self.seismics.len()]{rows=add(rows,size.checked_mul(2).ok_or_else(|| ValueError::new(ValueRefusalKind::WorkLimit, "EN1990 action row count overflow"))?)?}for size in[self.members.len(),self.bridge_sls.len(),self.effects.len(),self.fatigue_spectra.len(),self.reliability_analyses.len(),self.test_series.len(),self.placements.len(),self.cases.len(),self.factor_overrides.len(),self.traffic_groups.len()]{rows=add(rows,size)?}for case in &self.cases{rows=add(rows,case.numbers.len())?;rows=add(rows,case.texts.len())?}control.check_rows(rows)?;
  self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
  store::encode_sqlite_snapshot_record_native(encoding,<Self as pack::snapshot::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),control,native_owner)
 }

 fn preflight_sqlite_snapshot_encoding(&self, _: SnapshotEncoding, control: &mut SqliteSnapshotControl<'_>) -> Result<(), ValueError> {self.admit_sqlite_values(control,SqliteSnapshotPhase::EncodeNative)?;
        let mut bound = NativeEncodingBound::file_only(control)?;
        bound.add(32768)?;
        bound.repeated(self.project_id.len(), 24)?;
        for action in &self.permanents { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; bound.repeated(action.kind.len(), 24)?; }
        for action in &self.variables { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; bound.repeated(action.category.len(), 24)?; }
        for action in &self.accidentals { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; }
        for action in &self.seismics { bound.add(2048)?; bound.repeated(action.id.len(), 24)?; }
        for member in &self.members { bound.add(8192)?; for value in [&member.id, &member.label_en, &member.label_de] { bound.repeated(value.len(), 24)?; } }
        for bridge in &self.bridge_sls { bound.add(4096)?; bound.repeated(bridge.id.len(), 24)?; bound.repeated(bridge.member_id.len(), 24)?; }
        for effect in &self.effects { bound.add(2048)?; bound.repeated(effect.member_id.len(), 24)?; bound.repeated(effect.action_id.len(), 24)?; }
        for action in &self.variables { bound.repeated(action.group.len(), 24)?; }
        for spectrum in &self.fatigue_spectra { bound.add(4096)?; bound.repeated(spectrum.id.len(), 24)?; bound.repeated(spectrum.member_id.len(), 24)?; }
        for analysis in &self.reliability_analyses { bound.add(4096)?; bound.repeated(analysis.id.len(), 24)?; bound.repeated(analysis.resistance_distribution.len(), 24)?; bound.repeated(analysis.action_distribution.len(), 24)?; }
        for series in &self.test_series { bound.add(4096)?; bound.repeated(series.id.len(), 24)?; bound.repeated(series.quantity.len(), 24)?; bound.repeated(series.distribution.len(), 24)?; }
        for placement in &self.placements { bound.add(4096)?; bound.repeated(placement.id.len(), 24)?; }
        for row in &self.factor_overrides { bound.add(4096)?; for value in [&row.id, &row.factor, &row.target, &row.justification] { bound.repeated(value.len(), 24)?; } }
        for row in &self.traffic_groups { bound.add(4096)?; for value in [&row.id, &row.variable_id] { bound.repeated(value.len(), 24)?; } }
        for case in &self.cases { bound.add(4096)?; for value in [&case.id, &case.label_en, &case.label_de, &case.entry_id] { bound.repeated(value.len(), 24)?; } for number in &case.numbers { bound.add(2048)?; bound.repeated(number.id.len(), 24)?; } for text in &case.texts { bound.add(2048)?; for value in [&text.id, &text.value] { bound.repeated(value.len(), 24)?; } } }
        bound.finish()
    }

    

    
}
impl pack::snapshot::ArtifactSqliteSnapshot for En1990Snapshot {
const SQLITE_SCHEMA: &'static str = include_str!("🗄️.sql");
fn to_sqlite_database(&self,control:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{let mut out=RowWriter::new(Self::SQLITE_SCHEMA,control)?;self.write_sqlite_rows(&mut out)?;out.finish()}
fn from_sqlite_database(database: &SqliteDatabase, control: &mut SqliteSnapshotControl<'_>) -> Result<Self, ValueError> {
        validate_sqlite_database_schema(database, Self::SQLITE_SCHEMA, control.limits())?;
        control.check_database(database, SqliteSnapshotPhase::ReconstructSnapshot)?;
        let documents = entities(database, "en1990_document", 13, DOCUMENT_FLOATS, control)?;
        if documents.len() != 1 || !documents.contains_key(&1) { return Err(invalid("EN1990 requires exactly document identity one")); }
        let document = documents[&1];
        let actions = entities(database, "en1990_action", 5, &[], control)?;
        let permanent_rows = entities(database, "en1990_permanent_action", 3, PERMANENT_FLOATS, control)?;
        let variable_rows = entities(database, "en1990_variable_action", 4, VARIABLE_FLOATS, control)?;
        let accidental_rows = entities(database, "en1990_accidental_action", 2, ACCIDENTAL_FLOATS, control)?;
        let seismic_rows = entities(database, "en1990_seismic_action", 3, SEISMIC_FLOATS, control)?;
        let member_rows = entities(database, "en1990_member", 36, MEMBER_FLOATS, control)?;
        let bridge_rows = entities(database, "en1990_bridge_sls", 20, BRIDGE_FLOATS, control)?;
        let effect_rows = entities(database, "en1990_effect", 8, EFFECT_FLOATS, control)?;
        let fatigue_rows = entities(database, "en1990_fatigue_spectrum", 9, FATIGUE_FLOATS, control)?;
        let reliability_rows = entities(database, "en1990_reliability_analysis", 10, RELIABILITY_FLOATS, control)?;
        let test_rows = entities(database, "en1990_test_series", 14, TEST_FLOATS, control)?;
        let placement_rows = entities(database, "en1990_placement", 8, PLACEMENT_FLOATS, control)?;
        let override_rows = entities(database, "en1990_factor_override", 8, FACTOR_OVERRIDE_FLOATS, control)?;
        let traffic_rows = entities(database, "en1990_traffic_group", 6, &[], control)?;
        let case_rows = entities(database, "en1990_case", 7, &[], control)?;
        let mut case_numbers = BTreeMap::<i64, Vec<FloatRow<'_>>>::new();
        for row in entities(database, "en1990_case_number", 5, CASE_NUMBER_FLOATS, control)?.into_values() { let parent = row.integer(1)?; if !case_rows.contains_key(&parent) { return Err(invalid("EN1990 case number has an unknown case")); } case_numbers.entry(parent).or_default().push(row); }
        let mut case_texts = BTreeMap::<i64, Vec<FloatRow<'_>>>::new();
        for row in entities(database, "en1990_case_text", 5, &[], control)?.into_values() { let parent = row.integer(1)?; if !case_rows.contains_key(&parent) { return Err(invalid("EN1990 case text has an unknown case")); } case_texts.entry(parent).or_default().push(row); }
        let mut action_references = References::new();
        let mut permanent_order = Vec::new(); let mut variable_order = Vec::new(); let mut accidental_order = Vec::new(); let mut seismic_order = Vec::new();
        for (position, (&id, action)) in actions.iter().enumerate() {
            checkpoint(control, position, actions.len())?;
            if action.integer(1)? != 1 { return Err(invalid("EN1990 action has an unknown document")); }
            remember(&mut action_references, action.text(4)?, id);
            match action.text(2)? { "permanent" => permanent_order.push(*action), "variable" => variable_order.push(*action), "accidental" => accidental_order.push(*action), "seismic" => seismic_order.push(*action), _ => return Err(invalid("EN1990 action choice is not owned")) }
        }
        for (kind, rows, count) in [("permanent", &permanent_rows, permanent_order.len()), ("variable", &variable_rows, variable_order.len()), ("accidental", &accidental_rows, accidental_order.len()), ("seismic", &seismic_rows, seismic_order.len())] {
            if rows.len() != count { return Err(invalid("EN1990 action subtype coverage differs")); }
            for (position, &id) in rows.keys().enumerate() { checkpoint(control, position, rows.len())?; if actions.get(&id).ok_or_else(|| invalid("EN1990 action subtype is orphaned"))?.text(2)? != kind { return Err(invalid("EN1990 action subtype disagrees with its owned choice")); } }
        }
        let mut permanents = Vec::new();
        for action in ordered(permanent_order.into_iter(), 3, control)? { checkpoint(control, permanents.len(), 0)?; let row = permanent_rows[&action.rowid]; permanents.push(PermanentAction { id: reconstruct_text(control, action.text(4)?)?, kind: read_permanent_kind(row.text(1)?)?, gk: row.real(2)? }); }
        let mut variables = Vec::new();
        for action in ordered(variable_order.into_iter(), 3, control)? { checkpoint(control, variables.len(), 0)?; let row = variable_rows[&action.rowid]; variables.push(VariableAction { id: reconstruct_text(control, action.text(4)?)?, category: reconstruct_text(control, row.text(1)?)?, qk: row.real(2)?, group: reconstruct_text(control, row.text(3)?)? }); }
        let mut accidentals = Vec::new();
        for action in ordered(accidental_order.into_iter(), 3, control)? { checkpoint(control, accidentals.len(), 0)?; let row = accidental_rows[&action.rowid]; accidentals.push(AccidentalAction { id: reconstruct_text(control, action.text(4)?)?, ad: row.real(1)? }); }
        let mut seismics = Vec::new();
        for action in ordered(seismic_order.into_iter(), 3, control)? { checkpoint(control, seismics.len(), 0)?; let row = seismic_rows[&action.rowid]; seismics.push(SeismicAction { id: reconstruct_text(control, action.text(4)?)?, a_ek: row.real(1)?, importance_class: read_importance(row.text(2)?)? }); }
        let mut member_references = References::new();
        let mut members = Vec::new();
        for row in ordered(member_rows.values().copied(), 2, control)? {
            checkpoint(control, members.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 member has an unknown document")); }
            remember(&mut member_references, row.text(3)?, row.rowid);
            members.push(Member { id: reconstruct_text(control, row.text(3)?)?, label_en: reconstruct_text(control, row.text(4)?)?, label_de: reconstruct_text(control, row.text(5)?)?, rd_str: row.real(6)?, rd_geo: row.real(7)?, rd_equ_stab: row.real(8)?, rd_equ_destab: row.real(9)?, rd_fat: row.real(10)?, span: row.real(11)?, precamber: row.real(12)?, deflection_w1: row.real(13)?, deflection_w2: row.real(14)?, deflection_w3: row.real(15)?, deflection_limit_ratio: row.real(16)?, deflection_limit_ratio_w3: row.real(17)?, sls_limit_char: row.real(18)?, sls_limit_freq: row.real(19)?, sls_limit_qp: row.real(20)?, vibration_frequency: row.real(21)?, vibration_frequency_min: row.real(22)?, frame_height: row.real(23)?, frame_drift: row.real(24)?, storey_height: row.real(25)?, storey_drift: row.real(26)?, material_strength_k: row.real(27)?, material_conversion_factor: row.real(28)?, material_partial_factor: row.real(29)?, material_design_strength: row.real(30)?, nominal_dimension: row.real(31)?, dimension_tolerance: row.real(32)?, design_dimension: row.real(33)?, member_alpha_cr: row.real(34)?, second_order_analysis: row.real(35)? });
        }
        let mut bridge_sls = Vec::new();
        for row in ordered(bridge_rows.values().copied(), 2, control)? {
            checkpoint(control, bridge_sls.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 bridge SLS has an unknown document")); }
            validate_reference(row, 5, row.text(4)?, &member_references)?;
            bridge_sls.push(BridgeSls { id: reconstruct_text(control, row.text(3)?)?, member_id: reconstruct_text(control, row.text(4)?)?, deck_acceleration: row.real(6)?, deck_acceleration_limit: row.real(7)?, deck_twist: row.real(8)?, deck_twist_limit: row.real(9)?, bridge_deflection: row.real(10)?, bridge_deflection_limit: row.real(11)?, track_gauge: row.real(12)?, line_speed: row.real(13)?, ballast_depth: row.real(14)?, deck_mass: row.real(15)?, natural_frequency: row.real(16)?, end_rotation: row.real(17)?, basic_wind_speed: row.real(18)?, damping_ratio: row.real(19)? });
        }
        let mut effects = Vec::new();
        for row in ordered(effect_rows.values().copied(), 2, control)? {
            checkpoint(control, effects.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 influence has an unknown document")); }
            validate_reference(row, 5, row.text(3)?, &member_references)?; validate_reference(row, 6, row.text(4)?, &action_references)?;
            effects.push(MemberEffect { member_id: reconstruct_text(control, row.text(3)?)?, action_id: reconstruct_text(control, row.text(4)?)?, influence: row.real(7)? });
        }
        let mut fatigue_spectra = Vec::new();
        for row in ordered(fatigue_rows.values().copied(), 2, control)? {
            checkpoint(control, fatigue_spectra.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 fatigue spectrum has an unknown document")); }
            validate_reference(row, 5, row.text(4)?, &member_references)?;
            fatigue_spectra.push(FatigueSpectrum { id: reconstruct_text(control, row.text(3)?)?, member_id: reconstruct_text(control, row.text(4)?)?, detail_category: row.real(6)?, stress_range: row.real(7)?, cycles: row.real(8)? });
        }
        let mut reliability_analyses = Vec::new();
        for row in ordered(reliability_rows.values().copied(), 2, control)? {
            checkpoint(control, reliability_analyses.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 reliability analysis has an unknown document")); }
            reliability_analyses.push(ReliabilityAnalysis { id: reconstruct_text(control, row.text(3)?)?, resistance_distribution: read_distribution_kind(row.text(4)?)?, resistance_mean: row.real(5)?, resistance_sd: row.real(6)?, action_distribution: read_distribution_kind(row.text(7)?)?, action_mean: row.real(8)?, action_sd: row.real(9)? });
        }
        let mut test_series = Vec::new();
        for row in ordered(test_rows.values().copied(), 2, control)? {
            checkpoint(control, test_series.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 test series has an unknown document")); }
            test_series.push(TestSeries { id: reconstruct_text(control, row.text(3)?)?, quantity: reconstruct_text(control, row.text(4)?)?, distribution: read_test_distribution(row.text(5)?)?, sample_size: u32::try_from(row.integer(6)?).map_err(|error| invalid(error.to_string()))?, mean: row.real(7)?, std_dev: row.real(8)?, variation_known: row.real(9)?, conversion_factor: row.real(10)?, required: row.real(11)?,pairs: reconstruct_text(control, row.text(12)?)?,theory_variation: row.real(13)? });
        }
        let mut placements = Vec::new();
        for row in ordered(placement_rows.values().copied(), 2, control)? {
            checkpoint(control, placements.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 placement has an unknown document")); }
            placements.push(Placement { id: reconstruct_text(control, row.text(3)?)?, x: row.real(4)?, y: row.real(5)?, z: row.real(6)?, yaw: row.real(7)? });
        }
        let mut cases = Vec::new();
        for row in ordered(case_rows.values().copied(), 2, control)? {
            checkpoint(control, cases.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 case has an unknown document")); }
            let mut numbers = Vec::new();
            for number in ordered(case_numbers.remove(&row.rowid).unwrap_or_default().into_iter(), 2, control)? { numbers.push(CaseNumber { id: reconstruct_text(control, number.text(3)?)?, value: number.real(4)? }); }
            let mut texts = Vec::new();
            for text in ordered(case_texts.remove(&row.rowid).unwrap_or_default().into_iter(), 2, control)? { texts.push(CaseText { id: reconstruct_text(control, text.text(3)?)?, value: reconstruct_text(control, text.text(4)?)? }); }
            cases.push(CaseRow { id: reconstruct_text(control, row.text(3)?)?, label_en: reconstruct_text(control, row.text(4)?)?, label_de: reconstruct_text(control, row.text(5)?)?, entry_id: reconstruct_text(control, row.text(6)?)?, numbers, texts });
        }
        let mut factor_overrides = Vec::new();
        for row in ordered(override_rows.values().copied(), 2, control)? {
            checkpoint(control, factor_overrides.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 factor override has an unknown document")); }
            factor_overrides.push(FactorOverride { id: reconstruct_text(control, row.text(3)?)?, factor: reconstruct_text(control, row.text(4)?)?, target: reconstruct_text(control, row.text(5)?)?, value: row.real(6)?, justification: reconstruct_text(control, row.text(7)?)? });
        }
        let mut traffic_groups = Vec::new();
        for row in ordered(traffic_rows.values().copied(), 2, control)? {
            checkpoint(control, traffic_groups.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 traffic group has an unknown document")); }
            traffic_groups.push(TrafficGroup { id: reconstruct_text(control, row.text(3)?)?, group: read_load_group(row.text(4)?)?, variable_id: reconstruct_text(control, row.text(5)?)? });
        }
        let result = Self { annex: match document.text(1)? { "En" => AnnexChoice::En, "De" => AnnexChoice::De, _ => return Err(invalid("EN1990 annex choice is not owned")) }, project_id: reconstruct_text(control, document.text(2)?)?, structure_kind: read_structure_kind(document.text(3)?)?, altitude_m: document.real(4)?, consequence_class: u8::try_from(document.integer(5)?).map_err(|error| invalid(error.to_string()))?, reliability_class: read_reliability(document.text(6)?)?, design_working_life_category: u8::try_from(document.integer(7)?).map_err(|error| invalid(error.to_string()))?, design_working_life_years: document.real(8)?, reference_period_years: document.real(9)?, supervision_level: read_supervision_level(document.text(10)?)?, inspection_level: read_inspection_level(document.text(11)?)?, beta_computed: document.real(12)?, permanents, variables, accidentals, seismics, members, bridge_sls, effects, fatigue_spectra, reliability_analyses, test_series, placements, cases, factor_overrides, traffic_groups };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(result)
    }
}

/// 🪶️ Binds the EN1990 owner to its explicit relational capability.
pub fn sqlite_codec() -> store::os_store::serialization::ArtifactSqliteSnapshotCodec { <En1990Snapshot as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec() }

#[path="🛂️admission/🦀️.rs"]
pub mod admission;

