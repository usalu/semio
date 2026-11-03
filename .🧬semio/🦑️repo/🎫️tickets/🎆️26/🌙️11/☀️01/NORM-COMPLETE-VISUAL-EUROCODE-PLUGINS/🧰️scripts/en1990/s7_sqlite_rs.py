from spec import *
import json
SQ = SUB + "/🚪️io/🪶️sqlite/📸️snapshot"
p = SQ + "/🦀️.rs"
t = open(p, encoding="utf8").read()
if "FATIGUE_FLOATS" in t: raise SystemExit("done")
def rep(old, new, count=1):
    global t
    assert t.count(old) == count, (old[:70], t.count(old))
    t = t.replace(old, new)
rep("use semio_s_artifact_norm_en1990_model::{AccidentalAction, BridgeSls, ImportanceClass, Member, MemberEffect, PermanentAction, SeismicAction, VariableAction};", "use semio_s_artifact_norm_en1990_model::{AccidentalAction, BridgeSls, FatigueSpectrum, ImportanceClass, Member, MemberEffect, PermanentAction, ReliabilityAnalysis, SeismicAction, TestSeries, VariableAction};")
rep("const EFFECT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7)];\n", "const EFFECT_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7)];\nconst FATIGUE_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(6), FloatColumn::Binary64(7), FloatColumn::Binary64(8)];\nconst RELIABILITY_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(5), FloatColumn::Binary64(6), FloatColumn::Binary64(8), FloatColumn::Binary64(9)];\nconst TEST_FLOATS: &[FloatColumn] = &[FloatColumn::Binary64(7), FloatColumn::Binary64(8), FloatColumn::Binary64(9), FloatColumn::Binary64(10), FloatColumn::Binary64(11)];\n")
rep("&[Cell::Text(&action.category), Cell::Real(action.qk)], VARIABLE_FLOATS)", "&[Cell::Text(&action.category), Cell::Real(action.qk), Cell::Text(&action.group)], VARIABLE_FLOATS)")
rep("""EFFECT_FLOATS)?;
        }
        Ok(())
 }""", """EFFECT_FLOATS)?;
        }
        for (position, spectrum) in self.fatigue_spectra.iter().enumerate() {
            let member=resolve_index(&members,&spectrum.member_id,out)?;
            out.insert_float( "en1990_fatigue_spectrum", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&spectrum.id), Cell::Text(&spectrum.member_id), member, Cell::Real(spectrum.detail_category), Cell::Real(spectrum.stress_range), Cell::Real(spectrum.cycles)], FATIGUE_FLOATS)?;
        }
        for (position, analysis) in self.reliability_analyses.iter().enumerate() {
            out.insert_float( "en1990_reliability_analysis", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&analysis.id), Cell::Text(&analysis.resistance_distribution), Cell::Real(analysis.resistance_mean), Cell::Real(analysis.resistance_sd), Cell::Text(&analysis.action_distribution), Cell::Real(analysis.action_mean), Cell::Real(analysis.action_sd)], RELIABILITY_FLOATS)?;
        }
        for (position, series) in self.test_series.iter().enumerate() {
            out.insert_float( "en1990_test_series", &[Cell::Integer(1), ordinal(position)?, Cell::Text(&series.id), Cell::Text(&series.quantity), Cell::Text(&series.distribution), Cell::Integer(i64::from(series.sample_size)), Cell::Real(series.mean), Cell::Real(series.std_dev), Cell::Real(series.variation_known), Cell::Real(series.conversion_factor), Cell::Real(series.required)], TEST_FLOATS)?;
        }
        Ok(())
 }""")
