//! 🔢️ `math.values` computes: constants that feed numbers, angles, vectors, points and planes into other widgets.

use super::super::prelude::*;

fn number(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, inputs.number("value").map(|value| outputs([("value", GeometryValue::Number(value))])))
}

fn integer(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, inputs.integer("value").map(|value| outputs([("value", GeometryValue::Integer(value))])))
}

fn boolean(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, inputs.boolean("value").map(|value| outputs([("value", GeometryValue::Boolean(value))])))
}

fn vector(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, inputs.vector("value").map(|value| outputs([("value", GeometryValue::Vector(value))])))
}

fn point(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, inputs.point("value").map(|value| outputs([("value", GeometryValue::Point(value))])))
}

fn plane(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, inputs.plane("value").map(|value| outputs([("value", GeometryValue::Plane(value))])))
}

/// 🗃️ The `math.values` registrations.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "math.number", start: number },
    ComputeEntry { id: "math.integer", start: integer },
    ComputeEntry { id: "math.angle", start: number },
    ComputeEntry { id: "math.length", start: number },
    ComputeEntry { id: "math.boolean", start: boolean },
    ComputeEntry { id: "math.vector", start: vector },
    ComputeEntry { id: "math.point", start: point },
    ComputeEntry { id: "math.plane", start: plane },
];

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
