//! 🕸️ General planar frame of an EN 1993 snapshot (frame nodes in the x-z plane, z up, and frame bars) solved by the shared mechanics kernel: the elastic critical load factor α_cr and the buckling modes of EN 1993-1-1 5.2.1. See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).

use semio_s_artifact_norm_en1993_model::{En1993Snapshot, FrameActionKind, FrameBar, FrameNode, SteelJoint, SteelMaterial, SteelSection};
use semio_s_plugin_norm_mechanics as mech;

/// 🔪️ Elements per bar in the buckling solution; the inner nodes are named `<bar id>@<k>`.
pub const SEGMENTS: usize = 4;
const MODES: usize = 8;
const OUT_OF_PLANE: [bool; 6] = [false, true, false, true, false, true];
const MAX_ITERATIONS: usize = 40;
const TOLERANCE: f64 = 1e-7;
const NO_FREE_DOF: usize = usize::MAX;
const SPRING_LENGTH: f64 = 0.02;
const IN_PLANE_HINGE: [bool; 6] = [false, false, false, false, true, false];

/// 📏️ A uniform line load [N/m of bar length] in the global x and z directions on the solution segments `from..to` of a bar (segment `k` spans `k/SEGMENTS` to `(k+1)/SEGMENTS` of the bar).
#[derive(Clone, Debug, PartialEq)]
pub struct LineLoad {
    pub bar: String,
    pub fx: f64,
    pub fz: f64,
    pub from: usize,
    pub to: usize,
}

/// 🏗️ The loads of one load vector of the frame: forces `[fx, fz, my]` [N, N, Nm] at nodes and line loads on bars. One vector is one load case, one EN 1990 combination or the explicit design loads of the nodes.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct FrameLoads {
    pub nodal: Vec<(String, [f64; 3])>,
    pub line: Vec<LineLoad>,
}

fn grid(ratio: f64, action: &str) -> Result<usize, FrameFault> {
    let k = ratio * SEGMENTS as f64;
    if (0.0..=SEGMENTS as f64).contains(&k) && (k - k.round()).abs() < 1e-9 {
        Ok(k.round() as usize)
    } else {
        Err(FrameFault::new(format!("Frame action '{action}' starts or ends at {ratio}, which is not a multiple of 1/{SEGMENTS} of the bar."), format!("Rahmeneinwirkung '{action}' beginnt oder endet bei {ratio}, was kein Vielfaches von 1/{SEGMENTS} der Stablänge ist.")))
    }
}

impl FrameLoads {
    /// 🎯️ The explicit design loads of the nodes (`loadX`, `loadZ`), already factored.
    pub fn design(document: &En1993Snapshot) -> Self {
        Self { nodal: document.frame_nodes.iter().filter(|n| n.load_x != 0.0 || n.load_z != 0.0).map(|n| (n.id.clone(), [n.load_x, n.load_z, 0.0])).collect(), line: Vec::new() }
    }

    /// 📋️ The characteristic loads of the `frameActions` of one load case; a fault names an unknown target or a line load off the segment grid.
    pub fn of_case(document: &En1993Snapshot, case: &str) -> Result<Self, FrameFault> {
        let mut loads = Self::default();
        for action in document.frame_actions.iter().filter(|a| a.load_case_id == case) {
            match action.kind {
                FrameActionKind::NodalForce | FrameActionKind::NodalMoment => {
                    if !document.frame_nodes.iter().any(|n| n.id == action.target_id) {
                        return Err(missing("Frame node", "Rahmenknoten", &action.target_id));
                    }
                    let force = if action.kind == FrameActionKind::NodalForce { [action.fx, action.fz, 0.0] } else { [0.0, 0.0, action.my] };
                    loads.nodal.push((action.target_id.clone(), force));
                }
                FrameActionKind::BarUniform => {
                    if !document.frame_bars.iter().any(|b| b.id == action.target_id) {
                        return Err(missing("Frame bar", "Rahmenstab", &action.target_id));
                    }
                    let (from, to) = (grid(action.start_ratio, &action.id)?, grid(action.end_ratio, &action.id)?);
                    if to <= from {
                        return Err(FrameFault::new(format!("Frame action '{}' ends before it starts.", action.id), format!("Rahmeneinwirkung '{}' endet vor ihrem Beginn.", action.id)));
                    }
                    loads.line.push(LineLoad { bar: action.target_id.clone(), fx: action.fx, fz: action.fz, from, to });
                }
            }
        }
        Ok(loads)
    }

    /// ✖️ The loads multiplied by a combination factor.
    pub fn scaled(&self, k: f64) -> Self {
        Self { nodal: self.nodal.iter().map(|(n, f)| (n.clone(), f.map(|v| v * k))).collect(), line: self.line.iter().map(|l| LineLoad { fx: l.fx * k, fz: l.fz * k, ..l.clone() }).collect() }
    }

    /// ➕️ Adds the loads of another vector.
    pub fn add(&mut self, other: &Self) {
        self.nodal.extend(other.nodal.iter().cloned());
        self.line.extend(other.line.iter().cloned());
    }

    /// 🎯️ True when no load is present.
    pub fn is_empty(&self) -> bool {
        self.nodal.is_empty() && self.line.is_empty()
    }

    /// ➡️ The horizontal load [N] carried by every node: nodal forces plus half of every line load to each end of its bar.
    pub fn horizontal_at_nodes(&self, document: &En1993Snapshot) -> Vec<(String, f64)> {
        let mut out: Vec<(String, f64)> = self.nodal.iter().map(|(n, f)| (n.clone(), f[0])).collect();
        for line in &self.line {
            let Some(bar) = document.frame_bars.iter().find(|b| b.id == line.bar) else { continue };
            let half = 0.5 * line.fx * bar_length_of(document, bar) * (line.to - line.from) as f64 / SEGMENTS as f64;
            out.push((bar.start_node_id.clone(), half));
            out.push((bar.end_node_id.clone(), half));
        }
        out
    }

    /// ⬇️ The vertical load [N] carried by every node: nodal forces plus half of every line load to each end of its bar.
    pub fn vertical_at_nodes(&self, document: &En1993Snapshot) -> Vec<(String, f64)> {
        let mut out: Vec<(String, f64)> = self.nodal.iter().map(|(n, f)| (n.clone(), f[1])).collect();
        for line in &self.line {
            let Some(bar) = document.frame_bars.iter().find(|b| b.id == line.bar) else { continue };
            let at = |id: &str| document.frame_nodes.iter().find(|n| n.id == id).map_or((0.0, 0.0), |n| (n.x, n.z));
            let (a, b) = (at(&bar.start_node_id), at(&bar.end_node_id));
            let half = 0.5 * line.fz * (b.0 - a.0).hypot(b.1 - a.1) * (line.to - line.from) as f64 / SEGMENTS as f64;
            out.push((bar.start_node_id.clone(), half));
            out.push((bar.end_node_id.clone(), half));
        }
        out
    }

