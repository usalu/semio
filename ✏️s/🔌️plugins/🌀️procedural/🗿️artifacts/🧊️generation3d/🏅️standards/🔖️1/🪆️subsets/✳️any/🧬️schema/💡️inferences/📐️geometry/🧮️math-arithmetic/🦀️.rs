//! 🧮️ The math.arithmetic widget computes: the four basic operations, powers, rounding, limits and trigonometry over plain numbers.
//!
//! Every compute is pure, cheap and finishes in one step. A non-finite result is refused (`math-overflow`), so no downstream widget ever
//! sees an infinity or a NaN; a domain error (a square root of a negative number, a tangent at a quarter turn) is refused at the input port that causes it.
//!
//! 🔗️ [Rounding](https://en.wikipedia.org/wiki/Rounding) · [IEEE 754](https://en.wikipedia.org/wiki/IEEE_754)

use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;

const SAFE_INTEGER: f64 = 9_007_199_254_740_991.0;
const POLE_TOLERANCE: f64 = 1e-12;

//#region 🔖️Faults
/// 🚫️ The result does not fit a finite number.
pub(crate) fn overflow() -> WidgetFault {
    WidgetFault::new("generation3d.geometry.math-overflow", "The result is too large to be represented.", "Das Ergebnis ist zu groß, um dargestellt zu werden.")
}

/// 🚫️ A value outside the domain of the operation, attributed to its input port.
pub(crate) fn domain(port: &str, en: &str, de: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.math-domain", en, de).at(port)
}

/// 🚫️ A value outside the range the operation accepts, attributed to its input port.
pub(crate) fn out_of_range(port: &str, en: &str, de: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.math-range", en, de).at(port)
}

/// 🚫️ A division by zero, attributed to the divisor port.
fn division_by_zero(port: &str) -> WidgetFault {
    WidgetFault::new("generation3d.geometry.division-by-zero", "The divisor must not be zero.", "Der Divisor darf nicht null sein.").at(port)
}

/// 🛡️ The number itself, or `math-overflow` when it is infinite or not a number.
pub(crate) fn finite(value: f64) -> Result<f64, WidgetFault> {
    if value.is_finite() {
        Ok(value)
    } else {
        Err(overflow())
    }
}
//#endregion 🔖️Faults

//#region 🔖️Shapes
/// 🧮️ Runs a two-number operation over the named input ports and answers its `result`.
fn binary(kind: &Kind, inputs: WidgetInputs, first: &str, second: &str, calculate: fn(f64, f64) -> Result<f64, WidgetFault>) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let result = calculate(inputs.number(first)?, inputs.number(second)?)?;
            Ok(outputs([("result", GeometryValue::Number(result))]))
        })(),
    )
}

/// 🧮️ Runs a one-number operation over the named input port and answers its `result`.
fn unary(kind: &Kind, inputs: WidgetInputs, port: &str, calculate: fn(f64) -> Result<f64, WidgetFault>) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let result = calculate(inputs.number(port)?)?;
            Ok(outputs([("result", GeometryValue::Number(result))]))
        })(),
    )
}
//#endregion 🔖️Shapes

//#region 🔖️Computes
fn add(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "a", "b", |a, b| finite(a + b))
}

fn subtract(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "a", "b", |a, b| finite(a - b))
}

fn multiply(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "a", "b", |a, b| finite(a * b))
}

fn divide(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "a", "b", |a, b| if b == 0.0 { Err(division_by_zero("b")) } else { finite(a / b) })
}

fn modulo(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "a", "b", |a, b| if b == 0.0 { Err(division_by_zero("b")) } else { finite(a % b) })
}

fn power(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "base", "exponent", |base, exponent| {
        let result = base.powf(exponent);
        if result.is_nan() || (base == 0.0 && exponent < 0.0) {
            return Err(domain("base", "The base has no real power for this exponent.", "Die Basis hat für diesen Exponenten keine reelle Potenz."));
        }
        finite(result)
    })
}

fn negate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    unary(kind, inputs, "value", |value| Ok(0.0 - value))
}

fn absolute(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    unary(kind, inputs, "value", |value| Ok(value.abs()))
}

fn square_root(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    unary(kind, inputs, "value", |value| {
        if value < 0.0 {
            return Err(domain("value", "The square root of a negative number is not real.", "Die Quadratwurzel einer negativen Zahl ist nicht reell."));
        }
        Ok(value.sqrt())
    })
}

/// 🎯️ Rounds half away from zero (`nearest`), toward minus infinity (`down`) or toward plus infinity (`up`) into a whole number.
fn round(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let value = inputs.number("value")?;
            let rounded = match inputs.text("mode")? {
                "down" => value.floor(),
                "up" => value.ceil(),
                _ => value.round(),
            };
            if rounded.abs() > SAFE_INTEGER {
                return Err(overflow().at("value"));
            }
            Ok(outputs([("result", GeometryValue::Integer(rounded as i64))]))
        })(),
    )
}

fn minimum(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "a", "b", |a, b| Ok(a.min(b)))
}

fn maximum(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    binary(kind, inputs, "a", "b", |a, b| Ok(a.max(b)))
}

fn clamp(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (value, lower, upper) = (inputs.number("value")?, inputs.number("lower")?, inputs.number("upper")?);
            if lower > upper {
                return Err(out_of_range("lower", "The lower limit must not exceed the upper limit.", "Die untere Grenze darf die obere Grenze nicht überschreiten."));
            }
            Ok(outputs([("result", GeometryValue::Number(value.clamp(lower, upper)))]))
        })(),
    )
}

fn interpolate(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (a, b, t) = (inputs.number("a")?, inputs.number("b")?, inputs.number("t")?);
            Ok(outputs([("result", GeometryValue::Number(finite((1.0 - t) * a + t * b)?))]))
        })(),
    )
}

fn sine(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    unary(kind, inputs, "angle", |angle| finite(angle.sin()))
}

fn cosine(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    unary(kind, inputs, "angle", |angle| finite(angle.cos()))
}

fn tangent(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    unary(kind, inputs, "angle", |angle| {
        if angle.cos().abs() < POLE_TOLERANCE {
            return Err(domain("angle", "The tangent is undefined at a quarter turn.", "Der Tangens ist bei einer Vierteldrehung nicht definiert."));
        }
        finite(angle.tan())
    })
}
//#endregion 🔖️Computes

/// 🗃️ Every math.arithmetic catalogue kind and the compute that starts it.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "math.add", start: add },
    ComputeEntry { id: "math.subtract", start: subtract },
    ComputeEntry { id: "math.multiply", start: multiply },
    ComputeEntry { id: "math.divide", start: divide },
    ComputeEntry { id: "math.modulo", start: modulo },
    ComputeEntry { id: "math.power", start: power },
    ComputeEntry { id: "math.negate", start: negate },
    ComputeEntry { id: "math.absolute", start: absolute },
    ComputeEntry { id: "math.squareRoot", start: square_root },
    ComputeEntry { id: "math.round", start: round },
    ComputeEntry { id: "math.minimum", start: minimum },
    ComputeEntry { id: "math.maximum", start: maximum },
    ComputeEntry { id: "math.clamp", start: clamp },
    ComputeEntry { id: "math.interpolate", start: interpolate },
    ComputeEntry { id: "math.sine", start: sine },
    ComputeEntry { id: "math.cosine", start: cosine },
    ComputeEntry { id: "math.tangent", start: tangent },
];

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
