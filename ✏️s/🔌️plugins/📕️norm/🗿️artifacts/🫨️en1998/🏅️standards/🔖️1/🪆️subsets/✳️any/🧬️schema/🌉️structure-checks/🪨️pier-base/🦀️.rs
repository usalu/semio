//! 🪨 Soil-structure interaction at the base of a bridge pier (EN 1998-5 Annex and EN 1998-2): the Gazetas springs of the circular footing with the shear modulus degraded by EN 1998-5 Table 4.1, the lengthening of the period of the pier on these springs and the bearing displacement that follows from the lengthened period. The table cells are unverified registry rows. See [`schema::soil_structure`].

use super::*;
use compliance::CheckStatus;
use schema::bridge::{self, Direction};
use schema::soil_structure::{self, Springs};

type Bridge = model::En1998Bridge;
type Base = model::En1998PierBase;

const GRAVITY: f64 = 9.81;

fn q_time(s: f64) -> Quantity { Quantity::new(QuantityKind::Time, s) }

/// 🪨 Springs, period lengthening and bearing displacement of one pier base; `t_fixed_s` is the fundamental period of the bridge on fixed bases and `ground` the spectrum constants of the site.
pub fn check_pier_base(report: &mut CheckReport, annex: AnnexChoice, document: &En1998Snapshot, bridge_row: &Bridge, base: &Base, ground: &super::Ground, t_fixed_s: f64) {
    let path = id_sel("pierBases", &base.id);
    let id = |tail: &str| format!("en1998.2.{}.base.{}.{tail}", bridge_row.id, base.id);
    let subject = |field: &str, label: LocalizedCopy| SubjectRef::new(&base.id, format!("{path}.{field}"), label);
    let clause = || ClauseId::new("EN 1998-5", "5", "4.2.2");
    let title = lc("Foundation springs of the pier base", "Fundamentfedern des Pfeilerfußes");
    let Some(pier) = document.piers.iter().find(|p| p.id == base.pier_id && p.bridge_id == bridge_row.id) else {
        let options: Vec<String> = document.piers.iter().filter(|p| p.bridge_id == bridge_row.id).map(|p| p.id.clone()).collect();
        report.push(
            CheckResult::assess(id("pier"), "DIN EN 1998-5", clause(), subject("pierId", lc(&base.id, &base.id)), lc("Existing pier of the bridge", "Vorhandener Pfeiler der Brücke"))
                .status(CheckStatus::Fail)
                .annex(annex)
                .explanation(lc(&format!("pierBases[{}].pierId names no pier of bridge {}", base.id, bridge_row.id), &format!("pierBases[{}].pierId nennt keinen Pfeiler der Brücke {}", base.id, bridge_row.id)))
                .remedy(Remedy::one_of(subject("pierId", lc("Pier", "Pfeiler")), options, lc("Point the pier base at a pier of its bridge.", "Pfeilerfuß auf einen Pfeiler seiner Brücke setzen.")))
                .build(),
        );
        return;
    };
    let a_g_s_over_g = ground.gamma_i * ground.a_g * ground.s / GRAVITY;
    let ratio = soil_structure::table_4_1_vs_ratio(a_g_s_over_g, annex);
    let damping = soil_structure::table_4_1_damping(a_g_s_over_g, annex);
    let g_max = soil_structure::shear_modulus_max_pa(base.soil_unit_weight_n_m3, base.soil_vs_m_s);
    let g = soil_structure::degraded_shear_modulus_pa(g_max, ratio);
    let Some(springs) = Springs::circular(g, base.radius_m, base.poisson_ratio).filter(|_| ratio.is_finite()) else {
        let mut check = CheckResult::assess(id("springs"), "DIN EN 1998-5", clause(), subject("radiusM", lc(&base.id, &base.id)), title)
            .status(CheckStatus::Fail)
            .annex(annex)
            .explanation(lc(
                &format!("R = {:.3} m, v_s = {:.1} m/s, γ = {:.0} N/m³ and ν = {:.3} give no foundation springs: R, v_s and γ must be positive and 0 ≤ ν < 0.5", base.radius_m, base.soil_vs_m_s, base.soil_unit_weight_n_m3, base.poisson_ratio),
                &format!("R = {:.3} m, v_s = {:.1} m/s, γ = {:.0} N/m³ und ν = {:.3} ergeben keine Fundamentfedern: R, v_s und γ müssen positiv sein und 0 ≤ ν < 0,5", base.radius_m, base.soil_vs_m_s, base.soil_unit_weight_n_m3, base.poisson_ratio),
            ));
        if !(base.radius_m > 0.0) {
            check = check.remedy(Remedy::at_least(subject("radiusM", lc("Footing radius R", "Fundamentradius R")), q_len(base.radius_m), q_len(f64::EPSILON), lc("Enter the positive radius of the equivalent circular footing.", "Positiven Radius des flächengleichen Kreisfundaments eingeben.")));
        }
        if !(base.soil_vs_m_s > 0.0) {
            check = check.remedy(Remedy::at_least(subject("soilVsMS", lc("Shear wave velocity v_s", "Scherwellengeschwindigkeit v_s")), Quantity::new(QuantityKind::Velocity, base.soil_vs_m_s), Quantity::new(QuantityKind::Velocity, f64::EPSILON), lc("Enter the positive shear wave velocity of the soil.", "Positive Scherwellengeschwindigkeit des Bodens eingeben.")));
        }
        if !(base.soil_unit_weight_n_m3 > 0.0) {
            check = check.remedy(Remedy::at_least(subject("soilUnitWeightNM3", lc("Soil unit weight γ", "Wichte des Bodens γ")), Quantity::new(QuantityKind::Dimensionless, base.soil_unit_weight_n_m3), Quantity::new(QuantityKind::Dimensionless, f64::EPSILON), lc("Enter the positive unit weight of the soil.", "Positive Wichte des Bodens eingeben.")));
        }
        if !(0.0..0.5).contains(&base.poisson_ratio) {
            check = check.remedy(Remedy::at_most(subject("poissonRatio", lc("Poisson ratio ν", "Querdehnzahl ν")), Quantity::new(QuantityKind::Dimensionless, base.poisson_ratio), Quantity::new(QuantityKind::Dimensionless, 0.499), lc("Enter a Poisson ratio of the soil below 0.5.", "Querdehnzahl des Bodens unter 0,5 eingeben.")));
        }
        report.push(check.ndp_reads().build());
        return;
    };
    report.push(
        CheckResult::assess(id("springs"), "DIN EN 1998-5", clause(), subject("radiusM", lc(&base.id, &base.id)), title)
            .informative(q_dim(springs.horizontal_n_per_m))
            .annex(annex)
            .explanation(lc(
                &format!("G_max = (γ/g) v_s² = {g_max:.4e} Pa, v_s/v_s,max = {ratio:.3} at a_g S/g = {a_g_s_over_g:.3} (Table 4.1, damping {:.1} %), G = {g:.4e} Pa; K_x = 8GR/(2−ν) = {:.4e} N/m, K_z = {:.4e} N/m, K_rx = 8GR³/(3(1−ν)) = {:.4e} N·m/rad, K_t = {:.4e} N·m/rad", 100.0 * damping, springs.horizontal_n_per_m, springs.vertical_n_per_m, springs.rocking_nm_per_rad, springs.torsion_nm_per_rad),
                &format!("G_max = (γ/g) v_s² = {g_max:.4e} Pa, v_s/v_s,max = {ratio:.3} bei a_g S/g = {a_g_s_over_g:.3} (Tabelle 4.1, Dämpfung {:.1} %), G = {g:.4e} Pa; K_x = 8GR/(2−ν) = {:.4e} N/m, K_z = {:.4e} N/m, K_rx = 8GR³/(3(1−ν)) = {:.4e} N·m/rad, K_t = {:.4e} N·m/rad", 100.0 * damping, springs.horizontal_n_per_m, springs.vertical_n_per_m, springs.rocking_nm_per_rad, springs.torsion_nm_per_rad),
            ))
            .ndp_reads()
            .build(),
    );
    let section = schema::part_3::AnnexA::from_registry(annex);
    let stiffness = |direction: Direction| {
        let (width, depth) = if direction == Direction::Longitudinal { (pier.section_b_m, pier.section_h_m) } else { (pier.section_h_m, pier.section_b_m) };
        bridge::pier_stiffness_n_per_m(pier.modulus_mpa, bridge::pier_ei_ratio(pier, direction, bridge_row.pier_cracked_inertia_ratio, &section) * width * depth.powi(3) / 12.0, pier.height_m)
    };
    let (longitudinal, transverse) = (stiffness(Direction::Longitudinal), stiffness(Direction::Transverse));
    let (k_pier, direction) = if longitudinal >= transverse { (longitudinal, Direction::Longitudinal) } else { (transverse, Direction::Transverse) };
    if !(k_pier > 0.0) || !(t_fixed_s > 0.0) {
        report.push(CheckResult::assess(id("period"), "DIN EN 1998-5", clause(), subject("pierId", lc(&base.id, &base.id)), lc("Period lengthening of the pier on its foundation springs", "Periodenverlängerung des Pfeilers auf den Fundamentfedern")).not_applicable(lc("The pier has no flexural stiffness or the bridge no fundamental period, so there is no period to lengthen.", "Der Pfeiler hat keine Biegesteifigkeit oder die Brücke keine Grundperiode, es gibt keine Periode zu verlängern.")).annex(annex).build());
        return;
    }
    let lengthening = soil_structure::period_lengthening(k_pier, pier.height_m, &springs);
    let t_ssi = t_fixed_s * lengthening;
    let name = if direction == Direction::Longitudinal { ("longitudinal", "längs") } else { ("transverse", "quer") };
    report.push(
        CheckResult::assess(id("period"), "DIN EN 1998-5", clause(), subject("pierId", lc(&base.id, &base.id)), lc("Period lengthening of the pier on its foundation springs", "Periodenverlängerung des Pfeilers auf den Fundamentfedern"))
            .informative(q_time(t_ssi))
            .annex(annex)
            .explanation(lc(
                &format!("k_p = 3EI_eff/H³ = {k_pier:.4e} N/m ({}), T_ssi/T_fixed = √(1 + k_p/K_x + k_p H²/K_rx) = {lengthening:.3}: T_fixed = {t_fixed_s:.3} s becomes T_ssi = {t_ssi:.3} s", name.0),
                &format!("k_p = 3EI_eff/H³ = {k_pier:.4e} N/m ({}), T_ssi/T_fest = √(1 + k_p/K_x + k_p H²/K_rx) = {lengthening:.3}: T_fest = {t_fixed_s:.3} s wird zu T_ssi = {t_ssi:.3} s", name.1),
            ))
            .build(),
    );
    let d_fixed = schema::part_2::bearing_displacement_m(ground.elastic(t_fixed_s), t_fixed_s);
    let d_ssi = schema::part_2::bearing_displacement_m(ground.elastic(t_ssi), t_ssi);
    let path_bridge = id_sel("bridges", &bridge_row.id);
    let mut displacement = CheckResult::assess(id("displacement"), "DIN EN 1998-2", ClauseId::new("EN 1998-2", "2", "7.5"), subject("pierId", lc(&base.id, &base.id)), lc("Bearing displacement with the foundation flexibility", "Lagerverschiebung mit der Fundamentnachgiebigkeit"))
        .utilization(q_len(d_ssi), floor_limit(q_len(bridge_row.bearing_d_rd_m)))
        .annex(annex)
        .explanation(lc(
            &format!("d = γ_I S_e(T)(T/2π)² = {d_ssi:.4} m at T_ssi = {t_ssi:.3} s against {d_fixed:.4} m at T_fixed = {t_fixed_s:.3} s; capacity d_Rd = {:.4} m", bridge_row.bearing_d_rd_m),
            &format!("d = γ_I S_e(T)(T/2π)² = {d_ssi:.4} m bei T_ssi = {t_ssi:.3} s gegen {d_fixed:.4} m bei T_fest = {t_fixed_s:.3} s; Kapazität d_Rd = {:.4} m", bridge_row.bearing_d_rd_m),
        ));
    if d_ssi > bridge_row.bearing_d_rd_m {
        displacement = displacement.remedy(Remedy::at_least(SubjectRef::new(&bridge_row.id, format!("{path_bridge}.bearingDRdM"), lc("Bearing displacement capacity d_Rd", "Lagerverschiebungskapazität d_Rd")), q_len(bridge_row.bearing_d_rd_m), q_len(d_ssi), lc("Increase the bearing displacement capacity to the displacement with the foundation flexibility.", "Verschiebungskapazität der Lager auf die Verschiebung mit der Fundamentnachgiebigkeit erhöhen.")));
    }
    report.push(displacement.build());
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
