//! 🎚️ Project overrides of annex factors (γ, ψ₀/ψ₁/ψ₂, ξ, K_FI): which rows the annex allows, how they reach the combination generator and the checks that report every deviation from the National Annex. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
/// 🧭️ Why a factor-override row is (not) applied; only [`Verdict::Applied`] rows reach the generator.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Applied,
    UnknownKind,
    NotAllowed,
    UnknownTarget,
    InvalidValue,
    MissingJustification,
    Superseded,
}

pub(super) const KINDS: [&str; 6] = ["gamma", "psi0", "psi1", "psi2", "xi", "k_fi"];

fn allowance_id(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "gamma" => "ndp.en1990.a1.override.gamma.allowed",
        "psi0" | "psi1" | "psi2" => "ndp.en1990.a1.override.psi.allowed",
        "xi" => "ndp.en1990.a1.override.xi.allowed",
        "k_fi" => "ndp.en1990.a1.override.k_fi.allowed",
        _ => return None,
    })
}

/// 🎯️ The action ids a gamma or psi row reaches: an empty target means every action, otherwise an action id, exclusive group or psi category.
pub fn targets(document: &En1990Snapshot, row: &FactorOverride) -> Vec<String> {
    let target = row.target.trim();
    let mut ids: Vec<String> = Vec::new();
    for permanent in document.permanents.iter().filter(|p| row.factor == "gamma" && p.kind != semio_s_artifact_norm_en1990_model::PermanentKind::GInf && p.kind != semio_s_artifact_norm_en1990_model::PermanentKind::Prestress) {
        if target.is_empty() || permanent.id == target {
            ids.push(permanent.id.clone());
        }
    }
    for variable in &document.variables {
        let category = resolve_psi_category(document.annex, &variable.category, document.altitude_m);
        if target.is_empty() || variable.id == target || variable.group == target || variable.category == target || category == target {
            ids.push(variable.id.clone());
        }
    }
    ids
}

fn value_ok(row: &FactorOverride) -> bool {
    row.value.is_finite()
        && match row.factor.as_str() {
            "gamma" | "k_fi" => row.value > 0.0,
            "xi" => row.value > 0.0 && row.value <= 1.0,
            _ => (0.0..=1.0).contains(&row.value),
        }
}

/// 🧮️ The verdict of every row in document order: the first failing rule wins, a later row for the same factor and target supersedes an earlier one.
pub fn verdicts(document: &En1990Snapshot) -> Vec<Verdict> {
    let mut out: Vec<Verdict> = Vec::with_capacity(document.factor_overrides.len());
    for (index, row) in document.factor_overrides.iter().enumerate() {
        let verdict = match allowance_id(&row.factor) {
            None => Verdict::UnknownKind,
            Some(id) if ndp(document.annex, id) < 0.5 => Verdict::NotAllowed,
            Some(_) if !value_ok(row) => Verdict::InvalidValue,
            Some(_) if row.justification.trim().is_empty() => Verdict::MissingJustification,
            Some(_) if matches!(row.factor.as_str(), "gamma" | "psi0" | "psi1" | "psi2") && !row.target.trim().is_empty() && targets(document, row).is_empty() => Verdict::UnknownTarget,
            Some(_) if document.factor_overrides[index + 1..].iter().any(|later| later.factor == row.factor && later.target.trim() == row.target.trim()) => Verdict::Superseded,
            Some(_) => Verdict::Applied,
        };
        out.push(verdict);
    }
    out
}

