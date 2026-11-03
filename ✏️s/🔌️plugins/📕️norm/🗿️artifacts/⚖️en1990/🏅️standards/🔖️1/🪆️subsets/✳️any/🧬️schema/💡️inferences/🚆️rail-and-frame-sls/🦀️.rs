//! 🚆️ Computed serviceability checks of Annex A2.4.4 (rail bridges by line speed: track twist, vertical deck acceleration, end rotation) and Annex A1.4.3 (horizontal frame and storey drift). All limits come from the NDP registry and are unverified guidance. See [`super`].

use super::*;

//#region 🔖️ComplianceReport
struct Limit<'a> {
    id: String,
    clause: &'static str,
    path: String,
    title: (&'static str, &'static str),
    value: f64,
    limit: f64,
    kind: QuantityKind,
    unit: &'static str,
    detail: (String, String),
    document: &'a En1990Snapshot,
    label: LocalizedCopy,
    owner: String,
}

impl Limit<'_> {
    fn push(self, report: &mut CheckReport) {
        let subject = SubjectRef::new(&self.owner, self.path, self.label);
        let quantity = |value: f64| Quantity::new(self.kind, value);
        let mut builder = CheckResult::assess(self.id, "EN 1990 Annex A", ClauseId::new("EN 1990", self.clause, self.clause), subject.clone(), copy(self.title.0, self.title.1))
            .utilization(quantity(self.value), quantity(self.limit))
            .annex(self.document.annex)
            .explanation(copy(format!("{:.4}{unit} vs limit {:.4}{unit}; {}", self.value, self.limit, self.detail.0, unit = self.unit), format!("{:.4}{unit} gegenüber Grenze {:.4}{unit}; {}", self.value, self.limit, self.detail.1, unit = self.unit)));
        if self.value > self.limit {
            builder = builder.remedy(Remedy::at_most(subject, quantity(self.value), quantity(self.limit), copy(format!("Reduce the value to at most {:.4}{}.", self.limit, self.unit), format!("Den Wert auf höchstens {:.4}{} verringern.", self.limit, self.unit))));
        }
        report.push(builder.ndp_reads().build());
    }
}

