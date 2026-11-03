//! 🎵️ Modes: the first vertical vibration mode of a timber member from the shared mechanics kernel ([`mechanics`]): a Euler-Bernoulli chain of sixteen elements with the drawn supports (pinned and roller, fixed base, or two spans over three supports), bending in the vertical plane only, consistent mass. The frequency and the shape are eigen-solutions, not sines.

extern crate semio_s_artifact_norm_en1995_model as model;
extern crate semio_s_plugin_norm_mechanics as mechanics;

use mechanics::{Analysis, Control, MassType, Material, Member, Model, Node, Section, Support};
use model::SupportType;

const ELEMENTS: usize = 16;

/// 🎵️ The first mode: its frequency in Hz and the vertical displacement (largest 1, signed) at the ascending member coordinates `t = x / L`.
#[derive(Clone, Debug, PartialEq)]
pub struct Vibration {
    pub frequency: f64,
    pub shape: Vec<(f64, f64)>,
}

fn held(index: usize, fixed: [bool; 6]) -> Support {
    Support { node: format!("n{index}"), fixed, ..Default::default() }
}

/// 🎵️ The first vertical mode of a member of length `span` and bending stiffness `ei` (N m²) carrying the linear mass `mass` (kg/m) on `support`; `None` when the kernel rejects the model. For `ContinuousTwoSpan` the member is two spans of `span`.
pub fn first_vertical(support: SupportType, span: f64, ei: f64, mass: f64) -> Option<Vibration> {
    if span <= 0.0 || ei <= 0.0 || mass <= 0.0 {
        return None;
    }
    let length = if support == SupportType::ContinuousTwoSpan { 2.0 * span } else { span };
    let (area, e) = (1.0, 1.0e10);
    let planar = [false, true, false, true, false, true];
    let supports: Vec<Support> = (0..=ELEMENTS)
        .map(|i| {
            let mut fixed = planar;
            match support {
                SupportType::SimplySupported => {
                    fixed[0] = i == 0;
                    fixed[2] = i == 0 || i == ELEMENTS;
                }
                SupportType::Cantilever => {
                    if i == 0 {
                        fixed = [true; 6];
                    }
                }
                SupportType::ContinuousTwoSpan => {
                    fixed[0] = i == 0;
                    fixed[2] = i % (ELEMENTS / 2) == 0;
                }
            }
            held(i, fixed)
        })
        .collect();
    let model = Model {
        nodes: (0..=ELEMENTS).map(|i| Node { id: format!("n{i}"), x: length * i as f64 / ELEMENTS as f64, y: 0.0, z: 0.0 }).collect(),
        materials: vec![Material { id: "t".into(), e, g: e / 16.0, rho: mass / area }],
        sections: vec![Section { id: "s".into(), a: area, iy: ei / e, iz: ei / e, j: ei / e, iw: 0.0, asy: 0.0, asz: 0.0 }],
        members: (0..ELEMENTS).map(|i| Member { id: format!("e{i}"), node_i: format!("n{i}"), node_j: format!("n{}", i + 1), material: "t".into(), section: "s".into(), ..Default::default() }).collect(),
        supports,
        self_mass: true,
        ..Default::default()
    };
    let control = Control::none();
    let analysis = Analysis::new(&model, &control).ok()?;
    let mode = analysis.modal(1, MassType::Consistent, &control).ok()?.modes.into_iter().next()?;
    let uz = |i: usize| mode.shape.iter().find(|d| d.node == format!("n{i}")).map_or(0.0, |d| d.u[2]);
    let top = (0..=ELEMENTS).map(|i| uz(i).abs()).fold(0.0, f64::max);
    (top > 0.0).then(|| {
        let sign = (0..=ELEMENTS).map(uz).max_by(|a, b| a.abs().total_cmp(&b.abs())).map_or(1.0, f64::signum);
        Vibration { frequency: mode.frequency_hz, shape: (0..=ELEMENTS).map(|i| (i as f64 / ELEMENTS as f64, sign * uz(i) / top)).collect() }
    })
}
