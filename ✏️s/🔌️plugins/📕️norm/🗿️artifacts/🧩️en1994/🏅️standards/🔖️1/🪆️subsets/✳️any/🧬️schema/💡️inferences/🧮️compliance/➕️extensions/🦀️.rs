//! ➕️ Extension checks: equivalent span per span position (C-03), non-linear bending resistance M-κ (C-10), patch loading (C-24), load introduction (C-31) and LM1 bridge actions (C-48).

use super::*;

fn bref(beam: &CompositeBeam, leaf: &str) -> SubjectRef {
    SubjectRef::new(beam.id.clone(), format!("beams[id={}].{leaf}", beam.id), lc(format!("Composite beam {}", beam.id), format!("Verbundträger {}", beam.id)))
}

fn cref(col: &CompositeColumn, leaf: &str) -> SubjectRef {
    SubjectRef::new(col.id.clone(), format!("columns[id={}].{leaf}", col.id), lc(format!("Column {}", col.id), format!("Stütze {}", col.id)))
}

fn ratio(demand: f64, capacity: f64) -> f64 {
    if capacity > 1e-12 { demand / capacity } else if demand > 0.0 { f64::INFINITY } else { 0.0 }
}

fn span_label(position: cat::SpanPosition) -> (&'static str, &'static str) {
    match position {
        cat::SpanPosition::Simple => ("simple span", "Einfeldträger"),
        cat::SpanPosition::End => ("end span", "Endfeld"),
        cat::SpanPosition::Interior => ("interior span", "Innenfeld"),
        cat::SpanPosition::Cantilever => ("cantilever", "Kragarm"),
    }
}

