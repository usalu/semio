//! 🔥️ Fire (EN 1994-1-2) and bridge (EN 1994-2) checks (C-44..C-54).

use super::*;

/// 🔖️ Registry ids (`ndp.en1994.*`) of the unverified composite-column fire constants.
const FIRE_NDPS: &[&str] = &[
    "ndp.en1994.1-2.kc-theta",
    "ndp.en1994.1-2.eps-c1-theta",
    "ndp.en1994.1-2.ke-theta",
    "ndp.en1994.1-2.concrete-conduction",
    "ndp.en1994.1-2.surface-exchange",
    "ndp.en1994.1-2.stiffness-phi",
    "ndp.en1994.1-2.buckling-curve",
    "ndp.en1994.1-2.interaction",
    "ndp.en1994.1-2.tables",
    "ndp.en1994.1-2.rho-concrete",
];

/// 🔖️ Registry ids (`ndp.en1994.*`) of the unverified bridge girder constants.
const BRIDGE_NDPS: &[&str] = &[
    "ndp.en1994.2.shear-lag",
    "ndp.en1994.2.rebar-fatigue-range",
];

/// 🔖️ Registry ids (`ndp.en1994.*`) of the unverified girder detailing constants (C-54).
const DETAIL_NDPS: &[&str] = &[
    "ndp.en1994.2.stud-edge-distance",
    "ndp.en1994.2.stud-cross-spacing",
    "ndp.en1994.2.min-transverse-reinforcement",
    "ndp.en1994.2.end-plate-bearing",
];

fn ins_ref() -> SubjectRef {
    SubjectRef::new("", "insulationThicknessM", lc("Fire insulation", "Brandschutzdämmung"))
}

fn fire_rem(cur: f64, req: f64, en: &str, de: &str) -> Remedy {
    Remedy::at_least(ins_ref(), length(cur), length(req), lc(en, de))
}

fn insulation_for(mut ok: impl FnMut(f64) -> bool, current: f64) -> f64 {
    let (mut lo, mut hi) = (current.max(1e-3), 0.2);
    if ok(hi) {
        for _ in 0..40 {
            let mid = 0.5 * (lo + hi);
            if ok(mid) { hi = mid } else { lo = mid }
        }
    }
    hi
}

