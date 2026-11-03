//! 🕸️ Frame model: every frame bar as an extruded section between its nodes, its bending moment surface drawn on the frame, the supports and nodal design loads of the frame nodes and the α_cr clause of the frame check. The frame stands in the plane `y = ROW_FRAME` with `z` up.

use super::kit::*;
use super::layout::Layout;
use super::model::{FrameBar, FrameNode};
use super::scene::*;
use super::checks::{frame_eigen, frame_envelope, joint_frame};
use super::loads_and_supports::case_kind;
use super::tracks::{self, Mode};
use super::SceneInput;

const MODE_AMPLITUDE: f64 = 0.05;
const DEFORMED_AMPLITUDE: f64 = 0.03;
const MOMENT_EXTENT: f64 = 0.15;

/// 🔀️ Splits a moment course `(x, M)` at its zero crossings (linear interpolation) into runs of one sign: `true` for positive, `false` for negative. See EN 1993-1-1 5.2.2 for the second-order course.
pub fn sign_runs(points: &[(f64, f64)]) -> Vec<(bool, Vec<(f64, f64)>)> {
    fn flush(runs: &mut Vec<(bool, Vec<(f64, f64)>)>, cur: &mut Vec<(f64, f64)>) {
        let sign = cur.iter().find(|c| c.1 != 0.0).map(|c| c.1 > 0.0);
        match sign {
            Some(s) if cur.len() >= 2 => runs.push((s, std::mem::take(cur))),
            _ => cur.clear(),
        }
    }
    let mut runs = Vec::new();
    let mut cur: Vec<(f64, f64)> = Vec::new();
    for (k, &p) in points.iter().enumerate() {
        if k > 0 {
            let q = points[k - 1];
            if q.1 * p.1 < 0.0 {
                let x = q.0 + (p.0 - q.0) * q.1 / (q.1 - p.1);
                cur.push((x, 0.0));
                flush(&mut runs, &mut cur);
                cur.push((x, 0.0));
            }
        }
        cur.push(p);
    }
    flush(&mut runs, &mut cur);
    runs
}

/// 🏷️ Where a bending moment surface belongs: the suffix of its entity id, its layer (`None`: the annotations layer) and what it shows.
struct MomentSurface {
    tag: String,
    layer: Option<String>,
    what: LocalizedCopy,
}

#[allow(clippy::too_many_arguments)]
fn moment_surfaces(layout: &Layout, a: &FrameNode, c: &FrameNode, length: f64, stations: &[frame_eigen::Station], peak: f64, extent: f64, out: &mut Out, b: &FrameBar, surface: &MomentSurface) {
    if stations.len() < 2 || peak <= 1e-9 {
        return;
    }
    let (ex, ez) = ((c.x - a.x) / length, (c.z - a.z) / length);
    let (nx, nz) = (-ez, ex);
    let at = |x: f64, offset: f64| layout.frame_point(a.x + ex * x + nx * offset, a.z + ez * x + nz * offset);
    let points: Vec<(f64, f64)> = stations.iter().map(|s| (s.x, s.m)).collect();
    for (k, (positive, run)) in sign_runs(&points).into_iter().enumerate() {
        let edge: Vec<Vec3> = run.iter().map(|&(x, m)| at(x, MOMENT_EXTENT * extent * m / peak)).collect();
        let axis: Vec<Vec3> = run.iter().rev().filter(|r| r.1 != 0.0).map(|&(x, _)| at(x, 0.0)).collect();
        let ring: Vec<Vec3> = edge.into_iter().chain(axis).collect();
        if ring.len() < 3 {
            continue;
        }
        let (word, wort, tone) = if positive { ("positive", "positive", LoadKind::Variable) } else { ("negative", "negative", LoadKind::Permanent) };
        let name = loc(&format!("Bending moment surface of {} – {} ({word} part {})", b.id, surface.what.en, k + 1), &format!("Momentenfläche von {} – {} ({wort}r Anteil {})", b.id, surface.what.de, k + 1));
        let spec = Spec {
                kind: EntityKind::Surface,
                id: format!("{}:moment{}:{}{k}", b.id, surface.tag, if positive { "pos" } else { "neg" }),
                collection: "frameBars",
                geometry: Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Polygon { polygon: projection_kit::polygon3(&ring) } }),
                role: MaterialRole::Load { kind: tone },
                binding: None,
                detail: loc(&format!("Second-order bending moment, peak {:.1} kNm, drawn to the tension side; filled separately where the sign changes", peak / 1e3), &format!("Biegemoment nach Theorie II. Ordnung, Maximum {:.1} kNm, zur Zugseite aufgetragen; an Vorzeichenwechseln getrennt gefüllt", peak / 1e3)),
                name,
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
            };
        match &surface.layer {
            Some(layer) => out.push_tuned(spec, &ring, |entity| entity.layer = layer.clone()),
            None => out.push(spec, &ring),
        }
    }
}

