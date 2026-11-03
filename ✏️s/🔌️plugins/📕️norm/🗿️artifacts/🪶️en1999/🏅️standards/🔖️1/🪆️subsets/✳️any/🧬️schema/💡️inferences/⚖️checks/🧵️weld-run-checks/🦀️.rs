//! 🧵️ EN 1999 weld runs: the reference and HAZ checks of every run and the member the runs imply (element weld flags and the localized-weld distance x_s of §6.3.3.3).

use super::*;
use semio_s_artifact_norm_en1999_model::WeldRun;
use semio_s_plugin_norm_compliance::CheckStatus;

//#region 🔖️ComplianceHelpers
/// 🧵️ The runs that lie on one member.
pub fn runs_of<'a>(runs: &'a [WeldRun], member_id: &str) -> Vec<&'a WeldRun> {
    runs.iter().filter(|r| r.member_id == member_id).collect()
}

/// 🧵️ HAZ extent b_haz per side of one run [m]: the override when positive, else the §6.1.6.3 step value of the plate thickness times 1.5 for TIG and 1.33 in a T-junction.
pub fn haz_extent(run: &WeldRun, thickness: f64) -> f64 {
    if run.haz_extent > 0.0 { return run.haz_extent; }
    let process = if run.process == semio_s_artifact_norm_en1999_model::WeldProcess::Tig { 1.5 } else { 1.0 };
    let junction = if run.t_junction { 1.33 } else { 1.0 };
    part_1_1::haz_width(thickness) * process * junction
}

/// 🧵️ The b_haz overrides [m] the runs declare per plate element (the widest positive `hazExtent` of the runs on that element) for the section strips of the member checks.
pub fn haz_overrides(runs: &[&WeldRun]) -> Vec<(String, f64)> {
    let mut out: Vec<(String, f64)> = Vec::new();
    for run in runs.iter().filter(|r| r.haz_extent > 0.0 && r.haz_extent.is_finite()) {
        match out.iter_mut().find(|(id, _)| *id == run.element_id) {
            Some(entry) => entry.1 = entry.1.max(run.haz_extent),
            None => out.push((run.element_id.clone(), run.haz_extent)),
        }
    }
    out
}

/// 🧵️ The member and section the runs imply: every run overrides the weld flags of its plate element (transverse runs last), the nearest transverse run sets the localized weld of the member at x_s from the nearer end.
pub fn resolved(member: &AluminiumMember, section: &AluminiumSection, runs: &[&WeldRun]) -> (AluminiumMember, AluminiumSection) {
    let mut m = member.clone();
    let mut s = section.clone();
    let mut ordered: Vec<&WeldRun> = runs.to_vec();
    ordered.sort_by_key(|r| r.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse);
    for run in ordered {
        if let Some(el) = s.elements.iter_mut().find(|e| e.id == run.element_id) {
            el.welded = true;
            el.weld_position = run.offset.max(0.0);
            el.weld_process = run.process.clone();
            el.weld_t_junction = run.t_junction;
            el.weld_direction = run.direction.clone();
        }
    }
    let xs = runs.iter().filter(|r| r.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse).map(|r| r.start.min(member.length - r.start).max(0.0)).fold(f64::INFINITY, f64::min);
    if xs.is_finite() {
        m.transverse_weld = true;
        m.transverse_weld_offset = xs;
    }
    (m, s)
}

fn refusal(doc: &En1999Snapshot, run: &WeldRun, suffix: &str, field: &str, title: (&str, &str), en: String, de: String, remedy: Remedy) -> CheckResult {
    let subject = SubjectRef::new(&run.id, format!("weldRuns[id={}].{field}", run.id), loc("Weld run", "Schweißnaht"));
    CheckResult::assess(format!("en1999.ref.weldrun.{suffix}.{}", run.id), "DIN EN 1999-1-1", clause("6.1.6"), subject, loc(title.0, title.1))
        .status(CheckStatus::Fail)
        .explanation(loc(&en, &de))
        .annex(doc.annex)
        .remedy(remedy)
        .build()
}

