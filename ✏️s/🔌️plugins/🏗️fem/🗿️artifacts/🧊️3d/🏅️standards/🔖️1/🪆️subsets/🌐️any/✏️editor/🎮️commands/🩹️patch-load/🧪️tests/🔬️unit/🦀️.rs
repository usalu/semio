use super::*;
use crate::editor::fem3d::unit_tests::context::{dispatch, fem3d_app};
use crate::editor::fem3d::Fem3dCommand;

fn emit(snapshot: &Fem3dSnapshot, payload: PatchLoad) -> Result<Emit<Fem3dMutation, NoConfigMutation>, Fault> {
    let history = semio_framework_plugin::HistoryView::empty();
    let doc = ArtifactView::new(snapshot, &history);
    let config = NoConfig::default();
    handle(&payload, &doc, &ConfigView { snapshot: &config, window: None })
}

#[semio_framework_async_macros::async_test]
async fn patch_load_edits_each_variant_through_its_owning_case_3d() {
    let mut app = fem3d_app();
    dispatch(&mut app, Fem3dCommand::PatchLoad(PatchLoad { id: "l2".into(), field: "value".into(), value: "-7500".into() })).await;
    dispatch(&mut app, Fem3dCommand::PatchLoad(PatchLoad { id: "l3".into(), field: "pressure".into(), value: "2000".into() })).await;
    let snapshot = app.snapshot().expect("snapshot");
    let live = snapshot.load_cases.iter().find(|case| case.id == "live").expect("live");
    assert!(matches!(&live.loads[0], FemLoad::Nodal { value, .. } if *value == -7500.0));
    assert!(matches!(&live.loads[1], FemLoad::Area { pressure, .. } if *pressure == 2000.0));
    let emitted = emit(&snapshot, PatchLoad { id: "l2".into(), field: "dof".into(), value: "Tx".into() }).expect("handle");
    let [Fem3dMutation::ReplaceLoad(replace)] = emitted.artifact_mutations.as_slice() else { panic!("one replace-load") };
    assert_eq!((replace.case_id.as_str(), replace.load_id.as_str()), ("live", "l2"));
    assert!(matches!(*replace.new_load, FemLoad::Nodal { dof: FemDof::Tx, .. }));
    assert!(emit(&snapshot, PatchLoad { id: "l2".into(), field: "wx".into(), value: "1".into() }).is_err(), "a nodal load has no wx");
    assert!(emit(&snapshot, PatchLoad { id: "l2".into(), field: "dof".into(), value: "Tw".into() }).is_err());
    assert!(emit(&snapshot, PatchLoad { id: "ghost".into(), field: "value".into(), value: "1".into() }).is_err());
}