/// 🌀️ The in-plane shapes of the frame solved once per document, not once per entity fragment: every bar and node fragment of an edit reads the same retained solution (the frame solves, sway mode, combinations and envelope, are the heavy part of a frame fragment).
fn shared_shapes(d: &super::model::En1993Snapshot) -> std::sync::Arc<Option<Shapes>> {
    static SLOT: std::sync::Mutex<Option<(super::model::En1993Snapshot, std::sync::Arc<Option<Shapes>>)>> = std::sync::Mutex::new(None);
    let mut slot = SLOT.lock().unwrap_or_else(std::sync::PoisonError::into_inner);
    if let Some((known, shapes)) = slot.as_ref() {
        if known == d {
            return shapes.clone();
        }
    }
    let shapes = std::sync::Arc::new(shapes_of(d));
    *slot = Some((d.clone(), shapes.clone()));
    shapes
}

/// 🌀️ The in-plane shapes of the frame solved once per projection: the sway mode (largest translation 1) and the second-order displacements in metres (the deformed ghost), plus the bending moment courses of every bar.
struct Shapes {
    alpha: f64,
    mode: Vec<(String, f64, f64)>,
    deformed: Option<Vec<(String, f64, f64)>>,
    forces: Option<frame_eigen::SecondOrder>,
    cases: Vec<frame_envelope::CaseResult>,
    envelope: Vec<frame_envelope::BarEnvelope>,
    imperfection: Option<(String, Vec<(String, f64)>)>,
    extent: f64,
}

fn shapes_of(d: &super::model::En1993Snapshot) -> Option<Shapes> {
    let eigen = joint_frame::solve(d).ok()?;
    let mode = eigen.sway().or_else(|| eigen.modes.first())?.clone();
    let extent = d.frame_nodes.iter().fold(1.0_f64, |m, n| m.max(n.x.abs()).max(n.z.abs()));
    let combinations = frame_envelope::combination_results_cached(d).unwrap_or_default();
    let governing = frame_envelope::governing_loads(d).ok().and_then(|g| combinations.iter().find(|c| c.id == g.id));
    let imperfection = governing.map(|c| (c.id.clone(), c.imperfection_forces()));
    let forces = governing.map(|c| c.result.clone()).or_else(|| joint_frame::solve_second_order(d).ok());
    let deformed = forces.as_ref().map(|s| s.shape.clone());
    let (cases, envelope) = if d.frame_actions.is_empty() { (Vec::new(), Vec::new()) } else { (frame_envelope::case_results(d).unwrap_or_default(), frame_envelope::envelope(d, &combinations)) };
    Some(Shapes { alpha: mode.alpha_cr, mode: mode.shape, deformed, forces, cases, envelope, imperfection, extent })
}

fn samples(bar: &FrameBar, length: f64, shape: &[(String, f64, f64)]) -> Vec<DisplacementSample> {
    let ids = frame_eigen::bar_nodes(bar);
    let last = (ids.len() - 1).max(1) as f64;
    ids.iter()
        .enumerate()
        .map(|(k, id)| {
            let (ux, uz) = shape.iter().find(|s| &s.0 == id).map_or((0.0, 0.0), |s| (s.1, s.2));
            DisplacementSample { at: length * k as f64 / last, delta: v3(ux, 0.0, uz) }
        })
        .collect()
}

