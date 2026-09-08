
use super::geometry_import::{ObjIntoFem3d, StlIntoFem3d};
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::csv::v_rfc4180::any::csv_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::json::v_rfc8259::any::json_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::md::v_commonmark::any::md_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::obj::v3_0::any::obj_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::stl::v_ascii::any::stl_text;
use crate::standards::v1::subsets::any::io::export::serializers::artifacts::txt::v_utf_8::any::dsl_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::csv::v_rfc4180::any::from_csv_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::json::v_rfc8259::any::from_json_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::md::v_commonmark::any::from_md_text;
use crate::standards::v1::subsets::any::io::import::deserializers::artifacts::txt::v_utf_8::any::from_dsl_text;
use semio_framework::io::io_mechanism::Deserializer;
use semio_framework::io_schema::IoPayload;

/// 📄️ This subset's handcrafted `.semio` DSL example asset — the same bytes the shipped example
/// serves to the shell.
const EXAMPLE: &str = crate::examples::demo::PRIMARY_TEXT;

#[semio_framework_async_macros::async_test]
async fn txt_round_trips_the_example() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    let printed = dsl_text(&snapshot);
    assert_eq!(from_dsl_text(&printed).unwrap(), snapshot, "txt is not a fixed point");
}

#[semio_framework_async_macros::async_test]
async fn json_round_trips_the_example() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    assert_eq!(from_json_text(&json_text(&snapshot)).expect("json must read back"), snapshot, "json is not a lossless round trip");
}

#[semio_framework_async_macros::async_test]
async fn csv_round_trips_the_example_as_a_real_rfc4180_envelope() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    let text = csv_text(&snapshot);
    assert!(text.starts_with("payload\n"), "csv export must open with the envelope header row");
    assert_eq!(from_csv_text(&text).expect("csv must read back"), snapshot, "csv is not a lossless round trip");
}

#[semio_framework_async_macros::async_test]
async fn md_round_trips_the_example_as_a_real_fenced_block() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    let text = md_text(&snapshot);
    assert!(text.starts_with("```fem3d\n"), "md export must open the tagged fence");
    assert_eq!(from_md_text(&text).expect("md must read back"), snapshot, "md is not a lossless round trip");
}

#[semio_framework_async_macros::async_test]
async fn geometry_exports_are_real_and_their_imports_refuse_with_a_reason() {
    let snapshot = from_dsl_text(EXAMPLE).expect("the shipped example must parse");
    assert!(stl_text(&snapshot).expect("stl export must succeed").starts_with("solid"), "stl export must be real ascii stl");
    obj_text(&snapshot).expect("obj export must succeed");
    let empty = IoPayload::Text(String::new());
    for message in [StlIntoFem3d::deserialize(&empty).await.expect_err("stl import must refuse").message, ObjIntoFem3d::deserialize(&empty).await.expect_err("obj import must refuse").message] {
        assert!(message.contains("not supported for"), "every refusing hop must name the reason, got: {message}");
    }
}

#[semio_framework_async_macros::async_test]
async fn io_declaration_registers_both_directions_of_all_six_formats() {
    let declaration = super::io();
    assert_eq!(declaration.entries.len(), 12, "six formats x two directions");
    let own = "s.fem.fem3d";
    let mut foreign: Vec<&str> = Vec::new();
    for entry in declaration.entries {
        assert!(entry.from.artifact_kind == own || entry.into.artifact_kind == own, "every entry must touch this subset's own dialect");
        foreign.push(if entry.from.artifact_kind == own { entry.into.artifact_kind } else { entry.from.artifact_kind });
    }
    foreign.sort_unstable();
    foreign.dedup();
    assert_eq!(foreign, vec!["s.stdio.csv", "s.stdio.json", "s.stdio.md", "s.stdio.obj", "s.stdio.stl", "s.stdio.txt"]);
}
