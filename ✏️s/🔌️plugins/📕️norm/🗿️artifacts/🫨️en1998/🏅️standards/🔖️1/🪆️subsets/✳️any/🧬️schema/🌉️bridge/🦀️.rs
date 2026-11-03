//! 🌉️ Seismic response of a bridge (EN 1998-2): piers as cantilever springs in series with their bearings, the deck as a rigid mass or as a beam on the springs, the modal solution by the shared mechanics kernel, CQC-combined support forces and displacements, capacity design of the piers and the seat length. Pure functions of numbers and model rows; the checks and the scene call them so picture and report cannot disagree. See [EN 1998-2](https://eur-lex.europa.eu/).

use crate::{part_1, part_3};
use semio_s_artifact_norm_en1998_model::{En1998Bearing, En1998Pier, PierStiffnessBasis};
use semio_s_plugin_norm_mechanics::{Analysis, Control, MassType, Material, Member as BeamMember, Model, NodalMass, Node, Section, Support as BeamSupport};

pub const DECK_ELEMENTS: f64 = 20.0;
pub const MODES: usize = 4;
pub const DAMPING: f64 = 0.05;

/// 🧭️ The direction of the analysis: along the deck axis (piers bend about their transverse axis) or across it (piers bend about their longitudinal axis).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Longitudinal,
    Transverse,
}

/// 🏛️ One support line of the deck: a pier with its bearings, or an abutment bearing, at a position along the deck axis.
#[derive(Clone, Debug, PartialEq)]
pub struct Support {
    pub pier_id: String,
    pub bearing_ids: Vec<String>,
    pub x_m: f64,
    pub pier_longitudinal_n_per_m: f64,
    pub pier_transverse_n_per_m: f64,
    pub bearing_n_per_m: f64,
    pub bearing_rigid: bool,
}

/// 📐 Stiffness of a cantilever pier with the free top, `k = 3 E I / H³` for `E` in MPa.
pub fn pier_stiffness_n_per_m(modulus_mpa: f64, inertia_m4: f64, height_m: f64) -> f64 {
    if modulus_mpa <= 0.0 || inertia_m4 <= 0.0 || height_m <= 0.0 { 0.0 } else { 3.0 * modulus_mpa * 1.0e6 * inertia_m4 / height_m.powi(3) }
}

/// 🔗 Pier and bearing in series, `1/k = 1/k_p + 1/k_b`; a rigid bearing leaves the pier alone, a pier without stiffness leaves nothing.
pub fn series_n_per_m(pier: f64, bearing: f64, bearing_rigid: bool) -> f64 {
    if pier <= 0.0 { 0.0 } else if bearing_rigid || bearing <= 0.0 { pier } else { pier * bearing / (pier + bearing) }
}

impl Support {
    pub fn is_pier(&self) -> bool {
        !self.pier_id.is_empty()
    }

    /// 🔗 Stiffness of the support line in the direction; an abutment without a bearing stiffness gives no restraint.
    pub fn stiffness_n_per_m(&self, direction: Direction) -> f64 {
        if !self.is_pier() {
            return if self.bearing_rigid { 0.0 } else { self.bearing_n_per_m };
        }
        series_n_per_m(if direction == Direction::Longitudinal { self.pier_longitudinal_n_per_m } else { self.pier_transverse_n_per_m }, self.bearing_n_per_m, self.bearing_rigid)
    }

    /// 📏 Share of the support displacement that sits in the bearing, `k / k_b` (the pier takes the rest).
    pub fn bearing_share(&self, direction: Direction) -> f64 {
        if self.bearing_rigid || self.bearing_n_per_m <= 0.0 { return 0.0; }
        if !self.is_pier() { return 1.0; }
        self.stiffness_n_per_m(direction) / self.bearing_n_per_m
    }
}

