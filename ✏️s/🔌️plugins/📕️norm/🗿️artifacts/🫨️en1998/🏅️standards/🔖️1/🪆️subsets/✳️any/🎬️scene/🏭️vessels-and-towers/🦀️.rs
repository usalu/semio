//! 🏭️ Silos, tanks and towers: revolved shells with the filled content, cantilever towers and chimneys, base shear arrows from the settled checks and bound height and radius dimensions.

use super::handles::Index;
use super::kit::*;
use super::layout::Layout;
use super::response::Site;
use super::scene::*;
use super::schema::{part_1, part_6, tank};
use super::structure_views::{tank_view, tower_view};
use super::SceneInput;

const SHELL_THICKNESS: f64 = 0.02;

struct Vessel<'a> {
    id: &'a str,
    collection: &'static str,
    height: f64,
    radius: f64,
    filling: f64,
    force_check: &'a str,
    en: &'static str,
    de: &'static str,
}

pub fn project(input: &SceneInput, layout: &Layout, site: &Site, handles: &Index, out: &mut Out) {
    let d = input.document;
    for s in &d.silos {
        if !out.own("silos", &s.id) {
            continue;
        }
        vessel(input, layout, handles, &Vessel { id: &s.id, collection: "silos", height: s.height_m, radius: s.radius_m, filling: s.filling_ratio, force_check: "wall", en: "Silo", de: "Silo" }, out);
    }
    for t in &d.tanks {
        if !out.own("tanks", &t.id) {
            continue;
        }
        vessel(input, layout, handles, &Vessel { id: &t.id, collection: "tanks", height: t.height_m, radius: t.radius_m, filling: t.filling_ratio, force_check: "shear", en: "Tank", de: "Tank" }, out);
        tank_hydrodynamics(input, layout, handles, site, t, out);
    }
    for t in &d.towers {
        if !out.own("towers", &t.id) {
            continue;
        }
        tower(input, layout, site, handles, t, out);
    }
}

