//! 🏗️ Composite beam checks: EN 1994-1-1 §3, §5, §6.2, §6.4, §6.6, §7 (C-01..C-23, C-40..C-43).

use super::*;

const STATIONS: usize = 16;

#[derive(Clone, Copy)]
enum Effect {
    Sagging,
    Hogging,
    Shear,
}

/// 📈️ Relative internal forces of a uniformly loaded beam along its whole length (`s` in `[0, 1]` of the total length of all spans of the layout): the sagging and hogging moment and the shear force of the all-spans-loaded statics of the span position (simple, end, interior, cantilever, unequal neighbours by the three-moment equation), each normalised to the governing value of the designed span.
fn relative(profile: &cat::continuous::Profile, effect: Effect, s: f64) -> f64 {
    match effect {
        Effect::Sagging => profile.sagging(s),
        Effect::Hogging => profile.hogging(s),
        Effect::Shear => profile.shear(s),
    }
}

fn ratio(demand: f64, capacity: f64) -> f64 {
    if capacity > 1e-12 { demand / capacity } else if demand > 0.0 { f64::INFINITY } else { 0.0 }
}

struct BeamCtx<'a> {
    annex: AnnexChoice,
    reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    p: AnnexParams,
    f_y: f64,
    fyd: f64,
    fcc: f64,
    doc: &'a En1994Snapshot,
    beam: &'a CompositeBeam,
    studs: part_studs::BeamStuds,
    profile: cat::continuous::Profile,
    sec: cat::ISection,
    ys: f64,
    hc: f64,
    le: f64,
    beff: f64,
    beff_h: f64,
    uls: part_en1990::DesignEffects,
    uls_c: part_en1990::DesignEffects,
    sls_char: part_en1990::DesignEffects,
    sls_freq: part_en1990::DesignEffects,
    sls_qp: part_en1990::DesignEffects,
}

fn bpath(beam: &CompositeBeam, leaf: &str) -> String {
    format!("beams[id={}].{leaf}", beam.id)
}

fn bref(beam: &CompositeBeam, leaf: &str) -> SubjectRef {
    SubjectRef::new(beam.id.clone(), bpath(beam, leaf), lc(format!("Composite beam {}", beam.id), format!("Verbundträger {}", beam.id)))
}

fn heavier_clearing(beam: &CompositeBeam, ok: impl Fn(&CompositeBeam) -> bool) -> Vec<String> {
    SteelSection::heavier_heb_options(&beam.steel.designation)
        .into_iter()
        .filter(|d| {
            let Some(sec) = SteelSection::from_catalogue(d) else { return false };
            let mut trial = beam.clone();
            trial.steel = sec;
            ok(&trial)
        })
        .collect()
}

fn load_fix(beam: &CompositeBeam, factor: f64, en: &str, de: &str) -> Option<Remedy> {
    load_remedy(&beam.id, &format!("beams[id={}]", beam.id), lc(format!("Composite beam {}", beam.id), format!("Verbundträger {}", beam.id)), &beam.actions, factor, en, de)
}

pub(crate) fn evaluate_beam(report: &mut CheckReport, doc: &En1994Snapshot, beam_in: &CompositeBeam) {
    let annex = doc.annex;
    let p = AnnexParams::for_annex(annex);
    let mut beam = beam_in.clone();
    beam.steel = SteelSection::resolve(&beam.steel);
    let sec = part_1_1::isection(&beam.steel);
    let le = cat::equivalent_span_m(beam.support, beam.span_m, false);
    let le_h = if beam.has_hogging() { cat::le_hogging_m(beam.span_m, beam.neighbour_span_m()) } else { beam.span_m };
    let half = beam.spacing_m / 2.0;
    let (span, support, spacing) = (beam.span_m, beam.support, beam.spacing_m);
    let ctx = BeamCtx {
        annex,
        reliability_class: doc.reliability_class,
        p,
        f_y: doc.steel_f_y_pa,
        fyd: doc.steel_f_y_pa / p.gamma_m0,
        fcc: cat::fcc_beam_pa(beam.concrete_f_ck_pa, p.gamma_c),
        doc,
        studs: part_studs::of(&beam, &doc.studs, annex),
        sec,
        profile: cat::continuous::Profile::of(cat::span_position(beam.support), span, beam.adjacent_span_m),
        ys: beam.slab_thickness_m,
        hc: part_1_1::concrete_height_m(&beam),
        le,
        beff: cat::effective_width_m(le, 0.0, half, half, false),
        beff_h: cat::effective_width_m(le_h, 0.0, half, half, false),
        uls: part_en1990::uls_composite(&beam.actions, span, beam.adjacent_span_m, support, spacing, annex, doc.reliability_class),
        uls_c: part_en1990::uls_construction(&beam.actions, span, beam.adjacent_span_m, support, spacing, annex, doc.reliability_class),
        sls_char: part_en1990::sls_characteristic(&beam.actions, span, beam.adjacent_span_m, support, spacing, annex, doc.reliability_class),
        sls_freq: part_en1990::sls_frequent(&beam.actions, span, beam.adjacent_span_m, support, spacing, annex, doc.reliability_class),
        sls_qp: part_en1990::sls_quasi_permanent(&beam.actions, span, beam.adjacent_span_m, support, spacing, annex, doc.reliability_class),
        beam: &beam,
    };
    geometry(report, &ctx, doc);
    flexure(report, &ctx);
    connection(report, &ctx);
    shear(report, &ctx);
    ltb(report, &ctx);
    sls_stress(report, &ctx);
    deflection(report, &ctx);
    crack(report, &ctx);
    tension_stiffening(report, &ctx);
}

/// 🏷️ C-02 classification of the steel section (cl. 5.5, Table 5.2), C-03/C-04 effective width and equivalent span, global analysis (cl. 5.4).
fn geometry(r: &mut CheckReport, c: &BeamCtx, doc: &En1994Snapshot) {
    let (b, a) = (c.beam, c.annex);
    let f_u_stud = c.studs.rows.iter().map(|s| s.f_u_pa).fold(0.0, f64::max);
    let sc = cat::material_scope(b.concrete_f_ck_pa, c.f_y, f_u_stud);
    let ratio = (b.concrete_f_ck_pa / 60e6).max(c.f_y / 460e6).max(f_u_stud / 500e6);
    let mut m = head(a, format!("en1994.3.materials.{}", b.id), "1994-1-1", "3", bref(b, "concreteFCkPa"), "Material validity ranges", "Werkstoffgrenzen")
        .utilization(dim(ratio), dim(1.0))
        .explanation(lc(format!("f_ck = {:.0} MPa, f_y = {:.0} MPa, f_u,stud = {:.0} MPa inside C20/25..C60/75, S235..S460, 500 MPa.", b.concrete_f_ck_pa / 1e6, c.f_y / 1e6, f_u_stud / 1e6), format!("f_ck = {:.0} MPa, f_y = {:.0} MPa, f_u,Bolzen = {:.0} MPa innerhalb C20/25..C60/75, S235..S460, 500 MPa.", b.concrete_f_ck_pa / 1e6, c.f_y / 1e6, f_u_stud / 1e6)));
    if !sc.in_scope {
        m = m.not_applicable(lc(format!("Out of scope: {}.", sc.violation), format!("Außerhalb des Anwendungsbereichs: {}.", sc.violation)));
    }
    r.push(m.build());

    let capped = c.le / 4.0 > b.spacing_m;
    let mut beff = head(a, format!("en1994.5.4.1.2.beff.{}", b.id), "1994-1-1", "5.4.1.2", bref(b, "spacingM"), "Effective width b_eff vs beam spacing", "Mitwirkende Breite b_eff gegen Trägerabstand")
        .utilization(length(c.beff), length(b.spacing_m))
        .explanation(lc(
            format!("L_e = {:.2} m, b_ei = min(L_e/8, s/2) → b_eff = {:.3} m{} (hogging region {:.3} m).", c.le, c.beff, if capped { ", capped by the beam spacing" } else { "" }, c.beff_h),
            format!("L_e = {:.2} m, b_ei = min(L_e/8, s/2) → b_eff = {:.3} m{} (Stützbereich {:.3} m).", c.le, c.beff, if capped { ", durch den Trägerabstand begrenzt" } else { "" }, c.beff_h),
        ));
    if c.beff > b.spacing_m {
        beff = beff.remedy(Remedy::at_least(bref(b, "spacingM"), length(b.spacing_m), length(c.beff), lc(format!("Increase the beam spacing from {:.2} m to at least {:.2} m or shorten the equivalent span so that b_eff fits between the beams.", b.spacing_m, c.beff), format!("Trägerabstand von {:.2} m auf mindestens {:.2} m erhöhen oder die äquivalente Stützweite verkürzen, damit b_eff zwischen die Träger passt.", b.spacing_m, c.beff))));
    }
    r.push(beff.build());

    let class = part_1_1::section_class(&b.steel, c.f_y);
    let mut cls = head(a, format!("en1994.5.5.class.{}", b.id), "1994-1-1", "5.5", bref(b, "steel.twM"), "Steel section classification", "Querschnittsklasse Stahlprofil")
        .utilization(dim(class as f64), dim(3.0))
        .explanation(lc(format!("Section class {class} from c/t_f and h_w/t_w (t_w={:.1} mm, t_f={:.1} mm).", b.steel.tw_m * 1000.0, b.steel.tf_m * 1000.0), format!("Querschnittsklasse {class} aus c/t_f und h_w/t_w (t_w={:.1} mm, t_f={:.1} mm).", b.steel.tw_m * 1000.0, b.steel.tf_m * 1000.0)));
    if class > 3 {
        let clearing = heavier_clearing(b, |t| part_1_1::section_class(&t.steel, c.f_y) <= 3);
        if !clearing.is_empty() {
            cls = cls.remedy(Remedy::one_of(bref(b, "steel.designation"), clearing, lc("Select a catalogue section of class 1–3.", "Katalogquerschnitt der Klasse 1–3 wählen.")));
        } else {
            cls = cls.remedy(Remedy::at_least(bref(b, "steel.twM"), length(b.steel.tw_m), length(b.steel.tw_m * 1.2), lc("Increase web/flange thickness to improve the class.", "Steg-/Flanschdicke erhöhen, um die Klasse zu verbessern.")));
        }
    }
    r.push(cls.build());

    if b.is_continuous() || b.has_hogging() {
        let (l1, l2) = (b.span_m, b.neighbour_span_m());
        let ratio = l1.min(l2) / l1.max(l2);
        let ok = ratio >= 0.6;
        let mut an = head(a, format!("en1994.5.4.2.analysis.{}", b.id), "1994-1-1", "5.4.2", bref(b, "support"), "Global analysis: cracked regions", "Globale Berechnung: gerissene Bereiche")
            .minimum(dim(ratio), dim(0.6))
            .explanation(lc(
                format!("Two spans of {l1:.2} m / {l2:.2} m: ratio {ratio:.2} (limit 0.6 for the 15 % cracked-length rule of cl. 5.4.2.3); internal forces use constant EI of the two-span statics, the model carries one span length."),
                format!("Zwei Felder {l1:.2} m / {l2:.2} m: Verhältnis {ratio:.2} (Grenze 0,6 für die 15-%-Regel der gerissenen Länge nach Abs. 5.4.2.3); Schnittgrößen mit konstantem EI der Zweifeldstatik, das Modell führt eine Stützweite."),
            ));
        if !ok && b.cracked_length_fraction <= 1e-9 {
            an = an.not_applicable(lc("Uncracked global analysis selected (cracked length fraction 0): the 15 % rule is not used.", "Ungerissene globale Berechnung gewählt (gerissener Längenanteil 0): die 15-%-Regel wird nicht angewendet."));
        } else if !ok {
            an = an.remedy(Remedy::exactly(bref(b, "crackedLengthFraction"), dim(b.cracked_length_fraction), dim(0.0), lc("Use an uncracked analysis for unequal spans (cracked length fraction 0).", "Bei ungleichen Feldern ungerissene Berechnung verwenden (gerissener Längenanteil 0).")));
            an = an.remedy(if l2 < l1 {
                Remedy::at_least(bref(b, "adjacentSpanM"), length(l2), length(0.6 * l1 * (1.0 + 1e-9)), lc(format!("Or lengthen the adjacent span from {l2:.2} m to at least {:.2} m (ratio 0.6).", 0.6 * l1), format!("Oder das Nachbarfeld von {l2:.2} m auf mindestens {:.2} m verlängern (Verhältnis 0,6).", 0.6 * l1)))
            } else {
                Remedy::at_most(bref(b, "adjacentSpanM"), length(l2), length(l1 / 0.6 * (1.0 - 1e-9)), lc(format!("Or shorten the adjacent span from {l2:.2} m to at most {:.2} m (ratio 0.6).", l1 / 0.6), format!("Oder das Nachbarfeld von {l2:.2} m auf höchstens {:.2} m verkürzen (Verhältnis 0,6).", l1 / 0.6)))
            });
        }
        r.push(an.build());
        cracked_analysis(r, c);
    }
    let _ = doc;
}