/// 🚆️ Annex A2.4.4 for a rail bridge with a declared line speed: track twist t = θ·s per 3 m against the speed-banded limit, vertical deck acceleration against the ballasted or slab-track limit (verified up to the margin times the line speed, dynamic analysis above the NDP speed) and the deck end rotation against the speed-banded limit.
pub(super) fn push_rail_sls(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if document.structure_kind != semio_s_artifact_norm_en1990_model::StructureKind::RailBridge {
        return;
    }
    let Some(sls) = document.bridge_sls.iter().find(|row| row.member_id == member.id) else { return };
    let speed = sls.line_speed;
    if speed <= 0.0 {
        return;
    }
    let annex = document.annex;
    let label = member_label(member);
    let limit = |key: &str, clause: &'static str, field: &str, title: (&'static str, &'static str), value: f64, bound: f64, kind: QuantityKind, unit: &'static str, detail: (String, String)| Limit {
        id: format!("en1990.a2.rail-{key}.{}.{}", member.id, sls.id),
        clause,
        path: format!("bridgeSls[id={}].{field}", sls.id),
        title,
        value,
        limit: bound,
        kind,
        unit,
        detail,
        document,
        label: label.clone(),
        owner: sls.id.clone(),
    };
    if sls.track_gauge > 0.0 && sls.deck_twist > 0.0 {
        let band = if speed <= ndp(annex, "ndp.en1990.a2.rail.twist_speed_1") {
            "twist_120"
        } else if speed <= ndp(annex, "ndp.en1990.a2.rail.twist_speed_2") {
            "twist_200"
        } else {
            "twist_max"
        };
        let bound = ndp(annex, &format!("ndp.en1990.a2.rail.{band}"));
        let base = ndp(annex, "ndp.en1990.a2.rail.twist_length");
        let twist = sls.deck_twist * sls.track_gauge * 1000.0;
        limit(
            "twist",
            "A2.4.4.2.2",
            "deckTwist",
            ("Track twist by line speed", "Gleisverwindung nach Streckengeschwindigkeit"),
            twist,
            bound,
            QuantityKind::Dimensionless,
            " mm",
            (format!("t = θ·s = {:.5} rad · {:.3} m per {base:.0} m at {speed:.0} km/h (band {band})", sls.deck_twist, sls.track_gauge), format!("t = θ·s = {:.5} rad · {:.3} m je {base:.0} m bei {speed:.0} km/h (Stufe {band})", sls.deck_twist, sls.track_gauge)),
        )
        .push(report);
    }
    let margin = ndp(annex, "ndp.en1990.a2.rail.speed_margin");
    let (dynamic, reason) = rail_dynamics::dynamic_need(annex, member.span, sls.natural_frequency, speed);
    let modal_mass = sls.deck_mass * member.span / 2.0;
    let dynamics = (
        format!("verified up to {:.0} km/h{}; modal mass {:.0} kg at {:.2} Hz", speed * margin, if dynamic { ", dynamic analysis required" } else { "" }, modal_mass, sls.natural_frequency),
        format!("nachgewiesen bis {:.0} km/h{}; modale Masse {:.0} kg bei {:.2} Hz", speed * margin, if dynamic { ", dynamische Berechnung erforderlich" } else { "" }, modal_mass, sls.natural_frequency),
    );
    let bound = ndp(annex, if sls.ballast_depth > 0.0 { "ndp.en1990.a2.rail.acc_ballast" } else { "ndp.en1990.a2.rail.acc_slab" });
    if sls.deck_acceleration > 0.0 {
        limit("acceleration", "A2.4.4.2.1", "deckAcceleration", ("Vertical deck acceleration by track kind", "Vertikale Deckbeschleunigung nach Gleisart"), sls.deck_acceleration, bound, QuantityKind::Acceleration, " m/s²", dynamics).push(report);
    }
    push_dynamic_need(report, document, member, sls, dynamic, reason);
    if dynamic {
        push_resonance(report, document, member, sls, bound);
    }
    if sls.end_rotation > 0.0 {
        let low = speed <= ndp(annex, "ndp.en1990.a2.rail.end_rotation_speed");
        let bound = ndp(annex, if low { "ndp.en1990.a2.rail.end_rotation_low" } else { "ndp.en1990.a2.rail.end_rotation_high" });
        let band = (format!("{speed:.0} km/h, {} band", if low { "low-speed" } else { "high-speed" }), format!("{speed:.0} km/h, Stufe {}", if low { "niedrige Geschwindigkeit" } else { "hohe Geschwindigkeit" }));
        limit("end-rotation", "A2.4.4.2.3", "endRotation", ("Deck end rotation by line speed", "Endverdrehung des Decks nach Streckengeschwindigkeit"), sls.end_rotation, bound, QuantityKind::Dimensionless, " rad", band).push(report);
    }
}

/// 🏢️ Annex A1.4.3 horizontal displacements of a building: frame drift u against H/n and storey drift u_i against h/n of the NDP registry.
pub(super) fn push_frame_drift(report: &mut CheckReport, document: &En1990Snapshot, member: &Member) {
    if document.structure_kind.is_bridge() {
        return;
    }
    let annex = document.annex;
    for (key, field, height, drift, id, title) in [
        ("frame-drift", "frameDrift", member.frame_height, member.frame_drift, "ndp.en1990.sls.frame_drift.ratio", ("Horizontal frame drift u ≤ H/n", "Horizontale Rahmenverschiebung u ≤ H/n")),
        ("storey-drift", "storeyDrift", member.storey_height, member.storey_drift, "ndp.en1990.sls.storey_drift.ratio", ("Horizontal storey drift u_i ≤ h/n", "Horizontale Geschossverschiebung u_i ≤ h/n")),
    ] {
        if height <= 0.0 || drift <= 0.0 {
            continue;
        }
        let ratio = ndp(annex, id);
        Limit {
            id: format!("en1990.a14.{key}.{}", member.id),
            clause: "A1.4.3",
            path: member_path(&member.id, field),
            title,
            value: drift,
            limit: height / ratio,
            kind: QuantityKind::Length,
            unit: " m",
            detail: (format!("height {height:.3} m with n = {ratio:.0} (unverified guidance)"), format!("Höhe {height:.3} m mit n = {ratio:.0} (ungeprüfte Orientierung)")),
            document,
            label: member_label(member),
            owner: member.id.clone(),
        }
        .push(report);
    }
}
//#endregion 🔖️ComplianceReport

