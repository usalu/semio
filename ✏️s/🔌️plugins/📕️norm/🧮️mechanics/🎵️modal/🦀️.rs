//! 🎵️ Modal analysis: `K φ = ω² M φ` with consistent or lumped mass, mass-normalised participation and effective modal masses.

use crate::buckling::{solve_pairs, unit_shape};
use crate::elements::{to_global, zero};
use crate::*;

impl Analysis<'_> {
    /// 🎶️ The `count` lowest modes with frequencies, shapes (max translation 1), participation factors and effective modal masses in global X, Y, Z.
    pub fn modal(&self, count: usize, kind: MassType, control: &Control<'_>) -> MechanicsResult<ModalResult> {
        let sys = &self.system;
        let model = self.model;
        let mut m = sys.assemble(control, "mass", |i| if model.self_mass { sys.elements[i].mass(kind) } else { zero() })?;
        let mut total = [0.0; 3];
        if model.self_mass {
            for el in &sys.elements {
                (0..3).for_each(|k| total[k] += el.props.rho * el.props.a * el.l);
            }
        }
        for nm in &model.masses {
            let Some(&n) = sys.node_index.get(&nm.node) else {
                return Err(MechanicsError::invalid(format!("mass references unknown node {}", nm.node), format!("Masse verweist auf unbekannten Knoten {}", nm.node)));
            };
            for k in 0..6 {
                let d = 6 * sys.pos[n] + k;
                if k < 3 {
                    total[k] += nm.mass[k];
                }
                if sys.fmap[d] != usize::MAX {
                    m.add(sys.fmap[d], sys.fmap[d], nm.mass[k]);
                }
            }
        }
        if total.iter().all(|t| *t <= 0.0) {
            return Err(MechanicsError::invalid("model has no translational mass", "Modell hat keine translatorische Masse"));
        }
        let pairs = solve_pairs(self, &m, count.max(1), control)?;
        let mut modes = Vec::new();
        for (lambda, x) in pairs {
            let full = sys.expand(&x);
            let mut participation = [0.0; 3];
            if model.self_mass {
                for (mi, el) in sys.elements.iter().enumerate() {
                    let g = to_global(&el.r, &el.mass(kind));
                    let d = sys.dofs(mi);
                    for k in 0..3 {
                        for a in (0..12).filter(|a| a % 6 == k) {
                            participation[k] += (0..12).map(|b| full[d[b]] * g[b][a]).sum::<f64>();
                        }
                    }
                }
            }
            for nm in &model.masses {
                let n = sys.node_index[&nm.node];
                (0..3).for_each(|k| participation[k] += full[6 * sys.pos[n] + k] * nm.mass[k]);
            }
            let effective_mass = participation.map(|g| g * g);
            let omega = lambda.sqrt();
            modes.push(ModalMode {
                omega,
                frequency_hz: omega / std::f64::consts::TAU,
                participation,
                effective_mass,
                effective_mass_ratio: std::array::from_fn(|k| if total[k] > 0.0 { effective_mass[k] / total[k] } else { 0.0 }),
                shape: unit_shape(self, &x),
            });
        }
        Ok(ModalResult { total_mass: total, modes })
    }
}