fn cracked_analysis(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let position = cat::span_position(b.support);
    let fraction = b.cracked_length_fraction.clamp(0.0, 0.5);
    let id = format!("en1994.5.4.2.3.cracked.{}", b.id);
    let (en, de) = ("Global analysis with cracked stiffness EI₂", "Globale Berechnung mit gerissener Steifigkeit EI₂");
    if fraction <= 1e-9 {
        r.push(head(a, id, "1994-1-1", "5.4.2.3", bref(b, "crackedLengthFraction"), en, de).not_applicable(lc("Uncracked global analysis selected (cracked length fraction 0).", "Ungerissene globale Berechnung gewählt (gerissener Längenanteil 0).")).build());
        return;
    }
    let i_0 = cat::transformed(&c.sec, c.ys, c.beff, c.hc, cat::modular_ratio(b.concrete_e_cm_pa, 0.0, 0.0)).i;
    let (r_hog, r_sag) = cat::continuous::cracking_ratios(position, b.span_m, b.adjacent_span_m, stiffness(c, i_0), cat::continuous::Stiffness::uniform(cat::constants::E_A * i_0));
    let as_total = b.as_hogging_m2_per_m * c.beff_h;
    let (m_pl_hog, _) = cat::plastic_hogging(&c.sec, c.fyd, c.ys, as_total, cat::constants::REBAR_FSK / c.p.gamma_s, cat::constants::HOGGING_REBAR_DEPTH_M);
    let m_pl_sag = cat::plastic_full(&c.sec, c.fyd, c.ys, c.beff, c.hc, c.fcc).m_nm;
    let (m_sag, m_hog) = (c.uls.m_nm * r_sag, c.uls.m_hog_nm.abs() * r_hog);
    let util = ratio(m_sag, m_pl_sag).max(ratio(m_hog, m_pl_hog));
    let mut chk = head(a, id, "1994-1-1", "5.4.2.3", bref(b, "crackedLengthFraction"), en, de)
        .utilization(dim(util), dim(1.0))
        .explanation(lc(
            format!("EI₂ over {:.0} % of the span at each internal support (cl. 5.4.2.3(3)): support moment ×{r_hog:.3} and span moment ×{r_sag:.3} of the uncracked analysis, M_sag = {:.1} kNm ≤ M_pl,Rd = {:.1} kNm, M_hog = {:.1} kNm ≤ M_pl,Rd,hog = {:.1} kNm.", fraction * 100.0, m_sag / 1e3, m_pl_sag / 1e3, m_hog / 1e3, m_pl_hog / 1e3),
            format!("EI₂ auf {:.0} % der Stützweite an jeder Innenstütze (Abs. 5.4.2.3(3)): Stützmoment ×{r_hog:.3} und Feldmoment ×{r_sag:.3} der ungerissenen Berechnung, M_Feld = {:.1} kNm ≤ M_pl,Rd = {:.1} kNm, M_Stütz = {:.1} kNm ≤ M_pl,Rd,Stütz = {:.1} kNm.", fraction * 100.0, m_sag / 1e3, m_pl_sag / 1e3, m_hog / 1e3, m_pl_hog / 1e3),
        ));
    if util > 1.0 {
        chk = chk.remedy(Remedy::exactly(bref(b, "crackedLengthFraction"), dim(fraction), dim(0.0), lc("Use the uncracked global analysis.", "Ungerissene globale Berechnung verwenden.")));
        if let Some(rem) = load_fix(b, (1.0 / util).clamp(0.0, 1.0) * 0.95, "Reduce the imposed load.", "Nutzlast reduzieren.") {
            chk = chk.remedy(rem);
        }
    }
    r.push(chk.build());
}

