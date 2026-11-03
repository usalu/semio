//! 🧱️ EN 1999 member checks: classification, cross-section resistance, member stability, plated webs and serviceability of one aluminium member.

use super::*;
use super::part_1_1::{EffCtx, SectionClass};
use super::part_en1990::GoverningEffects;
use semio_s_plugin_norm_compliance::Station;

const STATIONS: usize = 10;

/// 📈️ `M(s)/max|M|` of the member under its governing action: `udl` or `external` (mid-span point load, tip load of a cantilever) on the declared support (simple span, fixed-fixed `continuous`, `cantilever` fixed at `s = 0`).
fn moment_shape(member: &AluminiumMember, udl: bool, s: f64) -> f64 {
    let m = s.min(1.0 - s);
    match (member.support, udl) {
        (SupportCondition::SimplySupported, true) => 4.0 * s * (1.0 - s),
        (SupportCondition::SimplySupported, false) => 2.0 * m,
        (SupportCondition::Cantilever, true) => (1.0 - s).powi(2),
        (SupportCondition::Cantilever, false) => 1.0 - s,
        (SupportCondition::Continuous, true) => (1.0 - 6.0 * s * (1.0 - s)).abs(),
        (SupportCondition::Continuous, false) => (1.0 - 4.0 * m).abs(),
    }
}

/// 📈️ `V(s)/max|V|`: `|1-2s|` of a uniform load on a span, `1 - s` of a loaded cantilever, constant for a point load.
fn shear_shape(member: &AluminiumMember, udl: bool, s: f64) -> f64 {
    match (member.support, udl) {
        (SupportCondition::Cantilever, true) => 1.0 - s,
        (_, true) => (1.0 - 2.0 * s).abs(),
        _ => 1.0,
    }
}

/// 📍️ Replaces the stations of `check` by `along(s)` at `s = i / 10`, rescaled so the largest station is exactly the governing utilisation of the check.
fn attach(check: &mut CheckResult, along: impl Fn(f64) -> (f64, Vec<Quantity>)) {
    if !check.utilization.is_finite() || check.status == semio_s_plugin_norm_compliance::CheckStatus::NotApplicable {
        return;
    }
    let raw: Vec<(f64, (f64, Vec<Quantity>))> = (0..=STATIONS).map(|i| i as f64 / STATIONS as f64).map(|s| (s, along(s))).collect();
    let peak = raw.iter().map(|r| r.1 .0).fold(0.0, f64::max);
    if peak > 0.0 {
        check.stations = raw.into_iter().map(|(s, (u, v))| Station::new(&[s], u / peak * check.utilization, v)).collect();
    }
}

//#region 🔖️ComplianceHelpers
pub fn actions_digest(actions: &[semio_s_artifact_norm_en1999_model::MemberAction]) -> String {
    actions.iter().map(|a| format!(
        "{id}/{kind:?}/{cat:?}/{src:?}/g={g:.4}/q={q:.4}/N={n:.4}/Vy={vy:.4}/Vz={vz:.4}/My={my:.4}/Mz={mz:.4}",
        id=a.id, kind=a.kind, cat=a.category, src=a.source,
        g=a.g_k_line, q=a.q_k_line, n=a.n_k, vy=a.v_y_k, vz=a.v_z_k, my=a.m_y_k, mz=a.m_z_k
    )).collect::<Vec<_>>().join(" | ")
}

fn member_support_digest(member: &AluminiumMember) -> String {
    format!(
        "support={sup};c1={c1:.4};LcrY={ly:.4};LcrZ={lz:.4};LcrT={lt:.4};Llt={llt:.4};restrained={r}",
        sup=member.support.code(), c1=member.c1, ly=member.buckling_length_y, lz=member.buckling_length_z,
        lt=member.buckling_length_t, llt=member.ltb_length, r=member.restrained_ltb
    )
}

pub fn member_label(id: &str) -> LocalizedCopy {
    loc(&format!("Member {id}"), &format!("Bauteil {id}"))
}

pub fn conn_label(id: &str) -> LocalizedCopy {
    loc(&format!("Connection {id}"), &format!("Anschluss {id}"))
}

/// 🔖️ Registry ids a funnelled utilisation check of the part, id and clause section rests on: the serviceability ratio of 7.2, the fire, fatigue, thin-wall and shell factors of their parts, γ_M2 (and γ_Mw for welds) of the connections of 8, else γ_M1 and γ_M2 of the resistances.
fn funnel_ndps(id: &str, part: &str, section: &str) -> Vec<&'static str> {
    use na_de::AnnexParams as P;
    if part.ends_with("1999-1-2") {
        vec![P::GAMMA_M_FI]
    } else if part.ends_with("1999-1-3") {
        vec![P::GAMMA_MF, P::GAMMA_FF]
    } else if part.ends_with("1999-1-4") || part.ends_with("1999-1-5") {
        vec![P::GAMMA_M1]
    } else if section.starts_with('7') {
        vec![if id.contains("defl") { P::DEFLECTION_RATIO } else { P::STRESS_RATIO }]
    } else if section.starts_with('8') && id.contains("weld") {
        vec![P::GAMMA_M1, P::GAMMA_M2, P::GAMMA_MW]
    } else if section.starts_with('8') {
        vec![P::GAMMA_M2]
    } else {
        vec![P::GAMMA_M1, P::GAMMA_M2]
    }
}

pub fn utilization_check(
    id: String,
    part: &str,
    clause: ClauseId,
    subject: SubjectRef,
    title: LocalizedCopy,
    computed: Quantity,
    limit: Quantity,
    annex: AnnexChoice,
    explanation: LocalizedCopy,
    mut remedies: Vec<Remedy>,
) -> CheckResult {
    let u = if limit.value.abs() > 0.0 { computed.value.abs() / limit.value.abs() } else if computed.value.abs() > 0.0 { f64::INFINITY } else { 0.0 };
    if u > 1.0 && remedies.is_empty() && !subject.path.is_empty() {
        let required = if subject.path.contains("bucklingLength") || subject.path.contains("ltbLength") {
            Quantity::new(computed.kind, (computed.value.abs() / u.max(1.0) * 0.99).max(1e-6))
        } else {
            Quantity::new(computed.kind, computed.value * (1.0 / u.max(1.0)) * 0.99)
        };
        remedies.push(Remedy::at_most(
            subject.clone(),
            computed,
            required,
            loc("Reduce the governing demand until utilization ≤ 1.", "Maßgebende Beanspruchung senken, bis Ausnutzung ≤ 1."),
        ));
    }
    let ndps = funnel_ndps(&id, part, &clause.section);
    let mut b = CheckResult::assess(id, part, clause, subject, title)
        .utilization(computed, limit)
        .annex(annex)
        .ndps(&ndps)
        .ndp_reads()
        .explanation(explanation);
    for r in remedies { b = b.remedy(r); }
    b.build()
}

pub fn clause(section: &str) -> ClauseId { ClauseId::new("EN 1999-1-1", format!("§{section}"), section) }