pub(crate) fn evaluate_fire(report: &mut CheckReport, doc: &En1994Snapshot, scope: Scope<'_>) {
    let a = doc.annex;
    let Some(rating) = doc.fire_rating.rated() else {
        if scope.part("fire-rating") {
                report.push(head(a, "en1994.1-2.4.2.insulation".into(), "1994-1-2", "4.2", SubjectRef::new("", "fireRating", lc("Fire rating", "Feuerwiderstandsklasse")), "Tabulated fire insulation thickness", "Tabellarische Brandschutzdämmstärke").not_applicable(lc("Fire rating not recognised (use R30/R60/R90/R120).", "Feuerwiderstandsklasse unbekannt.")).build());
        }
        return;
    };
    let t_min = part_1_2::rating_minutes(rating);
    let theta_g = cat::iso834_c(t_min * 60.0);
    for (_, beam) in doc.beams.iter().enumerate().filter(|(index, _)| scope.admits("fireBeams", *index)) {
        let steel = SteelSection::resolve(&beam.steel);
        let t_req = part_1_2::insulation_thickness_m(rating, beam.sheeting.profile, steel.height_m, a);
        let mut tab = head(a, format!("en1994.1-2.4.2.insulation.{}", beam.id), "1994-1-2", "4.2", ins_ref(), "Tabulated fire insulation thickness", "Tabellarische Brandschutzdämmstärke")
            .minimum(length(doc.insulation_thickness_m), length(t_req))
            .explanation(lc(format!("Provided {:.0} mm, required {:.0} mm for {rating:?} / {} / h = {:.3} m.", doc.insulation_thickness_m * 1e3, t_req * 1e3, beam.sheeting.profile, steel.height_m), format!("Vorhanden {:.0} mm, erforderlich {:.0} mm für {rating:?} / {} / h = {:.3} m.", doc.insulation_thickness_m * 1e3, t_req * 1e3, beam.sheeting.profile, steel.height_m)));
        if doc.insulation_thickness_m < t_req {
            tab = tab.remedy(fire_rem(doc.insulation_thickness_m, t_req, &format!("Increase the insulation to ≥ {:.0} mm.", t_req * 1e3), &format!("Dämmstärke auf ≥ {:.0} mm erhöhen.", t_req * 1e3)));
        }
        report.push(tab.build());

        let am_v = part_1_2::section_factor_m_inv(&steel);
        let theta_a = part_1_2::steel_temperature_c(rating, am_v, doc.insulation_thickness_m);
        let fire_e = part_en1990::fire_combination(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, a, doc.reliability_class);
        let uls_e = part_en1990::uls_composite(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, a, doc.reliability_class);
        let eta_fi = if uls_e.m_nm > 1.0 { (fire_e.m_nm / uls_e.m_nm).clamp(0.02, 0.99) } else { 0.65 };
        let theta_cr = part_1_2::critical_temperature_c(eta_fi);
        let mut crit = head(a, format!("en1994.1-2.4.3.theta.{}", beam.id), "1994-1-2", "4.3", ins_ref(), "Critical temperature method", "Kritische-Temperatur-Verfahren")
            .utilization(temperature(theta_a), temperature(theta_cr))
            .explanation(lc(format!("θ_g({t_min:.0} min) = {theta_g:.0} °C, θ_a = {theta_a:.0} °C (A_p/V = {am_v:.0} 1/m, board {:.0} mm), θ_cr = {theta_cr:.0} °C (η_fi = {eta_fi:.2}, {}).", doc.insulation_thickness_m * 1e3, fire_e.label_en), format!("θ_g({t_min:.0} min) = {theta_g:.0} °C, θ_a = {theta_a:.0} °C (A_p/V = {am_v:.0} 1/m, Platte {:.0} mm), θ_cr = {theta_cr:.0} °C (η_fi = {eta_fi:.2}, {}).", doc.insulation_thickness_m * 1e3, fire_e.label_de)));
        if theta_a > theta_cr {
            let need = insulation_for(|d| part_1_2::steel_temperature_c(rating, am_v, d) <= theta_cr, doc.insulation_thickness_m);
            crit = crit.remedy(fire_rem(doc.insulation_thickness_m, need, &format!("Increase the insulation to ≥ {:.0} mm so θ_a ≤ θ_cr.", need * 1e3), &format!("Dämmung auf ≥ {:.0} mm erhöhen, damit θ_a ≤ θ_cr.", need * 1e3)));
        }
        report.push(crit.build());

        let fire = part_1_2::beam_fire(beam, doc.steel_f_y_pa, doc.insulation_thickness_m);
        let mfi = |theta: f64| fire.reduced(theta, None).m_nm;
        let m_rd = mfi(theta_a);
        let mut mf = head(a, format!("en1994.1-2.4.3.mfi.{}", beam.id), "1994-1-2", "4.3.4", ins_ref(), "Fire bending resistance at the steel temperature", "Brandbiegetragfähigkeit bei Stahltemperatur")
            .utilization(moment(fire_e.m_nm), moment(m_rd))
            .explanation(lc(format!("M_fi,Ed = {:.1} kNm, M_fi,Rd(θ_a = {theta_a:.0} °C, k_y,θ = {:.2}) = {:.1} kNm (uniform steel temperature, γ_M,fi = 1.0; k_y,θ of EN 1993-1-2 Table 3.1, unverified: ndp.en1993.1-2.t31.ky.*; slab and reinforcement are not in this model).", fire_e.m_nm / 1e3, cat::ky_theta(theta_a), m_rd / 1e3), format!("M_fi,Ed = {:.1} kNm, M_fi,Rd(θ_a = {theta_a:.0} °C, k_y,θ = {:.2}) = {:.1} kNm (gleichmäßige Stahltemperatur, γ_M,fi = 1,0; k_y,θ nach EN 1993-1-2 Tabelle 3.1, ungeprüft: ndp.en1993.1-2.t31.ky.*; Platte und Bewehrung sind nicht Teil dieses Modells).", fire_e.m_nm / 1e3, cat::ky_theta(theta_a), m_rd / 1e3)));
        if fire_e.m_nm > m_rd {
            let need = insulation_for(|d| mfi(part_1_2::steel_temperature_c(rating, am_v, d)) >= fire_e.m_nm, doc.insulation_thickness_m);
            mf = mf.remedy(fire_rem(doc.insulation_thickness_m, need, &format!("Increase the insulation to ≥ {:.0} mm.", need * 1e3), &format!("Dämmung auf ≥ {:.0} mm erhöhen.", need * 1e3)));
        }
        let steel_ky = cat::beam_fire::steel_reduction_ndp_ids();
        report.push(mf.ndps(&steel_ky.iter().map(String::as_str).collect::<Vec<_>>()).build());
    }
    for (_, slab) in doc.slabs.iter().enumerate().filter(|(index, _)| scope.admits("fireSlabs", *index)) {
        let hp = slab.sheeting.height_m;
        let hc = (slab.concrete_thickness_m - hp).max(0.04);
        let heff = hc + hp * part_slab::rib_fill_fraction(slab);
        let req = match rating {
            semio_s_artifact_norm_en1994_model::FireRating::Unrated => 0.0,
            semio_s_artifact_norm_en1994_model::FireRating::R30 => 0.060,
            semio_s_artifact_norm_en1994_model::FireRating::R60 => 0.080,
            semio_s_artifact_norm_en1994_model::FireRating::R90 => 0.100,
            semio_s_artifact_norm_en1994_model::FireRating::R120 => 0.120,
        };
        let sref = SubjectRef::new(slab.id.clone(), format!("slabs[id={}].concreteThicknessM", slab.id), lc(format!("Composite slab {}", slab.id), format!("Verbunddecke {}", slab.id)));
        let mut sl = head(a, format!("en1994.1-2.slab.insulation.{}", slab.id), "1994-1-2", "4.3.2", sref.clone(), "Slab insulation criterion h_eff", "Deckenwärmedämmkriterium h_eff")
            .minimum(length(heff), length(req))
            .explanation(lc(format!("h_eff = h_c + h_p·b_0/pitch = {:.0} mm (idealisation, unverified: ndp.en1994.1-2.heff), required {:.0} mm for {rating:?}.", heff * 1e3, req * 1e3), format!("h_eff = h_c + h_p·b_0/Teilung = {:.0} mm (Idealisierung, ungeprüft: ndp.en1994.1-2.heff), erforderlich {:.0} mm für {rating:?}.", heff * 1e3, req * 1e3)));
        if heff < req {
            sl = sl.remedy(Remedy::at_least(sref, length(slab.concrete_thickness_m), length(slab.concrete_thickness_m + (req - heff) * 1.02), lc("Increase the slab thickness.", "Plattendicke erhöhen.")));
        }
        report.push(sl.ndps(&["ndp.en1994.1-2.heff"]).build());
    }
    for (_, col) in doc.columns.iter().enumerate().filter(|(index, _)| scope.admits("fireColumns", *index)) {
        column_fire(report, doc, col, t_min);
    }
}

