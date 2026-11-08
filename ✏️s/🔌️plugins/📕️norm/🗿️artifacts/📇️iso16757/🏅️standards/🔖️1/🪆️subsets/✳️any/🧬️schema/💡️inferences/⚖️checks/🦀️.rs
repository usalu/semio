//! ⚖️ ISO 16757 compliance check catalogue (Parts 1, 2, 4, 5).

#![allow(async_fn_in_trait)]

extern crate semio_os_kernel as dsl;
extern crate semio_os_kernel as protocol;
extern crate semio_os_kernel as store;
extern crate semio_framework_schema as framework_schema;
extern crate semio_framework_value_derive as value_derive;

#[path = "📐️part2/🦀️.rs"]
mod part2;
#[path = "📚️part4/🦀️.rs"]
mod part4;
#[path = "🔄️part5/🦀️.rs"]
mod part5;

use semio_s_plugin_norm_compliance::CheckReport;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

pub use semio_s_artifact_norm_iso16757_checks_common::Section;

/// 🧮️ Every evaluation section of Parts 1, 2, 4 and 5 in report order: the one table the full evaluation and the incremental units iterate.
pub fn sections() -> Vec<Section> {
    [semio_s_artifact_norm_iso16757_checks_part1::SECTIONS, crate::part2::SECTIONS, crate::part4::SECTIONS, crate::part5::SECTIONS].concat()
}

pub fn evaluate_all(document: &Iso16757Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    sections().iter().for_each(|section| (section.run)(document, &mut report));
    report
}