/// 🪨️ Cracked share of the gross pier inertia for the stiffness of the design displacement, `I_eff = r I_g` (EN 1998-2 2.3.6.1), as declared by the designer: a share above one is capped, a share of zero or less leaves the piers without stiffness (the check `action.stiffness` fails) instead of silently falling back to the gross section.
pub fn cracked_ratio(declared: f64) -> f64 {
    declared.clamp(0.0, 1.0)
}

/// 🧱 Section of a pier for bending in `direction` (longitudinal: depth `section_h_m`, transverse: depth `section_b_m`) with its reinforcement inputs and axial load, for the shared fibre kernel.
pub fn pier_section(p: &En1998Pier, direction: Direction) -> part_3::RcMember {
    let (width, depth) = if direction == Direction::Longitudinal { (p.section_b_m, p.section_h_m) } else { (p.section_h_m, p.section_b_m) };
    part_3::RcMember::pier(width, depth, p.cover_m, p.reinforcement_ratio, p.axial_load_n, p.concrete_fck_mpa, p.steel_fyk_mpa)
}

/// 🧱 Gross flexural stiffness `E I_g` in N·m² of a pier for bending in `direction`.
pub fn pier_gross_ei_nm2(p: &En1998Pier, direction: Direction) -> f64 {
    let section = pier_section(p, direction);
    p.modulus_mpa * 1.0e6 * section.b * section.h.powi(3) / 12.0
}

/// 🧱 Effective-to-gross stiffness `EI_eff / EI_g` of a pier for bending in `direction` (EN 1998-2 2.3.6.1): the declared share for [`PierStiffnessBasis::Declared`], `M_y/φ_y` of the fibre section at the pier axial load (Annex C) for [`PierStiffnessBasis::Computed`], capped at the gross section; a section without response leaves the pier without stiffness.
pub fn pier_ei_ratio(p: &En1998Pier, direction: Direction, declared: f64, k: &part_3::AnnexA) -> f64 {
    match p.stiffness_basis {
        PierStiffnessBasis::Declared => cracked_ratio(declared),
        PierStiffnessBasis::Computed => pier_section(p, direction).effective_stiffness_nm2(k).map_or(0.0, |ei| cracked_ratio(ei / pier_gross_ei_nm2(p, direction))),
    }
}

/// 🏛️ Whether the bridge has any pier or bearing row, that is, whether [`supports`] yields a support line.
pub fn has_supports(bridge_id: &str, piers: &[En1998Pier], bearings: &[En1998Bearing]) -> bool {
    piers.iter().any(|p| p.bridge_id == bridge_id) || bearings.iter().any(|b| b.bridge_id == bridge_id)
}

/// 🏛️ The support lines of one bridge in order of position with the pier inertia reduced by [`cracked_ratio`]: every pier of the bridge with the bearings that name it, then one abutment line per distinct position of the bearings that name no pier of the bridge.
pub fn supports(bridge_id: &str, piers: &[En1998Pier], bearings: &[En1998Bearing], declared_cracked_ratio: f64, k: &part_3::AnnexA) -> Vec<Support> {
    let own: Vec<&En1998Pier> = piers.iter().filter(|p| p.bridge_id == bridge_id).collect();
    let mut out: Vec<Support> = own
        .iter()
        .map(|p| {
            let carried: Vec<&En1998Bearing> = bearings.iter().filter(|b| b.bridge_id == bridge_id && b.pier_id == p.id).collect();
            Support {
                pier_id: p.id.clone(),
                bearing_ids: carried.iter().map(|b| b.id.clone()).collect(),
                x_m: p.x_m,
                pier_longitudinal_n_per_m: pier_stiffness_n_per_m(p.modulus_mpa, pier_ei_ratio(p, Direction::Longitudinal, declared_cracked_ratio, k) * p.section_b_m * p.section_h_m.powi(3) / 12.0, p.height_m),
                pier_transverse_n_per_m: pier_stiffness_n_per_m(p.modulus_mpa, pier_ei_ratio(p, Direction::Transverse, declared_cracked_ratio, k) * p.section_h_m * p.section_b_m.powi(3) / 12.0, p.height_m),
                bearing_n_per_m: carried.iter().map(|b| b.stiffness_n_per_m.max(0.0)).sum(),
                bearing_rigid: carried.iter().any(|b| b.stiffness_n_per_m <= 0.0),
            }
        })
        .collect();
    for b in bearings.iter().filter(|b| b.bridge_id == bridge_id && !own.iter().any(|p| p.id == b.pier_id)) {
        match out.iter_mut().find(|s| !s.is_pier() && (s.x_m - b.x_m).abs() < 1.0e-6) {
            Some(line) => {
                line.bearing_ids.push(b.id.clone());
                line.bearing_n_per_m += b.stiffness_n_per_m.max(0.0);
                line.bearing_rigid |= b.stiffness_n_per_m <= 0.0;
            }
            None => out.push(Support { pier_id: String::new(), bearing_ids: vec![b.id.clone()], x_m: b.x_m, pier_longitudinal_n_per_m: 0.0, pier_transverse_n_per_m: 0.0, bearing_n_per_m: b.stiffness_n_per_m.max(0.0), bearing_rigid: b.stiffness_n_per_m <= 0.0 }),
        }
    }
    out.sort_by(|a, b| a.x_m.total_cmp(&b.x_m));
    out
}

