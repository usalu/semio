from spec import *
p = SUB + "/🎬️scene/📈️diagrams/🦀️.rs"
t = open(p, encoding="utf8").read()
if "form_diagram" in t: raise SystemExit("done")
def rep(old, new, count=1):
    global t
    assert t.count(old) == count, (old[:80], t.count(old)); t = t.replace(old, new)
rep("use super::checks::{annex_c_alphas, endurance_cycles, k_n,", "use super::checks::{analysis_form, annex_c_alphas, endurance_cycles, k_n, spectrum_damage, test_design_value,")
rep("use super::model::{En1990Snapshot, Member};", "use super::model::{En1990Snapshot, Member, ReliabilityAnalysis, TestSeries};")
# form split
i = t.index("fn form(document: &En1990Snapshot) -> Diagram2d {"); j = t.index("fn annex_d()")
body = t[i:j]
body = body.replace("""fn form(document: &En1990Snapshot) -> Diagram2d {
    let beta = document.beta_computed.max(0.5);
    let (alpha_r, alpha_e) = annex_c_alphas(document.annex, 1.0, 1.0);
""", """fn form(document: &En1990Snapshot) -> Diagram2d {
    let (alpha_r, alpha_e) = annex_c_alphas(document.annex, 1.0, 1.0);
    form_diagram("form".into(), loc("FORM design point", "FORM-Bemessungspunkt"), document.beta_computed.max(0.5), alpha_r, alpha_e, "en1990.annex-c.beta".into(), None)
}

fn analysis_forms(document: &En1990Snapshot, focus: Option<&str>) -> Vec<Diagram2d> {
    let selected = |a: &&ReliabilityAnalysis| focus.is_none_or(|f| f == a.id || f == document.project_id);
    document.reliability_analyses.iter().filter(selected).filter_map(|a| {
        let form = analysis_form((a.resistance_distribution.as_str(), a.resistance_mean, a.resistance_sd), (a.action_distribution.as_str(), a.action_mean, a.action_sd)).ok()?;
        Some(form_diagram(format!("form:{}", a.id), loc(&format!("FORM design point – {}", a.id), &format!("FORM-Bemessungspunkt – {}", a.id)), form.beta.max(0.5), form.alpha_r, form.alpha_e, format!("en1990.annex-c.form.{}", a.id), Some(EntityRef::new(EntityKind::Plate, &a.id, format!("reliabilityAnalyses[id={}]", a.id)))))
    }).collect()
}

fn form_diagram(id: String, title: LocalizedCopy, beta: f64, alpha_r: f64, alpha_e: f64, bound: String, focus: Option<EntityRef>) -> Diagram2d {
""")
assert "let (alpha_r, alpha_e) = annex_c_alphas(document.annex, 1.0, 1.0);\n    let (nx, ny)" not in body
body = body.replace('        bound_check: Some("en1990.annex-c.beta".into()),', '        bound_check: Some(bound),')
body = body.replace('        id: "form".into(),\n        title: loc("FORM design point", "FORM-Bemessungspunkt"),', '        id,\n        title,')
body = body.replace("        focus: None,\n        legend: true,\n        table_twin: DataTable { columns: vec![column(\"Action effect u_E\"", "        focus,\n        legend: true,\n        table_twin: DataTable { columns: vec![column(\"Action effect u_E\"")
assert "        focus,\n" in body and "id,\n        title," in body and "bound_check: Some(bound)" in body
t = t[:i] + body + t[j:]
# annex d
rep("fn annex_d() -> Diagram2d {", "fn annex_d(document: &En1990Snapshot, focus: Option<&str>) -> Diagram2d {")
rep("""        markers: Vec::new(),
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Number of results n", "Anzahl der Ergebnisse n", "1", KN_NODES.to_vec())""", """        markers: document.test_series.iter().filter(|s| focus.is_none_or(|f| f == s.id || f == document.project_id)).filter_map(|s| test_marker(s)).collect(),
        annotations: Vec::new(),
        focus: None,
        legend: true,
        table_twin: DataTable { columns: vec![column("Number of results n", "Anzahl der Ergebnisse n", "1", KN_NODES.to_vec())""")
