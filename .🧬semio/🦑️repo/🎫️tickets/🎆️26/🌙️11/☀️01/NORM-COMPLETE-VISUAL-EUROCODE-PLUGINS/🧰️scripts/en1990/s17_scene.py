from spec import *
S = SUB + "/🎬️scene"
def patch(path, pairs):
    t = open(path, encoding="utf8").read()
    for old, new in pairs:
        assert t.count(old) == 1, (path[-30:], old[:70], t.count(old))
        t = t.replace(old, new)
    open(path, "w", encoding="utf8").write(t)
patch(S + "/🧰️kit/🦀️.rs", [
 ('pub(crate) const LAYER_ANNOTATIONS: &str = "annotations";\n', 'pub(crate) const LAYER_ANNOTATIONS: &str = "annotations";\npub(crate) const LAYER_ANALYSES: &str = "analyses";\n'),
 ('const LAYERS: [(&str, &str, &str); 5] = [', 'const LAYERS: [(&str, &str, &str); 6] = ['),
 ('    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),\n];', '    (LAYER_ANNOTATIONS, "Annotations", "Beschriftungen"),\n    (LAYER_ANALYSES, "Analyses", "Analysen"),\n];'),
 ('    kind("combination", "combinations",', '    kind("fatigueSpectrum", "fatigueSpectra", "Fatigue spectrum bin", "Ermüdungskollektiv-Klasse", "bar-chart", EntityKind::Plate),\n    kind("reliabilityAnalysis", "reliabilityAnalyses", "Reliability analysis", "Zuverlässigkeitsanalyse", "target", EntityKind::Plate),\n    kind("testSeries", "testSeries", "Test series", "Versuchsreihe", "flask-conical", EntityKind::Plate),\n    kind("combination", "combinations",'),
 ('let found: [(&str, bool); 7] = [', 'let found: [(&str, bool); 10] = ['),
 ('        ("bridgeSls", d.bridge_sls.iter().any(|e| e.id == base)),\n', '        ("bridgeSls", d.bridge_sls.iter().any(|e| e.id == base)),\n        ("fatigueSpectra", d.fatigue_spectra.iter().any(|e| e.id == base)),\n        ("reliabilityAnalyses", d.reliability_analyses.iter().any(|e| e.id == base)),\n        ("testSeries", d.test_series.iter().any(|e| e.id == base)),\n'),
 ('        out.extend(d.bridge_sls.iter().filter(|s| s.member_id == base).map(|s| s.id.clone()));\n    }\n', '        out.extend(d.bridge_sls.iter().filter(|s| s.member_id == base).map(|s| s.id.clone()));\n        out.extend(d.fatigue_spectra.iter().filter(|s| s.member_id == base).map(|s| s.id.clone()));\n    }\n    out.extend(d.fatigue_spectra.iter().filter(|e| e.id == base).map(|e| e.member_id.clone()));\n'),
 ('    std::iter::once(member).chain(d.bridge_sls.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str())).collect()', '    std::iter::once(member).chain(d.bridge_sls.iter().filter(|e| e.member_id == member).map(|e| e.id.as_str())).chain(d.fatigue_spectra.iter().filter(|e| e.member_id == member).take(1).map(|e| e.id.as_str())).collect()'),
 ('            MaterialRole::Support => ("Supports", "Lager"),', '            MaterialRole::Support => ("Supports", "Lager"),\n            MaterialRole::Steel => ("Analyses", "Analysen"),'),
])
patch(S + "/🦀️.rs", [
 ('#[path = "⚓️actions/🦀️.rs"]\nmod actions;', '#[path = "⚓️actions/🦀️.rs"]\nmod actions;\n\n#[path = "📊️analyses/🦀️.rs"]\nmod analyses;'),
 ('    actions::project(input, &layout, active, &mut out);\n', '    actions::project(input, &layout, active, &mut out);\n    analyses::project(input, &layout, &mut out);\n'),
 ("let actions = d.permanents.len() + d.variables.len() + d.accidentals.len() + d.seismics.len();", "let actions = d.permanents.len() + d.variables.len() + d.accidentals.len() + d.seismics.len();\n    let analyses = d.fatigue_spectra.len() + d.reliability_analyses.len() + d.test_series.len();"),
 ('format!("EN 1990 basis of design with {} members, {actions} actions and {combinations} combinations; worst utilisation {:.0} %.", d.members.len(), worst * 100.0)', 'format!("EN 1990 basis of design with {} members, {actions} actions, {analyses} analysis rows and {combinations} combinations; worst utilisation {:.0} %.", d.members.len(), worst * 100.0)'),
 ('format!("EN-1990-Tragwerksplanung mit {} Bauteilen, {actions} Einwirkungen und {combinations} Kombinationen; größte Ausnutzung {:.0} %.", d.members.len(), worst * 100.0)', 'format!("EN-1990-Tragwerksplanung mit {} Bauteilen, {actions} Einwirkungen, {analyses} Analysezeilen und {combinations} Kombinationen; größte Ausnutzung {:.0} %.", d.members.len(), worst * 100.0)'),
])
patch(S + "/🕹️handles/🦀️.rs", [
 ('use super::actions::{affected, lift, rows, selector_anchor};', 'use super::actions::{affected, lift, rows, selector_anchor};\nuse super::analyses;'),
 ('/// 🧭️ The view-state path', '''fn analysis_handles(d: &En1990Snapshot, layout: &Layout, id: &str) -> Vec<Handle> {
    analyses::bars(d, layout)
        .into_iter()
        .filter(|bar| bar.owner == id)
        .filter_map(|bar| {
            let field = bar.field?;
            let kind = if bar.collection == "fatigueSpectra" { EntityKind::Plate } else { EntityKind::Plate };
            let entity = entity_ref(kind, id, bar.collection);
            Some(handle(Spec {
                entity: &entity,
                tag: field,
                kind: HandleKind::ScalarAlong { axis: Vec3::Z },
                anchor: v3(bar.x, bar.y, bar.height()),
                grip: GripStyle::Arrow,
                writes: vec![binding(format!("{}[id={id}].{field}", bar.collection), Affine { base: 0.0, k: bar.per_metre }, if bar.collection == "fatigueSpectra" { "Pa" } else { "N" }, Some(Quantise::Round { step: bar.step }))],
                snap: grid(0.05),
                limits: (0.02, 12.0),
                name: bar.name.clone(),
            }))
        })
        .collect()
}

/// 🧭️ The view-state path'''),
 ('[member_handles(document, layout, id), action_handles(document, layout, id), combination_handles(document, layout, id)].concat()', '[member_handles(document, layout, id), action_handles(document, layout, id), analysis_handles(document, layout, id), combination_handles(document, layout, id)].concat()'),
])
print("ok")
