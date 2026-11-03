//! 🔗️ EN 1999 connection checks: bolts, rivets and welds (EN 1999-1-1 §8) of one aluminium connection.

use super::*;
use super::joints;
use super::part_en1990::GoverningEffects;

//#region 🔖️ComplianceHelpers
struct Demand {
    n: f64,
    v: f64,
    m: f64,
}

fn demand(g: &GoverningEffects) -> Demand {
    Demand { n: g.n_ed, v: g.v_y_ed.hypot(g.v_z_ed), m: g.m_y_ed.hypot(g.m_z_ed) }
}

struct Bolting {
    n_bolts: f64,
    d0: f64,
    f_v_rd: f64,
    f_b_rd: f64,
    f_t_rd: f64,
    f_v_ed: f64,
    f_t_ed: f64,
    governing: usize,
    beta_lf: f64,
    rivet: bool,
}

fn bolting(conn: &AluminiumConnection, alloy: &part_1_1::AlloyProps, params: &na_de::AnnexParams, d: &Demand) -> Bolting {
    let b = &conn.bolts;
    let n_bolts = (f64::from(b.rows) * f64::from(b.bolts_per_row)).max(1.0);
    let l_j = (f64::from(b.rows) - 1.0).max(0.0) * b.pitch;
    let beta_lf = joints::long_joint_factor(l_j, b.diameter);
    let f_ub = joints::fastener_fu_pa(&b.material);
    let f_v_rd = joints::shear_resistance(&b.material, b.diameter, params.gamma_m2) * beta_lf;
    let f_b_rd = joints::bearing_resistance(b.edge_distance, b.pitch, b.edge_distance, b.diameter, b.plate_thickness, alloy.f_u_pa, f_ub, params.gamma_m2);
    let f_t_rd = joints::tension_resistance(&b.material, b.diameter, params.gamma_m2);
    let (count, arm) = if b.rows > 1 { (f64::from(b.rows), b.pitch) } else { (f64::from(b.bolts_per_row), b.gauge) };
    let per_line = n_bolts / count.max(1.0);
    let z2: f64 = (0..(count as usize)).map(|i| (i as f64 - (count - 1.0) * 0.5).powi(2) * arm * arm).sum::<f64>() * per_line;
    let forces = joints::fastener_forces(conn.force_method, b.rows, b.bolts_per_row, b.pitch, b.gauge, d.v, conn.load_eccentricity);
    let governing = joints::governing_fastener(&forces);
    let z_max = (count - 1.0) * 0.5 * arm;
    let f_m = if z2 > 0.0 { d.m * z_max / z2 } else { d.m / arm.max(1e-6) };
    Bolting {
        n_bolts,
        d0: joints::hole_diameter(b.diameter),
        f_v_rd,
        f_b_rd,
        f_t_rd,
        f_v_ed: forces.get(governing).map_or(d.v / n_bolts, joints::FastenerForce::resultant),
        f_t_ed: d.n.max(0.0) / n_bolts + f_m,
        governing,
        beta_lf,
        rivet: joints::is_rivet(&b.material),
    }
}

fn conn_subject(conn: &AluminiumConnection, path: &str) -> SubjectRef {
    SubjectRef::new(&conn.id, format!("connections[id={}].{path}", conn.id), conn_label(&conn.id))
}

fn count_remedies(conn: &AluminiumConnection, u: f64) -> Vec<Remedy> {
    if u <= 1.0 { return Vec::new(); }
    let rows = f64::from(conn.bolts.rows);
    let per = f64::from(conn.bolts.bolts_per_row);
    vec![
        Remedy::at_least(conn_subject(conn, "bolts.rows"), Quantity::new(QuantityKind::Dimensionless, rows), Quantity::new(QuantityKind::Dimensionless, (rows * u * 1.01).ceil().max(rows + 1.0)), loc("Add bolt rows.", "Schraubenreihen ergänzen.")),
        Remedy::at_least(conn_subject(conn, "bolts.boltsPerRow"), Quantity::new(QuantityKind::Dimensionless, per), Quantity::new(QuantityKind::Dimensionless, (per * u * 1.01).ceil().max(per + 1.0)), loc("Add bolts per row.", "Schrauben je Reihe ergänzen.")),
    ]
}

