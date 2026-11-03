//! 🏢️ Buildings: plan footprint, storey slabs, column grid, joints, mass glyphs, centres of mass and stiffness with accidental eccentricity, lateral force arrows, the fundamental mode shape, drift bars and seismic joints between neighbours.

use super::handles::Index;
use super::kit::*;
use super::layout::Layout;
use super::model::{En1998Building, En1998Storey};
use super::response::{annex_params, joint_demand, respond, Mode, Response, Site};
use super::scene::*;
use super::schema::{critical_region, part_1, part_3, part_3_state, wall_plan};
use super::walls;
use super::SceneInput;

/// 🔗 Check families bound to the plan of a building: the modal participation, the plan and elevation regularity claims and the derived period.
const PLAN_CHECKS: [&str; 7] = [".modal.", ".regularity.", ".elevationRegularity.", ".planRegularity", ".t1Given", ".t1Rayleigh", ".combination"];
const MARKER: f64 = 0.5;
const COLUMN_SPACING: f64 = 6.0;
const MODE_AMPLITUDE: f64 = 0.03;
const DRIFT_BAR_UNIT: f64 = 2.0;
const DRIFT_MAP_MARGIN: f64 = 0.15;
const DRIFT_MAP_FIELD: &str = "drift-utilisation";
const DRIFT_MAP_MAX: f64 = 1.2;
const CHORD_FIELD: &str = "chord-rotation-utilisation";
const CHORD_FIELD_MAX: f64 = 1.2;
const DC_FIELD: &str = "assessment-dc";
const DC_FIELD_MAX: f64 = 1.2;

fn role_of(building: &En1998Building) -> MaterialRole {
    match building.systems.first().map(|s| s.material) {
        Some(semio_s_artifact_norm_en1998_model::StructuralMaterial::Steel | semio_s_artifact_norm_en1998_model::StructuralMaterial::Composite) => MaterialRole::Steel,
        Some(semio_s_artifact_norm_en1998_model::StructuralMaterial::Timber) => MaterialRole::Timber,
        Some(semio_s_artifact_norm_en1998_model::StructuralMaterial::Masonry) => MaterialRole::Masonry,
        _ => MaterialRole::Concrete,
    }
}

pub fn column_size(building: &En1998Building) -> f64 {
    building.members.iter().find(|m| m.role == super::model::MemberKind::Column).map(|m| m.min_dimension_m).filter(|v| *v > 0.0).unwrap_or(0.4)
}

fn storey_extent(b: &En1998Building, index: usize) -> (f64, f64) {
    let width = b.plan_width_m.max(0.1) * (1.0 - b.setback_ratio.clamp(0.0, 0.9)).powi(index as i32);
    ((b.plan_width_m.max(0.1) - width) / 2.0, width)
}

fn grid(extent: f64) -> usize {
    ((extent / COLUMN_SPACING).ceil() as usize + 1).clamp(2, 6)
}

fn storey_binding(input: &SceneInput, bid: &str, sid: &str) -> Option<UtilisationBinding> {
    let tails = [format!(".storeyForces.{sid}"), format!(".drift.{sid}"), format!(".pdelta.{sid}")];
    binding_where(input.report, |c| c.subject.entity_id == bid && tails.iter().any(|t| c.id.ends_with(t.as_str())))
}

fn mode_overlay(mode: &Mode, scale: f64, index: usize, whole: bool) -> Option<DeformationOverlay> {
    let top = *mode.shape.get(index)?;
    let base = if whole { top } else if index == 0 { 0.0 } else { mode.shape[index - 1] };
    let at = |value: f64| v3(if mode.along_y { 0.0 } else { mode.amplitude_m * value }, if mode.along_y { mode.amplitude_m * value } else { 0.0 }, 0.0);
    let samples = vec![DisplacementSample { at: 0.0, delta: at(base) }, DisplacementSample { at: 1.0, delta: at(top) }];
    Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: mode.index, frequency: Some(Quantity::new(QuantityKind::Frequency, 1.0 / mode.period)), participation: mode.gamma, scale, samples })
}

fn mode_scale(mode: &Mode, total_height: f64) -> f64 {
    let peak = mode.shape.iter().fold(0.0_f64, |a, v| a.max(v.abs())) * mode.amplitude_m.abs();
    if peak > 1e-12 { (MODE_AMPLITUDE * total_height / peak).max(1.0) } else { 1.0 }
}

fn mode_layer(mode: &Mode) -> &'static str {
    match (mode.along_y, mode.index) {
        (false, 1) => LAYER_MODE_X1,
        (false, 2) => LAYER_MODE_X2,
        (false, _) => LAYER_MODE_X3,
        (true, 1) => LAYER_MODE_Y1,
        (true, 2) => LAYER_MODE_Y2,
        (true, _) => LAYER_MODE_Y3,
    }
}