fn column_fire(report: &mut CheckReport, doc: &En1994Snapshot, col: &CompositeColumn, t_min: f64) {
    let a = doc.annex;
    let cref = |leaf: &str| SubjectRef::new(col.id.clone(), format!("columns[id={}].{leaf}", col.id), lc(format!("Column {}", col.id), format!("Stütze {}", col.id)));
    let fire_e = part_en1990::fire_column(&col.actions, a, doc.reliability_class);
    let uls_e = part_en1990::uls_column(&col.actions, a, doc.reliability_class);
    let eta_fi = if uls_e.n_n.abs() > 1.0 { (fire_e.n_n.abs() / uls_e.n_n.abs()).clamp(0.0, 1.0) } else { 0.0 };
    let cover = if col.rebar_cover_m > 1e-6 { col.rebar_cover_m } else { cat::constants::COLUMN_REBAR_EDGE_M };
    let ratio = col.reinforcement_as_m2 / col.concrete_a_m2.max(1e-9);
    let table = head(a, format!("en1994.1-2.4.2.table-column.{}", col.id), "1994-1-2", "4.2", cref("kind"), "Tabulated data of composite columns", "Tabellarische Daten von Verbundstützen");
    report.push(match cat::column_fire::check_tabulated(cat::column_fire::COLUMN_TABLE, t_min as u32, eta_fi, col.outer_size_m, cover, ratio) {
        cat::column_fire::Tabulated::NoData => table.not_applicable(lc(
            format!("No verified tabulated minimum dimensions for R{t_min:.0} at η_fi = {eta_fi:.2}: the cells of the EN 1994-1-2 column tables are not transcribed yet and are never guessed (NDP registry entry en1994.1-2.tables); the calculation model below applies."),
            format!("Keine verifizierten tabellarischen Mindestabmessungen für R{t_min:.0} bei η_fi = {eta_fi:.2}: die Zellen der Stützentabellen nach EN 1994-1-2 sind noch nicht übertragen und werden nie geraten (NDP-Register-Eintrag en1994.1-2.tables); es gilt das Rechenmodell unten."),
        )),
        cat::column_fire::Tabulated::Evaluated { utilization, verified } => {
            let mut t = table.utilization(dim(utilization), dim(1.0)).explanation(lc(format!("Minimum dimensions of the table row: utilisation {utilization:.2}."), format!("Mindestabmessungen der Tabellenzeile: Ausnutzung {utilization:.2}.")));
            if !verified && utilization <= 1.0 {
                t = t.status(CheckStatus::Warning);
            }
            if utilization > 1.0 && col.outer_size_m > 0.0 && utilization.is_finite() {
                let grown = col.outer_size_m * utilization;
                t = t.remedy(Remedy::at_least(cref("outerSizeM"), length(col.outer_size_m), length(grown), lc(format!("Increase the outer size to at least {:.0} mm so that the tabulated minimum dimensions are met.", grown * 1e3), format!("Außenmaß auf mindestens {:.0} mm erhöhen, damit die tabellarischen Mindestabmessungen eingehalten sind.", grown * 1e3))));
            }
            t
        }
    }.build());

    let model = head(a, format!("en1994.1-2.4.3.5.column.{}", col.id), "1994-1-2", "4.3.5", cref("actions"), "Fire resistance of the composite column (simple calculation model)", "Brandwiderstand der Verbundstütze (einfaches Rechenmodell)");
    let Some(g) = part_column::true_geometry(col) else {
        let reason = if col.kind == semio_s_artifact_norm_en1994_model::ColumnKind::ConcreteFilled {
            lc("Concrete-filled hollow sections need the filled-section model of EN 1994-1-2, which is not implemented; only encased and partially encased I-sections are covered.", "Betongefüllte Hohlprofile benötigen das Modell für gefüllte Querschnitte nach EN 1994-1-2, das nicht umgesetzt ist; nur einbetonierte und teilweise einbetonierte I-Profile sind abgedeckt.")
        } else {
            lc("Enter the I-section height, width, web and flange thickness of the column (section leaves) so the temperature field and the fibre resistance can be computed.", "Höhe, Breite, Steg- und Flanschdicke des I-Profils der Stütze (Querschnittsfelder) eingeben, damit Temperaturfeld und Faserwiderstand berechnet werden können.")
        };
        report.push(model.not_applicable(reason).build());
        return;
    };
    let product = cat::column_fire::RebarProduct::from(col.reinforcement_product);
    let strengths = cat::column_fire::FireStrengths::new(col.steel_f_y_pa, col.concrete_f_ck_pa, col.reinforcement_f_yk_pa, product);
    let evaluate = |geometry: &cat::encased::EncasedI| {
        let field = cat::column_fire::temperature_field(geometry, t_min * 60.0);
        (cat::column_fire::resistance(geometry, &field, &strengths, col.length_m), field)
    };
    let (n_ed, m_ed) = (fire_e.n_n.abs(), fire_e.m_nm.abs());
    let utilization = |res: &cat::column_fire::Resistance| n_ed / res.n_rd.max(1.0) + m_ed / res.m_pl.max(1.0);
    let (res, field) = evaluate(&g);
    let util = utilization(&res);
    let unverified = format!("{}, ndp.en1994.1-2.ks.{}.* (reinforcement strength reduction of {} bars, EN 1992-1-2 Table 3.2a, leaf columns[].reinforcementProduct)", FIRE_NDPS.join(", "), if product == cat::column_fire::RebarProduct::HotRolled { "hot" } else { "cold" }, product.wire());
    let mut chk = model.utilization(dim(util), dim(1.0)).explanation(lc(
        format!("R{t_min:.0}: N_fi,Ed = {:.0} kN, M_fi,Ed = {:.1} kNm (η_fi = {eta_fi:.2}); N_fi,pl = {:.0} kN, (EI)_fi = {:.1} MNm², N_fi,cr = {:.0} kN, λ̄_θ = {:.2}, χ_fi = {:.3}, N_fi,Rd = {:.0} kN, M_fi,pl,Rd = {:.1} kNm; steel mean {:.0} °C, hottest point {:.0} °C. Linear N–M interaction; unverified: {unverified}.", n_ed / 1e3, m_ed / 1e3, res.n_pl / 1e3, res.ei / 1e6, res.n_cr / 1e3, res.lambda, res.chi, res.n_rd / 1e3, res.m_pl / 1e3, field.steel_mean(), field.max()),
        format!("R{t_min:.0}: N_fi,Ed = {:.0} kN, M_fi,Ed = {:.1} kNm (η_fi = {eta_fi:.2}); N_fi,pl = {:.0} kN, (EI)_fi = {:.1} MNm², N_fi,cr = {:.0} kN, λ̄_θ = {:.2}, χ_fi = {:.3}, N_fi,Rd = {:.0} kN, M_fi,pl,Rd = {:.1} kNm; Stahl im Mittel {:.0} °C, heißester Punkt {:.0} °C. Lineare N–M-Interaktion; ungeprüft: {unverified}.", n_ed / 1e3, m_ed / 1e3, res.n_pl / 1e3, res.ei / 1e6, res.n_cr / 1e3, res.lambda, res.chi, res.n_rd / 1e3, res.m_pl / 1e3, field.steel_mean(), field.max()),
    ));
    if util > 1.0 {
        let grown = [1.1, 1.2, 1.35, 1.5, 1.75, 2.0].into_iter().find(|k| {
            let mut bigger = g;
            bigger.b_c *= k;
            bigger.h_c *= k;
            utilization(&evaluate(&bigger).0) <= 1.0
        });
        if let Some(k) = grown {
            chk = chk.remedy(Remedy::at_least(cref("outerSizeM"), length(col.outer_size_m), length(col.outer_size_m * k), lc(format!("Increase the outer size to ≥ {:.0} mm (more cover lowers the section temperatures).", col.outer_size_m * k * 1e3), format!("Außenabmessung auf ≥ {:.0} mm erhöhen (mehr Überdeckung senkt die Querschnittstemperaturen).", col.outer_size_m * k * 1e3))));
        }
    } else {
        chk = chk.status(CheckStatus::Warning);
    }
    let cited: Vec<String> = FIRE_NDPS.iter().map(|id| (*id).to_string()).chain(product.ndp_ids()).collect();
    report.push(chk.ndps(&cited.iter().map(String::as_str).collect::<Vec<_>>()).build());
}

