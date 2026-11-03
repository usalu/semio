//! 🌉️ Foundations, tie beams, slopes, retaining walls and bridges: footing bodies under their building, tie beams between two footings with their tie force, slopes with the slip circles and the pseudo-static forces, walls with the seismic thrust, bridge decks on piers with bearings and the bearing displacement capacity.

use super::handles::Index;
use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::norm::ndp_common;
use super::schema::{part_1, part_2, part_5};
use super::structure_views::{bridge_view, BridgeView};
use super::SceneInput;

pub const SLOPE_WIDTH: f64 = 10.0;
pub const SLOPE_CREST: f64 = 5.0;
pub const WALL_LENGTH: f64 = 10.0;
pub const WALL_THICKNESS: f64 = 0.4;
pub const WALL_BACKFILL_WIDTH: f64 = 3.0;
const DECK_LENGTH: f64 = 30.0;
pub const DECK_THICKNESS: f64 = 0.8;
const LAYER_THICKNESS: f64 = 0.5;
const PILE_LAYER_THICKNESS: f64 = 0.1;
pub const MIN_SOIL_LAYER: f64 = 0.1;
const CRACKED_FIELD: &str = "pier-cracked-share";
const CRACKED_FIELD_MAX: f64 = 1.0;
pub const BADGE_RISE: f64 = 1.5;
pub const BADGE_PER_CLASS: f64 = 0.5;
pub const VERTICAL_RISE: f64 = 3.0;
pub const VERTICAL_PER_SECOND: f64 = 4.0;

pub fn project(input: &SceneInput, layout: &Layout, handles: &Index, out: &mut Out) {
    let d = input.document;
    for f in &d.foundations {
        if !out.own("foundations", &f.id) {
            continue;
        }
        foundation(input, layout, handles, f, out);
    }
    for w in &d.retaining_walls {
        if !out.own("retainingWalls", &w.id) {
            continue;
        }
        wall(input, layout, handles, w, out);
    }
    for t in &d.tie_beams {
        if !out.own("tieBeams", &t.id) {
            continue;
        }
        tie_beam(input, layout, handles, t, out);
    }
    for sl in &d.slopes {
        if !out.own("slopes", &sl.id) {
            continue;
        }
        slope(input, layout, handles, sl, out);
    }
    for b in &d.bridges {
        if !out.own("bridges", &b.id) {
            continue;
        }
        bridge(input, layout, handles, b, out);
    }
    for p in &d.piers {
        if !out.own("piers", &p.id) {
            continue;
        }
        pier(input, layout, handles, p, out);
    }
    for k in &d.bearings {
        if !out.own("bearings", &k.id) {
            continue;
        }
        bearing(input, layout, handles, k, out);
    }
}

/// 🌉️ The bridge of a row and the analysis of the support lines it carries.
fn bridge_of<'a>(input: &'a SceneInput, bridge_id: &str) -> Option<(&'a super::model::En1998Bridge, BridgeView)> {
    let b = input.document.bridges.iter().find(|b| b.id == bridge_id)?;
    Some((b, bridge_view(input.document, b, &super::response::site(input.document))?))
}

fn deck_extent(b: &super::model::En1998Bridge, view: &BridgeView) -> f64 {
    if b.deck_length_m > 0.0 { b.deck_length_m } else { view.supports.iter().map(|s| s.x_m).fold(0.0, f64::max).max(DECK_LENGTH) }
}

fn deck_level(input: &SceneInput, bridge_id: &str) -> f64 {
    deck_level_of(input.document, bridge_id)
}

fn deck_level_of(document: &super::model::En1998Snapshot, bridge_id: &str) -> f64 {
    document.piers.iter().filter(|p| p.bridge_id == bridge_id).map(|p| if p.height_m > 0.0 { p.height_m } else { PIER_HEIGHT }).fold(PIER_HEIGHT, f64::max)
}

/// 🌉️ The deck origin of a bridge in the scene: the level of the highest pier row, or the bridge pier height without rows, above the layout origin.
pub fn deck_origin(document: &super::model::En1998Snapshot, b: &super::model::En1998Bridge, o: &Vec3) -> Vec3 {
    let level = if !super::schema::bridge::has_supports(&b.id, &document.piers, &document.bearings) { if b.pier_height_m > 0.0 { b.pier_height_m } else { PIER_HEIGHT } } else { deck_level_of(document, &b.id) };
    v3(o.x, o.y, o.z + level)
}

/// 🛞 Where an isolation system stands: below the deck plate at the start of the deck of its bridge, or `glyph` below the base under the middle of the plan of its building; `None` while it names neither.
pub fn isolation_anchor(document: &super::model::En1998Snapshot, layout: &Layout, system: &super::model::En1998IsolationSystem, glyph: f64) -> Option<Vec3> {
    if let Some(b) = document.bridges.iter().find(|b| !system.bridge_id.is_empty() && system.building_id.is_empty() && b.id == system.bridge_id) {
        let deck = deck_origin(document, b, &layout.origin(&b.id));
        return Some(v3(deck.x, deck.y + DECK_WIDTH / 2.0, deck.z - glyph));
    }
    document.buildings.iter().find(|b| !system.building_id.is_empty() && system.bridge_id.is_empty() && b.id == system.building_id).map(|b| {
        let o = layout.origin(&b.id);
        v3(o.x + b.plan_width_m / 2.0, o.y + b.plan_length_m / 2.0, o.z - glyph)
    })
}

/// 🏷️ Where the variable actions of a bridge rest: the middle of the deck top, at the deck level the projection uses (pier rows or the bridge pier height).
pub fn variables_anchor(document: &super::model::En1998Snapshot, b: &super::model::En1998Bridge, o: &Vec3) -> Vec3 {
    let deck = deck_origin(document, b, o);
    let length = if b.deck_length_m > 0.0 { b.deck_length_m } else { DECK_LENGTH };
    v3(deck.x + length / 2.0, deck.y + DECK_WIDTH / 2.0, deck.z + DECK_THICKNESS)
}

/// 🏷️ The variable action glyphs of one bridge on its deck.
fn variable_glyphs(input: &SceneInput, handles: &Index, b: &super::model::En1998Bridge, o: &Vec3, path: &str, out: &mut Out) {
    for mark in super::structure_views::variables::marks(&b.id, path, &b.variables, b.permanent_gk_n, &variables_anchor(input.document, b, o)) {
        let mut glyph = Spec::new(EntityKind::Load, mark.id.clone(), mark.path.clone(), mark.glyph(), MaterialRole::Load { kind: LoadKind::Variable }, mark.name(), LAYER_BRIDGES);
        glyph.handles = handles.of(&mark.id);
        out.push(glyph, &[mark.at.clone()]);
    }
}