/// 🎵️ The spectrum analysis of one direction: modal periods and effective mass shares, the unit-peak mode shapes at the nodes of the deck model, and the combined support forces and displacements.
#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub direction: Direction,
    pub periods_s: Vec<f64>,
    pub effective_mass_ratio: Vec<f64>,
    pub node_x_m: Vec<f64>,
    pub shapes: Vec<Vec<f64>>,
    pub support_force_n: Vec<f64>,
    pub support_displacement_m: Vec<f64>,
    pub base_shear_n: f64,
}

impl Response {
    pub fn participation(&self) -> f64 {
        self.effective_mass_ratio.iter().sum()
    }
}

struct Modes {
    periods: Vec<f64>,
    gamma: Vec<f64>,
    ratio: Vec<f64>,
    nodes: Vec<f64>,
    shapes: Vec<Vec<f64>>,
    support_nodes: Vec<usize>,
}

fn nodes_of(length_m: f64, xs: &[f64]) -> (Vec<f64>, Vec<usize>) {
    let mut key: Vec<f64> = xs.iter().copied().chain([0.0, length_m]).collect();
    key.sort_by(f64::total_cmp);
    key.dedup_by(|a, b| (*a - *b).abs() < 1.0e-6);
    let step = length_m / DECK_ELEMENTS;
    let mut nodes = vec![key[0]];
    for pair in key.windows(2) {
        let n = (((pair[1] - pair[0]) / step).ceil() as usize).max(1);
        nodes.extend((1..=n).map(|i| pair[0] + (pair[1] - pair[0]) * i as f64 / n as f64));
    }
    let support_nodes = xs.iter().map(|x| nodes.iter().enumerate().min_by(|a, b| (a.1 - x).abs().total_cmp(&(b.1 - x).abs())).map_or(0, |(i, _)| i)).collect();
    (nodes, support_nodes)
}

