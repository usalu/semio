//! 🧱️ Nominal fatigue resistance of welded lattice node details from EN 1993-1-9 Table 8.7.

use super::{Inputs, Outcome, lattice_factor, nominal_direct_checks};

/// 🧱️ Verifies one chord or brace with its associated geometry. Table 8.7 requires separate member checks.
pub fn lattice_node_fatigue(q: &Inputs<'_>) -> Outcome {
    if q.t("node_geometry")? != "confirmed" {
        return Err("Confirm the Table 8.7 detail shape and weld construction".into());
    }
    let joint = q.t("node_joint")?;
    let member = q.t("node_member")?;
    if !matches!(member, "chord" | "diagonal") && !(member == "vertical" && matches!(joint, "gap-n" | "overlap-n")) {
        return Err("Select a member present in the chosen joint".into());
    }
    let section = q.t("node_section")?;
    let gap = matches!(joint, "gap-k" | "gap-n");
    let (detail, class_low, class_high, ratio_high) = match (section, joint) {
        ("chs", "gap-k" | "gap-n") => (1, 45.0, 90.0, 2.0),
        ("rhs", "gap-k" | "gap-n") => (2, 36.0, 71.0, 2.0),
        ("chs" | "rhs", "overlap-k") => (3, 56.0, 71.0, 1.4),
        ("chs" | "rhs", "overlap-n") => (4, 50.0, 71.0, 1.4),
        _ => return Err("Select a circular or rectangular hollow-section K or N joint".into()),
    };
    let t0 = q.n("node_t0")?;
    let ti = q.n("node_ti")?;
    let width = q.n("node_chord_width")?;
    let depth = if section == "rhs" { q.n("node_chord_height")? } else { width };
    let brace = q.n("node_brace_width")?;
    let angle = q.n("node_angle")?;
    let eip = q.n("node_eip")?;
    let eop = q.n("node_eop")?;
    let ratio = t0 / ti;
    let (max_width, min_width_ratio) = if section == "rhs" { (200.0, 0.4) } else { (300.0, 0.25) };
    if ti <= 0.0 || ti > 8.0 || t0 <= 0.0 || t0 > 8.0 || ratio < 1.0
        || ![t0, ti, width, depth, brace, angle, eip, eop].into_iter().all(f64::is_finite)
        || width <= 0.0 || depth <= 0.0 || width > max_width || width / ti > 25.0
        || !(min_width_ratio..=1.0).contains(&(brace / width))
        || !(35.0..=50.0).contains(&angle)
        || !(-0.5 * depth..=0.25 * depth).contains(&eip)
        || eop.abs() > 0.02 * width
    {
        return Err("The wall thicknesses, section dimensions, diagonal angle or eccentricities exceed the limits in Table 8.7".into());
    }
    if gap {
        let g = q.n("node_gap")?;
        let width_difference = width - brace;
        if !g.is_finite() || g <= 0.0 || (section == "rhs" && (g < 2.0 * t0 || !(0.5 * width_difference..=1.1 * width_difference).contains(&g))) {
            return Err("Enter a positive gap within the limits of the joint detail in Table 8.7".into());
        }
    } else if !(0.30..=1.0).contains(&q.n("node_overlap")?) {
        return Err("Enter an overlap q/p between 30% and 100%".into());
    }
    let interpolation = ((ratio - 1.0) / (ratio_high - 1.0)).min(1.0);
    let category = class_low + (class_high - class_low) * interpolation;
    let basis = q.t("node_stress_basis")?;
    let (reference, factor) = match basis {
        "complete" => (format!("Table 8.7({detail})"), 1.0),
        "pinned" => {
            let (table, k1) = lattice_factor(section, joint, member, [q.n("node_beta")?, q.n("node_gamma")?, ti / t0, angle])?;
            (format!("Table 8.7({detail}), Table {table}: k₁ = {k1:.2}"), k1)
        }
        _ => return Err("Confirm the nominal stress basis and any pinned-model assumptions".into()),
    };
    let equivalent = q.n("node_equiv_range")?;
    let frequent = q.n("node_frequent_range")?;
    if ![equivalent, frequent].into_iter().all(|value| value.is_finite() && value >= 0.0)
        || ![q.n("fy")?, q.n("gamma_ff")?, q.n("gamma_mf")?].into_iter().all(|value| value.is_finite() && value > 0.0)
    {
        return Err("Enter finite nonnegative stress ranges and positive material and partial-factor values".into());
    }
    nominal_direct_checks(q, equivalent * factor, frequent, category, format!("{reference}, {member}, t₀/tᵢ = {ratio:.3}, Δσ_C = {category:.2} MPa, m = 5"))
}
