//! 🔢️ Unit-aware typed entry: the text typed during a drag (`35 cm`, `1,5m`, `12`) becomes the drag parameter of the handle, in the unit the bound field is shown in. See [`super`].

use crate::diagrams2d::{marker_grip, marker_rest};
use crate::direct_manipulation::{parse_entry, DragEffect, DragEvent, DragState, HandleDragTool, Modifiers};
use crate::FieldWrite;
use semio_s_plugin_norm_structural_scene::{Diagram2d, Handle, Marker};

const UNITS: [(&str, char, f64); 19] = [
    ("m", 'L', 1.0), ("cm", 'L', 0.01), ("mm", 'L', 0.001), ("km", 'L', 1000.0),
    ("N", 'F', 1.0), ("kN", 'F', 1e3), ("MN", 'F', 1e6),
    ("Pa", 'S', 1.0), ("kPa", 'S', 1e3), ("MPa", 'S', 1e6), ("GPa", 'S', 1e9),
    ("Nm", 'M', 1.0), ("kNm", 'M', 1e3), ("MNm", 'M', 1e6),
    ("deg", 'A', 1.0), ("°", 'A', 1.0), ("rad", 'A', 57.295_779_513_082_32),
    ("%", 'R', 0.01), ("", 'R', 1.0),
];

fn unit(symbol: &str) -> Option<(char, f64)> {
    UNITS.iter().find(|(name, _, _)| *name == symbol).map(|(_, dimension, factor)| (*dimension, *factor))
}

/// 🔢️ A typed quantity converted into `target` (the unit symbol of the bound field); a bare number is taken as already in `target`, a unit of another dimension or an unknown unit is `None`.
pub fn parse_quantity_entry(text: &str, target: &str) -> Option<f64> {
    let text = text.trim();
    let split = text.find(|c: char| c.is_alphabetic() || c == '°' || c == '%').unwrap_or(text.len());
    let (number, symbol) = text.split_at(split);
    let value = parse_entry(number)?;
    let symbol = symbol.trim();
    if symbol.is_empty() {
        return Some(value);
    }
    let (from_dimension, from) = unit(symbol)?;
    let (_, to) = unit(target).filter(|(dimension, _)| *dimension == from_dimension)?;
    Some(value * from / to)
}

/// 🎯️ The drag parameter `s` the typed text means for `handle`: converted into the unit of its first bound field, mapped back through the field binding and clamped to the limits. `None` when the text does not parse or the handle binds no field.
pub fn entry_to_s(handle: &Handle, text: &str) -> Option<f64> {
    let binding = handle.writes.first()?;
    let value = parse_quantity_entry(text, &binding.unit)?;
    let s = binding.map.invert(value);
    Some(handle.limits.as_ref().map_or(s, |limits| limits.clamp(s)))
}

/// ✍️ The field writes of a value typed into a row of the marker table (`35 cm`, `1,5m`): the same unit-aware parse, limits, field quantisation and single commit as a pointer drag released at that value; `None` when the text does not parse in the unit of the field, the marker has no binding, or the value equals the current one.
pub fn marker_row_commit(diagram: &Diagram2d, marker: &Marker, text: &str) -> Option<Vec<FieldWrite>> {
    let grip = marker_grip(diagram, marker)?;
    let rest = marker_rest(diagram, marker)?;
    let target = entry_to_s(&grip, text)?;
    let handles = [grip];
    let tool = HandleDragTool { handles: &handles, seq: 0 };
    let armed = DragState::Armed { handle: handles[0].id.clone(), origin: rest };
    let (dragging, _) = tool.step(armed, DragEvent::Move { s: target, modifiers: Modifiers::CTRL });
    match tool.step(dragging, DragEvent::Release).1 {
        DragEffect::Commit(writes) => Some(writes),
        _ => None,
    }
}