rep("fn annex_d(document", """/// 🧪️ The marker of one test series on the k_n curves, tinted by its Annex D check.
fn test_marker(series: &TestSeries) -> Option<Marker> {
    let kn = k_n(f64::from(series.sample_size), series.variation_known > 0.0)?;
    let design = test_design_value(&series.distribution, series.sample_size, series.mean, series.std_dev, series.variation_known, series.conversion_factor).ok()?;
    let tone = if design >= series.required { Tone::Info } else { Tone::Error };
    Some(Marker {
        id: format!("test:{}", series.id),
        at: p(f64::from(series.sample_size).clamp(3.0, 100.0), kn.clamp(1.5, 3.5)),
        shape: MarkerShape::Circle,
        tone,
        label: loc(&format!("{}: n = {}, k_n = {kn:.2}, X_d = {design:.3} vs {:.3}", series.id, series.sample_size, series.required), &format!("{}: n = {}, k_n = {kn:.2}, X_d = {design:.3} gegenüber {:.3}", series.id, series.sample_size, series.required)),
        utilisation: None,
        bound_check: Some(format!("en1990.annex-d.{}", series.id)),
        binding: None,
    })
}

fn annex_d(document""")
# fatigue
rep("fn fatigue(report: &CheckReport, member: &Member) -> Option<Diagram2d> {\n    let check = report.checks.iter().find(|c| c.id == format!(\"en1990.fat.{}\", member.id) && c.status != super::norm::CheckStatus::NotApplicable)?;", """fn fatigue(document: &En1990Snapshot, report: &CheckReport, member: &Member) -> Option<Diagram2d> {
    let bins: Vec<_> = document.fatigue_spectra.iter().filter(|b| b.member_id == member.id && b.detail_category > 0.0 && b.stress_range >= 0.0 && b.cycles >= 0.0).collect();
    let live = |id: String| report.checks.iter().find(|c| c.id == id && c.status != super::norm::CheckStatus::NotApplicable);
    let range_check = live(format!("en1990.fat.{}", member.id));
    let damage_check = live(format!("en1990.fat.damage.{}", member.id));
    let check = range_check.or(damage_check)?;""")
rep("""    let rho = check.utilization.max(0.0);
    let cycles = endurance_cycles(rho, 1.0);""", """    let rho = if bins.is_empty() { check.utilization.max(0.0) } else { 0.0 };
    let cycles = if bins.is_empty() { endurance_cycles(rho, 1.0) } else { None };""")
rep("""    let marker = Marker {
        id: "range".into(),""", """    let total = spectrum_damage(&bins.iter().map(|b| (b.stress_range, b.cycles, b.detail_category)).collect::<Vec<_>>()).unwrap_or(f64::NAN);
    let mut markers: Vec<Marker> = bins
        .iter()
        .map(|b| {
            let ratio = (b.stress_range / b.detail_category).clamp(0.2, 3.0);
            let endurance = endurance_cycles(b.stress_range, b.detail_category);
            let share = endurance.map_or(0.0, |n| b.cycles / n);
            Marker {
                id: format!("bin:{}", b.id),
                at: p(endurance.unwrap_or(1.0e9).clamp(1.0e4, 1.0e9), ratio),
                shape: MarkerShape::Square,
                tone: utilisation_tone(share),
                label: loc(&format!("{}: Δσ = {:.1} MPa, n = {:.2e}, d = {share:.4} (D = {total:.4})", b.id, b.stress_range * 1e-6, b.cycles), &format!("{}: Δσ = {:.1} MPa, n = {:.2e}, d = {share:.4} (D = {total:.4})", b.id, b.stress_range * 1e-6, b.cycles)),
                utilisation: Some(share),
                bound_check: Some(format!("en1990.fat.damage.{}", member.id)),
                binding: None,
            }
        })
        .collect();
    let marker = Marker {
        id: "range".into(),""")
rep("""        markers: vec![marker],
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Range ratio Δ/Δ_C\"""", """        markers: {
            if bins.is_empty() {
                markers.push(marker);
            }
            markers
        },
        annotations: Vec::new(),
        focus: Some(EntityRef::new(EntityKind::Member, &member.id, format!("members[id={}]", member.id))),
        legend: true,
        table_twin: DataTable { columns: vec![column("Range ratio Δ/Δ_C\"""")
rep("""        || document.seismics.iter().any(|a| a.id == id)
}""", """        || document.seismics.iter().any(|a| a.id == id)
        || document.fatigue_spectra.iter().any(|a| a.id == id)
        || document.reliability_analyses.iter().any(|a| a.id == id)
        || document.test_series.iter().any(|a| a.id == id)
}""")
rep("    out.push(annex_d());\n    for member in document.members.iter().filter(|m| m.rd_fat > 0.0 && focus.is_none_or(|f| f == m.id || f == document.project_id)) {\n        out.extend(fatigue(report, member));\n    }", "    out.extend(analysis_forms(document, focus));\n    out.push(annex_d(document, focus));\n    for member in document.members.iter().filter(|m| (m.rd_fat > 0.0 || document.fatigue_spectra.iter().any(|b| b.member_id == m.id)) && focus.is_none_or(|f| f == m.id || f == document.project_id)) {\n        out.extend(fatigue(document, report, member));\n    }")
open(p, "w", encoding="utf8").write(t)
print("ok")
