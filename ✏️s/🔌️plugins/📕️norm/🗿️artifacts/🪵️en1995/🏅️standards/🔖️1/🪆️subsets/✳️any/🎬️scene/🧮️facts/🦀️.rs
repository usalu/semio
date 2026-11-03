//! 🧮️ Facts: the numbers the scene draws that the check report does not carry, computed with the public kernel functions of EN 1995 (`checks::timber`) so the picture and the verdict share one arithmetic.

extern crate semio_s_artifact_norm_en1995_model as model;
extern crate semio_s_artifact_norm_en1995_schema as schema;
extern crate semio_s_plugin_norm_compliance as norm;

use model::{MemberRole, SupportType, TimberConnection, TimberMember, TimberProduct, TimberProperties};
use norm::AnnexChoice;
use schema::checks::timber as t;

/// 🪵️ Tabulated strength properties of the member or connection class.
pub fn properties(class: &str) -> Option<TimberProperties> {
    t::properties_for_class(class)
}

/// 🔩️ The fastener family of a connection as the kernel keys it: `nail`, `screw`, `bolt` or `dowel`.
pub fn fastener_kind(connection: &TimberConnection) -> &'static str {
    family_of(connection.fastener_type)
}

/// 🔩️ The fastener family (`nail`, `screw`, `bolt` or `dowel`) of a fastener type text.
pub fn family_of(fastener_type: semio_s_artifact_norm_en1995_model::FastenerType) -> &'static str {
    use semio_s_artifact_norm_en1995_model::FastenerType as F;
    match fastener_type { F::Nail => "nail", F::Screw => "screw", F::Bolt => "bolt", F::Dowel => "dowel" }
}

/// 📏️ The minima `(name, mm)` of Tables 8.2, 8.4, 8.5 and 8.6 of a fastener family of diameter `d_mm` at the load angle `alpha_deg`.
pub fn spacing_minima(kind: &str, d_mm: f64, alpha_deg: f64, rho_k: f64) -> Vec<(&'static str, f64)> {
    t::spacing_table(kind, d_mm, alpha_deg, rho_k)
}

/// 📏️ The fastener grid of one connection in metres: `per_row` along the grain at `a1`, `rows` across at `a2` (the Table 8.2/8.4/8.5 minimum, since the snapshot has no a2), `end` and `edge` distances and the plan extent.
#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub kind: &'static str,
    pub per_row: u32,
    pub rows: u32,
    pub a1: f64,
    pub a2: f64,
    pub end: f64,
    pub edge: f64,
    pub length: f64,
    pub width: f64,
}

pub fn grid(c: &TimberConnection) -> Grid {
    let kind = fastener_kind(c);
    let rows = c.rows.max(1);
    let per_row = ((f64::from(c.number) / f64::from(rows)).round() as u32).max(1);
    let rho = properties(c.strength_class.id()).map_or(350.0, |p| p.rho_k);
    let table = t::spacing_table(if kind == "screw" { "nail" } else { kind }, c.diameter_m * 1000.0, 0.0, rho);
    let a2 = table.iter().find(|r| r.0 == "a2").map_or(3.0 * c.diameter_m, |r| r.1 / 1000.0).max(c.diameter_m * 2.0).max(1e-3);
    let (a1, end, edge) = (c.spacing_m.max(1e-3), c.end_distance_m.max(1e-3), c.edge_distance_m.max(1e-3));
    Grid { kind, per_row, rows, a1, a2, end, edge, length: 2.0 * end + f64::from(per_row - 1) * a1, width: 2.0 * edge + f64::from(rows - 1) * a2 }
}

/// 🥪️ One layer of the connection stack, bottom up: thickness in m and whether it is a steel plate.
#[derive(Clone, Debug, PartialEq)]
pub struct Layer {
    pub thickness: f64,
    pub steel: bool,
}