fn deck_modes(mass_kg: f64, length_m: f64, ei_nm2: f64, xs: &[f64], stiffness: &[f64]) -> Option<Modes> {
    if mass_kg <= 0.0 || length_m <= 0.0 || ei_nm2 <= 0.0 || xs.is_empty() || stiffness.iter().all(|k| *k <= 0.0) {
        return None;
    }
    let (nodes, support_nodes) = nodes_of(length_m, xs);
    let linear = mass_kg / length_m;
    let lumps: Vec<f64> = (0..nodes.len()).map(|i| linear * (nodes[(i + 1).min(nodes.len() - 1)] - nodes[i.saturating_sub(1)]) / 2.0).collect();
    let e = 1.0e10;
    let id = |i: usize| format!("n{i}");
    let model = Model {
        nodes: nodes.iter().enumerate().map(|(i, x)| Node { id: id(i), x: *x, y: 0.0, z: 0.0 }).collect(),
        materials: vec![Material { id: "deck".into(), e, g: e / 2.6, rho: 0.0 }],
        sections: vec![Section { id: "deck".into(), a: 1.0, iy: ei_nm2 / e, iz: ei_nm2 / e, j: ei_nm2 / e, iw: 0.0, asy: 0.0, asz: 0.0 }],
        members: (0..nodes.len() - 1).map(|i| BeamMember { id: format!("e{i}"), node_i: id(i), node_j: id(i + 1), material: "deck".into(), section: "deck".into(), ..Default::default() }).collect(),
        supports: (0..nodes.len())
            .map(|i| {
                let spring: f64 = support_nodes.iter().zip(stiffness).filter(|(n, _)| **n == i).map(|(_, k)| k.max(0.0)).sum();
                BeamSupport { node: id(i), fixed: [true, false, true, true, true, false], spring: [0.0, spring, 0.0, 0.0, 0.0, 0.0], ..Default::default() }
            })
            .collect(),
        masses: lumps.iter().enumerate().map(|(i, m)| NodalMass { node: id(i), mass: [*m, *m, *m, 0.0, 0.0, 0.0] }).collect(),
        ..Default::default()
    };
    let control = Control::none();
    let analysis = Analysis::new(&model, &control).ok()?;
    let result = analysis.modal(MODES, MassType::Lumped, &control).ok()?;
    let total: f64 = lumps.iter().sum();
    let (mut periods, mut gamma, mut ratio, mut shapes) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for mode in &result.modes {
        let psi: Vec<f64> = (0..nodes.len()).map(|i| mode.shape.iter().find(|d| d.node == id(i)).map_or(0.0, |d| d.u[1])).collect();
        let linear_term: f64 = psi.iter().zip(&lumps).map(|(p, m)| p * m).sum();
        let quadratic: f64 = psi.iter().zip(&lumps).map(|(p, m)| p * p * m).sum();
        let peak = psi.iter().fold(0.0_f64, |a, v| if v.abs() > a.abs() { *v } else { a });
        if quadratic <= 0.0 || peak == 0.0 || mode.frequency_hz <= 0.0 {
            continue;
        }
        periods.push(1.0 / mode.frequency_hz);
        gamma.push(linear_term / quadratic);
        ratio.push(linear_term * linear_term / quadratic / total);
        shapes.push(psi.iter().map(|p| p / peak).collect());
    }
    (!periods.is_empty()).then_some(Modes { periods, gamma, ratio, nodes, shapes, support_nodes })
}

