//! 🌉️ Continuous beam on pinned supports (optionally clamped at the start) as a typed convenience over [`Analysis`]: up to three spans, uniform downward span loads and downward midspan forces; the oracle wiring of the Eurocode artifacts and the load arrangement envelope of EN 1990 6.4.3.2 drive it.

use super::*;

const SEGMENTS: usize = 48;

/// 📥️ A continuous beam: span lengths in m, E in Pa, section A in m² and I in m⁴, optional clamped start, downward line loads per span in N/m and downward midspan forces per span in N.
#[derive(Clone, Debug)]
pub struct ContinuousBeam<'a> {
    pub spans: &'a [f64],
    pub e: f64,
    pub area: f64,
    pub inertia: f64,
    pub fixed_start: bool,
    pub loads: &'a [f64],
    pub mid_forces: &'a [f64],
}

/// 📤️ Support reactions (upward positive) and per-span extremes: sagging and hogging moments in N·m and the downward deflection in m.
#[derive(Clone, Debug, Default)]
pub struct BeamActions {
    pub reactions: Vec<f64>,
    pub sagging: Vec<f64>,
    pub hogging: Vec<f64>,
    pub deflection: Vec<f64>,
}

/// 🧮️ Solve the beam with `SEGMENTS` elements per span through the linear statics of [`Analysis::solve_case`].
pub fn solve_continuous_beam(beam: &ContinuousBeam<'_>) -> MechanicsResult<BeamActions> {
    let count = beam.spans.len();
    assert!(count <= 3 && beam.loads.len() == count && beam.mid_forces.len() == count, "a continuous beam has up to three spans with one load and one midspan force each");
    let mut nodes = Vec::new();
    let mut start = 0.0;
    for length in beam.spans {
        for step in 0..SEGMENTS {
            nodes.push(Node { id: format!("n{}", nodes.len()), x: start + length * step as f64 / SEGMENTS as f64, y: 0.0, z: 0.0 });
        }
        start += length;
    }
    nodes.push(Node { id: format!("n{}", nodes.len()), x: start, y: 0.0, z: 0.0 });
    let members: Vec<Member> = (0..count * SEGMENTS).map(|j| Member { id: format!("m{j}"), node_i: format!("n{j}"), node_j: format!("n{}", j + 1), material: "S".into(), section: "X".into(), ..Default::default() }).collect();
    let supports: Vec<Support> = (0..=count)
        .map(|k| Support { node: format!("n{}", k * SEGMENTS), fixed: [k == 0, true, true, k == 0, k == 0 && beam.fixed_start, false], ..Default::default() })
        .collect();
    let distributed = members.iter().enumerate().map(|(j, member)| DistributedLoad { member: member.id.clone(), frame: LoadFrame::Global, start: [0.0, 0.0, -beam.loads[j / SEGMENTS]], end: [0.0, 0.0, -beam.loads[j / SEGMENTS]] }).collect();
    let nodal = beam.mid_forces.iter().enumerate().map(|(k, force)| NodalLoad { node: format!("n{}", k * SEGMENTS + SEGMENTS / 2), force: [0.0, 0.0, -force, 0.0, 0.0, 0.0] }).collect();
    let model = Model {
        nodes,
        materials: vec![Material { id: "S".into(), e: beam.e, g: beam.e / 2.6, rho: 7850.0 }],
        sections: vec![Section { id: "X".into(), a: beam.area, iy: beam.inertia, iz: beam.inertia, j: 1.0e-6, ..Default::default() }],
        members,
        supports,
        load_cases: vec![LoadCase { id: "q".into(), nodal, distributed, ..Default::default() }],
        ..Default::default()
    };
    let control = Control::none();
    let result = Analysis::new(&model, &control)?.solve_case(&model.load_cases[0], &control)?;
    let reactions = (0..=count).map(|k| result.reactions.iter().find(|reaction| reaction.node == format!("n{}", k * SEGMENTS)).map_or(0.0, |reaction| reaction.force[2])).collect();
    let moments = |k: usize| result.members.iter().skip(k * SEGMENTS).take(SEGMENTS).flat_map(|member| member.diagram.iter().map(|point| point.my)).collect::<Vec<f64>>();
    let sagging = (0..count).map(|k| moments(k).into_iter().fold(0.0, |peak, my| f64::max(peak, -my))).collect();
    let hogging = (0..count).map(|k| moments(k).into_iter().fold(0.0, f64::max)).collect();
    let deflection = (0..count)
        .map(|k| result.displacements.iter().filter(|displacement| (k * SEGMENTS..=(k + 1) * SEGMENTS).any(|j| displacement.node == format!("n{j}"))).fold(0.0, |peak, displacement| f64::max(peak, -displacement.u[2])))
        .collect();
    Ok(BeamActions { reactions, sagging, hogging, deflection })
}

