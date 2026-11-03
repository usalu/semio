//! 🗄️ Registry of the EN 1992 verification catalogue: every entry id mapped to its evaluator and the conversion of outcomes into check results. See [`super`].

use super::*;

/// 🗄 Every catalogue entry implemented by this artifact.
pub fn registry() -> Vec<&'static Entry> {
    let mut all: Vec<&'static Entry> = Vec::new();
    for group in [
        materials_and_time::ENTRIES,
        section_resistance::ENTRIES,
        section_resistance::GEOMETRY_ENTRIES,
        shear_torsion::ENTRIES,
        punching_and_struts::ENTRIES,
        detailing::ENTRIES,
        serviceability::ENTRIES,
        prestress_and_fatigue::ENTRIES,
        fire_design::ENTRIES,
        bridges_and_liquid::ENTRIES,
        fastenings::ENTRIES,
        analysis_models::ENTRIES,
        annex_parts::ENTRIES,
    ] {
        all.extend(group.iter());
    }
    all
}

/// 🔎 Entry by catalogue id such as `S-03`.
pub fn entry(id: &str) -> Option<&'static Entry> {
    registry().into_iter().find(|e| e.id == id)
}

fn subject(case: &Case) -> SubjectRef {
    SubjectRef::new(&case.id, format!("catalogue[id={}]", case.id), LocalizedCopy::new(case.id.as_str(), case.id.as_str()))
}

/// 🧾 Raw outcomes of one catalogue entry, `Err` with the reason when inputs are missing or the id is unknown.
pub fn evaluate_outcomes(entry_id: &str, case: &Case) -> R {
    (entry(entry_id).ok_or_else(|| format!("unknown catalogue entry {entry_id}"))?.eval)(case)
}

/// 🧮 Evaluate one case against its catalogue entry; missing inputs yield a single not-applicable result and every outcome that rests on an unverified NDP value of the shared registry (`ndp.en1992.*`, read during the evaluation or listed in the entry basis) gets the status `UnverifiedBasis`.
pub fn evaluate_case(entry_id: &str, case: &Case) -> Vec<CheckResult> {
    let Some(e) = entry(entry_id) else {
        return vec![CheckResult::assess(format!("en1992.{entry_id}.unknown"), "DIN EN 1992", ClauseId::new("EN 1992", "§0", "0"), subject(case), LocalizedCopy::new("Unknown catalogue entry", "Unbekannter Katalogeintrag"))
            .annex(case.annex)
            .not_applicable(LocalizedCopy::new(format!("No catalogue entry {entry_id}."), format!("Kein Katalogeintrag {entry_id}.")))
            .build()];
    };
    let base = |key: &str, en: &str, de: &str| {
        CheckResult::assess(format!("en1992.{}.{key}.{}", e.id.to_ascii_lowercase(), case.id), e.part, ClauseId::new("EN 1992", format!("§{}", e.clause), e.clause), subject(case), LocalizedCopy::new(en, de)).annex(case.annex)
    };
    let prefix = format!("ndp.en1992.{}.", e.id.to_ascii_lowercase());
    let (evaluated, read) = record_ndp_reads(|| (e.eval)(case));
    let mut ndp_ids: Vec<String> = NDP_TABLE.iter().filter(|n| n.id.starts_with(&prefix)).map(|n| n.id.to_string()).collect();
    ndp_ids.extend(e.basis.iter().chain(read.iter()).map(|id| id.to_string()));
    let unverified: Vec<&str> = ndp_ids.iter().map(String::as_str).filter(|id| !ndp_is_verified(id)).collect();
    match evaluated {
        Err(reason) => vec![base("input", e.en, e.de).not_applicable(semio_s_plugin_norm_compliance::reason_copy(e.id, &reason)).build()],
        Ok(outcomes) => outcomes
            .into_iter()
            .map(|o| {
                let title_en = format!("{} – {}", e.en, o.en);
                let title_de = format!("{} – {}", e.de, o.de);
                let note_en = if unverified.is_empty() { String::new() } else { format!(" [unverified NDP: {}]", unverified.join(", ")) };
                let note_de = if unverified.is_empty() { String::new() } else { format!(" [ungeprüfter NDP: {}]", unverified.join(", ")) };
                let expl = LocalizedCopy::new(format!("{}{}", o.steps, note_en), format!("{}{}", o.steps_de, note_de));
                let mut b = base(&o.key, &title_en, &title_de).explanation(expl);
                let q = |v: f64| Quantity::new(o.kind, v);
                let util = o.utilization();
                match o.mode {
                    Mode::Info => {
                        b = b.utilization(q(0.0), q(o.capacity.abs().max(1.0)));
                    }
                    Mode::Limit => {
                        b = b.utilization(q(o.demand), q(o.capacity));
                        if util > 1.0 || !util.is_finite() {
                            b = b.status(CheckStatus::Fail).remedy(Remedy::at_most(subject(case), q(o.demand), q(o.capacity), LocalizedCopy::new(format!("Reduce the demand to at most {:.4e} (SI) or raise the resistance of {}.", o.capacity, e.id), format!("Einwirkung auf höchstens {:.4e} (SI) verringern oder Tragfähigkeit von {} erhöhen.", o.capacity, e.id))));
                        }
                    }
                    Mode::Minimum => {
                        b = b.minimum(q(o.demand), q(o.capacity));
                        if o.demand < o.capacity {
                            b = b.remedy(Remedy::at_least(subject(case), q(o.demand), q(o.capacity), LocalizedCopy::new(format!("Provide at least {:.4e} (SI) for {}.", o.capacity, e.id), format!("Mindestens {:.4e} (SI) für {} vorsehen.", o.capacity, e.id))));
                        }
                    }
                }
                for id in &ndp_ids {
                    b = b.ndp(id);
                }
                b.build()
            })
            .collect(),
    }
}

pub use semio_s_artifact_norm_en1992_checks_case_inputs as case_inputs;

#[path = "🗂️cases/🦀️.rs"]
mod cases;
pub use cases::*;