pub fn layers(c: &TimberConnection) -> Vec<Layer> {
    let middle = Layer { thickness: if c.steel_plate { c.steel_plate_thickness_m } else { c.t2_m }.max(1e-3), steel: c.steel_plate };
    let side = Layer { thickness: c.t1_m.max(1e-3), steel: false };
    if c.shear_planes >= 2 { vec![side.clone(), middle, side] } else { vec![side, middle] }
}

/// 🔩️ The Johansen European Yield Model of a connection: modes with their characteristic load per shear plane in N, the governing index, and the inputs in N/mm.
#[derive(Clone, Debug)]
pub struct Johansen {
    pub config: &'static str,
    pub modes: Vec<(&'static str, f64)>,
    pub governing: usize,
    pub fh: f64,
    pub my: f64,
}

pub fn johansen(c: &TimberConnection) -> Option<Johansen> {
    let props = properties(c.strength_class.id())?;
    let kind = fastener_kind(c);
    let (d, t1, t2, ts) = (c.diameter_m * 1000.0, c.t1_m * 1000.0, c.t2_m * 1000.0, c.steel_plate_thickness_m * 1000.0);
    let fh = t::embedment_fh(kind, "timber", props.rho_k, d, 0.0, "softwood", 0.0);
    let my = t::yield_moment(kind, c.f_u_k / 1e6, d, 0.7 * d);
    let share = t::rope_share_of(kind);
    let fax = if kind == "screw" { t::screw_withdrawal_n(d, t2, props.rho_k) } else { 0.0 };
    let (config, modes) = match (c.steel_plate, c.shear_planes >= 2) {
        (true, true) => ("double_center", t::modes_steel_timber("double_center", t1, ts, d, fh, my, fax, share)),
        (true, false) => ("single", t::modes_steel_timber("single", t1, ts, d, fh, my, fax, share)),
        (false, true) => ("tt_double", t::modes_tt_double(t1, t2, d, fh, fh, my, fax, share)),
        (false, false) => ("tt_single", t::modes_tt_single(t1, t2, d, fh, fh, my, fax, share)),
    };
    let governing = modes.iter().enumerate().min_by(|a, b| a.1 .1.total_cmp(&b.1 .1)).map(|(i, _)| i)?;
    Some(Johansen { config, modes, governing, fh, my })
}

/// 🌿️ Slenderness data about both axes: relative slenderness, radius of gyration in m and the imperfection factor β_c.
#[derive(Clone, Copy, Debug)]
pub struct Slenderness {
    pub rel_y: f64,
    pub rel_z: f64,
    pub i_y: f64,
    pub i_z: f64,
    pub beta_c: f64,
    pub scale: f64,
}

pub fn slenderness(m: &TimberMember, props: &TimberProperties) -> Slenderness {
    let a = t::area_m2(m).max(1e-12);
    let (i_y, i_z) = ((t::i_m4(m) / a).sqrt(), (m.h_m * m.b_m.powi(3) / 12.0 / a).sqrt());
    let beta_c = if props.product == TimberProduct::Solid { 0.2 } else { 0.1 };
    let scale = (props.f_c_0_k / props.e_0_05).sqrt() / std::f64::consts::PI;
    Slenderness { rel_y: m.buckling_length_y_m / i_y.max(1e-12) * scale, rel_z: m.buckling_length_z_m / i_z.max(1e-12) * scale, i_y, i_z, beta_c, scale }
}

/// 🌿️ The relative lateral slenderness λ_rel,m and the factor k_crit of the member.
pub fn lateral(m: &TimberMember, props: &TimberProperties) -> (f64, f64) {
    let rel = t::lambda_rel_m(t::w_m3(m), props.f_m_k, t::effective_m_crit_nm(m, props.e_0_05));
    (rel, t::kcrit_factor(rel))
}

/// 🛋️ Deflection components of the characteristic SLS combination in m, with the limits.
#[derive(Clone, Copy, Debug)]
pub struct Deflection {
    pub w_g: f64,
    pub w_q: f64,
    pub creep: f64,
    pub w_fin: f64,
    pub limit_inst: f64,
    pub limit_fin: f64,
}

fn line_deflection(m: &TimberMember, ei: f64, q: f64, f: f64) -> f64 {
    let l = m.span_m;
    if ei <= 0.0 || l <= 0.0 {
        return 0.0;
    }
    match m.support {
        SupportType::SimplySupported => 5.0 * q * l.powi(4) / (384.0 * ei) + f * l.powi(3) / (48.0 * ei),
        SupportType::Cantilever => q * l.powi(4) / (8.0 * ei) + f * l.powi(3) / (3.0 * ei),
        SupportType::ContinuousTwoSpan => q * l.powi(4) / (185.0 * ei) + f * l.powi(3) / (100.0 * ei),
    }
}

pub fn deflection(annex: AnnexChoice, reliability_class: semio_s_plugin_norm_compliance::ReliabilityClass, m: &TimberMember, props: &TimberProperties) -> Option<Deflection> {
    let params = t::AnnexParams::new(annex, reliability_class);
    let sls = t::enumerate_combos(m, &params).into_iter().filter(|c| c.kind == t::ComboKind::SlsCharacteristic).max_by(|a, b| a.q_ed_line.abs().total_cmp(&b.q_ed_line.abs()))?;
    if m.span_m <= 0.0 || (sls.q_ed_line.abs() == 0.0 && sls.f_ed_point.abs() == 0.0) {
        return None;
    }
    let ei = props.e_0_mean * t::i_m4(m);
    let permanent = |k: semio_s_artifact_norm_en1995_model::ActionKind| k == semio_s_artifact_norm_en1995_model::ActionKind::Permanent;
    let (qg, fg) = m.actions.iter().filter(|a| permanent(a.kind)).fold((0.0, 0.0), |s, a| (s.0 + a.q_line_n_per_m, s.1 + a.f_point_n));
    let w_g = line_deflection(m, ei, qg, fg);
    let w_q = (line_deflection(m, ei, sls.q_ed_line, sls.f_ed_point) - w_g).max(0.0);
    let kdef = t::k_def(t::parse_service_class(m.service_class), props.product);
    let leff = if m.support == SupportType::Cantilever { 2.0 * m.span_m } else { m.span_m };
    Some(Deflection {
        w_g,
        w_q,
        creep: w_g * kdef + w_q * sls.psi2 * kdef,
        w_fin: w_g * (1.0 + kdef) + w_q * (1.0 + sls.psi2 * kdef),
        limit_inst: leff / params.w_inst_limit_divisor(),
        limit_fin: leff / params.w_fin_limit_divisor(),
    })
}

/// 🌊️ The floor or footbridge dynamic model: stiffnesses, mass, damping and the national limits; every response is a function of the span so the diagrams can sweep it.
#[derive(Clone, Copy, Debug)]
pub struct Dynamics {
    pub ei_l: f64,
    pub ei_b: f64,
    pub mass: f64,
    pub zeta: f64,
    pub f1_limit: f64,
    pub unit_limit: f64,
    pub bridge: bool,
}

pub fn dynamics(annex: AnnexChoice, m: &TimberMember, props: &TimberProperties) -> Option<Dynamics> {
    let de = annex == AnnexChoice::De;
    match m.role {
        MemberRole::Floor => {
            let strip = if m.mass_kg_per_m > 0.0 && m.mass_kg_per_m2 > 0.0 { m.mass_kg_per_m / m.mass_kg_per_m2 } else { m.b_m.max(0.05) };
            let mass = if m.mass_kg_per_m2 > 0.0 { m.mass_kg_per_m2 } else { (if m.mass_kg_per_m > 0.0 { m.mass_kg_per_m } else { props.rho_k * t::area_m2(m) }) / strip };
            Some(Dynamics { ei_l: props.e_0_mean * t::i_m4(m) / strip, ei_b: props.e_0_mean * 0.022_f64.powi(3) / 12.0, mass, zeta: if m.damping_xi > 0.0 { m.damping_xi } else { 0.01 }, f1_limit: if de { 6.0 } else { 8.0 }, unit_limit: if de { 0.5 } else { 1.5 }, bridge: false })
        }
        MemberRole::Bridge if m.mass_kg_per_m > 0.0 => Some(Dynamics { ei_l: props.e_0_mean * t::i_m4(m), ei_b: props.e_0_mean * 0.022_f64.powi(3) / 12.0, mass: m.mass_kg_per_m, zeta: m.damping_xi.max(1e-4), f1_limit: 5.0, unit_limit: 0.0, bridge: true }),
        _ => None,
    }
}

/// 📐️ Bending stiffness `E0,mean I` (N m²) of the whole section about the strong axis.
pub fn bending_stiffness(m: &TimberMember, props: &TimberProperties) -> f64 {
    props.e_0_mean * t::i_m4(m)
}

impl Dynamics {
    /// ⚖️ The linear mass (kg/m) of the member behind this model: the mass per area over the strip width the stiffness is per metre of (the whole member for a footbridge).
    pub fn linear_mass(&self, m: &TimberMember, props: &TimberProperties) -> f64 {
        self.mass * bending_stiffness(m, props) / self.ei_l.max(1e-9)
    }