/// 🎚️ The partial factors of one arrangement: γG,sup, γG,inf and γQ.
#[derive(Clone, Copy, Debug)]
pub struct SpanFactors {
    pub gamma_g: f64,
    pub gamma_g_inf: f64,
    pub gamma_q: f64,
}

/// 📥️ The beam of a load arrangement envelope: span lengths in m (up to three), E in Pa, A in m² and I in m⁴.
#[derive(Clone, Copy, Debug)]
pub struct SpanBeam<'a> {
    pub spans: &'a [f64],
    pub e: f64,
    pub area: f64,
    pub inertia: f64,
}

/// 📤️ The envelope: peak sagging moment per span, peak hogging moment, and the largest and smallest reaction per support (N, upward positive).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpanEnvelope {
    pub sagging: Vec<f64>,
    pub hogging: f64,
    pub reaction_max: Vec<f64>,
    pub reaction_min: Vec<f64>,
}

/// 🪜️ Every span subset `0..2ⁿ` as a flag per span (bit `k` = span `k`): the arrangements of a variable action, or of the unfavourable permanent parts.
pub fn span_subsets(count: usize) -> impl Iterator<Item = Vec<bool>> {
    (0..1usize << count).map(move |pattern| (0..count).map(|span| pattern >> span & 1 == 1).collect())
}

/// 🪜️ The permanent factor per span of every arrangement of EN 1990 A1.3.1: one permanent action is individual per span and takes γG,sup where it is unfavourable and γG,inf on the other spans (2ⁿ arrangements).
pub fn permanent_arrangements(count: usize, factors: &SpanFactors) -> Vec<Vec<f64>> {
    span_subsets(count).map(|unfavourable| unfavourable.iter().map(|sup| if *sup { factors.gamma_g } else { factors.gamma_g_inf }).collect()).collect()
}

/// 🧮️ The envelope of `g` and `q` (uniform downward line loads in N/m) over the 2ⁿ permanent arrangements ([`permanent_arrangements`]) times the 2ⁿ variable arrangements (γQ·q on a subset of spans, EN 1990 6.4.3.2), each solved by [`solve_continuous_beam`].
pub fn continuous_beam_envelope(beam: &SpanBeam<'_>, g: f64, q: f64, factors: &SpanFactors) -> Result<SpanEnvelope, String> {
    let count = beam.spans.len();
    let mut envelope = SpanEnvelope { sagging: vec![0.0; count], hogging: 0.0, reaction_max: vec![f64::NEG_INFINITY; count + 1], reaction_min: vec![f64::INFINITY; count + 1] };
    let unloaded = vec![0.0; count];
    for permanent in permanent_arrangements(count, factors) {
        for variable in span_subsets(count) {
            let loads: Vec<f64> = (0..count).map(|span| permanent[span] * g + if variable[span] { factors.gamma_q * q } else { 0.0 }).collect();
            let actions = solve_continuous_beam(&ContinuousBeam { spans: beam.spans, e: beam.e, area: beam.area, inertia: beam.inertia, fixed_start: false, loads: &loads, mid_forces: &unloaded }).map_err(|error| format!("{error:?}"))?;
            for span in 0..count {
                envelope.sagging[span] = envelope.sagging[span].max(actions.sagging[span]);
                envelope.hogging = envelope.hogging.max(actions.hogging[span]);
            }
            for (support, reaction) in actions.reactions.iter().enumerate() {
                envelope.reaction_max[support] = envelope.reaction_max[support].max(*reaction);
                envelope.reaction_min[support] = envelope.reaction_min[support].min(*reaction);
            }
        }
    }
    Ok(envelope)
}
