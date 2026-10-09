//! 🧮️ The table of a schedule: source rows become cells, the filters keep rows, the group keys and sort keys order them (stable, so equal rows keep element id order), grouping closes every group with a
//! subtotal row, `itemize = false` collapses groups (or, ungrouped, equal rows) into one row, and the columns flagged `total` are summed per group and in a grand total row.
//!
//! Ordering of cells: empty before numbers before texts; numbers by value; texts case-folded with every maximal run of ASCII digits compared as an integer (`D2` before `D10`), ties by the raw text.
//! Filters: a number cell against a numeric value compares as numbers (equality within 1e-9), everything else compares the case-folded display text; `Contains` is a substring test, `Empty` an empty cell.

use super::super::super::quantities::ElementQuantity;
use super::rows::{sources, AuthoredView, Reader};
use super::{RowKind, ScheduleCell, ScheduleRow, ScheduleTable};
use crate::{Schedule, ScheduleKey, ScheduleOp};
use std::cmp::Ordering;
use std::collections::BTreeMap;

//#region 🔖️Order
fn chunks(text: &str) -> Vec<(bool, String)> {
    let mut found: Vec<(bool, String)> = Vec::new();
    for ch in text.chars() {
        let digit = ch.is_ascii_digit();
        match found.last_mut() {
            Some((last, run)) if *last == digit => run.push(ch),
            _ => found.push((digit, ch.to_string())),
        }
    }
    found
}

/// 🔤️ Natural order of two texts: case-folded, digit runs as integers, ties by the raw text.
pub fn natural(a: &str, b: &str) -> Ordering {
    let (folded_a, folded_b) = (a.to_lowercase(), b.to_lowercase());
    let (left, right) = (chunks(&folded_a), chunks(&folded_b));
    for ((a_digit, a_run), (b_digit, b_run)) in left.iter().zip(right.iter()) {
        let order = match (a_digit, b_digit) {
            (true, true) => {
                let (a_int, b_int) = (a_run.trim_start_matches('0'), b_run.trim_start_matches('0'));
                a_int.len().cmp(&b_int.len()).then_with(|| a_int.cmp(b_int))
            }
            (true, false) => Ordering::Less,
            (false, true) => Ordering::Greater,
            (false, false) => a_run.cmp(b_run),
        };
        if order != Ordering::Equal {
            return order;
        }
    }
    left.len().cmp(&right.len()).then_with(|| a.cmp(b))
}

/// ↕️ The order of two cells: empty, then numbers by value, then texts in natural order.
pub fn order(a: &ScheduleCell, b: &ScheduleCell) -> Ordering {
    match (a, b) {
        (ScheduleCell::Empty, ScheduleCell::Empty) => Ordering::Equal,
        (ScheduleCell::Empty, _) => Ordering::Less,
        (_, ScheduleCell::Empty) => Ordering::Greater,
        (ScheduleCell::Number { value: x }, ScheduleCell::Number { value: y }) => x.total_cmp(y),
        (ScheduleCell::Number { .. }, _) => Ordering::Less,
        (_, ScheduleCell::Number { .. }) => Ordering::Greater,
        (ScheduleCell::Text { value: x }, ScheduleCell::Text { value: y }) => natural(x, y),
    }
}
//#endregion 🔖️Order

//#region 🔖️Filter
const EQUAL: f64 = 1e-9;

fn compare_numbers(x: f64, value: f64, op: ScheduleOp) -> bool {
    let equal = (x - value).abs() < EQUAL;
    match op {
        ScheduleOp::Equals => equal,
        ScheduleOp::NotEquals => !equal,
        ScheduleOp::Greater => x > value && !equal,
        ScheduleOp::GreaterOrEqual => x > value || equal,
        ScheduleOp::Less => x < value && !equal,
        ScheduleOp::LessOrEqual => x < value || equal,
        ScheduleOp::Contains | ScheduleOp::Empty | ScheduleOp::NotEmpty => false,
    }
}

/// 🔎️ Whether `cell` satisfies `op` against `value`.
pub fn passes(cell: &ScheduleCell, op: ScheduleOp, value: &str) -> bool {
    let empty = matches!(cell, ScheduleCell::Empty);
    match op {
        ScheduleOp::Empty => return empty,
        ScheduleOp::NotEmpty => return !empty,
        _ => {}
    }
    let number = value.trim().parse::<f64>().ok().filter(|number| number.is_finite());
    if let (ScheduleCell::Number { value: x }, Some(number), false) = (cell, number, op == ScheduleOp::Contains) {
        return compare_numbers(*x, number, op);
    }
    let (shown, wanted) = (cell.display().to_lowercase(), value.trim().to_lowercase());
    match op {
        ScheduleOp::Equals => shown == wanted,
        ScheduleOp::NotEquals => shown != wanted,
        ScheduleOp::Contains => shown.contains(&wanted),
        ScheduleOp::Greater => natural(&shown, &wanted) == Ordering::Greater,
        ScheduleOp::GreaterOrEqual => natural(&shown, &wanted) != Ordering::Less,
        ScheduleOp::Less => natural(&shown, &wanted) == Ordering::Less,
        ScheduleOp::LessOrEqual => natural(&shown, &wanted) != Ordering::Greater,
        ScheduleOp::Empty | ScheduleOp::NotEmpty => false,
    }
}
//#endregion 🔖️Filter

//#region 🔖️Rows
struct Line {
    id: String,
    cells: Vec<ScheduleCell>,
}

