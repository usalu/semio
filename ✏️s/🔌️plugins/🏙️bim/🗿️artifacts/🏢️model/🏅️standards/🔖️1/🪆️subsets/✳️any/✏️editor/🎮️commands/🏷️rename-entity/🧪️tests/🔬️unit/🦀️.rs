use crate::editor::bim::kit::fault;
use crate::standards::v1::subsets::any::schema::mutations::ModelMutation;
use semio_framework_plugin::{Emit, Fault, NoConfigMutation};
#[allow(unused_imports)]
use crate::editor::bim::entities::kind_holding;
#[allow(unused_imports)]
use crate::editor::bim::dispatch_context::BimDispatchCtx;
#[allow(unused_imports)]
use crate::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;
#[allow(unused_imports)]
use semio_framework_plugin::{ArtifactView, ConfigView, NoConfig};
#[allow(unused_imports)]
use value_derive::{FromValue, ToValue};
use crate::editor::bim::commands::rename_entity::*;
use crate::editor::bim::fixture::support::{applied, ctx, demo, run};

fn rename(id: &str, name: &str) -> Result<Emit<ModelMutation, NoConfigMutation>, Fault> {
    let snapshot = demo();
    let mut ctx = ctx(&[]);
    run(&snapshot, |doc, cfg| handle(&RenameEntity { id: id.into(), name: name.into() }, doc, cfg, &mut ctx))
}

#[semio_framework_async_macros::async_test]
async fn a_storey_is_renamed_through_its_rename_mutation() {
    let emit = rename("st-first", "Upper").expect("renames");
    assert!(matches!(emit.artifact_mutations.as_slice(), [ModelMutation::RenameElement(_)]));
    assert_eq!(applied(&demo(), &emit).storeys["st-first"].name, "Upper");
}

#[semio_framework_async_macros::async_test]
async fn a_missing_entity_is_refused() {
    assert_eq!(rename("nowhere", "X").err().map(|fault| fault.code.0), Some("bim.rename.target-missing".to_string()));
}

#[semio_framework_async_macros::async_test]
async fn every_kind_of_the_demo_model_can_be_renamed() {
    let snapshot = demo();
    for row in crate::editor::bim::entities::ENTITIES {
        for id in (row.ids)(&snapshot) {
            let emit = rename(&id, "Renamed").unwrap_or_else(|fault| panic!("{} {id} is not renamable: {}", row.kind, fault.code.0));
            assert!((row.name)(&applied(&snapshot, &emit), &id).is_some_and(|name| name == "Renamed"), "{} {id}", row.kind);
        }
    }
}
