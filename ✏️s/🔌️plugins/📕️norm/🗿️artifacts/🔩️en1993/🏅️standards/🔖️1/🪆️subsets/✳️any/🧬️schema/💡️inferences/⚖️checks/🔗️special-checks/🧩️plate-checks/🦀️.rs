//! 🧩️ Joint plate and foundation block checks: fin plate and splice cover plates (bolt group with eccentricity, bearing, block tearing, net section, weld) and base plate bearing on grout and concrete with the anchors in tension and shear (EN 1993-1-8 3.6, 3.10.2, 4.5.3.2, 6.2.2, 6.2.5, 6.2.8; EN 1992-1-1 6.7). See [EN 1993-1-8](https://eurocodes.jrc.ec.europa.eu/).

use super::*;
use crate::clauses::joint_components as jc;
use crate::clauses::joints as jt;
use semio_s_artifact_norm_en1993_model::{FoundationBlock, JointPlate, SteelJoint};
use semio_s_plugin_norm_mechanics::fastening::{cone_breakout, ConeInput};

const GAMMA_C: &str = "ndp.en1992.m-01.gamma-c-persistent";
const GAMMA_INST: &str = "ndp.en1992.gamma-inst";
const K1_CONE: &str = "ndp.en1992.cone-k-cast-cracked";

fn subject(coll: &str, id: &str, field: &str, en: &str, de: &str) -> SubjectRef {
    SubjectRef::new(id, format!("{coll}[id={id}].{field}"), loc(en, de))
}

fn scale(u: f64) -> f64 {
    if u.is_finite() { u.max(1.0) } else { 2.0 }
}

fn joint<'a>(d: &'a En1993Snapshot, id: &str) -> Option<&'a SteelJoint> {
    d.joints.iter().find(|j| j.id == id)
}

/// 🧮 Largest shear and tension (N) of the joint actions.
pub fn joint_demand(j: &SteelJoint) -> (f64, f64) {
    j.actions.iter().fold((0.0, 0.0), |(v, n), a| (v.max(a.shear.abs()), n.max(a.tension)))
}

/// 🧮 Bolt coordinates (mm) around the centroid: `n_1` rows along the line of action (y), `n_2` per row across (x).
pub fn bolt_points(p: &JointPlate) -> Vec<(f64, f64)> {
    let (n1, n2) = (p.bolt_rows.max(1) as usize, p.bolts_per_row.max(1) as usize);
    let (p1, p2) = (p.pitch * 1e3, p.gauge * 1e3);
    (0..n1).flat_map(|r| (0..n2).map(move |c| ((c as f64 - (n2 as f64 - 1.0) / 2.0) * p2, (r as f64 - (n1 as f64 - 1.0) / 2.0) * p1))).collect()
}

/// 🧮 Force (N) and moment (N mm) on the bolt group of a plate: flange cover plates carry the flange force `M/(h - t_f) + N A_f/A`, fin and web cover plates carry the shear with the eccentricity of the bolt line.
pub fn plate_load(document: &En1993Snapshot, p: &JointPlate, j: &SteelJoint) -> Option<(f64, f64, f64)> {
    let (v, n) = joint_demand(j);
    if p.role != "flangeCover" {
        return Some((n, v, v * p.eccentricity * 1e3));
    }
    let section = document.members.iter().find(|m| m.id == j.member_id).and_then(|m| document.sections.iter().find(|s| s.id == m.section_id))?;
    let lever = section.h - section.tf;
    (lever > 0.0 && section.area > 0.0).then(|| (0.0, (j.moment_demand.abs() / lever + n * section.b * section.tf / section.area).abs(), 0.0))
}

/// 🧮 Bolt group resistance per bolt (N): shear over the shear planes and bearing on the plates.
pub fn bolt_resistance(p: &JointPlate, gm2: f64) -> (f64, f64) {
    let (d, d0, e1, e2) = (p.bolt_diameter * 1e3, p.hole_diameter * 1e3, p.end_distance * 1e3, p.edge_distance * 1e3);
    let (fu, fub) = (p.plate_fu / 1e6, jt::bolt_fub(&p.bolt_class));
    let (p1, p2) = (p.pitch * 1e3, p.gauge * 1e3);
    let end = jt::alpha_b(true, e1, p1, d0, fub, fu);
    let alpha = if p.bolt_rows > 1 { end.min(jt::alpha_b(false, e1, p1, d0, fub, fu)) } else { end };
    let k1 = if p.bolts_per_row > 1 { jt::k1_bearing(true, e2, p2, d0) } else { (2.8 * e2 / d0 - 1.7).min(2.5) };
    (jt::fv_rd(&p.bolt_class, d, true, gm2) * f64::from(p.shear_planes.max(1)), jt::fb_rd(k1, alpha, fu, d, p.plate_thickness * 1e3 * f64::from(p.plate_count.max(1)), gm2, 1.0))
}

fn plate_valid(p: &JointPlate) -> bool {
    p.plate_thickness > 0.0 && p.plate_fu > 0.0 && p.plate_fy > 0.0 && p.bolt_diameter > 0.0 && p.hole_diameter > 0.0 && p.bolt_rows > 0 && p.bolts_per_row > 0 && p.end_distance > 0.0 && p.edge_distance > 0.0
}

