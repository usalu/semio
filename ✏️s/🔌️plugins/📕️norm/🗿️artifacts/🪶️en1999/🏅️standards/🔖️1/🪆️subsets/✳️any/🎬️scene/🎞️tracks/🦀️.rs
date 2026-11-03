//! 🎞️ Tracks of the EN 1999 scene: the buckling-mode index, each mode drawn by a sibling of its aluminium member on its own layer so the viewport reveals one eigenform at a time.

use super::kit::*;
use super::scene::*;

pub const LAYER_MODE_FLEXURAL: &str = "mode-flexural";
pub const LAYER_MODE_LTB: &str = "mode-ltb";
pub const LAYER_MODE_TF: &str = "mode-torsional-flexural";
pub const LAYER_MODE_SHELL: &str = "mode-shell";

pub const MODES: [(&str, &str, &str); 4] = [(LAYER_MODE_FLEXURAL, "Flexural buckling", "Biegeknicken"), (LAYER_MODE_LTB, "Lateral-torsional buckling", "Biegedrillknicken"), (LAYER_MODE_TF, "Torsional-flexural buckling", "Drillknicken und Biegedrillknicken"), (LAYER_MODE_SHELL, "Axial shell buckling", "Axiales Schalenbeulen")];

/// 🌀️ The eigenforms a member can show, in track order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Flexural,
    LateralTorsional,
    TorsionalFlexural,
    Shell,
}

impl Mode {
    pub fn index(self) -> u32 {
        self as u32 + 1
    }
}

/// 🌀️ Appends the mode sibling `<member>:mode-<kind>` carrying the eigenform overlay on the member's own geometry.
#[allow(clippy::too_many_arguments)]
pub fn push_mode(out: &mut Out, mode: Mode, member: &str, section: SectionShape, frame: &Frame, length: f64, shape: DeformationOverlay, end_release: EndRelease) {
    let (layer, en, de) = MODES[mode as usize];
    let end = along(&frame.origin, &frame.x, length);
    let name = loc(&format!("Member {member}: {en}"), &format!("Bauteil {member}: {de}"));
    out.push(
        Spec {
            kind: EntityKind::Member,
            id: format!("{member}:{layer}"),
            collection: "members",
            geometry: Geometry::Member(MemberGeometry { axis: polyline(&[frame.origin.clone(), end.clone()]), frame: frame.clone(), section, section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: EndRelease::Rigid, end_release, stations: Vec::new() }),
            role: MaterialRole::Aluminium,
            binding: None,
            detail: name.clone(),
            name,
            layer,
            handles: Vec::new(),
            deformation: Some(shape),
            style: None,
        },
        &[frame.origin.clone(), end],
    );
}

/// 🥫️ Appends the shell mode sibling `<shell>:mode-shell`: the shell with a generatrix refined at the pattern nodes and the surface displacement of the axial buckling pattern as its overlay.
#[allow(clippy::too_many_arguments)]
pub fn push_shell_mode(out: &mut Out, shell: &str, origin: &Vec3, radius: f64, thickness: f64, mode: &super::eigen::ShellMode, overlay: DeformationOverlay) {
    let (layer, en, de) = MODES[Mode::Shell as usize];
    let top = along(origin, &Vec3::Z, mode.generatrix.last().copied().unwrap_or(0.0));
    let name = loc(&format!("Shell {shell}: {en}"), &format!("Schale {shell}: {de}"));
    out.push(
        Spec {
            kind: EntityKind::Shell,
            id: format!("{shell}:{layer}"),
            collection: "shells",
            geometry: Geometry::Shell(ShellGeometry { shape: ShellShape::Revolved { origin: origin.clone(), axis: Vec3::Z, generatrix: Polyline2 { points: mode.generatrix.iter().map(|x| Vec2::new(radius, *x)).collect() }, sweep: std::f64::consts::TAU }, thickness }),
            role: MaterialRole::Aluminium,
            binding: None,
            detail: loc(&format!("{} half-waves of {:.1} mm between x = {:.2} m and {:.2} m", mode.waves, mode.half_wave * 1e3, mode.window.0, mode.window.1), &format!("{} Halbwellen von {:.1} mm zwischen x = {:.2} m und {:.2} m", mode.waves, mode.half_wave * 1e3, mode.window.0, mode.window.1)),
            name,
            layer,
            handles: Vec::new(),
            deformation: Some(overlay),
            style: None,
        },
        &[origin.clone(), top],
    );
}

/// 🎞️ The tracks of the finished entity list.
pub fn finish(scene: &mut StructuralScene) {
    let present: Vec<&str> = MODES.iter().map(|m| m.0).filter(|layer| scene.entities.iter().any(|e| e.layer == *layer)).collect();
    for entity in scene.entities.iter_mut() {
        if let (Some(position), Some(overlay)) = (present.iter().position(|layer| *layer == entity.layer), entity.appearance.deformation.as_mut()) {
            overlay.index = position as u32 + 1;
        }
    }
    scene.tracks.extend(projection_kit::tracks::mode_track(&scene.entities, &MODES));
    scene.tracks.extend(fire_track(&scene.entities));
}

/// 🔥️ The `fire` time track over the strip temperature fields present in the scene: one keyframe per fire time of [`super::fire_and_fatigue::FIRE_TIMES_S`], blended linearly.
pub fn fire_track(entities: &[SceneEntity]) -> Option<ParameterTrack> {
    let present = entities.iter().any(|e| matches!(&e.appearance.channel, ColourChannel::Fields { fields, .. } if fields.iter().any(|f| f.id.starts_with(super::fire_and_fatigue::FIRE_FIELD_PREFIX))));
    if !present {
        return None;
    }
    let stops: Vec<(f64, String, String)> = super::fire_and_fatigue::FIRE_TIMES_S.iter().map(|s| (*s, super::fire_and_fatigue::fire_field_id(*s), format!("{:.0} min", s / 60.0))).collect();
    let rows: Vec<(f64, &str, &str, &str)> = stops.iter().map(|(at, id, name)| (*at, id.as_str(), name.as_str(), name.as_str())).collect();
    projection_kit::tracks::timed("fire", TrackKind::Time, ("Fire exposure time", "Branddauer"), &rows)
}

/// 🔀️ The EN 1990 ULS combinations (6.10a and 6.10b for every leading variable action) the member checks run, as `(English, German)` captions in member order.
pub fn combination_captions(document: &super::model::En1999Snapshot) -> Vec<(String, String)> {
    let params = super::checks::na_de::AnnexParams::for_choice(document.annex);
    document
        .members
        .iter()
        .flat_map(|member| super::checks::part_en1990::uls_combinations(member, &params, document.reliability_class))
        .map(|g| {
            let equation = match g.combination {
                "uls-610a" => "6.10a",
                "uls-610b" => "6.10b",
                _ => "6.10",
            };
            (format!("ULS {equation} · lead {}", g.action_id), format!("GZT {equation} · führend {}", g.action_id))
        })
        .collect()
}