fn overlay(kind: DeformationKind, samples: Vec<DisplacementSample>, scale: f64, load_factor: Option<f64>) -> DeformationOverlay {
    let mode = matches!(kind, DeformationKind::Mode);
    DeformationOverlay { kind, index: if mode { Mode::Frame.index() } else { 1 }, frequency: None, participation: 1.0, scale, samples, surface: Vec::new(), motion: if mode { Motion::Harmonic } else { Motion::Static }, load_factor }
}

fn node<'a>(nodes: &'a [FrameNode], id: &str) -> Option<&'a FrameNode> {
    nodes.iter().find(|n| n.id == id)
}

fn release(kind: &str) -> EndRelease {
    if kind == "pinned" { EndRelease::Pinned } else { EndRelease::Rigid }
}

fn bar(input: &SceneInput, layout: &Layout, b: &FrameBar, shapes: Option<&Shapes>, out: &mut Out) {
    let d = input.document;
    let (Some(a), Some(c)) = (node(&d.frame_nodes, &b.start_node_id), node(&d.frame_nodes, &b.end_node_id)) else { return };
    let Some(section) = d.members.iter().find(|m| m.id == b.member_id).and_then(|m| d.sections.iter().find(|s| s.id == m.section_id)) else { return };
    let length = ((c.x - a.x).powi(2) + (c.z - a.z).powi(2)).sqrt();
    if length <= 1e-6 {
        return;
    }
    let (p, q) = (layout.frame_point(a.x, a.z), layout.frame_point(c.x, c.z));
    let frame = member_frame(&p, &v3((c.x - a.x) / length, 0.0, (c.z - a.z) / length));
    let binding = binding(input.report, &[b.id.as_str()]);
    let deformed = shapes.and_then(|s| {
        let shape = s.deformed.as_ref()?;
        let peak = shape.iter().fold(0.0_f64, |m, n| m.max(n.1.hypot(n.2)));
        (peak > 1e-12).then(|| overlay(DeformationKind::Deformed, samples(b, length, shape), DEFORMED_AMPLITUDE * s.extent / peak, None))
    });
    let mode = shapes.map(|s| overlay(DeformationKind::Mode, samples(b, length, &s.mode), MODE_AMPLITUDE * s.extent, Some(s.alpha)));
    if let Some(forces) = shapes.and_then(|s| s.forces.as_ref().map(|f| (f, s.extent))) {
        let peak = forces.0.bars.iter().flat_map(|f| f.second_profile.iter()).fold(0.0_f64, |m, s| m.max(s.m.abs()));
        if let Some(own) = forces.0.bar(&b.id) {
            let governing = MomentSurface { tag: String::new(), layer: None, what: loc("governing load vector", "maßgebender Lastvektor") };
            moment_surfaces(layout, a, c, length, &own.second_profile, peak, forces.1, out, b, &governing);
        }
    }
    if let Some(s) = shapes {
        for case in &s.cases {
            let Some(own) = case.result.bar(&b.id) else { continue };
            let peak = case.result.bars.iter().flat_map(|f| f.second_profile.iter()).fold(0.0_f64, |m, st| m.max(st.m.abs()));
            let surface = MomentSurface { tag: format!(":case:{}", case.case_id), layer: Some(case_layer(&case.case_id)), what: loc(&format!("load case {}", case.case_id), &format!("Lastfall {}", case.case_id)) };
            moment_surfaces(layout, a, c, length, &own.second_profile, peak, s.extent, out, b, &surface);
        }
        if let Some(own) = s.envelope.iter().find(|e| e.bar_id == b.id) {
            let peak = s.envelope.iter().flat_map(|e| e.max.iter().chain(e.min.iter())).fold(0.0_f64, |m, st| m.max(st.m.abs()));
            for (tag, stations, en, de) in [(":envelope-max", &own.max, "envelope maximum", "Einhüllende Maximum"), (":envelope-min", &own.min, "envelope minimum", "Einhüllende Minimum")] {
                let surface = MomentSurface { tag: tag.into(), layer: Some(LAYER_FRAME_ENVELOPE.into()), what: loc(en, de) };
                moment_surfaces(layout, a, c, length, stations, peak, s.extent, out, b, &surface);
            }
        }
    }
    let (section_outline, bar_release) = (section_shape(section), release(b.release_start.as_str()));
    out.push(
        Spec {
            kind: EntityKind::Member,
            id: b.id.clone(),
            collection: "frameBars",
            geometry: Geometry::Member(MemberGeometry {
                axis: polyline(&[p.clone(), q.clone()]),
                frame: frame.clone(),
                section: section_outline.clone(),
                section_taper: None,
                start_offset: Vec2::ZERO,
                end_offset: Vec2::ZERO,
                start_release: bar_release,
                end_release: release(b.release_end.as_str()),
                stations: Vec::new(),
            }),
            role: MaterialRole::Steel,
            binding,
            name: loc(&b.label, &format!("Rahmenstab {}", b.label)),
            detail: loc(&format!("{}, {:.2} m, {} to {}", section.designation, length, b.start_node_id, b.end_node_id), &format!("{}, {:.2} m, {} bis {}", section.designation, length, b.start_node_id, b.end_node_id)),
            layer: LAYER_MEMBERS,
            handles: Vec::new(),
            deformation: deformed,
        },
        &[p.clone(), q],
    );
    if let Some(shape) = mode {
        tracks::push_mode(out, Mode::Frame, "frameBars", &b.id, &b.label, section_outline, &frame, length, shape, bar_release);
    }
}