fn force_case(id: String, clause: (&str, &str), subject: SubjectRef, title: LocalizedCopy, ed: f64, rd: f64, annex: AnnexChoice, gammas: &[&str], explanation: LocalizedCopy) -> CheckResult {
    let mut builder = CheckResult::assess(id, "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", clause.0, clause.1), subject.clone(), title)
        .annex(annex)
        .ndps(gammas)
        .utilization(Quantity::new(QuantityKind::Force, ed), Quantity::new(QuantityKind::Force, rd))
        .explanation(explanation);
    if ed > rd && ed > 0.0 {
        let scaled = rd;
        builder = builder.remedy(Remedy::at_most(subject, force_n(ed), force_n(scaled), loc(&format!("Limit the force to {:.1} kN or enlarge the connection.", scaled / 1e3), &format!("Kraft auf {:.1} kN begrenzen oder den Anschluss vergrößern.", scaled / 1e3))));
    }
    builder.build()
}

/// 🧩️ Appends the fin plate and splice cover plate checks of every admitted plate.
pub fn check_joint_plates(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    let (gm0, gm2) = (params.gamma_m0, params.gamma_m2);
    for p in document.joint_plates.iter().filter(|p| scope.admits("jointPlates", &p.id) && plate_valid(p)) {
        let Some(j) = joint(document, &p.joint_id) else { continue };
        let Some((fx, fy, mz)) = plate_load(document, p, j) else { continue };
        if fx == 0.0 && fy == 0.0 {
            continue;
        }
        let at = |field: &str, en: &str, de: &str| subject("jointPlates", &p.id, field, en, de);
        let points = bolt_points(p);
        let f_ed = (0..points.len()).map(|i| jt::bolt_group_force(&points, fx, fy, mz, i)).fold(0.0, f64::max);
        let (fv, fb) = bolt_resistance(p, gm2);
        let plastic = (p.group_method == semio_s_artifact_norm_en1993_model::BoltGroupMethod::InstantaneousCentre).then(|| super::bolt_group_ic_checks::ic_case(document, p, annex, params)).flatten();
        report.push(plastic.unwrap_or_else(|| force_case(
            format!("en1993.1-8.3.6.plate.{}", p.id),
            ("3", "3.6"),
            at("boltDiameter", "Bolt diameter", "Schraubendurchmesser"),
            loc("Plate bolt group: shear and bearing", "Schraubengruppe des Blechs: Abscheren und Lochleibung"),
            f_ed,
            fv.min(fb),
            annex,
            &[AnnexParams::GAMMA_M2],
            loc(
                &format!("Elastic bolt group of {} bolts, F_Ed=({:.1} kN along, {:.1} kN across, {:.2} kNm): F_b,Ed,max={:.1} kN against F_v,Rd={:.1} kN (Table 3.4, {} shear planes) and F_b,Rd={:.1} kN (bearing on {} plate(s) of {:.0} mm).", points.len(), fy / 1e3, fx / 1e3, mz / 1e6, f_ed / 1e3, fv / 1e3, p.shear_planes.max(1), fb / 1e3, p.plate_count.max(1), p.plate_thickness * 1e3),
                &format!("Elastische Schraubengruppe mit {} Schrauben, F_Ed=({:.1} kN längs, {:.1} kN quer, {:.2} kNm): F_b,Ed,max={:.1} kN gegen F_v,Rd={:.1} kN (Tabelle 3.4, {} Scherfugen) und F_b,Rd={:.1} kN (Lochleibung an {} Blech(en) von {:.0} mm).", points.len(), fy / 1e3, fx / 1e3, mz / 1e6, f_ed / 1e3, fv / 1e3, p.shear_planes.max(1), fb / 1e3, p.plate_count.max(1), p.plate_thickness * 1e3),
            ),
        )));
        let t = p.plate_thickness * 1e3 * f64::from(p.plate_count.max(1));
        let (a_nt, a_nv) = jt::block_tearing_areas(p.bolt_rows, p.bolts_per_row, p.pitch * 1e3, p.gauge * 1e3, p.end_distance * 1e3, p.edge_distance * 1e3, p.hole_diameter * 1e3, t);
        let block = jt::block_tearing(p.plate_fu / 1e6, p.plate_fy / 1e6, a_nt, a_nv, p.eccentricity <= 0.0, gm0, gm2);
        report.push(force_case(
            format!("en1993.1-8.3.10.plateblock.{}", p.id),
            ("3", "3.10.2"),
            at("endDistance", "End distance", "Randabstand"),
            loc("Plate block tearing", "Blockversagen des Blechs"),
            fy.hypot(fx),
            block,
            annex,
            &[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2],
            loc(
                &format!("Block tearing: A_nt={a_nt:.0} mm², A_nv={a_nv:.0} mm², V_eff,{}Rd={:.1} kN.", if p.eccentricity <= 0.0 { "1,", "2," }, block / 1e3),
                &format!("Blockversagen: A_nt={a_nt:.0} mm², A_nv={a_nv:.0} mm², V_eff,{}Rd={:.1} kN.", if p.eccentricity <= 0.0 { "1,", "2," }, block / 1e3),
            ),
        ));
        if p.role == "flangeCover" && p.plate_width > 0.0 {
            let (gross, net) = (p.plate_width * 1e3 * t, (p.plate_width * 1e3 - f64::from(p.bolts_per_row) * p.hole_diameter * 1e3) * t);
            let (n_pl, n_u) = (gross * p.plate_fy / 1e6 / gm0, 0.9 * net * p.plate_fu / 1e6 / gm2);
            report.push(force_case(
                format!("en1993.1-1.6.2.3.platenet.{}", p.id),
                ("6", "6.2.3"),
                at("plateWidth", "Plate width", "Blechbreite"),
                loc("Cover plate tension: gross and net section", "Zug der Stoßlasche: Brutto- und Nettoquerschnitt"),
                fy,
                n_pl.min(n_u),
                annex,
                &[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2],
                loc(&format!("N_pl,Rd={:.1} kN (gross), N_u,Rd=0.9 A_net f_u/γ_M2={:.1} kN (net, {} holes across).", n_pl / 1e3, n_u / 1e3, p.bolts_per_row), &format!("N_pl,Rd={:.1} kN (brutto), N_u,Rd=0,9 A_net f_u/γ_M2={:.1} kN (netto, {} Löcher quer).", n_pl / 1e3, n_u / 1e3, p.bolts_per_row)),
            ));
        }
        if p.role == "finPlate" && p.weld_throat > 0.0 && p.weld_fu > 0.0 && p.weld_beta_w > 0.0 && p.plate_height > 0.0 {
            let (a, l) = (p.weld_throat * 1e3, p.plate_height * 1e3);
            let sigma = mz / (a * l * l / 3.0) / 2.0_f64.sqrt();
            let tau = fy / (2.0 * a * l);
            let u = jt::weld_directional(sigma, sigma, tau, p.weld_fu / 1e6, p.weld_beta_w, gm2);
            let k = scale(u);
            let mut weld =
                CheckResult::assess(format!("en1993.1-8.4.5.plateweld.{}", p.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "4", "4.5.3.2"), at("weldThroat", "Weld throat thickness", "Nahtdicke"), loc("Fin plate weld: directional method", "Naht der Fahnenblechs: richtungsbezogenes Verfahren"))
                    .annex(annex)
                    .ndp(AnnexParams::GAMMA_M2)
                    .utilization(dimensionless(u), dimensionless(1.0))
                    .explanation(loc(
                        &format!("Two fillet welds of a={a:.1} mm over {l:.0} mm: σ⊥=τ⊥={sigma:.1} N/mm² from M=V z={:.2} kNm, τ∥={tau:.1} N/mm² from V={:.1} kN; both criteria of 4.5.3.2 give a utilisation of {u:.2}.", mz / 1e6, fy / 1e3),
                        &format!("Zwei Kehlnähte a={a:.1} mm über {l:.0} mm: σ⊥=τ⊥={sigma:.1} N/mm² aus M=V z={:.2} kNm, τ∥={tau:.1} N/mm² aus V={:.1} kN; beide Kriterien nach 4.5.3.2 ergeben eine Ausnutzung von {u:.2}.", mz / 1e6, fy / 1e3),
                    ));
            if !(u <= 1.0) {
                weld = weld
                    .remedy(Remedy::at_least(at("weldThroat", "Weld throat thickness", "Nahtdicke"), length_m(a / 1e3), length_m(a * k / 1e3), loc(&format!("Increase the weld throat to at least {:.1} mm.", a * k), &format!("Nahtdicke auf mindestens {:.1} mm erhöhen.", a * k))))
                    .remedy(Remedy::at_least(at("plateHeight", "Plate height", "Fahnenblechhöhe"), length_m(l / 1e3), length_m(l * k / 1e3), loc(&format!("Or lengthen the weld (plate height) to at least {:.0} mm.", l * k), &format!("Oder die Naht (Blechhöhe) auf mindestens {:.0} mm verlängern.", l * k))));
            }
            report.push(weld.build());
        }
    }
}