/// 🏛️ One pier row: the rectangular column from the foundation to the bearing level coloured by its flexure and shear checks, the plastic hinge moment, the capacity-design shear and the shear resistance as draggable arrows, the height and the position as grips.
fn pier(input: &SceneInput, layout: &Layout, handles: &Index, p: &super::model::En1998Pier, out: &mut Out) {
    let Some((b, _)) = bridge_of(input, &p.bridge_id) else { return };
    let o = layout.origin(&b.id);
    let height = if p.height_m > 0.0 { p.height_m } else { PIER_HEIGHT };
    let (width, depth) = (if p.section_b_m > 0.0 { p.section_b_m } else { PIER_SIZE }, if p.section_h_m > 0.0 { p.section_h_m } else { PIER_SIZE });
    let base = v3(o.x + p.x_m, o.y + DECK_WIDTH / 2.0, o.z);
    let top = v3(base.x, base.y, base.z + height);
    let path = path_of("piers", &p.id);
    let mut spec = Spec::new(EntityKind::Member, p.id.clone(), path.clone(), Geometry::Member(MemberGeometry { axis: polyline(&[base.clone(), top.clone()]), frame: member_frame(&base, &Vec3::Z), section: SectionShape::parametric(ProfileKind::Rect, &[("h", depth), ("b", width)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Pinned, stations: Vec::new() }), MaterialRole::Concrete, loc(&format!("Pier {} of {}", p.id, b.id), &format!("Pfeiler {} von {}", p.id, b.id)), LAYER_BRIDGES);
    let ratio = binding_exact(input.report, &format!("en1998.2.{}.pier.{}.shear", b.id, p.id));
    spec.binding = ratio.clone().or_else(|| binding_prefix(input.report, &format!("en1998.2.{}.pier.{}.", b.id, p.id)));
    spec.handles = handles.of(&p.id);
    let share = super::schema::bridge::pier_ei_ratio(p, super::schema::bridge::Direction::Longitudinal, b.pier_cracked_inertia_ratio, &super::response::site(input.document).section);
    let cracked = ScalarField { id: CRACKED_FIELD.into(), unit: QuantityKind::Dimensionless, samples: FieldSamples::Station { values: vec![StationValue { at: 0.0, value: share }, StationValue { at: 1.0, value: share }] } };
    spec.channel = Some(ColourChannel::Fields { active: CRACKED_FIELD.into(), fields: vec![cracked] });
    if !out.legends.iter().any(|l| l.id == CRACKED_FIELD) {
        out.legends.push(Legend { id: CRACKED_FIELD.into(), title: loc("Effective stiffness share EI_eff / EI_g of the piers (EN 1998-2 2.3.6.1)", "Wirksamer Steifigkeitsanteil EI_eff / EI_g der Pfeiler (EN 1998-2 2.3.6.1)"), scale: LegendScale::Scalar { field_id: CRACKED_FIELD.into(), map: ColourMapRef { id: "viridis".into(), min: 0.0, max: CRACKED_FIELD_MAX }, unit: QuantityKind::Dimensionless } });
    }
    spec.detail = loc(&format!("Pier {}, height {:.2} m, section {:.2} by {:.2} m, E = {:.0} MPa, effective stiffness share EI_eff / EI_g = {:.2} ({})", p.id, height, width, depth, p.modulus_mpa, share, if p.stiffness_basis == super::model::PierStiffnessBasis::Computed { "computed M_y/phi_y" } else { "declared" }), &format!("Pfeiler {}, H\u{f6}he {:.2} m, Querschnitt {:.2} auf {:.2} m, E = {:.0} MPa, wirksamer Steifigkeitsanteil EI_eff / EI_g = {:.2} ({})", p.id, height, width, depth, p.modulus_mpa, share, if p.stiffness_basis == super::model::PierStiffnessBasis::Computed { "berechnet M_y/phi_y" } else { "angegeben" }));
    out.push(spec, &[base.clone(), top.clone()]);
    let case = load_case(&format!("seismic-{}", p.id));
    if p.moment_rd_nm > 0.0 {
        let id = format!("{}:mrd", p.id);
        let mut moment = Spec::new(EntityKind::Load, id.clone(), path.clone(), Geometry::Load(LoadGlyph::Moment { at: base.clone(), axis: Vec3::Y, magnitude: Quantity::new(QuantityKind::Moment, p.moment_rd_nm), case: case.clone() }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Plastic hinge moment M_Rd = {:.0} kN\u{b7}m", p.moment_rd_nm / 1000.0), &format!("Plastisches Gelenkmoment M_Rd = {:.0} kN\u{b7}m", p.moment_rd_nm / 1000.0)), LAYER_FORCES);
        moment.binding = binding_exact(input.report, &format!("en1998.2.{}.pier.{}.flexure", b.id, p.id));
        moment.handles = handles.of(&id);
        out.push(moment, &[base.clone()]);
        let demand = part_2_shear(input, b, p);
        if demand > 0.0 {
            let mut shear = Spec::new(EntityKind::Load, format!("{}:vcd", p.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: top.clone(), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, demand), case: case.clone() }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Capacity-design shear V_CD = {:.0} kN", demand / 1000.0), &format!("Kapazit\u{e4}ts-Querkraft V_CD = {:.0} kN", demand / 1000.0)), LAYER_FORCES);
            shear.binding = binding_exact(input.report, &format!("en1998.2.{}.pier.{}.shear", b.id, p.id));
            out.push(shear, &[top.clone()]);
        }
    }
    if p.shear_rd_n > 0.0 {
        let id = format!("{}:vrd", p.id);
        let at = v3(top.x, top.y + depth, base.z + 0.5 * height);
        let mut resistance = Spec::new(EntityKind::Load, id.clone(), path.clone(), Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: v3(-1.0, 0.0, 0.0), magnitude: Quantity::new(QuantityKind::Force, p.shear_rd_n), case }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Shear resistance V_Rd = {:.0} kN", p.shear_rd_n / 1000.0), &format!("Querkrafttragf\u{e4}higkeit V_Rd = {:.0} kN", p.shear_rd_n / 1000.0)), LAYER_FORCES);
        resistance.handles = handles.of(&id);
        out.push(resistance, &[at]);
    }
    let label = v3(top.x, top.y - depth, top.z + 0.5);
    out.annotate(Annotation::Label { id: format!("{}:label", p.id), anchor: label.clone(), text: loc(&format!("{}: M_Rd = {:.0} kN\u{b7}m, V_Rd = {:.0} kN", p.id, p.moment_rd_nm / 1000.0, p.shear_rd_n / 1000.0), &format!("{}: M_Rd = {:.0} kN\u{b7}m, V_Rd = {:.0} kN", p.id, p.moment_rd_nm / 1000.0, p.shear_rd_n / 1000.0)), tone: tone_of(&binding_prefix(input.report, &format!("en1998.2.{}.pier.{}.", b.id, p.id))) }, &label);
    if let Some(ratio) = ratio.as_ref() {
        let at = v3(top.x, top.y + depth, top.z + 0.5);
        out.annotate(Annotation::Label { id: format!("{}:capacity-ratio", p.id), anchor: at.clone(), text: loc(&format!("V_CD / V_Rd = {:.2}", ratio.ratio), &format!("V_CD / V_Rd = {:.2}", ratio.ratio)), tone: tone_of(&Some(ratio.clone())) }, &at);
    }
    let alpha_s = part_2::shear_span_ratio(height, depth.max(width));
    let q_max = part_2::rc_pier_behaviour_factor(ndp_common("ndp.en1998.en2.q.rc-pier"), alpha_s, ndp_common("ndp.en1998.en2.q.shear-span-full"), ndp_common("ndp.en1998.en2.q.shear-span-min"));
    let q_at = v3(top.x, top.y - depth, base.z + 0.5 * height);
    let (q_en, q_de) = match q_max {
        Some(q) => (format!("q \u{2264} {q:.2} (\u{3b1}_s = {alpha_s:.2})"), format!("q \u{2264} {q:.2} (\u{3b1}_s = {alpha_s:.2})")),
        None => (format!("squat: \u{3b1}_s = {alpha_s:.2}"), format!("gedrungen: \u{3b1}_s = {alpha_s:.2}")),
    };
    out.annotate(Annotation::Label { id: format!("{}:q", p.id), anchor: q_at.clone(), text: loc(&q_en, &q_de), tone: tone_of(&binding_exact(input.report, &format!("en1998.2.{}.pier.{}.behaviour", b.id, p.id))) }, &q_at);
    let from = v3(base.x - width, base.y, base.z);
    out.annotate(Annotation::Dimension { id: format!("{}:height", p.id), from: from.clone(), to: v3(from.x, from.y, base.z + height), offset: v3(-1.0, 0.0, 0.0), value: Quantity::new(QuantityKind::Length, height), binding: Some(FieldBinding::new(format!("{path}.heightM"), Affine::IDENTITY, "m")) }, &from);
}

fn part_2_shear(input: &SceneInput, b: &super::model::En1998Bridge, p: &super::model::En1998Pier) -> f64 {
    computed_of(input.report, &format!("en1998.2.{}.pier.{}.shear", b.id, p.id)).unwrap_or(0.0)
}

/// 🛞 One bearing row: the support symbol at the pier top (or at the deck end of an abutment) coloured by the displacement check, the displacement capacity and the seat length as draggable dimensions.
fn bearing(input: &SceneInput, layout: &Layout, handles: &Index, k: &super::model::En1998Bearing, out: &mut Out) {
    let Some((b, _)) = bridge_of(input, &k.bridge_id) else { return };
    let o = layout.origin(&b.id);
    let pier = input.document.piers.iter().find(|p| p.id == k.pier_id && p.bridge_id == b.id);
    let x = pier.map_or(k.x_m, |p| p.x_m);
    let z = o.z + pier.map_or_else(|| deck_level(input, &b.id), |p| if p.height_m > 0.0 { p.height_m } else { PIER_HEIGHT });
    let at = v3(o.x + x, o.y + DECK_WIDTH / 2.0, z);
    let path = path_of("bearings", &k.id);
    let kind = match k.device {
        semio_s_artifact_norm_en1998_model::BearingDevice::Fixed => SupportKind::Pinned,
        semio_s_artifact_norm_en1998_model::BearingDevice::Sliding => SupportKind::Roller { normal: Vec3::Z },
        _ if k.stiffness_n_per_m > 0.0 => SupportKind::Spring { stiffness: [k.stiffness_n_per_m, k.stiffness_n_per_m, 0.0, 0.0, 0.0, 0.0].iter().map(|v| Quantity::new(QuantityKind::Stiffness, *v)).collect() },
        _ => SupportKind::Pinned,
    };
    let mut spec = Spec::new(EntityKind::Support, k.id.clone(), path.clone(), Geometry::Support(SupportGeometry { kind, at: at.clone(), frame: world_frame(at.clone()) }), MaterialRole::Support, loc(&format!("Bearing {} ({:?}) of {}", k.id, k.device, b.id), &format!("Lager {} ({:?}) von {}", k.id, k.device, b.id)), LAYER_BRIDGES);
    spec.binding = binding_prefix(input.report, &format!("en1998.2.{}.bearing.{}.", b.id, k.id));
    spec.handles = handles.of(&k.id);
    spec.detail = loc(&format!("Bearing {}, {:?}, k = {:.3e} N/m, d_Rd = {:.3} m, seat {:.2} m", k.id, k.device, k.stiffness_n_per_m, k.d_rd_m, k.seat_length_m), &format!("Lager {}, {:?}, k = {:.3e} N/m, d_Rd = {:.3} m, Auflager {:.2} m", k.id, k.device, k.stiffness_n_per_m, k.d_rd_m, k.seat_length_m));
    out.push(spec, &[at.clone()]);
    let from = v3(at.x, at.y - DECK_WIDTH / 2.0 - 1.0, at.z);
    out.annotate(Annotation::Dimension { id: format!("{}:capacity", k.id), from: from.clone(), to: v3(from.x + k.d_rd_m, from.y, from.z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, k.d_rd_m), binding: Some(FieldBinding::new(format!("{path}.dRdM"), Affine::IDENTITY, "m")) }, &from);
    let seat_from = v3(at.x, at.y - DECK_WIDTH / 2.0 - 2.5, at.z);
    out.annotate(Annotation::Dimension { id: format!("{}:seat", k.id), from: seat_from.clone(), to: v3(seat_from.x + k.seat_length_m, seat_from.y, seat_from.z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, k.seat_length_m), binding: Some(FieldBinding::new(format!("{path}.seatLengthM"), Affine::IDENTITY, "m")) }, &seat_from);
    if let Some(required) = computed_of(input.report, &format!("en1998.2.{}.bearing.{}.seat", b.id, k.id)).filter(|v| *v > 0.0) {
        let from = v3(at.x, at.y - DECK_WIDTH / 2.0 - 3.5, at.z);
        out.annotate(Annotation::Dimension { id: format!("{}:seat-required", k.id), from: from.clone(), to: v3(from.x + required, from.y, from.z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, required), binding: None }, &from);
    }
    if let Some(demand) = computed_of(input.report, &format!("en1998.2.{}.bearing.{}.displacement", b.id, k.id)).map(f64::abs).filter(|v| *v > 0.0) {
        let origin = v3(at.x, at.y - 0.15, at.z + 0.3);
        let mut arrow = Spec::new(EntityKind::Solid, format!("{}:demand", k.id), path.clone(), box_solid(origin.clone(), v3(demand.max(0.02), 0.3, 0.3)), MaterialRole::Neutral, loc(&format!("Bearing displacement demand d_Ed = {:.1} mm against d_Rd = {:.1} mm", demand * 1000.0, k.d_rd_m * 1000.0), &format!("Lagerverschiebung d_Ed = {:.1} mm gegen d_Rd = {:.1} mm", demand * 1000.0, k.d_rd_m * 1000.0)), LAYER_BRIDGES);
        arrow.binding = binding_exact(input.report, &format!("en1998.2.{}.bearing.{}.displacement", b.id, k.id));
        out.push(arrow, &[origin.clone(), v3(origin.x + demand, origin.y, origin.z)]);
        let to = v3(at.x + demand, at.y, at.z + 0.9);
        out.annotate(Annotation::Dimension { id: format!("{}:demand-dim", k.id), from: v3(at.x, at.y, at.z + 0.9), to, offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, demand), binding: None }, &at);
    }
}

/// 🎵️ The first transverse and longitudinal modes of the deck as animated ghosts on the mode layers of the y and x directions: unit-peak shapes of the kernel at the nodes, scaled to a visible amplitude.
fn deck_mode_ghosts(b: &super::model::En1998Bridge, view: &BridgeView, o: &Vec3, deck_z: f64, out: &mut Out) {
    let length = deck_extent(b, view);
    let path = path_of("bridges", &b.id);
    let (start, end) = (v3(o.x, o.y + DECK_WIDTH / 2.0, deck_z + DECK_THICKNESS / 2.0), v3(o.x + length, o.y + DECK_WIDTH / 2.0, deck_z + DECK_THICKNESS / 2.0));
    let directions = [
        (view.transverse.as_ref(), [super::kit::LAYER_MODE_Y1, super::kit::LAYER_MODE_Y2, super::kit::LAYER_MODE_Y3], "y", ("Transverse", "Querschwingform")),
        (view.longitudinal.as_ref(), [super::kit::LAYER_MODE_X1, super::kit::LAYER_MODE_X2, super::kit::LAYER_MODE_X3], "x", ("Longitudinal", "L\u{e4}ngsschwingform")),
    ];
    for (response, layers, tag, (name_en, name_de)) in directions {
        let Some(response) = response else { continue };
        for (n, (shape, period)) in response.shapes.iter().zip(&response.periods_s).enumerate().take(3) {
            let samples = response.node_x_m.iter().zip(shape).map(|(x, v)| DisplacementSample { at: (x / length).clamp(0.0, 1.0), delta: if tag == "x" { v3(*v, 0.0, 0.0) } else { v3(0.0, *v, 0.0) } }).collect();
            let geometry = Geometry::Member(MemberGeometry { axis: polyline(&[start.clone(), end.clone()]), frame: member_frame(&start, &Vec3::X), section: SectionShape::parametric(ProfileKind::Rect, &[("h", DECK_THICKNESS), ("b", DECK_WIDTH / 5.0)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Free, end_release: EndRelease::Free, stations: Vec::new() });
            let name = loc(&format!("{name_en} mode {} of {}: T = {period:.3} s, M_eff = {:.0} %", n + 1, b.id, response.effective_mass_ratio[n] * 100.0), &format!("{name_de} {} von {}: T = {period:.3} s, M_eff = {:.0} %", n + 1, b.id, response.effective_mass_ratio[n] * 100.0));
            let mut spec = Spec::new(EntityKind::Member, format!("{}:mode-{tag}{}", b.id, n + 1), path.clone(), geometry, MaterialRole::Neutral, name, layers[n]);
            spec.opacity = 0.35;
            spec.deformation = Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: n as u32 + 1, frequency: Some(Quantity::new(QuantityKind::Frequency, 1.0 / period)), participation: response.effective_mass_ratio[n], scale: 0.06 * length, samples });
            out.push(spec, &[start.clone(), end.clone()]);
        }
    }
}

/// 🌉️ A bridge with pier or bearing rows: the deck at the level of the highest pier over the support lines, the transverse mode ghosts and the seismic base shear; the piers and bearings are entities of their own rows.
fn bridge_rows(input: &SceneInput, layout: &Layout, handles: &Index, b: &super::model::En1998Bridge, view: &BridgeView, out: &mut Out) {
    let o = layout.origin(&b.id);
    let path = path_of("bridges", &b.id);
    let binding = binding_prefix(input.report, &format!("en1998.2.{}.", b.id));
    let length = deck_extent(b, view);
    let deck = v3(o.x, o.y, o.z + deck_level(input, &b.id));
    let period = view.longitudinal.as_ref().and_then(|r| r.periods_s.first().copied()).unwrap_or(b.fundamental_period_s);
    let mut spec = Spec::new(EntityKind::Plate, b.id.clone(), path.clone(), Geometry::Plate(PlateGeometry { frame: world_frame(deck.clone()), outline: rect_ring(length, DECK_WIDTH), thickness: DECK_THICKNESS, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Concrete, loc(&format!("Bridge deck {}", b.id), &format!("Br\u{fc}ckendeck {}", b.id)), LAYER_BRIDGES);
    spec.binding = binding.clone();
    spec.handles = handles.of(&b.id);
    spec.detail = loc(&format!("Bridge {}, deck {:.1} m, {} support lines, longitudinal period {:.2} s", b.id, length, view.supports.len(), period), &format!("Br\u{fc}cke {}, \u{dc}berbau {:.1} m, {} Auflagerlinien, L\u{e4}ngsperiode {:.2} s", b.id, length, view.supports.len(), period));
    out.push(spec, &[deck.clone(), v3(o.x + length, o.y + DECK_WIDTH, deck.z + DECK_THICKNESS)]);
    deck_mode_ghosts(b, view, &o, deck.z, out);
    bridge_indicators(input, b, &deck, out);
    variable_glyphs(input, handles, b, &o, &path, out);
    if let Some(shear) = computed_of(input.report, &format!("en1998.2.{}.shear", b.id)).filter(|v| *v > 0.0) {
        let at = v3(o.x, o.y + DECK_WIDTH / 2.0, deck.z + DECK_THICKNESS / 2.0);
        out.push(Spec::new(EntityKind::Load, format!("{}:force", b.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, shear), case: load_case(&format!("seismic-{}", b.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Seismic base shear {:.0} kN of {}", shear / 1000.0, b.id), &format!("Erdbeben-Fu\u{df}querkraft {:.0} kN von {}", shear / 1000.0, b.id)), LAYER_FORCES), &[at]);
    }
    if let Some(clause) = binding.as_ref().and_then(|x| clause_of(input.report, &x.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", b.id), anchor: entity_ref(EntityKind::Plate, &b.id, &path), clause }, &deck);
    }
}

/// 🏅️ The bridge-level indicators of the new action fields: the importance class badge with `\u{3b3}_I` above the deck start (check `action.importance`) and, when the deck has a vertical period, the vertical spectrum indication `T_v` with `S_ve(T_v)`; both are stacked above the deck and carry grips on `importanceClass` and `verticalPeriodS`.
fn bridge_indicators(input: &SceneInput, b: &super::model::En1998Bridge, deck: &Vec3, out: &mut Out) {
    let class = b.importance_class.round().clamp(1.0, 3.0);
    let gamma_i = part_2::bridge_gamma_i(b.importance_class);
    let at = v3(deck.x, deck.y + DECK_WIDTH / 2.0, deck.z + DECK_THICKNESS + BADGE_RISE + BADGE_PER_CLASS * class);
    let tone = tone_of(&binding_exact(input.report, &format!("en1998.2.{}.action.importance", b.id)));
    out.annotate(Annotation::Label { id: format!("{}:importance", b.id), anchor: at.clone(), text: loc(&format!("Importance class {class:.0}, \u{3b3}_I = {gamma_i}"), &format!("Bedeutungsklasse {class:.0}, \u{3b3}_I = {gamma_i}")), tone }, &at);
    if b.vertical_period_s > 0.0 {
        let a_g = gamma_i * super::response::site(input.document).a_gr;
        let s_ve = part_2::bridge_vertical_spectrum(a_g, ndp_common("ndp.en1998.en2.spectrum.vertical-ratio"), b.vertical_period_s);
        let up = v3(deck.x, deck.y + DECK_WIDTH / 2.0, deck.z + DECK_THICKNESS + VERTICAL_RISE + VERTICAL_PER_SECOND * b.vertical_period_s);
        out.annotate(Annotation::Label { id: format!("{}:vertical", b.id), anchor: up.clone(), text: loc(&format!("Vertical spectrum: T_v = {:.2} s, S_ve = {s_ve:.2} m/s\u{b2}", b.vertical_period_s), &format!("Vertikalspektrum: T_v = {:.2} s, S_ve = {s_ve:.2} m/s\u{b2}", b.vertical_period_s)), tone: Tone::Info }, &up);
    }
}

fn foundation(input: &SceneInput, layout: &Layout, handles: &Index, f: &super::model::En1998Foundation, out: &mut Out) {
    let centre = foundation_centre(input.document, layout, f);
    let side = f.area_m2.max(0.01).sqrt();
    let origin = v3(centre.x - side / 2.0, centre.y - side / 2.0, centre.z - FOOTING_DEPTH);
    let path = path_of("foundations", &f.id);
    let binding = binding_prefix(input.report, &format!("en1998.5.{}.", f.id));
    let mut spec = Spec::new(EntityKind::Solid, f.id.clone(), path.clone(), Geometry::Solid(SolidGeometry { shape: SolidShape::FootingBody { frame: world_frame(origin.clone()), footprint: rect_ring(side, side), thickness: FOOTING_DEPTH, pedestal: None, pedestal_height: 0.0 } }), MaterialRole::Concrete, loc(&format!("Foundation {}", f.id), &format!("Gr\u{fc}ndung {}", f.id)), LAYER_GROUND);
    spec.binding = binding.clone();
    spec.handles = handles.of(&f.id);
    spec.detail = loc(&format!("Foundation {} under {}, area {:.1} m\u{b2}", f.id, f.supported_building_id, f.area_m2), &format!("Gr\u{fc}ndung {} unter {}, Fl\u{e4}che {:.1} m\u{b2}", f.id, f.supported_building_id, f.area_m2));
    out.push(spec, &[origin.clone(), v3(origin.x + side, origin.y + side, centre.z)]);
    let layered = input.document.soil_layers.iter().any(|l| l.foundation_id == f.id);
    if f.liq_depth_m > 0.0 && f.liq_n1_60 > 0.0 && !layered {
        let thickness = if f.liq_layer_thickness_m > 0.0 { f.liq_layer_thickness_m } else { LAYER_THICKNESS };
        let layer = v3(centre.x - 1.5 * side, centre.y - 1.5 * side, centre.z - f.liq_depth_m - thickness / 2.0);
        let liquefaction = binding_exact(input.report, &format!("en1998.5.{}.liquefaction", f.id));
        let mut soil = Spec::new(EntityKind::Solid, format!("{}:liquefiable-layer", f.id), path.clone(), box_solid(layer.clone(), v3(3.0 * side, 3.0 * side, thickness)), MaterialRole::Soil { class: "liquefiable".into() }, loc(&format!("Layer at {:.1} m, (N1)60 = {:.1}", f.liq_depth_m, f.liq_n1_60), &format!("Schicht in {:.1} m, (N1)60 = {:.1}", f.liq_depth_m, f.liq_n1_60)), LAYER_GROUND);
        soil.binding = liquefaction;
        soil.opacity = 0.5;
        soil.handles = handles.of(&format!("{}:liquefiable-layer", f.id));
        out.push(soil, &[layer.clone(), v3(layer.x + 3.0 * side, layer.y + 3.0 * side, layer.z + thickness)]);
        let from = v3(centre.x + 1.5 * side, centre.y, centre.z);
        out.annotate(Annotation::Dimension { id: format!("{}:liquefaction-depth", f.id), from: from.clone(), to: v3(from.x, from.y, centre.z - f.liq_depth_m), offset: v3(1.0, 0.0, 0.0), value: Quantity::new(QuantityKind::Length, f.liq_depth_m), binding: Some(FieldBinding::new(format!("{path}.liqDepthM"), Affine::IDENTITY, "m")) }, &from);
    }
    soil_column(input, handles, f, &centre, side, out);
    footing_actions(input, f, &centre, side, &path, out);
    pile(input, handles, f, &centre, &path, out);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", f.id), anchor: entity_ref(EntityKind::Solid, &f.id, &path), clause }, &centre);
    }
}

/// 🪨️ The layered soil column of a foundation: one translucent soil body per `soilLayers` row from the ground surface down (class `liquefiable` where (N1)60 is given), coloured by its own Annex B check, with a parameter label and a draggable interface at its bottom.
fn soil_column(input: &SceneInput, handles: &Index, f: &super::model::En1998Foundation, centre: &Vec3, side: f64, out: &mut Out) {
    let mut top = 0.0;
    for row in input.document.soil_layers.iter().filter(|l| l.foundation_id == f.id) {
        let thickness = row.thickness_m.max(MIN_SOIL_LAYER);
        let origin = v3(centre.x - 1.5 * side, centre.y - 1.5 * side, centre.z - top);
        let class = if row.n1_60 > 0.0 { "liquefiable" } else { "cohesive" };
        let name = loc(&format!("Soil layer {} of {}", row.id, f.id), &format!("Bodenschicht {} von {}", row.id, f.id));
        let strata = vec![Stratum { thickness, role: MaterialRole::Soil { class: class.into() }, label: name.clone() }];
        let shape = SolidShape::StrataColumn { frame: world_frame(origin.clone()), footprint: rect_ring(3.0 * side, 3.0 * side), strata };
        let path = path_of("soilLayers", &row.id);
        let mut spec = Spec::new(EntityKind::Solid, row.id.clone(), path.clone(), Geometry::Solid(SolidGeometry { shape }), MaterialRole::Soil { class: class.into() }, name, LAYER_GROUND);
        spec.binding = binding_exact(input.report, &format!("en1998.5.{}.layer.{}.liquefaction", f.id, row.id));
        spec.opacity = 0.55;
        spec.handles = handles.of(&row.id);
        spec.detail = loc(
            &format!("{:.2} to {:.2} m, gamma {:.1} kN/m\u{b3}, v_s {:.0} m/s, (N1)60 {:.1}, FC {:.0} %", top, top + thickness, row.unit_weight_n_m3 / 1000.0, row.shear_wave_velocity_m_s, row.n1_60, row.fines_percent),
            &format!("{:.2} bis {:.2} m, gamma {:.1} kN/m\u{b3}, v_s {:.0} m/s, (N1)60 {:.1}, FC {:.0} %", top, top + thickness, row.unit_weight_n_m3 / 1000.0, row.shear_wave_velocity_m_s, row.n1_60, row.fines_percent),
        );
        out.push(spec, &[origin.clone(), v3(origin.x + 3.0 * side, origin.y + 3.0 * side, origin.z - thickness)]);
        let anchor = v3(origin.x, origin.y, origin.z - thickness / 2.0);
        out.annotate(Annotation::Label { id: format!("{}:label", row.id), anchor: anchor.clone(), text: loc(&format!("{}: v_s {:.0} m/s, (N1)60 {:.1}", row.id, row.shear_wave_velocity_m_s, row.n1_60), &format!("{}: v_s {:.0} m/s, (N1)60 {:.1}", row.id, row.shear_wave_velocity_m_s, row.n1_60)), tone: Tone::Info }, &anchor);
        let from = v3(origin.x - 0.5, origin.y, origin.z);
        out.annotate(Annotation::Dimension { id: format!("{}:thickness", row.id), from: from.clone(), to: v3(from.x, from.y, from.z - thickness), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, thickness), binding: Some(FieldBinding::new(format!("{path}.thicknessM"), Affine::IDENTITY, "m")) }, &from);
        top += thickness;
    }
    ground_type_label(input, f, centre, side, out);
}

/// 🌍️ The ground type derived by Table 3.1 from the layer profile and the site's N_SPT, c_u, soft clay, liquefiable and alluvium inputs beside the soil column, with the soil factor S and the corner periods T_B, T_C, T_D of the ground type that the spectra use (the site-derived one, else the declared one); a warning names the spectrum type when it differs, and everything follows every edit of a layer or of the site.
fn ground_type_label(input: &SceneInput, f: &super::model::En1998Foundation, centre: &Vec3, side: f64, out: &mut Out) {
    let layers: Vec<(f64, f64)> = input.document.soil_layers.iter().filter(|l| l.foundation_id == f.id).map(|l| (l.thickness_m, l.shear_wave_velocity_m_s)).collect();
    let site = &input.document.site;
    let profile_v_s30 = part_1::time_averaged_over_depth(&layers, 30.0);
    let v_s30 = if profile_v_s30 > 0.0 { profile_v_s30 } else { site.v_s30_m_s };
    let Some(derived) = part_1::ground_type_table_3_1(&part_1::ground_inputs_of_site(site, v_s30)) else { return };
    let used = part_1::spectrum_ground_letter_of(input.document);
    let followed = part_1::effective_v_s30(input.document);
    let s = super::response::site(input.document);
    let mismatch = input.document.annex == semio_s_plugin_norm_compliance::AnnexChoice::En && derived.spectrum_letter().is_some_and(|letter| letter != used);
    let special = derived.spectrum_letter().is_none();
    let text = |de: bool| format!("v_s,30 = {v_s30:.0} m/s \u{2192} {} {} | {} | S = {:.2}, T_B = {:.2} s, T_C = {:.2} s, T_D = {:.2} s{}{}", if de { "Baugrundklasse" } else { "ground type" }, derived.name(), followed.1.describe(&f.id, de), s.s, s.tb, s.tc, s.td, if mismatch { if de { format!(" | Spektren nutzen: {used}") } else { format!(" | spectra use: {used}") } } else { String::new() }, if special { if de { " | Sonderuntersuchung" } else { " | special study" } } else { "" });
    let anchor = v3(centre.x - 1.5 * side, centre.y - 1.5 * side, centre.z + 0.8);
    out.annotate(Annotation::Label { id: format!("{}:ground-type", f.id), anchor: anchor.clone(), text: loc(&text(false), &text(true)), tone: if mismatch || special { Tone::Warning } else { Tone::Info } }, &anchor);
}

/// 🏗️ The design actions N_Ed, V_Ed and M_Ed of the Annex F bearing check as arrows on the footing, coloured by the surface check.
fn footing_actions(input: &SceneInput, f: &super::model::En1998Foundation, centre: &Vec3, side: f64, path: &str, out: &mut Out) {
    if f.footing_width_m <= 0.0 || f.bearing_n_max_n <= 0.0 {
        return;
    }
    let binding = binding_exact(input.report, &format!("en1998.5.{}.annexF", f.id));
    let case = load_case(&format!("seismic-{}", f.id));
    let glyphs = [
        ("n", Geometry::Load(LoadGlyph::Point { at: v3(centre.x, centre.y, centre.z + 2.0), direction: v3(0.0, 0.0, -1.0), magnitude: Quantity::new(QuantityKind::Force, f.bearing_axial_n), case: case.clone() }), loc(&format!("Axial force N_Ed = {:.0} kN", f.bearing_axial_n / 1000.0), &format!("Normalkraft N_Ed = {:.0} kN", f.bearing_axial_n / 1000.0)), v3(centre.x, centre.y, centre.z + 2.0)),
        ("v", Geometry::Load(LoadGlyph::Point { at: v3(centre.x - side / 2.0, centre.y, centre.z + 0.2), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, f.bearing_shear_n), case: case.clone() }), loc(&format!("Horizontal force V_Ed = {:.0} kN", f.bearing_shear_n / 1000.0), &format!("Horizontalkraft V_Ed = {:.0} kN", f.bearing_shear_n / 1000.0)), v3(centre.x - side / 2.0, centre.y, centre.z + 0.2)),
        ("m", Geometry::Load(LoadGlyph::Moment { at: v3(centre.x, centre.y, centre.z + 0.2), axis: Vec3::Y, magnitude: Quantity::new(QuantityKind::Moment, f.bearing_moment_nm.abs()), case }), loc(&format!("Moment M_Ed = {:.0} kN\u{b7}m", f.bearing_moment_nm / 1000.0), &format!("Moment M_Ed = {:.0} kN\u{b7}m", f.bearing_moment_nm / 1000.0)), v3(centre.x, centre.y, centre.z + 0.2)),
    ];
    for (tag, geometry, name, at) in glyphs {
        let mut spec = Spec::new(EntityKind::Load, format!("{}:bearing-{tag}", f.id), path, geometry, MaterialRole::Load { kind: LoadKind::Seismic }, name, LAYER_FORCES);
        spec.binding = binding.clone();
        out.push(spec, &[at]);
    }
}

/// 🏗️ One pile under the footing centre with its head shear, the stiffness interface of the soil and draggable dimensions of the length and the interface depth; the pile is coloured by the moment check.
fn pile(input: &SceneInput, handles: &Index, f: &super::model::En1998Foundation, centre: &Vec3, path: &str, out: &mut Out) {
    if f.pile_diameter_m <= 0.0 || f.pile_length_m <= 0.0 {
        return;
    }
    let top = centre.z - FOOTING_DEPTH;
    let foot = v3(centre.x, centre.y, top - f.pile_length_m);
    let id = format!("{}:pile", f.id);
    let binding = binding_exact(input.report, &format!("en1998.5.{}.pile.moment", f.id));
    let mut spec = Spec::new(EntityKind::Solid, id.clone(), path, Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: world_frame(foot.clone()), radius: f.pile_diameter_m / 2.0, height: f.pile_length_m } }), MaterialRole::Concrete, loc(&format!("Pile of {}", f.id), &format!("Pfahl von {}", f.id)), LAYER_GROUND);
    spec.binding = binding;
    spec.handles = handles.of(&id);
    spec.detail = loc(&format!("Pile diameter {:.2} m, length {:.1} m, head {}", f.pile_diameter_m, f.pile_length_m, if f.pile_head_fixed >= 0.5 { "fixed" } else { "free" }), &format!("Pfahldurchmesser {:.2} m, L\u{e4}nge {:.1} m, Kopf {}", f.pile_diameter_m, f.pile_length_m, if f.pile_head_fixed >= 0.5 { "eingespannt" } else { "frei" }));
    out.push(spec, &[v3(foot.x - f.pile_diameter_m / 2.0, foot.y - f.pile_diameter_m / 2.0, foot.z), v3(foot.x + f.pile_diameter_m / 2.0, foot.y + f.pile_diameter_m / 2.0, top)]);
    if f.pile_head_shear_n > 0.0 {
        let at = v3(centre.x, centre.y, top);
        out.push(Spec::new(EntityKind::Load, format!("{}:pile-shear", f.id), path, Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, f.pile_head_shear_n), case: load_case(&format!("seismic-{}", f.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Inertial head shear {:.0} kN", f.pile_head_shear_n / 1000.0), &format!("Tr\u{e4}gheitskraft am Pfahlkopf {:.0} kN", f.pile_head_shear_n / 1000.0)), LAYER_FORCES), &[at]);
    }
    let from = v3(centre.x + f.pile_diameter_m, centre.y, top);
    out.annotate(Annotation::Dimension { id: format!("{}:pile-length", f.id), from: from.clone(), to: v3(from.x, from.y, foot.z), offset: v3(1.0, 0.0, 0.0), value: Quantity::new(QuantityKind::Length, f.pile_length_m), binding: Some(FieldBinding::new(format!("{path}.pileLengthM"), Affine::IDENTITY, "m")) }, &from);
    if f.pile_interface_depth_m > 0.0 {
        let plane = v3(centre.x - 3.0, centre.y - 3.0, top - f.pile_interface_depth_m);
        let label = loc(&format!("Stiffness interface: v_s {:.0} to {:.0} m/s, G\u{2082}/G\u{2081} = {:.1}", f.pile_upper_vs_m_s, f.pile_lower_vs_m_s, part_5::stiffness_contrast(f.pile_upper_vs_m_s, f.pile_lower_vs_m_s)), &format!("Steifigkeitssprung: v_s {:.0} auf {:.0} m/s, G\u{2082}/G\u{2081} = {:.1}", f.pile_upper_vs_m_s, f.pile_lower_vs_m_s, part_5::stiffness_contrast(f.pile_upper_vs_m_s, f.pile_lower_vs_m_s)));
        let mut interface = Spec::new(EntityKind::Solid, format!("{}:pile-interface", f.id), path, box_solid(plane.clone(), v3(6.0, 6.0, PILE_LAYER_THICKNESS)), MaterialRole::Soil { class: "interface".into() }, label.clone(), LAYER_GROUND);
        interface.opacity = 0.4;
        interface.handles = handles.of(&format!("{}:pile-interface", f.id));
        out.push(interface, &[plane.clone(), v3(plane.x + 6.0, plane.y + 6.0, plane.z + PILE_LAYER_THICKNESS)]);
        let dimension_from = v3(centre.x - f.pile_diameter_m, centre.y, top);
        out.annotate(Annotation::Dimension { id: format!("{}:pile-interface-depth", f.id), from: dimension_from.clone(), to: v3(dimension_from.x, dimension_from.y, plane.z), offset: v3(-1.0, 0.0, 0.0), value: Quantity::new(QuantityKind::Length, f.pile_interface_depth_m), binding: Some(FieldBinding::new(format!("{path}.pileInterfaceDepthM"), Affine::IDENTITY, "m")) }, &dimension_from);
        out.annotate(Annotation::Label { id: format!("{}:pile-interface-label", f.id), anchor: plane.clone(), text: label, tone: Tone::Info }, &plane);
    }
}

fn wall(input: &SceneInput, layout: &Layout, handles: &Index, w: &super::model::En1998RetainingWall, out: &mut Out) {
    let o = layout.origin(&w.id);
    let h = w.height_m.max(0.1);
    let path = path_of("retainingWalls", &w.id);
    let binding = binding_prefix(input.report, &format!("en1998.5.{}.", w.id));
    let mut spec = Spec::new(EntityKind::Solid, w.id.clone(), path.clone(), Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: world_frame(o.clone()), base: rect_ring(WALL_LENGTH, WALL_THICKNESS), height: h } }), MaterialRole::Concrete, loc(&format!("Retaining wall {}", w.id), &format!("St\u{fc}tzwand {}", w.id)), LAYER_GROUND);
    spec.binding = binding.clone();
    spec.handles = handles.of(&w.id);
    spec.detail = loc(&format!("Wall {}, height {:.1} m, backfill friction angle {:.0} degrees", w.id, h, w.phi_deg), &format!("Wand {}, H\u{f6}he {:.1} m, Reibungswinkel der Hinterf\u{fc}llung {:.0} Grad", w.id, h, w.phi_deg));
    out.push(spec, &[o.clone(), v3(o.x + WALL_LENGTH, o.y + WALL_THICKNESS, o.z + h)]);
    if w.water_height_m > 0.0 {
        let wet = w.water_height_m.min(h);
        let behind = v3(o.x, o.y + WALL_THICKNESS, o.z);
        let id = format!("{}:water", w.id);
        let mut water = Spec::new(EntityKind::Solid, id.clone(), path.clone(), Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: world_frame(behind.clone()), base: rect_ring(WALL_LENGTH, WALL_BACKFILL_WIDTH), height: wet } }), MaterialRole::Neutral, loc(&format!("Retained water of {} up to {wet:.2} m", w.id), &format!("Angestautes Wasser von {} bis {wet:.2} m", w.id)), LAYER_GROUND);
        water.opacity = 0.3;
        water.binding = binding_exact(input.report, &format!("en1998.5.{}.water", w.id));
        water.handles = handles.of(&id);
        out.push(water, &[behind.clone(), v3(o.x + WALL_LENGTH, behind.y + WALL_BACKFILL_WIDTH, o.z + wet)]);
        if let Some(dynamic) = computed_of(input.report, &format!("en1998.5.{}.water", w.id)).filter(|v| *v > 0.0) {
            let at = v3(o.x + 0.5 * WALL_LENGTH, behind.y, o.z + 0.4 * wet);
            out.push(Spec::new(EntityKind::Load, format!("{}:water-thrust", w.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: v3(0.0, -1.0, 0.0), magnitude: Quantity::new(QuantityKind::Force, dynamic), case: load_case(&format!("seismic-{}", w.id)) }), MaterialRole::Load, loc(&format!("Water thrust on {}", w.id), &format!("Wasserschub auf {}", w.id)), LAYER_GROUND), &[at]);
        }
    }
    if let Some(thrust) = computed_of(input.report, &format!("en1998.5.{}.thrust", w.id)).filter(|v| *v > 0.0) {
        let start = v3(o.x, o.y + WALL_THICKNESS, o.z + h / 3.0);
        let end = v3(o.x + WALL_LENGTH, start.y, start.z);
        out.push(Spec::new(EntityKind::Load, format!("{}:thrust", w.id), path.clone(), Geometry::Load(LoadGlyph::Line { path: polyline(&[start.clone(), end]), direction: v3(0.0, -1.0, 0.0), profile: LoadProfile::Uniform { value: Quantity::new(QuantityKind::ForcePerLength, thrust) }, case: load_case(&format!("seismic-{}", w.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Seismic thrust {:.0} kN/m on {}", thrust / 1000.0, w.id), &format!("Erddruck aus Erdbeben {:.0} kN/m auf {}", thrust / 1000.0, w.id)), LAYER_FORCES), &[start]);
    }
}

/// ➡️ The capacity-design arrows of a ductile pier: the plastic hinge moment M_Rd at the base, the capacity-design shear V_CD = \u{3b3}\u{2080}\u{b7}M_Rd/L_s (the value of the shear check) at the top and the shear resistance V_Rd opposing it; the demand arrow is coloured by the pier shear check, the resistance arrow by nothing, and both can be dragged (V_Rd, M_Rd).
fn pier_arrows(input: &SceneInput, handles: &Index, b: &super::model::En1998Bridge, tag: &str, base: &Vec3, top: &Vec3, path: &str, out: &mut Out) {
    if b.pier_height_m <= 0.0 || b.pier_moment_rd_nm <= 0.0 {
        return;
    }
    let check = binding_exact(input.report, &format!("en1998.2.{}.pier.shear", b.id));
    let demand = computed_of(input.report, &format!("en1998.2.{}.pier.shear", b.id)).unwrap_or(0.0);
    let case = load_case(&format!("seismic-{}", b.id));
    let moment_id = format!("{}:pier-{tag}:mrd", b.id);
    let mut moment = Spec::new(EntityKind::Load, moment_id.clone(), path, Geometry::Load(LoadGlyph::Moment { at: base.clone(), axis: Vec3::Y, magnitude: Quantity::new(QuantityKind::Moment, b.pier_moment_rd_nm), case: case.clone() }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Plastic hinge moment M_Rd = {:.0} kN\u{b7}m", b.pier_moment_rd_nm / 1000.0), &format!("Plastisches Gelenkmoment M_Rd = {:.0} kN\u{b7}m", b.pier_moment_rd_nm / 1000.0)), LAYER_FORCES);
    moment.handles = handles.of(&moment_id);
    out.push(moment, &[base.clone()]);
    if demand > 0.0 {
        let mut shear = Spec::new(EntityKind::Load, format!("{}:pier-{tag}:vcd", b.id), path, Geometry::Load(LoadGlyph::Point { at: top.clone(), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, demand), case: case.clone() }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Capacity-design shear V_CD = {:.0} kN", demand / 1000.0), &format!("Kapazit\u{e4}ts-Querkraft V_CD = {:.0} kN", demand / 1000.0)), LAYER_FORCES);
        shear.binding = check;
        out.push(shear, &[top.clone()]);
    }
    if b.pier_shear_rd_n > 0.0 {
        let at = v3(top.x, top.y + PIER_SIZE, top.z - 0.5 * b.pier_height_m);
        let resistance_id = format!("{}:pier-{tag}:vrd", b.id);
        let mut resistance = Spec::new(EntityKind::Load, resistance_id.clone(), path, Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: v3(-1.0, 0.0, 0.0), magnitude: Quantity::new(QuantityKind::Force, b.pier_shear_rd_n), case }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Shear resistance V_Rd = {:.0} kN", b.pier_shear_rd_n / 1000.0), &format!("Querkrafttragf\u{e4}higkeit V_Rd = {:.0} kN", b.pier_shear_rd_n / 1000.0)), LAYER_FORCES);
        resistance.handles = handles.of(&resistance_id);
        out.push(resistance, &[at]);
    }
    let anchor = v3(top.x, top.y - PIER_SIZE, top.z + 0.5);
    out.annotate(Annotation::Label { id: format!("{}:pier-{tag}:capacity-design", b.id), anchor: anchor.clone(), text: loc(&format!("V_CD = \u{3b3}\u{2080}\u{b7}M_Rd/L_s = {:.0} kN against V_Rd = {:.0} kN", demand / 1000.0, b.pier_shear_rd_n / 1000.0), &format!("V_CD = \u{3b3}\u{2080}\u{b7}M_Rd/L_s = {:.0} kN gegen V_Rd = {:.0} kN", demand / 1000.0, b.pier_shear_rd_n / 1000.0)), tone: tone_of(&binding_exact(input.report, &format!("en1998.2.{}.pier.shear", b.id))) }, &anchor);
}

fn bridge(input: &SceneInput, layout: &Layout, handles: &Index, b: &super::model::En1998Bridge, out: &mut Out) {
    if let Some(view) = bridge_view(input.document, b, &super::response::site(input.document)) {
        bridge_rows(input, layout, handles, b, &view, out);
        return;
    }
    let o = layout.origin(&b.id);
    let path = path_of("bridges", &b.id);
    let binding = binding_prefix(input.report, &format!("en1998.2.{}.", b.id));
    let pier_height = if b.pier_height_m > 0.0 { b.pier_height_m } else { PIER_HEIGHT };
    let pier_binding = binding_exact(input.report, &format!("en1998.2.{}.pier.shear", b.id)).or_else(|| binding.clone());
    let deck = v3(o.x, o.y, o.z + pier_height);
    let mut spec = Spec::new(EntityKind::Plate, b.id.clone(), path.clone(), Geometry::Plate(PlateGeometry { frame: world_frame(deck.clone()), outline: rect_ring(DECK_LENGTH, DECK_WIDTH), thickness: DECK_THICKNESS, offset: 0.0, stiffeners: Vec::new() }), MaterialRole::Concrete, loc(&format!("Bridge deck {}", b.id), &format!("Br\u{fc}ckendeck {}", b.id)), LAYER_BRIDGES);
    spec.binding = binding.clone();
    spec.handles = handles.of(&b.id);
    spec.detail = loc(&format!("Bridge {}, period {:.2} s", b.id, b.fundamental_period_s), &format!("Br\u{fc}cke {}, Eigenperiode {:.2} s", b.id, b.fundamental_period_s));
    out.push(spec, &[deck.clone(), v3(o.x + DECK_LENGTH, o.y + DECK_WIDTH, deck.z + DECK_THICKNESS)]);
    bridge_indicators(input, b, &deck, out);
    variable_glyphs(input, handles, b, &o, &path, out);
    for (tag, x) in PIER_X {
        let base = v3(o.x + x, o.y + DECK_WIDTH / 2.0, o.z);
        let top = v3(base.x, base.y, deck.z);
        let mut pier = Spec::new(EntityKind::Member, format!("{}:pier-{tag}", b.id), path.clone(), Geometry::Member(MemberGeometry { axis: polyline(&[base.clone(), top.clone()]), frame: member_frame(&base, &Vec3::Z), section: SectionShape::parametric(ProfileKind::Rect, &[("h", PIER_SIZE), ("b", PIER_SIZE)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Pinned, stations: Vec::new() }), MaterialRole::Concrete, loc(&format!("Pier {tag} of {}", b.id), &format!("Pfeiler {tag} von {}", b.id)), LAYER_BRIDGES);
        pier.binding = pier_binding.clone();
        pier.handles = handles.of(&format!("{}:pier-{tag}", b.id));
        out.push(pier, &[base.clone(), top.clone()]);
        pier_arrows(input, handles, b, tag, &base, &top, &path, out);
        out.push(Spec::new(EntityKind::Support, format!("{}:bearing-{tag}", b.id), path.clone(), Geometry::Support(SupportGeometry { kind: SupportKind::Roller { normal: Vec3::Z }, at: top.clone(), frame: world_frame(top.clone()) }), MaterialRole::Support, loc(&format!("Bearing {tag} of {}", b.id), &format!("Lager {tag} von {}", b.id)), LAYER_BRIDGES), &[top]);
    }
    if b.pier_height_m > 0.0 {
        let from = v3(o.x + 8.0 - PIER_SIZE, o.y + DECK_WIDTH / 2.0, o.z);
        out.annotate(Annotation::Dimension { id: format!("{}:pier-height", b.id), from: from.clone(), to: v3(from.x, from.y, o.z + pier_height), offset: v3(-1.0, 0.0, 0.0), value: Quantity::new(QuantityKind::Length, pier_height), binding: Some(FieldBinding::new(format!("{path}.pierHeightM"), Affine::IDENTITY, "m")) }, &from);
    }
    if b.bearing_d_rd_m > 0.0 {
        let from = v3(o.x + 8.0, o.y - 1.0, deck.z);
        out.annotate(Annotation::Dimension { id: format!("{}:bearing-capacity", b.id), from: from.clone(), to: v3(from.x + b.bearing_d_rd_m, from.y, from.z), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, b.bearing_d_rd_m), binding: Some(FieldBinding::new(format!("{path}.bearingDRdM"), Affine::IDENTITY, "m")) }, &from);
    }
    if let Some(shear) = computed_of(input.report, &format!("en1998.2.{}.shear", b.id)).filter(|v| *v > 0.0) {
        let at = v3(o.x, o.y + DECK_WIDTH / 2.0, deck.z + DECK_THICKNESS / 2.0);
        out.push(Spec::new(EntityKind::Load, format!("{}:force", b.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, shear), case: load_case(&format!("seismic-{}", b.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Seismic base shear {:.0} kN of {}", shear / 1000.0, b.id), &format!("Erdbeben-Fu\u{df}querkraft {:.0} kN von {}", shear / 1000.0, b.id)), LAYER_FORCES), &[at]);
    }
    if let Some(clause) = binding.as_ref().and_then(|x| clause_of(input.report, &x.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", b.id), anchor: entity_ref(EntityKind::Plate, &b.id, &path), clause }, &deck);
    }
}

/// 🔗 The two ends of a tie beam: the centres of its foundations at mid footing depth; a beam whose footings coincide runs along the plan x axis by its span.
pub fn tie_beam_ends(document: &super::model::En1998Snapshot, layout: &Layout, t: &super::model::En1998TieBeam) -> Option<(Vec3, Vec3)> {
    let find = |id: &str| document.foundations.iter().find(|f| f.id == id);
    let (a, b) = (find(&t.foundation_a_id)?, find(&t.foundation_b_id)?);
    let level = |c: Vec3| v3(c.x, c.y, c.z - 0.5 * FOOTING_DEPTH);
    let (start, end) = (level(foundation_centre(document, layout, a)), level(foundation_centre(document, layout, b)));
    let length = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2) + (end.z - start.z).powi(2)).sqrt();
    Some(if length < 1e-6 { (start.clone(), v3(start.x + t.span.max(1.0), start.y, start.z)) } else { (start, end) })
}

/// 🔗 One tie beam between its two footings, coloured by its worst check, with the tie force at both ends (EN 1998-5 5.4.1.2), the span as a dimension and grips on the section and the relative displacement.
fn tie_beam(input: &SceneInput, layout: &Layout, handles: &Index, t: &super::model::En1998TieBeam, out: &mut Out) {
    let Some((start, end)) = tie_beam_ends(input.document, layout, t) else { return };
    let path = path_of("tieBeams", &t.id);
    let (width, depth) = (t.width.max(0.05), t.depth.max(0.05));
    let mut spec = Spec::new(EntityKind::Member, t.id.clone(), path.clone(), Geometry::Member(MemberGeometry { axis: polyline(&[start.clone(), end.clone()]), frame: member_frame(&start, &v3(end.x - start.x, end.y - start.y, end.z - start.z)), section: SectionShape::parametric(ProfileKind::Rect, &[("h", depth), ("b", width)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Rigid, stations: Vec::new() }), MaterialRole::Concrete, loc(&format!("Tie beam {} between {} and {}", t.id, t.foundation_a_id, t.foundation_b_id), &format!("Fundamentbalken {} zwischen {} und {}", t.id, t.foundation_a_id, t.foundation_b_id)), LAYER_GROUND);
    spec.binding = binding_prefix(input.report, &format!("en1998.5.{}.", t.id));
    spec.handles = handles.of(&t.id);
    spec.detail = loc(&format!("Section {width:.2} by {depth:.2} m, clear span {:.2} m, steel {:.0} mm\u{b2}", t.span, t.steel_area * 1.0e6), &format!("Querschnitt {width:.2} auf {depth:.2} m, lichte Spannweite {:.2} m, Stahl {:.0} mm\u{b2}", t.span, t.steel_area * 1.0e6));
    out.push(spec, &[v3(start.x.min(end.x) - 0.5 * width, start.y.min(end.y) - 0.5 * width, start.z.min(end.z) - 0.5 * depth), v3(start.x.max(end.x) + 0.5 * width, start.y.max(end.y) + 0.5 * width, start.z.max(end.z) + 0.5 * depth)]);
    if let Some(n) = computed_of(input.report, &format!("en1998.5.{}.tie", t.id)).filter(|v| *v > 0.0) {
        let length = ((end.x - start.x).powi(2) + (end.y - start.y).powi(2) + (end.z - start.z).powi(2)).sqrt().max(1e-9);
        let direction = v3((end.x - start.x) / length, (end.y - start.y) / length, (end.z - start.z) / length);
        for (tag, at, sign) in [("a", start.clone(), -1.0), ("b", end.clone(), 1.0)] {
            let mut arrow = Spec::new(EntityKind::Load, format!("{}:tie-force-{tag}", t.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: v3(sign * direction.x, sign * direction.y, sign * direction.z), magnitude: Quantity::new(QuantityKind::Force, n), case: load_case(&format!("seismic-{}", t.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Tie force N = \u{b1}{:.0} kN", n / 1000.0), &format!("Zugbandkraft N = \u{b1}{:.0} kN", n / 1000.0)), LAYER_FORCES);
            arrow.binding = binding_exact(input.report, &format!("en1998.5.{}.tie", t.id));
            out.push(arrow, &[at]);
        }
    }
    let from = v3(start.x, start.y, start.z - 0.5 * depth - 0.3);
    out.annotate(Annotation::Dimension { id: format!("{}:span", t.id), from: from.clone(), to: v3(end.x, end.y, end.z - 0.5 * depth - 0.3), offset: v3(0.0, 0.0, -0.5), value: Quantity::new(QuantityKind::Length, t.span), binding: None }, &from);
}

/// ⛰️ One slope: the embankment body, the given slip circle with its grips, the critical circle of the grid search, the pseudo-static forces on the sliding mass (EN 1998-5 4.1.3.3) and the factor of safety label, coloured by the slope check.
fn slope(input: &SceneInput, layout: &Layout, handles: &Index, sl: &super::model::En1998Slope, out: &mut Out) {
    if sl.height <= 1e-6 || sl.angle <= 1.0 || sl.angle >= 89.0 {
        return;
    }
    let o = layout.origin(&sl.id);
    let path = path_of("slopes", &sl.id);
    let run = sl.height / sl.angle.to_radians().tan();
    let y = o.y + 0.5 * SLOPE_WIDTH;
    let frame = Frame { origin: v3(o.x, y, o.z), x: Vec3::X, y: Vec3::Z, z: v3(0.0, -1.0, 0.0) };
    let at = |x: f64, z: f64| v3(o.x + x, y, o.z + z);
    let binding = binding_exact(input.report, &format!("en1998.5.{}.slope", sl.id));
    let mut body = Spec::new(EntityKind::Solid, sl.id.clone(), path.clone(), Geometry::Solid(SolidGeometry { shape: SolidShape::Prism { frame: frame.clone(), base: projection_kit::polygon2(&[[-SLOPE_CREST, 0.0], [-SLOPE_CREST, sl.height], [0.0, sl.height], [run, 0.0]]), height: SLOPE_WIDTH } }), MaterialRole::Soil { class: "slope".into() }, loc(&format!("Slope {}", sl.id), &format!("B\u{f6}schung {}", sl.id)), LAYER_GROUND);
    body.binding = binding.clone();
    body.opacity = 0.9;
    body.handles = handles.of(&sl.id);
    body.detail = loc(&format!("Height {:.2} m, angle {:.1} degrees, friction angle {:.1} degrees, cohesion {:.1} kPa, S_T {:.2}", sl.height, sl.angle, sl.friction_angle, sl.cohesion / 1000.0, sl.topographic_amplification), &format!("H\u{f6}he {:.2} m, Neigung {:.1} Grad, Reibungswinkel {:.1} Grad, Koh\u{e4}sion {:.1} kPa, S_T {:.2}", sl.height, sl.angle, sl.friction_angle, sl.cohesion / 1000.0, sl.topographic_amplification));
    out.push(body, &[v3(o.x - SLOPE_CREST, o.y - 0.5 * SLOPE_WIDTH, o.z), v3(o.x + run, o.y + 0.5 * SLOPE_WIDTH, o.z + sl.height)]);
    let crest = at(0.0, sl.height);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", sl.id), anchor: entity_ref(EntityKind::Solid, &sl.id, &path), clause }, &crest);
    }
    let site = super::response::site(input.document);
    let annex = super::response::annex_params(input.document).choice();
    let Some(forces) = part_5::slope_forces(input.document, sl, annex, site.gamma_i * site.a_gr, site.s) else { return };
    let result = part_5::slope_result(&forces, sl, sl.height, sl.angle, sl.friction_angle);
    let fos = result.least();
    let circles = [("slip-circle", result.fixed.as_ref(), "Given slip circle", "Gegebener Gleitkreis", 0.5), ("critical-circle", result.critical.as_ref(), "Critical slip circle", "Kritischer Gleitkreis", 0.25)];
    for (tag, found, en, de, opacity) in circles {
        let Some((circle, factor)) = found else { continue };
        let Some(mass) = part_5::slope_slip_mass(circle) else { continue };
        let theta = |p: [f64; 2]| (p[1] - circle.yc).atan2(p[0] - circle.xc);
        let sweep = theta(mass.exit) - theta(mass.entry);
        if sweep.abs() < 1e-6 {
            continue;
        }
        let id = format!("{}:{tag}", sl.id);
        let arc = Arc { centre: at(circle.xc, circle.yc), normal: v3(0.0, -1.0, 0.0), start: at(mass.entry[0], mass.entry[1]), sweep };
        let mut surface = Spec::new(EntityKind::Surface, id.clone(), path.clone(), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Arc { arc, width: SLOPE_WIDTH } }), MaterialRole::Soil { class: "mechanism".into() }, loc(&format!("{en} of {}", sl.id), &format!("{de} von {}", sl.id)), LAYER_GROUND);
        surface.binding = binding.clone();
        surface.opacity = opacity;
        surface.handles = handles.of(&id);
        surface.detail = loc(&format!("Bishop F_s = {factor:.2}, centre ({:.1}, {:.1}) m, radius {:.1} m", circle.xc, circle.yc, circle.r), &format!("Bishop F_s = {factor:.2}, Mittelpunkt ({:.1}, {:.1}) m, Radius {:.1} m", circle.xc, circle.yc, circle.r));
        out.push(surface, &[v3(o.x + mass.entry[0], o.y - 0.5 * SLOPE_WIDTH, o.z + mass.entry[1]), v3(o.x + mass.exit[0], o.y + 0.5 * SLOPE_WIDTH, o.z + mass.exit[1])]);
        if tag == "slip-circle" {
            let centre = at(circle.xc, circle.yc);
            out.annotate(Annotation::Label { id: format!("{}:factor", sl.id), anchor: centre.clone(), text: loc(&format!("F_s (Bishop, M2) = {factor:.2}"), &format!("F_s (Bishop, M2) = {factor:.2}")), tone: tone_of(&binding) }, &centre);
            let k = forces.pseudostatic.horizontal;
            let f_h = k * mass.weight_n_per_m * SLOPE_WIDTH;
            let seismic = at(mass.centroid[0], mass.centroid[1]);
            let case = load_case(&format!("seismic-{}", sl.id));
            let arrows = [("fh", Vec3::X, f_h, "F_H = ", "F_H = "), ("fv-down", v3(0.0, 0.0, -1.0), forces.vertical_share * f_h, "F_V = +", "F_V = +"), ("fv-up", Vec3::Z, forces.vertical_share * f_h, "F_V = \u{2212}", "F_V = \u{2212}")];
            for (suffix, direction, magnitude, en_prefix, de_prefix) in arrows {
                out.push(Spec::new(EntityKind::Load, format!("{}:{suffix}", sl.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: seismic.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, magnitude), case: case.clone() }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("{en_prefix}{:.0} kN (pseudo-static, EN 1998-5 4.1.3.3)", magnitude / 1000.0), &format!("{de_prefix}{:.0} kN (pseudostatisch, EN 1998-5 4.1.3.3)", magnitude / 1000.0)), LAYER_FORCES), &[seismic.clone()]);
            }
            if *factor < 1.0 {
                out.annotate(Annotation::FailureGlyph { id: format!("{}:failure-slip", sl.id), at: entity_ref(EntityKind::Solid, &sl.id, &path), mode: FailureMode::SlipCircle, extent: None }, &centre);
            }
        }
    }
    if fos.is_none() {
        out.annotate(Annotation::Label { id: format!("{}:factor", sl.id), anchor: crest.clone(), text: loc("No sliding mass develops", "Keine Gleitmasse entsteht"), tone: Tone::Info }, &crest);
    }
}