fn vessel(input: &SceneInput, layout: &Layout, handles: &Index, v: &Vessel<'_>, out: &mut Out) {
    let o = layout.origin(v.id);
    let path = path_of(v.collection, v.id);
    let (h, r) = (v.height.max(0.1), v.radius.max(0.1));
    let binding = binding_prefix(input.report, &format!("en1998.4.{}.", v.id));
    let name = loc(&format!("{} {}", v.en, v.id), &format!("{} {}", v.de, v.id));
    let mut shell = Spec::new(EntityKind::Shell, v.id, path.clone(), Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: o.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(r, 0.0), Vec2::new(r, h)] }, sweep: std::f64::consts::TAU }, thickness: SHELL_THICKNESS }), MaterialRole::Steel, name.clone(), LAYER_VESSELS);
    shell.binding = binding.clone();
    shell.opacity = 0.6;
    shell.handles = handles.of(v.id);
    shell.detail = loc(&format!("{} {}, height {:.1} m, radius {:.2} m, filling {:.0} %", v.en, v.id, h, r, v.filling * 100.0), &format!("{} {}, H\u{f6}he {:.1} m, Radius {:.2} m, F\u{fc}llgrad {:.0} %", v.de, v.id, h, r, v.filling * 100.0));
    out.push(shell, &[v3(o.x - r, o.y - r, o.z), v3(o.x + r, o.y + r, o.z + h)]);
    let fill = v.filling.clamp(0.0, 1.0);
    if fill > 0.0 {
        let mut content = Spec::new(EntityKind::Solid, format!("{}:content", v.id), path.clone(), Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame: world_frame(o.clone()), radius: r * 0.97, height: h * fill } }), MaterialRole::Soil { class: "content".into() }, loc(&format!("Content of {}", v.id), &format!("F\u{fc}llgut von {}", v.id)), LAYER_VESSELS);
        content.opacity = 0.5;
        out.push(content, &[o.clone()]);
    }
    if let Some(force) = computed_of(input.report, &format!("en1998.4.{}.{}", v.id, v.force_check)).filter(|f| *f > 0.0) {
        let at = v3(o.x - r, o.y, o.z + h * 0.5);
        out.push(Spec::new(EntityKind::Load, format!("{}:force", v.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, force), case: load_case(&format!("seismic-{}", v.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Seismic base shear {:.0} kN of {}", force / 1000.0, v.id), &format!("Erdbeben-Fu\u{df}querkraft {:.0} kN von {}", force / 1000.0, v.id)), LAYER_FORCES), &[at]);
    }
    let from = v3(o.x + r + 1.0, o.y, o.z);
    out.annotate(Annotation::Dimension { id: format!("{}:height", v.id), from: from.clone(), to: v3(from.x, from.y, o.z + h), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, h), binding: Some(FieldBinding::new(format!("{path}.heightM"), Affine::IDENTITY, "m")) }, &from);
    let label = v3(o.x, o.y, o.z + h + 0.5);
    out.annotate(Annotation::Label { id: format!("{}:label", v.id), anchor: label.clone(), text: name, tone: tone_of(&binding) }, &label);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", v.id), anchor: entity_ref(EntityKind::Shell, v.id, &path), clause }, &label);
    }
}

/// 🌊️ The hydrodynamic wall pressure of a tank as a sampled load along the wall in the plane of the ground motion, the sloshing surface of the first convective mode over the fluid level, and the freeboard as a draggable dimension of the filling ratio.
fn tank_hydrodynamics(input: &SceneInput, layout: &Layout, handles: &Index, site: &Site, t: &super::model::En1998Tank, out: &mut Out) {
    let view = tank_view(t, site);
    let (r, fluid) = (t.radius_m, view.fluid_height_m);
    if r <= 0.0 || fluid <= 0.0 || view.profile.is_empty() {
        return;
    }
    let o = layout.origin(&t.id);
    let path = path_of("tanks", &t.id);
    let (start, end) = (v3(o.x - r, o.y, o.z), v3(o.x - r, o.y, o.z + fluid));
    let peak = view.profile.iter().map(|row| row.total_pa).fold(0.0_f64, f64::max);
    let samples = view.profile.iter().map(|row| ProfileSample { at: row.z_m / fluid, value: Quantity::new(QuantityKind::Pressure, row.total_pa) }).collect();
    let mut pressure = Spec::new(EntityKind::Load, format!("{}:pressure", t.id), path.clone(), Geometry::Load(LoadGlyph::Line { path: polyline(&[start.clone(), end]), direction: Vec3::X, profile: LoadProfile::Sampled { samples }, case: load_case(&format!("seismic-{}", t.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Hydrostatic and hydrodynamic wall pressure, peak {:.1} kPa", peak / 1000.0), &format!("Hydrostatischer und hydrodynamischer Wanddruck, Spitze {:.1} kPa", peak / 1000.0)), LAYER_FORCES);
    pressure.binding = binding_exact(input.report, &format!("en1998.4.{}.pressure", t.id));
    pressure.handles = handles.of(&format!("{}:pressure", t.id));
    out.push(pressure, &[start.clone()]);
    for (tag, offset, en, de) in [("impulsive", -0.25, "Impulsive", "Impulsiver"), ("convective", 0.25, "Convective", "Konvektiver")] {
        let shifted = v3(start.x, start.y + offset * r, start.z);
        let samples = view.profile.iter().map(|row| ProfileSample { at: row.z_m / fluid, value: Quantity::new(QuantityKind::Pressure, if tag == "impulsive" { row.impulsive_pa } else { row.convective_pa }) }).collect();
        let peak_part = view.profile.iter().map(|row| if tag == "impulsive" { row.impulsive_pa } else { row.convective_pa }).fold(0.0_f64, f64::max);
        let mut part = Spec::new(EntityKind::Load, format!("{}:pressure-{tag}", t.id), path.clone(), Geometry::Load(LoadGlyph::Line { path: polyline(&[shifted.clone(), v3(shifted.x, shifted.y, shifted.z + fluid)]), direction: Vec3::X, profile: LoadProfile::Sampled { samples }, case: load_case(&format!("seismic-{}", t.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("{en} wall pressure of {}, peak {:.1} kPa", t.id, peak_part / 1000.0), &format!("{de} Wanddruck von {}, Spitze {:.1} kPa", t.id, peak_part / 1000.0)), LAYER_VESSELS);
        part.binding = binding_exact(input.report, &format!("en1998.4.{}.pressure", t.id));
        out.push(part, &[shifted]);
    }
    let slosh = computed_of(input.report, &format!("en1998.4.{}.slosh", t.id)).unwrap_or(0.0);
    let (rows, columns) = (SLOSH_RINGS, SLOSH_SECTORS);
    let level = o.z + fluid;
    let points: Vec<Vec3> = (0..rows).flat_map(|i| (0..columns).map(move |j| (i, j))).map(|(i, j)| {
        let radius = r * i as f64 / (rows - 1) as f64;
        let theta = std::f64::consts::TAU * j as f64 / (columns - 1) as f64;
        v3(o.x + radius * theta.cos(), o.y + radius * theta.sin(), level + tank::surface_elevation_m(slosh, r, radius, theta))
    }).collect();
    let mut surface = Spec::new(EntityKind::Surface, format!("{}:slosh", t.id), path.clone(), Geometry::Surface(SurfaceGeometry { shape: SurfaceShape::Grid { grid: PointGrid { rows: rows as u32, columns: columns as u32, points: points.clone() } } }), MaterialRole::Soil { class: "content".into() }, loc(&format!("Sloshing surface of the first convective mode, wave height {:.2} m at T_c = {:.2} s", slosh, view.convective_period_s), &format!("Schwappfl\u{e4}che der ersten Konvektionsform, Wellenh\u{f6}he {:.2} m bei T_c = {:.2} s", slosh, view.convective_period_s)), LAYER_VESSELS);
    surface.binding = binding_exact(input.report, &format!("en1998.4.{}.slosh", t.id));
    surface.opacity = 0.45;
    out.push(surface, &points);
    if slosh > 0.0 {
        let from = v3(o.x + r + 1.0, o.y, level);
        out.annotate(Annotation::Dimension { id: format!("{}:slosh-dim", t.id), from: from.clone(), to: v3(from.x, from.y, level + slosh), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, slosh), binding: None }, &from);
        let at = v3(o.x, o.y, level + slosh);
        out.annotate(Annotation::Label { id: format!("{}:slosh-height", t.id), anchor: at.clone(), text: loc(&format!("Sloshing height d_max = {slosh:.2} m, T_c = {:.2} s", view.convective_period_s), &format!("Schwappwellenh\u{f6}he d_max = {slosh:.2} m, T_c = {:.2} s", view.convective_period_s)), tone: tone_of(&binding_exact(input.report, &format!("en1998.4.{}.slosh", t.id))) }, &at);
    }
    if t.height_m > fluid {
        let from = v3(o.x + r + 2.0, o.y, level);
        out.annotate(Annotation::Dimension { id: format!("{}:freeboard", t.id), from: from.clone(), to: v3(from.x, from.y, o.z + t.height_m), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, t.height_m - fluid), binding: Some(FieldBinding::new(format!("{path}.fillingRatio"), Affine { base: 1.0, k: -1.0 / t.height_m }, "1")) }, &from);
    }
}

const SLOSH_RINGS: usize = 6;
const SLOSH_SECTORS: usize = 25;

fn tower(input: &SceneInput, layout: &Layout, site: &Site, handles: &Index, t: &super::model::En1998Tower, out: &mut Out) {
    let o = layout.origin(&t.id);
    let path = path_of("towers", &t.id);
    let h = t.height_m.max(0.1);
    let diameter = (h / 15.0).clamp(0.6, 6.0);
    let top = v3(o.x, o.y, o.z + h);
    let frame = member_frame(&o, &Vec3::Z);
    let binding = binding_prefix(input.report, &format!("en1998.6.{}.", t.id));
    let (en, de) = if t.is_chimney { ("Chimney", "Schornstein") } else { ("Tower", "Turm") };
    let mut spec = Spec::new(EntityKind::Member, t.id.clone(), path.clone(), Geometry::Member(MemberGeometry { axis: polyline(&[o.clone(), top.clone()]), frame, section: SectionShape::parametric(ProfileKind::Chs, &[("d", diameter), ("t", diameter / 50.0)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Free, stations: Vec::new() }), MaterialRole::Steel, loc(&format!("{en} {}", t.id), &format!("{de} {}", t.id)), LAYER_TOWERS);
    spec.binding = binding.clone();
    spec.handles = handles.of(&t.id);
    spec.detail = loc(&format!("{en} {}, height {:.1} m, q {:.2}", t.id, h, t.q_nominal), &format!("{de} {}, H\u{f6}he {:.1} m, q {:.2}", t.id, h, t.q_nominal));
    out.push(spec, &[o.clone(), top.clone()]);
    mode_ghosts(input, site, t, &o, diameter, &path, out);
    for mark in super::structure_views::variables::marks(&t.id, &path, &t.variables, t.permanent_gk_n, &top) {
        let mut glyph = Spec::new(EntityKind::Load, mark.id.clone(), mark.path.clone(), mark.glyph(), MaterialRole::Load { kind: LoadKind::Variable }, mark.name(), LAYER_TOWERS);
        glyph.handles = handles.of(&mark.id);
        out.push(glyph, &[mark.at.clone()]);
    }
    let (base_kind, base_en, base_de) = if t.base_rotational_stiffness_nm > 0.0 { (SupportKind::Spring { stiffness: vec![Quantity::new(QuantityKind::Stiffness, t.base_rotational_stiffness_nm)] }, format!("Foundation spring k\u{3b8} = {:.3e} N\u{b7}m/rad", t.base_rotational_stiffness_nm), format!("Gr\u{fc}ndungsfeder k\u{3b8} = {:.3e} N\u{b7}m/rad", t.base_rotational_stiffness_nm)) } else { (SupportKind::Fixed, "Fixed base".to_owned(), "Einspannung".to_owned()) };
    let mut base = Spec::new(EntityKind::Support, format!("{}:base", t.id), path.clone(), Geometry::Support(SupportGeometry { kind: base_kind, at: o.clone(), frame: world_frame(o.clone()) }), MaterialRole::Support, loc(&base_en, &base_de), LAYER_TOWERS);
    base.binding = binding_exact(input.report, &format!("en1998.6.{}.interaction", t.id));
    base.handles = handles.of(&format!("{}:base", t.id));
    out.push(base, &[o.clone()]);
    if let Some(moment) = computed_of(input.report, &format!("en1998.6.{}.overturning", t.id)).filter(|m| *m > 0.0) {
        let shear = moment / (2.0 / 3.0 * h);
        let at = v3(o.x, o.y, o.z + 2.0 / 3.0 * h);
        out.push(Spec::new(EntityKind::Load, format!("{}:force", t.id), path.clone(), Geometry::Load(LoadGlyph::Point { at: at.clone(), direction: Vec3::X, magnitude: Quantity::new(QuantityKind::Force, shear), case: load_case(&format!("seismic-{}", t.id)) }), MaterialRole::Load { kind: LoadKind::Seismic }, loc(&format!("Seismic base shear {:.0} kN of {}", shear / 1000.0, t.id), &format!("Erdbeben-Fu\u{df}querkraft {:.0} kN von {}", shear / 1000.0, t.id)), LAYER_FORCES), &[at]);
    }
    let from = v3(o.x + diameter, o.y, o.z);
    out.annotate(Annotation::Dimension { id: format!("{}:height", t.id), from: from.clone(), to: v3(from.x, from.y, o.z + h), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, h), binding: Some(FieldBinding::new(format!("{path}.heightM"), Affine::IDENTITY, "m")) }, &from);
    let period = computed_of(input.report, &format!("en1998.6.{}.period", t.id));
    let suffix_en = period.map_or(String::new(), |p| format!(", T1 = {p:.2} s"));
    out.annotate(Annotation::Label { id: format!("{}:label", t.id), anchor: top.clone(), text: loc(&format!("{en} {}{suffix_en}", t.id), &format!("{de} {}{suffix_en}", t.id)), tone: tone_of(&binding) }, &top);
    if let Some(clause) = binding.as_ref().and_then(|b| clause_of(input.report, &b.check_id)) {
        out.annotate(Annotation::Clause { id: format!("{}:clause", t.id), anchor: entity_ref(EntityKind::Member, &t.id, &path), clause }, &top);
    }
}

const TOWER_MODES: u32 = 3;
const TOWER_STATIONS: usize = 16;
const TOWER_MODE_TIP: f64 = 0.06;

/// 🌊️ The first three cantilever modes of a tower with a given stiffness as animated ghost members on the mode layers: tip-normalised Euler-Bernoulli shapes, periods T_n = T_1·(β₁L/β_nL)², amplitude Γ_n·S_e(T_n)·(T_n/2π)²·γ_I and the participation factor Γ_n of the uniform mass.
fn mode_ghosts(input: &SceneInput, site: &Site, t: &super::model::En1998Tower, o: &Vec3, diameter: f64, path: &str, out: &mut Out) {
    if let Some(view) = tower_view(t, site) {
        let h = t.height_m.max(0.1);
        let top = v3(o.x, o.y, o.z + h);
        for (index, mode) in view.modes.modes.iter().enumerate().take(TOWER_MODES as usize) {
            let n = index as u32 + 1;
            let s_e = part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, mode.period_s);
            let amplitude = mode.gamma * site.gamma_i * s_e * (mode.period_s / (2.0 * std::f64::consts::PI)).powi(2);
            let scale = if amplitude > 1e-12 { (TOWER_MODE_TIP * h / amplitude).max(1.0) } else { 1.0 };
            let samples = view.modes.elevations_m.iter().zip(&mode.shape).map(|(z, phi)| DisplacementSample { at: z / h, delta: v3(amplitude * phi, 0.0, 0.0) }).collect();
            let geometry = Geometry::Member(MemberGeometry { axis: polyline(&[o.clone(), top.clone()]), frame: member_frame(o, &Vec3::Z), section: SectionShape::parametric(ProfileKind::Chs, &[("d", diameter), ("t", diameter / 50.0)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Free, stations: Vec::new() });
            let name = loc(&format!("Mode {n} of {}: T = {:.3} s, \u{393} = {:.2}, M_eff = {:.0} %", t.id, mode.period_s, mode.gamma, mode.mass_ratio * 100.0), &format!("Eigenform {n} von {}: T = {:.3} s, \u{393} = {:.2}, M_eff = {:.0} %", t.id, mode.period_s, mode.gamma, mode.mass_ratio * 100.0));
            let mut spec = Spec::new(EntityKind::Member, format!("{}:mode-x{n}", t.id), path, geometry, MaterialRole::Neutral, name, tower_mode_layer(n));
            spec.opacity = 0.35;
            spec.deformation = Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: n, frequency: Some(Quantity::new(QuantityKind::Frequency, 1.0 / mode.period_s)), participation: mode.gamma, scale, samples });
            out.push(spec, &[o.clone(), top.clone()]);
        }
        return;
    }
    let Some(t1) = computed_of(input.report, &format!("en1998.6.{}.period", t.id)).filter(|p| *p > 0.0) else { return };
    let h = t.height_m.max(0.1);
    let top = v3(o.x, o.y, o.z + h);
    for n in 1..=TOWER_MODES {
        let period = t1 * part_6::cantilever_period_ratio(n);
        let gamma = part_6::cantilever_participation(n);
        let s_e = part_1::elastic_response_spectrum(site.a_gr, site.s, site.tb, site.tc, site.td, period);
        let amplitude = gamma * site.gamma_i * s_e * (period / (2.0 * std::f64::consts::PI)).powi(2);
        let scale = if amplitude > 1e-12 { (TOWER_MODE_TIP * h / amplitude).max(1.0) } else { 1.0 };
        let samples = (0..=TOWER_STATIONS).map(|i| { let at = i as f64 / TOWER_STATIONS as f64; DisplacementSample { at, delta: v3(amplitude * part_6::cantilever_mode_shape(n, at), 0.0, 0.0) } }).collect();
        let geometry = Geometry::Member(MemberGeometry { axis: polyline(&[o.clone(), top.clone()]), frame: member_frame(o, &Vec3::Z), section: SectionShape::parametric(ProfileKind::Chs, &[("d", diameter), ("t", diameter / 50.0)]), section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release: EndRelease::Free, stations: Vec::new() });
        let name = loc(&format!("Mode {n} of {}: T = {period:.3} s, \u{393} = {gamma:.2}", t.id), &format!("Eigenform {n} von {}: T = {period:.3} s, \u{393} = {gamma:.2}", t.id));
        let mut spec = Spec::new(EntityKind::Member, format!("{}:mode-x{n}", t.id), path, geometry, MaterialRole::Neutral, name, tower_mode_layer(n));
        spec.opacity = 0.35;
        spec.deformation = Some(DeformationOverlay { surface: Vec::new(), motion: Motion::Harmonic, load_factor: None, kind: DeformationKind::Mode, index: n, frequency: Some(Quantity::new(QuantityKind::Frequency, 1.0 / period)), participation: gamma, scale, samples });
        out.push(spec, &[o.clone(), top.clone()]);
    }
}