/// 💪️ C-07..C-10 bending resistance of buildings and C-50 classification and resistance of bridge girders (EN 1994-2 6.2: plastic only for classes 1 and 2).
fn flexure(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let m_ed = c.uls.m_nm;
    let capacity = c.studs.capacity_n();
    let ncf = cat::plastic_full(&c.sec, c.fyd, c.ys, c.beff, c.hc, c.fcc).nc_n.max(1.0);
    let nc = capacity.min(ncf);
    let eta = part_1_1::degree_from_capacity(capacity, ncf);
    let sb = cat::plastic_sagging(&c.sec, c.fyd, c.ys, c.beff, c.hc, c.fcc, nc);
    let full = cat::plastic_full(&c.sec, c.fyd, c.ys, c.beff, c.hc, c.fcc);
    let m_pla = b.steel.w_pl_y_m3 * c.fyd;
    let class = part_1_1::section_class(&b.steel, c.f_y);
    let n0 = cat::modular_ratio(b.concrete_e_cm_pa, 0.0, 0.0);
    let m_a = if b.construction == semio_s_artifact_norm_en1994_model::ConstructionMethod::Unpropped { c.uls_c.m_nm.max(0.0) } else { 0.0 };
    let m_rd = if class <= 2 {
        sb.m_nm
    } else {
        let el = cat::transformed(&c.sec, c.ys, c.beff, c.hc, n0);
        cat::elastic_resistance(m_a, &c.sec, &el, c.ys, n0, c.fyd, c.fcc, 1.0).1.min(sb.m_nm)
    };
    let mut bending = head(a, format!("en1994.6.2.1.3.mrd.{}", b.id), "1994-1-1", "6.2.1.3", bref(b, "actions"), "Composite plastic bending resistance", "Plastische Verbund-Biegetragfähigkeit")
        .utilization(moment(m_ed), moment(m_rd))
        .explanation(lc(
            format!("{}: M_Ed = {:.1} kNm, M_Rd(η={eta:.2}, stress block, NA in {}) = {:.1} kNm (class {class}), M_pl,Rd = {:.1} kNm, b_eff = {:.3} m.", c.uls.label_en, m_ed / 1e3, sb.zone, m_rd / 1e3, full.m_nm / 1e3, c.beff),
            format!("{}: M_Ed = {:.1} kNm, M_Rd(η={eta:.2}, Spannungsblock, PNA im {}) = {:.1} kNm (Klasse {class}), M_pl,Rd = {:.1} kNm, b_eff = {:.3} m.", c.uls.label_de, m_ed / 1e3, sb.zone, m_rd / 1e3, full.m_nm / 1e3, c.beff),
        ))
        .sampled(STATIONS, |s| {
            let m = m_ed * relative(&c.profile, Effect::Sagging, s);
            (ratio(m, m_rd), vec![moment(m), moment(m_rd)])
        });
    if m_ed > m_rd {
        if let Some(rem) = load_fix(b, m_rd / m_ed.max(1.0) * 0.95, "Reduce the imposed floor load.", "Nutzlast reduzieren.") {
            bending = bending.remedy(rem);
        }
        bending = bending.remedy(Remedy::at_least(bref(b, "slabThicknessM"), length(b.slab_thickness_m), length((b.slab_thickness_m * m_ed / m_rd.max(1.0)).min(0.30)), lc("Increase slab thickness.", "Plattendicke erhöhen.")));
        let clearing = heavier_clearing(b, |t| {
            let tc = BeamCtxTrial::of(c, t);
            tc.m_rd_full() >= m_ed
        });
        let heavier = if clearing.is_empty() { SteelSection::heavier_heb_options(&b.steel.designation) } else { clearing };
        if !heavier.is_empty() {
            bending = bending.remedy(Remedy::one_of(bref(b, "steel.designation"), heavier, lc("Select a heavier steel section.", "Schwereren Stahlquerschnitt wählen.")));
        }
    }
    r.push(bending.build());

    let m_lin = cat::plastic_linear(m_pla, full.m_nm, eta);
    let mut lin = head(a, format!("en1994.6.2.1.3.linear.{}", b.id), "1994-1-1", "6.2.1.3", bref(b, "actions"), "Partial connection, linear interpolation (conservative)", "Teilverbund, lineare Interpolation (konservativ)")
        .utilization(moment(m_ed), moment(m_lin))
        .explanation(lc(format!("M_a,pl,Rd + η·(M_pl,Rd − M_a,pl,Rd) = {:.1} kNm ≤ stress-block {:.1} kNm.", m_lin / 1e3, sb.m_nm / 1e3), format!("M_a,pl,Rd + η·(M_pl,Rd − M_a,pl,Rd) = {:.1} kNm ≤ Spannungsblock {:.1} kNm.", m_lin / 1e3, sb.m_nm / 1e3)));
    if m_ed > m_lin {
        lin = lin.status(CheckStatus::Warning);
    }
    r.push(lin.build());

    if b.has_hogging() {
        let as_total = b.as_hogging_m2_per_m * c.beff_h;
        let fsd = cat::constants::REBAR_FSK / c.p.gamma_s;
        let (m_hog_rd, _) = cat::plastic_hogging(&c.sec, c.fyd, c.ys, as_total, fsd, cat::constants::HOGGING_REBAR_DEPTH_M);
        let m_hog = c.uls.m_hog_nm.abs();
        let mut hog = head(a, format!("en1994.6.2.1.2.hogging.{}", b.id), "1994-1-1", "6.2.1.2", bref(b, "asHoggingM2PerM"), "Hogging plastic bending resistance", "Plastische Biegetragfähigkeit im Stützbereich")
            .utilization(moment(m_hog), moment(m_hog_rd))
            .explanation(lc(format!("M_Ed,hog = {:.1} kNm, M_pl,Rd,hog = {:.1} kNm (A_s = {:.0} mm² over b_eff = {:.2} m).", m_hog / 1e3, m_hog_rd / 1e3, as_total * 1e6, c.beff_h), format!("M_Ed,Stütz = {:.1} kNm, M_pl,Rd,Stütz = {:.1} kNm (A_s = {:.0} mm² auf b_eff = {:.2} m).", m_hog / 1e3, m_hog_rd / 1e3, as_total * 1e6, c.beff_h)))
            .sampled(STATIONS, |s| {
                let m = m_hog * relative(&c.profile, Effect::Hogging, s);
                (ratio(m, m_hog_rd), vec![moment(m), moment(m_hog_rd)])
            });
        if m_hog > m_hog_rd {
            hog = hog.remedy(Remedy::at_least(bref(b, "asHoggingM2PerM"), area(b.as_hogging_m2_per_m), area(b.as_hogging_m2_per_m * (m_hog / m_hog_rd.max(1.0)).max(1.05) * 1.5), lc("Increase the hogging reinforcement.", "Stützbewehrung erhöhen.")));
            if let Some(rem) = load_fix(b, m_hog_rd / m_hog.max(1.0) * 0.95, "Reduce the imposed load.", "Nutzlast reduzieren.") {
                hog = hog.remedy(rem);
            }
        }
        r.push(hog.build());
    }

    let m_c = c.uls_c.m_nm;
    let mut cst = head(a, format!("en1994.6.2.1.construction.{}", b.id), "1994-1-1", "6.2.1", bref(b, "actions"), "Construction-stage steel bending", "Biegung Stahl im Bauzustand")
        .utilization(moment(m_c), moment(m_pla))
        .explanation(lc(format!("Construction {}: M_Ed,c = {:.1} kNm ({}), M_pl,a,Rd = {:.1} kNm.", c.uls_c.label_en, m_c / 1e3, b.construction, m_pla / 1e3), format!("Herstellung {}: M_Ed,c = {:.1} kNm ({}), M_pl,a,Rd = {:.1} kNm.", c.uls_c.label_de, m_c / 1e3, b.construction, m_pla / 1e3)));
    if m_c > m_pla {
        let cons: Vec<CharacteristicAction> = b.actions.iter().filter(|x| x.stage == semio_s_artifact_norm_en1994_model::ActionStage::Construction && x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Construction).cloned().collect();
        let fix = load_remedy(&b.id, &format!("beams[id={}]", b.id), lc(format!("Composite beam {}", b.id), format!("Verbundträger {}", b.id)), &cons, m_pla / m_c.max(1.0) * 0.9, "Reduce the construction load.", "Belastung im Bauzustand reduzieren.").or_else(|| load_fix(b, m_pla / m_c.max(1.0) * 0.9, "Reduce the load.", "Last reduzieren."));
        if let Some(rem) = fix {
            cst = cst.remedy(rem);
        }
        cst = cst.remedy(Remedy::at_most(bref(b, "spanM"), length(b.span_m), length(b.span_m * (m_pla / m_c.max(1.0)).sqrt() * 0.95), lc("Shorten the span or add temporary props.", "Spannweite verkürzen oder Hilfsstützen anordnen.")));
    }
    r.push(cst.build());
}

struct BeamCtxTrial {
    sec: cat::ISection,
    fyd: f64,
    fcc: f64,
    ys: f64,
    hc: f64,
    beff: f64,
}

impl BeamCtxTrial {
    fn of(c: &BeamCtx, t: &CompositeBeam) -> Self {
        Self { sec: part_1_1::isection(&t.steel), fyd: c.fyd, fcc: c.fcc, ys: c.ys, hc: c.hc, beff: c.beff }
    }
    fn m_rd_full(&self) -> f64 {
        cat::plastic_full(&self.sec, self.fyd, self.ys, self.beff, self.hc, self.fcc).m_nm
    }
}

fn studs_ref(beam: &CompositeBeam) -> SubjectRef {
    SubjectRef::new(beam.id.clone(), "studs", lc(format!("Shear studs of {}", beam.id), format!("Kopfbolzendübel von {}", beam.id)))
}

fn stud_ref(beam: &CompositeBeam, stud: &str, leaf: &str) -> SubjectRef {
    SubjectRef::new(beam.id.clone(), format!("studs[id={stud}].{leaf}"), lc(format!("Shear stud {stud} of {}", beam.id), format!("Kopfbolzendübel {stud} von {}", beam.id)))
}

/// 📈️ The elastic shear flow `v(x) ∝ |V(x)|` [N/m] that transfers `nc_req_n` over each half of the designed span.
fn elastic_flow<'a>(c: &'a BeamCtx, nc_req_n: f64) -> impl Fn(f64) -> f64 + 'a {
    let (lo, mid, hi) = (c.studs.origin_m, c.studs.mid_m(), c.studs.end_m());
    let shape = move |x: f64| relative(&c.profile, Effect::Shear, x / c.profile.total().max(1e-9));
    let integral = |a: f64, b: f64| (0..200).map(|k| shape(a + (b - a) * (k as f64 + 0.5) / 200.0)).sum::<f64>() * (b - a) / 200.0;
    let (left, right) = (integral(lo, mid).max(1e-9), integral(mid, hi).max(1e-9));
    move |x| nc_req_n * shape(x) / if x < mid { left } else { right }
}

impl BeamCtx<'_> {
    /// 🔩️ The stud a new stud copies: the last stud inside the span, or the typical stud when the beam has none.
    fn reference_stud(&self) -> Stud {
        let b = self.beam;
        self.studs.in_span().last().map_or_else(|| Stud::typical("", &b.id, 0.0), |r| Stud { id: String::new(), beam_id: b.id.clone(), x_m: r.x_m, diameter_m: r.diameter_m, height_m: r.height_m, f_u_pa: r.f_u_pa })
    }

    /// 🔩️ The resistance a new stud adds to a station of its own.
    fn reference_resistance_n(&self) -> f64 {
        part_studs::resistance_of(self.beam, &self.reference_stud(), 1, &self.p).3
    }

    /// 🆔️ The first free id `<beam>-stud-<n>`.
    fn fresh_stud_id(&self) -> String {
        (0..).map(|n| format!("{}-stud-{n}", self.beam.id)).find(|id| !self.doc.studs.iter().any(|s| &s.id == id)).unwrap_or_default()
    }

    /// ➕ The remedy that inserts one stud like the reference stud at the next position of the weaker half span.
    fn insert_remedy(&self, en: String, de: String) -> Remedy {
        let (b, s) = (self.beam, self.reference_stud());
        let row = format!(r#"{{"id":"{}","beamId":"{}","xM":"$required","diameterM":{},"heightM":{},"fUPa":{}}}"#, self.fresh_stud_id(), b.id, s.diameter_m, s.height_m, s.f_u_pa);
        Remedy::insert(studs_ref(b), row, length(self.studs.next_position_m()), lc(en, de))
    }
}

