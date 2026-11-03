//! ©️ EN 1994 compliance report: En1994Snapshot → CheckReport for beams, columns, slabs, fire (EN 1994-1-2) and bridges (EN 1994-2).

use semio_s_artifact_norm_en1994_checks::{case_catalogue, catalogue as cat, part_1_1, part_1_2, part_2, part_column, part_en1990, part_slab, part_studs, AnnexParams};
use semio_s_artifact_norm_en1994_model::{CharacteristicAction, CompositeBeam, CompositeColumn, CompositeSlab, En1994Snapshot, SlabAction, SteelSection, Stud};
use semio_s_plugin_norm_compliance::incremental_evaluation::Scope;
use semio_s_plugin_norm_compliance::{AnnexChoice, CheckBuilder, CheckReport, CheckResult, CheckStatus, ClauseId, LocalizedCopy, Quantity, QuantityKind, Remedy, SubjectRef};

#[path = "🏗️beams/🦀️.rs"]
mod beams;
#[path = "🏛️columns/🦀️.rs"]
mod columns;
#[path = "➕️extensions/🦀️.rs"]
mod extensions;
#[path = "🔥️fire-and-bridge/🦀️.rs"]
mod fire_and_bridge;
#[path = "🧱️slabs/🦀️.rs"]
mod slabs;

pub(crate) fn lc(en: impl Into<String>, de: impl Into<String>) -> LocalizedCopy {
    LocalizedCopy::new(en, de)
}

pub(crate) fn dim(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Dimensionless, v)
}
pub(crate) fn moment(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Moment, v)
}
pub(crate) fn force(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Force, v)
}
pub(crate) fn stress(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Stress, v)
}
pub(crate) fn length(v: f64) -> Quantity {
    Quantity::length_m(v)
}
pub(crate) fn area(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Area, v)
}
pub(crate) fn temperature(v: f64) -> Quantity {
    Quantity::new(QuantityKind::Temperature, v)
}

pub(crate) fn head(annex: AnnexChoice, id: String, base: &str, section: &str, subject: SubjectRef, en: &str, de: &str) -> CheckBuilder {
    let part = match annex {
        AnnexChoice::En => format!("EN {base}"),
        AnnexChoice::De => format!("DIN EN {base}"),
    };
    CheckResult::assess(id, part, ClauseId::new(format!("EN {base}"), format!("§{section}"), section), subject, lc(en, de)).annex(annex).ndps(AnnexParams::ndps_of(base)).ndp_reads()
}

pub(crate) fn first_load_action(actions: &[CharacteristicAction]) -> Option<&CharacteristicAction> {
    actions.iter().find(|a| a.kind == semio_s_artifact_norm_en1994_model::ActionKind::Imposed && a.q_area_pa.abs() > 1e-9).or_else(|| actions.iter().find(|a| a.q_area_pa.abs() > 1e-9))
}

pub(crate) fn load_remedy(owner: &str, prefix: &str, label: LocalizedCopy, actions: &[CharacteristicAction], factor: f64, en: &str, de: &str) -> Option<Remedy> {
    let a = first_load_action(actions)?;
    let cur = a.q_area_pa.abs();
    Some(Remedy::at_most(SubjectRef::new(owner, format!("{prefix}.actions[id={}].qAreaPa", a.id), label), dim(cur), dim(cur * factor.clamp(0.0, 1.0)), lc(en, de)))
}

pub(crate) fn m_to_q(m_nm: f64, span_m: f64, continuous: bool) -> f64 {
    m_nm * if continuous { 16.0 } else { 8.0 } / (span_m * span_m).max(1e-9)
}