fn emit(out: &mut Vec<CheckResult>, conn: &AluminiumConnection, annex: AnnexChoice, g: &GoverningEffects, id: &str, sec: &str, title: (&str, &str), path: &str, computed: Quantity, limit: Quantity, expl: String, mut remedies: Vec<Remedy>) {
    let u = if limit.value > 0.0 { computed.value.abs() / limit.value } else { 0.0 };
    remedies.extend(leaf_remedies("connections", &conn.id, &conn.actions, u, &conn_label(&conn.id), "Reduce the connection load", "Anschlusslast senken", if id.ends_with("tension") || id.ends_with("punch") { "nK" } else { "vZK" }));
    out.push(utilization_check(
        format!("{id}.{}", conn.id),
        "DIN EN 1999-1-1",
        clause(sec),
        conn_subject(conn, path),
        loc(title.0, title.1),
        computed,
        limit,
        annex,
        loc(&format!("{expl} (ULS {}, lead {}); {}", g.combination, g.action_id, actions_digest(&conn.actions)), &format!("{expl} (GZT {}, führend {}); {}", g.combination, g.action_id, actions_digest(&conn.actions))),
        remedies,
    ));
}

fn bolted_checks(conn: &AluminiumConnection, annex: AnnexChoice, params: &na_de::AnnexParams, alloy: &part_1_1::AlloyProps, g: &GoverningEffects, out: &mut Vec<CheckResult>) {
    let d = demand(g);
    let bt = bolting(conn, alloy, params, &d);
    let b = &conn.bolts;
    if joints::fastener_fu_pa(&b.material) <= 0.0 {
        let subject = conn_subject(conn, "bolts.material");
        out.push(
            CheckResult::assess(format!("en1999.8.5.grade.{}", conn.id), "DIN EN 1999-1-1", clause("8.5"), subject.clone(), loc("Fastener grade", "Verbindungsmittelgüte"))
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(&format!("Unknown fastener grade '{}'.", b.material), &format!("Unbekannte Verbindungsmittelgüte '{}'.", b.material)))
                .annex(annex)
                .remedy(Remedy::one_of(subject, joints::FASTENER_GRADES.iter().map(|s| (*s).to_string()).collect(), loc("Pick a supported fastener grade.", "Unterstützte Verbindungsmittelgüte wählen.")))
                .build(),
        );
        return;
    }
    let f_rd = bt.f_v_rd.min(bt.f_b_rd);
    let u = bt.f_v_ed / f_rd.max(1e-9);
    let rem = count_remedies(conn, u);
    emit(out, conn, annex, g, "en1999.8.5.bolt", "8.5", ("Bolt shear and bearing", "Schrauben Abscheren und Lochleibung"), "bolts.rows",
        q_force(bt.f_v_ed), q_force(f_rd.max(1e-9)),
        format!("F_v,Ed,max = {:.2} kN (i = {}), V/n = {:.2} kN, e = {:.0} mm ({}), F_v,Rd = {:.2} kN (β_Lf = {:.3}), F_b,Rd = {:.2} kN, n = {}", bt.f_v_ed / 1e3, bt.governing + 1, d.v / bt.n_bolts / 1e3, conn.load_eccentricity * 1e3, conn.force_method.code(), bt.f_v_rd / 1e3, bt.beta_lf, bt.f_b_rd / 1e3, bt.n_bolts), rem);
    if !bt.rivet {
        let u_t = bt.f_t_ed / bt.f_t_rd.max(1e-9);
        emit(out, conn, annex, g, "en1999.8.5.tension", "8.5", ("Bolt tension", "Schrauben Zug"), "bolts.rows", q_force(bt.f_t_ed), q_force(bt.f_t_rd.max(1e-9)),
            format!("F_t,Ed = {:.2} kN per bolt, F_t,Rd = {:.2} kN", bt.f_t_ed / 1e3, bt.f_t_rd / 1e3), count_remedies(conn, u_t));
        let u_vt = bt.f_v_ed / f_rd.max(1e-9) + bt.f_t_ed / (1.4 * bt.f_t_rd.max(1e-9));
        emit(out, conn, annex, g, "en1999.8.5.vt", "8.5", ("Bolt shear and tension interaction", "Schrauben Interaktion Abscheren und Zug"), "bolts.rows", q_dim(u_vt), q_dim(1.0),
            format!("F_v,Ed/F_Rd + F_t,Ed/(1.4·F_t,Rd) = {u_vt:.3}"), count_remedies(conn, u_vt));
        let b_p = joints::punching_resistance(b.diameter, b.plate_thickness, alloy.f_u_pa, params.gamma_m2);
        let u_p = bt.f_t_ed / b_p.max(1e-9);
        emit(out, conn, annex, g, "en1999.8.5.punch", "8.5", ("Punching shear B_p,Rd", "Durchstanzen B_p,Rd"), "bolts.plateThickness", q_force(bt.f_t_ed), q_force(b_p.max(1e-9)),
            format!("B_p,Rd = {:.2} kN (d_m = 1.5·d)", b_p / 1e3),
            if u_p > 1.0 { vec![Remedy::at_least(conn_subject(conn, "bolts.plateThickness"), q_length(b.plate_thickness), q_length(b.plate_thickness * u_p * 1.01), loc("Increase the plate thickness.", "Blechdicke erhöhen."))] } else { Vec::new() });
    }
    let across = b.bolts_per_row > 1;
    let spacing: [(&str, &str, &str, &str, f64, f64, bool, bool); 4] = [
        ("pitch", "Pitch p1 ≥ 2.2·d0", "Teilung p1 ≥ 2,2·d0", "bolts.pitch", b.pitch, joints::MIN_PITCH_D0 * bt.d0, true, true),
        ("edge", "Edge distances e1, e2 ≥ 1.2·d0", "Randabstände e1, e2 ≥ 1,2·d0", "bolts.edgeDistance", b.edge_distance, joints::MIN_END_EDGE_D0 * bt.d0, true, true),
        ("gauge", "Spacing p2 ≥ {}·d0", "Lochabstand p2 ≥ {}·d0", "bolts.gauge", b.gauge, 0.0, true, across),
        ("emax", "Edge distances e1, e2 ≤ 4·t + 40 mm", "Randabstände e1, e2 ≤ 4·t + 40 mm", "bolts.edgeDistance", b.edge_distance, 4.0 * b.plate_thickness + 0.040, false, true),
    ];
    for (suffix, en, de, path, actual, bound, is_min, applies) in spacing {
        let p2 = (suffix == "gauge").then(joints::min_spacing_d0);
        let bound = p2.map_or(bound, |factor| factor * bt.d0);
        let (en, de) = p2.map_or((en.to_owned(), de.to_owned()), |factor| (en.replace("{}", &factor.to_string()), de.replace("{}", &factor.to_string().replace('.', ","))));
        let (en, de) = (en.as_str(), de.as_str());
        let util = if !applies { 0.0 } else if is_min { bound / actual.max(1e-9) } else { actual / bound.max(1e-9) };
        let mut result = CheckResult::assess(format!("en1999.8.5.{suffix}.{}", conn.id), "DIN EN 1999-1-1", clause("8.5"), conn_subject(conn, path), loc(en, de))
            .utilization(q_dim(util), q_dim(1.0))
            .annex(annex)
            .explanation(loc(&format!("{en}: {:.1} mm against {:.1} mm (d0 = {:.1} mm)", actual * 1e3, bound * 1e3, bt.d0 * 1e3), &format!("{de}: {:.1} mm gegenüber {:.1} mm (d0 = {:.1} mm)", actual * 1e3, bound * 1e3, bt.d0 * 1e3)));
        if util > 1.0 {
            result = result.remedy(if is_min {
                Remedy::at_least(conn_subject(conn, path), q_length(actual), q_length(bound), loc("Increase the distance.", "Abstand vergrößern."))
            } else {
                Remedy::at_most(conn_subject(conn, path), q_length(actual), q_length(bound), loc("Reduce the distance.", "Abstand verkleinern."))
            });
        }
        out.push(result.ndp_reads().build());
    }
    let p_max = (14.0 * b.plate_thickness).min(0.200);
    if b.rows > 1 {
        let u_p = b.pitch / p_max.max(1e-9);
        let mut c = CheckResult::assess(format!("en1999.8.5.pmax.{}", conn.id), "DIN EN 1999-1-1", clause("8.5"), conn_subject(conn, "bolts.pitch"), loc("Pitch p1 ≤ min(14·t, 200 mm)", "Teilung p1 ≤ min(14·t, 200 mm)"))
            .utilization(q_dim(u_p), q_dim(1.0))
            .annex(annex)
            .explanation(loc(&format!("Pitch p1 = {:.1} mm, maximum p1,max = {:.1} mm", b.pitch * 1e3, p_max * 1e3), &format!("Teilung p1 = {:.1} mm, Höchstwert p1,max = {:.1} mm", b.pitch * 1e3, p_max * 1e3)));
        if u_p > 1.0 { c = c.remedy(Remedy::at_most(conn_subject(conn, "bolts.pitch"), q_length(b.pitch), q_length(p_max), loc("Reduce the pitch.", "Teilung verkleinern."))); }
        out.push(c.ndp_reads().build());
    }
    let block = joints::block_tearing(b.rows, b.edge_distance, b.pitch, b.edge_distance, b.diameter, b.plate_thickness, alloy.f_u_pa, alloy.f_o_pa, params.gamma_m1, params.gamma_m2);
    let u_blk = d.v / block.max(1e-9);
    emit(out, conn, annex, g, "en1999.8.5.block", "8.5.2.3", ("Block tearing", "Blockversagen"), "bolts.edgeDistance", q_force(d.v), q_force(block.max(1e-9)),
        format!("V_eff,Rd = {:.2} kN (single line, concentric)", block / 1e3),
        if u_blk > 1.0 { vec![Remedy::at_least(conn_subject(conn, "bolts.edgeDistance"), q_length(b.edge_distance), q_length(b.edge_distance * u_blk * 1.1), loc("Increase the edge distance.", "Randabstand vergrößern."))] } else { Vec::new() });
    let width = joints::plate_width(b.bolts_per_row, b.gauge, b.edge_distance);
    let net = joints::net_width(width, bt.d0, b.bolts_per_row, b.gauge, conn.hole_stagger);
    let (n_o_rd, n_u_rd) = joints::plate_tension_resistances(width, net, b.plate_thickness, alloy.f_o_pa, alloy.f_u_pa, params.gamma_m1, params.gamma_m2);
    let n_rd = n_o_rd.min(n_u_rd).max(1e-9);
    let u_net = d.v / n_rd;
    let mut net_remedies = Vec::new();
    if u_net > 1.0 {
        let wider = (u_net * 1.01 - 1.0) * net / 2.0;
        net_remedies.push(Remedy::at_least(conn_subject(conn, "bolts.plateThickness"), q_length(b.plate_thickness), q_length(b.plate_thickness * u_net * 1.01), loc("Increase the plate thickness.", "Blechdicke erhöhen.")));
        net_remedies.push(Remedy::at_least(conn_subject(conn, "bolts.edgeDistance"), q_length(b.edge_distance), q_length(b.edge_distance + wider.max(1e-3)), loc("Increase the edge distance (wider plate).", "Randabstand vergrößern (breiteres Blech).")));
    }
    emit(out, conn, annex, g, "en1999.8.5.net", "6.2.3", ("Plate tension at the bolt lines (gross yield, net section)", "Blechzug an den Schraubenlinien (Brutto-Fließen, Nettoquerschnitt)"), "bolts.plateThickness", q_force(d.v), q_force(n_rd),
        format!("b = {:.1} mm, b_net = {:.2} mm (n = {} lines, d0 = {:.1} mm, s = {:.1} mm), N_o,Rd = {:.2} kN, N_u,Rd = 0.9·A_net·f_u/γ_M2 = {:.2} kN (A_net = {:.1} mm²)", width * 1e3, net * 1e3, b.bolts_per_row, bt.d0 * 1e3, conn.hole_stagger * 1e3, n_o_rd / 1e3, n_u_rd / 1e3, net * b.plate_thickness * 1e6), net_remedies);
}