/// 🔩️ C-15..C-20 shear connection of one beam from its persisted studs: spacing and layout (6.6.5.5, 6.6.1.1), detailing (6.6.5), the resistance of the weaker half span (6.6.3.1, 6.6.4), the force of every stud (6.6.2) and the minimum degree of connection (6.6.1.2), then the longitudinal shear of the slab (6.6.6).
fn connection(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let hp = b.sheeting.height_m;
    let origin = c.studs.origin_m;
    let (_, s_max) = part_1_1::stud_spacing_limits_m(b, 0.0);
    let gaps = c.studs.gaps();
    let worst_gap = gaps.iter().map(|g| (g, cat::stud_spacing_rule(g.diameter_m, c.hc, g.gap_m))).max_by(|x, y| x.1 .0.total_cmp(&y.1 .0));
    let mut ssp = head(a, format!("en1994.6.6.5.5.spacing.{}", b.id), "1994-1-1", "6.6.5.5", worst_gap.map_or_else(|| studs_ref(b), |(g, _)| stud_ref(b, &g.after, "xM")), "Stud longitudinal spacing limits", "Grenzwerte Bolzenlängsabstand");
    if let Some((g, (u, rule))) = worst_gap {
        let (s_min, _) = part_1_1::stud_spacing_limits_m(b, g.diameter_m);
        ssp = ssp.utilization(dim(u), dim(1.0)).explanation(lc(
            format!("{} gaps between stud stations, governing {:.0} mm between {} and {} within {:.0}–{:.0} mm required ({rule}); unverified: s_max = min(6·h_c, 800 mm) is the stricter reading, 6.6.5.5(3) may relate the limit to the total slab thickness (clause text not in the repo).", gaps.len(), g.gap_m * 1000.0, g.before, g.after, s_min * 1000.0, s_max * 1000.0),
            format!("{} Abstände zwischen den Bolzenstationen, maßgebend {:.0} mm zwischen {} und {} innerhalb {:.0}–{:.0} mm gefordert ({rule}); ungeprüft: s_max = min(6·h_c, 800 mm) ist die strengere Lesart, 6.6.5.5(3) kann den Grenzwert auf die gesamte Plattendicke beziehen (Klauseltext nicht im Repo).", gaps.len(), g.gap_m * 1000.0, g.before, g.after, s_min * 1000.0, s_max * 1000.0),
        ));
        if u > 1.0 {
            ssp = ssp.remedy(Remedy::exactly(stud_ref(b, &g.after, "xM"), length(g.after_x_m), length(g.after_x_m - g.gap_m + g.gap_m.clamp(s_min, s_max)), lc(format!("Move stud {} to a gap within {:.0}–{:.0} mm.", g.after, s_min * 1000.0, s_max * 1000.0), format!("Bolzen {} auf einen Abstand von {:.0}–{:.0} mm verschieben.", g.after, s_min * 1000.0, s_max * 1000.0))));
        }
    } else {
        ssp = ssp.not_applicable(lc("Fewer than two stud stations inside the span.", "Weniger als zwei Bolzenstationen innerhalb der Stützweite."));
    }
    r.push(ssp.build());

    let total = c.studs.rows.len();
    let outside: Vec<&part_studs::StudResistance> = c.studs.outside_span().collect();
    let mut lay = head(a, format!("en1994.6.6.1.1.layout.{}", b.id), "1994-1-1", "6.6.1.1", studs_ref(b), "Studs lie inside the span", "Bolzen liegen innerhalb der Stützweite")
        .utilization(dim(total as f64), dim((total - outside.len()).max(1) as f64))
        .explanation(lc(format!("{} of {total} studs lie inside the span [{:.2}, {:.2}] m, {} outside.", total - outside.len(), origin, c.studs.end_m(), outside.len()), format!("{} von {total} Bolzen liegen innerhalb der Stützweite [{:.2}, {:.2}] m, {} außerhalb.", total - outside.len(), origin, c.studs.end_m(), outside.len())));
    if let Some(first) = outside.first() {
        lay = lay.status(CheckStatus::Warning).remedy(Remedy::exactly(stud_ref(b, &first.id, "xM"), length(first.x_m), length(first.x_m.clamp(origin, c.studs.end_m())), lc(format!("Move stud {} into the span.", first.id), format!("Bolzen {} in die Stützweite verschieben.", first.id))));
    }
    r.push(lay.build());

    let geometry = c.studs.rows.iter().map(|s| (s, cat::stud_detailing_geometry(s.diameter_m, s.height_m, hp, b.sheeting.thickness_m))).max_by(|x, y| x.1 .0.total_cmp(&y.1 .0));
    let (det, rule, governing) = match (geometry, worst_gap) {
        (Some((s, (gu, gr))), Some((g, (su, sr)))) => if su > gu { (su, sr, Some(g.after.clone())) } else { (gu, gr, Some(s.id.clone())) },
        (Some((s, (gu, gr))), None) => (gu, gr, Some(s.id.clone())),
        (None, _) => (0.0, "none", None),
    };
    let mut dt = head(a, format!("en1994.6.6.5.detailing.{}", b.id), "1994-1-1", "6.6.5", governing.as_ref().map_or_else(|| studs_ref(b), |id| stud_ref(b, id, "diameterM")), "Stud detailing rules", "Konstruktionsregeln Kopfbolzen")
        .utilization(dim(det), dim(1.0))
        .explanation(lc(format!("Governing rule: {rule} (utilisation {det:.2})."), format!("Maßgebende Regel: {rule} (Ausnutzung {det:.2}).")));
    if let Some(id) = governing.as_ref().filter(|_| det > 1.0) {
        if let Some(stud) = c.studs.rows.iter().find(|s| &s.id == id) {
            dt = if rule.starts_with("hsc") {
                dt.remedy(Remedy::at_least(stud_ref(b, id, "heightM"), length(stud.height_m), length(hp + 2.0 * stud.diameter_m + 0.005), lc("Use longer studs (h_sc ≥ h_p + 2d).", "Längere Bolzen verwenden (h_sc ≥ h_p + 2d).")))
            } else if rule.starts_with("d <=") {
                dt.remedy(Remedy::at_most(stud_ref(b, id, "diameterM"), length(stud.diameter_m), length(0.025), lc("Use studs of at most 25 mm.", "Bolzen von höchstens 25 mm verwenden.")))
            } else if rule.starts_with("t <=") {
                dt.remedy(Remedy::at_most(bref(b, "sheeting.thicknessM"), length(b.sheeting.thickness_m), length(0.00125), lc("Use sheeting of at most 1.25 mm.", "Profilblech von höchstens 1,25 mm verwenden.")))
            } else if let Some((g, _)) = worst_gap {
                let (s_min, _) = part_1_1::stud_spacing_limits_m(b, g.diameter_m);
                dt.remedy(Remedy::exactly(stud_ref(b, &g.after, "xM"), length(g.after_x_m), length(g.after_x_m - g.gap_m + g.gap_m.clamp(s_min, s_max)), lc("Bring the stud spacing into the allowed range.", "Bolzenabstand in den zulässigen Bereich bringen.")))
            } else {
                dt
            };
        }
    }
    r.push(dt.build());

    let p_rd = c.studs.mean_resistance_n();
    let (p1, p2) = c.studs.in_span().next().map_or((0.0, 0.0), |s| (s.p_rd1_n, s.p_rd2_n));
    let ncf = cat::plastic_full(&c.sec, c.fyd, c.ys, c.beff, c.hc, c.fcc).nc_n.max(1.0);
    let nc_req = cat::required_concrete_force(&c.sec, c.fyd, c.ys, c.beff, c.hc, c.fcc, c.uls.m_nm);
    let cap = c.studs.capacity_n();
    let kt = c.studs.min_reduction();
    let n_half = c.studs.half_count();
    let p_ref = c.reference_resistance_n().max(1.0);
    let mut stud = head(a, format!("en1994.6.6.3.1.prd.{}", b.id), "1994-1-1", "6.6.3.1", studs_ref(b), "Headed stud resistance P_Rd vs moment-dependent demand", "Tragfähigkeit Kopfbolzen P_Rd gegen momentenabhängige Beanspruchung")
        .utilization(force(nc_req), force(cap))
        .explanation(lc(
            format!("N_c,req(M_Ed) = {:.0} kN, {n_half:.1} studs per half span, Σ P_Rd = {:.1} kN (mean P_Rd = {:.1} kN, P_Rd,1 = {:.1}, P_Rd,2 = {:.1}, k_t/k_l = {kt:.2}).", nc_req / 1e3, cap / 1e3, p_rd / 1e3, p1 / 1e3, p2 / 1e3),
            format!("N_c,req(M_Ed) = {:.0} kN, {n_half:.1} Bolzen je halber Stützweite, Σ P_Rd = {:.1} kN (mittleres P_Rd = {:.1} kN, P_Rd,1 = {:.1}, P_Rd,2 = {:.1}, k_t/k_l = {kt:.2}).", nc_req / 1e3, cap / 1e3, p_rd / 1e3, p1 / 1e3, p2 / 1e3),
        ))
        .sampled(STATIONS, |s| {
            let flow = relative(&c.profile, Effect::Shear, s);
            (ratio(nc_req, cap) * flow, vec![force(nc_req * flow), force(cap)])
        });
    if kt <= 0.0 && total > 0 {
        stud = stud.not_applicable(lc("Sheeting reduction out of scope (n_r > 2 studs per rib).", "Blechabminderung außerhalb des Anwendungsbereichs (n_r > 2 Bolzen je Rippe)."));
    } else if nc_req > cap {
        let more = ((nc_req - cap) / p_ref).ceil() as u32;
        stud = stud.remedy(c.insert_remedy(format!("Insert the next stud (≥ {more} more per half span needed)."), format!("Nächsten Bolzen einfügen (≥ {more} weitere je halber Stützweite nötig).")));
    }
    r.push(stud.build());

    let ductile = total > 0 && c.studs.in_span().all(|s| cat::studs_ductile(s.diameter_m, s.height_m));
    let flow: Box<dyn Fn(f64) -> f64 + '_> = if ductile {
        let uniform = part_studs::plastic_flow_n_per_m(nc_req, b.span_m);
        Box::new(move |_| uniform)
    } else {
        Box::new(elastic_flow(c, nc_req))
    };
    let forces = c.studs.forces(&*flow);
    let governing_force = forces.iter().max_by(|x, y| x.utilisation().total_cmp(&y.utilisation()));
    let l_total = c.profile.total().max(1e-9);
    let mut per = head(a, format!("en1994.6.6.2.perstud.{}", b.id), "1994-1-1", "6.6.2", governing_force.map_or_else(|| studs_ref(b), |g| stud_ref(b, &g.id, "diameterM")), "Longitudinal shear force per headed stud", "Längsschubkraft je Kopfbolzen");
    per = match governing_force {
        Some(g) if kt > 0.0 => {
            let u = g.utilisation();
            let mut checked = per
                .utilization(force(g.force_n), force(g.resistance_n))
                .explanation(lc(
                    format!("Stud {} at x = {:.2} m carries P_Ed = {:.1} kN against P_Rd = {:.1} kN ({} flow, {} studs, k_t/k_l ≥ {kt:.2}).", g.id, g.x_m, g.force_n / 1e3, g.resistance_n / 1e3, if ductile { "uniform plastic" } else { "elastic" }, forces.len()),
                    format!("Bolzen {} bei x = {:.2} m trägt P_Ed = {:.1} kN gegen P_Rd = {:.1} kN ({} Fluss, {} Bolzen, k_t/k_l ≥ {kt:.2}).", g.id, g.x_m, g.force_n / 1e3, g.resistance_n / 1e3, if ductile { "gleichmäßig plastischer" } else { "elastischer" }, forces.len()),
                ))
                .sampled(STATIONS, |s| {
                    let x = s * l_total;
                    forces.iter().min_by(|p, q| (p.x_m - x).abs().total_cmp(&(q.x_m - x).abs())).map_or((0.0, Vec::new()), |n| (n.utilisation(), vec![force(n.force_n), force(n.resistance_n)]))
                });
            if u > 1.0 {
                let own = c.studs.rows.iter().find(|s| s.id == g.id).filter(|s| s.diameter_m * u.sqrt() <= 0.025);
                checked = match own {
                    Some(s) => checked.remedy(Remedy::at_least(stud_ref(b, &g.id, "diameterM"), length(s.diameter_m), length(s.diameter_m * u.sqrt() * 1.001), lc(format!("Increase the diameter of stud {}.", g.id), format!("Durchmesser von Bolzen {} erhöhen.", g.id)))),
                    None => checked.remedy(c.insert_remedy(format!("Insert a stud next to stud {}.", g.id), format!("Einen Bolzen neben Bolzen {} einfügen.", g.id))),
                };
            }
            checked
        }
        Some(_) => per.not_applicable(lc("Sheeting reduction out of scope (n_r > 2 studs per rib).", "Blechabminderung außerhalb des Anwendungsbereichs (n_r > 2 Bolzen je Rippe).")),
        None => per.not_applicable(lc("No studs inside the span.", "Keine Bolzen innerhalb der Stützweite.")),
    };
    r.push(per.build());

    let eta_min = cat::eta_min(c.le, c.f_y, ductile);
    let eta = part_1_1::degree_from_capacity(cap, ncf);
    let mut etac = head(a, format!("en1994.6.6.1.2.etamin.{}", b.id), "1994-1-1", "6.6.1.2", studs_ref(b), "Minimum degree of shear connection", "Mindestverdübelungsgrad")
        .minimum(dim(eta), dim(eta_min))
        .explanation(lc(format!("η = {eta:.3}, η_min = {eta_min:.3} (L_e = {:.2} m, {} studs).", c.le, if ductile { "ductile" } else { "non-ductile" }), format!("η = {eta:.3}, η_min = {eta_min:.3} (L_e = {:.2} m, {} Bolzen).", c.le, if ductile { "duktile" } else { "nicht duktile" })));
    if eta < eta_min {
        let more = ((eta_min * ncf - cap) / p_ref).ceil() as u32;
        etac = etac.remedy(c.insert_remedy(format!("Insert the next stud (≥ {more} more per half span so η ≥ η_min)."), format!("Nächsten Bolzen einfügen (≥ {more} weitere je halber Stützweite, damit η ≥ η_min).")));
    }
    r.push(etac.build());

    let hf = c.hc;
    let l_crit = (c.le / 2.0).max(1e-6);
    let (vl, _) = cat::connector_flow(nc_req, l_crit, p_rd);
    let l_surf = 2.0 * hf;
    let (v_ed, v_rd_max, asf_req) = cat::slab_shear(vl, l_surf, hf, b.concrete_f_ck_pa, c.p.gamma_c, c.p.gamma_s, 1.0);
    let u_max = v_ed / v_rd_max.max(1.0);
    let u_as = asf_req / b.transverse_as_m2_per_m.max(1e-12);
    let mut vlc = head(a, format!("en1994.6.6.6.vlrd.{}", b.id), "1994-1-1", "6.6.6", bref(b, "transverseAsM2PerM"), "Longitudinal shear in the slab", "Längsschub in der Gurtplatte")
        .utilization(if u_max > u_as { stress(v_ed) } else { area(asf_req) }, if u_max > u_as { stress(v_rd_max) } else { area(b.transverse_as_m2_per_m) })
        .explanation(lc(format!("v_Ed = {:.2} MPa (shear flow {:.0} kN/m over 2·h_f), v_Rd,max = {:.2} MPa, A_sf/s_f required {:.0} mm²/m, provided {:.0} mm²/m.", v_ed / 1e6, vl / 1e3, v_rd_max / 1e6, asf_req * 1e6, b.transverse_as_m2_per_m * 1e6), format!("v_Ed = {:.2} MPa (Schubfluss {:.0} kN/m über 2·h_f), v_Rd,max = {:.2} MPa, A_sf/s_f erforderlich {:.0} mm²/m, vorhanden {:.0} mm²/m.", v_ed / 1e6, vl / 1e3, v_rd_max / 1e6, asf_req * 1e6, b.transverse_as_m2_per_m * 1e6)))
        .sampled(STATIONS, |s| {
            let flow = relative(&c.profile, Effect::Shear, s);
            (u_max.max(u_as) * flow, vec![force(vl * flow)])
        });
    if u_as > 1.0 {
        vlc = vlc.remedy(Remedy::at_least(bref(b, "transverseAsM2PerM"), area(b.transverse_as_m2_per_m), area(asf_req * 1.02), lc(format!("Increase transverse reinforcement to ≥ {:.0} mm²/m.", asf_req * 1.02e6), format!("Querbewehrung auf ≥ {:.0} mm²/m erhöhen.", asf_req * 1.02e6))));
    }
    if u_max > 1.0 {
        vlc = vlc.remedy(Remedy::at_least(bref(b, "slabThicknessM"), length(b.slab_thickness_m), length((b.slab_thickness_m * u_max * 1.05).min(0.40)), lc("Increase the slab thickness (concrete strut).", "Plattendicke erhöhen (Druckstrebe).")));
    }
    r.push(vlc.build());
}