    /// ➡️ The net horizontal load [N] of nodal forces and line loads.
    pub fn horizontal(&self, document: &En1993Snapshot) -> f64 {
        let nodal: f64 = self.nodal.iter().map(|(_, f)| f[0]).sum();
        let line: f64 = self.line.iter().map(|l| l.fx * document.frame_bars.iter().find(|b| b.id == l.bar).map_or(0.0, |b| bar_length_of(document, b)) * (l.to - l.from) as f64 / SEGMENTS as f64).sum();
        nodal + line
    }
}

/// 📏️ The length of a bar between its nodes [m].
pub fn bar_length_of(document: &En1993Snapshot, bar: &FrameBar) -> f64 {
    let at = |id: &str| document.frame_nodes.iter().find(|n| n.id == id).map_or((0.0, 0.0), |n| (n.x, n.z));
    let (a, b) = (at(&bar.start_node_id), at(&bar.end_node_id));
    (b.0 - a.0).hypot(b.1 - a.1)
}

fn push_nodal(case: &mut mech::LoadCase, node: &str, force: [f64; 6]) {
    match case.nodal.iter_mut().find(|l| l.node == node) {
        Some(load) => load.force.iter_mut().zip(force).for_each(|(f, v)| *f += v),
        None => case.nodal.push(mech::NodalLoad { node: node.to_string(), force }),
    }
}

/// 🚫️ Why the frame cannot be solved, in English and German.
#[derive(Clone, Debug, PartialEq)]
pub struct FrameFault {
    pub en: String,
    pub de: String,
}

impl FrameFault {
    fn new(en: impl Into<String>, de: impl Into<String>) -> Self {
        Self { en: en.into(), de: de.into() }
    }
}

/// 🌀️ One buckling mode: its load factor, whether the horizontal translation dominates and the in-plane shape of all solution nodes (`id`, `u_x`, `u_z`; largest translation 1).
#[derive(Clone, Debug, PartialEq)]
pub struct FrameMode {
    pub alpha_cr: f64,
    pub sway: bool,
    pub shape: Vec<(String, f64, f64)>,
}

/// 🕸️ The positive buckling modes of the frame, lowest first.
#[derive(Clone, Debug, PartialEq)]
pub struct FrameEigen {
    pub modes: Vec<FrameMode>,
}

impl FrameEigen {
    /// 🎯️ The lowest load factor of any mode.
    pub fn lowest(&self) -> f64 {
        self.modes.first().map_or(f64::INFINITY, |m| m.alpha_cr)
    }

    /// 🎯️ The lowest load factor of a sway mode, the α_cr of EN 1993-1-1 5.2.1.
    pub fn sway(&self) -> Option<&FrameMode> {
        self.modes.iter().find(|m| m.sway)
    }

    /// 🎯️ The sway load factor when a sway mode exists, otherwise the lowest one.
    pub fn governing(&self) -> f64 {
        self.sway().map_or_else(|| self.lowest(), |m| m.alpha_cr)
    }
}

fn support_flags(kind: &str) -> [bool; 6] {
    match kind {
        "fixed" => [true; 6],
        "pinned" => [true, true, true, true, false, true],
        "roller" => [false, true, true, true, false, true],
        _ => [false; 6],
    }
}

fn release(kind: &str) -> [bool; 6] {
    if kind == "pinned" { IN_PLANE_HINGE } else { [false; 6] }
}

/// 🔗️ The secant rotational stiffness S_j [N·m/rad] of a joint for the global analysis (EN 1993-1-8 5.1.2), `None` when the joint kind has no component result.
pub type Secant<'a> = &'a dyn Fn(&SteelJoint) -> Option<f64>;

fn no_joint_stiffness(_: &SteelJoint) -> Option<f64> {
    None
}

fn node_of<'a>(model: &'a mech::Model, id: &str) -> Result<&'a mech::Node, FrameFault> {
    model.nodes.iter().find(|n| n.id == id).ok_or_else(|| missing("Frame node", "Rahmenknoten", id))
}

fn joint_spring(document: &En1993Snapshot, bar: &FrameBar, secant: Secant<'_>) -> Result<f64, FrameFault> {
    document
        .joints
        .iter()
        .filter(|j| j.member_id == bar.member_id)
        .find_map(secant)
        .filter(|s| s.is_finite() && *s > 0.0)
        .ok_or_else(|| FrameFault::new(format!("Bar '{}' has a semi-rigid end but its member has no joint with a positive secant stiffness.", bar.id), format!("Stab '{}' hat ein verformbares Ende, aber sein Bauteil hat keinen Anschluss mit positiver Sekantensteifigkeit.", bar.id)))
}

fn connector(model: &mut mech::Model, bar: &FrameBar, at_start: bool, s_j: f64, section: &SteelSection, material: &SteelMaterial) -> Result<String, FrameFault> {
    let (own, other) = if at_start { (&bar.start_node_id, &bar.end_node_id) } else { (&bar.end_node_id, &bar.start_node_id) };
    let (a, b) = (node_of(model, own)?.clone(), node_of(model, other)?.clone());
    let length = ((b.x - a.x).powi(2) + (b.z - a.z).powi(2)).sqrt();
    let ell = SPRING_LENGTH.min(0.25 * length);
    let t = ell / length.max(1e-12);
    let tag = if at_start { "i" } else { "j" };
    let node_id = format!("{}~sj{tag}", bar.id);
    model.nodes.push(mech::Node { id: node_id.clone(), x: a.x + (b.x - a.x) * t, y: 0.0, z: a.z + (b.z - a.z) * t });
    let i = s_j * ell / material.e_modulus;
    let section_id = format!("{}~sj{tag}", bar.id);
    model.sections.push(mech::Section { id: section_id.clone(), a: section.area, iy: i, iz: i, j: i, iw: 0.0, asy: section.shear_area_y, asz: section.shear_area_z });
    let (node_i, node_j) = if at_start { (own.clone(), node_id.clone()) } else { (node_id.clone(), own.clone()) };
    model.members.push(mech::Member { id: section_id.clone(), node_i, node_j, material: material.id.clone(), section: section_id, ..mech::Member::default() });
    Ok(node_id)
}

