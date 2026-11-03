//! 🥫️ Cold-formed sheeting (EN 1999-1-4) as thin-walled profiles along the span and cylindrical shells (EN 1999-1-5) as surfaces of revolution.

use super::kit::*;
use super::layout::Layout;
use super::scene::*;
use super::{handles, SceneInput};

const RIB_HEIGHT: f64 = 0.2;
const MIN_FLAT: f64 = 0.01;

/// 🥫️ Top-hat centreline of developed flat width `b`: rib height `0.2 b`, 1:2 inclined webs, flanges sharing the rest; centred on the section origin.
pub fn hat_centreline(b: f64) -> Polyline2 {
    let h = RIB_HEIGHT * b;
    let dx = 0.5 * h;
    let slant = h.hypot(dx);
    let top = (0.5 * b - slant).max(0.05 * b);
    let foot = (0.5 * top).max(0.02 * b);
    let half = 0.5 * (2.0 * foot + top + 2.0 * dx);
    let mut x = -half;
    let mut points = vec![Vec2::new(x, -h / 2.0)];
    for (dx_step, z) in [(foot, -h / 2.0), (dx, h / 2.0), (top, h / 2.0), (dx, -h / 2.0), (foot, -h / 2.0)] {
        x += dx_step;
        points.push(Vec2::new(x, z));
    }
    Polyline2 { points }
}

/// 🪗 Centreline of `width` of the trapezoidal ribs of a profiled sheet (the pitch repeated `round(width/pitch)` times, at least once), centred on the section origin.
pub fn rib_centreline(rib: &super::checks::part_1_4_1_5::Rib, width: f64) -> Polyline2 {
    let count = (width / rib.pitch).round().max(1.0) as usize;
    let (h, e) = (rib.height, rib.projection());
    let mut x = -0.5 * count as f64 * rib.pitch;
    let mut points = vec![Vec2::new(x, -h / 2.0)];
    for _ in 0..count {
        for (dx, z) in [(rib.bottom, -h / 2.0), (e, h / 2.0), (rib.top, h / 2.0), (e, -h / 2.0)] {
            x += dx;
            points.push(Vec2::new(x, z));
        }
    }
    Polyline2 { points }
}

