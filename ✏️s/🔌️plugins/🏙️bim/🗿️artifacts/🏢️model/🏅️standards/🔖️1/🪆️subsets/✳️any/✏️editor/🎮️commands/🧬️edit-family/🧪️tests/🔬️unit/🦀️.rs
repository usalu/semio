use crate::editor::bim::kit::fault;
use crate::standards::v1::subsets::any::schema::{mutations::ModelMutation, snapshot::ModelSnapshot};
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
#[allow(unused_imports)]
use crate::editor::bim::entities::id_taken;
#[allow(unused_imports)]
use crate::editor::bim::kit::IdMint;
#[allow(unused_imports)]
use crate::editor::bim::modes::edit::windows::family::edit::{apply, Edit};
#[allow(unused_imports)]
use crate::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use crate::editor::bim::commands::edit_family::*;
use crate::editor::bim::fixture::support::{applied, ctx, run};
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_test_fixtures::fixture_text;

static TABLE: std::sync::LazyLock<&'static str> = std::sync::LazyLock::new(|| fixture_text!("../../../../../../../🧪️tests/🧫️fixtures/💡️inferences/🧬️families/🪑️table/📸️snapshot/🔣️.json"));

fn model() -> ModelSnapshot {
    from_json_str((*TABLE), JsonMemberPolicy::Reject).expect("the table decodes")
}

fn edit(part: &str, op: &str, key: &str, value: &str) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = model();
    let mut ctx = ctx(&[]);
    run(&snapshot, |doc, cfg| handle(&EditFamily { id: "fam-table".into(), part: part.into(), op: op.into(), key: key.into(), value: value.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn a_formula_edit_is_one_sparse_mutation_that_stands() {
    let emit = edit("parameter", "formula", "width", "1.8m").expect("edits");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::SetFamilyParameter(_)]));
    assert_eq!(applied(&model(), &emit).family_parameters["fam-table.width"].value, "1.8 m");
}

#[semio_framework_async_macros::async_test]
async fn a_new_solid_takes_a_fresh_id_and_a_removal_leaves() {
    let added = applied(&model(), &edit("solid", "add", "Sweep", "").expect("adds"));
    assert_eq!(added.family_solids.len(), model().family_solids.len() + 1);
    let removed = applied(&model(), &edit("solid", "remove", "s-top", "").expect("removes"));
    assert!(!removed.family_solids.contains_key("s-top"));
}

#[semio_framework_async_macros::async_test]
async fn text_that_does_not_parse_and_unknown_parts_are_refused_with_their_codes() {
    assert_eq!(edit("parameter", "formula", "width", "1.8 m +").err().map(|fault| fault.code.0), Some("bim.family.formula-invalid".to_string()));
    assert_eq!(edit("kitchen", "add", "", "").err().map(|fault| fault.code.0), Some("bim.family.part-unknown".to_string()));
    assert_eq!(edit("solid", "slot", "s-top#nothing", "1 m").err().map(|fault| fault.code.0), Some("bim.family.slot-unknown".to_string()));
}