fn missing(what: &str, what_de: &str, id: &str) -> FrameFault {
    FrameFault::new(format!("{what} '{id}' does not exist."), format!("{what_de} '{id}' existiert nicht."))
}

fn point(node: &FrameNode) -> mech::Node {
    mech::Node { id: node.id.clone(), x: node.x, y: 0.0, z: node.z }
}

fn bar_member(document: &En1993Snapshot, bar: &FrameBar, model: &mut mech::Model, secant: Secant<'_>) -> Result<(), FrameFault> {
    let steel = document.members.iter().find(|m| m.id == bar.member_id).ok_or_else(|| missing("Steel member", "Stahlbauteil", &bar.member_id))?;
    let section = document.sections.iter().find(|s| s.id == steel.section_id).ok_or_else(|| missing("Section", "Querschnitt", &steel.section_id))?;
    let material = document.materials.iter().find(|m| m.id == steel.material_id).ok_or_else(|| missing("Material", "Werkstoff", &steel.material_id))?;
    for id in [&bar.start_node_id, &bar.end_node_id] {
        if !model.nodes.iter().any(|n| &n.id == id) {
            return Err(missing("Frame node", "Rahmenknoten", id));
        }
    }
    if !model.materials.iter().any(|m| m.id == material.id) {
        model.materials.push(mech::Material { id: material.id.clone(), e: material.e_modulus, g: material.g_modulus, rho: 0.0 });
    }
    if !model.sections.iter().any(|s| s.id == section.id) {
        model.sections.push(mech::Section { id: section.id.clone(), a: section.area, iy: section.iy, iz: section.iz, j: section.it, iw: section.iw, asy: section.shear_area_y, asz: section.shear_area_z });
    }
    let (mut node_i, mut node_j) = (bar.start_node_id.clone(), bar.end_node_id.clone());
    if bar.release_start == semio_s_artifact_norm_en1993_model::FrameBarRelease::SemiRigid {
        node_i = connector(model, bar, true, joint_spring(document, bar, secant)?, section, material)?;
    }
    if bar.release_end == semio_s_artifact_norm_en1993_model::FrameBarRelease::SemiRigid {
        node_j = connector(model, bar, false, joint_spring(document, bar, secant)?, section, material)?;
    }
    model.members.push(mech::Member {
        id: bar.id.clone(),
        node_i,
        node_j,
        material: material.id.clone(),
        section: section.id.clone(),
        release_i: release(bar.release_start.as_str()),
        release_j: release(bar.release_end.as_str()),
        ..mech::Member::default()
    });
    Ok(())
}

/// 🏗️ The kernel model of the frame with the explicit nodal design loads; the out-of-plane degrees of freedom of every node are restrained so only in-plane modes appear. A bar end released as `semiRigid` becomes a rotational spring of the secant stiffness of its member's joint (a 20 mm connector element of bending stiffness `S_j l/E`), so only `model_with` with a joint stiffness can solve it.
pub fn model(document: &En1993Snapshot) -> Result<mech::Model, FrameFault> {
    model_with(document, &no_joint_stiffness, &FrameLoads::design(document))
}

/// 🏗️ The kernel model of the frame with semi-rigid bar ends from the joint stiffness `secant` (see `model`), loaded with the nodal forces of `loads`; its line loads are added to the refined model by `add_line_loads`.
pub fn model_with(document: &En1993Snapshot, secant: Secant<'_>, loads: &FrameLoads) -> Result<mech::Model, FrameFault> {
    if document.frame_nodes.is_empty() || document.frame_bars.is_empty() {
        return Err(FrameFault::new("The frame needs at least two nodes and one bar.", "Der Rahmen braucht mindestens zwei Knoten und einen Stab."));
    }
    let mut model = mech::Model { nodes: document.frame_nodes.iter().map(point).collect(), ..mech::Model::default() };
    for bar in &document.frame_bars {
        bar_member(document, bar, &mut model, secant)?;
    }
    model.supports = document
        .frame_nodes
        .iter()
        .map(|n| mech::Support { node: n.id.clone(), fixed: support_flags(n.support.as_str()), ..mech::Support::default() })
        .collect();
    let mut case = mech::LoadCase { id: "design".into(), ..mech::LoadCase::default() };
    for (node, force) in &loads.nodal {
        node_of(&model, node)?;
        push_nodal(&mut case, node, [force[0], 0.0, force[1], 0.0, force[2], 0.0]);
    }
    model.load_cases = vec![case];
    Ok(model)
}

fn fault_of(e: mech::MechanicsError) -> FrameFault {
    match e {
        mech::MechanicsError::NoPositiveEigenvalue => FrameFault::new("No bar of the frame is in compression, so the frame has no buckling mode.", "Kein Stab des Rahmens ist gedrückt, der Rahmen hat daher keine Knickfigur."),
        mech::MechanicsError::Unstable { node, .. } => FrameFault::new(format!("The frame is a mechanism at node '{node}': add a support or remove a hinge."), format!("Der Rahmen ist am Knoten '{node}' kinematisch: Lager ergänzen oder Gelenk entfernen.")),
        mech::MechanicsError::Cancelled => FrameFault::new("The frame analysis was cancelled.", "Die Rahmenberechnung wurde abgebrochen."),
        other => FrameFault::new(format!("The frame could not be solved: {other}."), format!("Der Rahmen konnte nicht berechnet werden: {other}.")),
    }
}

fn restrain_out_of_plane(model: &mut mech::Model) {
    let nodes: Vec<String> = model.nodes.iter().map(|n| n.id.clone()).collect();
    for id in nodes {
        match model.supports.iter_mut().find(|s| s.node == id) {
            Some(support) => support.fixed.iter_mut().zip(OUT_OF_PLANE).for_each(|(f, o)| *f |= o),
            None => model.supports.push(mech::Support { node: id, fixed: OUT_OF_PLANE, ..mech::Support::default() }),
        }
    }
}

fn add_line_loads(model: &mut mech::Model, loads: &FrameLoads) -> Result<(), FrameFault> {
    for line in &loads.line {
        if !model.members.iter().any(|m| bar_of(&m.id) == line.bar) {
            return Err(missing("Frame bar", "Rahmenstab", &line.bar));
        }
        let q = [line.fx, 0.0, line.fz];
        model.load_cases[0].distributed.extend((line.from + 1..=line.to).map(|k| mech::DistributedLoad { member: format!("{}#{k}", line.bar), frame: mech::LoadFrame::Global, start: q, end: q }));
    }
    Ok(())
}