/// 🩹 One at-most remedy per non-zero action leaf, each scaled by 0.99/u so that applying all of them clears a failing check of utilization `u`.
pub fn leaf_remedies(owner_kind: &str, owner_id: &str, actions: &[semio_s_artifact_norm_en1999_model::MemberAction], u: f64, label: &LocalizedCopy, what_en: &str, what_de: &str, lead: &str) -> Vec<Remedy> {
    let mut out = Vec::new();
    if u <= 1.0 { return out; }
    for a in actions.iter().filter(|a| a.kind != semio_s_artifact_norm_en1999_model::ActionKind::Fire) {
        let leaves = [
            ("nK", a.n_k, true), ("vYK", a.v_y_k, true), ("vZK", a.v_z_k, true), ("mYK", a.m_y_k, false),
            ("mZK", a.m_z_k, false), ("gKLine", a.g_k_line, true), ("qKLine", a.q_k_line, true),
            ("torsionK", a.torsion_k, false), ("localLoadK", a.local_load_k, true),
        ];
        for (leaf, cur, is_force) in leaves {
            if cur.abs() < 1e-18 { continue; }
            let to_q = |v: f64| if is_force { q_force(v) } else { q_moment(v) };
            out.push(Remedy::at_most(
                SubjectRef::new(owner_id, format!("{owner_kind}[id={owner_id}].actions[id={}].{leaf}", a.id), label.clone()),
                to_q(cur),
                to_q(cur * 0.99 / u),
                loc(&format!("{what_en} ({leaf} of {}).", a.id), &format!("{what_de} ({leaf} von {}).", a.id)),
            ));
        }
    }
    out.sort_by_key(|r| !r.target.path.ends_with(&format!(".{lead}")));
    out
}

/// 🧯 A resistance that never drops below one millionth of the demand, so a zero resistance reports the capped utilization 10⁶ instead of a vanishing one.
pub fn floor_limit(demand: f64, resistance: f64) -> f64 {
    if resistance > demand.abs() * 1.0e-6 { resistance } else { demand.abs() * 1.0e-6 }
}

fn ratio(x: f64, r: f64) -> f64 {
    if x.abs() < 1e-12 { 0.0 } else if r <= 0.0 { 1.0e6 } else { (x.abs() / r).min(1.0e6) }
}

fn alloy_fail(id: String, part: &str, ctx_label: &str, material: &semio_s_artifact_norm_en1999_model::AluminiumMaterial, annex: AnnexChoice) -> CheckResult {
    let subject = SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung"));
    CheckResult::assess(id, part, ClauseId::new("EN 1999-1-1", "§3.2", "3.2"), subject.clone(), loc("Alloy designation", "Legierungsbezeichnung"))
        .status(semio_s_plugin_norm_compliance::CheckStatus::Fail)
        .explanation(loc(
            &format!("Unknown alloy designation '{}' ({ctx_label}).", material.designation),
            &format!("Unbekannte Legierungsbezeichnung '{}' ({ctx_label}).", material.designation),
        ))
        .annex(annex)
        .remedy(Remedy::one_of(subject, part_1_1::CATALOGUE_ALLOYS.iter().map(|s| (*s).to_string()).collect(), loc("Pick a catalogue alloy from EN 1999-1-1 Table 3.2.", "Kataloglegierung nach EN 1999-1-1 Tabelle 3.2 wählen.")))
        .build()
}

pub fn alloy_unknown(id: String, ctx_label: &str, material: &semio_s_artifact_norm_en1999_model::AluminiumMaterial, annex: AnnexChoice) -> CheckResult {
    alloy_fail(id, "DIN EN 1999-1-1", ctx_label, material, annex)
}

struct Run<'a> {
    member: &'a AluminiumMember,
    annex: AnnexChoice,
    gov: &'a GoverningEffects,
    act_dig: String,
    sup_dig: String,
}

impl Run<'_> {
    #[allow(clippy::too_many_arguments)]
    fn emit(&self, out: &mut Vec<CheckResult>, id: &str, sec: &str, title: (&str, &str), computed: Quantity, limit: Quantity, expl: (String, String), path: String, remedies: Vec<Remedy>) {
        let tag_en = format!(" (ULS {}, lead {}); {}; {}", self.gov.combination, self.gov.action_id, self.act_dig, self.sup_dig);
        let tag_de = format!(" (GZT {}, führend {}); {}; {}", self.gov.combination, self.gov.action_id, self.act_dig, self.sup_dig);
        let udl = self.member.actions.iter().find(|a| a.id == self.gov.action_id).is_some_and(|a| a.source == semio_s_artifact_norm_en1999_model::ActionSource::Udl);
        out.push(utilization_check(
            format!("{id}.{}", self.member.id),
            "DIN EN 1999-1-1",
            clause(sec),
            SubjectRef::new(&self.member.id, path, member_label(&self.member.id)),
            loc(title.0, title.1),
            computed,
            limit,
            self.annex,
            loc(&format!("{}{tag_en}", expl.0), &format!("{}{tag_de}", expl.1)),
            remedies,
        ));
        let member = self.member;
        let (c, l) = (computed, limit);
        let shape: Option<Box<dyn Fn(f64) -> f64 + '_>> = match sec {
            "6.2.3" | "6.3.1.1" | "6.3.1.4" => Some(Box::new(|_| 1.0)),
            "6.2.5" | "6.2.8" | "6.3.2" => Some(Box::new(move |s| moment_shape(member, udl, s))),
            "6.2.6" | "6.7.4" => Some(Box::new(move |s| shear_shape(member, udl, s))),
            _ => None,
        };
        if let (Some(shape), Some(last)) = (shape, out.last_mut()) {
            attach(last, |s| (shape(s) * c.value.abs() / l.value.abs().max(1e-300), vec![Quantity::new(c.kind, c.value * shape(s)), l]));
        }
    }

    fn loads(&self, u: f64, what_en: &str, what_de: &str, lead: &str) -> Vec<Remedy> {
        leaf_remedies("members", &self.member.id, &self.member.actions, u, &member_label(&self.member.id), what_en, what_de, lead)
    }
}

fn solve_length(l_start: f64, need: f64, resistance: impl Fn(f64) -> f64) -> f64 {
    let (mut lo, mut hi) = (0.05, l_start.max(0.1));
    for _ in 0..48 {
        let mid = 0.5 * (lo + hi);
        if resistance(mid) >= need { lo = mid; } else { hi = mid; }
    }
    lo
}