fn push_dual_source_action_check(report: &mut CheckReport, annex: AnnexChoice, member_id: &str, path: &str, action: &CharacteristicAction) {
    let fatigue_load = action.kind == semio_s_artifact_norm_en1994_model::ActionKind::Fatigue && (action.q_area_pa.abs() > 1e-12 || action.f_k_n.abs() > 1e-12);
    let dual = action.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue && action.q_area_pa.abs() > 1e-12 && action.f_k_n.abs() > 1e-12;
    let bad = dual || fatigue_load;
    let leaf = if action.f_k_n.abs() > 1e-12 || action.kind == semio_s_artifact_norm_en1994_model::ActionKind::Fatigue { "fKN" } else { "qAreaPa" };
    let subject = SubjectRef::new(member_id, format!("{path}.{leaf}"), lc(format!("Action {}", action.id), format!("Einwirkung {}", action.id)));
    let mut chk = head(annex, format!("en1994.action.single-source.{member_id}.{}", action.id), "1990", "6.1", subject, "Single characteristic load source", "Eine charakteristische Lastquelle")
        .utilization(dim(if bad { 2.0 } else { 0.0 }), dim(1.0))
        .explanation(lc(
            if fatigue_load { format!("Fatigue action {} must not carry qAreaPa/fKN; use Δσ_k/Δτ_k only.", action.id) } else if dual { format!("Action {} sets both qAreaPa and fKN; keep exactly one characteristic source.", action.id) } else { format!("Action {} uses a single characteristic source (area or point force).", action.id) },
            if fatigue_load { format!("Ermüdungseinwirkung {} darf kein qAreaPa/fKN tragen; nur Δσ_k/Δτ_k.", action.id) } else if dual { format!("Einwirkung {} setzt qAreaPa und fKN; genau eine charakteristische Quelle belassen.", action.id) } else { format!("Einwirkung {} nutzt eine charakteristische Quelle (Fläche oder Einzellast).", action.id) },
        ));
    if bad {
        chk = chk.remedy(Remedy::at_most(SubjectRef::new(member_id, format!("{path}.fKN"), lc(format!("Action {}", action.id), format!("Einwirkung {}", action.id))), force(action.f_k_n), force(0.0), lc("Clear the conflicting force field.", "Konfliktierende Kraftgröße löschen.")));
    }
    report.push(chk.build());
}

/// ©️ Complete EN 1994 assessment of one snapshot: every member check, then fire and bridge checks.
pub fn evaluate(document: &En1994Snapshot) -> CheckReport {
    evaluate_scoped(document, Scope::All)
}

/// 🎯️ The evaluation restricted to `scope`: the parts `empty`, `fire-rating`, `bridge-kind` (verdicts) or one row of `beams`, `columns`, `slabs`, `cases`, `fireBeams`, `fireSlabs`, `fireColumns`, `bridgeBeams`; the ordered concatenation of the units in [`En1994Units`] is [`evaluate`].
pub fn evaluate_scoped(document: &En1994Snapshot, scope: Scope<'_>) -> CheckReport {
    let mut report = CheckReport::default();
    let annex = document.annex;
    if document.beams.is_empty() && document.columns.is_empty() && document.slabs.is_empty() {
        if scope.part("empty") {
            report.push(head(annex, "en1994.subject.empty".into(), "1994-1-1", "1", SubjectRef::whole(lc("Composite structure", "Verbundtragwerk")), "Subject presence", "Gegenstand vorhanden").not_applicable(lc("No beams, columns or slabs to assess.", "Keine Träger, Stützen oder Decken zur Bewertung.")).build());
        }
        report.extend(case_catalogue::evaluate_design_cases(annex, &document.cases, scope).checks);
        return report;
    }
    for (_, beam) in document.beams.iter().enumerate().filter(|(index, _)| scope.admits("beams", *index)) {
        for action in &beam.actions {
            push_dual_source_action_check(&mut report, annex, &beam.id, &format!("beams[id={}].actions[id={}]", beam.id, action.id), action);
        }
        beams::evaluate_beam(&mut report, document, beam);
        extensions::beam_extensions(&mut report, document, beam);
    }
    for (_, col) in document.columns.iter().enumerate().filter(|(index, _)| scope.admits("columns", *index)) {
        columns::evaluate_column(&mut report, document, col);
        extensions::column_extensions(&mut report, document, col);
    }
    for (_, slab) in document.slabs.iter().enumerate().filter(|(index, _)| scope.admits("slabs", *index)) {
        for action in &slab.actions {
            push_dual_source_action_check(&mut report, annex, &slab.id, &format!("slabs[id={}].actions[id={}]", slab.id, action.id), &CharacteristicAction::from(action));
        }
        slabs::evaluate_slab(&mut report, document, slab);
    }
    fire_and_bridge::evaluate_fire(&mut report, document, scope);
    fire_and_bridge::evaluate_bridge(&mut report, document, scope);
    report.extend(case_catalogue::evaluate_design_cases(annex, &document.cases, scope).checks);
    report
}
