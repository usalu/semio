//! 🧮️ Norm entities of wave 3 in 3D: the bolt forces and the instantaneous centre of a joint plate, the stress range bars of a fatigue detail, the buckling length of a column, the work hardening of a cold-formed member, the sheets and fasteners of a screwed joint and the ductility bars of a material.

use super::kit::*;
use super::layout::Layout;
use super::plate_layout::{bolt_positions, plate_pose};
use super::scene::*;
use super::{checks, SceneInput};

const BAR_WIDTH: f64 = 0.08;
const BAR_HEIGHT: f64 = 0.5;
const ARROW_FRACTION: f64 = 0.12;

fn case_ref() -> LoadCaseRef {
    LoadCaseRef { id: "norm".into(), kind: LoadKind::Variable, psi: None }
}

fn rect(width: f64, depth: f64) -> Polygon2 {
    let (w, h) = (width / 2.0, depth / 2.0);
    Polygon2::solid(vec![Vec2::new(-w, -h), Vec2::new(w, -h), Vec2::new(w, h), Vec2::new(-w, h)])
}

struct Bars<'a> {
    collection: &'static str,
    owner: &'a str,
    base: Vec3,
    frame: Frame,
    binding: Option<UtilisationBinding>,
}

impl Bars<'_> {
    fn bar(&self, out: &mut Out, tag: &str, slot: f64, height: f64, name: (&str, &str), text: &str, utilised: bool) {
        let h = height.max(0.004);
        let origin = along(&along(&self.base, &self.frame.y, slot * (BAR_WIDTH * 1.5)), &self.frame.z, h / 2.0);
        out.push(
            Spec {
                kind: EntityKind::Plate,
                id: format!("{}:{tag}", self.owner),
                collection: self.collection,
                geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: origin.clone(), x: self.frame.x.clone(), y: self.frame.y.clone(), z: self.frame.z.clone() }, outline: rect(BAR_WIDTH, BAR_WIDTH), thickness: h, offset: 0.0, stiffeners: Vec::new() }),
                role: MaterialRole::Steel,
                binding: if utilised { self.binding.clone() } else { None },
                name: loc(name.0, name.1),
                detail: loc(text, text),
                layer: LAYER_FATIGUE,
                handles: Vec::new(),
                deformation: None,
            },
            &[self.base.clone(), origin],
        );
    }

    fn label(&self, out: &mut Out, tag: &str, slot: f64, height: f64, text: &str) {
        let at = along(&along(&self.base, &self.frame.y, slot * (BAR_WIDTH * 1.5)), &self.frame.z, height + 0.08);
        out.annotate(Annotation::Label { id: format!("{}:{tag}-label", self.owner), anchor: at.clone(), text: loc(text, text), tone: tone_of(&self.binding) }, &at);
    }
}

fn group(out: &mut Out, collection: &'static str, id: &str, host: EntityRef, name: LocalizedCopy, detail: LocalizedCopy, binding: &Option<UtilisationBinding>, at: &Vec3) {
    out.push(Spec { kind: EntityKind::Group, id: id.to_owned(), collection, geometry: Geometry::Group { members: vec![host] }, role: MaterialRole::Steel, binding: binding.clone(), name, detail, layer: LAYER_ANNOTATIONS, handles: Vec::new(), deformation: None }, &[at.clone()]);
}