fn member_uls(
    member: &AluminiumMember,
    section: &AluminiumSection,
    material: &semio_s_artifact_norm_en1999_model::AluminiumMaterial,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    gov: &GoverningEffects,
    haz_extents: &[(String, f64)],
    web_rows: &[(String, f64)],
) -> Vec<CheckResult> {
    let run = Run { member, annex, gov, act_dig: actions_digest(&member.actions), sup_dig: member_support_digest(member) };
    let mut out = Vec::new();
    let (fo, fu, g1, g2) = (alloy.f_o_pa, alloy.f_u_pa, params.gamma_m1, params.gamma_m2);
    let class_a = alloy.buckling_class_a;
    let (n, vy, vz, my, mz) = (gov.n_ed, gov.v_y_ed, gov.v_z_ed, gov.m_y_ed, gov.m_z_ed);
    let nc = (-n).max(0.0);
    let (_, i_y, i_z, _, _, i_t, i_w) = part_1_1::section_geometry(section);
    let gross = part_1_1::props(section, &EffCtx::gross(fo, class_a));
    let compression = -n / gross.a.max(1e-18);
    let (s1, s2) = (compression + my.abs() / gross.wy.max(1e-18), compression - my.abs() / gross.wy.max(1e-18));
    let (s_max, s_min) = (s1.max(s2), s1.min(s2));
    let eta = if s_max > 0.0 { part_1_1::stress_gradient_eta(s_min / s_max) } else { 1.0 };
    let class = part_1_1::section_class(section, fo, class_a, eta);
    let base = EffCtx { f_o_pa: fo, class_a, eta_web: eta, local: true, haz_rho: None, haz_extents };
    let loc_p = part_1_1::props(section, &base);
    let eff_o = part_1_1::props(section, &EffCtx { haz_rho: Some(alloy.rho_o_haz), ..base });
    let haz_o = part_1_1::props(section, &EffCtx { local: false, haz_rho: Some(alloy.rho_o_haz), ..base });
    let haz_u = part_1_1::props(section, &EffCtx { local: false, haz_rho: Some(alloy.rho_u_haz), ..base });
    let has_haz = section.elements.iter().any(|e| e.welded);
    let path_n = format!("members[id={}].actions[id={}].nK", member.id, gov.action_id);
    let path_m = format!("members[id={}].actions[id={}].mYK", member.id, gov.action_id);
    let path_mz = format!("members[id={}].actions[id={}].mZK", member.id, gov.action_id);
    let path_vy = format!("members[id={}].actions[id={}].vYK", member.id, gov.action_id);
    let path_vz = format!("members[id={}].actions[id={}].vZK", member.id, gov.action_id);

    let (gov_beta, gov_limits) = if part_1_1::is_tube(section) {
        (part_1_1::tube_beta(section) / part_1_1::epsilon(fo), part_1_1::class_limits(class_a, false, false))
    } else {
        part_1_1::governing_element(section, fo, class_a, eta).map(|(_, b, l)| (b, l)).unwrap_or((0.0, [11.0, 16.0, 22.0]))
    };

    let eps = part_1_1::epsilon(fo);
    for el in &section.elements {
        let b_eps = part_1_1::element_beta_over_eps(el, fo, eta);
        let outstand = part_1_1::behaves_outstand(el);
        let lim = part_1_1::class_limits(class_a, outstand, el.welded);
        let el_class = part_1_1::class_of(b_eps, lim);
        let rho = part_1_1::rho_c(b_eps, class_a, outstand, el.welded);
        let path = format!("sections[id={}].elements[id={}].thickness", section.id, el.id);
        let mut remedies = Vec::new();
        if b_eps > lim[2] {
            let t_req = (if outstand || !el.id.to_ascii_lowercase().contains("web") { 1.0 } else { eta }) * el.width / (lim[2] * eps);
            remedies.push(Remedy::at_least(
                SubjectRef::new(&section.id, path.clone(), loc(&format!("Plate {}", el.id), &format!("Blech {}", el.id))),
                q_length(el.thickness),
                q_length(t_req),
                loc(
                    &format!("Increase the thickness of {} to at least {:.1} mm for class 3 (no local buckling reduction).", el.id, t_req * 1e3),
                    &format!("Dicke von {} auf mindestens {:.1} mm erhöhen für Klasse 3 (keine Beulabminderung).", el.id, t_req * 1e3),
                ),
            ));
        }
        let tag_en = format!(" (ULS {}, lead {})", gov.combination, gov.action_id);
        let tag_de = format!(" (GZT {}, führend {})", gov.combination, gov.action_id);
        let mut c = CheckResult::assess(
            format!("en1999.6.1.4.class.{}.{}", member.id, el.id), "DIN EN 1999-1-1", clause("6.1.4"),
            SubjectRef::new(&member.id, path, member_label(&member.id)),
            loc("Cross-section classification (Table 6.2)", "Querschnittsklasse (Tabelle 6.2)"),
        )
        .utilization(q_dim(b_eps), q_dim(lim[2]))
        .annex(annex)
        .explanation(loc(
            &format!("Plate {} β/ε = {b_eps:.2} (η = {eta:.2}), limits {:.1}/{:.1}/{:.1} → class {:?}, ρ_c = {rho:.4}; the class-4 effective thickness t_eff = ρ_c·t enters A_eff and W_eff{tag_en}", el.id, lim[0], lim[1], lim[2], el_class),
            &format!("Blech {} β/ε = {b_eps:.2} (η = {eta:.2}), Grenzen {:.1}/{:.1}/{:.1} → Klasse {:?}, ρ_c = {rho:.4}; die wirksame Dicke t_eff = ρ_c·t geht in A_eff und W_eff ein{tag_de}", el.id, lim[0], lim[1], lim[2], el_class),
        ));
        if b_eps > lim[2] { c = c.status(semio_s_plugin_norm_compliance::CheckStatus::Warning); }
        for r in remedies { c = c.remedy(r); }
        out.push(c.ndps(&[na_de::AnnexParams::GAMMA_M1, na_de::AnnexParams::GAMMA_M2]).ndp_reads().build());
    }

    let n_t_yield = haz_o.a * fo / g1;
    let n_t_ult = (if has_haz { haz_u.a } else { 0.9 * gross.a }) * fu / g2;
    let n_t_rd = n_t_yield.min(n_t_ult);
    let n_c_rd = eff_o.a * fo / g1;
    let (n_abs, n_rd, n_title) = if n >= 0.0 {
        (n, n_t_rd, ("Tension resistance N_t,Rd (yield, net/HAZ ultimate)", "Zugwiderstand N_t,Rd (Fließen, Netto/WEZ-Bruch)"))
    } else {
        (-n, n_c_rd, ("Compression resistance N_c,Rd (A_eff, HAZ)", "Druckwiderstand N_c,Rd (A_eff, WEZ)"))
    };
    let mut rem = run.loads(n_abs / n_rd.max(1e-18), "Reduce the axial force", "Normalkraft senken", "nK");
    if n_abs > n_rd {
        rem.push(Remedy::one_of(
            SubjectRef::new(&material.id, format!("materials[id={}].designation", material.id), loc("Alloy", "Legierung")),
            vec!["aw6082-t6".into(), "aw6061-t6".into()],
            loc("Upgrade alloy/temper to raise f_o.", "Legierung/Zustand erhöhen, um f_o zu steigern."),
        ));
    }
    run.emit(&mut out, "en1999.6.2.3.n", "6.2.3", n_title, q_force(n_abs), q_force(n_rd),
        (format!("N_Ed = {:.1} kN, N_o,Rd = {:.1} kN (A_haz,o = {:.1} mm²), N_u,Rd = {:.1} kN, N_c,Rd = {:.1} kN (A_eff = {:.1} mm²)", n / 1e3, n_t_yield / 1e3, haz_o.a * 1e6, n_t_ult / 1e3, n_c_rd / 1e3, eff_o.a * 1e6),
         format!("N_Ed = {:.1} kN, N_o,Rd = {:.1} kN (A_WEZ,o = {:.1} mm²), N_u,Rd = {:.1} kN, N_c,Rd = {:.1} kN (A_eff = {:.1} mm²)", n / 1e3, n_t_yield / 1e3, haz_o.a * 1e6, n_t_ult / 1e3, n_c_rd / 1e3, eff_o.a * 1e6)),
        path_n.clone(), rem);

    let (wpl_y, wpl_z) = part_1_1::plastic_moduli(section);
    let alpha_y = if has_haz { 1.0 } else { part_1_1::shape_factor(class, wpl_y, gross.wy, gov_beta, gov_limits) };
    let alpha_z = if has_haz { 1.0 } else { part_1_1::shape_factor(class, wpl_z, gross.wz, gov_beta, gov_limits) };
    let w_y = if has_haz { eff_o.wy } else if class == SectionClass::Class4 { loc_p.wy } else { gross.wy };
    let w_z = if has_haz { eff_o.wz } else if class == SectionClass::Class4 { loc_p.wz } else { gross.wz };
    let m_rd_y = alpha_y * w_y * fo / g1;
    let m_rd_z = alpha_z * w_z * fo / g1;
    let rem = run.loads(my.abs() / m_rd_y.max(1e-18), "Reduce the strong-axis moment", "Moment um die starke Achse senken", "mYK");
    run.emit(&mut out, "en1999.6.2.5.m", "6.2.5", ("Bending resistance M_y,Rd", "Biegewiderstand M_y,Rd"), q_moment(my.abs()), q_moment(m_rd_y),
        (format!("M_y,Ed = {:.2} kNm, α = {alpha_y:.3}, W = {:.0} mm³, class {:?}, M_y,Rd = {:.2} kNm", my / 1e3, w_y * 1e9, class, m_rd_y / 1e3),
         format!("M_y,Ed = {:.2} kNm, α = {alpha_y:.3}, W = {:.0} mm³, Klasse {:?}, M_y,Rd = {:.2} kNm", my / 1e3, w_y * 1e9, class, m_rd_y / 1e3)),
        path_m.clone(), rem);
    let rem = run.loads(mz.abs() / m_rd_z.max(1e-18), "Reduce the weak-axis moment", "Moment um die schwache Achse senken", "mZK");
    run.emit(&mut out, "en1999.6.2.5.mz", "6.2.5", ("Bending resistance M_z,Rd", "Biegewiderstand M_z,Rd"), q_moment(mz.abs()), q_moment(m_rd_z),
        (format!("M_z,Ed = {:.2} kNm, α = {alpha_z:.3}, W = {:.0} mm³, M_z,Rd = {:.2} kNm", mz / 1e3, w_z * 1e9, m_rd_z / 1e3),
         format!("M_z,Ed = {:.2} kNm, α = {alpha_z:.3}, W = {:.0} mm³, M_z,Rd = {:.2} kNm", mz / 1e3, w_z * 1e9, m_rd_z / 1e3)),
        path_mz.clone(), rem);

    let v_pl_z = haz_o.a_v * fo / (3.0f64.sqrt() * g1);
    let v_pl_y = haz_o.a_f * fo / (3.0f64.sqrt() * g1);
    let hw = (section.height - 2.0 * section.flange_thickness).max(0.0);
    let tw = section.web_thickness;
    let web_panels = web_stiffener::web_panels(member.length, &web_rows.iter().map(|(_, p)| *p).collect::<Vec<f64>>());
    let spacings = web_stiffener::spacings(&web_panels);
    let a_widest = web_stiffener::widest(&web_panels);
    let rho_v = if part_1_1::is_tube(section) || tw <= 0.0 || hw <= 0.0 { 1.0 } else { spacings.iter().map(|a| part_1_1::shear_buckling_rho(hw / tw, fo, hw, *a)).fold(f64::INFINITY, f64::min) };
    let (panels_en, panels_de) = if web_panels.is_empty() { ("web unstiffened over the member length".to_string(), "Steg über die Bauteillänge ungesteift".to_string()) } else { (format!("{} web panels between the transverse stiffeners and the supports, widest a = {:.0} mm", web_panels.len(), a_widest * 1e3), format!("{} Stegfelder zwischen den Quersteifen und den Auflagern, breitestes a = {:.0} mm", web_panels.len(), a_widest * 1e3)) };
    let v_b_rd = rho_v * v_pl_z;
    let rem = run.loads(vz.abs() / v_pl_z.max(1e-18), "Reduce the shear force V_z", "Querkraft V_z senken", "vZK");
    run.emit(&mut out, "en1999.6.2.6.v", "6.2.6", ("Shear resistance V_z,Rd", "Querkraftwiderstand V_z,Rd"), q_force(vz.abs()), q_force(v_pl_z),
        (format!("V_z,Ed = {:.1} kN, A_v = {:.1} mm², V_pl,Rd = {:.1} kN", vz / 1e3, haz_o.a_v * 1e6, v_pl_z / 1e3),
         format!("V_z,Ed = {:.1} kN, A_v = {:.1} mm², V_pl,Rd = {:.1} kN", vz / 1e3, haz_o.a_v * 1e6, v_pl_z / 1e3)),
        path_vz.clone(), rem);
    let rem = run.loads(vy.abs() / v_pl_y.max(1e-18), "Reduce the shear force V_y", "Querkraft V_y senken", "vYK");
    run.emit(&mut out, "en1999.6.2.6.vy", "6.2.6", ("Shear resistance V_y,Rd (flanges)", "Querkraftwiderstand V_y,Rd (Gurte)"), q_force(vy.abs()), q_force(v_pl_y),
        (format!("V_y,Ed = {:.1} kN, A_v,y = {:.1} mm², V_pl,y,Rd = {:.1} kN", vy / 1e3, haz_o.a_f * 1e6, v_pl_y / 1e3),
         format!("V_y,Ed = {:.1} kN, A_v,y = {:.1} mm², V_pl,y,Rd = {:.1} kN", vy / 1e3, haz_o.a_f * 1e6, v_pl_y / 1e3)),
        path_vy.clone(), rem);
    let hw_tw = if tw > 0.0 { hw / tw } else { 0.0 };
    let mut rem = run.loads(vz.abs() / v_b_rd.max(1e-18), "Reduce the shear force V_z", "Querkraft V_z senken", "vZK");
    if vz.abs() > v_b_rd && !part_1_1::is_tube(section) && tw > 0.0 && hw > 0.0 {
        let passes = |panels: &[f64]| web_stiffener::spacings(panels).iter().map(|a| part_1_1::shear_buckling_rho(hw / tw, fo, hw, *a)).fold(f64::INFINITY, f64::min) * v_pl_z >= vz.abs();
        if let Some(x) = web_stiffener::minimal_insert(member.length, web_rows, passes) {
            let id = web_stiffener::fresh_id(&member.id, web_rows);
            let row = format!(r#"{{"id":"{id}","girderId":"{}","position":"$required","width":{},"thickness":{},"sides":"two"}}"#, member.id, 0.1 * hw, tw);
            rem.push(Remedy::insert(SubjectRef::new(&id, "transverseStiffeners", loc(&format!("Transverse stiffeners of {}", member.id), &format!("Quersteifen von {}", member.id))), row, q_length(x), loc(&format!("Insert a transverse stiffener at x = {:.2} m so that every web panel resists the shear force.", x), &format!("Quersteife bei x = {:.2} m einfügen, sodass jedes Stegfeld die Querkraft aufnimmt.", x))));
        }
        if let Some((id, from, to)) = web_stiffener::nearest_move(member.length, web_rows, passes) {
            rem.push(Remedy::exactly(SubjectRef::new(&id, format!("transverseStiffeners[id={id}].position"), loc(&format!("Transverse stiffener {id}"), &format!("Quersteife {id}"))), q_length(from), q_length(to), loc("Move the transverse stiffener so that every web panel resists the shear force.", "Quersteife so verschieben, dass jedes Stegfeld die Querkraft aufnimmt.")));
        }
    }
    run.emit(&mut out, "en1999.6.7.4.vb", "6.7.4", ("Web shear buckling V_b,Rd", "Stegschubbeulen V_b,Rd"), q_force(vz.abs()), q_force(v_b_rd),
        (format!("h_w/t_w = {hw_tw:.1}, ρ_v = {rho_v:.3}, V_b,Rd = {:.1} kN ({panels_en})", v_b_rd / 1e3),
         format!("h_w/t_w = {hw_tw:.1}, ρ_v = {rho_v:.3}, V_b,Rd = {:.1} kN ({panels_de})", v_b_rd / 1e3)),
        path_vz.clone(), rem);

    let w_t = part_1_1::torsion_modulus(section);
    let t_rd = w_t * fo / (3.0f64.sqrt() * g1);
    let tau_t = if w_t > 0.0 { gov.t_ed.abs() / w_t } else { 0.0 };
    let path_t = format!("members[id={}].actions[id={}].torsionK", member.id, gov.action_id);
    let rem = run.loads(gov.t_ed.abs() / t_rd.max(1e-18), "Reduce the torque", "Torsionsmoment senken", "torsionK");
    run.emit(&mut out, "en1999.6.2.7.t", "6.2.7", ("Torsion resistance T_Rd", "Torsionswiderstand T_Rd"), q_moment(gov.t_ed.abs()), q_moment(floor_limit(gov.t_ed, t_rd)),
        (format!("T_Ed = {:.2} kNm, W_t = {:.0} mm³, τ_t,Ed = {:.1} MPa, T_Rd = {:.2} kNm (St. Venant torsion)", gov.t_ed / 1e3, w_t * 1e9, tau_t / 1e6, t_rd / 1e3),
         format!("T_Ed = {:.2} kNm, W_t = {:.0} mm³, τ_t,Ed = {:.1} MPa, T_Rd = {:.2} kNm (St.-Venantsche Torsion)", gov.t_ed / 1e3, w_t * 1e9, tau_t / 1e6, t_rd / 1e3)),
        path_t.clone(), rem);
    let v_rd_t = v_pl_z.min(v_b_rd) * (1.0 - tau_t * 3.0f64.sqrt() * g1 / fo).max(0.0);
    let rem = run.loads(vz.abs() / v_rd_t.max(1e-18), "Reduce the shear force or the torque", "Querkraft oder Torsionsmoment senken", "vZK");
    run.emit(&mut out, "en1999.6.2.7.vt", "6.2.7", ("Shear with torsion V_Rd,T", "Querkraft mit Torsion V_Rd,T"), q_force(vz.abs()), q_force(floor_limit(vz, v_rd_t)),
        (format!("V_Rd,T = V_Rd·(1 − τ_t,Ed·√3·γ_M1/f_o) = {:.1} kN", v_rd_t / 1e3),
         format!("V_Rd,T = V_Rd·(1 − τ_t,Ed·√3·γ_M1/f_o) = {:.1} kN", v_rd_t / 1e3)),
        path_vz.clone(), rem);
    if !part_1_1::is_tube(section) {
        let webs = if matches!(section.kind, semio_s_artifact_norm_en1999_model::SectionKind::Box | semio_s_artifact_norm_en1999_model::SectionKind::Rhs) { 2.0 } else { 1.0 };
        let (a_patch, f_one) = spacings.iter().map(|a| (*a, part_1_1::patch_load_resistance(hw, tw, section.width, section.flange_thickness, member.bearing_length, *a, fo, g1))).min_by(|x, y| x.1.total_cmp(&y.1)).unwrap_or((0.0, 0.0));
        let f_rd = webs * f_one;
        let path_f = format!("members[id={}].actions[id={}].localLoadK", member.id, gov.action_id);
        let rem = run.loads(gov.f_loc_ed.abs() / f_rd.max(1e-18), "Reduce the patch load", "Einzellast senken", "localLoadK");
        run.emit(&mut out, "en1999.6.7.5.patch", "6.7.5", ("Patch load resistance F_Rd", "Beulwiderstand unter Einzellast F_Rd"), q_force(gov.f_loc_ed.abs()), q_force(floor_limit(gov.f_loc_ed, f_rd)),
            (format!("F_Ed = {:.1} kN, s_s = {:.0} mm, a = {:.0} mm, F_Rd = {:.1} kN", gov.f_loc_ed / 1e3, member.bearing_length * 1e3, a_patch * 1e3, f_rd / 1e3),
             format!("F_Ed = {:.1} kN, s_s = {:.0} mm, a = {:.0} mm, F_Rd = {:.1} kN", gov.f_loc_ed / 1e3, member.bearing_length * 1e3, a_patch * 1e3, f_rd / 1e3)),
            path_f, rem);
    }
    let v_rd_eff = v_pl_z.min(v_b_rd);
    let rho_vm = if v_rd_eff > 0.0 && vz.abs() > 0.5 * v_rd_eff { (2.0 * vz.abs() / v_rd_eff - 1.0).powi(2) } else { 0.0 };
    let m_v_rd = (m_rd_y - rho_vm * haz_o.a_v * haz_o.a_v / (4.0 * tw.max(1e-9)) * fo / g1).max(0.0);
    let rem = run.loads(my.abs() / m_v_rd.max(1e-18), "Reduce the strong-axis moment", "Moment um die starke Achse senken", "mYK");
    run.emit(&mut out, "en1999.6.2.8.mv", "6.2.8", ("Bending with shear M_V,Rd", "Biegung mit Querkraft M_V,Rd"), q_moment(my.abs()), q_moment(floor_limit(my, m_v_rd)),
        (format!("ρ_V = {rho_vm:.3} (V_z,Ed/V_Rd = {:.3}), M_V,Rd = {:.2} kNm", vz.abs() / v_rd_eff.max(1e-18), m_v_rd / 1e3),
         format!("ρ_V = {rho_vm:.3} (V_z,Ed/V_Rd = {:.3}), M_V,Rd = {:.2} kNm", vz.abs() / v_rd_eff.max(1e-18), m_v_rd / 1e3)),
        path_m.clone(), rem);

    let nm_rd = if n >= 0.0 { n_t_rd } else { n_c_rd };
    let e_nz = if class == SectionClass::Class4 { (loc_p.zg - gross.zg).abs() } else { 0.0 };
    let e_ny = if class == SectionClass::Class4 { (loc_p.yg - gross.yg).abs() } else { 0.0 };
    let linear_class = class >= SectionClass::Class3 || has_haz;
    let (u_nm, nm_en, nm_de) = if linear_class {
        let u = ratio(n_abs, nm_rd) + ratio(my.abs() + n_abs * e_nz, m_rd_y) + ratio(mz.abs() + n_abs * e_ny, m_rd_z);
        (u,
         format!("N/N_Rd + (M_y + N·e_N,z)/M_y,Rd + (M_z + N·e_N,y)/M_z,Rd = {u:.3} (class {class:?}, e_N,z = {:.2} mm, e_N,y = {:.2} mm)", e_nz * 1e3, e_ny * 1e3),
         format!("N/N_Rd + (M_y + N·e_N,z)/M_y,Rd + (M_z + N·e_N,y)/M_z,Rd = {u:.3} (Klasse {class:?}, e_N,z = {:.2} mm, e_N,y = {:.2} mm)", e_nz * 1e3, e_ny * 1e3))
    } else {
        let (psi, eta_y, xi_z) = part_1_1::section_interaction_exponents(section.kind);
        let u = ratio(n_abs, nm_rd).powf(psi) + ratio(my, m_rd_y).powf(eta_y) + ratio(mz, m_rd_z).powf(xi_z);
        (u,
         format!("(N/N_Rd)^{psi} + (M_y/M_y,Rd)^{eta_y} + (M_z/M_z,Rd)^{xi_z} = {u:.3} (class {class:?})"),
         format!("(N/N_Rd)^{psi} + (M_y/M_y,Rd)^{eta_y} + (M_z/M_z,Rd)^{xi_z} = {u:.3} (Klasse {class:?})"))
    };
    let rem = run.loads(u_nm, "Reduce the combined axial force and moments", "Normalkraft und Momente gemeinsam senken", "nK");
    run.emit(&mut out, "en1999.6.2.9.nm", "6.2.9", ("Combined axial force and bending", "Interaktion Normalkraft und Biegung"), q_dim(u_nm), q_dim(1.0), (nm_en, nm_de), path_n.clone(), rem);
    let udl = member.actions.iter().find(|a| a.id == gov.action_id).is_some_and(|a| a.source == semio_s_artifact_norm_en1999_model::ActionSource::Udl);
    if let Some(last) = out.last_mut() {
        let (psi, eta_y, xi_z) = part_1_1::section_interaction_exponents(section.kind);
        attach(last, |s| {
            let f = moment_shape(member, udl, s);
            let u = if linear_class {
                ratio(n_abs, nm_rd) + ratio(my.abs() * f + n_abs * e_nz, m_rd_y) + ratio(mz.abs() * f + n_abs * e_ny, m_rd_z)
            } else {
                ratio(n_abs, nm_rd).powf(psi) + ratio(my * f, m_rd_y).powf(eta_y) + ratio(mz * f, m_rd_z).powf(xi_z)
            };
            (u, vec![q_dim(u)])
        });
    }

    let k_sup = match member.support { SupportCondition::Continuous => 0.7, SupportCondition::Cantilever => 2.0, SupportCondition::SimplySupported => 1.0 };
    let pick = |l: f64| if l > 0.0 { l } else { k_sup * member.length };
    let (l_cr_y, l_cr_z, l_cr_t) = (pick(member.buckling_length_y), pick(member.buckling_length_z), pick(member.buckling_length_t));
    let l_lt = if member.ltb_length > 0.0 { member.ltb_length } else { member.length };
    let a_loc = loc_p.a;
    let kappa_0 = if has_haz && a_loc > 0.0 { eff_o.a / a_loc } else { 1.0 };
    let (alpha, lambda_0) = part_1_1::buckling_curve(class_a);
    let omega = |kappa_chi: f64, l: f64| if member.transverse_weld { part_1_1::omega_x(kappa_chi, alloy.rho_o_haz, member.transverse_weld_offset, l, 1.0) } else { 1.0 };
    let n_b = |i: f64, l: f64| {
        let lam = part_1_1::lambda_bar(l, i, a_loc, fo);
        let kc = part_1_1::kappa(kappa_0, lam) * part_1_1::chi_from_lambda(lam, alpha, lambda_0);
        (lam, omega(kc, l) * kc * a_loc * fo / g1)
    };
    let (lam_y, n_b_y) = n_b(i_y, l_cr_y);
    let (lam_z, n_b_z) = n_b(i_z, l_cr_z);
    let chi_z = part_1_1::chi_from_lambda(lam_z, alpha, lambda_0);
    for (id, axis, lam, n_rd_b, i, l_cr, len_field) in [
        ("en1999.6.3.1.fb", "y", lam_y, n_b_y, i_y, l_cr_y, "bucklingLengthY"),
        ("en1999.6.3.1.fb.z", "z", lam_z, n_b_z, i_z, l_cr_z, "bucklingLengthZ"),
    ] {
        let mut rem = run.loads(nc / n_rd_b.max(1e-18), &format!("Reduce the compression force (flexural buckling about {axis})"), &format!("Druckkraft senken (Biegeknicken um {axis})"), "nK");
        if nc > n_rd_b && n_rd_b > 0.0 {
            let lo = solve_length(l_cr, nc, |l| n_b(i, l).1);
            let cur = if axis == "y" { member.buckling_length_y } else { member.buckling_length_z };
            let req = lo.min(cur);
            rem.push(Remedy::at_most(
                SubjectRef::new(&member.id, format!("members[id={}].{len_field}", member.id), member_label(&member.id)),
                q_length(cur), q_length(req),
                loc(&format!("Reduce the buckling length about {axis} to at most {req:.2} m."), &format!("Knicklänge um {axis} auf höchstens {req:.2} m verkürzen.")),
            ));
        }
        run.emit(&mut out, id, "6.3.1.1", (if axis == "y" { "Flexural buckling N_b,Rd,y" } else { "Flexural buckling N_b,Rd,z" }, if axis == "y" { "Biegeknicken N_b,Rd,y" } else { "Biegeknicken N_b,Rd,z" }),
            q_force(nc), q_force(n_rd_b),
            (format!("λ̄_{axis} = {lam:.4}, κ_0 = {kappa_0:.3}, α = {alpha:.2}, λ̄_0 = {lambda_0:.2}, N_b,Rd,{axis} = {:.1} kN (A_eff = {:.1} mm², L_cr = {l_cr:.2} m)", n_rd_b / 1e3, a_loc * 1e6),
             format!("λ̄_{axis} = {lam:.4}, κ_0 = {kappa_0:.3}, α = {alpha:.2}, λ̄_0 = {lambda_0:.2}, N_b,Rd,{axis} = {:.1} kN (A_eff = {:.1} mm², L_cr = {l_cr:.2} m)", n_rd_b / 1e3, a_loc * 1e6)),
            format!("members[id={}].{len_field}", member.id), rem);
    }
    if has_haz || member.transverse_weld {
        let (n_w, axis, l_w) = if n_b_y <= n_b_z { (n_b_y, "y", l_cr_y) } else { (n_b_z, "z", l_cr_z) };
        let i_w_axis = if axis == "y" { i_y } else { i_z };
        let lam_w = part_1_1::lambda_bar(l_w, i_w_axis, a_loc, fo);
        let kappa_w = part_1_1::kappa(kappa_0, lam_w);
        let omega_w = omega(kappa_w * part_1_1::chi_from_lambda(lam_w, alpha, lambda_0), l_w);
        let rem = run.loads(nc / n_w.max(1e-18), "Reduce the compression force (welded member, HAZ)", "Druckkraft senken (geschweißtes Bauteil, WEZ)", "nK");
        run.emit(&mut out, "en1999.6.3.1.haz", "6.3.1.1", ("Welded member buckling, HAZ factors κ, ω_x", "Geschweißtes Bauteil, WEZ-Faktoren κ, ω_x"), q_force(nc), q_force(n_w),
            (format!("governing axis {axis}: κ = {kappa_w:.3} (κ_0 = {kappa_0:.3}), ω_x = {omega_w:.3}, N_b,Rd = {:.1} kN", n_w / 1e3),
             format!("maßgebende Achse {axis}: κ = {kappa_w:.3} (κ_0 = {kappa_0:.3}), ω_x = {omega_w:.3}, N_b,Rd = {:.1} kN", n_w / 1e3)),
            format!("members[id={}].transverseWeld", member.id), rem);
    }
    let n_cr_t = part_1_1::n_cr_torsional(gross.a, i_t, i_w, i_y, i_z, part_1_1::shear_centre_offset(section), l_cr_t);
    let lam_t = if n_cr_t.is_finite() && n_cr_t > 0.0 { (a_loc * fo / n_cr_t).sqrt() } else { 0.0 };
    let (alpha_t, lambda_0_t) = part_1_1::torsional_curve(class_a);
    let n_b_t = part_1_1::kappa(kappa_0, lam_t) * part_1_1::chi_from_lambda(lam_t, alpha_t, lambda_0_t) * a_loc * fo / g1;
    let rem = run.loads(nc / n_b_t.max(1e-18), "Reduce the compression force (torsional buckling)", "Druckkraft senken (Drillknicken)", "nK");
    run.emit(&mut out, "en1999.6.3.1.tb", "6.3.1.4", ("Torsional buckling N_b,T,Rd", "Drillknicken N_b,T,Rd"), q_force(nc), q_force(n_b_t),
        (format!("N_cr,T = {:.1} kN, λ̄_T = {lam_t:.4}, N_b,T,Rd = {:.1} kN", n_cr_t.min(1.0e12) / 1e3, n_b_t / 1e3),
         format!("N_cr,T = {:.1} kN, λ̄_T = {lam_t:.4}, N_b,T,Rd = {:.1} kN", n_cr_t.min(1.0e12) / 1e3, n_b_t / 1e3)),
        format!("members[id={}].bucklingLengthT", member.id), rem);

    let m_cr = if member.restrained_ltb { f64::INFINITY } else { part_1_1::m_cr_of(section, member.c1, l_lt) };
    let lam_lt = if m_cr.is_finite() && m_cr > 0.0 { (m_rd_y * g1 / m_cr).sqrt() } else { 0.0 };
    let (alpha_lt, lambda_0_lt) = part_1_1::ltb_curve(class);
    let chi_lt = part_1_1::chi_from_lambda(lam_lt, alpha_lt, lambda_0_lt);
    let omega_lt = omega(chi_lt, l_lt);
    let m_b_rd = omega_lt * chi_lt * m_rd_y;
    let mut rem = run.loads(my.abs() / m_b_rd.max(1e-18), "Reduce the strong-axis moment (lateral-torsional buckling)", "Moment um die starke Achse senken (Biegedrillknicken)", "mYK");
    if my.abs() > m_b_rd && !member.restrained_ltb {
        let lo = solve_length(l_lt, my.abs(), |l| {
            let mc = part_1_1::m_cr_of(section, member.c1, l);
            part_1_1::chi_from_lambda((m_rd_y * g1 / mc).sqrt(), alpha_lt, lambda_0_lt) * m_rd_y
        });
        rem.push(Remedy::at_most(
            SubjectRef::new(&member.id, format!("members[id={}].ltbLength", member.id), member_label(&member.id)),
            q_length(member.ltb_length), q_length(lo.min(member.ltb_length)),
            loc("Reduce the unrestrained length L_LT.", "Gabellagerungslänge L_LT verkürzen."),
        ));
    }
    run.emit(&mut out, "en1999.6.3.2.ltb", "6.3.2", ("Lateral-torsional buckling M_b,Rd", "Biegedrillknicken M_b,Rd"), q_moment(my.abs()), q_moment(m_b_rd),
        (format!("M_cr = {:.2} kNm, λ̄_LT = {lam_lt:.4}, α_LT = {alpha_lt:.2}, λ̄_0,LT = {lambda_0_lt:.2}, χ_LT = {chi_lt:.4}, M_b,Rd = {:.2} kNm", m_cr.min(1.0e12) / 1e3, m_b_rd / 1e3),
         format!("M_cr = {:.2} kNm, λ̄_LT = {lam_lt:.4}, α_LT = {alpha_lt:.2}, λ̄_0,LT = {lambda_0_lt:.2}, χ_LT = {chi_lt:.4}, M_b,Rd = {:.2} kNm", m_cr.min(1.0e12) / 1e3, m_b_rd / 1e3)),
        format!("members[id={}].ltbLength", member.id), rem);

    let (eta_c, gamma_c, xi_c) = part_1_1::member_interaction_exponents(class, chi_z);
    let eq59 = ratio(nc, n_b_y).powf(eta_c) + ratio(my, omega_lt * m_rd_y).powf(gamma_c) + ratio(mz, m_rd_z).powf(xi_c);
    let eq60 = ratio(nc, n_b_z).powf(eta_c) + ratio(my, m_b_rd).powf(gamma_c) + ratio(mz, m_rd_z).powf(xi_c);
    let u_int = eq59.max(eq60);
    let rem = run.loads(u_int, "Reduce the combined compression and bending demand", "Druck und Biegung gemeinsam senken", "nK");
    run.emit(&mut out, "en1999.6.3.3.int", "6.3.3", ("Member interaction (6.59/6.60)", "Bauteilinteraktion (6.59/6.60)"), q_dim(u_int), q_dim(1.0),
        (format!("Eq. 6.59 = {eq59:.3}, Eq. 6.60 = {eq60:.3} with η_c = {eta_c:.2}, γ_c = {gamma_c:.2}, ξ_c = {xi_c:.2}, ω_x = 1 (no localized transverse weld)"),
         format!("Gl. 6.59 = {eq59:.3}, Gl. 6.60 = {eq60:.3} mit η_c = {eta_c:.2}, γ_c = {gamma_c:.2}, ξ_c = {xi_c:.2}, ω_x = 1 (keine örtliche Querschweißnaht)")),
        path_n, rem);
    if let Some(last) = out.last_mut() {
        attach(last, |s| {
            let f = moment_shape(member, udl, s);
            let e59 = ratio(nc, n_b_y).powf(eta_c) + ratio(my * f, m_rd_y).powf(gamma_c) + ratio(mz * f, m_rd_z).powf(xi_c);
            let e60 = ratio(nc, n_b_z).powf(eta_c) + ratio(my * f, m_b_rd).powf(gamma_c) + ratio(mz * f, m_rd_z).powf(xi_c);
            (e59.max(e60), vec![q_dim(e59.max(e60))])
        });
    }
    out
}

fn member_sls(
    member: &AluminiumMember,
    section: &AluminiumSection,
    annex: AnnexChoice,
    params: &na_de::AnnexParams,
    alloy: &part_1_1::AlloyProps,
    kind: &str,
    gov: &GoverningEffects,
) -> Vec<CheckResult> {
    let fo = alloy.f_o_pa;
    let base = EffCtx { f_o_pa: fo, class_a: alloy.buckling_class_a, eta_web: 1.0, local: true, haz_rho: None, haz_extents: &[] };
    let p = part_1_1::props(section, &base);
    let act_dig = actions_digest(&member.actions);
    let sup_dig = member_support_digest(member);
    let tag_en = format!(" (SLS {}, lead {}); {act_dig}; {sup_dig}", gov.combination, gov.action_id);
    let tag_de = format!(" (GZG {}, führend {}); {act_dig}; {sup_dig}", gov.combination, gov.action_id);
    let label = member_label(&member.id);
    let sigma = gov.n_ed.abs() / p.a.max(1e-18) + gov.m_y_ed.abs() / p.wy.max(1e-18) + gov.m_z_ed.abs() / p.wz.max(1e-18);
    let sigma_lim = params.stress_ratio * fo;
    let mut out = Vec::new();
    if kind == "quasiPermanent" || kind == "characteristic" {
        let k_w = match member.support { SupportCondition::SimplySupported => 5.0 / 48.0, SupportCondition::Continuous => 1.0 / 32.0, SupportCondition::Cantilever => 0.25 };
        let w = if p.iy > 0.0 { k_w * gov.m_y_ed.abs() * member.length.powi(2) / (na_de::E_PA * p.iy) } else { 0.0 };
        let ratio_lim = if member.support == SupportCondition::Cantilever { params.deflection_ratio * 0.5 } else { params.deflection_ratio };
        let w_lim = member.length / ratio_lim;
        let rem = if w > w_lim { leaf_remedies("members", &member.id, &member.actions, w / w_lim.max(1e-12), &label, "Reduce the loads for the deflection limit", "Lasten für die Durchbiegungsgrenze senken", "mYK") } else { Vec::new() };
        out.push(utilization_check(
            format!("{}.{}", if kind == "quasiPermanent" { "en1999.7.2.defl-qp" } else { "en1999.7.2.defl" }, member.id), "DIN EN 1999-1-1", clause("7.2"),
            SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, gov.action_id), label.clone()),
            if kind == "quasiPermanent" { loc("SLS deflection (quasi-permanent)", "GZG Durchbiegung (quasi-ständig)") } else { loc("SLS deflection (characteristic)", "GZG Durchbiegung (charakteristisch)") },
            q_length(w), q_length(w_lim.max(1e-9)), annex,
            loc(&format!("w = {:.1} mm ≤ L/{ratio_lim:.0} = {:.1} mm, I_eff = {:.3e} m⁴{tag_en}", w * 1e3, w_lim * 1e3, p.iy),
                &format!("w = {:.1} mm ≤ L/{ratio_lim:.0} = {:.1} mm, I_eff = {:.3e} m⁴{tag_de}", w * 1e3, w_lim * 1e3, p.iy)),
            rem,
        ));
        if kind == "quasiPermanent" { return out; }
    }
    let (id, title) = if kind == "characteristic" { ("en1999.7.2.stress", ("SLS stress (characteristic)", "GZG Spannung (charakteristisch)")) } else { ("en1999.7.2.sls-freq", ("SLS stress (frequent)", "GZG Spannung (häufig)")) };
    let rem = if sigma > sigma_lim { leaf_remedies("members", &member.id, &member.actions, sigma / sigma_lim.max(1.0), &label, "Reduce the loads for the stress limit", "Lasten für die Spannungsgrenze senken", "mYK") } else { Vec::new() };
    let mut stress = utilization_check(
        format!("{id}.{}", member.id), "DIN EN 1999-1-1", clause("7.2"),
        SubjectRef::new(&member.id, format!("members[id={}].actions[id={}].mYK", member.id, gov.action_id), label),
        loc(title.0, title.1),
        q_stress(sigma), q_stress(sigma_lim), annex,
        loc(&format!("σ = {:.1} MPa ≤ {:.2}·f_o = {:.1} MPa{tag_en}", sigma / 1e6, params.stress_ratio, sigma_lim / 1e6),
            &format!("σ = {:.1} MPa ≤ {:.2}·f_o = {:.1} MPa{tag_de}", sigma / 1e6, params.stress_ratio, sigma_lim / 1e6)),
        rem,
    );
    if stress.status == semio_s_plugin_norm_compliance::CheckStatus::Fail { stress.status = semio_s_plugin_norm_compliance::CheckStatus::Warning; }
    out.push(stress);
    out
}

