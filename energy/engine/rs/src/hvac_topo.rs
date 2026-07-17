//! 🌀 HVAC fluid topology: nodes, branches, splitters, mixers, and loop validation.

use crate::error::{Diagnostics, Error, Severity};
use serde::{Deserialize, Serialize};

// #region 🔖FluidNode
/// 💧 Fluid stream state at a topology node (air or water).
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct FluidNode {
    pub id: usize,
    pub temperature_c: f64,
    pub humidity_ratio: f64,
    pub pressure_pa: f64,
    pub mass_flow_kg_s: f64,
}

impl FluidNode {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            temperature_c: 20.0,
            humidity_ratio: 0.008,
            pressure_pa: 101_325.0,
            mass_flow_kg_s: 0.0,
        }
    }
}
// #endregion 🔖FluidNode

// #region 🔖Branch
/// 🔀 Directed fluid branch between two nodes.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Branch {
    pub id: usize,
    pub inlet: usize,
    pub outlet: usize,
    pub component: BranchComponent,
}

/// ⚙️ Branch-resident component type.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum BranchComponent {
    Duct { hydraulic_diameter_m: f64, length_m: f64 },
    Pipe { diameter_m: f64, length_m: f64 },
    Pump { design_head_pa: f64, design_flow_kg_s: f64 },
    Coil { ua_w_per_k: f64 },
    Valve { cv: f64 },
    Bypass,
}

impl Branch {
    pub fn pressure_drop_pa(&self, inlet: &FluidNode, outlet: &FluidNode) -> f64 {
        match &self.component {
            BranchComponent::Duct { hydraulic_diameter_m, length_m } => {
                let rho = 1.2;
                let area = std::f64::consts::PI * hydraulic_diameter_m * hydraulic_diameter_m / 4.0;
                let v = inlet.mass_flow_kg_s.abs() / (rho * area).max(1e-6);
                0.02 * (length_m / hydraulic_diameter_m.max(0.01)) * 0.5 * rho * v * v
            }
            BranchComponent::Pipe { diameter_m, length_m } => {
                let rho = 998.0;
                let area = std::f64::consts::PI * diameter_m * diameter_m / 4.0;
                let v = inlet.mass_flow_kg_s.abs() / (rho * area).max(1e-6);
                0.02 * (length_m / diameter_m.max(0.01)) * 0.5 * rho * v * v
            }
            BranchComponent::Pump { design_head_pa, design_flow_kg_s } => {
                let frac = (inlet.mass_flow_kg_s / design_flow_kg_s.max(1e-6)).clamp(0.0, 1.2);
                -design_head_pa * (1.0 - 0.3 * (1.0 - frac).powi(2))
            }
            BranchComponent::Coil { ua_w_per_k } => {
                let delta_t = (inlet.temperature_c - outlet.temperature_c).abs();
                ua_w_per_k * delta_t / (inlet.mass_flow_kg_s.abs().max(0.01) * 1006.0)
            }
            BranchComponent::Valve { cv } => {
                let delta_p = (inlet.pressure_pa - outlet.pressure_pa).abs();
                let flow_gpm = cv * delta_p.sqrt();
                flow_gpm * 0.063_09
            }
            BranchComponent::Bypass => 5.0,
        }
    }
}
// #endregion 🔖Branch

// #region 🔖SplitterMixer
/// 🔱 Flow splitter: one inlet, multiple outlets with prescribed fractions.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Splitter {
    pub id: usize,
    pub inlet: usize,
    pub outlets: Vec<(usize, f64)>,
}

impl Splitter {
    pub fn distribute(&self, inlet: &FluidNode) -> Vec<FluidNode> {
        self.outlets
            .iter()
            .map(|(id, frac)| FluidNode {
                id: *id,
                temperature_c: inlet.temperature_c,
                humidity_ratio: inlet.humidity_ratio,
                pressure_pa: inlet.pressure_pa,
                mass_flow_kg_s: inlet.mass_flow_kg_s * frac,
            })
            .collect()
    }
}

/// 🔀 Flow mixer: multiple inlets blended by mass flow.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Mixer {
    pub id: usize,
    pub inlets: Vec<usize>,
    pub outlet: usize,
}

impl Mixer {
    pub fn blend(&self, nodes: &[FluidNode]) -> FluidNode {
        let inlets: Vec<_> = self.inlets.iter().map(|&id| &nodes[id]).collect();
        let m_total: f64 = inlets.iter().map(|n| n.mass_flow_kg_s).sum();
        if m_total < 1e-9 {
            return FluidNode::new(self.outlet);
        }
        let t = inlets.iter().map(|n| n.temperature_c * n.mass_flow_kg_s).sum::<f64>() / m_total;
        let w = inlets.iter().map(|n| n.humidity_ratio * n.mass_flow_kg_s).sum::<f64>() / m_total;
        let p = inlets.iter().map(|n| n.pressure_pa * n.mass_flow_kg_s).sum::<f64>() / m_total;
        FluidNode {
            id: self.outlet,
            temperature_c: t,
            humidity_ratio: w,
            pressure_pa: p,
            mass_flow_kg_s: m_total,
        }
    }
}
// #endregion 🔖SplitterMixer

// #region 🔖Loops
/// 🌬️ Air loop topology: supply/return paths with zone connections.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct AirLoop {
    pub id: usize,
    pub name: String,
    pub nodes: Vec<FluidNode>,
    pub branches: Vec<Branch>,
    pub splitters: Vec<Splitter>,
    pub mixers: Vec<Mixer>,
    pub supply_inlet: usize,
    pub supply_outlet: usize,
    pub return_inlet: usize,
    pub return_outlet: usize,
    pub zone_outlets: Vec<usize>,
    pub zone_returns: Vec<usize>,
}