fn mode(document: &En1993Snapshot, found: mech::BucklingMode) -> FrameMode {
    let shape: Vec<(String, f64, f64)> = found.shape.iter().map(|d| (d.node.clone(), d.u[0], d.u[2])).collect();
    let corners = shape.iter().filter(|s| document.frame_nodes.iter().any(|n| n.id == s.0));
    let (ux, uz) = corners.fold((0.0_f64, 0.0_f64), |(x, z), s| (x.max(s.1.abs()), z.max(s.2.abs())));
    FrameMode { alpha_cr: found.alpha_cr, sway: ux >= uz, shape }
}

/// 🧭️ The node ids along a bar in the buckling solution, from its start node to its end node.
pub fn bar_nodes(bar: &FrameBar) -> Vec<String> {
    std::iter::once(bar.start_node_id.clone()).chain((1..SEGMENTS).map(|k| format!("{}@{k}", bar.id))).chain(std::iter::once(bar.end_node_id.clone())).collect()
}

/// 🌀️ The lowest buckling modes of the frame: the kernel solves `(K + α Kg) φ = 0` for the axial forces of the design loads on bars split into four elements.
pub fn frame_eigen(document: &En1993Snapshot, control: &mech::Control<'_>) -> Result<FrameEigen, FrameFault> {
    frame_eigen_with(document, control, &no_joint_stiffness, &FrameLoads::design(document))
}

/// 🌀️ The lowest buckling modes of the frame under the load vector `loads` with semi-rigid bar ends from the joint stiffness `secant`.
pub fn frame_eigen_with(document: &En1993Snapshot, control: &mech::Control<'_>, secant: Secant<'_>, loads: &FrameLoads) -> Result<FrameEigen, FrameFault> {
    let mut model = model_with(document, secant, loads)?.refine(SEGMENTS);
    restrain_out_of_plane(&mut model);
    add_line_loads(&mut model, loads)?;
    let analysis = mech::Analysis::new(&model, control).map_err(fault_of)?;
    let result = analysis.buckling(&model.load_cases[0], MODES, control).map_err(fault_of)?;
    Ok(FrameEigen { modes: result.modes.into_iter().map(|m| mode(document, m)).collect() })
}

/// 🌀️ The buckling modes of the frame without progress or cancellation.
pub fn solve(document: &En1993Snapshot) -> Result<FrameEigen, FrameFault> {
    frame_eigen(document, &mech::Control::none())
}

/// 🌀️ The sway load factor α_cr of the frame (EN 1993-1-1 5.2.1) without progress or cancellation.
pub fn alpha_cr(document: &En1993Snapshot) -> Result<f64, FrameFault> {
    solve(document).map(|e| e.governing())
}

/// 📊️ Peak internal forces of one bar over all its solution elements: signed axial force (tension positive) of the largest magnitude, largest shear and largest bending moment magnitude (N, N, Nm).
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct BarEffects {
    pub n: f64,
    pub v: f64,
    pub m: f64,
}

/// 📍️ The internal forces of one bar section: distance `x` from the start node [m], axial force `n` (tension positive) [N], shear `v` [N] and bending moment `m` [Nm] in the local frame of the bar.
#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub struct Station {
    pub x: f64,
    pub n: f64,
    pub v: f64,
    pub m: f64,
}

/// 🔁️ First-order and second-order peak internal forces of one frame bar and their courses along the bar (one station per solution node).
#[derive(Clone, Debug, PartialEq)]
pub struct BarForces {
    pub bar_id: String,
    pub first: BarEffects,
    pub second: BarEffects,
    pub first_profile: Vec<Station>,
    pub second_profile: Vec<Station>,
}

/// 📈️ The first-order course with shear and bending moment multiplied by 1/(1-1/α_cr) (EN 1993-1-1 5.2.2(5)B); `None` outside 3 ≤ α_cr < 10, where the amplified first-order method is not admitted. The axial force stays unamplified. See [EN 1993-1-1](https://eurocodes.jrc.ec.europa.eu/).
pub fn amplified_profile(profile: &[Station], alpha_cr: f64) -> Option<Vec<Station>> {
    (3.0..10.0).contains(&alpha_cr).then(|| {
        let k = 1.0 / (1.0 - 1.0 / alpha_cr);
        profile.iter().map(|s| Station { x: s.x, n: s.n, v: s.v * k, m: s.m * k }).collect()
    })
}

impl BarForces {
    /// 📈️ Second-order over first-order peak bending moment; 1 when the first-order moment vanishes.
    pub fn moment_amplification(&self) -> f64 {
        if self.first.m > 1e-9 { self.second.m / self.first.m } else { 1.0 }
    }
}

/// 📐️ Equivalent loads of the imperfections of EN 1993-1-1 5.3.2 added to the design loads of a second-order analysis: horizontal forces `(node id, N)` for the global sway imperfection (5.3.2(7)) and uniform transverse loads `(bar id, N/m)` in the local z direction for the bow imperfection (5.3.2(11)). The clause values are derived by the caller from `clauses::global`.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct Imperfections {
    pub sway: Vec<(String, f64)>,
    pub bow: Vec<(String, f64)>,
}

impl Imperfections {
    /// 🎯️ True when no imperfection load is present.
    pub fn is_empty(&self) -> bool {
        self.sway.is_empty() && self.bow.is_empty()
    }
}

/// 🔁️ Geometrically nonlinear (P-Δ and P-δ) solution of the design loads and the imperfection loads: the stiffness `K + Kg(N)` is iterated on the member axial forces until they settle.
#[derive(Clone, Debug, PartialEq)]
pub struct SecondOrder {
    pub iterations: usize,
    pub converged: bool,
    pub bars: Vec<BarForces>,
    pub first_shape: Vec<(String, f64, f64)>,
    pub shape: Vec<(String, f64, f64)>,
}

impl SecondOrder {
    /// 🎯️ The forces of one bar.
    pub fn bar(&self, id: &str) -> Option<&BarForces> {
        self.bars.iter().find(|b| b.bar_id == id)
    }

    /// 📏️ Largest in-plane translation of any solution node, second order over first order.
    pub fn drift_amplification(&self) -> f64 {
        let peak = |shape: &[(String, f64, f64)]| shape.iter().fold(0.0_f64, |m, s| m.max(s.1.hypot(s.2)));
        let first = peak(&self.first_shape);
        if first > 1e-12 { peak(&self.shape) / first } else { 1.0 }
    }
}

