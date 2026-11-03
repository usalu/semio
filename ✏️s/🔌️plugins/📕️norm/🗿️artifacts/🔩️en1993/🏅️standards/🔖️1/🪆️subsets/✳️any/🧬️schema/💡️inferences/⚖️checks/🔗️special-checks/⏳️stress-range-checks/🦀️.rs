//! ⏳️ Fatigue stress ranges from the fatigue load model: the stresses of the highest and the lowest load case at the detail fibre, the range with the reduction of the compressive part, the stress concentration, the size effect and the damage of the range over its cycles (EN 1993-1-9 5.1 to 5.4 and 7.1). See [EN 1993-1-9](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::{bridge_fatigue, fatigue_fire, stress_range as sr};
use semio_s_artifact_norm_en1993_model::{FatigueDetail, FatigueStress, StressRelief};

const CATEGORIES: [u8; 14] = [36, 40, 45, 50, 56, 63, 71, 80, 90, 100, 112, 125, 140, 160];

/// ⏳️ Derived values of one stress range row (Pa); `strength` is the detail category with the size effect before `γ_Mf`.
#[derive(Clone, Debug, PartialEq)]
pub struct Range {
    pub sigma_max: f64,
    pub sigma_min: f64,
    pub raw: f64,
    pub design: f64,
    pub strength: f64,
    pub concentration: f64,
    pub size_effect: f64,
    pub gamma_ff: f64,
    pub gamma_mf: f64,
    pub damage: f64,
}

fn actions(document: &En1993Snapshot, member_id: &str, case_id: &str) -> Option<(f64, f64, f64)> {
    let rows: Vec<_> = document.member_actions.iter().filter(|a| a.member_id == member_id && a.load_case_id == case_id).collect();
    (!rows.is_empty()).then(|| rows.iter().fold((0.0, 0.0, 0.0), |s, a| (s.0 + a.action.n, s.1 + a.action.my, s.2 + a.action.mz)))
}

fn gamma_mf(detail: &FatigueDetail) -> f64 {
    part_1_9::gamma_mf(part_1_9::parse_method(detail.method.as_str()))
}

/// ⏳️ The range of `s` against its detail, `None` while the detail, the member, the section or one of the two load cases is missing.
pub fn derive(document: &En1993Snapshot, s: &FatigueStress, annex: AnnexChoice) -> Option<Range> {
    let detail = document.fatigue_details.iter().find(|d| d.id == s.detail_id)?;
    let member = document.members.iter().find(|m| m.id == detail.member_id)?;
    let section = find_section(document, &member.section_id)?;
    let stress = |case: &str| actions(document, &member.id, case).map(|(n, my, mz)| sr::fibre_stress(n, my, mz, section.area, section.iy, section.iz, s.fibre_z, s.fibre_y));
    let (high, low) = (stress(&s.max_case_id)?, stress(&s.min_case_id)?);
    let raw = sr::range(high, low, s.relief != StressRelief::AsWelded);
    let concentration = if s.concentration > 0.0 { s.concentration } else { 1.0 };
    let gamma_ff = semio_s_plugin_norm_compliance::ndp_number_opt("ndp.en1993.1-9.gamma-ff", annex)?;
    let size_effect = if s.thickness > 0.0 { bridge_fatigue::size_effect(s.thickness * 1e3) } else { 1.0 };
    let (design, strength, gamma_mf) = (gamma_ff * concentration * raw, part_1_9::detail_category_pa(detail.category) * size_effect, gamma_mf(detail));
    let damage = fatigue_fire::miner_damage(&[(design / 1e6, s.cycles)], strength / 1e6, gamma_mf, 1.0);
    Some(Range { sigma_max: high.max(low), sigma_min: high.min(low), raw, design, strength, concentration, size_effect, gamma_ff, gamma_mf, damage })
}

fn subject(s: &FatigueStress, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&s.id, format!("fatigueStresses[id={}].{field}", s.id), loc(en, de))
}