fn pinned_checks(conn: &AluminiumConnection, annex: AnnexChoice, params: &na_de::AnnexParams, alloy: &part_1_1::AlloyProps, g: &GoverningEffects, out: &mut Vec<CheckResult>) {
    let d = demand(g);
    let b = &conn.bolts;
    if joints::fastener_fu_pa(&b.material) <= 0.0 {
        let subject = conn_subject(conn, "bolts.material");
        out.push(
            CheckResult::assess(format!("en1999.8.5.grade.{}", conn.id), "DIN EN 1999-1-1", clause("8.5"), subject.clone(), loc("Pin grade", "Bolzengüte"))
                .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
                .explanation(loc(&format!("Unknown pin grade '{}'.", b.material), &format!("Unbekannte Bolzengüte '{}'.", b.material)))
                .annex(annex)
                .remedy(Remedy::one_of(subject, joints::FASTENER_GRADES.iter().map(|s| (*s).to_string()).collect(), loc("Pick a supported pin grade.", "Unterstützte Bolzengüte wählen.")))
                .build(),
        );
        return;
    }
    let n_pins = (f64::from(b.rows) * f64::from(b.bolts_per_row)).max(1.0);
    let f_ed = d.v.hypot(d.n.abs()) / n_pins;
    let f_v_ed = f_ed * 0.5;
    let f_v_rd = joints::pin_shear_resistance(&b.material, b.diameter, params.gamma_m2);
    let f_b_rd = joints::pin_bearing_resistance(b.plate_thickness, b.diameter, alloy.f_o_pa, params.gamma_m1);
    let m_ed = joints::pin_bending_moment(f_ed, b.plate_thickness);
    let m_rd = joints::pin_bending_resistance(&b.material, b.diameter, params.gamma_m2);
    let diameter_remedy = |u: f64| if u > 1.0 { vec![Remedy::at_least(conn_subject(conn, "bolts.diameter"), q_length(b.diameter), q_length(b.diameter * u.sqrt() * 1.01), loc("Increase the pin diameter.", "Bolzendurchmesser erhöhen."))] } else { Vec::new() };
    let u_v = f_v_ed / f_v_rd.max(1e-9);
    emit(out, conn, annex, g, "en1999.8.5.pin.shear", "8.5", ("Pin shear (double shear)", "Bolzen Abscheren (zweischnittig)"), "bolts.diameter", q_force(f_v_ed), q_force(f_v_rd.max(1e-9)),
        format!("F_v,Ed = {:.2} kN per plane, F_v,Rd = {:.2} kN, {} pin(s)", f_v_ed / 1e3, f_v_rd / 1e3, n_pins), diameter_remedy(u_v));
    let u_b = f_ed / f_b_rd.max(1e-9);
    let mut rem_b = diameter_remedy(u_b);
    if u_b > 1.0 { rem_b.push(Remedy::at_least(conn_subject(conn, "bolts.plateThickness"), q_length(b.plate_thickness), q_length(b.plate_thickness * u_b * 1.01), loc("Increase the plate thickness.", "Blechdicke erhöhen."))); }
    emit(out, conn, annex, g, "en1999.8.5.pin.bearing", "8.5", ("Plate bearing at the pin", "Lochleibung am Bolzen"), "bolts.plateThickness", q_force(f_ed), q_force(f_b_rd.max(1e-9)),
        format!("F_Ed = {:.2} kN per pin, F_b,Rd = 1.5·t·d·f_o/γ_M1 = {:.2} kN", f_ed / 1e3, f_b_rd / 1e3), rem_b);
    let u_m = m_ed / m_rd.max(1e-9);
    emit(out, conn, annex, g, "en1999.8.5.pin.bend", "8.5", ("Pin bending", "Bolzen Biegung"), "bolts.diameter", q_moment(m_ed), q_moment(m_rd.max(1e-9)),
        format!("M_Ed = F_Ed·b/8 = {:.1} N·m, M_Rd = {:.1} N·m", m_ed, m_rd), diameter_remedy(u_m));
    let u_i = u_m * u_m + u_v * u_v;
    emit(out, conn, annex, g, "en1999.8.5.pin.int", "8.5", ("Pin shear and bending interaction", "Bolzen Interaktion Abscheren und Biegung"), "bolts.diameter", q_dim(u_i), q_dim(1.0),
        format!("(M_Ed/M_Rd)² + (F_v,Ed/F_v,Rd)² = {u_i:.3}"), diameter_remedy(u_i.sqrt()));
}