fn plus(a: &mech::elements::M12, b: &mech::elements::M12) -> mech::elements::M12 {
    std::array::from_fn(|i| std::array::from_fn(|j| a[i][j] + b[i][j]))
}

fn bar_of(member_id: &str) -> &str {
    member_id.rsplit_once('#').map_or(member_id, |p| p.0)
}

fn peaks(document: &En1993Snapshot, model: &mech::Model, forces: &[[f64; 12]]) -> Vec<(String, BarEffects)> {
    document
        .frame_bars
        .iter()
        .map(|bar| {
            let mut effects = BarEffects::default();
            for f in model.members.iter().zip(forces).filter(|(m, _)| bar_of(&m.id) == bar.id).map(|(_, f)| f) {
                let n = 0.5 * (-f[0] + f[6]);
                if n.abs() > effects.n.abs() {
                    effects.n = n;
                }
                effects.v = effects.v.max(f[2].abs()).max(f[8].abs());
                effects.m = effects.m.max(f[4].abs()).max(f[10].abs());
            }
            (bar.id.clone(), effects)
        })
        .collect()
}

fn profiles(document: &En1993Snapshot, model: &mech::Model, forces: &[[f64; 12]]) -> Vec<Vec<Station>> {
    document
        .frame_bars
        .iter()
        .map(|bar| {
            let origin = document.frame_nodes.iter().find(|n| n.id == bar.start_node_id);
            let at = |id: &str| model.nodes.iter().find(|n| n.id == id).zip(origin).map_or(0.0, |(n, o)| (n.x - o.x).hypot(n.z - o.z));
            let mut elements: Vec<(usize, usize)> = model.members.iter().enumerate().filter(|(_, m)| bar_of(&m.id) == bar.id).filter_map(|(i, m)| Some((m.id.rsplit_once('#')?.1.parse::<usize>().ok()?, i))).collect();
            elements.sort_unstable();
            let mut out = Vec::new();
            for (k, &(_, i)) in elements.iter().enumerate() {
                let (m, f) = (&model.members[i], &forces[i]);
                if k == 0 {
                    out.push(Station { x: at(&m.node_i), n: -f[0], v: -f[2], m: -f[4] });
                }
                out.push(Station { x: at(&m.node_j), n: f[6], v: f[8], m: f[10] });
            }
            out
        })
        .collect()
}

fn shape_of(analysis: &mech::Analysis<'_>, full: &[f64]) -> Vec<(String, f64, f64)> {
    analysis.system.displacements(analysis.model, full).into_iter().map(|d| (d.node, d.u[0], d.u[2])).collect()
}

fn add_imperfections(model: &mut mech::Model, imperfections: &Imperfections) -> Result<(), FrameFault> {
    for (node, force) in &imperfections.sway {
        if !model.nodes.iter().any(|n| &n.id == node) {
            return Err(missing("Frame node", "Rahmenknoten", node));
        }
        push_nodal(&mut model.load_cases[0], node, [*force, 0.0, 0.0, 0.0, 0.0, 0.0]);
    }
    for (bar, q) in &imperfections.bow {
        if !model.members.iter().any(|m| bar_of(&m.id) == bar) {
            return Err(missing("Frame bar", "Rahmenstab", bar));
        }
        model.load_cases[0].distributed.extend((1..=SEGMENTS).map(|k| mech::DistributedLoad { member: format!("{bar}#{k}"), frame: mech::LoadFrame::Local, start: [0.0, 0.0, *q], end: [0.0, 0.0, *q] }));
    }
    Ok(())
}

/// 🔁️ Second-order analysis of the frame (P-Δ with P-δ on four elements per bar) by iterating the geometric stiffness of the member axial forces; the internal forces are the first-order ones plus the elastic increment `k (u₂ − u₁)` of every element, so the bending moments include the amplification of EN 1993-1-1 5.2.2. Needs `α_cr > 1`; the faults name the mechanism or the unstable load level.
pub fn second_order(document: &En1993Snapshot, control: &mech::Control<'_>, loads: &FrameLoads, imperfections: &Imperfections) -> Result<SecondOrder, FrameFault> {
    second_order_with(document, control, loads, imperfections, &no_joint_stiffness)
}

/// 🔁️ The second-order analysis under the load vector `loads` with semi-rigid bar ends from the joint stiffness `secant`; the first-order course of the same load vector is `BarForces::first*`.
pub fn second_order_with(document: &En1993Snapshot, control: &mech::Control<'_>, loads: &FrameLoads, imperfections: &Imperfections, secant: Secant<'_>) -> Result<SecondOrder, FrameFault> {
    let mut model = model_with(document, secant, loads)?.refine(SEGMENTS);
    restrain_out_of_plane(&mut model);
    add_line_loads(&mut model, loads)?;
    add_imperfections(&mut model, imperfections)?;
    let analysis = mech::Analysis::new(&model, control).map_err(fault_of)?;
    let sys = &analysis.system;
    let raw = analysis.solve_raw(&model.load_cases[0], control).map_err(fault_of)?;
    let mut first = vec![0.0; sys.n_free];
    for (d, &f) in sys.fmap.iter().enumerate().filter(|(_, f)| **f != NO_FREE_DOF) {
        first[f] = raw.u[d];
    }
    let rhs = analysis.stiffness.mul_vec(&first);
    let local = |full: &[f64], m: usize| -> [f64; 12] {
        let d = sys.dofs(m);
        let global: [f64; 12] = std::array::from_fn(|a| full[d[a]]);
        mech::elements::rotate(&sys.elements[m].r, &global, false)
    };
    let forces_of = |full: &[f64]| -> Vec<[f64; 12]> {
        (0..sys.elements.len())
            .map(|m| {
                let (second, base) = (local(full, m), local(&raw.u, m));
                let step: [f64; 12] = std::array::from_fn(|a| second[a] - base[a]);
                let increment = mech::elements::mat_vec(&sys.elements[m].k, &step);
                std::array::from_fn(|a| raw.forces[m][a] + increment[a])
            })
            .collect()
    };
    let mut n = analysis.axial_forces(&raw);
    let mut full = raw.u.clone();
    let mut forces = raw.forces.clone();
    let (mut iterations, mut converged) = (0, false);
    for step in 1..=MAX_ITERATIONS {
        control.check().map_err(fault_of)?;
        control.report("second-order", step as f64 / MAX_ITERATIONS as f64);
        let mut tangent = sys.assemble(control, "second-order", |m| plus(&sys.elements[m].k, &sys.elements[m].geometric(n[m]))).map_err(fault_of)?;
        for d in (0..sys.fmap.len()).filter(|&d| sys.fmap[d] != NO_FREE_DOF && sys.spring[d] > 0.0) {
            tangent.add(sys.fmap[d], sys.fmap[d], sys.spring[d]);
        }
        tangent.cholesky(control).map_err(|e| match e {
            mech::MechanicsError::Singular { .. } => FrameFault::new("The design load reaches the elastic critical load (α_cr ≤ 1): the second-order solution does not exist.", "Die Bemessungslast erreicht die elastische Verzweigungslast (α_cr ≤ 1): eine Lösung nach Theorie II. Ordnung existiert nicht."),
            other => fault_of(other),
        })?;
        let mut x = rhs.clone();
        tangent.solve(&mut x);
        full = sys.expand(&x);
        forces = forces_of(&full);
        let next: Vec<f64> = forces.iter().map(|f| 0.5 * (-f[0] + f[6])).collect();
        let change = next.iter().zip(&n).map(|(a, b)| (a - b).abs()).fold(0.0_f64, f64::max);
        let scale = next.iter().fold(1.0_f64, |m, v| m.max(v.abs()));
        n = next;
        iterations = step;
        if change <= TOLERANCE * scale {
            converged = true;
            break;
        }
    }
    let (before, after) = (peaks(document, &model, &raw.forces), peaks(document, &model, &forces));
    let (course_before, course_after) = (profiles(document, &model, &raw.forces), profiles(document, &model, &forces));
    let bars = before.into_iter().zip(after).zip(course_before.into_iter().zip(course_after)).map(|(((bar_id, first), (_, second)), (first_profile, second_profile))| BarForces { bar_id, first, second, first_profile, second_profile }).collect();
    Ok(SecondOrder { iterations, converged, bars, first_shape: shape_of(&analysis, &raw.u), shape: shape_of(&analysis, &full) })
}