/// 🧮 Width (m) of the column flange `b_fc` of the section of the member that carries the base plate joint, `None` while the member, its section or the width is missing.
pub fn column_flange_width(document: &En1993Snapshot, j: &SteelJoint) -> Option<f64> {
    let b = document.members.iter().find(|m| m.id == j.member_id).and_then(|m| document.sections.iter().find(|s| s.id == m.section_id))?.b;
    (b > 0.0).then_some(b)
}

/// 🧮 Bearing of the base plate on grout and concrete (mm, N, MPa) as the c-strip of the flange T-stub with `b_eff = t_fc + 2c` and `l_eff = b_fc + 2c`, both limited by the plate (EN 1993-1-8 6.2.5(3)), and the load spreading of the block (EN 1992-1-1 6.7); `bfc` in m.
pub fn base_bearing(b: &FoundationBlock, j: &SteelJoint, bfc: f64, fcd: f64, gm0: f64) -> jc::CStrip {
    let (lp, bp, tp, fyp, tfc) = (j.base_plate_length * 1e3, j.plate_width * 1e3, j.plate_thickness * 1e3, j.plate_fy / 1e6, j.column_flange_thickness * 1e3);
    let (a, w, h) = (b.block_length * 1e3, b.block_width * 1e3, b.block_depth * 1e3);
    jc::base_plate_c_strip(tfc, bfc * 1e3, tp, fyp, gm0, lp, bp, 2.0 / 3.0, fcd, &|b1, l1| (b1 * l1, a.min(b1 + h).min(3.0 * b1) * w.min(l1 + h).min(3.0 * l1)))
}