fn support_kind(kind: &str) -> Option<SupportKind> {
    match kind {
        "fixed" => Some(SupportKind::Fixed),
        "pinned" => Some(SupportKind::Pinned),
        "roller" => Some(SupportKind::Roller { normal: Vec3::Z }),
        _ => None,
    }
}

fn node_entity(input: &SceneInput, layout: &Layout, n: &FrameNode, out: &mut Out) {
    let d = input.document;
    let at = layout.frame_point(n.x, n.z);
    let incident: Vec<EntityRef> = d.frame_bars.iter().filter(|b| b.start_node_id == n.id || b.end_node_id == n.id).map(|b| entity_ref(EntityKind::Member, &b.id, "frameBars")).collect();
    out.push(
        Spec {
            kind: EntityKind::Group,
            id: n.id.clone(),
            collection: "frameNodes",
            geometry: Geometry::Group { members: incident },
            role: MaterialRole::Steel,
            binding: None,
            name: loc(&n.label, &format!("Rahmenknoten {}", n.label)),
            detail: loc(&format!("({:.2} m, {:.2} m), support {}", n.x, n.z, n.support), &format!("({:.2} m, {:.2} m), Lagerung {}", n.x, n.z, n.support)),
            layer: LAYER_ANNOTATIONS,
            handles: super::handles::ids(d, layout, &n.id),
            deformation: None,
        },
        &[at.clone()],
    );
    if let Some(kind) = support_kind(n.support.as_str()) {
        out.push(
            Spec {
                kind: EntityKind::Support,
                id: format!("{}:support", n.id),
                collection: "frameNodes",
                geometry: Geometry::Support(SupportGeometry { kind, at: at.clone(), frame: member_frame(&at, &Vec3::X) }),
                role: MaterialRole::Steel,
                binding: None,
                name: loc(&format!("Support of {}", n.id), &format!("Lager von {}", n.id)),
                detail: loc(n.support.as_str(), n.support.as_str()),
                layer: LAYER_ANNOTATIONS,
                handles: Vec::new(),
                deformation: None,
            },
            &[at.clone()],
        );
    }
    let magnitude = n.load_x.hypot(n.load_z);
    if magnitude > 0.0 {
        let name = loc(&format!("Design load on {}", n.id), &format!("Bemessungslast auf {}", n.id));
        out.push(
            Spec {
                kind: EntityKind::Load,
                id: format!("{}:load", n.id),
                collection: "frameNodes",
                geometry: Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: v3(n.load_x / magnitude, 0.0, n.load_z / magnitude), magnitude: Quantity::new(QuantityKind::Force, magnitude), case: LoadCaseRef { id: "frame-design".into(), kind: LoadKind::Variable, psi: None } }),
                role: MaterialRole::Load { kind: LoadKind::Variable },
                binding: None,
                detail: name.clone(),
                name,
                layer: LAYER_LOADS,
                handles: Vec::new(),
                deformation: None,
            },
            &[at],
        );
    }
}