fn mode_ghosts(b: &En1998Building, r: &Response, o: &Vec3, sp: &str, index: usize, top: f64, out: &mut Out) {
    let total = r.elevations.last().copied().unwrap_or(0.0);
    for mode in r.modes.iter().filter(|m| m.index <= 3) {
        let origin = v3(o.x, o.y, o.z + top - SLAB_THICKNESS);
        let tag = format!("mode-{}{}", if mode.along_y { "y" } else { "x" }, mode.index);
        let id = format!("{}:{}:{tag}", b.id, b.storeys[index].id);
        let mut spec = Spec::new(EntityKind::Plate, id, sp, Geometry::Plate(PlateGeometry { frame: world_frame(origin.clone()), outline: rect_ring(b.plan_width_m.max(0.1), b.plan_length_m.max(0.1)), thickness: 0.05, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Neutral, loc(&format!("Mode {} ({}) of {}: T = {:.3} s, effective mass {:.0} %", mode.index, if mode.along_y { "y" } else { "x" }, b.storeys[index].id, mode.period, mode.m_eff_ratio * 100.0), &format!("Eigenform {} ({}) von {}: T = {:.3} s, effektive Masse {:.0} %", mode.index, if mode.along_y { "y" } else { "x" }, b.storeys[index].id, mode.period, mode.m_eff_ratio * 100.0)), mode_layer(mode));
        spec.opacity = 0.35;
        spec.deformation = mode_overlay(mode, mode_scale(mode, total), index, true);
        out.push(spec, &[origin, v3(o.x + b.plan_width_m, o.y + b.plan_length_m, o.z + top)]);
    }
}

fn marker(kind: EntityKind, id: String, path: &str, at: &Vec3, role: MaterialRole, name: LocalizedCopy, handles: Vec<String>) -> Spec {
    let mut spec = Spec::new(kind, id, path, box_solid(v3(at.x - MARKER / 2.0, at.y - MARKER / 2.0, at.z), v3(MARKER, MARKER, MARKER)), role, name, LAYER_MASSES);
    spec.handles = handles;
    spec
}

pub fn project(input: &SceneInput, layout: &Layout, site: &Site, handles: &Index, out: &mut Out) {
    let d = input.document;
    for building in &d.buildings {
        if !out.own("buildings", &building.id) {
            continue;
        }
        let response = respond(site, building);
        project_building(input, layout, site, handles, building, &response, out);
    }
    seismic_joints(input, layout, out);
}

fn project_building(input: &SceneInput, layout: &Layout, site: &Site, handles: &Index, b: &En1998Building, r: &Response, out: &mut Out) {
    let o = layout.origin(&b.id);
    let bp = path_of("buildings", &b.id);
    let role = role_of(b);
    let height = r.elevations.last().copied().unwrap_or(0.0);
    let plan = Spec { handles: handles.of(&b.id), ..Spec::new(EntityKind::Plate, b.id.clone(), bp.clone(), Geometry::Plate(PlateGeometry { frame: world_frame(o.clone()), outline: rect_ring(b.plan_width_m.max(0.1), b.plan_length_m.max(0.1)), thickness: 0.05, offset: -0.05, stiffeners: Vec::new() }), MaterialRole::Neutral, loc(&format!("Plan of {}", b.name), &format!("Grundriss {}", b.name)), LAYER_STOREYS) };
    let mut plan = plan;
    plan.binding = binding_where(input.report, |c| c.subject.entity_id == b.id && c.id.starts_with(&format!("en1998.1.{}.", b.id)) && PLAN_CHECKS.iter().any(|tag| c.id.contains(tag)));
    out.push(plan, &[o.clone(), v3(o.x + b.plan_width_m, o.y + b.plan_length_m, o.z)]);
    for (from, to, field, value) in [
        (o.clone(), v3(o.x + b.plan_width_m, o.y, o.z), "planWidthM", b.plan_width_m),
        (o.clone(), v3(o.x, o.y + b.plan_length_m, o.z), "planLengthM", b.plan_length_m),
    ] {
        let offset = if field == "planWidthM" { v3(0.0, -1.5, 0.0) } else { v3(-1.5, 0.0, 0.0) };
        out.annotate(Annotation::Dimension { id: format!("{}:{field}", b.id), from: from.clone(), to, offset, value: Quantity::new(QuantityKind::Length, value), binding: Some(FieldBinding::new(format!("{bp}.{field}"), Affine::IDENTITY, "m")) }, &from);
    }
    let size = column_size(b);
    let (nx, ny) = (grid(b.plan_width_m), grid(b.plan_length_m));
    let mut previous = 0.0;
    for (index, storey) in b.storeys.iter().enumerate() {
        let top = r.elevations[index];
        let sp = format!("{bp}.storeys[id={}]", storey.id);
        let binding = storey_binding(input, &b.id, &storey.id);
        let soft = soft_storey(b, index);
        slab(input, handles, b, storey, index, r, &o, top, &sp, &binding, soft, &role, out);
        columns(b, storey, &o, previous, top, &sp, &binding, (nx, ny, size), storey_extent(b, index), &role, out);
        joints(input, b, storey, &o, top, &sp, (nx, ny, size), storey_extent(b, index), out);
        masses_and_centres(handles, b, storey, index, r, &o, top, &sp, out);
        mode_ghosts(b, r, &o, &sp, index, top, out);
        forces(b, storey, index, r, &o, top, &sp, out);
        drift_bars(input, b, storey, r, &o, previous, top, &sp, out);
        previous = top;
    }
    drift_map(input, b, r, &o, &bp, out);
    walls::wall_bodies(input, handles, b, r, &o, &bp, out);
    masonry_walls(input, site, b, r, &o, out);
    recess(input, b, &o, &bp, out);
    nonstructural(input, site, handles, b, r, &o, &bp, out);
    let roof = v3(o.x + b.plan_width_m / 2.0, o.y + b.plan_length_m / 2.0, o.z + height);
    for (index, a) in input.document.assessments.iter().filter(|a| a.supported_building_id == b.id).enumerate() {
        assessed_member(input, handles, b, a, &assessed_origin(&o, b, index), out);
        let binding = binding_prefix(input.report, &format!("en1998.3.{}.", a.id));
        let at = v3(roof.x, roof.y, roof.z + 1.5);
        let text = loc(&format!("Assessment {}: limit state {}, knowledge level {}", a.id, a.limit_state.token().to_uppercase(), format!("{:?}", a.knowledge_level).to_uppercase()), &format!("Bewertung {}: Grenzzustand {}, Kenntnisstand {}", a.id, a.limit_state.token().to_uppercase(), format!("{:?}", a.knowledge_level).to_uppercase()));
        out.annotate(Annotation::Label { id: format!("{}:assessment-{}", b.id, a.id), anchor: at.clone(), text, tone: tone_of(&binding) }, &at);
    }
    critical_regions(input, handles, site, b, r, &o, out);
}

/// 🧱 Per storey and plan direction a label with the wall-area density Σ l·t / A of the masonry wall rows against the required share of a simple masonry building (EN 1998-1 §9.7), warning below it; the wall bodies come from [`walls::wall_bodies`].
fn masonry_walls(input: &SceneInput, site: &Site, b: &En1998Building, r: &Response, o: &Vec3, out: &mut Out) {
    let walls = part_1::masonry_walls_of(&b.members);
    if walls.is_empty() {
        return;
    }
    let limit = part_1::masonry_wall_ratio_limit(site.a_gr, input.document.site.importance_class);
    let area = b.plan_width_m * b.plan_length_m;
    for storey in 1..=b.storeys.len() {
        let top = r.elevations[storey - 1];
        let storey_id = b.storeys[storey - 1].id.as_str();
        for along_y in [false, true] {
            let density = part_1::masonry_wall_density(&walls, area, along_y, storey_id);
            let direction = if along_y { "y" } else { "x" };
            let anchor = v3(o.x + if along_y { b.plan_width_m + 0.5 } else { -0.5 }, o.y + if along_y { 0.0 } else { b.plan_length_m + 0.5 }, o.z + top);
            let text = |de: bool| format!("{} {storey_id}, {} {direction}: \u{3a3} l\u{b7}t / A = {:.2} % {} {:.2} %", if de { "Geschoss" } else { "Storey" }, if de { "Richtung" } else { "direction" }, density * 100.0, if de { "erforderlich" } else { "required" }, limit * 100.0);
            out.annotate(Annotation::Label { id: format!("{}:wall-density:{direction}:{storey_id}", b.id), anchor: anchor.clone(), text: loc(&text(false), &text(true)), tone: if density + 1e-12 >= limit { Tone::Info } else { Tone::Warning } }, &anchor);
        }
    }
}

fn recess(input: &SceneInput, b: &En1998Building, o: &Vec3, bp: &str, out: &mut Out) {
    if b.plan_envelope_gap_ratio <= 0.0 {
        return;
    }
    let side = (b.plan_envelope_gap_ratio * b.plan_width_m * b.plan_length_m).sqrt();
    let (w, l) = (side.min(b.plan_width_m).max(0.1), side.min(b.plan_length_m).max(0.1));
    let origin = v3(o.x + b.plan_width_m - w, o.y + b.plan_length_m - l, o.z + 0.01);
    let mut spec = Spec::new(EntityKind::Plate, format!("{}:recess", b.id), bp, Geometry::Plate(PlateGeometry { frame: world_frame(origin.clone()), outline: rect_ring(w, l), thickness: 0.05, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Neutral, loc(&format!("Plan recess of {} ({:.1} % of the floor area)", b.name, b.plan_envelope_gap_ratio * 100.0), &format!("Grundrissaussparung von {} ({:.1} % der Geschossfl\u{e4}che)", b.name, b.plan_envelope_gap_ratio * 100.0)), LAYER_STOREYS);
    spec.binding = binding_exact(input.report, &format!("en1998.1.{}.plan.envelope", b.id));
    spec.opacity = 0.8;
    out.push(spec, &[origin, v3(o.x + b.plan_width_m, o.y + b.plan_length_m, o.z)]);
}

fn nonstructural(input: &SceneInput, site: &Site, handles: &Index, b: &En1998Building, r: &Response, o: &Vec3, bp: &str, out: &mut Out) {
    if b.nonstructural_weight_n <= 0.0 {
        return;
    }
    let total = r.elevations.last().copied().unwrap_or(0.0);
    let z = b.nonstructural_height_m.clamp(0.0, total.max(0.1));
    let at = v3(o.x + b.plan_width_m / 2.0, o.y + b.plan_length_m / 2.0, o.z + z);
    let f_a = part_1::non_structural_force_n(site.gamma_i * site.a_gr / 9.81, site.s, z, total, b.nonstructural_period_s, r.t1, b.nonstructural_weight_n, 1.0, b.nonstructural_q);
    let id = format!("{}:nonstructural", b.id);
    let mut spec = Spec::new(EntityKind::Solid, id.clone(), bp, box_solid(v3(at.x - 0.6, at.y - 0.6, at.z), v3(1.2, 1.2, 1.2)), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Non-structural element {:.0} kN at {:.1} m, F_a = {:.1} kN", b.nonstructural_weight_n / 1000.0, z, f_a / 1000.0), &format!("Nichttragendes Bauteil {:.0} kN auf {:.1} m, F_a = {:.1} kN", b.nonstructural_weight_n / 1000.0, z, f_a / 1000.0)), LAYER_MASSES);
    spec.binding = binding_exact(input.report, &format!("en1998.1.{}.nonstructural.force", b.id));
    spec.handles = handles.of(&id);
    out.push(spec, &[at.clone()]);
    let case = LoadCaseRef { id: "seismic-nonstructural".into(), kind: LoadKind::Seismic, psi: None };
    out.push(Spec::new(EntityKind::Load, format!("{id}:force"), bp, Geometry::Load(LoadGlyph::Point { at: v3(at.x, at.y, at.z + 0.6), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, f_a), case }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Seismic force F_a = {:.1} kN", f_a / 1000.0), &format!("Erdbebenkraft F_a = {:.1} kN", f_a / 1000.0)), LAYER_FORCES), &[at]);
}

fn soft_storey(b: &En1998Building, index: usize) -> bool {
    b.storeys.get(index + 1).is_some_and(|upper| part_1::soft_storey_utilization(b.storeys[index].stiffness_x, upper.stiffness_x) > 1.0)
}

#[allow(clippy::too_many_arguments)]
fn slab(input: &SceneInput, handles: &Index, b: &En1998Building, s: &En1998Storey, index: usize, r: &Response, o: &Vec3, top: f64, sp: &str, binding: &Option<UtilisationBinding>, soft: bool, role: &MaterialRole, out: &mut Out) {
    let id = format!("{}:{}", b.id, s.id);
    let (shift, width) = storey_extent(b, index);
    let origin = v3(o.x + shift, o.y, o.z + top - SLAB_THICKNESS);
    let name = loc(&format!("Storey {}", s.id), &format!("Geschoss {}", s.id));
    let mut spec = Spec::new(EntityKind::Plate, id.clone(), sp, Geometry::Plate(PlateGeometry { frame: world_frame(origin.clone()), outline: rect_ring(width, b.plan_length_m.max(0.1)), thickness: SLAB_THICKNESS, offset: 0.0, stiffeners: Vec::new() }), role.clone(), name.clone(), LAYER_STOREYS);
    spec.binding = binding.clone();
    spec.style = soft.then_some(StyleState::Error);
    spec.opacity = 0.55;
    spec.handles = handles.of(&id);
    spec.detail = loc(&format!("Storey {} at {:.2} m, height {:.2} m, seismic mass {:.0} t{}", s.id, top, s.height_m, r.masses[index] / 1000.0, if soft { ", soft storey" } else { "" }), &format!("Geschoss {} auf {:.2} m, H\u{f6}he {:.2} m, Erdbebenmasse {:.0} t{}", s.id, top, s.height_m, r.masses[index] / 1000.0, if soft { ", weiches Geschoss" } else { "" }));
    out.push(spec, &[origin.clone(), v3(o.x + shift + width, o.y + b.plan_length_m, o.z + top)]);
    if index == 1 && b.setback_ratio > 0.0 {
        let from = v3(o.x, o.y - 0.5, o.z + top);
        out.annotate(Annotation::Dimension { id: format!("{}:setback", b.id), from: from.clone(), to: v3(o.x + shift, from.y, from.z), offset: v3(0.0, -1.0, 0.0), value: Quantity::new(QuantityKind::Length, shift), binding: Some(FieldBinding::new(format!("{}.setbackRatio", path_of("buildings", &b.id)), Affine { base: 0.0, k: 2.0 / b.plan_width_m.max(0.1) }, "m")) }, &from);
    }
    let entity = entity_ref(EntityKind::Plate, &id, sp);
    let corner = v3(o.x, o.y, o.z + top);
    out.annotate(Annotation::Label { id: format!("{id}:label"), anchor: corner.clone(), text: loc(&format!("{} \u{b7} {:.0} t", s.id, r.masses[index] / 1000.0), &format!("{} \u{b7} {:.0} t", s.id, r.masses[index] / 1000.0)), tone: tone_of(binding) }, &corner);
    let from = v3(o.x - 1.5, o.y, o.z + top - s.height_m);
    let to = v3(from.x, from.y, o.z + top);
    out.annotate(Annotation::Dimension { id: format!("{id}:height"), from: from.clone(), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, s.height_m), binding: Some(FieldBinding::new(format!("{sp}.heightM"), Affine::IDENTITY, "m")) }, &from);
    if let Some(clause) = binding.as_ref().and_then(|x| clause_of(input.report, &x.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{id}:clause"), anchor: entity.clone(), clause }, &corner);
    }
    if soft || binding.as_ref().is_some_and(|x| x.status == scene::CheckStatus::Fail) {
        out.annotate(Annotation::FailureGlyph { id: format!("{id}:failure"), at: entity, mode: FailureMode::Other, extent: None }, &corner);
    }
}

#[allow(clippy::too_many_arguments)]
fn columns(b: &En1998Building, s: &En1998Storey, o: &Vec3, from: f64, top: f64, sp: &str, binding: &Option<UtilisationBinding>, grid: (usize, usize, f64), ext: (f64, f64), role: &MaterialRole, out: &mut Out) {
    let (nx, ny, size) = grid;
    let section = SectionShape::parametric(ProfileKind::Rect, &[("h", size), ("b", size)]);
    for i in 0..nx {
        for j in 0..ny {
            let x = o.x + ext.0 + size / 2.0 + i as f64 * (ext.1 - size).max(0.0) / (nx - 1) as f64;
            let y = o.y + size / 2.0 + j as f64 * (b.plan_length_m - size).max(0.0) / (ny - 1) as f64;
            let start = v3(x, y, o.z + from);
            let end = v3(x, y, o.z + top - SLAB_THICKNESS);
            let frame = member_frame(&start, &Vec3::Z);
            let mut spec = Spec::new(EntityKind::Member, format!("{}:{}:col-{i}-{j}", b.id, s.id), sp, Geometry::Member(MemberGeometry { axis: polyline(&[start.clone(), end.clone()]), frame, section: section.clone(), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Rigid, stations: Vec::new() }), role.clone(), loc(&format!("Column {i}-{j} of {}", s.id), &format!("St\u{fc}tze {i}-{j} in {}", s.id)), LAYER_COLUMNS);
            spec.binding = binding.clone();
            out.push(spec, &[start, end]);
        }
    }
}

fn joints(input: &SceneInput, b: &En1998Building, s: &En1998Storey, o: &Vec3, top: f64, sp: &str, grid: (usize, usize, f64), ext: (f64, f64), out: &mut Out) {
    if input.document.joints.iter().any(|j| j.building_id == b.id) {
        return;
    }
    let (nx, ny, size) = grid;
    let binding = binding_exact(input.report, &format!("en1998.1.{}.capacityDesign.nodes", b.id)).or_else(|| binding_where(input.report, |c| c.subject.entity_id == b.id && c.id.contains(".member.")));
    let mut first: Option<(String, Vec3)> = None;
    for i in 0..nx {
        for j in 0..ny {
            let x = o.x + ext.0 + size / 2.0 + i as f64 * (ext.1 - size).max(0.0) / (nx - 1) as f64;
            let y = o.y + size / 2.0 + j as f64 * (b.plan_length_m - size).max(0.0) / (ny - 1) as f64;
            let at = v3(x - size * 0.75, y - size * 0.75, o.z + top - SLAB_THICKNESS - size * 1.5);
            let id = format!("{}:{}:joint-{i}-{j}", b.id, s.id);
            let mut spec = Spec::new(EntityKind::Solid, id.clone(), sp, box_solid(at.clone(), v3(size * 1.5, size * 1.5, size * 1.5)), MaterialRole::Neutral, loc(&format!("Joint {i}-{j} of {}", s.id), &format!("Knoten {i}-{j} in {}", s.id)), LAYER_JOINTS);
            spec.binding = binding.clone();
            spec.opacity = 0.7;
            spec.detail = loc("Capacity design node: sum of column resistances at least 1.3 times the sum of beam resistances", "Kapazit\u{e4}tsbemessung am Knoten: Summe der St\u{fc}tzenwiderst\u{e4}nde mindestens 1,3-fache Summe der Balkenwiderst\u{e4}nde");
            out.push(spec, &[at.clone()]);
            first.get_or_insert((id, at));
        }
    }
    if let (Some((id, at)), Some(clause)) = (first, binding.as_ref().and_then(|x| clause_of(input.report, &x.check_id))) {
        out.annotate(Annotation::Clause { id: format!("{id}:clause"), anchor: entity_ref(EntityKind::Solid, &id, sp), clause }, &at);
    }
}

#[allow(clippy::too_many_arguments)]
fn masses_and_centres(handles: &Index, b: &En1998Building, s: &En1998Storey, index: usize, r: &Response, o: &Vec3, top: f64, sp: &str, out: &mut Out) {
    let com = v3(o.x + s.centre_of_mass_x_m, o.y + s.centre_of_mass_y_m, o.z + top);
    let (cos_x, cos_y, from_walls) = wall_plan::stiffness_centre_of(&b.members, s);
    let cos = v3(o.x + cos_x, o.y + cos_y, o.z + top);
    let id = format!("{}:{}", b.id, s.id);
    let mass_id = format!("{id}:mass");
    let mut mass = Spec::new(EntityKind::Load, mass_id.clone(), sp, Geometry::Load(LoadGlyph::Inertial { origin: com.clone(), direction: Vec3::X, masses: vec![PointMass { at: com.clone(), mass: Quantity::new(QuantityKind::Mass, r.masses[index]) }], case: load_case("seismic-mass") }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Seismic mass {:.0} t of {}", r.masses[index] / 1000.0, s.id), &format!("Erdbebenmasse {:.0} t von {}", r.masses[index] / 1000.0, s.id)), LAYER_MASSES);
    mass.handles = handles.of(&mass_id);
    out.push(mass, &[com.clone()]);
    for mark in super::structure_views::variables::marks(&id, sp, &s.variables, s.permanent_gk_n, &com) {
        let mut glyph = Spec::new(EntityKind::Load, mark.id.clone(), mark.path.clone(), mark.glyph(), MaterialRole::Load { kind: LoadKind::Variable }, mark.name(), LAYER_MASSES);
        glyph.handles = handles.of(&mark.id);
        out.push(glyph, &[mark.at.clone()]);
    }
    out.push(marker(EntityKind::Solid, format!("{id}:com"), sp, &com, MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Centre of mass of {}", s.id), &format!("Massenmittelpunkt von {}", s.id)), handles.of(&format!("{id}:com"))), &[com.clone()]);
    out.push(marker(EntityKind::Solid, format!("{id}:cos"), sp, &cos, MaterialRole::Support, loc(&format!("Centre of stiffness of {}", s.id), &format!("Steifigkeitsmittelpunkt von {}", s.id)), handles.of(&format!("{id}:cos"))), &[cos.clone()]);
    if from_walls {
        walls::stiffness_centre(b, o, top, index, out);
    }
    if com.distance(&cos) > 1e-6 {
        out.annotate(Annotation::Dimension { id: format!("{id}:e0"), from: com.clone(), to: cos.clone(), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, com.distance(&cos)), binding: None }, &com);
    }
    for (tag, plan, field, axis) in [("", b.plan_length_m, "y", Vec3::Y), ("x-", b.plan_width_m, "x", Vec3::X)] {
        let e_ai = b.accidental_eccentricity_ratio.max(0.0) * plan;
        if e_ai <= 1e-6 || plan <= 0.0 {
            continue;
        }
        for (sign_tag, sign) in [("plus", 1.0), ("minus", -1.0)] {
            let at = add(&com, &mul(&axis, sign * e_ai));
            let text = format!("{}e_ai,{field} = {:.2} m", if sign > 0.0 { "+" } else { "\u{2212}" }, e_ai);
            out.annotate(Annotation::Label { id: format!("{id}:eai-{tag}{sign_tag}"), anchor: at.clone(), text: loc(&text, &text), tone: Tone::Neutral }, &at);
        }
        out.annotate(Annotation::Dimension { id: format!("{id}:eai{}", if tag.is_empty() { String::new() } else { format!("-{field}") }), from: com.clone(), to: add(&com, &mul(&axis, e_ai)), offset: v3(0.4, 0.4, 0.0), value: Quantity::new(QuantityKind::Length, e_ai), binding: Some(FieldBinding::new(format!("{}.accidentalEccentricityRatio", path_of("buildings", &b.id)), Affine { base: 0.0, k: 1.0 / plan }, "m")) }, &com);
    }
}

#[allow(clippy::too_many_arguments)]
fn forces(b: &En1998Building, s: &En1998Storey, index: usize, r: &Response, o: &Vec3, top: f64, sp: &str, out: &mut Out) {
    for system in &r.systems {
        let force = system.forces.get(index).copied().unwrap_or(0.0);
        if force <= 0.0 {
            continue;
        }
        let e_ai = b.accidental_eccentricity_ratio.max(0.0) * if system.along_y { b.plan_width_m } else { b.plan_length_m };
        let (direction, shift) = if system.along_y { (Vec3::Y, v3(e_ai, 0.0, 0.0)) } else { (Vec3::X, v3(0.0, e_ai, 0.0)) };
        let at = add(&v3(o.x + s.centre_of_mass_x_m, o.y + s.centre_of_mass_y_m, o.z + top), &shift);
        let case = LoadCaseRef { id: format!("seismic-{}", system.id), kind: LoadKind::Seismic, psi: None };
        out.push(Spec::new(EntityKind::Load, format!("{}:{}:force-{}", b.id, s.id, system.id), sp, Geometry::Load(LoadGlyph::Point { at: at.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, force), case }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Lateral force {:.0} kN on {} ({})", force / 1000.0, s.id, system.id), &format!("Horizontalkraft {:.0} kN auf {} ({})", force / 1000.0, s.id, system.id)), LAYER_FORCES), &[at]);
    }
}

#[allow(clippy::too_many_arguments)]
fn drift_bars(input: &SceneInput, b: &En1998Building, s: &En1998Storey, r: &Response, o: &Vec3, from: f64, top: f64, sp: &str, out: &mut Out) {
    for system in &r.systems {
        let binding = binding_exact(input.report, &format!("en1998.1.{}.{}.drift.{}", b.id, system.id, s.id));
        let utilisation = binding.as_ref().map_or(0.0, |x| x.ratio).min(3.0);
        let length = (utilisation * DRIFT_BAR_UNIT).max(0.05);
        let h = (top - from) * 0.6;
        let z = o.z + from + (top - from) * 0.2;
        let (geometry, anchor) = if system.along_y {
            let origin = v3(o.x + b.plan_width_m + 1.0, o.y, z);
            (Geometry::Solid(SolidGeometry { shape: projection_kit::solid::box_from_corner(&Frame { origin: origin.clone(), x: Vec3::Y, y: v3(-1.0, 0.0, 0.0), z: Vec3::Z }, v3(length, 0.3, h)) }), v3(origin.x, origin.y + length, z + h))
        } else {
            let origin = v3(o.x, o.y + b.plan_length_m + 1.0, z);
            (box_solid(origin.clone(), v3(length, 0.3, h)), v3(origin.x + length, origin.y, z + h))
        };
        let mut spec = Spec::new(EntityKind::Solid, format!("{}:{}:drift-{}", b.id, s.id, system.id), sp, geometry, MaterialRole::Neutral, loc(&format!("Drift utilisation of {} ({})", s.id, system.id), &format!("Verschiebungsausnutzung von {} ({})", s.id, system.id)), LAYER_DRIFT);
        spec.binding = binding.clone();
        out.push(spec, &[anchor.clone()]);
        out.annotate(Annotation::Label { id: format!("{}:{}:drift-{}:label", b.id, s.id, system.id), anchor: anchor.clone(), text: loc(&format!("d\u{b7}\u{3bd}/(\u{3b8}\u{b7}h) = {utilisation:.2}"), &format!("d\u{b7}\u{3bd}/(\u{3b8}\u{b7}h) = {utilisation:.2}")), tone: tone_of(&binding) }, &anchor);
    }
}

fn seismic_joints(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for pair in d.buildings.windows(2) {
        if !out.own("buildings", &pair[0].id) {
            continue;
        }
        let (a, b) = (&pair[0], &pair[1]);
        let (oa, ob) = (layout.origin(&a.id), layout.origin(&b.id));
        let gap = ob.x - (oa.x + a.plan_width_m);
        if gap <= 0.0 {
            continue;
        }
        let required = joint_demand(a, b);
        let id = format!("{}:{}:joint", a.id, b.id);
        let origin = v3(oa.x + a.plan_width_m, oa.y, oa.z);
        let mut spec = Spec::new(EntityKind::Plate, id.clone(), path_of("buildings", &a.id), Geometry::Plate(PlateGeometry { frame: world_frame(origin.clone()), outline: rect_ring(gap, a.plan_length_m.max(b.plan_length_m).max(0.1)), thickness: 0.05, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Neutral, loc(&format!("Seismic joint between {} and {}", a.id, b.id), &format!("Erdbebenfuge zwischen {} und {}", a.id, b.id)), LAYER_SEISMIC_JOINTS);
        spec.style = (gap < required).then_some(StyleState::Error);
        spec.detail = loc(&format!("Gap {:.3} m, required {:.3} m", gap, required), &format!("Fuge {:.3} m, erforderlich {:.3} m", gap, required));
        out.push(spec, &[origin.clone(), v3(ob.x, origin.y + a.plan_length_m, origin.z)]);
        out.annotate(Annotation::Dimension { id: format!("{id}:gap"), from: origin.clone(), to: v3(ob.x, origin.y, origin.z), offset: v3(0.0, -2.5, 0.0), value: Quantity::new(QuantityKind::Length, gap), binding: (a.joint_gap_m > 0.0).then(|| FieldBinding::new(format!("{}.jointGapM", path_of("buildings", &a.id)), Affine::IDENTITY, "m")) }, &origin);
        let at = v3(origin.x + gap / 2.0, origin.y + 1.0, origin.z);
        out.annotate(Annotation::Label { id: format!("{id}:label"), anchor: at.clone(), text: loc(&format!("d \u{2265} {:.0} mm", required * 1000.0), &format!("d \u{2265} {:.0} mm", required * 1000.0)), tone: if gap < required { Tone::Error } else { Tone::Info } }, &at);
    }
}

/// 🌡️ The 3D storey drift heat map: one translucent prism per storey around the building, coloured by the scalar field `drift-utilisation` (the largest d_r·ν/(θ·h) over the systems, blended into the neighbour storeys so the ramp is continuous), with the drift in x and y as further fields. The drift profile diagram carries the same values as its table twin.
fn drift_map(input: &SceneInput, b: &En1998Building, r: &Response, o: &Vec3, bp: &str, out: &mut Out) {
    let worst: Vec<Option<UtilisationBinding>> = b.storeys.iter().map(|s| {
        r.systems.iter().filter_map(|system| binding_exact(input.report, &format!("en1998.1.{}.{}.drift.{}", b.id, system.id, s.id))).fold(None, |best: Option<UtilisationBinding>, next| if best.as_ref().is_some_and(|x| x.ratio >= next.ratio) { best } else { Some(next) })
    }).collect();
    let utilisation: Vec<f64> = worst.iter().map(|w| w.as_ref().map_or(0.0, |x| x.ratio.min(3.0))).collect();
    if utilisation.iter().all(|u| *u <= 0.0) {
        return;
    }
    let mut previous = 0.0;
    for (index, storey) in b.storeys.iter().enumerate() {
        let top = r.elevations[index];
        let u = utilisation[index];
        let below = if index == 0 { u } else { 0.5 * (u + utilisation[index - 1]) };
        let above = utilisation.get(index + 1).map_or(u, |next| 0.5 * (u + next));
        let origin = v3(o.x - DRIFT_MAP_MARGIN, o.y - DRIFT_MAP_MARGIN, o.z + previous);
        let geometry = Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: world_frame(origin.clone()), base: rect_ring(b.plan_width_m.max(0.1) + 2.0 * DRIFT_MAP_MARGIN, b.plan_length_m.max(0.1) + 2.0 * DRIFT_MAP_MARGIN), height: (top - previous).max(0.1) } });
        let sp = format!("{bp}.storeys[id={}]", storey.id);
        let name = loc(&format!("Drift heat map of {}: {u:.2}", storey.id), &format!("Drift-Heatmap von {}: {u:.2}", storey.id));
        let mut spec = Spec::new(EntityKind::Solid, format!("{}:{}:drift-map", b.id, storey.id), sp, geometry, MaterialRole::Neutral, name, LAYER_DRIFT_MAP);
        spec.binding = worst[index].clone();
        spec.opacity = 0.4;
        let field = |id: &str, unit: QuantityKind, values: Vec<f64>| ScalarField { id: id.into(), unit, samples: FieldSamples::Vertex { values } };
        let ring = |bottom: f64, top: f64| [vec![bottom; 4], vec![top; 4]].concat();
        spec.channel = Some(ColourChannel::Fields { active: DRIFT_MAP_FIELD.into(), fields: vec![field(DRIFT_MAP_FIELD, QuantityKind::Dimensionless, ring(below, above)), field("drift-x", QuantityKind::Length, ring(storey.drift_x_m, storey.drift_x_m)), field("drift-y", QuantityKind::Length, ring(storey.drift_y_m, storey.drift_y_m))] });
        spec.detail = loc(&format!("Storey {} drift utilisation {u:.2} (x {:.1} mm, y {:.1} mm)", storey.id, storey.drift_x_m * 1000.0, storey.drift_y_m * 1000.0), &format!("Verschiebungsausnutzung von Geschoss {} {u:.2} (x {:.1} mm, y {:.1} mm)", storey.id, storey.drift_x_m * 1000.0, storey.drift_y_m * 1000.0));
        out.push(spec, &[origin.clone(), v3(origin.x + b.plan_width_m + 2.0 * DRIFT_MAP_MARGIN, origin.y + b.plan_length_m + 2.0 * DRIFT_MAP_MARGIN, o.z + top)]);
        previous = top;
    }
    if !out.legends.iter().any(|l| l.id == DRIFT_MAP_FIELD) {
        out.legends.push(Legend { id: DRIFT_MAP_FIELD.into(), title: loc("Storey drift utilisation d\u{b7}\u{3bd}/(\u{3b8}\u{b7}h)", "Ausnutzung der Stockwerksverschiebung d\u{b7}\u{3bd}/(\u{3b8}\u{b7}h)"), scale: LegendScale::Scalar { field_id: DRIFT_MAP_FIELD.into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: DRIFT_MAP_MAX }, unit: QuantityKind::Dimensionless } });
    }
}

/// 🏛️ The assessed member of an assessment as a cantilever of the shear span L_V with the section h by b: fixed end, the chord rotation demand as static deflection, draggable dimensions for L_V, h and the stirrup spacing, and the Annex A capacities in the label.
fn assessed_member(input: &SceneInput, handles: &Index, b: &En1998Building, a: &super::model::En1998Assessment, base: &Vec3, out: &mut Out) {
    let Some(member) = part_3::RcMember::of(a) else { return };
    let path = path_of("assessments", &a.id);
    let id = format!("{}:member", a.id);
    let tip = v3(base.x + member.l_v, base.y, base.z);
    let binding = governing_binding(input, a);
    let demand = a.chord_rotation_demand_rad;
    let geometry = Geometry::Member(MemberGeometry { axis: polyline(&[base.clone(), tip.clone()]), frame: member_frame(base, &Vec3::X), section: SectionShape::parametric(ProfileKind::Rect, &[("h", member.h), ("b", member.b)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Free, stations: Vec::new() });
    let mut spec = Spec::new(EntityKind::Member, id.clone(), path.clone(), geometry, MaterialRole::Concrete, loc(&format!("Assessed member of {} ({})", a.id, b.name), &format!("Bewertetes Bauteil von {} ({})", a.id, b.name)), LAYER_ASSESSED);
    spec.binding = binding.clone();
    spec.handles = handles.of(&id);
    spec.detail = loc(&format!("Section {:.2} m by {:.2} m, shear span {:.2} m, N = {:.0} kN, chord rotation demand {demand:.4} rad", member.h, member.b, member.l_v, member.n / 1000.0), &format!("Querschnitt {:.2} m mal {:.2} m, Schubspannweite {:.2} m, N = {:.0} kN, Sehnendrehwinkel-Einwirkung {demand:.4} rad", member.h, member.b, member.l_v, member.n / 1000.0));
    let annex = annex_params(input.document).choice();
    let cf = part_3_state::basis(a, annex).cf;
    let ultimate = part_3::design_capacities(a, &part_3::AnnexA::from_registry(annex), cf).ultimate_rad;
    let ratios = part_3_state::member_ratios(a, annex);
    let station_field = |id: &str, ratio: f64| ScalarField { id: id.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Station { values: vec![StationValue { at: 0.0, value: ratio }, StationValue { at: 1.0, value: ratio }] } };
    let mut fields = Vec::new();
    if demand > 0.0 && ultimate > 0.0 {
        fields.push(station_field(CHORD_FIELD, demand / ultimate));
        legend_once(out, CHORD_FIELD, loc("Chord rotation utilisation \u{3b8}Ed/\u{3b8}um", "Ausnutzung des Sehnendrehwinkels \u{3b8}Ed/\u{3b8}um"), CHORD_FIELD_MAX);
    }
    if let Some(ratio) = ratios.governing().filter(|r| r.is_finite()) {
        fields.push(station_field(DC_FIELD, ratio));
        legend_once(out, DC_FIELD, loc("Demand over capacity D/C of the assessed member (ductile and brittle, EN 1998-3 4.4)", "Beanspruchung zu Kapazit\u{e4}t D/C des bewerteten Bauteils (duktil und spr\u{f6}d, EN 1998-3 4.4)"), DC_FIELD_MAX);
    }
    if let Some(active) = fields.last().map(|f| f.id.clone()) {
        spec.channel = Some(ColourChannel::Fields { active, fields });
    }
    if demand > 0.0 {
        let scale = (0.15 / demand).max(1.0);
        spec.deformation = Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Static, load_factor: None, kind: DeformationKind::Deformed, index: 0, frequency: None, participation: 1.0, scale, samples: vec![DisplacementSample { at: 0.0, delta: Vec3::ZERO }, DisplacementSample { at: 1.0, delta: v3(0.0, 0.0, -demand * member.l_v) }] });
    }
    out.push(spec, &[v3(base.x, base.y - member.b, base.z - member.h), v3(tip.x, tip.y + member.b, tip.z + member.h)]);
    out.push(Spec::new(EntityKind::Support, format!("{}:support", a.id), path.clone(), Geometry::Support(SupportGeometry { kind: SupportKind::Fixed, at: base.clone(), frame: world_frame(base.clone()) }), MaterialRole::Support, loc("Fixed end of the assessed member", "Einspannung des bewerteten Bauteils"), LAYER_ASSESSED), &[base.clone()]);
    let span_from = v3(base.x, base.y - member.b - 0.5, base.z);
    out.annotate(Annotation::Dimension { id: format!("{}:shear-span", a.id), from: span_from.clone(), to: v3(tip.x, span_from.y, span_from.z), offset: v3(0.0, -0.5, 0.0), value: Quantity::new(QuantityKind::Length, member.l_v), binding: Some(FieldBinding::new(format!("{path}.shearSpanM"), Affine::IDENTITY, "m")) }, &span_from);
    let depth_from = v3(tip.x + 0.5, tip.y, tip.z - member.h / 2.0);
    out.annotate(Annotation::Dimension { id: format!("{}:depth", a.id), from: depth_from.clone(), to: v3(depth_from.x, depth_from.y, tip.z + member.h / 2.0), offset: v3(0.5, 0.0, 0.0), value: Quantity::new(QuantityKind::Length, member.h), binding: Some(FieldBinding::new(format!("{path}.sectionDepthM"), Affine::IDENTITY, "m")) }, &depth_from);
    if member.s_h > 0.0 {
        let from = v3(base.x + 0.1, base.y, base.z + member.h / 2.0 + 0.3);
        out.annotate(Annotation::Dimension { id: format!("{}:stirrup-spacing", a.id), from: from.clone(), to: v3(from.x + member.s_h, from.y, from.z), offset: v3(0.0, 0.0, 0.3), value: Quantity::new(QuantityKind::Length, member.s_h), binding: Some(FieldBinding::new(format!("{path}.stirrupSpacingM"), Affine::IDENTITY, "m")) }, &from);
    }
    let capacity = computed_of(input.report, &format!("en1998.3.{}.chordRotation.derivation", a.id));
    let anchor = v3(tip.x, tip.y, tip.z + member.h + 0.6);
    let text = capacity.map_or_else(|| loc(&format!("θ_Ed {demand:.4} rad"), &format!("θ_Ed {demand:.4} rad")), |c| loc(&format!("θ_Ed {demand:.4} rad against θ_um {c:.4} rad (Annex A)"), &format!("θ_Ed {demand:.4} rad gegen θ_um {c:.4} rad (Anhang A)")));
    out.annotate(Annotation::Label { id: format!("{}:capacity", a.id), anchor: anchor.clone(), text, tone: tone_of(&binding_exact(input.report, &format!("en1998.3.{}.chordRotation", a.id))) }, &anchor);
    let basis = part_3_state::basis(a, annex);
    let kl = part_3_state::knowledge_level_token(a.knowledge_level).to_uppercase();
    let ratio = |ratio: Option<f64>, label: &str| ratio.map_or_else(String::new, |r| format!(" \u{b7} {label} {r:.2}"));
    let badge_at = v3(base.x, base.y, base.z + member.h + 1.2);
    let badge = |german: bool| format!("{kl} \u{b7} CF {:.2} \u{b7} T_R {:.0} {}{}{}", basis.cf, basis.return_period_yr, if german { "a" } else { "yr" }, ratio(ratios.chord, "D/C"), ratio(ratios.shear, "E/R"));
    out.annotate(Annotation::Label { id: format!("{}:badge", a.id), anchor: badge_at.clone(), text: loc(&badge(false), &badge(true)), tone: tone_of(&binding) }, &badge_at);
    hoop_layout(&member, a, &binding, base, &path, out);
}

/// 🔗 Number of hoops drawn along the shear span of an assessed member, the most the scene shows.
pub const HOOPS_MAX: usize = 120;

/// 🔗 Hoops s apart over the shear span L_V of an assessed member: the number of rings that fit and the first ring at s/2 (none without hoop spacing).
pub fn hoop_positions(shear_span_m: f64, spacing_m: f64) -> Vec<f64> {
    if shear_span_m <= 0.0 || spacing_m <= 0.0 {
        return Vec::new();
    }
    let count = ((shear_span_m / spacing_m).floor() as usize).min(HOOPS_MAX);
    (0..count).map(|i| (i as f64 + 0.5) * spacing_m).collect()
}

/// 🔗 One hoop ring of the confined core b_0 by h_0 whose bar has the diameter `diameter`: a hollow rectangle prism one bar thick along x, centred on `origin` in y and z.
fn hoop_ring(origin: &Vec3, b_0: f64, h_0: f64, diameter: f64) -> Geometry {
    let (half_b, half_h) = (b_0 / 2.0, h_0 / 2.0);
    let corners = |hb: f64, hh: f64| vec![Vec2::new(-hb, -hh), Vec2::new(hb, -hh), Vec2::new(hb, hh), Vec2::new(-hb, hh)];
    let mut ring = Polygon2::solid(corners(half_b, half_h));
    ring.holes.push(Polygon2::solid(corners(half_b - diameter, half_h - diameter)).outer);
    Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: Frame { origin: origin.clone(), x: Vec3::Y, y: Vec3::Z, z: Vec3::X }, base: ring, height: diameter } })
}

/// 🔗 3D hoop layout of an assessed member (EN 1998-3 A.3.2.2): one ring of the confined core b_0 by h_0 per hoop spacing s_h along the shear span, the bar diameter from the leg area, the confinement effectiveness α in the detail text; silent without core, spacing or leg area.
fn hoop_layout(member: &part_3::RcMember, a: &super::model::En1998Assessment, binding: &Option<UtilisationBinding>, base: &Vec3, path: &str, out: &mut Out) {
    let diameter = (4.0 * a.stirrup_leg_area_m2 / std::f64::consts::PI).sqrt();
    if member.b_0 <= 0.0 || member.h_0 <= 0.0 || diameter <= 0.0 || member.b_0 <= 2.0 * diameter || member.h_0 <= 2.0 * diameter {
        return;
    }
    let alpha = part_3::confinement_effectiveness(member);
    let (half_b, half_h) = (member.b_0 / 2.0, member.h_0 / 2.0);
    for (index, at) in hoop_positions(member.l_v, member.s_h).into_iter().enumerate() {
        let origin = v3(base.x + at, base.y, base.z);
        let geometry = hoop_ring(&origin, member.b_0, member.h_0, diameter);
        let name = loc(&format!("Hoop {} of {} at x = {at:.2} m", index + 1, a.id), &format!("B\u{fc}gel {} von {} bei x = {at:.2} m", index + 1, a.id));
        let mut spec = Spec::new(EntityKind::Solid, format!("{}:hoop:{}", a.id, index + 1), path.to_owned(), geometry, MaterialRole::Rebar, name, LAYER_ASSESSED);
        spec.binding = binding.clone();
        spec.detail = loc(&format!("Hoop \u{2300} {:.1} mm, spacing s_h = {:.0} mm, core {:.0} by {:.0} mm, confinement effectiveness \u{3b1} = {alpha:.2}", diameter * 1000.0, member.s_h * 1000.0, member.b_0 * 1000.0, member.h_0 * 1000.0), &format!("B\u{fc}gel \u{2300} {:.1} mm, Abstand s_h = {:.0} mm, Kern {:.0} mal {:.0} mm, Umschn\u{fc}rungswirksamkeit \u{3b1} = {alpha:.2}", diameter * 1000.0, member.s_h * 1000.0, member.b_0 * 1000.0, member.h_0 * 1000.0));
        out.push(spec, &[v3(origin.x, origin.y - half_b, origin.z - half_h), v3(origin.x + diameter, origin.y + half_b, origin.z + half_h)]);
    }
}

const REGION_CHECKS_HOOPS: [&str; 5] = ["hoopDiameter", "hoopSpacing", "barSpacing", "confinement", "omegaWdMin"];
const REGION_CHECKS_MEMBER: [&str; 2] = ["length", "rhoTot"];

fn region_binding(input: &SceneInput, b: &En1998Building, r: &super::model::En1998CriticalRegion, suffixes: &[&str]) -> Option<UtilisationBinding> {
    suffixes.iter().filter_map(|suffix| binding_exact(input.report, &format!("en1998.1.{}.region.{}.{suffix}", b.id, r.id))).fold(None, |best: Option<UtilisationBinding>, next| match best {
        Some(b) if b.ratio >= next.ratio => Some(b),
        _ => Some(next),
    })
}

/// 🔩 The critical regions of a building as member glyphs with their hoop layout (EN 1998-1 5.4.3, 5.5.3): the member of the region over its clear length, one hoop ring per spacing over the length with the given hoops, the critical region length l_cr and the hoop spacing as dimension lines, the hoops coloured by their checks and the member by length and ρ_tot, grips for the hoop spacing, the hoop diameter and the hooped length.
fn critical_regions(input: &SceneInput, handles: &Index, site: &Site, b: &En1998Building, response: &Response, o: &Vec3, out: &mut Out) {
    for (index, r) in input.document.critical_regions.iter().filter(|r| r.building_id == b.id && region_drawn(r)).enumerate() {
        region_glyph(input, handles, site, b, response, r, &region_origin(o, index), out);
    }
}

fn region_glyph(input: &SceneInput, handles: &Index, site: &Site, b: &En1998Building, response: &Response, r: &super::model::En1998CriticalRegion, base: &Vec3, out: &mut Out) {
    let path = path_of("criticalRegions", &r.id);
    let id = format!("{}:region", r.id);
    let member = b.members.iter().find(|m| m.id == r.member_id);
    let detail = member.zip(governing_system_of(b)).and_then(|(m, system)| critical_region::detail(r, m.role, system.ductility_class, system.q0, response.t1, site.tc, annex_params(input.document).choice()));
    let tip = v3(base.x + r.clear_length_m, base.y, base.z);
    let geometry = Geometry::Member(MemberGeometry { axis: polyline(&[base.clone(), tip.clone()]), frame: member_frame(base, &Vec3::X), section: SectionShape::parametric(ProfileKind::Rect, &[("h", r.section_depth_m), ("b", r.section_width_m)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Rigid, stations: Vec::new() });
    let mut spec = Spec::new(EntityKind::Member, id.clone(), path.clone(), geometry, MaterialRole::Concrete, loc(&format!("Critical region of {} in {} (member {})", r.id, b.name, r.member_id), &format!("Kritischer Bereich von {} in {} (Bauteil {})", r.id, b.name, r.member_id)), LAYER_REGIONS);
    spec.binding = region_binding(input, b, r, &REGION_CHECKS_MEMBER);
    spec.opacity = 0.35;
    spec.handles = handles.of(&id);
    let required = detail.map_or_else(String::new, |d| format!(", l_cr = {:.2} m", d.critical_length_m));
    spec.detail = loc(&format!("Section {:.2} m by {:.2} m, clear length {:.2} m, hoops over {:.2} m{required}", r.section_width_m, r.section_depth_m, r.clear_length_m, r.hooped_length_m), &format!("Querschnitt {:.2} m mal {:.2} m, lichte L\u{e4}nge {:.2} m, B\u{fc}gel auf {:.2} m{}", r.section_width_m, r.section_depth_m, r.clear_length_m, r.hooped_length_m, required));
    out.push(spec, &[v3(base.x, base.y - r.section_width_m / 2.0, base.z - r.section_depth_m / 2.0), v3(tip.x, tip.y + r.section_width_m / 2.0, tip.z + r.section_depth_m / 2.0)]);
    let rings = (r.hoop_spacing_m > 0.0 && r.hoop_diameter_m > 0.0 && r.core_width_m > 2.0 * r.hoop_diameter_m && r.core_depth_m > 2.0 * r.hoop_diameter_m).then(|| hoop_positions(r.hooped_length_m, r.hoop_spacing_m)).unwrap_or_default();
    let binding = region_binding(input, b, r, &REGION_CHECKS_HOOPS);
    let (alpha, required_text) = detail.map_or((f64::NAN, String::new()), |d| (d.alpha, format!("{:.3}", d.confinement_required)));
    for (n, at) in rings.into_iter().enumerate() {
        let origin = v3(base.x + at, base.y, base.z);
        let mut ring = Spec::new(EntityKind::Solid, format!("{}:hoop:{}", r.id, n + 1), path.clone(), hoop_ring(&origin, r.core_width_m, r.core_depth_m, r.hoop_diameter_m), MaterialRole::Rebar, loc(&format!("Hoop {} of {} at x = {at:.2} m", n + 1, r.id), &format!("B\u{fc}gel {} von {} bei x = {at:.2} m", n + 1, r.id)), LAYER_REGIONS);
        ring.binding = binding.clone();
        let facts = |german: bool| {
            let confinement = if alpha.is_finite() { format!(", {} \u{3b1} = {alpha:.2}, \u{3b1}\u{b7}\u{3c9}_wd \u{2265} {required_text}", if german { "Wirksamkeit" } else { "effectiveness" }) } else { String::new() };
            format!("\u{2300} {:.1} mm, s = {:.0} mm, {} {:.0} by {:.0} mm{confinement}", r.hoop_diameter_m * 1000.0, r.hoop_spacing_m * 1000.0, if german { "Kern" } else { "core" }, r.core_width_m * 1000.0, r.core_depth_m * 1000.0)
        };
        ring.detail = loc(&format!("Hoop {}", facts(false)), &format!("B\u{fc}gel {}", facts(true)));
        out.push(ring, &[v3(origin.x, origin.y - r.core_width_m / 2.0, origin.z - r.core_depth_m / 2.0), v3(origin.x + r.hoop_diameter_m, origin.y + r.core_width_m / 2.0, origin.z + r.core_depth_m / 2.0)]);
    }
    let below = v3(base.x, base.y - r.section_width_m / 2.0 - 0.4, base.z);
    if let Some(d) = detail {
        out.annotate(Annotation::Dimension { id: format!("{}:critical-length", r.id), from: below.clone(), to: v3(below.x + d.critical_length_m.min(r.clear_length_m), below.y, below.z), offset: v3(0.0, -0.4, 0.0), value: Quantity::new(QuantityKind::Length, d.critical_length_m), binding: None }, &below);
    }
    if r.hoop_spacing_m > 0.0 {
        let from = v3(base.x + 0.5 * r.hoop_spacing_m, base.y, base.z + r.section_depth_m / 2.0 + 0.3);
        out.annotate(Annotation::Dimension { id: format!("{}:hoop-spacing", r.id), from: from.clone(), to: v3(from.x + r.hoop_spacing_m, from.y, from.z), offset: v3(0.0, 0.0, 0.3), value: Quantity::new(QuantityKind::Length, r.hoop_spacing_m), binding: Some(FieldBinding::new(format!("{path}.hoopSpacingM"), Affine::IDENTITY, "m")) }, &from);
    }
    let badge_at = v3(base.x, base.y, base.z + r.section_depth_m / 2.0 + 0.9);
    let worst = region_binding(input, b, r, &[REGION_CHECKS_MEMBER.as_slice(), REGION_CHECKS_HOOPS.as_slice()].concat());
    let class = governing_system_of(b).map_or("-".to_owned(), |s| format!("{:?}", s.ductility_class).to_uppercase());
    let text = format!("{class} \u{b7} {} \u{b7} {}", r.member_id, worst.as_ref().map_or_else(String::new, |w| format!("{:.2}", w.ratio)));
    out.annotate(Annotation::Label { id: format!("{}:badge", r.id), anchor: badge_at.clone(), text: loc(&text, &text), tone: tone_of(&worst) }, &badge_at);
}

fn governing_system_of(b: &En1998Building) -> Option<&super::model::En1998System> {
    critical_region::governing_system(&b.systems)
}

fn legend_once(out: &mut Out, id: &str, title: LocalizedCopy, max: f64) {
    if !out.legends.iter().any(|l| l.id == id) {
        out.legends.push(Legend { id: id.into(), title, scale: LegendScale::Scalar { field_id: id.into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max }, unit: QuantityKind::Dimensionless } });
    }
}

fn governing_binding(input: &SceneInput, a: &super::model::En1998Assessment) -> Option<UtilisationBinding> {
    ["chordRotation", "cyclicShear", "capacity", "limitState", "analysisMethod"].iter().filter_map(|suffix| binding_exact(input.report, &format!("en1998.3.{}.{suffix}", a.id))).fold(None, |best: Option<UtilisationBinding>, next| match best {
        Some(b) if b.ratio >= next.ratio => Some(b),
        _ => Some(next),
    })
}