fn sagging_interaction(r: &mut CheckReport, c: &BeamCtx, v_ed: f64, v_pl: f64) {
    let (b, a) = (c.beam, c.annex);
    let plastic = cat::plastic_full(&c.sec, c.fyd, c.ys, c.beff, c.hc, c.fcc);
    let at = |s: f64| {
        let m = c.uls.m_nm * relative(&c.profile, Effect::Sagging, s);
        let v = v_ed * relative(&c.profile, Effect::Shear, s);
        let (m_v, rho) = cat::shear_reduced_sagging_moment(&plastic, &c.sec, c.ys, c.beff, c.hc, c.fcc, c.fyd, v, v_pl);
        (m, v, m_v, rho)
    };
    let worst = (0..=200).map(|k| k as f64 / 200.0).map(|s| { let (m, _, m_v, _) = at(s); ratio(m, m_v) }).fold(0.0, f64::max);
    let rho_max = (0..=200).map(|k| at(k as f64 / 200.0).3).fold(0.0, f64::max);
    let mut vm = head(a, format!("en1994.6.2.2.4.vm-sagging.{}", b.id), "1994-1-1", "6.2.2.4", bref(b, "steel.designation"), "Bending–shear interaction in the sagging region", "Interaktion Biegung–Querkraft im Feldbereich")
        .utilization(dim(worst), dim(1.0))
        .explanation(lc(
            format!("Largest ρ = {rho_max:.3} along the span for V_pl,Rd = {:.0} kN (web strength reduced by 1 − ρ above V_Ed = ½·V_pl,Rd); M_Ed(x) ≤ M_V,Rd(x) with M_pl,Rd = {:.1} kNm.", v_pl / 1e3, plastic.m_nm / 1e3),
            format!("Größtes ρ = {rho_max:.3} entlang des Feldes für V_pl,Rd = {:.0} kN (Stegfestigkeit um 1 − ρ abgemindert ab V_Ed = ½·V_pl,Rd); M_Ed(x) ≤ M_V,Rd(x) mit M_pl,Rd = {:.1} kNm.", v_pl / 1e3, plastic.m_nm / 1e3),
        ))
        .sampled(STATIONS, |s| {
            let (m, _, m_v, _) = at(s);
            (ratio(m, m_v), vec![moment(m), moment(m_v)])
        });
    if worst > 1.0 {
        let heavier = SteelSection::heavier_heb_options(&b.steel.designation);
        if !heavier.is_empty() {
            vm = vm.remedy(Remedy::one_of(bref(b, "steel.designation"), heavier, lc("Select a heavier steel section.", "Schwereren Stahlquerschnitt wählen.")));
        }
    }
    r.push(vm.build());
}