/// 📊 The design resistances of a member for one combination, shared with viewers so they never re-derive them.
#[derive(Clone, Debug, PartialEq)]
pub struct MemberResistances {
    pub class: SectionClass,
    pub eta: f64,
    pub n_t_rd: f64,
    pub n_c_rd: f64,
    pub m_y_rd: f64,
    pub m_z_rd: f64,
    pub v_rd: f64,
    pub n_b_rd_y: f64,
    pub n_b_rd_z: f64,
    pub m_b_rd: f64,
}

/// 📊 Resistances of `member` under the stress state of `gov` (the stress gradient η and the class depend on it); `None` for an unknown alloy.
pub fn member_resistances(member: &AluminiumMember, section: &AluminiumSection, material: &semio_s_artifact_norm_en1999_model::AluminiumMaterial, annex: AnnexChoice, gov: &GoverningEffects, haz_extents: &[(String, f64)]) -> Option<MemberResistances> {
    let params = na_de::AnnexParams::for_choice(annex);
    let alloy = part_1_1::resolve_alloy_for_section(&material.designation, section)?;
    let (fo, fu, g1, g2) = (alloy.f_o_pa, alloy.f_u_pa, params.gamma_m1, params.gamma_m2);
    let class_a = alloy.buckling_class_a;
    let (_, i_y, i_z, _, _, _, _) = part_1_1::section_geometry(section);
    let gross = part_1_1::props(section, &EffCtx::gross(fo, class_a));
    let c = -gov.n_ed / gross.a.max(1e-18);
    let (s1, s2) = (c + gov.m_y_ed.abs() / gross.wy.max(1e-18), c - gov.m_y_ed.abs() / gross.wy.max(1e-18));
    let (s_max, s_min) = (s1.max(s2), s1.min(s2));
    let eta = if s_max > 0.0 { part_1_1::stress_gradient_eta(s_min / s_max) } else { 1.0 };
    let class = part_1_1::section_class(section, fo, class_a, eta);
    let base = EffCtx { f_o_pa: fo, class_a, eta_web: eta, local: true, haz_rho: None, haz_extents };
    let loc_p = part_1_1::props(section, &base);
    let eff_o = part_1_1::props(section, &EffCtx { haz_rho: Some(alloy.rho_o_haz), ..base });
    let haz_o = part_1_1::props(section, &EffCtx { local: false, haz_rho: Some(alloy.rho_o_haz), ..base });
    let haz_u = part_1_1::props(section, &EffCtx { local: false, haz_rho: Some(alloy.rho_u_haz), ..base });
    let has_haz = section.elements.iter().any(|e| e.welded);
    let n_t_rd = (haz_o.a * fo / g1).min((if has_haz { haz_u.a } else { 0.9 * gross.a }) * fu / g2);
    let (wpl_y, wpl_z) = part_1_1::plastic_moduli(section);
    let (gb, gl) = if part_1_1::is_tube(section) {
        (part_1_1::tube_beta(section) / part_1_1::epsilon(fo), part_1_1::class_limits(class_a, false, false))
    } else {
        part_1_1::governing_element(section, fo, class_a, eta).map(|(_, b, l)| (b, l)).unwrap_or((0.0, [11.0, 16.0, 22.0]))
    };
    let al_y = if has_haz { 1.0 } else { part_1_1::shape_factor(class, wpl_y, gross.wy, gb, gl) };
    let al_z = if has_haz { 1.0 } else { part_1_1::shape_factor(class, wpl_z, gross.wz, gb, gl) };
    let w_y = if has_haz { eff_o.wy } else if class == SectionClass::Class4 { loc_p.wy } else { gross.wy };
    let w_z = if has_haz { eff_o.wz } else if class == SectionClass::Class4 { loc_p.wz } else { gross.wz };
    let (m_y_rd, m_z_rd) = (al_y * w_y * fo / g1, al_z * w_z * fo / g1);
    let k_sup = match member.support { SupportCondition::Continuous => 0.7, SupportCondition::Cantilever => 2.0, SupportCondition::SimplySupported => 1.0 };
    let pick = |l: f64| if l > 0.0 { l } else { k_sup * member.length };
    let a_loc = loc_p.a;
    let kappa_0 = if has_haz && a_loc > 0.0 { eff_o.a / a_loc } else { 1.0 };
    let (alpha, l0) = part_1_1::buckling_curve(class_a);
    let nb = |i: f64, l: f64| { let lam = part_1_1::lambda_bar(l, i, a_loc, fo); part_1_1::kappa(kappa_0, lam) * part_1_1::chi_from_lambda(lam, alpha, l0) * a_loc * fo / g1 };
    let l_lt = if member.ltb_length > 0.0 { member.ltb_length } else { member.length };
    let m_cr = if member.restrained_ltb { f64::INFINITY } else { part_1_1::m_cr_of(section, member.c1, l_lt) };
    let lam_lt = if m_cr.is_finite() && m_cr > 0.0 { (m_y_rd * g1 / m_cr).sqrt() } else { 0.0 };
    let (a_lt, l0_lt) = part_1_1::ltb_curve(class);
    Some(MemberResistances {
        class,
        eta,
        n_t_rd,
        n_c_rd: eff_o.a * fo / g1,
        m_y_rd,
        m_z_rd,
        v_rd: haz_o.a_v * fo / (3.0f64.sqrt() * g1),
        n_b_rd_y: nb(i_y, pick(member.buckling_length_y)),
        n_b_rd_z: nb(i_z, pick(member.buckling_length_z)),
        m_b_rd: part_1_1::chi_from_lambda(lam_lt, a_lt, l0_lt) * m_y_rd,
    })
}