    pub fn f1(&self, span: f64) -> f64 {
        if self.bridge { t::bridge_f1(span, self.ei_l, self.mass) } else { t::floor_f1(span, self.ei_l, self.mass) }
    }

    pub fn velocity(&self, span: f64) -> f64 {
        t::velocity_response(self.f1(span), span, span, self.mass, self.ei_l, self.ei_b).0
    }

    pub fn velocity_limit(&self, span: f64) -> f64 {
        100.0_f64.powf(self.f1(span) * self.zeta - 1.0)
    }
}

/// 🔥️ The fire state after a duration: charring rate in m/s, char depth, effective depth with the zero-strength layer and the residual section, all in m.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fire {
    pub minutes: f64,
    pub beta_n: f64,
    pub d_char: f64,
    pub d_ef: f64,
    pub b_fi: f64,
    pub h_fi: f64,
}

pub fn fire_after(annex: AnnexChoice, product: TimberProduct, b: f64, h: f64, seconds: f64) -> Fire {
    let beta_n = t::AnnexParams::charring_rate(annex, product);
    let minutes = (seconds / 60.0).max(0.0);
    let d_char_mm = beta_n * 1e3 * 60.0 * minutes;
    let d_ef = t::d_ef_mm(d_char_mm, minutes) / 1000.0;
    let (b_fi, h_fi) = t::residual_bh(b, h, d_ef);
    Fire { minutes, beta_n, d_char: d_char_mm / 1000.0, d_ef, b_fi, h_fi }
}

