//! 🗼️ Modal response of a tower or chimney on a flexible foundation (EN 1998-6): the uniform cantilever with a rotational spring at the base solved by the mechanics kernel, effective modal masses, the modal inertia forces of the design spectrum, CQC-combined base shear and overturning moment, and the period lengthening of the foundation flexibility. Pure functions of numbers; the checks and the scene call them so picture and report cannot disagree. See [EN 1998-6](https://eur-lex.europa.eu/).

use crate::part_1;
use semio_s_plugin_norm_mechanics::{Analysis, Control, MassType, Material, Member as BeamMember, Model, NodalMass, Node, Section, Support as BeamSupport};

pub const ELEMENTS: usize = 24;
pub const MODES: usize = 6;
pub const DAMPING: f64 = 0.05;

/// 🎵️ One mode of the tower: period, unit-tip shape at the nodes, participation factor of the unit-tip shape and effective mass share.
#[derive(Clone, Debug, PartialEq)]
pub struct Mode {
    pub period_s: f64,
    pub shape: Vec<f64>,
    pub gamma: f64,
    pub mass_ratio: f64,
}

/// 🎵️ The modes of a tower of height `H` and total mass `M` distributed uniformly, with the nodes' elevations `z`.
#[derive(Clone, Debug, PartialEq)]
pub struct Modes {
    pub elevations_m: Vec<f64>,
    pub node_masses_kg: Vec<f64>,
    pub modes: Vec<Mode>,
}

impl Modes {
    pub fn participation(&self) -> f64 {
        self.modes.iter().map(|m| m.mass_ratio).sum()
    }

    /// 📈 Displacement of the mode at the height ratio `x/H` by linear interpolation of the nodes, for the unit-tip shape.
    pub fn shape_at(&self, mode: usize, ratio: f64) -> f64 {
        let shape = &self.modes[mode].shape;
        let t = ratio.clamp(0.0, 1.0) * (shape.len() - 1) as f64;
        let i = (t.floor() as usize).min(shape.len() - 2);
        shape[i] + (shape[i + 1] - shape[i]) * (t - i as f64)
    }
}

/// 🧮️ The first modes of the uniform cantilever of stiffness `EI` and total mass `M` on the rotational base spring `k_θ` (`0` = fixed base), solved by the mechanics kernel with lumped masses; `None` when an input is not positive or the kernel rejects the model.
pub fn modes(mass_kg: f64, height_m: f64, ei_nm2: f64, base_rotational_nm_per_rad: f64) -> Option<Modes> {
    if mass_kg <= 0.0 || height_m <= 0.0 || ei_nm2 <= 0.0 {
        return None;
    }
    let h = height_m / ELEMENTS as f64;
    let lumps: Vec<f64> = (0..=ELEMENTS).map(|i| mass_kg / ELEMENTS as f64 * if i == 0 || i == ELEMENTS { 0.5 } else { 1.0 }).collect();
    let e = 1.0e10;
    let id = |i: usize| format!("n{i}");
    let model = Model {
        nodes: (0..=ELEMENTS).map(|i| Node { id: id(i), x: 0.0, y: 0.0, z: h * i as f64 }).collect(),
        materials: vec![Material { id: "tower".into(), e, g: e / 2.6, rho: 0.0 }],
        sections: vec![Section { id: "tower".into(), a: 1.0, iy: ei_nm2 / e, iz: ei_nm2 / e, j: ei_nm2 / e, iw: 0.0, asy: 0.0, asz: 0.0 }],
        members: (0..ELEMENTS).map(|i| BeamMember { id: format!("e{i}"), node_i: id(i), node_j: id(i + 1), material: "tower".into(), section: "tower".into(), ..Default::default() }).collect(),
        supports: (0..=ELEMENTS)
            .map(|i| {
                let mut fixed = [i == 0, true, true, true, false, true];
                let mut spring = [0.0; 6];
                if i == 0 {
                    if base_rotational_nm_per_rad > 0.0 { spring[4] = base_rotational_nm_per_rad } else { fixed[4] = true }
                }
                BeamSupport { node: id(i), fixed, spring, ..Default::default() }
            })
            .collect(),
        masses: lumps.iter().enumerate().map(|(i, m)| NodalMass { node: id(i), mass: [*m, *m, *m, 0.0, 0.0, 0.0] }).collect(),
        ..Default::default()
    };
    let control = Control::none();
    let analysis = Analysis::new(&model, &control).ok()?;
    let result = analysis.modal(MODES, MassType::Lumped, &control).ok()?;
    let total: f64 = lumps.iter().sum();
    let mut out = Vec::new();
    for mode in &result.modes {
        let psi: Vec<f64> = (0..=ELEMENTS).map(|i| mode.shape.iter().find(|d| d.node == id(i)).map_or(0.0, |d| d.u[0])).collect();
        let linear: f64 = psi.iter().zip(&lumps).map(|(p, m)| p * m).sum();
        let quadratic: f64 = psi.iter().zip(&lumps).map(|(p, m)| p * p * m).sum();
        let tip = psi[ELEMENTS];
        if quadratic <= 0.0 || tip == 0.0 || mode.frequency_hz <= 0.0 {
            continue;
        }
        let shape: Vec<f64> = psi.iter().map(|p| p / tip).collect();
        let l_tip: f64 = shape.iter().zip(&lumps).map(|(p, m)| p * m).sum();
        let m_tip: f64 = shape.iter().zip(&lumps).map(|(p, m)| p * p * m).sum();
        out.push(Mode { period_s: 1.0 / mode.frequency_hz, gamma: l_tip / m_tip, mass_ratio: linear * linear / quadratic / total, shape });
    }
    (!out.is_empty()).then(|| Modes { elevations_m: (0..=ELEMENTS).map(|i| h * i as f64).collect(), node_masses_kg: lumps, modes: out })
}