/// 📋️ Evaluate one member against EN 1999-1-1 cross-section, stability, plated-web and serviceability clauses; every combination is evaluated and the worst utilization per check is kept. `web_rows` are the transverse stiffeners `(id, position)` of the member (see `web_stiffener::rows_of`; the web panels between them and the supports drive 6.7.4 and 6.7.5, none = unstiffened web), `haz_extents` are the per-element b_haz overrides of the member's weld runs ([`weld_run_checks::haz_overrides`]), empty for the §6.1.6.3 table.
pub fn check_member(
    member: &AluminiumMember,
    section: &AluminiumSection,
    material: &semio_s_artifact_norm_en1999_model::AluminiumMaterial,
    annex: AnnexChoice,
    reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass,
    haz_extents: &[(String, f64)],
    web_rows: &[(String, f64)],
) -> Vec<CheckResult> {
    let params = na_de::AnnexParams::for_choice(annex);
    let Some(alloy) = part_1_1::resolve_alloy_for_section(&material.designation, section) else {
        return vec![alloy_fail(format!("en1999.3.2.alloy.{}", member.id), "DIN EN 1999-1-1", &member.id, material, annex)];
    };
    let mut sets: Vec<Vec<CheckResult>> = part_en1990::uls_combinations(member, &params, reliability_class)
        .iter()
        .map(|g| member_uls(member, section, material, annex, &params, &alloy, g, haz_extents, web_rows))
        .collect();
    for kind in ["quasiPermanent", "characteristic", "frequent"] {
        for g in part_en1990::sls_combinations(member, kind) {
            sets.push(member_sls(member, section, annex, &params, &alloy, kind, &g));
        }
    }
    worst_by_id(sets)
}
//#endregion 🔖️ComplianceHelpers
