//! 📚️ The math.list widget computes: number sequences and reading from lists.
//!
//! Sequences are bounded by their catalogue item limit; a request beyond it, or a sequence that leaves the finite numbers, is refused
//! instead of truncated, and a list index outside the list never wraps.
//!
//! 🔗️ [Arithmetic progression](https://en.wikipedia.org/wiki/Arithmetic_progression) · [Linear interpolation](https://en.wikipedia.org/wiki/Linear_interpolation)

use super::math_arithmetic::{finite, out_of_range};
use crate::standards::v1::subsets::any::schema::inferences::geometry::prelude::*;

const MAXIMUM_COUNT: i64 = 100_000;

/// 🧮️ The item count of a sequence, refused outside `minimum..=MAXIMUM_COUNT`.
fn count_in_range(inputs: &WidgetInputs, minimum: i64) -> Result<usize, WidgetFault> {
    let count = inputs.integer("count")?;
    if !(minimum..=MAXIMUM_COUNT).contains(&count) {
        return Err(out_of_range("count", &format!("The count must lie between {minimum} and {MAXIMUM_COUNT}."), &format!("Die Anzahl muss zwischen {minimum} und {MAXIMUM_COUNT} liegen.")));
    }
    Ok(count as usize)
}

/// 🔢️ The numbers of a sequence as the `numbers` output, every one of them finite.
fn numbers_output(numbers: Vec<f64>) -> Result<Outputs, WidgetFault> {
    for number in &numbers {
        finite(*number)?;
    }
    Ok(outputs([("numbers", GeometryValue::List(numbers.into_iter().map(GeometryValue::Number).collect()))]))
}

/// 🌈️ Evenly spaced numbers from `start` to `end`, both included exactly.
fn range(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (start, end, count) = (inputs.number("start")?, inputs.number("end")?, count_in_range(&inputs, 2)?);
            let last = count - 1;
            numbers_output((0..count).map(|index| if index == last { end } else { (1.0 - index as f64 / last as f64) * start + (index as f64 / last as f64) * end }).collect())
        })(),
    )
}

/// 🪜️ Numbers `start + step * index`.
fn series(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (start, step, count) = (inputs.number("start")?, inputs.number("step")?, count_in_range(&inputs, 1)?);
            numbers_output((0..count).map(|index| start + step * index as f64).collect())
        })(),
    )
}

/// 🎯️ The item at a zero-based position.
fn list_item(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(
        kind,
        (|| {
            let (list, index) = (inputs.list("list")?, inputs.integer("index")?);
            let item = usize::try_from(index).ok().and_then(|position| list.get(position)).ok_or_else(|| {
                WidgetFault::new("generation3d.geometry.list-index", format!("The index {index} lies outside a list of {} items.", list.len()), format!("Der Index {index} liegt außerhalb einer Liste mit {} Einträgen.", list.len())).at("index")
            })?;
            Ok(outputs([("item", item.clone())]))
        })(),
    )
}

/// 🔢️ The number of items.
fn list_length(kind: &Kind, inputs: WidgetInputs) -> Box<dyn WidgetJob> {
    finish(kind, (|| Ok(outputs([("count", GeometryValue::Integer(inputs.list("list")?.len() as i64))])))())
}

/// 🗃️ Every math.list catalogue kind and the compute that starts it.
pub const COMPUTES: &[ComputeEntry] = &[
    ComputeEntry { id: "math.range", start: range },
    ComputeEntry { id: "math.series", start: series },
    ComputeEntry { id: "math.listItem", start: list_item },
    ComputeEntry { id: "math.listLength", start: list_length },
];

#[cfg(test)]
//#region 🧪️Tests
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