fn welded_checks(conn: &AluminiumConnection, annex: AnnexChoice, params: &na_de::AnnexParams, alloy: &part_1_1::AlloyProps, g: &GoverningEffects, designation: &str, out: &mut Vec<CheckResult>) {
    let d = demand(g);
    let w = &conn.welds;
    let parent_t = conn.bolts.plate_thickness.max(w.throat / 0.7);
    let band = part_1_1::resolve_alloy_for_thickness(designation, parent_t).unwrap_or(*alloy);
    let f_w = joints::filler_fw_pa(&w.filler_alloy, band.f_u_pa);
    let (metal, haz) = joints::weld_resistance_per_length(w.throat, w.beta_w, f_w, band.f_u_pa, band.rho_u_haz, params.gamma_mw, params.gamma_m2);
    let rd = metal.min(haz);
    let len = w.length.max(1e-9);
    let f_ed = (d.n.abs() / len + 6.0 * d.m / (len * len)).hypot(d.v / len);
    let b_haz = if w.haz_extent > 0.0 { w.haz_extent } else { part_1_1::haz_width(parent_t) };
    let u = f_ed / rd.max(1e-9);
    let mut rem = Vec::new();
    if u > 1.0 {
        rem.push(Remedy::at_least(conn_subject(conn, "welds.throat"), q_length(w.throat), q_length(w.throat * u * 1.01), loc("Increase the weld throat.", "Nahtdicke erhöhen.")));
        rem.push(Remedy::at_least(conn_subject(conn, "welds.length"), q_length(w.length), q_length(w.length * u * 1.05), loc("Increase the weld length.", "Nahtlänge erhöhen.")));
    }
    emit(out, conn, annex, g, "en1999.8.6.weld", "8.6", ("Fillet weld resistance (metal and HAZ)", "Kehlnahtwiderstand (Schweißgut und WEZ)"), "welds.throat",
        Quantity::new(QuantityKind::Force, f_ed), Quantity::new(QuantityKind::Force, rd),
        format!("f_Ed = {:.1} N/mm, weld metal {:.1} N/mm (f_w = {:.0} MPa, β_w = {:.2}, γ_Mw = {:.2}), HAZ boundary {:.1} N/mm (ρ_u,haz = {:.2}), b_haz = {:.0} mm", f_ed / 1e3, metal / 1e3, f_w / 1e6, w.beta_w, params.gamma_mw, haz / 1e3, band.rho_u_haz, b_haz * 1e3), rem);
    let t_p = conn.bolts.plate_thickness;
    let longitudinal = w.haz_extent > 0.0;
    let (a_o, a_u) = if longitudinal {
        let a_g = t_p * len;
        let a_haz = (2.0 * b_haz * t_p).min(a_g);
        (a_g - a_haz * (1.0 - band.rho_o_haz), a_g - a_haz * (1.0 - band.rho_u_haz))
    } else {
        (band.rho_o_haz * t_p * len, band.rho_u_haz * t_p * len)
    };
    let n_haz = (a_o * band.f_o_pa / params.gamma_m1).min(a_u * band.f_u_pa / params.gamma_m2);
    let f_haz = d.n.abs() + d.v;
    let mut rem_haz = Vec::new();
    if f_haz > n_haz {
        rem_haz.push(Remedy::at_least(conn_subject(conn, "bolts.plateThickness"), q_length(conn.bolts.plate_thickness), q_length(conn.bolts.plate_thickness * f_haz / n_haz.max(1e-9) * 1.01), loc("Increase the connected plate thickness.", "Dicke des angeschlossenen Blechs erhöhen.")));
    }
    emit(out, conn, annex, g, "en1999.8.6.haz", "8.6", ("Connected plate in the heat-affected zone", "Angeschlossenes Blech in der Wärmeeinflusszone"), "bolts.plateThickness", q_force(f_haz), q_force(n_haz),
        format!("N_haz,Rd = min(A_o·f_o/γ_M1, A_u·f_u/γ_M2) = {:.2} kN, {} weld ({}), b_haz = {:.0} mm, ρ_o,haz = {:.2}, ρ_u,haz = {:.2} (plate width taken as the weld length)", n_haz / 1e3, if longitudinal { "longitudinal" } else { "transverse" }, if longitudinal { "HAZ strips on both sides" } else { "full width in the HAZ" }, b_haz * 1e3, band.rho_o_haz, band.rho_u_haz), rem_haz);
    let l_min = joints::min_weld_length(w.throat);
    let u_l = l_min / len;
    let mut c = CheckResult::assess(format!("en1999.8.6.length.{}", conn.id), "DIN EN 1999-1-1", clause("8.6"), conn_subject(conn, "welds.length"), loc("Minimum weld length max(8·a, 40 mm)", "Mindestnahtlänge max(8·a, 40 mm)"))
        .utilization(q_dim(u_l), q_dim(1.0))
        .annex(annex)
        .explanation(loc(&format!("Weld length L = {:.0} mm, minimum L,min = {:.0} mm", len * 1e3, l_min * 1e3), &format!("Nahtlänge L = {:.0} mm, Mindestwert L,min = {:.0} mm", len * 1e3, l_min * 1e3)));
    if u_l > 1.0 { c = c.remedy(Remedy::at_least(conn_subject(conn, "welds.length"), q_length(w.length), q_length(l_min), loc("Lengthen the weld.", "Naht verlängern."))); }
    out.push(c.ndps(&[na_de::AnnexParams::GAMMA_M1, na_de::AnnexParams::GAMMA_M2, na_de::AnnexParams::GAMMA_MW]).ndp_reads().build());
}