pub fn project(input: &SceneInput, layout: &Layout, out: &mut Out) {
    let d = input.document;
    for sheet in d.cold_formed.iter().filter(|s| s.thickness > 1e-6 && s.width > MIN_FLAT && s.span > 1e-6) {
        if !out.own("coldFormed", &sheet.id) {
            continue;
        }
        let o = layout.origin(&sheet.id);
        let end = along(&o, &Vec3::X, sheet.span);
        let frame = member_frame(&o, &Vec3::X);
        let profile = d.sheet_profiles.iter().find(|p| p.sheet_id == sheet.id).map(|p| super::checks::part_1_4_1_5::Rib { pitch: p.pitch, top: p.top_flange, bottom: p.bottom_flange, height: p.web_height }).filter(super::checks::part_1_4_1_5::Rib::is_valid);
        let centreline = profile.map_or_else(|| hat_centreline(sheet.width), |r| rib_centreline(&r, sheet.width));
        let walls = vec![sheet.thickness; centreline.points.len() - 1];
        let binding = out.binding(input, &[sheet.id.as_str()]);
        let rib = profile.map_or(RIB_HEIGHT * sheet.width, |r| r.height);
        out.push(
            Spec {
                kind: EntityKind::Member,
                id: sheet.id.clone(),
                collection: "coldFormed",
                geometry: Geometry::Member(MemberGeometry {
                    axis: polyline(&[o.clone(), end.clone()]),
                    frame: frame.clone(),
                    section: SectionShape::ThinWalled { centreline, thickness: walls },
                    section_taper: None,
                    start_offset: Vec2::ZERO,
                    end_offset: Vec2::ZERO,
                    start_release: EndRelease::Pinned,
                    end_release: EndRelease::Pinned,
                    stations: Vec::new(),
                }),
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                name: loc(&format!("Cold-formed sheet {}", sheet.id), &format!("Kaltprofil {}", sheet.id)),
                detail: loc(&format!("flat width {:.0} mm, t = {:.2} mm, span {:.2} m{}", sheet.width * 1e3, sheet.thickness * 1e3, sheet.span, if sheet.welded { ", welded" } else { "" }), &format!("Flachbreite {:.0} mm, t = {:.2} mm, Stützweite {:.2} m{}", sheet.width * 1e3, sheet.thickness * 1e3, sheet.span, if sheet.welded { ", geschweißt" } else { "" })),
                layer: LAYER_SHEETS,
                handles: handles::ids(d, layout, &sheet.id),
                deformation: None,
                style: None,
            },
            &[o.clone(), end.clone()],
        );
        let label_at = along(&along(&o, &Vec3::X, sheet.span / 2.0), &Vec3::Z, rib + 0.1);
        out.annotate(Annotation::Label { id: format!("{}:label", sheet.id), anchor: label_at.clone(), text: loc(&format!("{} t = {:.2} mm", sheet.id, sheet.thickness * 1e3), &format!("{} t = {:.2} mm", sheet.id, sheet.thickness * 1e3)), tone: tone_of(&binding) }, &label_at);
        let dim_at = along(&o, &Vec3::Z, -(rib / 2.0 + 0.2));
        out.annotate(
            Annotation::Dimension { id: format!("{}:span", sheet.id), from: dim_at.clone(), to: along(&dim_at, &Vec3::X, sheet.span), offset: Vec3::ZERO, value: Quantity::new(QuantityKind::Length, sheet.span), binding: Some(FieldBinding::new(format!("coldFormed[id={}].span", sheet.id), Affine::IDENTITY, "m")) },
            &dim_at,
        );
        for (tag, at, kind, name) in [("support-a", o.clone(), SupportKind::Pinned, loc("Pinned support", "Gelenklager")), ("support-b", end.clone(), SupportKind::Roller { normal: Vec3::Z }, loc("Roller support", "Rollenlager"))] {
            let at = along(&at, &Vec3::Z, -rib / 2.0);
            out.push(Spec { kind: EntityKind::Support, id: format!("{}:{tag}", sheet.id), collection: "coldFormed", geometry: Geometry::Support(SupportGeometry { kind, frame: Frame { origin: at.clone(), x: Vec3::X, y: Vec3::Y, z: Vec3::Z }, at: at.clone() }), role: MaterialRole::Support, binding: None, detail: name.clone(), name, layer: LAYER_SUPPORTS, handles: Vec::new(), deformation: None, style: None }, &[at]);
        }
        super::members::action_glyphs(&sheet.id, "coldFormed", &sheet.actions, &frame, sheet.span, rib, out);
    }
    for shell in d.shells.iter().filter(|s| s.radius > 1e-6 && s.length > 1e-6 && s.thickness > 1e-6) {
        if !out.own("shells", &shell.id) {
            continue;
        }
        let o = layout.origin(&shell.id);
        let binding = out.binding(input, &[shell.id.as_str()]);
        out.push(
            Spec {
                kind: EntityKind::Shell,
                id: shell.id.clone(),
                collection: "shells",
                geometry: Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: o.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(shell.radius, 0.0), Vec2::new(shell.radius, shell.length)] }, sweep: std::f64::consts::TAU }, thickness: shell.thickness }),
                role: MaterialRole::Aluminium,
                binding: binding.clone(),
                name: loc(&format!("Shell {}", shell.id), &format!("Schale {}", shell.id)),
                detail: loc(&format!("R = {:.2} m, L = {:.2} m, t = {:.1} mm", shell.radius, shell.length, shell.thickness * 1e3), &format!("R = {:.2} m, L = {:.2} m, t = {:.1} mm", shell.radius, shell.length, shell.thickness * 1e3)),
                layer: LAYER_SHELLS,
                handles: handles::ids(d, layout, &shell.id),
                deformation: None,
                style: None,
            },
            &[v3(o.x - shell.radius, o.y - shell.radius, o.z), v3(o.x + shell.radius, o.y + shell.radius, o.z + shell.length)],
        );
        let mut cuts: Vec<f64> = d.ring_stiffeners.iter().filter(|r| r.shell_id == shell.id && r.x > 0.0 && r.x < shell.length).map(|r| r.x).collect();
        cuts.sort_by(f64::total_cmp);
        cuts.dedup();
        let edges: Vec<f64> = std::iter::once(0.0).chain(cuts).chain(std::iter::once(shell.length)).collect();
        let bay = edges.windows(2).map(|w| (w[0], w[1])).fold((0.0, 0.0), |best, b| if b.1 - b.0 > best.1 - best.0 + 1e-12 { b } else { best });
        if let Some(mode) = super::eigen::shell_axial(shell.radius, shell.thickness, shell.length, bay) {
            let demand = input.report.checks.iter().find(|c| c.id == format!("en1999.1-5.buckle.{}", shell.id)).map(|c| c.computed.value.abs()).filter(|s| *s > 0.0);
            let critical = super::checks::part_1_4_1_5::sigma_x_rcr(shell.radius, shell.thickness, bay.1 - bay.0);
            let overlay = DeformationOverlay { kind: DeformationKind::Mode, index: super::tracks::Mode::Shell.index(), frequency: None, participation: 1.0, scale: 0.1 * mode.half_wave, samples: Vec::new(), surface: mode.samples.clone(), motion: Motion::Harmonic, load_factor: demand.map(|s| critical / s) };
            super::tracks::push_shell_mode(out, &shell.id, &o, shell.radius, shell.thickness, &mode, overlay);
        }
        for ring in d.ring_stiffeners.iter().filter(|r| r.shell_id == shell.id && r.depth > 1e-6 && r.thickness > 1e-6 && r.x > 0.0 && r.x < shell.length) {
            if !out.own("ringStiffeners", &ring.id) {
                continue;
            }
            let skin = 0.5 * shell.thickness;
            let (inner, outer) = if ring.internal { ((shell.radius - skin - ring.depth).max(1e-3), shell.radius - skin) } else { (shell.radius + skin, shell.radius + skin + ring.depth) };
            let binding = out.binding(input, &[ring.id.as_str()]);
            let name = loc(&format!("Ring stiffener {} at x = {:.2} m: {:.0} × {:.0} mm", ring.id, ring.x, ring.depth * 1e3, ring.thickness * 1e3), &format!("Ringsteife {} bei x = {:.2} m: {:.0} × {:.0} mm", ring.id, ring.x, ring.depth * 1e3, ring.thickness * 1e3));
            out.push(
                Spec {
                    kind: EntityKind::Shell,
                    id: ring.id.clone(),
                    collection: "ringStiffeners",
                    geometry: Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: o.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: vec![Vec2::new(inner, ring.x), Vec2::new(outer, ring.x)] }, sweep: std::f64::consts::TAU }, thickness: ring.thickness }),
                    role: MaterialRole::Aluminium,
                    binding: binding.clone(),
                    detail: name.clone(),
                    name,
                    layer: LAYER_SHELLS,
                    handles: handles::ids(d, layout, &ring.id),
                    deformation: None,
                    style: None,
                },
                &[v3(o.x - outer, o.y - outer, o.z + ring.x), v3(o.x + outer, o.y + outer, o.z + ring.x)],
            );
            let at = v3(o.x + outer, o.y, o.z + ring.x);
            if let Some(check) = super::kit::projection_kit::check_with(input.report, &format!("en1999.1-5.stiffener.{}", ring.id)) {
                out.annotate(Annotation::Leader { id: format!("{}:stiffener-label", ring.id), anchor: at.clone(), text: loc(&format!("Ring {}: {}", ring.id, check.explanation.en), &format!("Ring {}: {}", ring.id, check.explanation.de)), offset_px: vec![24, -24] }, &at);
            }
            if binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) {
                out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", ring.id), at: entity_ref(EntityKind::Shell, &ring.id, "ringStiffeners"), mode: FailureMode::Other, extent: None }, &at);
            }
        }
        let top = v3(o.x, o.y, o.z + shell.length);
        out.annotate(Annotation::Label { id: format!("{}:label", shell.id), anchor: top.clone(), text: loc(&format!("{} r/t = {:.0}", shell.id, shell.radius / shell.thickness), &format!("{} r/t = {:.0}", shell.id, shell.radius / shell.thickness)), tone: tone_of(&binding) }, &top);
        if binding.as_ref().is_some_and(|b| b.status == CheckStatus::Fail) {
            out.annotate(Annotation::FailureGlyph { id: format!("{}:failure", shell.id), at: entity_ref(EntityKind::Shell, &shell.id, "shells"), mode: FailureMode::Other, extent: None }, &top);
        }
    }
}