pub(crate) fn evaluate_bridge(report: &mut CheckReport, doc: &En1994Snapshot, scope: Scope<'_>) {
    let a = doc.annex;
    if doc.structure_kind != semio_s_artifact_norm_en1994_model::StructureKind::Bridge {
        if scope.part("bridge-kind") {
            report.push(head(a, "en1994.2.fatigue".into(), "1994-2", "6.8", SubjectRef::new("", "structureKind", lc("Structure kind", "Tragwerkstyp")), "Bridge fatigue", "Brückenermüdung").not_applicable(lc("Structure is not a bridge — EN 1994-2 fatigue not applicable.", "Kein Brückentragwerk — EN 1994-2 Ermüdung nicht anwendbar.")).build());
        }
        return;
    }
    for (_, beam_in) in doc.beams.iter().enumerate().filter(|(index, _)| scope.admits("bridgeBeams", *index)) {
        let mut beam = beam_in.clone();
        beam.steel = SteelSection::resolve(&beam.steel);
        let sec = part_1_1::isection(&beam.steel);
        let bref = |leaf: &str| SubjectRef::new(beam.id.clone(), format!("beams[id={}].{leaf}", beam.id), lc(format!("Composite beam {}", beam.id), format!("Verbundträger {}", beam.id)));
        let mut delta = beam.actions.iter().filter(|x| x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Fatigue).map(|x| x.delta_sigma_k_pa).fold(0.0_f64, f64::max);
        if delta < 1.0 {
            delta = part_en1990::flm3_delta_sigma_pa(beam.span_m, sec.w_el());
        }
        let steel_curve = part_2::steel_curve(doc.fatigue_detail, a);
        let limit = steel_curve.resistance_pa(beam.n_cycles);
        let mut fat = head(a, format!("en1994.2.6.8.2.delta-sigma.{}", beam.id), "1994-2", "6.8.2", bref("actions"), "Bridge steel fatigue Δσ (FLM3)", "Brückenstahl Ermüdung Δσ (FLM3)")
            .utilization(stress(delta), stress(limit))
            .explanation(lc(format!("Δσ = {:.1} MPa, Δσ_R = {:.1} MPa (detail {}, N = {:.2e}, W_el = I/z from the resolved section), damage D = n/N_R = {:.3}.", delta / 1e6, limit / 1e6, doc.fatigue_detail, beam.n_cycles, steel_curve.damage(delta, beam.n_cycles)), format!("Δσ = {:.1} MPa, Δσ_R = {:.1} MPa (Detail {}, N = {:.2e}, W_el = I/z am aufgelösten Querschnitt), Schädigung D = n/N_R = {:.3}.", delta / 1e6, limit / 1e6, doc.fatigue_detail, beam.n_cycles, steel_curve.damage(delta, beam.n_cycles))));
        if delta > limit {
            fat = fat.remedy(Remedy::at_least(bref("steel.wPlYM3"), dim(beam.steel.w_pl_y_m3), dim(beam.steel.w_pl_y_m3 * delta / limit), lc("Increase the section modulus or reduce the FLM stress.", "Widerstandsmoment erhöhen oder FLM-Spannung reduzieren.")));
        }
        report.push(fat.build());

        let studs = part_studs::of(&beam, &doc.studs, a);
        let thinnest = studs.rows.iter().min_by(|x, y| x.diameter_m.total_cmp(&y.diameter_m));
        let d_min = thinnest.map_or(0.0, |s| s.diameter_m);
        let mut dtau = beam.actions.iter().filter(|x| x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Fatigue).map(|x| x.delta_tau_k_pa).fold(0.0_f64, f64::max);
        if dtau < 1.0 {
            dtau = part_en1990::flm3_delta_tau_pa(d_min);
        }
        let stud_curve = part_2::stud_curve(a);
        let stud_lim = stud_curve.resistance_pa(beam.n_cycles);
        let mut sf = head(a, format!("en1994.2.6.8.3.stud.{}", beam.id), "1994-2", "6.8.3", thinnest.map_or_else(|| SubjectRef::new(beam.id.clone(), "studs", lc(format!("Shear studs of {}", beam.id), format!("Kopfbolzendübel von {}", beam.id))), |s| SubjectRef::new(beam.id.clone(), format!("studs[id={}].diameterM", s.id), lc(format!("Shear stud {} of {}", s.id, beam.id), format!("Kopfbolzendübel {} von {}", s.id, beam.id)))), "Stud fatigue Δτ (FLM3)", "Bolzenermüdung Δτ (FLM3)")
            .utilization(stress(dtau), stress(stud_lim))
            .explanation(lc(format!("Δτ = {:.1} MPa, Δτ_R = {:.1} MPa (Δτ_c = 90 MPa, m = 8), damage D = n/N_R = {:.3}.", dtau / 1e6, stud_lim / 1e6, stud_curve.damage(dtau, beam.n_cycles)), format!("Δτ = {:.1} MPa, Δτ_R = {:.1} MPa (Δτ_c = 90 MPa, m = 8), Schädigung D = n/N_R = {:.3}.", dtau / 1e6, stud_lim / 1e6, stud_curve.damage(dtau, beam.n_cycles))));
        if let Some(s) = thinnest.filter(|_| dtau > stud_lim) {
            sf = sf.remedy(Remedy::at_least(SubjectRef::new(beam.id.clone(), format!("studs[id={}].diameterM", s.id), lc(format!("Shear stud {} of {}", s.id, beam.id), format!("Kopfbolzendübel {} von {}", s.id, beam.id))), length(s.diameter_m), length(s.diameter_m * (dtau / stud_lim).sqrt()), lc(format!("Increase the diameter of stud {}.", s.id), format!("Durchmesser von Bolzen {} erhöhen.", s.id))));
        }
        if thinnest.is_none() {
            sf = sf.not_applicable(lc("No studs on the beam.", "Keine Bolzen am Träger."));
        }
        report.push(sf.build());

        let le = cat::equivalent_span_m(beam.support, beam.span_m, false);
        let le_h = if beam.has_hogging() { cat::le_hogging_m(beam.span_m, beam.neighbour_span_m()) } else { beam.span_m };
        let half = beam.spacing_m / 2.0;
        let (ys, hc) = (beam.slab_thickness_m, part_1_1::concrete_height_m(&beam));
        let (beff, beff_h) = (cat::effective_width_m(le, 0.0, half, half, false), cat::effective_width_m(le_h, 0.0, half, half, false));

        if doc.fatigue_detail == semio_s_artifact_norm_en1994_model::FatigueDetail::Reinforcement {
            let as_total = beam.as_hogging_m2_per_m * beff_h;
            let cr = cat::cracked(&sec, ys, as_total, cat::constants::HOGGING_REBAR_DEPTH_M);
            let rho_s = as_total / (beff_h * hc).max(1e-9);
            let d_ts = cat::delta_sigma_ts_fatigue(cat::fctm_pa(beam.concrete_f_ck_pa), cr.area, cr.i, sec.area(), sec.i_y, rho_s);
            let m_range = delta * sec.w_el();
            let m_hog = part_en1990::sls_characteristic(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, a, doc.reliability_class).m_hog_nm.abs().max(m_range);
            let st = cat::bridge::rebar_fatigue(m_hog, m_range, cr.y_na, cat::constants::HOGGING_REBAR_DEPTH_M, cr.i, d_ts);
            let d_s = st.delta_equ_pa;
            let rebar_curve = cat::fatigue::rebar_curve();
            let r_lim = rebar_curve.resistance_pa(beam.n_cycles);
            let mut rf = head(a, format!("en1994.2.6.8.2.rebar.{}", beam.id), "1994-2", "6.8.5.4", bref("asHoggingM2PerM"), "Reinforcement fatigue with tension stiffening", "Bewehrungsermüdung mit Zugversteifung")
                .utilization(stress(d_s), stress(r_lim))
                .explanation(lc(
                    format!("σ_s,0 = {:.1} MPa on the cracked section (M = {:.1} kNm), Δσ_s = {:.1} MPa tension stiffening (ρ_s = {:.2} %), σ_s,max,f = σ_s,0 + Δσ_s = {:.1} MPa; Δσ_s,equ = {:.1} MPa (cracked range {:.1} MPa from {:.1} kNm, plus Δσ_s) ≤ Δσ_Rsk(N)/γ_s,fat = {:.1} MPa (straight bars, k₂ = 9), damage D = n/N_R = {:.3}; unverified: ndp.en1994.2.fatigue-tension-stiffening, ndp.en1994.2.rebar-fatigue-range.", st.sigma_0_pa / 1e6, m_hog / 1e3, d_ts / 1e6, rho_s * 100.0, st.sigma_max_f_pa / 1e6, d_s / 1e6, st.delta_cracked_pa / 1e6, m_range / 1e3, r_lim / 1e6, rebar_curve.damage(d_s, beam.n_cycles)),
                    format!("σ_s,0 = {:.1} MPa am gerissenen Querschnitt (M = {:.1} kNm), Δσ_s = {:.1} MPa Zugversteifung (ρ_s = {:.2} %), σ_s,max,f = σ_s,0 + Δσ_s = {:.1} MPa; Δσ_s,equ = {:.1} MPa (gerissene Schwingbreite {:.1} MPa aus {:.1} kNm, plus Δσ_s) ≤ Δσ_Rsk(N)/γ_s,fat = {:.1} MPa (gerade Stäbe, k₂ = 9), Schädigung D = n/N_R = {:.3}; ungeprüft: ndp.en1994.2.fatigue-tension-stiffening, ndp.en1994.2.rebar-fatigue-range.", st.sigma_0_pa / 1e6, m_hog / 1e3, d_ts / 1e6, rho_s * 100.0, st.sigma_max_f_pa / 1e6, d_s / 1e6, st.delta_cracked_pa / 1e6, m_range / 1e3, r_lim / 1e6, rebar_curve.damage(d_s, beam.n_cycles)),
                ));
            if d_s > r_lim {
                rf = rf.remedy(Remedy::at_least(bref("asHoggingM2PerM"), area(beam.as_hogging_m2_per_m), area(beam.as_hogging_m2_per_m * d_s / r_lim.max(1.0) * 1.05), lc("Increase the hogging reinforcement.", "Stützbewehrung erhöhen.")));
            }
            report.push(rf.ndps(&["ndp.en1994.2.rebar-fatigue-range", "ndp.en1994.2.fatigue-tension-stiffening"]).build());
        }

        let b0 = ((beam.steel.width_m - beam.steel.tw_m) / 2.0).max(0.0);
        let (kappa, kappa_h) = (cat::bridge::kappa(b0, le, 1.0), cat::bridge::kappa(b0, le_h, 1.0));
        let (beta, beta_h) = (cat::bridge::beta_sagging(kappa), if beam.has_hogging() { cat::bridge::beta_hogging(kappa_h) } else { 1.0 });
        let uls = part_en1990::uls_composite(&beam.actions, beam.span_m, beam.adjacent_span_m, beam.support, beam.spacing_m, a, doc.reliability_class);
        let n0 = cat::modular_ratio(beam.concrete_e_cm_pa, 0.0, 0.0);
        let fyd = doc.steel_f_y_pa / AnnexParams::for_annex(a).gamma_m0;
        let bottom = |section: &cat::ISection| {
            let el = cat::transformed(section, ys, beff, hc, n0);
            uls.m_nm * (ys + section.h - el.y_na) / el.i
        };
        let (nominal, lagged) = (bottom(&sec), bottom(&cat::bridge::shear_lag_section(&sec, beta)));
        let mut lag = head(a, format!("en1994.2.5.4.1.shear-lag.{}", beam.id), "1994-2", "5.4.1.2", bref("steel.widthM"), "Shear lag of the steel flanges", "Schubverzerrung der Stahlgurte")
            .utilization(stress(lagged), stress(fyd))
            .explanation(lc(
                format!("b₀ = {:.0} mm: κ = {kappa:.3}, β = {beta:.3} (sagging, L_e = {le:.1} m), κ_hog = {kappa_h:.3}, β₂ = {beta_h:.3} (hogging, L_e = {le_h:.1} m); bottom-flange stress {:.1} MPa on the reduced flanges against {:.1} MPa unreduced, f_yd = {:.0} MPa; factors of EN 1993-1-5 Table 3.1 (unverified: en1994.2.shear-lag).", b0 * 1e3, lagged / 1e6, nominal / 1e6, fyd / 1e6),
                format!("b₀ = {:.0} mm: κ = {kappa:.3}, β = {beta:.3} (Feld, L_e = {le:.1} m), κ_Stütz = {kappa_h:.3}, β₂ = {beta_h:.3} (Stütze, L_e = {le_h:.1} m); Spannung im Untergurt {:.1} MPa mit abgeminderten Gurten gegen {:.1} MPa ungemindert, f_yd = {:.0} MPa; Beiwerte nach EN 1993-1-5 Tabelle 3.1 (ungeprüft: en1994.2.shear-lag).", b0 * 1e3, lagged / 1e6, nominal / 1e6, fyd / 1e6),
            ));
        if lagged > fyd {
            let heavier = SteelSection::heavier_heb_options(&beam.steel.designation);
            if !heavier.is_empty() {
                lag = lag.remedy(Remedy::one_of(bref("steel.designation"), heavier, lc("Select a heavier steel section.", "Schwereren Stahlquerschnitt wählen.")));
            }
        }
        report.push(lag.ndps(&["ndp.en1994.2.shear-lag"]).build());
        girder_detail(report, doc, &beam);
    }
}

