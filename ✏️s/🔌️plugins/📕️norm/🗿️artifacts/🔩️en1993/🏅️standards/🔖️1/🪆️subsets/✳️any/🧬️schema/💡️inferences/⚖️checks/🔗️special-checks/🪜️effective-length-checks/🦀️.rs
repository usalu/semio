//! 🪜️ Buckling length of columns in frames from the end restraint: the effective length factor of the nomogram for the non-sway and the sway mode against the buckling length entered at the member (EN 1993-1-1 6.3.1.3). See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::effective_length as el;
use semio_s_artifact_norm_en1993_model::{BucklingLength, BucklingPlane, FrameSway, NodeARestraint, NodeBRestraint};

/// 🪜️ Derived values of one row: the distribution factors at both nodes and the effective length factor `K`; `None` while the frame is a mechanism (sway with both ends pinned).
pub fn factors(b: &BucklingLength) -> Option<(f64, f64, f64)> {
    let ga = match b.restraint_a {
        NodeARestraint::Framed => el::distribution(b.column_stiffness_a, b.beam_stiffness_a),
        NodeARestraint::Fixed => 0.0,
        NodeARestraint::Pinned => el::UNBOUNDED,
    };
    let gb = match b.restraint_b {
        NodeBRestraint::Framed => el::distribution(b.column_stiffness_b, b.beam_stiffness_b),
        NodeBRestraint::Fixed => 0.0,
        NodeBRestraint::Pinned => el::UNBOUNDED,
    };
    let k = match b.sway {
        FrameSway::NonSway => Some(el::k_non_sway(ga, gb)),
        FrameSway::Sway => el::k_sway(ga, gb),
    };
    k.map(|k| (ga, gb, k))
}

fn subject(b: &BucklingLength, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(&b.id, format!("bucklingLengths[id={}].{field}", b.id), loc(en, de))
}

fn mode(b: &BucklingLength) -> (&'static str, &'static str) {
    match b.sway {
        FrameSway::NonSway => ("non-sway", "unverschieblich"),
        FrameSway::Sway => ("sway", "verschieblich"),
    }
}

/// 🪜️ Appends one check per row: the buckling length `K L` of the nomogram against the buckling length entered at the member, with the entered length as remedy.
pub fn check_buckling_lengths(document: &En1993Snapshot, annex: AnnexChoice, scope: Scope<'_>, report: &mut CheckReport) {
    for b in document.buckling_lengths.iter().filter(|b| scope.admits("bucklingLengths", &b.id)) {
        let Some(member) = document.members.iter().find(|m| m.id == b.member_id) else { continue };
        if member.length <= 0.0 {
            continue;
        }
        let (leaf, entered) = match b.plane {
            BucklingPlane::MajorAxis => ("bucklingLengthY", member.buckling_length_y),
            BucklingPlane::MinorAxis => ("bucklingLengthZ", member.buckling_length_z),
        };
        let (en, de) = mode(b);
        let base = CheckResult::assess(format!("en1993.1-1.6.3.1.3.lcr.{}", b.id), "DIN EN 1993-1-1", ClauseId::new("EN 1993-1-1", "6", "6.3.1.3"), subject(b, "sway", "Frame mode", "Rahmenverformung"), loc("Buckling length from the frame restraint", "Knicklänge aus der Rahmeneinspannung")).annex(annex);
        let Some((ga, gb, k)) = factors(b) else {
            report.push(base.utilization(dimensionless(2.0), dimensionless(1.0)).explanation(loc("Sway frame with both column ends pinned: the column is a mechanism, the buckling length is unbounded.", "Verschiebliches System mit beidseitig gelenkigen Stützenenden: die Stütze ist ein Mechanismus, die Knicklänge ist unbegrenzt.")).remedy(Remedy::advisory(subject(b, "restraintA", "Node A", "Knoten A"), loc("Restrain the column end at node A by beams or brace the frame.", "Das Stützenende an Knoten A durch Riegel einspannen oder das System aussteifen."))).build());
            continue;
        };
        let derived = k * member.length;
        let required = derived * (1.0 + 1e-9);
        let mut builder = base.utilization(length_m(derived), length_m(entered)).explanation(loc(
            &format!("{en}: G_A={:.3}, G_B={:.3} give K={k:.3}, L_cr=K L={derived:.2} m (L={:.2} m) against the buckling length {entered:.2} m entered at the member.", ga.min(1.0e3), gb.min(1.0e3), member.length),
            &format!("{de}: G_A={:.3}, G_B={:.3} ergeben K={k:.3}, L_cr=K L={derived:.2} m (L={:.2} m) gegenüber der am Bauteil angesetzten Knicklänge {entered:.2} m.", ga.min(1.0e3), gb.min(1.0e3), member.length),
        ));
        if derived > entered {
            builder = builder.remedy(Remedy::at_least(
                SubjectRef::new(&member.id, format!("members[id={}].{leaf}", member.id), loc("Buckling length", "Knicklänge")),
                length_m(entered),
                length_m(required),
                loc(&format!("Enter a buckling length of at least {derived:.2} m (K={k:.3})."), &format!("Eine Knicklänge von mindestens {derived:.2} m ansetzen (K={k:.3}).")),
            ));
        }
        report.push(builder.build());
    }
}