pub fn fire_of(annex: AnnexChoice, m: &TimberMember, props: &TimberProperties) -> Option<Fire> {
    (m.fire_duration_s > 0.0).then(|| fire_after(annex, props.product, m.b_m, m.h_m, m.fire_duration_s))
}

/// 🔀️ The EN 1990 combinations the checks of the members run, as `(English, German)` captions in member order: the ULS, SLS and accidental combinations of every member with actions.
pub fn combination_captions(document: &model::En1995Snapshot) -> Vec<(String, String)> {
    let params = t::AnnexParams::new(document.annex, document.reliability_class);
    document
        .members
        .iter()
        .flat_map(|member| t::enumerate_combos(member, &params))
        .map(|combo| {
            let (en, de) = match combo.kind {
                t::ComboKind::Uls => ("ULS", "GZT"),
                t::ComboKind::SlsCharacteristic => ("SLS characteristic", "GZG charakteristisch"),
                t::ComboKind::SlsFrequent => ("SLS frequent", "GZG häufig"),
                t::ComboKind::SlsQuasiPermanent => ("SLS quasi-permanent", "GZG quasi-ständig"),
                t::ComboKind::Accidental => ("Accidental", "Außergewöhnlich"),
            };
            (format!("{en} · {}", combo.id), format!("{de} · {}", combo.id))
        })
        .collect()
}