/// 🏗️ The glyph of one frame action on the layer of its load case: a force arrow or a moment at the node, or a uniform line load along the loaded part of the bar. The grips on it write `fx`, `fz` and `my`.
fn action_glyph(input: &SceneInput, layout: &Layout, action: &super::model::FrameAction, out: &mut Out) {
    use super::model::FrameActionKind as K;
    let d = input.document;
    let Some(case) = d.load_cases.iter().find(|c| c.id == action.load_case_id) else { return };
    let kind = case_kind(case.kind);
    let case_ref = LoadCaseRef { id: case.id.clone(), kind, psi: None };
    let layer = case_layer(&case.id);
    let name = |en: &str, de: &str| loc(&format!("{en} {} ({})", action.id, case.name), &format!("{de} {} ({})", action.id, case.name));
    let (glyph, id, at, label) = match action.kind {
        K::NodalForce | K::NodalMoment => {
            let Some(node) = node(&d.frame_nodes, &action.target_id) else { return };
            let at = layout.frame_point(node.x, node.z);
            if action.kind == K::NodalForce {
                let magnitude = action.fx.hypot(action.fz);
                if magnitude <= 0.0 {
                    return;
                }
                (LoadGlyph::Point { at: at.clone(), direction: v3(action.fx / magnitude, 0.0, action.fz / magnitude), magnitude: Quantity::new(QuantityKind::Force, magnitude), case: case_ref }, format!("{}:force", action.id), at, name("Force", "Kraft"))
            } else {
                if action.my == 0.0 {
                    return;
                }
                (LoadGlyph::Moment { at: at.clone(), axis: Vec3::Y, magnitude: Quantity::new(QuantityKind::Moment, action.my.abs()), case: case_ref }, format!("{}:moment", action.id), at, name("Moment", "Moment"))
            }
        }
        K::BarUniform => {
            let Some(bar) = d.frame_bars.iter().find(|b| b.id == action.target_id) else { return };
            let (Some(a), Some(c)) = (node(&d.frame_nodes, &bar.start_node_id), node(&d.frame_nodes, &bar.end_node_id)) else { return };
            let magnitude = action.fx.hypot(action.fz);
            if magnitude <= 0.0 || action.end_ratio <= action.start_ratio {
                return;
            }
            let at = |ratio: f64| layout.frame_point(a.x + (c.x - a.x) * ratio, a.z + (c.z - a.z) * ratio);
            let (from, to) = (at(action.start_ratio), at(action.end_ratio));
            let middle = at(0.5 * (action.start_ratio + action.end_ratio));
            (LoadGlyph::Line { path: polyline(&[from, to]), direction: v3(action.fx / magnitude, 0.0, action.fz / magnitude), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, magnitude) }, case: case_ref }, format!("{}:line", action.id), middle, name("Line load", "Streckenlast"))
        }
    };
    out.push_tuned(
        Spec { kind: EntityKind::Load, id: id.clone(), collection: "frameActions", geometry: Geometry::Load(glyph), role: MaterialRole::Load { kind }, binding: None, detail: label.clone(), name: label, layer: LAYER_LOADS, handles: super::handles::ids(d, layout, &id), deformation: None },
        &[at],
        |entity| entity.layer = layer,
    );
}