rep("SQLITE_SCHEMA,9,57,control)", "SQLITE_SCHEMA,12,57,control)")
rep("for size in[self.members.len(),self.bridge_sls.len(),self.effects.len()]{rows=add(rows,size)?}", "for size in[self.members.len(),self.bridge_sls.len(),self.effects.len(),self.fatigue_spectra.len(),self.reliability_analyses.len(),self.test_series.len()]{rows=add(rows,size)?}")
rep("        for effect in &self.effects { bound.add(2048)?; bound.repeated(effect.member_id.len(), 24)?; bound.repeated(effect.action_id.len(), 24)?; }\n", "        for effect in &self.effects { bound.add(2048)?; bound.repeated(effect.member_id.len(), 24)?; bound.repeated(effect.action_id.len(), 24)?; }\n        for action in &self.variables { bound.repeated(action.group.len(), 24)?; }\n        for spectrum in &self.fatigue_spectra { bound.add(4096)?; bound.repeated(spectrum.id.len(), 24)?; bound.repeated(spectrum.member_id.len(), 24)?; }\n        for analysis in &self.reliability_analyses { bound.add(4096)?; bound.repeated(analysis.id.len(), 24)?; bound.repeated(analysis.resistance_distribution.len(), 24)?; bound.repeated(analysis.action_distribution.len(), 24)?; }\n        for series in &self.test_series { bound.add(4096)?; bound.repeated(series.id.len(), 24)?; bound.repeated(series.quantity.len(), 24)?; bound.repeated(series.distribution.len(), 24)?; }\n")
rep('"en1990_variable_action", 3, VARIABLE_FLOATS', '"en1990_variable_action", 4, VARIABLE_FLOATS')
rep('        let effect_rows = entities(database, "en1990_effect", 8, EFFECT_FLOATS, control)?;\n', '        let effect_rows = entities(database, "en1990_effect", 8, EFFECT_FLOATS, control)?;\n        let fatigue_rows = entities(database, "en1990_fatigue_spectrum", 9, FATIGUE_FLOATS, control)?;\n        let reliability_rows = entities(database, "en1990_reliability_analysis", 10, RELIABILITY_FLOATS, control)?;\n        let test_rows = entities(database, "en1990_test_series", 12, TEST_FLOATS, control)?;\n')
rep("category: reconstruct_text(control, row.text(1)?)?, qk: row.real(2)? });", "category: reconstruct_text(control, row.text(1)?)?, qk: row.real(2)?, group: reconstruct_text(control, row.text(3)?)? });")
rep("""        let result = Self { annex:""", """        let mut fatigue_spectra = Vec::new();
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
            reliability_analyses.push(ReliabilityAnalysis { id: reconstruct_text(control, row.text(3)?)?, resistance_distribution: reconstruct_text(control, row.text(4)?)?, resistance_mean: row.real(5)?, resistance_sd: row.real(6)?, action_distribution: reconstruct_text(control, row.text(7)?)?, action_mean: row.real(8)?, action_sd: row.real(9)? });
        }
        let mut test_series = Vec::new();
        for row in ordered(test_rows.values().copied(), 2, control)? {
            checkpoint(control, test_series.len(), 0)?;
            if row.integer(1)? != 1 { return Err(invalid("EN1990 test series has an unknown document")); }
            test_series.push(TestSeries { id: reconstruct_text(control, row.text(3)?)?, quantity: reconstruct_text(control, row.text(4)?)?, distribution: reconstruct_text(control, row.text(5)?)?, sample_size: u32::try_from(row.integer(6)?).map_err(|error| invalid(error.to_string()))?, mean: row.real(7)?, std_dev: row.real(8)?, variation_known: row.real(9)?, conversion_factor: row.real(10)?, required: row.real(11)? });
        }
        let result = Self { annex:""")
rep("members, bridge_sls, effects };\n        control.checkpoint", "members, bridge_sls, effects, fatigue_spectra, reliability_analyses, test_series };\n        control.checkpoint")
open(p, "w", encoding="utf8").write(t)

# admission
p = SQ + "/🛂️admission/🦀️.rs"; t = open(p, encoding="utf8").read()
rep("for id in 17..=19{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}", "for id in 17..=22{rows=n::add(rows,n::list(n::field(root,id)?)?.len())?}")
rep('14=>out.insert_key_float("en1990_variable_action",id,&n::cells(a,[Text(1),Real(2)])?,VARIABLE_FLOATS)?', '14=>out.insert_key_float("en1990_variable_action",id,&n::cells(a,[Text(1),Real(2),Text(3)])?,VARIABLE_FLOATS)?')
rep("""EFFECT_FLOATS)?;}Ok(())
}""", """EFFECT_FLOATS)?;}
 for(index,f)in n::list(n::field(root,20)?)?.iter().enumerate(){let f=n::record(f)?;let reference=n::text(n::field(f,1)?)?;let member=resolve_index(&members,reference,out)?;out.insert_float("en1990_fatigue_spectrum",&[Cell::Integer(1),n::ordinal(index)?,Cell::Text(n::text(n::field(f,0)?)?),Cell::Text(reference),member,Cell::Real(n::real(n::field(f,2)?)?),Cell::Real(n::real(n::field(f,3)?)?),Cell::Real(n::real(n::field(f,4)?)?)],FATIGUE_FLOATS)?;}
 for(index,r)in n::list(n::field(root,21)?)?.iter().enumerate(){let r=n::record(r)?;n::entity(out,"en1990_reliability_analysis",1,index,&n::cells(r,[Text(0),Text(1),Real(2),Real(3),Text(4),Real(5),Real(6)])?,RELIABILITY_FLOATS)?;}
 for(index,s)in n::list(n::field(root,22)?)?.iter().enumerate(){let s=n::record(s)?;n::entity(out,"en1990_test_series",1,index,&n::cells(s,[Text(0),Text(1),Text(2),Unsigned(3,u32::MAX as u64),Real(4),Real(5),Real(6),Real(7),Real(8)])?,TEST_FLOATS)?;}Ok(())
}""")
rep("/// 🛂️ Nine SQL tables", "/// 🛂️ Twelve SQL tables")
rep("SQLITE_SCHEMA,9,57,limits)?;let rows", "SQLITE_SCHEMA,12,57,limits)?;let rows")
rep("En1990Snapshot::SQLITE_SCHEMA,9,57,rows,", "En1990Snapshot::SQLITE_SCHEMA,12,57,rows,")
open(p, "w", encoding="utf8").write(t)
print("rs ok")