/// 🔁️ The second-order solution of the frame under the load vector `loads` and the given imperfection loads without progress or cancellation.
pub fn solve_with_imperfections(document: &En1993Snapshot, loads: &FrameLoads, imperfections: &Imperfections) -> Result<SecondOrder, FrameFault> {
    second_order(document, &mech::Control::none(), loads, imperfections)
}

/// 🔁️ The second-order solution of the frame without imperfections, progress or cancellation.
pub fn solve_second_order(document: &En1993Snapshot) -> Result<SecondOrder, FrameFault> {
    second_order(document, &mech::Control::none(), &FrameLoads::design(document), &Imperfections::default())
}

#[cfg(test)]
mod tests {
    use super::*;
    use semio_s_artifact_norm_en1993_clauses::global::alpha_cr_portal_fixed;

    fn portal() -> En1993Snapshot {
        let mut doc = En1993Snapshot::frame_portal_showcase();
        doc.frame_nodes.iter_mut().for_each(|n| n.load_x = 0.0);
        doc
    }

    #[test]
    fn the_fixed_base_portal_matches_the_stability_function_solution() {
        let doc = portal();
        let (e, i) = (doc.materials[0].e_modulus, doc.sections[0].iy);
        let expected = alpha_cr_portal_fixed(e, i, i, 4.0, 6.0, 200_000.0).unwrap();
        let eigen = frame_eigen(&doc, &mech::Control::none()).unwrap();
        let sway = eigen.sway().expect("a sway mode").alpha_cr;
        assert!((sway - expected).abs() / expected < 0.02, "kernel {sway} vs closed form {expected}");
        assert!(eigen.lowest() <= sway);
    }

    #[test]
    fn a_fixed_base_column_follows_euler_with_the_cantilever_length() {
        let mut doc = portal();
        doc.frame_nodes.retain(|n| n.id == "node-a" || n.id == "node-b");
        doc.frame_bars.retain(|b| b.id == "bar-1");
        doc.frame_nodes.iter_mut().find(|n| n.id == "node-b").unwrap().load_z = -100_000.0;
        let (e, i) = (doc.materials[0].e_modulus, doc.sections[0].iy);
        let expected = std::f64::consts::PI.powi(2) * e * i / (4.0 * 16.0) / 100_000.0;
        let got = alpha_cr(&doc).unwrap();
        assert!((got - expected).abs() / expected < 0.005, "kernel {got} vs Euler {expected}");
    }

    #[test]
    fn the_kernel_reproduces_the_independent_scipy_assembly_of_the_oracle() {
        let doc = portal();
        let got = frame_eigen(&doc, &mech::Control::none()).unwrap();
        let sway = got.sway().unwrap().alpha_cr;
        assert!((sway - 48.761_076_625).abs() / 48.761_076_625 < 0.01, "portal {sway} vs oracle 48.7611");
        assert!(got.modes.windows(2).all(|w| w[0].alpha_cr <= w[1].alpha_cr));
        assert_eq!(bar_nodes(&doc.frame_bars[0]), vec!["node-a", "bar-1@1", "bar-1@2", "bar-1@3", "node-b"]);
    }

    #[test]
    fn faults_name_the_missing_reference() {
        let mut doc = portal();
        doc.frame_bars[0].end_node_id = "node-x".into();
        assert!(model(&doc).unwrap_err().en.contains("node-x"));
        doc.frame_bars.clear();
        assert!(frame_eigen(&doc, &mech::Control::none()).is_err());
    }
    #[test]
    fn the_cantilever_sway_follows_the_exact_second_order_solution() {
        let mut doc = portal();
        doc.frame_nodes.retain(|n| n.id == "node-a" || n.id == "node-b");
        doc.frame_bars.retain(|b| b.id == "bar-1");
        let (e, i) = (doc.materials[0].e_modulus, doc.sections[0].iy);
        let (h, p, l) = (10_000.0, 200_000.0, 4.0);
        {
            let tip = doc.frame_nodes.iter_mut().find(|n| n.id == "node-b").unwrap();
            tip.load_x = h;
            tip.load_z = -p;
        }
        let result = second_order(&doc, &mech::Control::none(), &FrameLoads::design(&doc), &Imperfections::default()).unwrap();
        assert!(result.converged);
        let k = (p / (e * i)).sqrt();
        let exact = h / p * ((k * l).tan() / k - l);
        let first = h * l.powi(3) / (3.0 * e * i);
        let tip = |s: &[(String, f64, f64)]| s.iter().find(|s| s.0 == "node-b").unwrap().1;
        assert!((tip(&result.first_shape) - first).abs() / first < 1e-6);
        assert!((tip(&result.shape) - exact).abs() / exact < 0.01, "kernel {} vs exact {exact}", tip(&result.shape));
        let bar = result.bar("bar-1").unwrap();
        assert!(bar.moment_amplification() > 1.0);
        let alpha = alpha_cr(&doc).unwrap();
        let estimate = 1.0 / (1.0 - 1.0 / alpha);
        assert!((bar.moment_amplification() - estimate).abs() / estimate < 0.1, "bar {} vs 1/(1-1/a) {estimate}", bar.moment_amplification());
    }