/// 🌊️ The spectrum analysis of the deck mass `mass_kg` on the support lines at `xs` with the stiffnesses `stiffness`: a rigid deck (`ei_nm2 <= 0`) is one mode of period `2π √(M/K)`, a flexible deck of length `length_m` is solved by the mechanics kernel; `s_d` and `s_e` are the design and the elastic spectrum in m/s² as functions of the period. `None` without any restraint.
pub fn analyse(direction: Direction, mass_kg: f64, length_m: f64, ei_nm2: f64, xs: &[f64], stiffness: &[f64], s_d: &dyn Fn(f64) -> f64, s_e: &dyn Fn(f64) -> f64) -> Option<Response> {
    let total_stiffness: f64 = stiffness.iter().map(|k| k.max(0.0)).sum();
    if mass_kg <= 0.0 || total_stiffness <= 0.0 || xs.len() != stiffness.len() {
        return None;
    }
    if ei_nm2 <= 0.0 || xs.len() < 2 {
        let period = 2.0 * std::f64::consts::PI * (mass_kg / total_stiffness).sqrt();
        let omega2 = (2.0 * std::f64::consts::PI / period).powi(2);
        let force = mass_kg * s_d(period);
        return Some(Response {
            direction,
            periods_s: vec![period],
            effective_mass_ratio: vec![1.0],
            node_x_m: xs.to_vec(),
            shapes: vec![vec![1.0; xs.len()]],
            support_force_n: stiffness.iter().map(|k| force * k.max(0.0) / total_stiffness).collect(),
            support_displacement_m: vec![s_e(period) / omega2; xs.len()],
            base_shear_n: force,
        });
    }
    let length = if length_m > 0.0 { length_m } else { xs.iter().copied().fold(f64::MIN, f64::max) - xs.iter().copied().fold(f64::MAX, f64::min) };
    let modes = deck_modes(mass_kg, length, ei_nm2, xs, stiffness)?;
    let count = modes.periods.len();
    let mut forces = vec![vec![0.0; count]; xs.len()];
    let mut displacements = vec![vec![0.0; count]; xs.len()];
    let mut shears = vec![0.0; count];
    for n in 0..count {
        let omega2 = (2.0 * std::f64::consts::PI / modes.periods[n]).powi(2);
        let (a_d, a_e) = (s_d(modes.periods[n]), s_e(modes.periods[n]));
        for i in 0..xs.len() {
            let psi = modes.shapes[n][modes.support_nodes[i]];
            forces[i][n] = stiffness[i].max(0.0) * modes.gamma[n] * psi * a_d / omega2;
            displacements[i][n] = modes.gamma[n] * psi * a_e / omega2;
            shears[n] += forces[i][n];
        }
    }
    let combine = |values: &[f64]| part_1::cqc_combine(values, &modes.periods, DAMPING);
    Some(Response {
        direction,
        support_force_n: forces.iter().map(|v| combine(v.as_slice())).collect(),
        support_displacement_m: displacements.iter().map(|v| combine(v.as_slice())).collect(),
        base_shear_n: combine(shears.as_slice()),
        periods_s: modes.periods,
        effective_mass_ratio: modes.ratio,
        node_x_m: modes.nodes,
        shapes: modes.shapes,
    })
}

/// 🏛️ Capacity-design shear of a ductile pier `V_CD = γ₀ M_Rd / H` ([`part_2::pier_capacity_design_shear_n`](crate::part_2::pier_capacity_design_shear_n)) and the base moment `M_Ed = F H` of the design force at the pier top.
pub fn pier_base_moment_nm(top_force_n: f64, height_m: f64) -> f64 {
    top_force_n.abs() * height_m.max(0.0)
}

/// 🌍️ Effective ground displacement `d_g = c a_g S T_C T_D` (EN 1998-1 3.2.2.4) with the unverified coefficient `c = 0.025`.
pub fn ground_displacement_m(coefficient: f64, a_g: f64, s: f64, tc: f64, td: f64) -> f64 {
    coefficient * a_g * s * tc * td
}

/// 📏 Required seat length `l_ov = l_m + d_eg + d_Es` (EN 1998-2 6.6.2) with the effective ground displacement `d_eg = f d_g` and the seismic displacement of the support `d_Es`.
pub fn seat_length_m(min_support_m: f64, ground_factor: f64, ground_displacement_m: f64, support_displacement_m: f64) -> f64 {
    min_support_m + ground_factor * ground_displacement_m + support_displacement_m
}

/// 📏 Displacement the bearing of a support line takes, the deck displacement times the bearing share.
pub fn bearing_displacement_m(support: &Support, direction: Direction, deck_displacement_m: f64) -> f64 {
    deck_displacement_m.abs() * support.bearing_share(direction)
}

/// 📏 Displacement the pier top takes, the rest of the deck displacement.
pub fn pier_top_displacement_m(support: &Support, direction: Direction, deck_displacement_m: f64) -> f64 {
    if support.is_pier() { deck_displacement_m.abs() * (1.0 - support.bearing_share(direction)) } else { 0.0 }
}