/// 🧮 Anchor forces (N) from the base plate statics, tension positive: (T on the tension side, C on the compression side).
pub fn base_forces(j: &SteelJoint) -> (f64, f64) {
    let (n, m, zl, zr) = (j.base_normal_force, j.base_moment.abs(), j.base_lever_left, j.base_lever_right);
    if zl + zr <= 0.0 {
        return (n.max(0.0), (-n).max(0.0));
    }
    let t = (m + n * zr) / (zl + zr);
    if t <= 0.0 { (0.0, (-n).max(0.0)) } else { (t, (t - n).max(0.0)) }
}

/// 🧮 Compression (N) of the left and the right flange T-stub from the statics of `base_forces`; with the whole section in compression the force splits over both flanges, otherwise the compression side carries `C` alone.
pub fn base_flange_compression(j: &SteelJoint) -> (f64, f64) {
    let (n, m, zl, zr) = (j.base_normal_force, j.base_moment.abs(), j.base_lever_left, j.base_lever_right);
    if zl + zr <= 0.0 {
        return (0.0, (-n).max(0.0));
    }
    let t = (m + n * zr) / (zl + zr);
    if t > 0.0 { (0.0, (t - n).max(0.0)) } else { ((-t).max(0.0), (t - n).max(0.0)) }
}