    #[test]
    fn a_load_above_the_critical_load_is_refused() {
        let mut doc = portal();
        doc.frame_nodes.retain(|n| n.id == "node-a" || n.id == "node-b");
        doc.frame_bars.retain(|b| b.id == "bar-1");
        let (e, i) = (doc.materials[0].e_modulus, doc.sections[0].iy);
        let tip = doc.frame_nodes.iter_mut().find(|n| n.id == "node-b").unwrap();
        tip.load_x = 1_000.0;
        tip.load_z = -3.0 * std::f64::consts::PI.powi(2) * e * i / 64.0;
        assert!(second_order(&doc, &mech::Control::none(), &FrameLoads::design(&doc), &Imperfections::default()).is_err());
    }


    #[test]
    fn the_sway_imperfection_forces_reproduce_the_pynite_p_delta_portal() {
        let doc = portal();
        let h = 0.005 * (0.5 * (1.0 + 0.5_f64)).sqrt() * 200_000.0;
        let imperfections = Imperfections { sway: vec![("node-b".into(), h), ("node-c".into(), h)], bow: vec![] };
        let result = second_order(&doc, &mech::Control::none(), &FrameLoads::design(&doc), &imperfections).unwrap();
        assert!(result.converged);
        let tip = result.shape.iter().find(|s| s.0 == "node-b").unwrap().1;
        assert!((tip - 3.194_674_3e-4).abs() / 3.194_674_3e-4 < 0.01, "sway {tip} vs PyNite 3.1947e-4");
        let moment = result.bar("bar-1").unwrap().second.m;
        assert!((moment - 2_114.940_3).abs() / 2_114.940_3 < 0.01, "column moment {moment} vs PyNite 2114.94");
    }

    #[test]
    fn the_bow_load_reproduces_the_pynite_and_the_secant_solution() {
        let mut doc = portal();
        doc.frame_nodes.retain(|n| n.id == "node-a" || n.id == "node-b");
        doc.frame_bars.retain(|b| b.id == "bar-1");
        for node in doc.frame_nodes.iter_mut() {
            node.load_x = 0.0;
            node.load_z = 0.0;
            node.support = if node.id == "node-a" { "pinned" } else { "roller" }.into();
        }
        let tip = doc.frame_nodes.iter_mut().find(|n| n.id == "node-b").unwrap();
        (tip.x, tip.z, tip.load_x) = (8.0, 0.0, -1.0e6);
        let imperfections = Imperfections { sway: vec![], bow: vec![("bar-1".into(), 8.0 * 1.0e6 * (8.0 / 250.0) / 64.0)] };
        let result = second_order(&doc, &mech::Control::none(), &FrameLoads::design(&doc), &imperfections).unwrap();
        let moment = result.bar("bar-1").unwrap().second.m;
        assert!((moment - 44_440.282).abs() / 44_440.282 < 0.01, "bow moment {moment} vs PyNite 44440.3");
        assert!((moment - 44_442.973).abs() / 44_442.973 < 0.01, "bow moment {moment} vs secant 44443.0");
        assert!(second_order(&doc, &mech::Control::none(), &FrameLoads::design(&doc), &Imperfections { sway: vec![], bow: vec![("bar-x".into(), 1.0)] }).is_err());
    }

    fn beam_ends(release: &str, spring: Option<f64>) -> f64 {
        let mut doc = portal();
        doc.joints.push(SteelJoint { member_id: "member-b1".into(), ..En1993Snapshot::special_parts_showcase().joints[0].clone() });
        doc.frame_bars[1].release_start = release.into();
        doc.frame_bars[1].release_end = release.into();
        let secant = move |_: &SteelJoint| spring;
        frame_eigen_with(&doc, &mech::Control::none(), &secant, &FrameLoads::design(&doc)).unwrap().sway().unwrap().alpha_cr
    }

    #[test]
    fn semi_rigid_beam_ends_lie_between_the_pinned_and_the_rigid_frame_and_reach_both_limits() {
        let (rigid, pinned) = (beam_ends("rigid", None), beam_ends("pinned", None));
        let (soft, medium, stiff) = (beam_ends("semiRigid", Some(1.0e4)), beam_ends("semiRigid", Some(2.0e7)), beam_ends("semiRigid", Some(1.0e12)));
        assert!(pinned < medium && medium < rigid, "{pinned} < {medium} < {rigid}");
        assert!((soft - pinned).abs() / pinned < 0.03, "soft {soft} vs pinned {pinned}");
        assert!((stiff - rigid).abs() / rigid < 0.01, "stiff {stiff} vs rigid {rigid}");
    }

    #[test]
    fn semi_rigid_beam_ends_match_the_exact_spring_oracle_of_the_python_assembly_and_pynite() {
        for (spring, expected) in [(1.0e4, 18.394988), (2.0e7, 37.857085), (1.0e12, 49.192523)] {
            let alpha = beam_ends("semiRigid", Some(spring));
            assert!((alpha - expected).abs() / expected < 0.03, "S={spring}: kernel {alpha} vs spring oracle {expected}");
        }
    }

    #[test]
    fn a_semi_rigid_end_without_a_joint_stiffness_names_the_bar() {
        let mut doc = portal();
        doc.frame_bars[1].release_start = semio_s_artifact_norm_en1993_model::FrameBarRelease::SemiRigid;
        assert!(model(&doc).unwrap_err().en.contains("bar-2"));
    }