fn bolt_forces(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for p in &d.joint_plates {
        if !out.own("jointPlates", &p.id) {
            continue;
        }
        let (Some(pose), Some(view)) = (plate_pose(d, layout, p), checks::bolt_group_ic_checks::view(d, p, checks::AnnexParams::for_choice(d.annex).gamma_m2)) else { continue };
        let binding = binding(input.report, &[p.id.as_str()]);
        let forces = view.plastic.as_ref().unwrap_or(&view.elastic);
        let peak = forces.iter().map(|v| v.0.hypot(v.1)).fold(0.0, f64::max);
        for (i, (at, v)) in bolt_positions(&pose, p).into_iter().zip(forces).enumerate() {
            let magnitude = v.0.hypot(v.1);
            if magnitude < 1.0 || peak <= 0.0 {
                continue;
            }
            let direction = unit(&add(&mul(&pose.across, v.0 / magnitude), &mul(&pose.line, v.1 / magnitude)));
            let from = along(&at, &pose.normal, pose.top + ARROW_FRACTION * magnitude / peak);
            out.push(
                Spec {
                    kind: EntityKind::Load,
                    id: format!("{}:bolt-force-{}", p.id, i + 1),
                    collection: "jointPlates",
                    geometry: Geometry::Load(LoadGlyph::Point { at: from.clone(), direction, magnitude: Quantity::new(QuantityKind::Force, magnitude), case: case_ref() }),
                    role: MaterialRole::Load { kind: LoadKind::Variable },
                    binding: binding.clone(),
                    name: loc(&format!("Force on bolt {} of {}", i + 1, p.id), &format!("Kraft auf Schraube {} von {}", i + 1, p.id)),
                    detail: loc(&format!("{:.1} kN ({})", magnitude / 1e3, if view.plastic.is_some() { "instantaneous centre" } else { "elastic" }), &format!("{:.1} kN ({})", magnitude / 1e3, if view.plastic.is_some() { "Momentanpol" } else { "elastisch" })),
                    layer: LAYER_LOADS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[from],
            );
        }
        if let Some((cx, cy)) = view.centre {
            let at = along(&along(&pose.bolt_centre, &pose.across, cx / 1e3), &pose.line, cy / 1e3);
            let frame = Frame { origin: along(&at, &pose.normal, -pose.bottom), x: pose.across.clone(), y: pose.line.clone(), z: pose.normal.clone() };
            out.push(
                Spec {
                    kind: EntityKind::Solid,
                    id: format!("{}:ic", p.id),
                    collection: "jointPlates",
                    geometry: Geometry::Solid(SolidGeometry { shape: SolidShape::Cylinder { frame, radius: 0.006, height: pose.top + pose.bottom } }),
                    role: MaterialRole::Neutral,
                    binding: binding.clone(),
                    name: loc(&format!("Instantaneous centre of {}", p.id), &format!("Momentanpol von {}", p.id)),
                    detail: loc(&format!("({cx:.1}, {cy:.1}) mm from the bolt group centroid"), &format!("({cx:.1}, {cy:.1}) mm vom Schwerpunkt der Schraubengruppe")),
                    layer: LAYER_JOINTS,
                    handles: Vec::new(),
                    deformation: None,
                },
                &[at.clone()],
            );
            let label = along(&at, &pose.normal, pose.top + 0.08);
            out.annotate(Annotation::Label { id: format!("{}:ic-label", p.id), anchor: label.clone(), text: loc("IC", "MP"), tone: tone_of(&binding) }, &label);
        }
    }
}

fn fatigue_ranges(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for s in &d.fatigue_stresses {
        if !out.own("fatigueStresses", &s.id) {
            continue;
        }
        let Some(r) = checks::stress_range_checks::derive(d, s, d.annex) else { continue };
        let Some(detail) = d.fatigue_details.iter().find(|x| x.id == s.detail_id) else { continue };
        let Some((pose, section)) = layout.member(&detail.member_id).zip(d.members.iter().find(|m| m.id == detail.member_id).and_then(|m| d.sections.iter().find(|x| x.id == m.section_id))) else { continue };
        let f = &pose.frame;
        let bars = Bars { collection: "fatigueStresses", owner: &s.id, base: along(&along(&f.origin, &f.x, pose.length * 0.25), &f.z, section.h / 2.0 + 0.6), frame: f.clone(), binding: binding(input.report, &[s.id.as_str()]) };
        let reference = (r.strength / r.gamma_mf).max(1.0);
        let (raw, design) = (BAR_HEIGHT * r.raw / reference, BAR_HEIGHT * r.design / reference);
        bars.bar(out, "raw", 0.0, raw, ("Stress range of the load model", "Spannungsschwingbreite des Lastmodells"), &format!("Δσ = {:.1} MPa (σ_max {:.1}, σ_min {:.1})", r.raw / 1e6, r.sigma_max / 1e6, r.sigma_min / 1e6), false);
        bars.label(out, "raw", 0.0, raw, &format!("Δσ = {:.1} MPa", r.raw / 1e6));
        bars.bar(out, "design", 1.0, design, ("Design stress range", "Bemessungsschwingbreite"), &format!("γ_Ff k_f Δσ = {:.1} MPa", r.design / 1e6), true);
        bars.label(out, "design", 1.0, design, &format!("{:.1} MPa", r.design / 1e6));
        bars.bar(out, "strength", 2.0, BAR_HEIGHT, ("Detail strength", "Kerbfallfestigkeit"), &format!("k_s Δσ_C/γ_Mf = {:.1} MPa", r.strength / r.gamma_mf / 1e6), false);
        bars.label(out, "strength", 2.0, BAR_HEIGHT, &format!("D = {:.3}", r.damage));
        group(out, "fatigueStresses", &s.id, entity_ref(EntityKind::Group, &detail.id, "fatigueDetails"), loc(&format!("Stress range {}", s.id), &format!("Spannungsschwingbreite {}", s.id)), loc(&format!("{:.2e} cycles on detail {}", s.cycles, detail.id), &format!("{:.2e} Lastspiele am Detail {}", s.cycles, detail.id)), &bars.binding, &bars.base);
    }
}

fn buckling_lengths(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for b in &d.buckling_lengths {
        if !out.own("bucklingLengths", &b.id) {
            continue;
        }
        let (Some(pose), Some((ga, gb, k))) = (layout.member(&b.member_id), checks::effective_length_checks::factors(b)) else { continue };
        let f = &pose.frame;
        let binding = binding(input.report, &[b.id.as_str()]);
        let (end, buckling) = (along(&f.origin, &f.x, pose.length), along(&f.origin, &f.x, k * pose.length));
        group(out, "bucklingLengths", &b.id, entity_ref(EntityKind::Member, &b.member_id, "members"), loc(&format!("Buckling length {}", b.id), &format!("Knicklänge {}", b.id)), loc(&format!("K = {k:.3}, G_A = {:.2}, G_B = {:.2}", ga.min(1.0e3), gb.min(1.0e3)), &format!("K = {k:.3}, G_A = {:.2}, G_B = {:.2}", ga.min(1.0e3), gb.min(1.0e3))), &binding, &end);
        out.annotate(Annotation::Dimension { id: format!("{}:length", b.id), from: f.origin.clone(), to: end, offset: mul(&f.y, 0.4), value: Quantity::length_m(pose.length), binding: None }, &f.origin);
        out.annotate(Annotation::Dimension { id: format!("{}:buckling", b.id), from: f.origin.clone(), to: buckling.clone(), offset: mul(&f.y, 0.8), value: Quantity::length_m(k * pose.length), binding: None }, &buckling);
        let at = along(&buckling, &f.y, 0.8);
        out.annotate(Annotation::Label { id: format!("{}:k-label", b.id), anchor: at.clone(), text: loc(&format!("L_cr = {k:.3} L"), &format!("L_cr = {k:.3} L")), tone: tone_of(&binding) }, &at);
    }
}

fn forming_gains(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for g in &d.forming_gains {
        if !out.own("formingGains", &g.id) {
            continue;
        }
        let Some(m) = d.cold_formed_members.iter().find(|m| m.id == g.member_id) else { continue };
        if m.fy <= 0.0 || m.thickness <= 0.0 || g.gross_area <= 0.0 || g.ultimate_strength <= m.fy {
            continue;
        }
        let fya = checks::forming_gain_checks::average_yield(g, checks::forming_gain_checks::bends_counted(g, m.thickness, m.corner_radius), m.fy, m.thickness);
        let origin = layout.origin(&g.member_id);
        let bars = Bars { collection: "formingGains", owner: &g.id, base: along(&origin, &Vec3::Z, 0.4), frame: Frame { origin: origin.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, binding: binding(input.report, &[g.id.as_str()]) };
        let used = if g.used_yield > 0.0 { g.used_yield } else { m.fy };
        let cap = (g.ultimate_strength + m.fy) / 2.0;
        for (slot, tag, value, en, de, utilised) in [(0.0, "fyb", m.fy, "Basic yield strength f_yb", "Grundstreckgrenze f_yb", false), (1.0, "fya", fya, "Average yield strength f_ya", "Mittlere Streckgrenze f_ya", false), (2.0, "used", used, "Yield strength used", "Angesetzte Streckgrenze", true), (3.0, "cap", cap, "Limit (f_u + f_yb)/2", "Grenze (f_u + f_yb)/2", false)] {
            let height = BAR_HEIGHT * value / cap;
            bars.bar(out, tag, slot, height, (en, de), &format!("{:.1} MPa", value / 1e6), utilised);
            bars.label(out, tag, slot, height, &format!("{:.0} MPa", value / 1e6));
        }
        group(out, "formingGains", &g.id, entity_ref(EntityKind::Member, &g.member_id, "coldFormedMembers"), loc(&format!("Forming gain {}", g.id), &format!("Kaltverfestigung {}", g.id)), loc(&format!("f_ya = {:.1} MPa from {} bends", fya / 1e6, g.bends), &format!("f_ya = {:.1} MPa aus {} Biegungen", fya / 1e6, g.bends)), &bars.binding, &origin);
    }
}

fn fastened_joints(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for f in &d.fastened_joints {
        if !out.own("fastenedJoints", &f.id) {
            continue;
        }
        let Some(r) = checks::fastener_checks::resistance(f, checks::AnnexParams::for_choice(d.annex).gamma_m2) else { continue };
        let origin = along(&layout.origin(&f.member_id), &Vec3::Z, 0.6);
        let binding = binding(input.report, &[f.id.as_str()]);
        let per_row = f.per_row.max(1);
        let rows = f.count.div_ceil(per_row).max(1);
        let length = 2.0 * f.end_distance + f.pitch * f64::from(rows - 1);
        let sheet = |id: String, z: f64, t: f64, name: (&str, &str)| {
            let centre = along(&origin, &Vec3::Z, z + t / 2.0);
            (Spec { kind: EntityKind::Plate, id, collection: "fastenedJoints", geometry: Geometry::Plate(PlateGeometry { frame: Frame { origin: centre.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, outline: rect(length, f.sheet_width.max(2.0 * f.edge_distance)), thickness: t, offset: 0.0, stiffeners: Vec::new() }), role: MaterialRole::Steel, binding: binding.clone(), name: loc(name.0, name.1), detail: loc(&format!("t = {:.2} mm", t * 1e3), &format!("t = {:.2} mm", t * 1e3)), layer: LAYER_JOINTS, handles: Vec::new(), deformation: None }, centre)
        };
        let (base, head) = (sheet(format!("{}:base", f.id), 0.0, f.base_thickness, ("Base sheet", "Untere Lage")), sheet(format!("{}:head", f.id), f.base_thickness, f.head_thickness, ("Head sheet", "Obere Lage")));
        let top = f.base_thickness + f.head_thickness;
        out.push(base.0, &[base.1]);
        out.push(head.0, &[head.1]);
        let mut members = vec![entity_ref(EntityKind::Plate, &format!("{}:base", f.id), "fastenedJoints"), entity_ref(EntityKind::Plate, &format!("{}:head", f.id), "fastenedJoints")];
        for i in 0..f.count {
            let (row, col) = (i / per_row, i % per_row);
            let at = along(&along(&origin, &Vec3::X, -length / 2.0 + f.end_distance + f64::from(row) * f.pitch), &Vec3::Y, (f64::from(col) - (f64::from(per_row) - 1.0) / 2.0) * f.gauge);
            let (from, to) = (along(&at, &Vec3::Z, top + 0.004), along(&at, &Vec3::Z, -0.004));
            let id = format!("{}:fastener-{}", f.id, i + 1);
            let kind = if f.kind == super::model::FastenerKind::SelfTappingScrew { FastenerKind::Screw } else { FastenerKind::Dowel };
            out.push(
                Spec { kind: EntityKind::Fastener, id: id.clone(), collection: "fastenedJoints", geometry: Geometry::Fastener(FastenerGeometry { kind, axis: Segment3 { from: from.clone(), to: to.clone() }, diameter: f.diameter.max(1e-3), head: HeadShape::Round, pattern: None, hole_clearance: None }), role: MaterialRole::Bolt, binding: binding.clone(), name: loc(&format!("Fastener {} of {}", i + 1, f.id), &format!("Verbindungsmittel {} von {}", i + 1, f.id)), detail: loc(&format!("⌀ {:.1} mm", f.diameter * 1e3), &format!("⌀ {:.1} mm", f.diameter * 1e3)), layer: LAYER_JOINTS, handles: Vec::new(), deformation: None },
                &[from, to],
            );
            members.push(entity_ref(EntityKind::Fastener, &id, "fastenedJoints"));
        }
        let shear_at = along(&origin, &Vec3::Z, top);
        if f.shear_force != 0.0 {
            out.push(
                Spec { kind: EntityKind::Load, id: format!("{}:shear", f.id), collection: "fastenedJoints", geometry: Geometry::Load(LoadGlyph::Point { at: shear_at.clone(), direction: if f.shear_force > 0.0 { Vec3::X } else { mul(&Vec3::X, -1.0) }, magnitude: Quantity::new(QuantityKind::Force, f.shear_force.abs()), case: case_ref() }), role: MaterialRole::Load { kind: LoadKind::Variable }, binding: binding.clone(), name: loc("Shear force", "Querkraft"), detail: loc(&format!("{:.2} kN, F_b,Rd = {:.2} kN per fastener", f.shear_force.abs() / 1e3, r.bearing / 1e3), &format!("{:.2} kN, F_b,Rd = {:.2} kN je Verbindungsmittel", f.shear_force.abs() / 1e3, r.bearing / 1e3)), layer: LAYER_LOADS, handles: Vec::new(), deformation: None },
                &[shear_at.clone()],
            );
        }
        if f.tension_force != 0.0 {
            out.push(
                Spec { kind: EntityKind::Load, id: format!("{}:tension", f.id), collection: "fastenedJoints", geometry: Geometry::Load(LoadGlyph::Point { at: shear_at.clone(), direction: if f.tension_force > 0.0 { Vec3::Z } else { mul(&Vec3::Z, -1.0) }, magnitude: Quantity::new(QuantityKind::Force, f.tension_force.abs()), case: case_ref() }), role: MaterialRole::Load { kind: LoadKind::Variable }, binding: binding.clone(), name: loc("Tension force", "Zugkraft"), detail: loc(&format!("{:.2} kN", f.tension_force.abs() / 1e3), &format!("{:.2} kN", f.tension_force.abs() / 1e3)), layer: LAYER_LOADS, handles: Vec::new(), deformation: None },
                &[shear_at.clone()],
            );
        }
        let text = format!("{} × ⌀ {:.1} mm", f.count, f.diameter * 1e3);
        out.push(Spec { kind: EntityKind::Group, id: f.id.clone(), collection: "fastenedJoints", geometry: Geometry::Group { members }, role: MaterialRole::Steel, binding: binding.clone(), name: loc(&format!("Fastened joint {}", f.id), &format!("Befestigungsverbindung {}", f.id)), detail: loc(&text, &text), layer: LAYER_JOINTS, handles: Vec::new(), deformation: None }, &[origin.clone()]);
        out.annotate(Annotation::Label { id: format!("{}:label", f.id), anchor: shear_at.clone(), text: loc(&text, &text), tone: tone_of(&binding) }, &shear_at);
    }
}

fn ductility(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for c in &d.material_ductility {
        if !out.own("materialDuctility", &c.id) {
            continue;
        }
        let Some(m) = d.materials.iter().find(|m| m.id == c.material_id) else { continue };
        if m.fy <= 0.0 || m.fu <= 0.0 || m.e_modulus <= 0.0 {
            continue;
        }
        let origin = layout.origin(&c.material_id);
        let bars = Bars { collection: "materialDuctility", owner: &c.id, base: along(&origin, &Vec3::Z, 0.5), frame: Frame { origin: origin.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, binding: binding(input.report, &[c.id.as_str()]) };
        let rule = semio_s_plugin_norm_compliance::ndp_common;
        let rows = [
            ("ratio", "f_u/f_y", "f_u/f_y", m.fu / m.fy, rule("ndp.en1993.1-1.ductility.fu-fy")),
            ("elongation", "Elongation", "Bruchdehnung", c.elongation, rule("ndp.en1993.1-1.ductility.elongation")),
            ("strain", "Ultimate strain", "Gleichmaßdehnung", c.ultimate_strain, rule("ndp.en1993.1-1.ductility.eps-u-factor") * m.fy / m.e_modulus),
        ];
        for (k, (tag, en, de, value, required)) in rows.iter().enumerate() {
            if *required <= 0.0 {
                continue;
            }
            let slot = 2.0 * k as f64;
            let (height, limit) = (BAR_HEIGHT * (value / required).min(2.0) / 2.0, BAR_HEIGHT / 2.0);
            bars.bar(out, tag, slot, height, (*en, *de), &format!("{value:.4} against {required:.4}"), true);
            bars.bar(out, &format!("{tag}-required"), slot + 1.0, limit, (&format!("{en} required"), &format!("{de} gefordert")), &format!("{required:.4}"), false);
            bars.label(out, tag, slot, height, &format!("{value:.3} / {required:.3}"));
        }
        group(out, "materialDuctility", &c.id, entity_ref(EntityKind::Plate, &c.material_id, "materials"), loc(&format!("Ductility {}", c.id), &format!("Duktilität {}", c.id)), loc(&format!("Material {}", c.material_id), &format!("Werkstoff {}", c.material_id)), &bars.binding, &origin);
    }
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    bolt_forces(input, layout, out);
    fatigue_ranges(input, layout, out);
    buckling_lengths(input, layout, out);
    forming_gains(input, layout, out);
    fastened_joints(input, layout, out);
    ductility(input, layout, out);
}