fn connection_sls(conn: &AluminiumConnection, annex: AnnexChoice, params: &na_de::AnnexParams, alloy: &part_1_1::AlloyProps, kind: semio_s_artifact_norm_en1999_model::ConnectionKind, g: &GoverningEffects) -> Option<CheckResult> {
    use semio_s_artifact_norm_en1999_model::ConnectionKind as K;
    let d = demand(g);
    let mut cap = f64::INFINITY;
    if matches!(kind, K::Bolted | K::Combined) {
        let bt = bolting(conn, alloy, params, &d);
        cap = cap.min(bt.n_bolts * bt.f_v_rd.min(bt.f_b_rd));
    }
    if kind == K::Pinned {
        let n_pins = (f64::from(conn.bolts.rows) * f64::from(conn.bolts.bolts_per_row)).max(1.0);
        cap = cap.min(2.0 * n_pins * joints::pin_shear_resistance(&conn.bolts.material, conn.bolts.diameter, params.gamma_m2));
    }
    if matches!(kind, K::Welded | K::Combined) {
        let w = &conn.welds;
        let f_w = joints::filler_fw_pa(&w.filler_alloy, alloy.f_u_pa);
        let (metal, haz) = joints::weld_resistance_per_length(w.throat, w.beta_w, f_w, alloy.f_u_pa, alloy.rho_u_haz, params.gamma_mw, params.gamma_m2);
        cap = cap.min(metal.min(haz) * w.length);
    }
    if !cap.is_finite() { return None; }
    let f_ed = d.v + d.n.abs();
    let u = f_ed / cap.max(1e-9);
    let mut rem = leaf_remedies("connections", &conn.id, &conn.actions, u, &conn_label(&conn.id), "Reduce the connection load", "Anschlusslast senken", "vZK");
    rem.extend(count_remedies(conn, u));
    Some(utilization_check(
        format!("en1999.8.sls-freq.{}", conn.id), "DIN EN 1999-1-1", ClauseId::new("EN 1990", "§6.5.3", "6.5.3"), conn_subject(conn, "bolts.rows"),
        loc("Connection under the frequent combination", "Anschluss unter häufiger Kombination"),
        q_force(f_ed), q_force(cap), annex,
        loc(&format!("F_Ed,freq = {:.2} kN ≤ {:.2} kN (SLS {}, lead {}); {}", f_ed / 1e3, cap / 1e3, g.combination, g.action_id, actions_digest(&conn.actions)),
            &format!("F_Ed,freq = {:.2} kN ≤ {:.2} kN (GZG {}, führend {}); {}", f_ed / 1e3, cap / 1e3, g.combination, g.action_id, actions_digest(&conn.actions))),
        rem,
    ))
}