/// 🧱️ Appends the grout, bearing, anchor tension and anchor shear checks of every admitted foundation block.
pub fn check_foundation_blocks(document: &En1993Snapshot, annex: AnnexChoice, params: AnnexParams, scope: Scope<'_>, report: &mut CheckReport) {
    let (gm0, gm2) = (params.gamma_m0, params.gamma_m2);
    for b in document.foundation_blocks.iter().filter(|b| scope.admits("foundationBlocks", &b.id)) {
        let Some(j) = joint(document, &b.joint_id) else { continue };
        let Some(gamma_c) = semio_s_plugin_norm_compliance::ndp_number_opt(GAMMA_C, annex) else { continue };
        let Some(bfc) = column_flange_width(document, j) else { continue };
        if !base_valid(b, j) {
            continue;
        }
        let at = |field: &str, en: &str, de: &str| subject("foundationBlocks", &b.id, field, en, de);
        let limit = 0.2 * j.base_plate_length.min(j.plate_width);
        let u_grout = (b.grout_thickness / limit).max(0.2 * b.concrete_fck / b.grout_fck);
        let mut grout =
            CheckResult::assess(format!("en1993.1-8.6.2.grout.{}", b.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "6", "6.2.5"), at("groutThickness", "Grout thickness", "Dicke des Vergussmörtels"), loc("Grout condition for β_j = 2/3", "Vergussbedingung für β_j = 2/3"))
                .annex(annex)
                .utilization(dimensionless(u_grout), dimensionless(1.0))
                .explanation(loc(
                    &format!("β_j=2/3 requires f_ck,grout ≥ 0.2 f_ck ({:.1} MPa ≥ {:.1} MPa) and t_g ≤ 0.2 min(a, b) ({:.1} mm ≤ {:.1} mm).", b.grout_fck / 1e6, 0.2 * b.concrete_fck / 1e6, b.grout_thickness * 1e3, limit * 1e3),
                    &format!("β_j=2/3 verlangt f_ck,Mörtel ≥ 0,2 f_ck ({:.1} MPa ≥ {:.1} MPa) und t_g ≤ 0,2 min(a, b) ({:.1} mm ≤ {:.1} mm).", b.grout_fck / 1e6, 0.2 * b.concrete_fck / 1e6, b.grout_thickness * 1e3, limit * 1e3),
                ));
        if b.grout_thickness > limit {
            grout = grout.remedy(Remedy::at_most(at("groutThickness", "Grout thickness", "Dicke des Vergussmörtels"), length_m(b.grout_thickness), length_m(limit), loc(&format!("Reduce the grout thickness to at most {:.1} mm.", limit * 1e3), &format!("Vergussdicke auf höchstens {:.1} mm verringern.", limit * 1e3))));
        }
        if b.grout_fck < 0.2 * b.concrete_fck {
            grout = grout.remedy(Remedy::at_least(at("groutFck", "Grout strength", "Mörtelfestigkeit"), stress_pa(b.grout_fck), stress_pa(0.2 * b.concrete_fck), loc(&format!("Use a grout of at least {:.1} MPa.", 0.2 * b.concrete_fck / 1e6), &format!("Einen Mörtel von mindestens {:.1} MPa verwenden.", 0.2 * b.concrete_fck / 1e6))));
        }
        if !(u_grout <= 1.0) && b.grout_thickness <= limit && b.grout_fck >= 0.2 * b.concrete_fck {
            grout = grout.remedy(Remedy::at_most(at("groutThickness", "Grout thickness", "Dicke des Vergussmörtels"), length_m(b.grout_thickness), length_m(b.grout_thickness / scale(u_grout)), loc("Reduce the grout thickness.", "Vergussdicke verringern.")));
        }
        report.push(grout.build());
        let (t, _) = base_forces(j);
        let c = {
            let (left, right) = base_flange_compression(j);
            left.max(right)
        };
        if u_grout <= 1.0 && c > 0.0 {
            let jc::CStrip { beta_j: beta, k_j: kj, f_jd: fjd, b_eff, l_eff, fc_rd: fc, .. } = base_bearing(b, j, bfc, b.concrete_fck / 1e6 / gamma_c, gm0);
            report.push(
                CheckResult::assess(format!("en1993.1-8.6.2.bearing.{}", b.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "6", "6.2.5"), at("concreteFck", "Concrete strength", "Betonfestigkeit"), loc("Base plate bearing on grout and concrete", "Fußplatte auf Mörtel und Beton: Pressung"))
                    .annex(annex)
                    .ndps(&[AnnexParams::GAMMA_M0, GAMMA_C])
                    .utilization(force_n(c), force_n(fc))
                    .explanation(loc(
                        &format!("f_jd=β_j F_Rdu/(b_eff l_eff)={fjd:.1} MPa with β_j={beta:.3}, k_j={kj:.2} from the block spreading (EN 1992-1-1 6.7, F_Rdu ≤ 3 f_cd A_c0), f_cd={:.1} MPa; flange T-stub b_eff=t_fc+2c={b_eff:.0} mm, l_eff=b_fc+2c={l_eff:.0} mm (limited by the plate edges), b_fc={:.0} mm, F_c,Rd={:.1} kN against C_Ed={:.1} kN.", b.concrete_fck / 1e6 / gamma_c, bfc * 1e3, fc / 1e3, c / 1e3),
                        &format!("f_jd=β_j F_Rdu/(b_eff l_eff)={fjd:.1} MPa mit β_j={beta:.3}, k_j={kj:.2} aus der Lastausbreitung im Block (EN 1992-1-1 6.7, F_Rdu ≤ 3 f_cd A_c0), f_cd={:.1} MPa; Flansch-T-Stummel b_eff=t_fc+2c={b_eff:.0} mm, l_eff=b_fc+2c={l_eff:.0} mm (durch die Plattenränder begrenzt), b_fc={:.0} mm, F_c,Rd={:.1} kN gegen C_Ed={:.1} kN.", b.concrete_fck / 1e6 / gamma_c, bfc * 1e3, fc / 1e3, c / 1e3),
                    ))
                    .remedy(Remedy::at_most(SubjectRef::new(&j.id, format!("joints[id={}].baseNormalForce", j.id), loc("Base normal force", "Fußnormalkraft")), force_n(j.base_normal_force), force_n(j.base_normal_force * fc / c), loc(&format!("Limit the compression to {:.1} kN or enlarge the block or the plate.", c.min(fc) / 1e3), &format!("Druck auf {:.1} kN begrenzen oder Block oder Platte vergrößern.", c.min(fc) / 1e3))))
                    .build(),
            );
        }
        if t > 0.0 && b.anchor_count > 0 && b.anchor_stress_area > 0.0 {
            let (fub, a_s) = (jt::bolt_fub(&b.anchor_class), b.anchor_stress_area * 1e6);
            let per = t / f64::from(b.anchor_count);
            let rd = 0.9 * fub * a_s / gm2;
            report.push(force_case(
                format!("en1993.1-8.6.2.anchort.{}", b.id),
                ("6", "6.2.6"),
                at("anchorCount", "Anchors in tension", "Zugbeanspruchte Anker"),
                loc("Anchor tension", "Ankerzug"),
                per,
                rd,
                annex,
                &[AnnexParams::GAMMA_M2],
                loc(&format!("T_Ed={:.1} kN on {} anchors, F_t,Rd=0.9 f_ub A_s/γ_M2={:.1} kN; the pull-out and cone failure of the concrete are not checked.", t / 1e3, b.anchor_count, rd / 1e3), &format!("T_Ed={:.1} kN auf {} Anker, F_t,Rd=0,9 f_ub A_s/γ_M2={:.1} kN; Herausziehen und Betonausbruch sind nicht nachgewiesen.", t / 1e3, b.anchor_count, rd / 1e3)),
            ));
        }
        let (v, _) = joint_demand(j);
        if v > 0.0 && b.anchor_count > 0 && b.anchor_stress_area > 0.0 {
            let (fub, fyb, a_s) = (jt::bolt_fub(&b.anchor_class), jt::bolt_fyb(&b.anchor_class), b.anchor_stress_area * 1e6);
            let alpha = 0.44 - 0.0003 * fyb;
            let rd = f64::from(b.anchor_count) * alpha * fub * a_s / gm2;
            report.push(force_case(
                format!("en1993.1-8.6.2.anchorv.{}", b.id),
                ("6", "6.2.2"),
                at("anchorDiameter", "Anchor diameter", "Ankerdurchmesser"),
                loc("Anchor shear without friction", "Ankerabscheren ohne Reibung"),
                v,
                rd,
                annex,
                &[AnnexParams::GAMMA_M2],
                loc(&format!("V_Ed={:.1} kN carried by the {} anchors alone: F_v,Rd=n α_b f_ub A_s/γ_M2={:.1} kN with α_b=0.44-0.0003 f_yb={alpha:.3}; friction in the joint is not used.", v / 1e3, b.anchor_count, rd / 1e3), &format!("V_Ed={:.1} kN allein von den {} Ankern: F_v,Rd=n α_b f_ub A_s/γ_M2={:.1} kN mit α_b=0,44-0,0003 f_yb={alpha:.3}; Reibung in der Fuge wird nicht angesetzt.", v / 1e3, b.anchor_count, rd / 1e3)),
            ));
        }
        check_base_plate_components(b, j, bfc, annex, params, gamma_c, u_grout <= 1.0, report);
    }
}

