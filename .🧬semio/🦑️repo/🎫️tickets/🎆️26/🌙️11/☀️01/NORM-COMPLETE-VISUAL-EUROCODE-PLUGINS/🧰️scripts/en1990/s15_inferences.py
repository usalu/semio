from spec import *
INF = SCH + "/💡️inferences"
def patch(path, pairs):
    t = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert t.count(old) == 1, (path[-40:], old[:70], t.count(old))
        t = t.replace(old, new)
    open(path, "w", encoding="utf8").write(t)
patch(INF + "/🦀️.rs", [
 ('                "effects",\n            ],', '                "effects",\n                "fatigueSpectra",\n                "reliabilityAnalyses",\n                "testSeries",\n            ],'),
 ('#[path = "🧮️partial-factor-sets/🦀️.rs"]', '#[path = "📊️annex-analyses/🦀️.rs"]\nmod annex_analyses;\nuse annex_analyses::*;\n\n#[path = "🧮️partial-factor-sets/🦀️.rs"]'),
])
patch(INF + "/📋️compliance-evaluation/🦀️.rs", [
 ("        push_a2_bridge_sls(&mut report, document, member);\n    }\n    report\n}", """        push_a2_bridge_sls(&mut report, document, member);
        push_fatigue_damage(&mut report, document, member);
    }
    for (_, analysis) in document.reliability_analyses.iter().enumerate().filter(|(index, _)| scope.admits("reliabilityAnalyses", *index)) {
        push_reliability_analysis(&mut report, document, analysis);
    }
    for (_, series) in document.test_series.iter().enumerate().filter(|(index, _)| scope.admits("testSeries", *index)) {
        push_test_series(&mut report, document, series);
    }
    report
}"""),
])
patch(INF + "/🧮️units/🦀️.rs", [
 ('Rows::except(d, &["members"])', 'Rows::except(d, &["members", "reliabilityAnalyses", "testSeries"])'),
 ('&["members.*.id"], move', '&["members.*.id", "fatigueSpectra.*"], move'),
 ("        rows.finish()", """        for (index, analysis) in d.reliability_analyses.iter().enumerate() {
            rows = rows.row("reliabilityAnalyses", "reliabilityAnalyses", index, &analysis.id, CostTier::T1, &[], move |d: &En1990Snapshot| evaluate_scoped(d, Scope::Row("reliabilityAnalyses", index)).checks);
        }
        for (index, series) in d.test_series.iter().enumerate() {
            rows = rows.row("testSeries", "testSeries", index, &series.id, CostTier::T0, &[], move |d: &En1990Snapshot| evaluate_scoped(d, Scope::Row("testSeries", index)).checks);
        }
        rows.finish()"""),
])
patch(INF + "/🧾outline/🦀️.rs", [
 ('    "effects",\n];', '    "effects",\n    "fatigueSpectra",\n    "reliabilityAnalyses",\n    "testSeries",\n];'),
 ("snapshot.bridge_sls.len() + snapshot.effects.len()) as u32", "snapshot.bridge_sls.len() + snapshot.effects.len() + snapshot.fatigue_spectra.len() + snapshot.reliability_analyses.len() + snapshot.test_series.len()) as u32"),
])
patch(INF + "/⛓️referential-integrity/🦀️.rs", [
 ('    push_duplicate_ids(report, document, "bridgeSls", &document.bridge_sls.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("bridgeSls[id={id}].id"));\n', '    push_duplicate_ids(report, document, "bridgeSls", &document.bridge_sls.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("bridgeSls[id={id}].id"));\n    push_duplicate_ids(report, document, "fatigueSpectra", &document.fatigue_spectra.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("fatigueSpectra[id={id}].id"));\n    push_duplicate_ids(report, document, "reliabilityAnalyses", &document.reliability_analyses.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("reliabilityAnalyses[id={id}].id"));\n    push_duplicate_ids(report, document, "testSeries", &document.test_series.iter().map(|s| s.id.clone()).collect::<Vec<_>>(), &|id| format!("testSeries[id={id}].id"));\n'),
 ("    if !document.effects.is_empty() {", """    for spectrum in &document.fatigue_spectra {
        if !member_set.contains(spectrum.member_id.as_str()) {
            push_dangling_ref(report, document, format!("en1990.integrity.fatigueSpectra.{}.memberId", spectrum.id), format!("fatigueSpectra[id={}].memberId", spectrum.id), &spectrum.member_id, "Fatigue spectrum memberId", "Ermüdungskollektiv memberId", &spectrum.member_id, members.clone());
        }
    }

    if !document.effects.is_empty() {"""),
])
patch(INF + "/🧰️report-helpers/🦀️.rs", [
 ("actions.push(Action::new(&variable.id, ActionClass::Variable, value).category(category));", "actions.push(Action::new(&variable.id, ActionClass::Variable, value).category(category).group(variable.group.clone()));"),
])
print("ok")