pub fn check_connection(conn: &AluminiumConnection, material: &AluminiumMaterial, member: Option<&AluminiumMember>, member_ids: &[String], annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy(&material.designation) else {
        return vec![alloy_unknown(format!("en1999.3.2.alloy.{}", conn.id), &conn.id, material, annex)];
    };
    let Some(member) = member else {
        return vec![CheckResult::assess(
            format!("en1999.8.ref.{}", conn.id), "DIN EN 1999-1-1", ClauseId::new("EN 1999-1-1", "§8", "8"),
            SubjectRef::new(&conn.id, format!("connections[id={}].memberId", conn.id), loc("Connection", "Anschluss")),
            loc("Connection member reference", "Anschluss-Bauteilverweis"),
        )
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(
            &format!("Connection '{}' references missing member '{}'.", conn.id, conn.member_id),
            &format!("Anschluss '{}' verweist auf fehlendes Bauteil '{}'.", conn.id, conn.member_id),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(
            SubjectRef::new(&conn.id, format!("connections[id={}].memberId", conn.id), conn_label(&conn.id)),
            member_ids.to_vec(),
            loc("Point connection.memberId at an existing member.", "connections.memberId auf ein vorhandenes Bauteil setzen."),
        ))
        .build()];
    };
    let alloy = part_1_1::resolve_alloy_for_thickness(&material.designation, conn.bolts.plate_thickness).unwrap_or(alloy);
    use semio_s_artifact_norm_en1999_model::ConnectionKind as K;
    let kind = conn.kind;
    let uls: Vec<GoverningEffects> = if conn.actions.is_empty() {
        part_en1990::uls_combinations(member, &params, reliability_class)
    } else {
        part_en1990::uls_combinations(&part_en1990::connection_proxy(conn), &params, reliability_class)
    };
    let mut sets = Vec::new();
    for g in &uls {
        let mut out = Vec::new();
        if matches!(kind, K::Bolted | K::Combined) { bolted_checks(conn, annex, &params, &alloy, g, &mut out); }
        if kind == K::Pinned { pinned_checks(conn, annex, &params, &alloy, g, &mut out); }
        if matches!(kind, K::Welded | K::Combined) { welded_checks(conn, annex, &params, &alloy, g, &material.designation, &mut out); }
        sets.push(out);
    }
    let sls_source = if conn.actions.is_empty() { member.clone() } else { part_en1990::connection_proxy(conn) };
    for g in part_en1990::sls_combinations(&sls_source, "frequent") {
        if let Some(c) = connection_sls(conn, annex, &params, &alloy, conn.kind, &g) { sets.push(vec![c]); }
    }
    worst_by_id(sets)
}
//#endregion 🔖️ComplianceHelpers