fn base_valid(b: &FoundationBlock, j: &SteelJoint) -> bool {
    b.concrete_fck > 0.0 && b.grout_fck > 0.0 && j.plate_width > 0.0 && j.base_plate_length > 0.0 && j.plate_thickness > 0.0 && j.plate_fy > 0.0 && b.block_length > 0.0 && b.block_width > 0.0
}

/// 🪨 EN 1993-1-8 6.2.5 and 6.2.6.11: `c = t_p √(f_y/(3 f_jd γ_M0))` of the base plate on its foundation block with the flange T-stub footprint `(t_fc + 2c) × (b_fc + 2c)` limited by the plate; `bfc` in m, `None` while the block, the joint or γ_C of the registry is missing.
pub fn compression_strip(b: &FoundationBlock, j: &SteelJoint, bfc: f64, annex: AnnexChoice) -> Option<jc::CStrip> {
    let gamma_c = semio_s_plugin_norm_compliance::ndp_number_opt(GAMMA_C, annex)?;
    if !base_valid(b, j) || bfc <= 0.0 {
        return None;
    }
    Some(base_bearing(b, j, bfc, b.concrete_fck / 1e6 / gamma_c, AnnexParams::for_choice(annex).gamma_m0))
}

/// 🪝 The tension T-stub of the anchors under the column flange (EN 1993-1-8 6.2.6.11 with Table 6.2 modes 1 to 3, with and without prying, and the effective lengths of Table 6.4 for the single row inside the flange; mm, N, MPa): modes, effective lengths and the anchor resistance `Σ F_t,Rd`.
pub fn tension_stub(b: &FoundationBlock, j: &SteelJoint, gm0: f64, gm2: f64) -> Option<(jc::TStubModes, jc::LEff, f64)> {
    let (m, e, tp, a_s, lb) = (j.bolt_to_web * 1e3, j.edge_distance * 1e3, j.plate_thickness * 1e3, b.anchor_stress_area * 1e6, j.anchor_length * 1e3);
    if m <= 0.0 || e <= 0.0 || tp <= 0.0 || a_s <= 0.0 || lb <= 0.0 || b.anchor_count == 0 || j.plate_fy <= 0.0 {
        return None;
    }
    let (n, leff) = (f64::from(b.anchor_count), jc::l_eff_column_flange_single(m, e));
    let sum_ft = n * 0.9 * jt::bolt_fub(&b.anchor_class) * a_s / gm2;
    let stub = jc::t_stub_modes(leff.l1(), leff.l2(), tp, j.plate_fy / 1e6, m, jc::n_prying(e, m), 0.0, sum_ft, a_s, n, lb, jc::Mode1Method::Two, gm0);
    Some((stub, leff, sum_ft))
}

