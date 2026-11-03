//! 🎞️ Tracks of the EN 1993 scene: the buckling-mode index, each mode drawn by a sibling of its member on its own layer so the viewport reveals one eigenform at a time.

use super::kit::*;
use super::scene::*;

pub const LAYER_MODE_FLEXURAL: &str = "mode-flexural";
pub const LAYER_MODE_LTB: &str = "mode-ltb";
pub const LAYER_MODE_FRAME: &str = "mode-frame";

pub const MODES: [(&str, &str, &str); 3] = [(LAYER_MODE_FLEXURAL, "Flexural buckling", "Biegeknicken"), (LAYER_MODE_LTB, "Lateral-torsional buckling", "Biegedrillknicken"), (LAYER_MODE_FRAME, "Frame buckling", "Rahmenknicken")];

/// 🌀️ The eigenforms a member can show, in track order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    Flexural,
    LateralTorsional,
    Frame,
}

impl Mode {
    pub fn index(self) -> u32 {
        self as u32 + 1
    }

    fn row(self) -> (&'static str, &'static str, &'static str) {
        MODES[self as usize]
    }
}

/// 🌀️ Appends the mode sibling `<member>:mode:<layer>` carrying the eigenform overlay on the member's own geometry.
#[allow(clippy::too_many_arguments)]
pub fn push_mode(out: &mut Out, mode: Mode, collection: &'static str, member: &str, label: &str, section: SectionShape, frame: &Frame, length: f64, shape: DeformationOverlay, release: EndRelease) {
    let (layer, en, de) = mode.row();
    let end = along(&frame.origin, &frame.x, length);
    let name = loc(&format!("{label}: {en}"), &format!("{label}: {de}"));
    out.push(
        Spec {
            kind: EntityKind::Member,
            id: format!("{member}:{layer}"),
            collection,
            geometry: Geometry::Member(MemberGeometry { axis: polyline(&[frame.origin.clone(), end.clone()]), frame: frame.clone(), section, section_taper: None, start_offset: Vec2::ZERO, end_offset: Vec2::ZERO, start_release: release, end_release: release, stations: Vec::new() }),
            role: MaterialRole::Steel,
            binding: None,
            detail: name.clone(),
            name,
            layer,
            handles: Vec::new(),
            deformation: Some(shape),
        },
        &[frame.origin.clone(), end],
    );
}

/// 🎞️ The tracks of the finished entity list.
pub fn finish(scene: &mut StructuralScene) {
    scene.tracks.extend(projection_kit::tracks::mode_track(&scene.entities, &MODES));
    let cases = case_track(scene);
    scene.tracks.extend(cases);
}