/// 🌊️ The combined response of the design spectrum `s_d` (m/s², function of the period): base shear, overturning moment about the base and the modal inertia forces at the nodes.
#[derive(Clone, Debug, PartialEq)]
pub struct Response {
    pub base_shear_n: f64,
    pub overturning_nm: f64,
    pub modal_shear_n: Vec<f64>,
    pub modal_moment_nm: Vec<f64>,
    pub modal_forces_n: Vec<Vec<f64>>,
}

/// 🌊️ Modal inertia forces `F_i = m_i Γ_n φ_i S_d(T_n)` of every mode, their base shears and moments, combined by CQC.
pub fn respond(modes: &Modes, s_d: &dyn Fn(f64) -> f64) -> Response {
    let periods: Vec<f64> = modes.modes.iter().map(|m| m.period_s).collect();
    let forces: Vec<Vec<f64>> = modes.modes.iter().map(|m| modes.node_masses_kg.iter().zip(&m.shape).map(|(mass, phi)| mass * m.gamma * phi * s_d(m.period_s)).collect()).collect();
    let shear: Vec<f64> = forces.iter().map(|f| f.iter().sum::<f64>()).collect();
    let moment: Vec<f64> = forces.iter().map(|f| f.iter().zip(&modes.elevations_m).map(|(force, z)| force * z).sum::<f64>()).collect();
    Response { base_shear_n: part_1::cqc_combine(&shear, &periods, DAMPING), overturning_nm: part_1::cqc_combine(&moment, &periods, DAMPING), modal_shear_n: shear, modal_moment_nm: moment, modal_forces_n: forces }
}

/// 📈 Period lengthening `T₁(k_θ) / T₁(fixed)` of the foundation flexibility; `None` when a model fails.
pub fn period_lengthening(mass_kg: f64, height_m: f64, ei_nm2: f64, base_rotational_nm_per_rad: f64) -> Option<f64> {
    let flexible = modes(mass_kg, height_m, ei_nm2, base_rotational_nm_per_rad)?.modes.first()?.period_s;
    let fixed = modes(mass_kg, height_m, ei_nm2, 0.0)?.modes.first()?.period_s;
    (fixed > 0.0).then_some(flexible / fixed)
}

/// 🔩 Smallest base rotational stiffness for which the period lengthening stays at or below `limit` (bisection on the monotone ratio); `None` when even a stiff spring does not reach it.
pub fn stiffness_for_lengthening(mass_kg: f64, height_m: f64, ei_nm2: f64, limit: f64) -> Option<f64> {
    let ratio = |k: f64| period_lengthening(mass_kg, height_m, ei_nm2, k);
    let (mut low, mut high) = (ei_nm2 / height_m * 1.0e-3, ei_nm2 / height_m * 1.0e3);
    if ratio(high)? > limit {
        return None;
    }
    for _ in 0..60 {
        let mid = (low * high).sqrt();
        if ratio(mid)? > limit { low = mid } else { high = mid }
    }
    Some(high)
}

/// 📊 Level profiles of the CQC-combined response: the storey shear `V_k = Σ_{j≥k} F_j` and the design displacement `q·d_e` with `d_e = F / (m ω²)` per node, each combined over the modes (EN 1998-1, 4.3.3.3 and 4.3.4).
#[derive(Clone, Debug, PartialEq)]
pub struct LevelProfile {
    pub shear_n: Vec<f64>,
    pub displacement_m: Vec<f64>,
    pub drift_m: Vec<f64>,
}

pub fn level_profile(modes: &Modes, response: &Response, q: f64) -> LevelProfile {
    let periods: Vec<f64> = modes.modes.iter().map(|m| m.period_s).collect();
    let nodes = modes.elevations_m.len();
    let displacement = |mode: usize, node: usize| {
        let omega = 2.0 * std::f64::consts::PI / periods[mode].max(1e-9);
        let mass = modes.node_masses_kg[node];
        if mass > 0.0 { q.max(1.0) * response.modal_forces_n[mode][node] / (mass * omega * omega) } else { 0.0 }
    };
    let combine = |value: &dyn Fn(usize) -> f64| part_1::cqc_combine(&(0..periods.len()).map(value).collect::<Vec<_>>(), &periods, DAMPING);
    LevelProfile {
        shear_n: (0..nodes).map(|k| combine(&|m| response.modal_forces_n[m][k..].iter().sum::<f64>())).collect(),
        displacement_m: (0..nodes).map(|k| combine(&|m| displacement(m, k))).collect(),
        drift_m: (0..nodes).map(|k| if k == 0 { combine(&|m| displacement(m, 0)) } else { combine(&|m| displacement(m, k) - displacement(m, k - 1)) }).collect(),
    }
}