fn shear(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let v_ed = c.uls.v_n.abs();
    let v_pl = part_1_1::vertical_shear_resistance_n(b, c.f_y, a);
    let sb = part_1_1::shear_buckling_util(&b.steel, c.f_y);
    let mut v = head(a, format!("en1994.6.2.2.vpl.{}", b.id), "1994-1-1", "6.2.2", bref(b, "steel.twM"), "Vertical shear resistance", "Querkrafttragfähigkeit")
        .utilization(force(v_ed), force(v_pl))
        .explanation(lc(format!("{}: V_Ed = {:.1} kN, V_pl,a,Rd = {:.1} kN, h_w/t_w utilisation {sb:.2}.", c.uls.label_en, v_ed / 1e3, v_pl / 1e3), format!("{}: V_Ed = {:.1} kN, V_pl,a,Rd = {:.1} kN, h_w/t_w-Ausnutzung {sb:.2}.", c.uls.label_de, v_ed / 1e3, v_pl / 1e3)))
        .sampled(STATIONS, |s| {
            let v = v_ed * relative(&c.profile, Effect::Shear, s);
            (ratio(v, v_pl), vec![force(v), force(v_pl)])
        });
    if v_ed > v_pl || sb > 1.0 {
        let clearing = heavier_clearing(b, |t| v_ed <= part_1_1::vertical_shear_resistance_n(t, c.f_y, a) && part_1_1::shear_buckling_util(&t.steel, c.f_y) <= 1.0);
        if !clearing.is_empty() {
            v = v.remedy(Remedy::one_of(bref(b, "steel.designation"), clearing, lc("Select a catalogue section whose web covers V_Ed.", "Katalogquerschnitt wählen, dessen Steg V_Ed deckt.")));
        } else if let Some(rem) = load_fix(b, v_pl / v_ed.max(1.0) * 0.98, "Reduce the area load so V_Ed ≤ V_pl,Rd.", "Flächenlast senken, damit V_Ed ≤ V_pl,Rd.") {
            v = v.remedy(rem);
        }
        v = v.remedy(Remedy::at_least(bref(b, "steel.twM"), length(b.steel.tw_m), length(b.steel.tw_m * (v_ed / v_pl.max(1.0)).max(sb).max(1.05)), lc("Increase web thickness t_w (custom sections).", "Stegdicke t_w erhöhen (Sonderprofile).")));
    }
    r.push(v.build());
    sagging_interaction(r, c, v_ed, v_pl);

    if b.has_hogging() {
        let m_hog = c.uls.m_hog_nm.abs();
        let as_total = b.as_hogging_m2_per_m * c.beff_h;
        let fsd = cat::constants::REBAR_FSK / c.p.gamma_s;
        let (m_rd, _) = cat::plastic_hogging(&c.sec, c.fyd, c.ys, as_total, fsd, cat::constants::HOGGING_REBAR_DEPTH_M);
        let (m_v, rho) = cat::shear_reduced_hogging_moment(m_rd, &c.sec, c.ys, c.fyd, as_total, fsd, cat::constants::HOGGING_REBAR_DEPTH_M, v_ed, v_pl);
        let mut vm = head(a, format!("en1994.6.2.2.4.vm.{}", b.id), "1994-1-1", "6.2.2.4", bref(b, "asHoggingM2PerM"), "Bending–shear interaction at the support", "Interaktion Biegung–Querkraft am Auflager")
            .utilization(moment(m_hog), moment(m_v))
            .explanation(lc(format!("ρ = {rho:.3} for V_Ed = {:.0} kN of V_pl,Rd = {:.0} kN; M_V,Rd = {:.1} kNm.", v_ed / 1e3, v_pl / 1e3, m_v / 1e3), format!("ρ = {rho:.3} für V_Ed = {:.0} kN von V_pl,Rd = {:.0} kN; M_V,Rd = {:.1} kNm.", v_ed / 1e3, v_pl / 1e3, m_v / 1e3)));
        if m_hog > m_v {
            vm = vm.remedy(Remedy::at_least(bref(b, "asHoggingM2PerM"), area(b.as_hogging_m2_per_m), area(b.as_hogging_m2_per_m * (m_hog / m_v.max(1.0)).max(1.05) * 1.5), lc("Increase the hogging reinforcement.", "Stützbewehrung erhöhen.")));
            if let Some(rem) = load_fix(b, m_v / m_hog.max(1.0) * 0.95, "Reduce the load.", "Last reduzieren.") {
                vm = vm.remedy(rem);
            }
        }
        r.push(vm.build());
    }
}

/// 🦋️ C-23 lateral-torsional buckling of continuous girders and C-51 of bridge girders as inverted U-frames (EN 1994-2 6.4).
fn ltb(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let construction = b.construction == semio_s_artifact_norm_en1994_model::ConstructionMethod::Unpropped && c.uls_c.m_nm > 1.0;
    let hogging = c.uls.m_hog_nm > 1.0;
    if !(construction || hogging) {
        r.push(head(a, format!("en1994.6.4.ltb.{}", b.id), "1994-1-1", "6.4", bref(b, "construction"), "Lateral-torsional buckling", "Biegedrillknicken").not_applicable(lc("§6.4 not applicable: propped construction without hogging demand.", "§6.4 nicht anwendbar: mit Hilfsstützen ohne Stützmoment.")).build());
        return;
    }
    let use_construction = construction && (!hogging || c.uls_c.m_nm >= c.uls.m_hog_nm);
    let (m_ed, leaf, en, de) = if use_construction { (c.uls_c.m_nm, "construction", "construction stage (steel alone, M_cr bare)", "Herstellung (nur Stahl, M_cr ohne Platte)") } else { (c.uls.m_hog_nm, "support", "hogging region (inverted U-frame M_cr)", "Stützbereich (umgekehrter U-Rahmen M_cr)") };
    let resist = |t: &CompositeBeam| if use_construction { part_1_1::ltb_moment_resistance_nm(t, c.f_y, a) } else { part_1_1::ltb_hogging(t, c.f_y, a).3 };
    let m_b = resist(b);
    let (mcr, lam, chi) = if use_construction {
        let mcr = cat::mcr_bare(&c.sec, b.ltb_length_m.max(0.05));
        let (l, x) = cat::chi_lt(part_1_1::ltb_modulus_m3(b, c.f_y) * c.f_y, mcr, c.sec.h / c.sec.b, part_1_1::is_welded(b));
        (mcr, l, x)
    } else {
        let h = part_1_1::ltb_hogging(b, c.f_y, a);
        (h.0, h.1, h.2)
    };
    let mut l = head(a, format!("en1994.6.4.ltb.{}", b.id), "1994-1-1", "6.4", bref(b, leaf), "Lateral-torsional buckling", "Biegedrillknicken")
        .utilization(moment(m_ed), moment(m_b))
        .explanation(lc(format!("LTB {en}: M_Ed = {:.1} kNm, M_cr = {:.1} kNm, λ_LT = {lam:.2}, χ_LT = {chi:.2}, M_b,Rd = {:.1} kNm.", m_ed / 1e3, mcr / 1e3, m_b / 1e3), format!("BDK {de}: M_Ed = {:.1} kNm, M_cr = {:.1} kNm, λ_LT = {lam:.2}, χ_LT = {chi:.2}, M_b,Rd = {:.1} kNm.", m_ed / 1e3, mcr / 1e3, m_b / 1e3)))
        .sampled(STATIONS, |s| {
            let effect = if use_construction { Effect::Sagging } else { Effect::Hogging };
            let m = m_ed * relative(&c.profile, effect, s);
            (ratio(m, m_b), vec![moment(m), moment(m_b)])
        });
    if m_ed > m_b {
        let mut short = b.clone();
        short.ltb_length_m = 0.05;
        if resist(&short) >= m_ed {
            let (mut lo, mut hi) = (0.05_f64, b.ltb_length_m.max(0.05));
            for _ in 0..40 {
                let mid = 0.5 * (lo + hi);
                let mut t = b.clone();
                t.ltb_length_m = mid;
                if resist(&t) >= m_ed { lo = mid } else { hi = mid }
            }
            l = l.remedy(Remedy::at_most(bref(b, "ltbLengthM"), length(b.ltb_length_m), length(lo), lc("Reduce the LTB length (add restraints).", "Biegedrillknicklänge verkürzen (Aussteifungen).")));
        } else {
            let clearing = heavier_clearing(b, |t| resist(t) >= m_ed);
            if !clearing.is_empty() {
                l = l.remedy(Remedy::one_of(bref(b, "steel.designation"), clearing, lc("Select a heavier steel section so M_b,Rd ≥ M_Ed.", "Schwereren Stahlquerschnitt wählen, damit M_b,Rd ≥ M_Ed.")));
            }
            if let Some(rem) = load_fix(b, m_b / m_ed.max(1.0) * 0.98, "Reduce the governing area load so LTB M_Ed ≤ M_b,Rd.", "Maßgebende Flächenlast absenken, damit M_Ed ≤ M_b,Rd.") {
                l = l.remedy(rem);
            }
        }
    }
    r.push(l.build());
}