/// 🧰 Appends the compression plate bending (c-strip), tension T-stub, column base moment resistance and concrete cone checks of one foundation block; pull-out `N_Rk,p` needs the head bearing area, which the model does not carry, and is not modelled.
pub fn check_base_plate_components(b: &FoundationBlock, j: &SteelJoint, bfc: f64, annex: AnnexChoice, params: AnnexParams, gamma_c: f64, grout_ok: bool, report: &mut CheckReport) {
    let (gm0, gm2) = (params.gamma_m0, params.gamma_m2);
    let at = |field: &str, en: &str, de: &str| subject("foundationBlocks", &b.id, field, en, de);
    let joint_at = |field: &str, en: &str, de: &str| SubjectRef::new(&j.id, format!("joints[id={}].{field}", j.id), loc(en, de));
    let (t, _) = base_forces(j);
    let c_ed = {
        let (left, right) = base_flange_compression(j);
        left.max(right)
    };
    let strip = compression_strip(b, j, bfc, annex).filter(|_| grout_ok);
    if let Some(s) = strip.as_ref().filter(|_| c_ed > 0.0) {
        let (tp, bp, lp, tfc) = (j.plate_thickness * 1e3, j.plate_width * 1e3, j.base_plate_length * 1e3, j.column_flange_thickness * 1e3);
        let c_req = jc::base_plate_c_required(tfc, bfc * 1e3, bp, c_ed / s.f_jd);
        let b_req = tfc + 2.0 * c_req;
        let t_req = tp * c_req / s.c;
        let thick = b_req <= lp;
        let (ed, rd) = if thick { (t_req, tp) } else { (b_req, lp) };
        let remedy = if ed <= rd { None } else if thick { Some(Remedy::at_least(joint_at("plateThickness", "Base plate thickness", "Fußplattendicke"), length_m(tp / 1e3), length_m(t_req / 1e3), loc(&format!("Increase the base plate thickness to at least {t_req:.0} mm."), &format!("Fußplattendicke auf mindestens {t_req:.0} mm erhöhen.")))) } else { Some(Remedy::at_least(joint_at("basePlateLength", "Base plate length", "Fußplattenlänge"), length_m(lp / 1e3), length_m(b_req / 1e3), loc(&format!("Lengthen the base plate to at least {b_req:.0} mm, a thicker plate cannot widen the c-strip beyond it."), &format!("Fußplatte auf mindestens {b_req:.0} mm verlängern, eine dickere Platte kann den c-Streifen nicht darüber hinaus verbreitern.")))) };
        let mut builder = CheckResult::assess(format!("en1993.1-8.6.2.cbend.{}", b.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "6", "6.2.6.11"), at("concreteFck", "Concrete strength", "Betonfestigkeit"), loc("Base plate bending in compression: c-strip", "Fußplattenbiegung auf Druck: c-Streifen"))
            .annex(annex)
            .ndps(&[AnnexParams::GAMMA_M0, GAMMA_C])
            .utilization(length_m(ed / 1e3), length_m(rd / 1e3))
            .explanation(loc(
                &format!("Equivalent T-stub in compression: c=t_p √(f_y/(3 f_jd γ_M0))={:.1} mm with f_jd={:.1} MPa, b_eff=t_fc+2c={:.0} mm, l_eff=b_fc+2c={:.0} mm (b_fc={:.0} mm, both limited by the plate); C_Ed={:.1} kN needs b_eff≥{b_req:.0} mm, i.e. t_p≥{t_req:.1} mm (plate length {lp:.0} mm).", s.c, s.f_jd, s.b_eff, s.l_eff, bfc * 1e3, c_ed / 1e3),
                &format!("Ersatz-T-Stummel auf Druck: c=t_p √(f_y/(3 f_jd γ_M0))={:.1} mm mit f_jd={:.1} MPa, b_eff=t_fc+2c={:.0} mm, l_eff=b_fc+2c={:.0} mm (b_fc={:.0} mm, beide durch die Platte begrenzt); C_Ed={:.1} kN erfordert b_eff≥{b_req:.0} mm, also t_p≥{t_req:.1} mm (Plattenlänge {lp:.0} mm).", s.c, s.f_jd, s.b_eff, s.l_eff, bfc * 1e3, c_ed / 1e3),
            ));
        if let Some(r) = remedy {
            builder = builder.remedy(r);
        }
        report.push(builder.build());
    }
    let stub = tension_stub(b, j, gm0, gm2);
    if let Some((st, leff, sum_ft)) = stub.as_ref().filter(|_| t > 0.0) {
        let subject_t = at("anchorCount", "Anchors in tension", "Zugbeanspruchte Anker");
        report.push(force_case(
            format!("en1993.1-8.6.2.tbend.{}", b.id),
            ("6", "6.2.6.11"),
            subject_t,
            loc("Base plate bending in tension: T-stub", "Fußplattenbiegung auf Zug: T-Stummel"),
            t,
            st.ft_rd,
            annex,
            &[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2],
            loc(
                &format!("T-stub mode {} {}: l_eff,1={:.0} mm, l_eff,2={:.0} mm, F_T,1={:.1} kN, F_T,2={} kN, F_T,3=ΣF_t,Rd={:.1} kN; T_Ed={:.1} kN, F_T,Rd={:.1} kN.", st.mode, if st.prying { "with prying" } else { "without prying (L_b > L_b*)" }, leff.l1(), leff.l2(), st.ft_1 / 1e3, if st.ft_2.is_finite() { format!("{:.1}", st.ft_2 / 1e3) } else { "n/a".into() }, sum_ft / 1e3, t / 1e3, st.ft_rd / 1e3),
                &format!("T-Stummel Modus {} {}: l_eff,1={:.0} mm, l_eff,2={:.0} mm, F_T,1={:.1} kN, F_T,2={} kN, F_T,3=ΣF_t,Rd={:.1} kN; T_Ed={:.1} kN, F_T,Rd={:.1} kN.", st.mode, if st.prying { "mit Hebelkräften" } else { "ohne Hebelkräfte (L_b > L_b*)" }, leff.l1(), leff.l2(), st.ft_1 / 1e3, if st.ft_2.is_finite() { format!("{:.1}", st.ft_2 / 1e3) } else { "k. A.".into() }, sum_ft / 1e3, t / 1e3, st.ft_rd / 1e3),
            ),
        ));
    }
    if let (Some(s), Some((st, _, sum_ft)), true) = (strip.as_ref(), stub.as_ref(), j.base_moment != 0.0) {
        let ft = st.ft_rd.min(*sum_ft);
        let m_rd = jc::base_plate_m_rd(j.base_normal_force, ft, s.fc_rd, ft, s.fc_rd, j.base_lever_left * 1e3, j.base_lever_right * 1e3).map_or(0.0, |m| m / 1e3);
        let u_mj = j.base_moment.abs() / m_rd;
        let mut mjrd =
            CheckResult::assess(format!("en1993.1-8.6.2.mjrd.{}", b.id), "DIN EN 1993-1-8", ClauseId::new("EN 1993-1-8", "6", "6.2.8.3"), at("concreteFck", "Concrete strength", "Betonfestigkeit"), loc("Column base moment resistance on the block", "Momententragfähigkeit des Stützenfußes auf dem Block"))
                .annex(annex)
                .ndps(&[AnnexParams::GAMMA_M0, AnnexParams::GAMMA_M2, GAMMA_C])
                .utilization(moment_nm(j.base_moment.abs()), moment_nm(m_rd))
                .explanation(loc(
                    &format!("M_j,Rd from the plastic equilibrium of F_T,Rd={:.1} kN and F_C,Rd={:.1} kN (f_jd={:.1} MPa of the block) on both sides at N_Ed={:.1} kN: M_j,Rd={:.1} kNm against M_Ed={:.1} kNm.", ft / 1e3, s.fc_rd / 1e3, s.f_jd, j.base_normal_force / 1e3, m_rd / 1e3, j.base_moment.abs() / 1e3),
                    &format!("M_j,Rd aus dem plastischen Gleichgewicht von F_T,Rd={:.1} kN und F_C,Rd={:.1} kN (f_jd={:.1} MPa des Blocks) beidseitig bei N_Ed={:.1} kN: M_j,Rd={:.1} kNm gegen M_Ed={:.1} kNm.", ft / 1e3, s.fc_rd / 1e3, s.f_jd, j.base_normal_force / 1e3, m_rd / 1e3, j.base_moment.abs() / 1e3),
                ));
        if !(u_mj <= 1.0) {
            let k = scale(u_mj);
            mjrd = mjrd
                .remedy(Remedy::at_least(joint_at("plateThickness", "Base plate thickness", "Fußplattendicke"), length_m(j.plate_thickness), length_m(j.plate_thickness * k.sqrt()), loc(&format!("Increase the base plate thickness to about {:.0} mm (T-stub mode 1 scales with t²).", j.plate_thickness * k.sqrt() * 1e3), &format!("Fußplattendicke auf etwa {:.0} mm erhöhen (T-Stummel Modus 1 skaliert mit t²).", j.plate_thickness * k.sqrt() * 1e3))))
                .remedy(Remedy::at_least(at("anchorCount", "Anchor count", "Ankeranzahl"), dimensionless(f64::from(b.anchor_count)), dimensionless((f64::from(b.anchor_count) * k).ceil()), loc(&format!("Or use at least {:.0} anchors.", (f64::from(b.anchor_count) * k).ceil()), &format!("Oder mindestens {:.0} Anker verwenden.", (f64::from(b.anchor_count) * k).ceil()))))
                .remedy(Remedy::at_least(joint_at("baseLeverRight", "Lever arm of the compression side", "Hebelarm der Druckseite"), length_m(j.base_lever_right), length_m(j.base_lever_right * k), loc(&format!("Or enlarge the lever arm to at least {:.0} mm.", j.base_lever_right * k * 1e3), &format!("Oder den Hebelarm auf mindestens {:.0} mm vergrößern.", j.base_lever_right * k * 1e3))));
        }
        report.push(mjrd.build());
    }
    if t > 0.0 && b.anchor_count > 0 && b.anchor_embedment > 0.0 {
        let annex_number = |id: &str| semio_s_plugin_norm_compliance::ndp_number_opt(id, annex);
        if let (Some(k1), Some(inst)) = (annex_number(K1_CONE), annex_number(GAMMA_INST)) {
            let cone = cone_breakout(&ConeInput::single(b.concrete_fck / 1e6, b.anchor_embedment * 1e3, k1, b.anchor_edge_distance * 1e3));
            let (per, rd) = (t / f64::from(b.anchor_count), cone.n_rk_c / (gamma_c * inst));
            report.push(force_case(
                format!("en1993.1-8.6.2.cone.{}", b.id),
                ("7", "7.2.1.4"),
                at("anchorEmbedment", "Embedment depth", "Verankerungstiefe"),
                loc("Anchor concrete cone breakout", "Betonausbruch der Anker"),
                per,
                rd,
                annex,
                &[GAMMA_C, GAMMA_INST, K1_CONE],
                loc(
                    &format!("Cracked concrete, cone of each anchor taken as isolated (spacing ≥ 3 h_ef is not verified): N0_Rk,c=k_1 √f_ck h_ef^1.5={:.1} kN, A_c,N/A0_c,N={:.3}, ψ_s,N={:.3}, ψ_re,N={:.3}, N_Rd,c=N_Rk,c/(γ_C γ_inst)={:.1} kN against N_Ed={:.1} kN; pull-out N_Rk,p is not modelled (head bearing area missing).", cone.n0_rk_c / 1e3, cone.ratio, cone.psi_s, cone.psi_re, rd / 1e3, per / 1e3),
                    &format!("Gerissener Beton, Ausbruchkegel jedes Ankers als einzeln angenommen (Abstand ≥ 3 h_ef nicht nachgewiesen): N0_Rk,c=k_1 √f_ck h_ef^1,5={:.1} kN, A_c,N/A0_c,N={:.3}, ψ_s,N={:.3}, ψ_re,N={:.3}, N_Rd,c=N_Rk,c/(γ_C γ_inst)={:.1} kN gegen N_Ed={:.1} kN; Herausziehen N_Rk,p ist nicht abgebildet (Kopfauflagefläche fehlt).", cone.n0_rk_c / 1e3, cone.ratio, cone.psi_s, cone.psi_re, rd / 1e3, per / 1e3),
                ),
            ));
        }
    }
}
