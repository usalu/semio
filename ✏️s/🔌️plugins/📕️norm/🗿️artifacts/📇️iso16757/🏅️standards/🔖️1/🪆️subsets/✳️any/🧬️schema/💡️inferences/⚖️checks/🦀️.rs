//! ⚖️ ISO 16757 compliance check catalogue (Parts 1, 2, 4, 5).

#[path = "🧰common.rs"]
mod common;
#[path = "📈️part1.rs"]
mod part1;
#[path = "📐️part2.rs"]
mod part2;
#[path = "📚️part4.rs"]
mod part4;
#[path = "🔄part5.rs"]
mod part5;

use crate::document::CheckReport;
use crate::Iso16757Snapshot;

pub fn evaluate_all(document: &Iso16757Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    part1::check_part_1(document, &mut report);
    part2::check_part_2(document, &mut report);
    part4::check_part_4(document, &mut report);
    part5::check_part_5(document, &mut report);
    report
}