    #[test]
    fn the_course_along_a_cantilever_column_is_the_statical_moment_and_shear_and_amplifies_in_the_admitted_range() {
        let mut doc = portal();
        doc.frame_nodes.retain(|n| n.id == "node-a" || n.id == "node-b");
        doc.frame_bars.retain(|b| b.id == "bar-1");
        let top = doc.frame_nodes.iter_mut().find(|n| n.id == "node-b").unwrap();
        top.load_x = 10_000.0;
        top.load_z = 0.0;
        let forces = solve_second_order(&doc).unwrap();
        let bar = forces.bar("bar-1").unwrap();
        assert_eq!(bar.first_profile.len(), SEGMENTS + 1);
        assert!(bar.first_profile.windows(2).all(|w| w[0].x < w[1].x));
        assert!((bar.first_profile.last().unwrap().x - 4.0).abs() < 1e-9);
        assert!((bar.first_profile[0].m.abs() - 40_000.0).abs() < 1.0, "base moment {}", bar.first_profile[0].m);
        assert!(bar.first_profile.last().unwrap().m.abs() < 1.0);
        assert!(bar.first_profile.iter().all(|s| (s.v.abs() - 10_000.0).abs() < 1.0));
        let expected = 1.0 / (1.0 - 1.0 / 5.0);
        let amplified = amplified_profile(&bar.first_profile, 5.0).unwrap();
        assert!((amplified[0].m / bar.first_profile[0].m - expected).abs() < 1e-12);
        assert!(amplified_profile(&bar.first_profile, 2.9).is_none() && amplified_profile(&bar.first_profile, 10.0).is_none());
    }

    #[test]
    fn the_first_order_courses_of_the_demo_portal_match_pynite_and_anastruct() {
        let doc = En1993Snapshot::frame_portal_showcase();
        let forces = solve_second_order(&doc).unwrap();
        for (bar, moment, shear, axial) in [("bar-1", 24_089.589, 10_026.073, 194_671.698), ("bar-2", 16_014.705, 5_328.302, 9_973.927)] {
            let first = forces.bar(bar).unwrap().first;
            for (got, expected, what) in [(first.m, moment, "M"), (first.v, shear, "V"), (first.n.abs(), axial, "N")] {
                assert!((got - expected).abs() / expected < 0.005, "{bar} {what}: kernel {got} vs PyNite/anastruct {expected}");
            }
        }
    }

    fn cantilever() -> En1993Snapshot {
        let mut doc = portal();
        doc.frame_nodes.retain(|n| n.id == "node-a" || n.id == "node-b");
        doc.frame_nodes.iter_mut().for_each(|n| n.load_z = 0.0);
        doc.frame_bars.retain(|b| b.id == "bar-1");
        doc
    }

    fn base_moment(doc: &En1993Snapshot, loads: &FrameLoads) -> (f64, f64) {
        let result = second_order(doc, &mech::Control::none(), loads, &Imperfections::default()).unwrap();
        let first = &result.bar("bar-1").unwrap().first_profile[0];
        (first.m.abs(), first.v.abs())
    }

    #[test]
    fn a_line_load_on_a_cantilever_column_gives_the_statical_base_moment_and_shear() {
        let doc = cantilever();
        let full = FrameLoads { nodal: vec![], line: vec![LineLoad { bar: "bar-1".into(), fx: 5_000.0, fz: 0.0, from: 0, to: SEGMENTS }] };
        let (m, v) = base_moment(&doc, &full);
        assert!((m - 40_000.0).abs() < 1.0 && (v - 20_000.0).abs() < 1.0, "full M {m} V {v}");
        let upper = FrameLoads { nodal: vec![], line: vec![LineLoad { bar: "bar-1".into(), fx: 5_000.0, fz: 0.0, from: 2, to: SEGMENTS }] };
        let (m, v) = base_moment(&doc, &upper);
        assert!((m - 30_000.0).abs() < 1.0 && (v - 10_000.0).abs() < 1.0, "upper half M {m} V {v}");
    }

    #[test]
    fn the_first_order_forces_of_load_vectors_superpose_and_a_nodal_moment_is_a_constant_moment() {
        let doc = cantilever();
        let a = FrameLoads { nodal: vec![("node-b".into(), [10_000.0, 0.0, 0.0])], line: vec![] };
        let b = FrameLoads { nodal: vec![], line: vec![LineLoad { bar: "bar-1".into(), fx: 5_000.0, fz: 0.0, from: 0, to: SEGMENTS }] };
        let mut both = a.clone();
        both.add(&b);
        let ((ma, _), (mb, _), (mc, _)) = (base_moment(&doc, &a), base_moment(&doc, &b), base_moment(&doc, &both));
        assert!((ma + mb - mc).abs() < 1.0, "{ma} + {mb} vs {mc}");
        let (m, v) = base_moment(&doc, &a.scaled(2.5));
        assert!((m - 100_000.0).abs() < 1.0 && (v - 25_000.0).abs() < 1.0);
        let moment = FrameLoads { nodal: vec![("node-b".into(), [0.0, 0.0, 12_000.0])], line: vec![] };
        let (m, v) = base_moment(&doc, &moment);
        assert!((m - 12_000.0).abs() < 1.0 && v < 1.0, "moment {m} shear {v}");
    }

    #[test]
    fn the_actions_of_a_load_case_become_loads_and_misplaced_ones_name_themselves() {
        let mut doc = cantilever();
        let action = |id: &str, kind, target: &str, start: f64, end: f64| semio_s_artifact_norm_en1993_model::FrameAction { id: id.into(), load_case_id: "q-imposed".into(), target_id: target.into(), kind, fx: 1_000.0, fz: -2_000.0, my: 300.0, start_ratio: start, end_ratio: end };
        doc.frame_actions = vec![action("a1", FrameActionKind::NodalForce, "node-b", 0.0, 1.0), action("a2", FrameActionKind::NodalMoment, "node-b", 0.0, 1.0), action("a3", FrameActionKind::BarUniform, "bar-1", 0.25, 0.75)];
        let loads = FrameLoads::of_case(&doc, "q-imposed").unwrap();
        assert_eq!(loads.nodal, vec![("node-b".to_string(), [1_000.0, -2_000.0, 0.0]), ("node-b".to_string(), [0.0, 0.0, 300.0])]);
        assert_eq!(loads.line, vec![LineLoad { bar: "bar-1".into(), fx: 1_000.0, fz: -2_000.0, from: 1, to: 3 }]);
        assert!(FrameLoads::of_case(&doc, "g-permanent").unwrap().is_empty());
        doc.frame_actions[2].start_ratio = 0.3;
        assert!(FrameLoads::of_case(&doc, "q-imposed").unwrap_err().en.contains("a3"));
        doc.frame_actions[2].start_ratio = 0.75;
        assert!(FrameLoads::of_case(&doc, "q-imposed").unwrap_err().en.contains("ends before"));
        doc.frame_actions[2] = action("a4", FrameActionKind::BarUniform, "bar-x", 0.0, 1.0);
        assert!(FrameLoads::of_case(&doc, "q-imposed").unwrap_err().en.contains("bar-x"));
    }
}