/// ⏳️ Appends one check per stress range row: the damage `n / N_R` of the derived range over its cycles, with a better detail category and fewer cycles as remedies.
pub fn check_stress_ranges(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for s in document.fatigue_stresses.iter().filter(|s| scope.admits("fatigueStresses", &s.id)) {
        let Some(r) = derive(document, s, annex) else { continue };
        let Some(detail) = document.fatigue_details.iter().find(|d| d.id == s.detail_id) else { continue };
        let relief = match s.relief {
            StressRelief::AsWelded => ("as welded: full range", "geschweißt: volle Schwingbreite"),
            StressRelief::StressRelieved => ("stress relieved: compression counted reduced", "spannungsarm: Druckanteil abgemindert"),
            StressRelief::NonWelded => ("non-welded: compression counted reduced", "nicht geschweißt: Druckanteil abgemindert"),
        };
        let mut builder = CheckResult::assess(format!("en1993.1-9.5.range.{}", s.id), "DIN EN 1993-1-9", ClauseId::new("EN 1993-1-9", "5", "5.4"), subject(s, "cycles", "Number of cycles", "Lastspielzahl"), loc("Fatigue damage of the load model stress range", "Ermüdungsschädigung der Spannungsschwingbreite des Lastmodells"))
            .annex(annex)
            .utilization(dimensionless(r.damage), dimensionless(1.0))
            .explanation(loc(
                &format!("σ_max={:.1} MPa, σ_min={:.1} MPa ({}): Δσ={:.1} MPa, γ_Ff k_f Δσ={:.1} MPa against k_s Δσ_C/γ_Mf={:.1} MPa (category {}, k_s={:.3}, γ_Mf={}); D=n/N_R={:.3} over {:.3e} cycles.", r.sigma_max / 1e6, r.sigma_min / 1e6, relief.0, r.raw / 1e6, r.design / 1e6, r.strength / r.gamma_mf / 1e6, detail.category, r.size_effect, r.gamma_mf, r.damage, s.cycles),
                &format!("σ_max={:.1} MPa, σ_min={:.1} MPa ({}): Δσ={:.1} MPa, γ_Ff k_f Δσ={:.1} MPa gegenüber k_s Δσ_C/γ_Mf={:.1} MPa (Kerbfall {}, k_s={:.3}, γ_Mf={}); D=n/N_R={:.3} bei {:.3e} Lastspielen.", r.sigma_max / 1e6, r.sigma_min / 1e6, relief.1, r.raw / 1e6, r.design / 1e6, r.strength / r.gamma_mf / 1e6, detail.category, r.size_effect, r.gamma_mf, r.damage, s.cycles),
            ))
            .ndps(&[AnnexParams::GAMMA_MF])
            .ndp_reads();
        if r.damage > 1.0 {
            let options: Vec<String> = CATEGORIES.iter().filter(|c| fatigue_fire::miner_damage(&[(r.design / 1e6, s.cycles)], part_1_9::detail_category_pa(**c) * r.size_effect / 1e6, r.gamma_mf, 1.0) <= 1.0).map(|c| c.to_string()).collect();
            if !options.is_empty() {
                builder = builder.remedy(Remedy::one_of(SubjectRef::new(&detail.id, format!("fatigueDetails[id={}].category", detail.id), loc("Detail category", "Kerbfall")), options.clone(), loc(&format!("Improve the detail category to one of: {}.", options.join(", ")), &format!("Kerbfall verbessern auf eine von: {}.", options.join(", ")))));
            }
            if let Some(n_r) = fatigue_fire::endurance(r.design / 1e6, r.strength / r.gamma_mf / 1e6) {
                builder = builder.remedy(Remedy::at_most(subject(s, "cycles", "Number of cycles", "Lastspielzahl"), dimensionless(s.cycles), dimensionless(n_r * 0.999), loc(&format!("Limit the design life to at most {:.3e} cycles.", n_r * 0.999), &format!("Nutzungsdauer auf höchstens {:.3e} Lastspiele begrenzen.", n_r * 0.999))));
            }
        }
        report.push(builder.build());
    }
}