fn sum(lines: &[&Line], index: usize) -> ScheduleCell {
    let numbers: Vec<f64> = lines.iter().filter_map(|line| if let ScheduleCell::Number { value } = &line.cells[index] { Some(*value) } else { None }).collect();
    if numbers.is_empty() {
        ScheduleCell::Empty
    } else {
        ScheduleCell::number(numbers.iter().sum())
    }
}

fn common(lines: &[&Line], index: usize) -> ScheduleCell {
    let first = &lines[0].cells[index];
    if lines.iter().all(|line| line.cells[index] == *first) {
        first.clone()
    } else {
        ScheduleCell::Empty
    }
}

fn partition<'a>(lines: &[&'a Line], keys: &[usize]) -> Vec<Vec<&'a Line>> {
    let mut parts: Vec<Vec<&Line>> = Vec::new();
    for line in lines {
        match parts.iter_mut().find(|part| keys.iter().all(|key| part[0].cells[*key] == line.cells[*key])) {
            Some(part) => part.push(line),
            None => parts.push(vec![line]),
        }
    }
    parts
}

fn elements(lines: &[&Line]) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for line in lines {
        if !ids.contains(&line.id) {
            ids.push(line.id.clone());
        }
    }
    ids
}

fn summary(part: &[&Line], shown: &[usize], columns: &[(usize, bool)], level: usize, fill: bool) -> ScheduleRow {
    let cells = columns
        .iter()
        .map(|(index, total)| {
            if shown.contains(index) {
                part[0].cells[*index].clone()
            } else if *total {
                sum(part, *index)
            } else if fill {
                common(part, *index)
            } else {
                ScheduleCell::Empty
            }
        })
        .collect();
    ScheduleRow { kind: RowKind::Group, level: level as u32, elements: elements(part), cells }
}

fn itemized(lines: &[&Line], depth: usize, groups: &[usize], columns: &[(usize, bool)], out: &mut Vec<ScheduleRow>) {
    if depth == groups.len() {
        out.extend(lines.iter().map(|line| ScheduleRow { kind: RowKind::Item, level: depth as u32, elements: vec![line.id.clone()], cells: columns.iter().map(|(index, _)| line.cells[*index].clone()).collect() }));
        return;
    }
    for part in partition(lines, &groups[depth..=depth]) {
        itemized(&part, depth + 1, groups, columns, out);
        out.push(summary(&part, &groups[..=depth], columns, depth, false));
    }
}
//#endregion 🔖️Rows

//#region 🔖️Table
/// 📋️ The table of `schedule` from the quantities of its candidates and the authored view.
pub fn table_of(schedule: &Schedule, view: &AuthoredView, quantities: &BTreeMap<&str, &ElementQuantity>) -> ScheduleTable {
    let reader = Reader::new(schedule, view);
    let mut keys: Vec<ScheduleKey> = schedule.columns.iter().map(|column| column.key.clone()).collect();
    for key in schedule.sort.iter().map(|sort| &sort.key).chain(schedule.filter.iter().map(|filter| &filter.key)).chain(schedule.group.iter().map(|group| &group.key)) {
        if !keys.contains(key) {
            keys.push(key.clone());
        }
    }
    let at = |key: &ScheduleKey| keys.iter().position(|known| known == key).expect("every key of the definition was collected");
    let mut lines: Vec<Line> = sources(schedule, quantities).iter().map(|source| Line { id: source.id.to_string(), cells: keys.iter().map(|key| reader.cell(source, key)).collect() }).collect();
    let filters: Vec<(usize, ScheduleOp, &str)> = schedule.filter.iter().map(|filter| (at(&filter.key), filter.op, filter.value.as_str())).collect();
    lines.retain(|line| filters.iter().all(|(index, op, value)| passes(&line.cells[*index], *op, value)));
    let groups: Vec<usize> = schedule.group.iter().map(|group| at(&group.key)).collect();
    let sorts: Vec<(usize, bool)> = groups.iter().map(|index| (*index, false)).chain(schedule.sort.iter().map(|sort| (at(&sort.key), sort.descending))).collect();
    lines.sort_by(|a, b| {
        sorts
            .iter()
            .map(|(index, descending)| {
                let found = order(&a.cells[*index], &b.cells[*index]);
                if *descending {
                    found.reverse()
                } else {
                    found
                }
            })
            .find(|found| *found != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    });
    let columns: Vec<(usize, bool)> = schedule.columns.iter().enumerate().map(|(position, column)| (position, column.total)).collect();
    let all: Vec<&Line> = lines.iter().collect();
    let mut rows: Vec<ScheduleRow> = Vec::new();
    if schedule.itemize {
        itemized(&all, 0, &groups, &columns, &mut rows);
    } else if !all.is_empty() {
        let partition_keys: Vec<usize> = if groups.is_empty() { columns.iter().filter(|(_, total)| !*total).map(|(index, _)| *index).collect() } else { groups.clone() };
        rows.extend(partition(&all, &partition_keys).iter().map(|part| summary(part, &partition_keys, &columns, 0, true)));
    }
    if columns.iter().any(|(_, total)| *total) && !all.is_empty() {
        let cells = columns.iter().map(|(index, total)| if *total { sum(&all, *index) } else { ScheduleCell::Empty }).collect();
        rows.push(ScheduleRow { kind: RowKind::Total, level: 0, elements: Vec::new(), cells });
    }
    ScheduleTable { keys: schedule.columns.iter().map(|column| column.key.clone()).collect(), rows, items: lines.len() as u32 }
}
//#endregion 🔖️Table