/// 🏭 Plant loop topology: hot/cold water or steam distribution.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlantLoop {
    pub id: usize,
    pub name: String,
    pub fluid: PlantFluid,
    pub nodes: Vec<FluidNode>,
    pub branches: Vec<Branch>,
    pub splitters: Vec<Splitter>,
    pub mixers: Vec<Mixer>,
    pub supply_inlet: usize,
    pub supply_outlet: usize,
    pub demand_inlet: usize,
    pub demand_outlet: usize,
}

/// 💧 Plant loop working fluid.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlantFluid {
    Water,
    Steam,
    CondenserWater,
    Glycol { fraction: f64 },
}

/// ❄️ Condenser loop for heat rejection (cooling tower / dry cooler).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CondenserLoop {
    pub id: usize,
    pub plant_loop: PlantLoop,
    pub heat_rejection_w: f64,
}
// #endregion 🔖Loops

// #region 🔖Validation
/// ✅ Validate HVAC topology mass balance and connectivity.
pub fn validate_topology(
    nodes: &[FluidNode],
    branches: &[Branch],
    splitters: &[Splitter],
    mixers: &[Mixer],
) -> Diagnostics {
    let mut diag = Diagnostics::default();
    let n = nodes.len();

    for branch in branches {
        if branch.inlet >= n || branch.outlet >= n {
            diag.push(
                Error::fatal(format!("branch {} references invalid node", branch.id))
                    .with_context("hvac_topo"),
            );
        }
        if branch.inlet == branch.outlet {
            diag.push(
                Error::severe(format!("branch {} has identical inlet/outlet", branch.id))
                    .with_context("hvac_topo"),
            );
        }
    }

    for splitter in splitters {
        let frac_sum: f64 = splitter.outlets.iter().map(|(_, f)| f).sum();
        if (frac_sum - 1.0).abs() > 0.01 {
            diag.push(
                Error::warning(format!(
                    "splitter {} outlet fractions sum to {:.3}, expected 1.0",
                    splitter.id, frac_sum
                ))
                .with_context("hvac_topo"),
            );
        }
        if splitter.inlet >= n {
            diag.push(
                Error::fatal(format!("splitter {} invalid inlet", splitter.id))
                    .with_context("hvac_topo"),
            );
        }
    }

    for mixer in mixers {
        for &inlet in &mixer.inlets {
            if inlet >= n {
                diag.push(
                    Error::fatal(format!("mixer {} invalid inlet {}", mixer.id, inlet))
                        .with_context("hvac_topo"),
                );
            }
        }
        if mixer.outlet >= n {
            diag.push(
                Error::fatal(format!("mixer {} invalid outlet", mixer.id))
                    .with_context("hvac_topo"),
            );
        }
    }

    let mut net_flow = vec![0.0_f64; n];
    for branch in branches {
        if branch.inlet < n && branch.outlet < n {
            let m = nodes[branch.inlet].mass_flow_kg_s;
            net_flow[branch.inlet] -= m;
            net_flow[branch.outlet] += m;
        }
    }

    for (i, &nf) in net_flow.iter().enumerate() {
        if nf.abs() > 0.1 && i < n {
            diag.push(
                Error::warning(format!("node {} mass imbalance {:.4} kg/s", i, nf))
                    .with_context("hvac_topo"),
            );
        }
    }

    diag
}

impl AirLoop {
    pub fn validate(&self) -> Diagnostics {
        let mut diag = validate_topology(&self.nodes, &self.branches, &self.splitters, &self.mixers);
        for &z in &self.zone_outlets {
            if z >= self.nodes.len() {
                diag.push(
                    Error::severe(format!("air loop {} zone outlet {} invalid", self.id, z))
                        .with_context("air_loop"),
                );
            }
        }
        diag
    }
}

impl PlantLoop {
    pub fn validate(&self) -> Diagnostics {
        validate_topology(&self.nodes, &self.branches, &self.splitters, &self.mixers)
    }
}
// #endregion 🔖Validation

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mixer_blends_by_mass_flow() {
        let nodes = vec![
            FluidNode { id: 0, temperature_c: 10.0, humidity_ratio: 0.005, pressure_pa: 101_325.0, mass_flow_kg_s: 1.0 },
            FluidNode { id: 1, temperature_c: 30.0, humidity_ratio: 0.015, pressure_pa: 101_325.0, mass_flow_kg_s: 1.0 },
        ];
        let mixer = Mixer { id: 0, inlets: vec![0, 1], outlet: 2 };
        let out = mixer.blend(&nodes);
        assert!((out.temperature_c - 20.0).abs() < 1e-9);
        assert!((out.mass_flow_kg_s - 2.0).abs() < 1e-9);
    }

    #[test]
    fn splitter_preserves_mass() {
        let inlet = FluidNode { id: 0, temperature_c: 20.0, humidity_ratio: 0.01, pressure_pa: 101_325.0, mass_flow_kg_s: 2.0 };
        let splitter = Splitter {
            id: 0,
            inlet: 0,
            outlets: vec![(1, 0.6), (2, 0.4)],
        };
        let outs = splitter.distribute(&inlet);
        let m_sum: f64 = outs.iter().map(|n| n.mass_flow_kg_s).sum();
        assert!((m_sum - 2.0).abs() < 1e-9);
    }

    #[test]
    fn valid_topology_passes() {
        let nodes = vec![
            FluidNode::new(0),
            FluidNode::new(1),
        ];
        let branches = vec![Branch {
            id: 0,
            inlet: 0,
            outlet: 1,
            component: BranchComponent::Bypass,
        }];
        let diag = validate_topology(&nodes, &branches, &[], &[]);
        assert!(!diag.has_fatal());
    }
}