fn sls_stress(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let (span, support, spacing) = (b.span_m, b.support, b.spacing_m);
    let n0 = cat::modular_ratio(b.concrete_e_cm_pa, 0.0, 0.0);
    let nl = cat::modular_ratio(b.concrete_e_cm_pa, cat::constants::PSI_L_PERMANENT, cat::constants::CREEP_PHI);
    let perm: Vec<CharacteristicAction> = b.actions.iter().filter(|x| x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent).cloned().collect();
    let m_g = part_en1990::stage_effects(&perm, span, b.adjacent_span_m, support, spacing, "composite", a, c.reliability_class, "sls_char").m_nm;
    let m_q = (c.sls_char.m_nm - m_g).max(0.0);
    let m_a = if b.construction == semio_s_artifact_norm_en1994_model::ConstructionMethod::Unpropped { part_en1990::stage_effects(&b.actions, span, b.adjacent_span_m, support, spacing, "construction", a, c.reliability_class, "sls_char").m_nm } else { 0.0 };
    let (sa_t, sa_b) = cat::steel_stresses(&c.sec, m_a);
    let el_l = cat::transformed(&c.sec, c.ys, c.beff, c.hc, nl);
    let el_0 = cat::transformed(&c.sec, c.ys, c.beff, c.hc, n0);
    let (cg, tg, bg) = cat::composite_stresses(&el_l, &c.sec, c.ys, nl, m_g);
    let (cq, tq, bq) = cat::composite_stresses(&el_0, &c.sec, c.ys, n0, m_q);
    let sigma_s = (sa_t + tg + tq).abs().max((sa_b + bg + bq).abs());
    let sigma_c = (cg + cq).max(0.0);
    let u_s = sigma_s / c.f_y;
    let u_c = sigma_c / (0.6 * b.concrete_f_ck_pa);
    let (computed, limit) = if u_s >= u_c { (sigma_s, c.f_y) } else { (sigma_c, 0.6 * b.concrete_f_ck_pa) };
    let mut s = head(a, format!("en1994.7.2.2.stress.{}", b.id), "1994-1-1", "7.2.2", bref(b, "actions"), "SLS characteristic stress limitation (staged)", "GZG charakteristische Spannungsbegrenzung (phasenweise)")
        .utilization(stress(computed), stress(limit))
        .explanation(lc(format!("{}: M_a = {:.1}, M_G = {:.1}, M_Q = {:.1} kNm; σ_a = {:.1} MPa ≤ f_y = {:.0} MPa, σ_c = {:.1} MPa ≤ 0.6 f_ck = {:.1} MPa.", c.sls_char.label_en, m_a / 1e3, m_g / 1e3, m_q / 1e3, sigma_s / 1e6, c.f_y / 1e6, sigma_c / 1e6, 0.6 * b.concrete_f_ck_pa / 1e6), format!("{}: M_a = {:.1}, M_G = {:.1}, M_Q = {:.1} kNm; σ_a = {:.1} MPa ≤ f_y = {:.0} MPa, σ_c = {:.1} MPa ≤ 0,6 f_ck = {:.1} MPa.", c.sls_char.label_de, m_a / 1e3, m_g / 1e3, m_q / 1e3, sigma_s / 1e6, c.f_y / 1e6, sigma_c / 1e6, 0.6 * b.concrete_f_ck_pa / 1e6)));
    if computed > limit {
        if let Some(rem) = load_fix(b, limit / computed.max(1.0) * 0.98, "Reduce the area load so the SLS stresses stay within the limits.", "Flächenlast senken, damit die GZG-Spannungen eingehalten werden.") {
            s = s.remedy(rem);
        }
        s = s.remedy(Remedy::at_most(bref(b, "spanM"), length(b.span_m), length(b.span_m * (limit / computed.max(1.0)).sqrt() * 0.98), lc("Shorten the span.", "Spannweite verkürzen.")));
    }
    r.push(s.build());
}

fn governing(effects: &part_en1990::DesignEffects, position: cat::SpanPosition) -> f64 {
    if position == cat::SpanPosition::Cantilever { effects.m_hog_nm.abs() } else { effects.m_nm }
}

fn unit_moment(kind: semio_s_artifact_norm_en1994_model::ActionKind, b: &CompositeBeam) -> f64 {
    let action = CharacteristicAction { q_area_pa: 1.0, ..CharacteristicAction::point_force("", semio_s_artifact_norm_en1994_model::ActionStage::Composite, kind, "", 0.0) };
    let (m_span, m_hog, _, _) = part_en1990::action_internals(&action, b.span_m, b.adjacent_span_m, b.support, 1.0);
    if cat::span_position(b.support) == cat::SpanPosition::Cantilever { m_hog.abs().max(1e-9) } else { m_span.max(1e-9) }
}

fn stiffness(c: &BeamCtx, i_uncracked: f64) -> cat::continuous::Stiffness {
    let ei1 = cat::constants::E_A * i_uncracked;
    let as_total = c.beam.as_hogging_m2_per_m * c.beff_h;
    let ei2 = cat::constants::E_A * cat::cracked(&c.sec, c.ys, as_total, cat::constants::HOGGING_REBAR_DEPTH_M).i;
    cat::continuous::Stiffness { ei1, ei2: ei2.min(ei1), cracked_fraction: c.beam.cracked_length_fraction.clamp(0.0, 0.5) }
}

fn deflection(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let position = cat::span_position(b.support);
    let cantilever = position == cat::SpanPosition::Cantilever;
    let (span, support, spacing, neighbour) = (b.span_m, b.support, b.spacing_m, b.neighbour_span_m());
    let n0 = cat::modular_ratio(b.concrete_e_cm_pa, 0.0, 0.0);
    let nl = cat::modular_ratio(b.concrete_e_cm_pa, cat::constants::PSI_L_PERMANENT, cat::constants::CREEP_PHI);
    let ns = cat::modular_ratio(b.concrete_e_cm_pa, cat::constants::PSI_L_SHRINKAGE, cat::constants::CREEP_PHI);
    let perm: Vec<CharacteristicAction> = b.actions.iter().filter(|x| x.kind == semio_s_artifact_norm_en1994_model::ActionKind::Permanent).cloned().collect();
    let m_g = governing(&part_en1990::stage_effects(&perm, span, b.adjacent_span_m, support, spacing, "composite", a, c.reliability_class, "sls_char"), position);
    let g = m_g / unit_moment(semio_s_artifact_norm_en1994_model::ActionKind::Permanent, b);
    let q = (governing(&c.sls_freq, position) - m_g).max(0.0) / unit_moment(semio_s_artifact_norm_en1994_model::ActionKind::Imposed, b);
    let i_l = cat::transformed(&c.sec, c.ys, c.beff, c.hc, nl).i;
    let i_0 = cat::transformed(&c.sec, c.ys, c.beff, c.hc, n0).i;
    let defl = |load: f64, pattern: cat::continuous::Pattern, stiff: cat::continuous::Stiffness| cat::continuous::span_deflection_m(position, span, neighbour, load, pattern, stiff);
    let d_perm = defl(g, cat::continuous::Pattern::AllSpans, stiffness(c, i_l));
    let d_var = defl(q, cat::continuous::Pattern::OwnSpan, stiffness(c, i_0));
    let d_c = if b.construction == semio_s_artifact_norm_en1994_model::ConstructionMethod::Unpropped {
        let qc = governing(&part_en1990::stage_effects(&b.actions, span, b.adjacent_span_m, support, spacing, "construction", a, c.reliability_class, "sls_char"), position) / unit_moment(semio_s_artifact_norm_en1994_model::ActionKind::Permanent, b);
        defl(qc, cat::continuous::Pattern::AllSpans, cat::continuous::Stiffness::uniform(cat::constants::E_A * c.sec.i_y))
    } else {
        0.0
    };
    let d_sh = if position == cat::SpanPosition::Simple {
        let es = cat::transformed(&c.sec, c.ys, c.beff, c.hc, ns);
        cat::deflection_shrinkage(span, cat::constants::SHRINKAGE_STRAIN, c.beff, c.hc, ns, (es.y_na - c.hc / 2.0).max(0.0), es.i)
    } else {
        0.0
    };
    let total = d_c + d_perm + d_var + d_sh;
    let reference = if cantilever { 2.0 * span } else { span };
    let lim = part_1_1::deflection_limit_m(reference, a);
    let div = part_1_1::deflection_span_divisor(a);
    let cracked_note = if b.has_hogging() && b.cracked_length_fraction > 1e-9 { format!(", EI₂ over {:.0} % of the span at the internal supports", b.cracked_length_fraction * 100.0) } else { String::new() };
    let cracked_note_de = if b.has_hogging() && b.cracked_length_fraction > 1e-9 { format!(", EI₂ auf {:.0} % der Stützweite an den Innenstützen", b.cracked_length_fraction * 100.0) } else { String::new() };
    let mut d = head(a, format!("en1994.7.3.1.deflection.{}", b.id), "1994-1-1", "7.3.1", bref(b, "actions"), "SLS deflection with creep and shrinkage", "Durchbiegung GZG mit Kriechen und Schwinden")
        .utilization(length(total.abs()), length(lim))
        .explanation(lc(format!("δ = δ_construction {:.1} + δ_G(n_L) {:.1} + δ_Q(n_0) {:.1} + δ_shrinkage {:.1} = {:.1} mm, |δ| ≤ {}/{div:.0} = {:.1} mm (three-moment statics of the {}{cracked_note}).", d_c * 1e3, d_perm * 1e3, d_var * 1e3, d_sh * 1e3, total * 1e3, if cantilever { "2L" } else { "L" }, lim * 1e3, b.support.as_str().replace('_', " ")), format!("δ = δ_Bauzustand {:.1} + δ_G(n_L) {:.1} + δ_Q(n_0) {:.1} + δ_Schwinden {:.1} = {:.1} mm, |δ| ≤ {}/{div:.0} = {:.1} mm (Dreimomentengleichung für {}{cracked_note_de}).", d_c * 1e3, d_perm * 1e3, d_var * 1e3, d_sh * 1e3, total * 1e3, if cantilever { "2L" } else { "L" }, lim * 1e3, b.support.as_str().replace('_', " "))));
    if total.abs() > lim {
        let ratio = (lim / total.abs().max(1e-12)).clamp(0.0, 1.0);
        d = d.remedy(Remedy::at_most(bref(b, "spanM"), length(b.span_m), length(b.span_m * ratio.powf(0.25) * 0.95), lc(format!("Reduce the span so δ ≤ L/{div:.0}."), format!("Spannweite reduzieren, damit δ ≤ L/{div:.0}."))));
        if let Some(rem) = load_fix(b, ratio * 0.5, "Reduce the imposed load.", "Nutzlast reduzieren.") {
            d = d.remedy(rem);
        }
    }
    r.push(d.build());

    let lim_q = reference / 300.0;
    let mut dv = head(a, format!("en1994.7.3.1.deflection-variable.{}", b.id), "1994-1-1", "7.3.1", bref(b, "actions"), "SLS deflection from variable actions", "Durchbiegung GZG aus veränderlichen Einwirkungen")
        .utilization(length(d_var.abs()), length(lim_q))
        .explanation(lc(format!("δ_Q = {:.1} mm ≤ L/300 = {:.1} mm.", d_var * 1e3, lim_q * 1e3), format!("δ_Q = {:.1} mm ≤ L/300 = {:.1} mm.", d_var * 1e3, lim_q * 1e3)));
    if d_var.abs() > lim_q {
        dv = dv.remedy(Remedy::at_most(bref(b, "spanM"), length(b.span_m), length(b.span_m * (lim_q / d_var.abs().max(1e-12)).powf(0.25) * 0.95), lc("Reduce the span.", "Spannweite reduzieren.")));
    }
    r.push(dv.build());
}

