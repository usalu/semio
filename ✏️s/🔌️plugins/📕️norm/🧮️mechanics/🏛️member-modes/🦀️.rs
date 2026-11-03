//! 🏛️ Member modes: the buckling load and shape and the deflection line of one prismatic member in its own frame (local x along the axis, y and z the section axes), built on the frame kernel with eight Hermite elements. The scene crates of the norm artifacts map the local displacements to the world.

use crate::*;

const SEGMENTS: usize = 8;
const MEMBER: &str = "m";

/// 📐️ Section and material properties of the member (SI).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Props {
    pub length: f64,
    pub e: f64,
    pub g: f64,
    pub a: f64,
    pub iy: f64,
    pub iz: f64,
    pub j: f64,
}

/// 🔩️ End conditions: `Pinned` (pin and roller), `Cantilever` (fixed at x = 0, free tip) or `Fixed` (fixed at x = 0, fixed in rotation and lateral translation at x = L, axially free). Torsion is held at both ends.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ends {
    Pinned,
    Cantilever,
    Fixed,
}

/// 🏋️ Loads of the deflection line, in the local frame (z up): a downward uniform load per metre, a downward point load at `x` and end couples about the local y axis (positive sagging at x = 0, `-` at x = L).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Loads {
    pub udl: f64,
    pub point: (f64, f64),
    pub couples: (f64, f64),
}

/// 🌀️ A shape sampled at ascending `t = x / L`: the local displacement `[ux, uy, uz]` of each sample.
pub type Curve = Vec<(f64, [f64; 3])>;

fn model(p: &Props, ends: Ends, case: LoadCase) -> Model {
    let held = |id: &str, fixed: [bool; 6]| Support { node: id.into(), fixed, ..Default::default() };
    let (a, b) = match ends {
        Ends::Pinned => (held("a", [true, true, true, true, false, false]), Some(held("b", [false, true, true, true, false, false]))),
        Ends::Cantilever => (held("a", [true; 6]), None),
        Ends::Fixed => (held("a", [true; 6]), Some(held("b", [false, true, true, true, true, true]))),
    };
    Model {
        nodes: vec![Node { id: "a".into(), x: 0.0, y: 0.0, z: 0.0 }, Node { id: "b".into(), x: p.length, y: 0.0, z: 0.0 }],
        materials: vec![Material { id: "s".into(), e: p.e, g: p.g, rho: 0.0 }],
        sections: vec![Section { id: "p".into(), a: p.a, iy: p.iy, iz: p.iz, j: p.j, iw: 0.0, asy: 0.0, asz: 0.0 }],
        members: vec![Member { id: MEMBER.into(), node_i: "a".into(), node_j: "b".into(), material: "s".into(), section: "p".into(), ..Default::default() }],
        supports: std::iter::once(a).chain(b).collect(),
        load_cases: vec![case],
        ..Default::default()
    }
    .refine(SEGMENTS)
}

fn curve(analysis: &Analysis<'_>, shape: &[NodeDisplacement]) -> MechanicsResult<Curve> {
    let mut points = Vec::new();
    for k in 1..=SEGMENTS {
        let part = analysis.member_curve(shape, &format!("{MEMBER}#{k}"), 3)?;
        for (j, u) in part.into_iter().enumerate().skip(usize::from(k > 1)) {
            points.push((((k - 1) as f64 + j as f64 / 2.0) / SEGMENTS as f64, u));
        }
    }
    Ok(points)
}

/// 🌀️ Flexural buckling under a unit compression at x = L: the Euler load `Ncr` in N and the first mode with the largest translation scaled to 1.
pub fn buckling(p: &Props, ends: Ends) -> MechanicsResult<(f64, Curve)> {
    let case = LoadCase { id: "unit".into(), nodal: vec![NodalLoad { node: "b".into(), force: [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0] }], ..Default::default() };
    let refined = model(p, ends, case);
    let control = Control::none();
    let analysis = Analysis::new(&refined, &control)?;
    let mode = analysis.buckling(&refined.load_cases[0], 1, &control)?.modes.into_iter().next().ok_or(MechanicsError::NoPositiveEigenvalue)?;
    let mut shape = curve(&analysis, &mode.shape)?;
    let top = shape.iter().flat_map(|(_, u)| u.iter()).fold(0.0_f64, |m, v| m.max(v.abs())).max(1e-300);
    shape.iter_mut().for_each(|(_, u)| u.iter_mut().for_each(|v| *v /= top));
    Ok((mode.alpha_cr, shape))
}

/// 📐️ Deflection line of the member under `loads` with the true displacements in metres.
pub fn deflection(p: &Props, ends: Ends, loads: &Loads) -> MechanicsResult<Curve> {
    let l = p.length;
    let mut case = LoadCase { id: "static".into(), ..Default::default() };
    if loads.udl != 0.0 {
        case.distributed = vec![DistributedLoad { member: MEMBER.into(), frame: LoadFrame::Local, start: [0.0, 0.0, -loads.udl], end: [0.0, 0.0, -loads.udl] }];
    }
    let mut model = model(p, ends, case);
    if loads.point.1 != 0.0 {
        let k = ((loads.point.0 / l * SEGMENTS as f64).round() as usize).min(SEGMENTS);
        let node = match k {
            0 => "a".to_string(),
            k if k == SEGMENTS => "b".to_string(),
            k => format!("{MEMBER}@{k}"),
        };
        model.load_cases[0].nodal.push(NodalLoad { node, force: [0.0, 0.0, -loads.point.1, 0.0, 0.0, 0.0] });
    }
    if loads.couples != (0.0, 0.0) {
        model.load_cases[0].nodal.push(NodalLoad { node: "a".into(), force: [0.0, 0.0, 0.0, 0.0, loads.couples.0, 0.0] });
        model.load_cases[0].nodal.push(NodalLoad { node: "b".into(), force: [0.0, 0.0, 0.0, 0.0, -loads.couples.1, 0.0] });
    }
    let control = Control::none();
    let analysis = Analysis::new(&model, &control)?;
    let result = analysis.solve_case(&model.load_cases[0], &control)?;
    curve(&analysis, &result.displacements)
}