/// 🧵️ Reference, geometry and HAZ checks of one weld run (EN 1999-1-1 §6.1.6, §8.6).
pub fn check_run(doc: &En1999Snapshot, run: &WeldRun) -> Vec<CheckResult> {
    let mut out = Vec::new();
    let subject = |field: &str| SubjectRef::new(&run.id, format!("weldRuns[id={}].{field}", run.id), loc("Weld run", "Schweißnaht"));
    let member = doc.members.iter().find(|m| m.id == run.member_id);
    let section = member.and_then(|m| doc.sections.iter().find(|s| s.id == m.section_id));
    let element = section.and_then(|s| s.elements.iter().find(|e| e.id == run.element_id));
    if member.is_none() {
        out.push(refusal(doc, run, "member", "memberId", ("Welded member", "Geschweißtes Bauteil"),
            format!("Weld run '{}' names the unknown member '{}'.", run.id, run.member_id), format!("Schweißnaht '{}' nennt das unbekannte Bauteil '{}'.", run.id, run.member_id),
            Remedy::one_of(subject("memberId"), doc.members.iter().map(|m| m.id.clone()).collect(), loc("Point the run at an existing member.", "Naht auf ein vorhandenes Bauteil richten."))));
    }
    if let Some(sec) = section {
        if element.is_none() && (!run.element_id.is_empty() || run.direction != semio_s_artifact_norm_en1999_model::WeldDirection::Transverse) {
            out.push(refusal(doc, run, "element", "elementId", ("Welded plate element", "Geschweißtes Blechelement"),
                format!("Weld run '{}' names '{}', which is no plate element of section '{}'.", run.id, run.element_id, sec.id), format!("Schweißnaht '{}' nennt '{}', das kein Blechelement des Querschnitts '{}' ist.", run.id, run.element_id, sec.id),
                Remedy::one_of(subject("elementId"), sec.elements.iter().map(|e| e.id.clone()).collect(), loc("Pick a plate element of the section.", "Blechelement des Querschnitts wählen."))));
        }
    }
    let finite = [run.start, run.length, run.offset, run.throat, run.haz_extent].iter().all(|v| v.is_finite() && *v >= 0.0);
    let length = member.map_or(f64::INFINITY, |m| m.length);
    let span_end = if run.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse { run.start } else { run.start + run.length };
    if !finite || span_end > length + 1e-9 || element.is_some_and(|e| run.offset > e.width + 1e-9) {
        let (field, current, required) = if finite && span_end > length + 1e-9 && run.direction != semio_s_artifact_norm_en1999_model::WeldDirection::Transverse { ("length", run.length, (length - run.start).max(0.0)) } else if finite && span_end > length + 1e-9 { ("start", run.start, length) } else if finite { ("offset", run.offset, element.map_or(0.0, |e| e.width)) } else { ("length", run.length, 0.0) };
        out.push(refusal(doc, run, "geometry", field, ("Weld run geometry", "Nahtgeometrie"),
            format!("Weld run '{}' must lie on its member and plate element with finite non-negative values (start {:.3} m, length {:.3} m, offset {:.4} m).", run.id, run.start, run.length, run.offset),
            format!("Schweißnaht '{}' muss auf Bauteil und Blechelement liegen und endliche, nichtnegative Werte haben (Anfang {:.3} m, Länge {:.3} m, Lage {:.4} m).", run.id, run.start, run.length, run.offset),
            Remedy::at_most(subject(field), q_length(current), q_length(required), loc("Shorten the run or move it back onto the member and element.", "Naht kürzen oder zurück auf Bauteil und Blechelement legen."))));
    }
    if !(run.throat > 0.0) {
        out.push(refusal(doc, run, "throat", "throat", ("Weld throat", "Kehlnahtdicke"),
            format!("Weld run '{}' needs a positive throat thickness.", run.id), format!("Schweißnaht '{}' braucht eine positive Kehlnahtdicke.", run.id),
            Remedy::at_least(subject("throat"), q_length(run.throat), q_length(0.003), loc("Give the weld a throat of at least 3 mm.", "Naht mit einer Kehlnahtdicke von mindestens 3 mm versehen."))));
    }
    if let (Some(el), true) = (element, finite) {
        let b_haz = haz_extent(run, el.thickness);
        let reach = if run.direction == semio_s_artifact_norm_en1999_model::WeldDirection::Transverse { el.width } else { 2.0 * b_haz };
        let mut c = CheckResult::assess(format!("en1999.6.1.6.haz.{}", run.id), "DIN EN 1999-1-1", clause("6.1.6"), subject("hazExtent"), loc("HAZ extent of the weld run (§6.1.6.3)", "WEZ-Ausdehnung der Naht (§6.1.6.3)"))
            .utilization(q_length(reach), q_length(el.width.max(1e-9)))
            .annex(doc.annex)
            .explanation(loc(
                &format!("Run {}: b_haz = {:.1} mm per side ({:?} weld{}), HAZ reach {:.1} mm of the {:.1} mm wide element {}.", run.id, b_haz * 1e3, run.process, if run.t_junction { " in a T-junction" } else { "" }, reach * 1e3, el.width * 1e3, el.id),
                &format!("Naht {}: b_haz = {:.1} mm je Seite ({:?}-Naht{}), WEZ-Reichweite {:.1} mm des {:.1} mm breiten Elements {}.", run.id, b_haz * 1e3, run.process, if run.t_junction { " im T-Stoß" } else { "" }, reach * 1e3, el.width * 1e3, el.id)));
        if reach > el.width {
            if let Some(sec) = section {
                let width = SubjectRef::new(&sec.id, format!("sections[id={}].elements[id={}].width", sec.id, el.id), loc("Plate element", "Blechelement"));
                c = c.remedy(Remedy::at_least(width, q_length(el.width), q_length(reach), loc("Widen the plate element beyond the HAZ reach.", "Blechelement über die WEZ-Reichweite verbreitern.")));
            }
            if run.process == semio_s_artifact_norm_en1999_model::WeldProcess::Tig {
                c = c.remedy(Remedy::one_of(subject("process"), vec!["mig".into()], loc("A MIG weld has the narrower HAZ.", "Eine MIG-Naht hat die schmalere WEZ.")));
            }
            if run.haz_extent > 0.0 {
                c = c.remedy(Remedy::at_most(subject("hazExtent"), q_length(run.haz_extent), q_length(0.49 * el.width), loc("Reduce the HAZ extent below half the element width.", "WEZ-Ausdehnung unter die halbe Elementbreite senken.")));
            }
        }
        out.push(c.build());
    }
    out
}
//#endregion 🔖️ComplianceHelpers