/// 🏗️ The equivalent horizontal force φ·V of the sway imperfection (EN 1993-1-1 5.3.2(7)) at one node under the governing combination, on the toggleable layer `frame-imperfection`. Computed, so it has no grips.
fn imperfection_glyph(layout: &Layout, n: &FrameNode, combination: &str, force: f64, out: &mut Out) {
    if force == 0.0 {
        return;
    }
    let at = layout.frame_point(n.x, n.z);
    let name = loc(&format!("Sway imperfection force at {} ({combination})", n.id), &format!("Schiefstellungskraft an {} ({combination})", n.id));
    out.push(
        Spec {
            kind: EntityKind::Load,
            id: format!("{}:imperfection", n.id),
            collection: "frameNodes",
            geometry: Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: v3(force.signum(), 0.0, 0.0), magnitude: Quantity::new(QuantityKind::Force, force.abs()), case: LoadCaseRef { id: "frame-imperfection".into(), kind: LoadKind::Variable, psi: None } }),
            role: MaterialRole::Load { kind: LoadKind::Variable },
            binding: None,
            detail: name.clone(),
            name,
            layer: LAYER_FRAME_IMPERFECTION,
            handles: Vec::new(),
            deformation: None,
        },
        &[at],
    );
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    let mut shapes: Option<std::sync::Arc<Option<Shapes>>> = None;
    for b in &d.frame_bars {
        if !out.own("frameBars", &b.id) {
            continue;
        }
        let solved = shapes.get_or_insert_with(|| shared_shapes(d));
        bar(input, layout, b, (**solved).as_ref(), out);
    }
    for n in &d.frame_nodes {
        if !out.own("frameNodes", &n.id) {
            continue;
        }
        node_entity(input, layout, n, out);
        let solved = shapes.get_or_insert_with(|| shared_shapes(d));
        if let Some((combination, forces)) = (**solved).as_ref().and_then(|s| s.imperfection.as_ref()) {
            if let Some((_, force)) = forces.iter().find(|f| f.0 == n.id) {
                imperfection_glyph(layout, n, combination, *force, out);
            }
        }
    }
    for action in &d.frame_actions {
        if out.own("frameActions", &action.id) {
            action_glyph(input, layout, action, out);
        }
    }
    if let Some(first) = d.frame_bars.first() {
        if !out.own("frameBars", &first.id) {
            return;
        }
        if let (Some(clause), Some(a)) = (clause_of(input.report, &format!("en1993.1-1.5.2.frame.{}", first.id)), node(&d.frame_nodes, &first.start_node_id)) {
            let at = layout.frame_point(a.x, a.z);
            out.annotate(Annotation::Clause { id: format!("{}:clause", first.id), anchor: entity_ref(EntityKind::Member, &first.id, "frameBars"), clause }, &at);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::sign_runs;

    #[test]
    fn a_sign_changing_moment_splits_at_the_linear_zero_crossing() {
        let runs = sign_runs(&[(0.0, 30.0), (2.0, 10.0), (4.0, -10.0), (6.0, -30.0)]);
        assert_eq!(runs.len(), 2);
        assert!(runs[0].0 && !runs[1].0);
        assert_eq!(runs[0].1.last(), Some(&(3.0, 0.0)));
        assert_eq!(runs[1].1.first(), Some(&(3.0, 0.0)));
        assert!(runs[0].1.iter().all(|p| p.1 >= 0.0) && runs[1].1.iter().all(|p| p.1 <= 0.0));
    }

    #[test]
    fn a_one_signed_moment_stays_one_run_and_a_zero_course_has_none() {
        assert_eq!(sign_runs(&[(0.0, 1.0), (1.0, 2.0), (2.0, 0.0)]).len(), 1);
        assert!(sign_runs(&[(0.0, 0.0), (1.0, 0.0)]).is_empty());
    }

    #[test]
    fn two_crossings_give_three_runs() {
        let runs = sign_runs(&[(0.0, -1.0), (1.0, 1.0), (2.0, 1.0), (3.0, -1.0)]);
        assert_eq!(runs.iter().map(|r| r.0).collect::<Vec<_>>(), vec![false, true, false]);
    }
}