/// 🎚️ The γ and ψ overrides of one action id from the applied rows (a later, more specific row wins over an earlier general one).
pub fn action_override(document: &En1990Snapshot, action_id: &str) -> ActionOverride {
    let verdicts = verdicts(document);
    let mut out = ActionOverride::default();
    for (row, verdict) in document.factor_overrides.iter().zip(&verdicts) {
        if *verdict != Verdict::Applied || !matches!(row.factor.as_str(), "gamma" | "psi0" | "psi1" | "psi2") || !targets(document, row).iter().any(|id| id == action_id) {
            continue;
        }
        match row.factor.as_str() {
            "gamma" => out.gamma = Some(row.value),
            "psi0" => out.psi[0] = Some(row.value),
            "psi1" => out.psi[1] = Some(row.value),
            _ => out.psi[2] = Some(row.value),
        }
    }
    out.gamma = out.gamma.filter(|v| !same_as_annex(*v, annex_for_action(document, "gamma", action_id)));
    for (slot, factor) in ["psi0", "psi1", "psi2"].into_iter().enumerate() {
        out.psi[slot] = out.psi[slot].filter(|v| !same_as_annex(*v, annex_for_action(document, factor, action_id)));
    }
    out
}

/// 🎚️ The applied global override of `xi` or `k_fi` (`None` when absent or not applied).
pub fn global_override(document: &En1990Snapshot, factor: &str) -> Option<f64> {
    let verdicts = verdicts(document);
    let annex = annex_global(document, factor);
    document.factor_overrides.iter().zip(verdicts).filter(|(row, verdict)| row.factor == factor && *verdict == Verdict::Applied).map(|(row, _)| row.value).last().filter(|v| !same_as_annex(*v, annex))
}

const SAME: f64 = 1.0e-9;

fn same_as_annex(value: f64, annex: Option<f64>) -> bool {
    annex.is_some_and(|a| (a - value).abs() < SAME)
}

fn annex_global(document: &En1990Snapshot, factor: &str) -> Option<f64> {
    match factor {
        "xi" => Some(xi(document.annex, document.structure_kind.domain())),
        "k_fi" => k_fi_for_rc(document.reliability_class.number()),
        _ => None,
    }
}

/// 📖️ The annex value of `factor` (gamma, psi0, psi1, psi2) for one action id: gamma_G,sup of a permanent action, gamma_Q or the psi table row of the resolved category of a variable one (STR set B).
pub fn annex_for_action(document: &En1990Snapshot, factor: &str, action_id: &str) -> Option<f64> {
    let domain = document.structure_kind.domain();
    let permanent = document.permanents.iter().any(|p| p.id == action_id);
    let variable = document.variables.iter().find(|v| v.id == action_id);
    let category = variable.map(|v| resolve_psi_category(document.annex, &v.category, document.altitude_m));
    match factor {
        "gamma" if permanent => Some(gamma_g(document.annex, domain, FactorSet::StrB).0),
        "gamma" => category.map(|c| gamma_q(document.annex, domain, FactorSet::StrB, &c)),
        "psi0" => category.and_then(|c| psi_lookup(document.annex, &c)).map(|r| r.psi_0),
        "psi1" => category.and_then(|c| psi_lookup(document.annex, &c)).map(|r| r.psi_1),
        "psi2" => category.and_then(|c| psi_lookup(document.annex, &c)).map(|r| r.psi_2),
        _ => None,
    }
}

/// 🎚️ Whether each row, once applied, really differs from the National Annex value of at least one target; a row set back to the annex value (the reset to NA) is applied but does not deviate.
pub fn deviations(document: &En1990Snapshot) -> Vec<bool> {
    verdicts(document)
        .into_iter()
        .zip(&document.factor_overrides)
        .map(|(verdict, row)| {
            verdict == Verdict::Applied
                && match row.factor.as_str() {
                    "xi" | "k_fi" => !same_as_annex(row.value, annex_global(document, &row.factor)),
                    factor => targets(document, row).iter().any(|id| !same_as_annex(row.value, annex_for_action(document, factor, id))),
                }
        })
        .collect()
}