fn reason_text(reason: rail_dynamics::DynamicReason, speed: f64, f0: f64, span: f64, limits: Option<(f64, f64)>) -> (String, String) {
    use rail_dynamics::DynamicReason::*;
    let band = limits.map_or_else(|| "-".to_string(), |(lower, upper)| format!("{lower:.2} to {upper:.2} Hz"));
    match reason {
        FastLine => (format!("line speed {speed:.0} km/h exceeds the dynamic-analysis speed"), format!("Streckengeschwindigkeit {speed:.0} km/h überschreitet die Grenze für dynamische Berechnungen")),
        BelowLowerLimit => (format!("n0 = {f0:.2} Hz is below the Fig. 6.10 lower limit (band {band}, span {span:.1} m)"), format!("n0 = {f0:.2} Hz liegt unter der unteren Grenze nach Bild 6.10 (Bereich {band}, Stützweite {span:.1} m)")),
        AboveUpperLimit => (format!("n0 = {f0:.2} Hz is above the Fig. 6.10 upper limit (band {band}, span {span:.1} m)"), format!("n0 = {f0:.2} Hz liegt über der oberen Grenze nach Bild 6.10 (Bereich {band}, Stützweite {span:.1} m)")),
        SpanOutOfRange => (format!("span {span:.1} m is outside the Fig. 6.10 range"), format!("Stützweite {span:.1} m liegt außerhalb des Bereichs von Bild 6.10")),
        NotRequired => (format!("n0 = {f0:.2} Hz lies in the Fig. 6.10 band {band}; line speed {speed:.0} km/h"), format!("n0 = {f0:.2} Hz liegt im Bereich {band} nach Bild 6.10; Streckengeschwindigkeit {speed:.0} km/h")),
    }
}

/// 🚦️ The dynamic-analysis requirement flag of A2.4.4.2.1 from the line speed and the Fig. 6.10 natural-frequency band (unverified formulas): a warning with the missing input as remedy when the deck is declared without natural frequency, a passing information otherwise.
fn push_dynamic_need(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, sls: &BridgeSls, dynamic: bool, reason: rail_dynamics::DynamicReason) {
    let annex = document.annex;
    let subject = SubjectRef::new(&sls.id, format!("bridgeSls[id={}].naturalFrequency", sls.id), member_label(member));
    let limits = rail_dynamics::frequency_limits(annex, member.span);
    let (en, de) = reason_text(reason, sls.line_speed, sls.natural_frequency, member.span, limits);
    let mut builder = CheckResult::assess(format!("en1990.a2.rail-dynamic.{}.{}", member.id, sls.id), "EN 1990 Annex A2.4", ClauseId::new("EN 1990", "A2.4.4.2.1", "A2.4.4.2.1"), subject.clone(), copy("Dynamic analysis of the deck required", "Dynamische Berechnung des Decks erforderlich"))
        .annex(annex)
        .explanation(copy(format!("{} - {en}", if dynamic { "dynamic analysis required" } else { "no dynamic analysis required" }), format!("{} - {de}", if dynamic { "dynamische Berechnung erforderlich" } else { "keine dynamische Berechnung erforderlich" })));
    builder = if sls.natural_frequency <= 0.0 && dynamic {
        builder.informative(Quantity::new(QuantityKind::Dimensionless, 1.0)).status(CheckStatus::Warning).remedy(Remedy::at_least(subject, Quantity::new(QuantityKind::Dimensionless, 0.0), Quantity::new(QuantityKind::Dimensionless, limits.map_or(1.0, |(lower, _)| lower)), copy("Declare the natural frequency n0 of the deck (Hz) so the resonance can be estimated.", "Die Eigenfrequenz n0 des Decks (Hz) angeben, damit die Resonanz abgeschätzt werden kann.")))
    } else {
        builder.informative(Quantity::new(QuantityKind::Dimensionless, f64::from(u8::from(dynamic))))
    };
    for id in ["ndp.en1990.a2.rail.dynamic_speed", "ndp.en1990.a2.rail.f_range_min", "ndp.en1990.a2.rail.f_range_max", "ndp.en1990.a2.rail.f_upper_a", "ndp.en1990.a2.rail.f_upper_b", "ndp.en1990.a2.rail.f_lower_a", "ndp.en1990.a2.rail.f_lower_b", "ndp.en1990.a2.rail.f_lower_short", "ndp.en1990.a2.rail.f_lower_break"] {
        builder = builder.ndp(id);
    }
    report.push(builder.build());
}

