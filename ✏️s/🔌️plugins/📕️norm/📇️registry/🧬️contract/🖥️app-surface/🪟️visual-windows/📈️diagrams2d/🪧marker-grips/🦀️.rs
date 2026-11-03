//! 🪧 Marker grips: a diagram marker bound to a field is the same kind of grip as a 3D handle, so the keyboard, the typed value and the table twin reach it through the one handle drag statechart. See [`super`].
//!
//! 🔗️ Table-twin commit: `marker_row_commit` of the entity-editing typed entry; node: [`handle_node`](crate::accessibility::screen_reader::handle_node).

use semio_s_plugin_norm_structural_scene::{CheckStatus, Diagram2d, EntityKind, EntityRef, GripStyle, Handle, HandleKind, Interval, Marker, SnapRule, Tone, Vec3};

/// 🏷️ The id prefix of the grip of a marker, `marker:<diagram>:<marker>`.
pub const MARKER_GRIP_PREFIX: &str = "marker:";

/// 🆔️ The handle id of a marker of a diagram.
pub fn marker_grip_id(diagram: &Diagram2d, marker: &Marker) -> String {
    format!("{MARKER_GRIP_PREFIX}{}:{}", diagram.id, marker.id)
}

/// 📐️ The coordinate of a marker that its bound field drives: the one on the first axis shown in the unit of the binding, else the first.
pub fn bound_coordinate(diagram: &Diagram2d, marker: &Marker) -> Option<f64> {
    let unit = &marker.binding.as_ref()?.unit;
    let axis = diagram.axes.iter().position(|axis| axis.unit == *unit).unwrap_or(0);
    Some(if axis == 0 { marker.at.x } else { marker.at.y })
}

/// 🚦️ The status a marker speaks: its tone read as the check status it was drawn with.
pub fn marker_status(marker: &Marker) -> CheckStatus {
    match marker.tone {
        Tone::Error => CheckStatus::Fail,
        Tone::Warning => CheckStatus::Warning,
        _ => CheckStatus::Pass,
    }
}

/// 🚦️ The live utilisation of the check a marker is bound to, `(ratio, status)`.
pub fn marker_utilisation(marker: &Marker) -> Option<(f64, CheckStatus)> {
    marker.utilisation.filter(|ratio| ratio.is_finite()).map(|ratio| (ratio, marker_status(marker)))
}

/// 🕹️ The grip of a marker with a binding: a dimension-line handle at the marker writing the bound field, named by the label of the marker, limited to the domain of the axis shown in the unit of the field; `None` for a marker without a binding. The parameter `s` of the grip is the inverse of the field map, so the grip rests at [`marker_rest`].
pub fn marker_grip(diagram: &Diagram2d, marker: &Marker) -> Option<Handle> {
    let binding = marker.binding.clone()?;
    let limits = diagram.axes.iter().find(|axis| axis.unit == binding.unit).and_then(|axis| axis.domain.as_ref()).map(|domain| {
        let (a, b) = (binding.map.invert(domain.min), binding.map.invert(domain.max));
        Interval::new(a.min(b), a.max(b))
    });
    Some(Handle {
        id: marker_grip_id(diagram, marker),
        entity: diagram.focus.clone().unwrap_or_else(|| EntityRef::new(EntityKind::Group, diagram.id.clone(), String::new())),
        kind: HandleKind::DimensionLine,
        anchor: Vec3::new(marker.at.x, marker.at.y, 0.0),
        grip: GripStyle::Diamond,
        writes: vec![binding],
        snap: SnapRule::NONE,
        limits,
        accessible_name: marker.label.clone(),
    })
}

/// 📍️ The drag parameter a marker grip rests at: the inverse of the field map at the bound coordinate.
pub fn marker_rest(diagram: &Diagram2d, marker: &Marker) -> Option<f64> {
    Some(marker.binding.as_ref()?.map.invert(bound_coordinate(diagram, marker)?))
}