/// 🔗️ The rows (index and verdict) of a gamma or psi factor that reach one action, in document order.
pub fn rows_for(document: &En1990Snapshot, action_id: &str) -> Vec<(usize, Verdict)> {
    verdicts(document)
        .into_iter()
        .enumerate()
        .filter(|(index, _)| {
            let row = &document.factor_overrides[*index];
            matches!(row.factor.as_str(), "gamma" | "psi0" | "psi1" | "psi2") && targets(document, row).iter().any(|id| id == action_id)
        })
        .collect()
}

/// 📖️ The annex value the row replaces, for the deviation text (the first target of a broad row); `None` for a target the annex has no single value for.
pub fn annex_value(document: &En1990Snapshot, row: &FactorOverride) -> Option<f64> {
    match row.factor.as_str() {
        "xi" | "k_fi" => annex_global(document, &row.factor),
        factor => targets(document, row).first().and_then(|id| annex_for_action(document, factor, id)),
    }
}

fn row_subject(_document: &En1990Snapshot, row: &FactorOverride, field: &str) -> SubjectRef {
    let label = copy(format!("Factor override {}", row.id), format!("Faktorüberschreibung {}", row.id));
    SubjectRef::new(&row.id, format!("factorOverrides[id={}].{field}", row.id), label)
}

fn q(value: f64) -> Quantity {
    q_dim(value)
}