/// 🧩 The extension checks of one composite beam (reads the snapshot only).
pub(crate) fn beam_extensions(report: &mut CheckReport, doc: &En1994Snapshot, beam_in: &CompositeBeam) {
    let annex = doc.annex;
    let p = AnnexParams::for_annex(annex);
    let mut beam = beam_in.clone();
    beam.steel = SteelSection::resolve(&beam.steel);
    let sec = part_1_1::isection(&beam.steel);
    let fyd = doc.steel_f_y_pa / p.gamma_m0;
    let fcc = cat::fcc_beam_pa(beam.concrete_f_ck_pa, p.gamma_c);
    let hc = part_1_1::concrete_height_m(&beam);
    let ys = beam.slab_thickness_m;
    let le = cat::equivalent_span_m(beam.support, beam.span_m, false);
    let le_h = if beam.has_hogging() { cat::le_hogging_m(beam.span_m, beam.neighbour_span_m()) } else { beam.span_m };
    let half = beam.spacing_m / 2.0;
    let beff = cat::effective_width_m(le, 0.0, half, half, false);
    let uls = part_en1990::uls_composite(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, annex, doc.reliability_class);
    let b = &beam;

    let position = cat::span_position(b.support);
    let ratio = cat::adjacent_span_ratio(&[b.span_m, b.neighbour_span_m()]);
    let (pos_en, pos_de) = span_label(position);
    let mut le_check = head(annex, format!("en1994.5.4.1.2.le.{}", b.id), "1994-1-1", "5.4.1.2", bref(b, "support"), "Equivalent span L_e for the span position", "Ersatzstützweite L_e für die Feldlage")
        .utilization(dim(if b.has_hogging() { 0.6 / ratio.max(1e-9) } else { 0.0 }), dim(1.0))
        .explanation(lc(
            format!("{pos_en}: L_e,sagging = {le:.2} m, L_e,hogging = {le_h:.2} m (Fig. 5.1); neighbouring span {:.2} m, span ratio {ratio:.2}.", b.neighbour_span_m()),
            format!("{pos_de}: L_e,Feld = {le:.2} m, L_e,Stütze = {le_h:.2} m (Bild 5.1); Nachbarfeld {:.2} m, Feldlängenverhältnis {ratio:.2}.", b.neighbour_span_m()),
        ));
    if b.has_hogging() && ratio < 0.6 {
        le_check = le_check.status(CheckStatus::Warning);
    }
    report.push(le_check.build());

    let m_ed = uls.m_nm;
    let inp = cat::NonlinearInput::standard(fyd, fcc, 0.0, 0.0);
    let m_nl = cat::nonlinear_mrd(&sec, ys, beff, hc, &inp);
    let kappa_u = cat::ultimate_curvature(&sec, ys, beff, hc, &inp);
    let m_pl = cat::plastic_full(&sec, fyd, ys, beff, hc, fcc).m_nm;
    let mut mk = head(annex, format!("en1994.6.2.1.4.mkappa.{}", b.id), "1994-1-1", "6.2.1.4", bref(b, "actions"), "Non-linear bending resistance (M-κ)", "Nichtlineare Biegetragfähigkeit (M-κ)")
        .utilization(moment(m_ed), moment(m_nl))
        .explanation(lc(
            format!("Strain-compatible M-κ: M_Rd,nl = {:.1} kNm at κ_u = {:.2} 1/km (ε_cu = 3.5 ‰, ε_su = 20 ‰); plastic M_pl,Rd = {:.1} kNm, ratio {:.3}.", m_nl / 1e3, kappa_u * 1e3, m_pl / 1e3, ratio(m_nl, m_pl)),
            format!("Dehnungskompatible M-κ: M_Rd,nl = {:.1} kNm bei κ_u = {:.2} 1/km (ε_cu = 3,5 ‰, ε_su = 20 ‰); plastisch M_pl,Rd = {:.1} kNm, Verhältnis {:.3}.", m_nl / 1e3, kappa_u * 1e3, m_pl / 1e3, ratio(m_nl, m_pl)),
        ));
    if m_ed > m_nl {
        mk = mk.remedy(Remedy::at_least(bref(b, "slabThicknessM"), length(b.slab_thickness_m), length((b.slab_thickness_m * m_ed / m_nl.max(1.0)).min(0.30)), lc("Increase slab thickness.", "Plattendicke erhöhen.")));
    }
    report.push(mk.build());

    let f_ed: f64 = b.actions.iter().filter(|x| x.kind != semio_s_artifact_norm_en1994_model::ActionKind::Fatigue && x.f_k_n.abs() > 1e-9).map(|x| x.f_k_n.abs() * if x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent { part_en1990::gamma_g(annex, doc.reliability_class) } else { part_en1990::gamma_q(annex, doc.reliability_class) }).sum();
    if f_ed > 0.0 {
        let bearing = if b.patch_bearing_length_m > 1e-9 { b.patch_bearing_length_m } else { cat::constants::PATCH_BEARING_LENGTH_M };
        let patch = |section: &SteelSection| {
            let s = part_1_1::isection(section);
            cat::patch_loading(s.hw(), section.tw_m, section.width_m, section.tf_m, doc.steel_f_y_pa, doc.steel_f_y_pa, bearing, b.ltb_length_m.max(s.hw()), p.gamma_m1)
        };
        let (f_rd, chi, ly, lam) = patch(&b.steel);
        let mut pl = head(annex, format!("en1994.6.4.patch.{}", b.id), "1993-1-5", "6", bref(b, "steel.designation"), "Patch loading of the steel web", "Konzentrierte Lasteinleitung in den Stahlsteg")
            .utilization(force(f_ed), force(f_rd))
            .explanation(lc(
                format!("F_Ed = {:.1} kN, F_Rd = χ_F·ℓ_y·t_w·f_yw/γ_M1 = {:.1} kN (χ_F = {chi:.3}, ℓ_y = {ly:.3} m, λ_F = {lam:.3}); bearing length {:.0} mm and stiffener spacing = lateral restraint spacing assumed.", f_ed / 1e3, f_rd / 1e3, bearing * 1e3),
                format!("F_Ed = {:.1} kN, F_Rd = χ_F·ℓ_y·t_w·f_yw/γ_M1 = {:.1} kN (χ_F = {chi:.3}, ℓ_y = {ly:.3} m, λ_F = {lam:.3}); Lasteinleitungslänge {:.0} mm und Steifenabstand = Abstand der seitlichen Halterungen angenommen.", f_ed / 1e3, f_rd / 1e3, bearing * 1e3),
            ));
        if f_ed > f_rd {
            let clearing: Vec<String> = SteelSection::heavier_heb_options(&b.steel.designation).into_iter().filter(|d| SteelSection::from_catalogue(d).is_some_and(|s| patch(&s).0 >= f_ed)).collect();
            let options = if clearing.is_empty() { SteelSection::heavier_heb_options(&b.steel.designation) } else { clearing };
            if !options.is_empty() {
                pl = pl.remedy(Remedy::one_of(bref(b, "steel.designation"), options, lc("Select a heavier steel section or add web stiffeners.", "Schwereren Stahlquerschnitt wählen oder Stegsteifen anordnen.")));
            }
        }
        report.push(pl.build());
    }

    if doc.structure_kind == semio_s_artifact_norm_en1994_model::StructureKind::Bridge {
        let lanes = if b.bridge_lanes > 0 { (b.bridge_lanes as usize).clamp(1, 3) } else { ((b.spacing_m / cat::LANE_WIDTH_M).floor() as usize).clamp(1, 3) };
        let (mut m_lm, mut v_lm) = (0.0, 0.0);
        for lane in 1..=lanes {
            let (axle, udl) = cat::lm1_lane(annex == AnnexChoice::De, lane);
            let (m, v) = cat::lm1_simple_span_effects(b.span_m, axle, udl);
            m_lm += m;
            v_lm += v;
        }
        let permanent: Vec<CharacteristicAction> = b.actions.iter().filter(|x| x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent).cloned().collect();
        let g = part_en1990::uls_composite(&permanent, b.span_m, b.adjacent_span_m, b.support, b.spacing_m, annex, doc.reliability_class);
        let gamma_q = part_en1990::gamma_q(annex, doc.reliability_class);
        let m_demand = g.m_nm + gamma_q * m_lm;
        let m_rd = cat::plastic_full(&sec, fyd, ys, beff, hc, fcc).m_nm;
        let v_demand = g.v_n + gamma_q * v_lm;
        let v_rd = b.steel.a_v_m2 * fyd / 3f64.sqrt();
        let util = ratio(m_demand, m_rd).max(ratio(v_demand, v_rd));
        let mut lm = head(annex, format!("en1994.2.lm1.{}", b.id), "1991-2", "4.3.2", bref(b, "spanM"), "Load Model 1 on the girder", "Lastmodell 1 auf dem Hauptträger")
            .utilization(dim(util), dim(1.0))
            .explanation(lc(
                format!("{lanes} notional lane(s) of {:.0} m: M_LM1 = {:.1} kNm, V_LM1 = {:.1} kN (tandem + UDL, simple span); with permanent actions M = {:.1} kNm ≤ M_pl,Rd = {:.1} kNm, V = {:.1} kN ≤ V_pl,Rd = {:.1} kN.", cat::LANE_WIDTH_M, m_lm / 1e3, v_lm / 1e3, m_demand / 1e3, m_rd / 1e3, v_demand / 1e3, v_rd / 1e3),
                format!("{lanes} rechnerische Fahrstreifen à {:.0} m: M_LM1 = {:.1} kNm, V_LM1 = {:.1} kN (Doppelachse + Gleichlast, Einfeldträger); mit ständigen Einwirkungen M = {:.1} kNm ≤ M_pl,Rd = {:.1} kNm, V = {:.1} kN ≤ V_pl,Rd = {:.1} kN.", cat::LANE_WIDTH_M, m_lm / 1e3, v_lm / 1e3, m_demand / 1e3, m_rd / 1e3, v_demand / 1e3, v_rd / 1e3),
            ));
        if util > 1.0 {
            let heavier = SteelSection::heavier_heb_options(&b.steel.designation);
            if !heavier.is_empty() {
                lm = lm.remedy(Remedy::one_of(bref(b, "steel.designation"), heavier, lc("Select a heavier steel section.", "Schwereren Stahlquerschnitt wählen.")));
            }
        }
        report.push(lm.build());
    }
}