/// 🌉️ Bridge girder detailing of one beam (C-54): stud edge distance and transverse spacing, minimum transverse reinforcement and the bearing of the end plate (0 = not declared, check not applicable).
fn girder_detail(report: &mut CheckReport, doc: &En1994Snapshot, beam: &CompositeBeam) {
    let a = doc.annex;
    let p = AnnexParams::for_annex(a);
    let bref = |leaf: &str| SubjectRef::new(beam.id.clone(), format!("beams[id={}].{leaf}", beam.id), lc(format!("Composite beam {}", beam.id), format!("Verbundträger {}", beam.id)));
    let studs = part_studs::of(beam, &doc.studs, a);
    let d = studs.max_diameter_m();
    let hc = part_1_1::concrete_height_m(beam);

    let edge_min = cat::bridge::STUD_EDGE_DISTANCE_MIN_M;
    let mut edge = head(a, format!("en1994.2.6.6.5.edge.{}", beam.id), "1994-2", "6.6.5", bref("studEdgeDistanceM"), "Stud edge distance", "Bolzenrandabstand").ndps(DETAIL_NDPS);
    if beam.stud_edge_distance_m <= 0.0 {
        edge = edge.not_applicable(lc("Stud edge distance not declared.", "Bolzenrandabstand nicht angegeben."));
    } else {
        edge = edge.minimum(length(beam.stud_edge_distance_m), length(edge_min)).explanation(lc(
            format!("Edge distance {:.0} mm ≥ {:.0} mm (unverified: ndp.en1994.2.stud-edge-distance).", beam.stud_edge_distance_m * 1e3, edge_min * 1e3),
            format!("Randabstand {:.0} mm ≥ {:.0} mm (ungeprüft: ndp.en1994.2.stud-edge-distance).", beam.stud_edge_distance_m * 1e3, edge_min * 1e3),
        ));
        if beam.stud_edge_distance_m < edge_min {
            edge = edge.remedy(Remedy::at_least(bref("studEdgeDistanceM"), length(beam.stud_edge_distance_m), length(edge_min), lc(format!("Keep the studs ≥ {:.0} mm from the flange edge.", edge_min * 1e3), format!("Bolzen ≥ {:.0} mm vom Flanschrand anordnen.", edge_min * 1e3))));
        }
    }
    report.push(edge.build());

    let solid = beam.sheeting.height_m <= 1e-9;
    let cross_min = cat::bridge::stud_cross_spacing_min_m(d, solid);
    let mut cross = head(a, format!("en1994.2.6.6.5.cross-spacing.{}", beam.id), "1994-2", "6.6.5", bref("studTransverseSpacingM"), "Stud transverse spacing", "Bolzenquerabstand").ndps(DETAIL_NDPS);
    if beam.stud_transverse_spacing_m <= 0.0 {
        cross = cross.not_applicable(lc("Stud transverse spacing not declared.", "Bolzenquerabstand nicht angegeben."));
    } else {
        cross = cross.minimum(length(beam.stud_transverse_spacing_m), length(cross_min)).explanation(lc(
            format!("Transverse spacing {:.0} mm ≥ {} d = {:.0} mm ({} slab; unverified: ndp.en1994.2.stud-cross-spacing).", beam.stud_transverse_spacing_m * 1e3, if solid { "4" } else { "2.5" }, cross_min * 1e3, if solid { "solid" } else { "ribbed" }),
            format!("Querabstand {:.0} mm ≥ {} d = {:.0} mm ({}; ungeprüft: ndp.en1994.2.stud-cross-spacing).", beam.stud_transverse_spacing_m * 1e3, if solid { "4" } else { "2,5" }, cross_min * 1e3, if solid { "Vollplatte" } else { "Profilblechdecke" }),
        ));
        if beam.stud_transverse_spacing_m < cross_min {
            cross = cross.remedy(Remedy::at_least(bref("studTransverseSpacingM"), length(beam.stud_transverse_spacing_m), length(cross_min), lc(format!("Space the stud rows ≥ {:.0} mm apart.", cross_min * 1e3), format!("Bolzenreihen ≥ {:.0} mm auseinander anordnen.", cross_min * 1e3))));
        }
    }
    report.push(cross.build());

    let as_min = cat::bridge::min_transverse_reinforcement_m2_per_m(hc);
    let as_total = beam.transverse_as_m2_per_m + beam.transverse_as_top_m2_per_m;
    let mut minr = head(a, format!("en1994.2.6.6.6.transverse-min.{}", beam.id), "1994-2", "6.6.6.3", bref("transverseAsTopM2PerM"), "Minimum transverse reinforcement of the deck", "Mindestquerbewehrung der Fahrbahnplatte")
        .minimum(area(as_total), area(as_min))
        .explanation(lc(
            format!("A_s = {:.0} mm²/m (bottom {:.0} + top {:.0}) ≥ 0.002·h_c = {:.0} mm²/m (unverified: ndp.en1994.2.min-transverse-reinforcement).", as_total * 1e6, beam.transverse_as_m2_per_m * 1e6, beam.transverse_as_top_m2_per_m * 1e6, as_min * 1e6),
            format!("A_s = {:.0} mm²/m (unten {:.0} + oben {:.0}) ≥ 0,002·h_c = {:.0} mm²/m (ungeprüft: ndp.en1994.2.min-transverse-reinforcement).", as_total * 1e6, beam.transverse_as_m2_per_m * 1e6, beam.transverse_as_top_m2_per_m * 1e6, as_min * 1e6),
        ))
        .ndps(DETAIL_NDPS);
    if as_total < as_min {
        minr = minr.remedy(Remedy::at_least(bref("transverseAsTopM2PerM"), area(beam.transverse_as_top_m2_per_m), area(beam.transverse_as_top_m2_per_m + as_min - as_total), lc(format!("Add top transverse reinforcement to reach {:.0} mm²/m.", as_min * 1e6), format!("Obere Querbewehrung ergänzen, um {:.0} mm²/m zu erreichen.", as_min * 1e6))));
    }
    report.push(minr.build());

    let (w, h) = (beam.end_plate_width_m, beam.end_plate_height_m);
    let mut plate = head(a, format!("en1994.2.6.6.5.end-plate.{}", beam.id), "1994-2", "6.6.5", bref("endPlateWidthM"), "End plate bearing on the concrete", "Pressung der Endplatte auf dem Beton").ndps(DETAIL_NDPS);
    if w <= 0.0 || h <= 0.0 {
        plate = plate.not_applicable(lc("No end plate declared.", "Keine Endplatte angegeben."));
    } else {
        let in_zone: Vec<&part_studs::StudResistance> = studs.in_span().filter(|s| s.x_m - studs.origin_m <= beam.end_zone_length_m + 1e-9).collect();
        let n_end = in_zone.len();
        let f_ed: f64 = in_zone.iter().map(|s| s.p_rd_n).sum();
        let fcd = cat::fcc_beam_pa(beam.concrete_f_ck_pa, p.gamma_c) / cat::constants::CONCRETE_BEAM_FACTOR;
        let slab_b = (beam.spacing_m).max(w);
        let f_rd = cat::bridge::end_plate_bearing_n(fcd, w, h, slab_b, beam.slab_thickness_m);
        plate = plate.utilization(force(f_ed), force(f_rd)).explanation(lc(
            format!("{n_end} studs in the {:.2} m end zone transfer F_Ed = {:.0} kN onto the {:.0} × {:.0} mm plate; F_Rdu = A_c0·f_cd·√(A_c1/A_c0) ≤ 3·f_cd·A_c0 = {:.0} kN (EN 1992-1-1 6.7; unverified: ndp.en1994.2.end-plate-bearing).", beam.end_zone_length_m, f_ed / 1e3, w * 1e3, h * 1e3, f_rd / 1e3),
            format!("{n_end} Bolzen im {:.2} m langen Endbereich übertragen F_Ed = {:.0} kN auf die Platte {:.0} × {:.0} mm; F_Rdu = A_c0·f_cd·√(A_c1/A_c0) ≤ 3·f_cd·A_c0 = {:.0} kN (EN 1992-1-1 6.7; ungeprüft: ndp.en1994.2.end-plate-bearing).", beam.end_zone_length_m, f_ed / 1e3, w * 1e3, h * 1e3, f_rd / 1e3),
        ));
        if f_ed > f_rd {
            plate = plate.remedy(Remedy::at_least(bref("endPlateHeightM"), length(h), length(h * f_ed / f_rd.max(1.0) * 1.05), lc("Enlarge the end plate.", "Endplatte vergrößern.")));
        }
    }
    report.push(plate.build());
}