/// ✅️ One check per override row: an applied row is a warning that names the deviation from the National Annex (value, annex value, justification); every refused row fails with a remedy that makes it applicable.
pub(super) fn push_factor_overrides(report: &mut CheckReport, document: &En1990Snapshot, index: usize) {
    let row = &document.factor_overrides[index];
    let verdict = verdicts(document)[index];
    let id = format!("en1990.override.{}", row.id);
    let clause = ClauseId::new("EN 1990", "§A1.1", "A1.1");
    let title = copy("Project factor override", "Projektbezogene Faktorüberschreibung");
    let annex_text = annex_value(document, row).map_or_else(|| "-".to_string(), |v| format!("{v:.4}"));
    let mut builder = CheckResult::assess(id, "EN 1990 Annex A1", clause, row_subject(document, row, "value"), title).annex(document.annex);
    let value_subject = row_subject(document, row, "value");
    match verdict {
        Verdict::Applied if !deviations(document)[index] => {
            builder = builder.informative(q(row.value)).explanation(copy(
                format!("{} on {} equals the National Annex value {annex_text}; the row has no effect.", row.factor, if row.target.trim().is_empty() { "all actions" } else { row.target.trim() }),
                format!("{} auf {} entspricht dem Wert des Nationalen Anhangs {annex_text}; die Zeile ist wirkungslos.", row.factor, if row.target.trim().is_empty() { "alle Einwirkungen" } else { row.target.trim() }),
            ));
        }
        Verdict::Applied => {
            builder = builder.informative(q(row.value)).status(CheckStatus::Warning).explanation(copy(
                format!("Deviates from the National Annex: {} on {} is {:.4} instead of {annex_text}. Justification: {}", row.factor, if row.target.trim().is_empty() { "all actions" } else { row.target.trim() }, row.value, row.justification.trim()),
                format!("Weicht vom Nationalen Anhang ab: {} auf {} ist {:.4} statt {annex_text}. Begründung: {}", row.factor, if row.target.trim().is_empty() { "alle Einwirkungen" } else { row.target.trim() }, row.value, row.justification.trim()),
            ));
            if let Some(annex) = annex_value(document, row) {
                builder = builder.remedy(Remedy::exactly(value_subject, q(row.value), q(annex), copy(format!("Reset to the National Annex value {annex:.4} (or keep the override and its justification)."), format!("Auf den Wert des Nationalen Anhangs {annex:.4} zurücksetzen (oder die Überschreibung samt Begründung beibehalten)."))));
            }
        }
        Verdict::Superseded => {
            builder = builder.informative(q(row.value)).status(CheckStatus::Warning).explanation(copy("A later row overrides the same factor on the same target; this row has no effect.", "Eine spätere Zeile überschreibt denselben Faktor am selben Ziel; diese Zeile ist wirkungslos.")).remedy(Remedy::one_of(row_subject(document, row, "factor"), KINDS.iter().map(|k| k.to_string()).collect(), copy("Remove this row or give it a different factor or target.", "Diese Zeile entfernen oder anderen Faktor bzw. anderes Ziel wählen.")));
        }
        Verdict::UnknownKind => {
            builder = builder.informative(q(row.value)).status(CheckStatus::Fail).explanation(copy(format!("Unknown factor kind {:?}.", row.factor), format!("Unbekannte Faktorart {:?}.", row.factor))).remedy(Remedy::one_of(row_subject(document, row, "factor"), KINDS.iter().map(|k| k.to_string()).collect(), copy("Choose gamma, psi0, psi1, psi2, xi or k_fi.", "gamma, psi0, psi1, psi2, xi oder k_fi wählen.")));
        }
        Verdict::NotAllowed => {
            let value = annex_value(document, row).unwrap_or(row.value);
            builder = builder.informative(q(row.value)).status(CheckStatus::Fail).explanation(copy(
                format!("The annex does not allow a project override of {}; the row is ignored.", row.factor),
                format!("Der Anhang erlaubt keine projektbezogene Überschreibung von {}; die Zeile wird ignoriert.", row.factor),
            )).remedy(Remedy::exactly(value_subject, q(row.value), q(value), copy(format!("Remove the row (annex value {value:.4})."), format!("Die Zeile entfernen (Anhangswert {value:.4})."))));
        }
        Verdict::InvalidValue => {
            let (lo, hi) = match row.factor.as_str() {
                "xi" => (f64::MIN_POSITIVE, 1.0),
                "psi0" | "psi1" | "psi2" => (0.0, 1.0),
                _ => (f64::MIN_POSITIVE, f64::MAX),
            };
            let required = if row.value.is_finite() { row.value.clamp(lo, hi) } else { lo.max(0.0) };
            builder = builder.informative(q(row.value)).status(CheckStatus::Fail).explanation(copy(format!("{:.4} is not a valid {}.", row.value, row.factor), format!("{:.4} ist kein gültiger Wert für {}.", row.value, row.factor))).remedy(Remedy::exactly(value_subject, q(row.value), q(required), copy(format!("Set the value to {required:.4} or choose another.", ), format!("Den Wert auf {required:.4} setzen oder einen anderen wählen."))));
        }
        Verdict::MissingJustification => {
            builder = builder.informative(q(row.value)).status(CheckStatus::Fail).explanation(copy("A deviation from the National Annex needs a justification; the row is ignored.", "Eine Abweichung vom Nationalen Anhang braucht eine Begründung; die Zeile wird ignoriert.")).remedy(Remedy::one_of(row_subject(document, row, "justification"), vec!["justification".into()], copy("Enter the justification of the deviation.", "Die Begründung der Abweichung eintragen.")));
        }
        Verdict::UnknownTarget => {
            let options: Vec<String> = document.variables.iter().map(|v| v.id.clone()).chain(document.permanents.iter().map(|p| p.id.clone())).chain(document.variables.iter().map(|v| v.group.clone()).filter(|g| !g.is_empty())).collect();
            builder = builder.informative(q(row.value)).status(CheckStatus::Fail).explanation(copy(format!("Target {:?} matches no action, group or category.", row.target), format!("Ziel {:?} passt auf keine Einwirkung, Gruppe oder Kategorie.", row.target))).remedy(Remedy::one_of(row_subject(document, row, "target"), options, copy("Pick an action id, an exclusive group or a ψ category.", "Eine Einwirkungs-Id, Ausschlussgruppe oder ψ-Kategorie wählen.")));
        }
    }
    if let Some(id) = allowance_id(&row.factor) {
        builder = builder.ndp(id);
    }
    report.push(builder.ndp_reads().build());
}
//#endregion 🔖️ComplianceReport