/// 🧩 The extension checks of one composite column (load introduction, cl. 6.7.4).
pub(crate) fn column_extensions(report: &mut CheckReport, doc: &En1994Snapshot, col: &CompositeColumn) {
    let annex = doc.annex;
    let p = AnnexParams::for_annex(annex);
    let n_ed = part_en1990::uls_column(&col.actions, annex, doc.reliability_class).n_n.abs();
    let n_pl = part_column::n_pl_rd_n(col, annex).max(1.0);
    let steel_share = (col.steel_a_m2 * col.steel_f_y_pa / p.gamma_m0 / n_pl).clamp(0.0, 1.0);
    let n_intro = if col.introduced_force_n > 1e-9 { col.introduced_force_n } else { n_ed * (1.0 - steel_share) };
    let kind = match col.kind {
        semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled => cat::InterfaceKind::FilledRectangular,
        semio_s_artifact_norm_en1994_model::ColumnKind::PartiallyEncased => cat::InterfaceKind::PartiallyEncasedFlange,
        _ => cat::InterfaceKind::ConcreteEncased,
    };
    let perimeter = 4.0 * col.outer_size_m;
    let l_v = cat::introduction_length_m(col.outer_size_m, col.length_m);
    let (ed, rd, util) = cat::load_introduction(kind, n_intro, perimeter, l_v);
    let mut chk = head(annex, format!("en1994.6.7.4.introduction.{}", col.id), "1994-1-1", "6.7.4", cref(col, "outerSizeM"), "Introduction of loads into the composite column", "Lasteinleitung in die Verbundstütze")
        .utilization(dim(util), dim(1.0))
        .explanation(lc(
            format!("N_intro = N_Ed·(1 − A_a·f_yd/N_pl,Rd) = {:.1} kN over l_v = min(2d, L/3) = {:.2} m and u = {:.2} m: τ_Ed = {:.3} MPa ≤ τ_Rd = {:.2} MPa (Table 6.6).", n_intro / 1e3, l_v, perimeter, ed / 1e6, rd / 1e6),
            format!("N_intro = N_Ed·(1 − A_a·f_yd/N_pl,Rd) = {:.1} kN auf l_v = min(2d, L/3) = {:.2} m und u = {:.2} m: τ_Ed = {:.3} MPa ≤ τ_Rd = {:.2} MPa (Tabelle 6.6).", n_intro / 1e3, l_v, perimeter, ed / 1e6, rd / 1e6),
        ));
    if util > 1.0 && util.is_finite() {
        chk = chk.remedy(Remedy::at_least(cref(col, "outerSizeM"), length(col.outer_size_m), length(col.outer_size_m * util.sqrt()), lc("Increase the outer size or add shear connectors in the introduction length.", "Außenabmessung vergrößern oder Verbundmittel in der Einleitungslänge anordnen.")));
    }
    report.push(chk.build());
}