struct Crack {
    util: f64,
    as_min_per_m: f64,
    sigma_s: f64,
    s_allow: Option<f64>,
    w_k: f64,
}

fn crack_eval(c: &BeamCtx, as_per_m: f64, bar_spacing: f64) -> Crack {
    let b = c.beam;
    let n0 = cat::modular_ratio(b.concrete_e_cm_pa, 0.0, 0.0);
    let fct = cat::fctm_pa(b.concrete_f_ck_pa);
    let el0 = cat::transformed(&c.sec, c.ys, c.beff_h, c.hc, n0);
    let z0 = (el0.y_na - c.hc / 2.0).max(1e-3);
    let act = c.beff_h * c.hc;
    let as_min_per_m = cat::as_min_m2(fct, act, c.hc, z0, cat::constants::REBAR_FSK) / c.beff_h.max(1e-9);
    let m_hog = c.sls_qp.m_hog_nm.abs();
    let (sigma_s, s_allow, w_k) = if m_hog > 1.0 {
        let state = cat::crack_width::hogging(&c.sec, c.ys, c.hc, c.beff_h, as_per_m, b.concrete_f_ck_pa);
        let s0 = state.sigma_per_nm * m_hog;
        let s = s0 + state.delta_sigma_pa;
        (s, cat::max_bar_spacing_m(s, b.wk_limit_m.max(0.0002)), cat::crack_width::crack_width_m(s0, as_per_m, bar_spacing, c.hc, b.concrete_f_ck_pa, b.concrete_e_cm_pa))
    } else {
        (0.0, cat::max_bar_spacing_m(0.0, b.wk_limit_m.max(0.0002)), 0.0)
    };
    let u_min = as_min_per_m / as_per_m.max(1e-12);
    let u_sig = sigma_s / (0.8 * cat::constants::REBAR_FSK);
    let u_s = s_allow.map(|s| bar_spacing / s.max(1e-6)).unwrap_or(10.0);
    Crack { util: u_min.max(u_sig).max(u_s), as_min_per_m, sigma_s, s_allow, w_k }
}

fn crack(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let ev = crack_eval(c, b.as_hogging_m2_per_m, b.bar_spacing_m);
    let mut ck = head(a, format!("en1994.7.4.crack.{}", b.id), "1994-1-1", "7.4", bref(b, "asHoggingM2PerM"), "SLS crack control and minimum reinforcement", "GZG Rissbreitenbegrenzung und Mindestbewehrung")
        .utilization(dim(ev.util), dim(1.0))
        .explanation(lc(
            format!("{}: A_s,min = {:.0} mm²/m, provided {:.0}; σ_s = {:.0} MPa, w_k = {:.3} mm (EN 1992-1-1 7.3.4), s_max = {} mm, bar s = {:.0} mm.", c.sls_qp.label_en, ev.as_min_per_m * 1e6, b.as_hogging_m2_per_m * 1e6, ev.sigma_s / 1e6, ev.w_k * 1e3, ev.s_allow.map(|s| format!("{:.0}", s * 1e3)).unwrap_or_else(|| "n/a".into()), b.bar_spacing_m * 1e3),
            format!("{}: A_s,min = {:.0} mm²/m, vorhanden {:.0}; σ_s = {:.0} MPa, w_k = {:.3} mm (EN 1992-1-1 7.3.4), s_max = {} mm, Stababstand = {:.0} mm.", c.sls_qp.label_de, ev.as_min_per_m * 1e6, b.as_hogging_m2_per_m * 1e6, ev.sigma_s / 1e6, ev.w_k * 1e3, ev.s_allow.map(|s| format!("{:.0}", s * 1e3)).unwrap_or_else(|| "n/a".into()), b.bar_spacing_m * 1e3),
        ));
    if ev.util > 1.0 {
        let feasible = |as_m: f64, s: f64| crack_eval(c, as_m, s).util <= 0.99;
        let mut as_fix = None;
        if feasible(0.05, b.bar_spacing_m) {
            let (mut lo, mut hi) = (b.as_hogging_m2_per_m, 0.05);
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                if feasible(mid, b.bar_spacing_m) { hi = mid } else { lo = mid }
            }
            as_fix = Some(hi);
        }
        let mut s_fix = None;
        let s_cand = (b.bar_spacing_m * 0.5).max(0.05);
        if feasible(b.as_hogging_m2_per_m, s_cand) {
            let (mut lo, mut hi) = (s_cand, b.bar_spacing_m);
            for _ in 0..60 {
                let mid = 0.5 * (lo + hi);
                if feasible(b.as_hogging_m2_per_m, mid) { lo = mid } else { hi = mid }
            }
            s_fix = Some(lo);
        }
        if let Some(as_req) = as_fix {
            ck = ck.remedy(Remedy::at_least(bref(b, "asHoggingM2PerM"), area(b.as_hogging_m2_per_m), area(as_req), lc(format!("Provide ≥ {:.0} mm²/m hogging reinforcement.", as_req * 1e6), format!("≥ {:.0} mm²/m Stützbewehrung vorsehen.", as_req * 1e6))));
        }
        if let Some(s_req) = s_fix {
            ck = ck.remedy(Remedy::at_most(bref(b, "barSpacingM"), length(b.bar_spacing_m), length(s_req), lc(format!("Reduce the bar spacing to ≤ {:.0} mm.", s_req * 1e3), format!("Stababstand auf ≤ {:.0} mm verringern.", s_req * 1e3))));
        }
        if as_fix.is_none() && s_fix.is_none() {
            ck = ck.remedy(Remedy::at_least(bref(b, "asHoggingM2PerM"), area(b.as_hogging_m2_per_m), area(0.05), lc("Provide the maximum practical hogging reinforcement and reduce the bar spacing.", "Maximale praktische Stützbewehrung vorsehen und Stababstand verringern.")));
        }
    }
    r.push(ck.build());
}

/// 🧵 C-43 tension stiffening of the hogging zone (EN 1994-1-1, 7.4.3; EN 1994-2, 6.8.5.4): the bar stress σ_s = σ_s,0 + Δσ_s with σ_s,0 from the characteristic hogging moment of the span envelope on the cracked section and Δσ_s = f_ctm·k/(α_st·ρ_s), limited to 0.8·f_sk (7.2.2); unverified k and f_sk (`ndp.en1994.2.rebar-fatigue-range`).
fn tension_stiffening(r: &mut CheckReport, c: &BeamCtx) {
    let (b, a) = (c.beam, c.annex);
    let m_hog = c.sls_char.m_hog_nm.abs();
    if !b.has_hogging() || m_hog <= 1.0 || b.as_hogging_m2_per_m <= 1e-12 {
        return;
    }
    let st = cat::crack_width::hogging(&c.sec, c.ys, c.hc, c.beff_h, b.as_hogging_m2_per_m, b.concrete_f_ck_pa);
    let s0 = st.sigma_per_nm * m_hog;
    let sigma = s0 + st.delta_sigma_pa;
    let lim = 0.8 * cat::constants::REBAR_FSK;
    let rho_s = b.as_hogging_m2_per_m * c.beff_h / (c.beff_h * c.hc).max(1e-9);
    let mut ts = head(a, format!("en1994.7.4.3.tension-stiffening.{}", b.id), "1994-1-1", "7.4.3", bref(b, "asHoggingM2PerM"), "Reinforcement stress with tension stiffening in the hogging zone", "Bewehrungsspannung mit Zugversteifung im Stützbereich")
        .utilization(dim(sigma / lim), dim(1.0))
        .explanation(lc(
            format!("{}: M_hog = {:.1} kNm (span envelope), σ_s,0 = {:.1} MPa on the cracked section, Δσ_s = {:.1} MPa (ρ_s = {:.2} %), σ_s = {:.1} MPa ≤ 0.8·f_sk = {:.0} MPa; unverified: ndp.en1994.2.rebar-fatigue-range.", c.sls_char.label_en, m_hog / 1e3, s0 / 1e6, st.delta_sigma_pa / 1e6, rho_s * 100.0, sigma / 1e6, lim / 1e6),
            format!("{}: M_Stütz = {:.1} kNm (Feldumhüllende), σ_s,0 = {:.1} MPa am gerissenen Querschnitt, Δσ_s = {:.1} MPa (ρ_s = {:.2} %), σ_s = {:.1} MPa ≤ 0,8·f_sk = {:.0} MPa; ungeprüft: ndp.en1994.2.rebar-fatigue-range.", c.sls_char.label_de, m_hog / 1e3, s0 / 1e6, st.delta_sigma_pa / 1e6, rho_s * 100.0, sigma / 1e6, lim / 1e6),
        ));
    if sigma > lim {
        let req = b.as_hogging_m2_per_m * (sigma / lim) * 1.05;
        ts = ts.remedy(Remedy::at_least(bref(b, "asHoggingM2PerM"), area(b.as_hogging_m2_per_m), area(req), lc(format!("Provide ≥ {:.0} mm²/m hogging reinforcement.", req * 1e6), format!("≥ {:.0} mm²/m Stützbewehrung vorsehen.", req * 1e6))));
    }
    r.push(ts.ndps(&["ndp.en1994.2.rebar-fatigue-range"]).build());
}