/// 🌊️ The resonance estimate: the largest mid-span acceleration under the HSLM-like spectrum at the resonance speeds and at 1.2 times the line speed (kernel modes, modal superposition, unverified train data) against the deck acceleration limit; the remedy raises the damping ratio in proportion to the resonant response.
fn push_resonance(report: &mut CheckReport, document: &En1990Snapshot, member: &Member, sls: &BridgeSls, bound: f64) {
    let annex = document.annex;
    if sls.natural_frequency <= 0.0 || sls.deck_mass <= 0.0 || member.span <= 0.0 {
        return;
    }
    let Some(modes) = rail_dynamics::DeckModes::compute(member.span, sls.deck_mass, sls.natural_frequency) else { return };
    let zeta = if sls.damping_ratio > 0.0 { sls.damping_ratio } else { ndp(annex, "ndp.en1990.a2.rail.damping_default") };
    let verification = sls.line_speed * ndp(annex, "ndp.en1990.a2.rail.speed_margin") / 3.6;
    let Some(governing) = rail_dynamics::governing_acceleration(&modes, &rail_dynamics::Train::spectrum(annex), verification, zeta) else { return };
    let acceleration = |value: f64| Quantity::new(QuantityKind::Acceleration, value);
    let subject = SubjectRef::new(&sls.id, format!("bridgeSls[id={}].dampingRatio", sls.id), member_label(member));
    let mut builder = CheckResult::assess(format!("en1990.a2.rail-resonance.{}.{}", member.id, sls.id), "EN 1990 Annex A2.4", ClauseId::new("EN 1990", "A2.4.4.2.1", "A2.4.4.2.1"), subject.clone(), copy("Resonance: computed deck acceleration", "Resonanz: berechnete Deckbeschleunigung"))
        .utilization(acceleration(governing.acceleration), acceleration(bound))
        .annex(annex)
        .explanation(copy(
            format!("{:.3} m/s² under {} at {:.0} km/h (zeta {zeta:.4}, f1 {:.2} Hz, {} kernel modes, HSLM-like train data unverified) against {bound:.2} m/s²", governing.acceleration, governing.train, governing.speed * 3.6, modes.frequencies[0], modes.frequencies.len()),
            format!("{:.3} m/s² unter {} bei {:.0} km/h (Zeta {zeta:.4}, f1 {:.2} Hz, {} Kernmodi, HSLM-ähnliche Zugdaten ungeprüft) gegenüber {bound:.2} m/s²", governing.acceleration, governing.train, governing.speed * 3.6, modes.frequencies[0], modes.frequencies.len()),
        ));
    if governing.acceleration > bound {
        let needed = zeta * governing.acceleration / bound;
        builder = builder.remedy(Remedy::at_least(subject, Quantity::new(QuantityKind::Dimensionless, zeta), Quantity::new(QuantityKind::Dimensionless, needed), copy(format!("Raise the damping ratio to about {needed:.4} (resonant response scales with 1/zeta) or move the natural frequency away from the resonance speeds."), format!("Dämpfungsgrad auf etwa {needed:.4} erhöhen (Resonanzantwort wächst mit 1/Zeta) oder die Eigenfrequenz von den Resonanzgeschwindigkeiten entfernen."))));
    }
    for id in ["ndp.en1990.a2.rail.speed_margin", "ndp.en1990.a2.rail.damping_default"] {
        builder = builder.ndp(id);
    }
    for k in 1..=10 {
        builder = builder.ndp(&format!("ndp.en1990.a2.rail.hslm.a{k}.axle_load"));
    }
    report.push(builder.build());
}
