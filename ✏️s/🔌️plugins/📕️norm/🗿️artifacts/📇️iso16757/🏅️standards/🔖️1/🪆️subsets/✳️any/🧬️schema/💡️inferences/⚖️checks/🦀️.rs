//! ⚖️ ISO 16757 compliance check catalogue (Parts 1, 2, 4, 5).

#![allow(async_fn_in_trait)]

extern crate semio_framework_os_kernel as dsl;
extern crate semio_framework_os_kernel as protocol;
extern crate semio_framework_os_kernel as store;
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

pub fn evaluate_all(document: &Iso16757Snapshot) -> CheckReport {
    let mut report = CheckReport::default();
    semio_s_artifact_norm_iso16757_checks_part1::check_part_1(document, &mut report);
    crate::part2::check_part_2(document, &mut report);
    crate::part4::check_part_4(document, &mut report);
    crate::part5::check_part_5(document, &mut report);
    report
}
