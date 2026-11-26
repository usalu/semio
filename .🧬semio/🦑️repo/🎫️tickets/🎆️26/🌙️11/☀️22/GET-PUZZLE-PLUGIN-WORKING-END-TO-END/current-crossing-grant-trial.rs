#![allow(unused)]
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️prelude/🦀️.rs"));
mod context {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/../../🧪️tests/🧰️context/🦀️.rs"));
}
use context::*;
use semio_plugin::artifact_app_laws;

#[semio_framework_async_macros::async_test]
async fn small_translation_receipts() {
    let width: usize = std::env::var("PUZZLE_PROBE_WIDTH").unwrap().parse().unwrap();
    let mut app=app().await;
    let mut policy=artifact_app_laws::fixture_mounted_policy();
    for grant in [&mut policy.preparation, &mut policy.maintenance, &mut policy.close] {grant.maximum_items=width;}
    app.mounted_policy=policy;
    let victim=first_object_id(&app);
    select_id(&mut app,semio_s_artifact_puzzle_3d_editor_snapshot::PUZZLE3D_GRANULARITY_OBJECT,&victim).await.unwrap();
    dispatch_unsettled(&mut app,"translateSelection",Some(&json!({"ids":[victim],"dx":3.5,"dy":0.0,"dz":0.0})),None).await.unwrap();
    let instance=app.instance_id;

    let original=app.parts().1.retained_grant();
    for turn in 1..256 {
        if !app.has_pending_typed_operations(){break}
        artifact_app_laws::fixture_renew_turn(app.parts().1,artifact_app_laws::OriginalFixtureCaller::turn_fuel(),original).unwrap();
        app.measure_maintenance_step(SETTLE_HOST_TURN_ITEMS,policy.maintenance.maximum_copy_bytes).unwrap();
        let (raw,cx)=app.parts();
        artifact_app_laws::fixture_renew_turn(cx,artifact_app_laws::OriginalFixtureCaller::turn_fuel(),policy.maintenance).unwrap();
        let before=semio_plugin::app::typed_operation_unit_census();
        raw.advance_typed_operation_publication(&mut artifact_app_laws::fixture_identity(),cx).await.unwrap();
        let census=semio_plugin::app::typed_operation_unit_census()-before;
        let mut receipts=Vec::new();
        artifact_app_laws::retain_original_ownership_receipts(raw,policy,&mut receipts).unwrap();
        use semio_plugin::artifact_app_laws::TypedOperationFixtureOwnershipReceipt as R;
        if receipts.is_empty() {
            eprintln!("[DEBUG] turn={turn} {census} no ownership receipt; {}",raw.typed_operation_diagnostic());
        }
        if !receipts.is_empty() { eprintln!("[DEBUG] turn={turn} stage {}",raw.typed_operation_diagnostic()); }
        for receipt in receipts {
         let (label,paid)=match receipt {
          R::Preparation{progress,..}=>("prepare",progress),R::Retirement{progress,..}=>("retire",progress),
          R::Constructor{progress,..}=>("constructor",progress),R::History{progress,..}=>("history",progress),
          R::ToolRun{progress,..}=>("tool",progress),R::LocalInteraction{progress,..}=>("interaction",progress),R::Maintenance{progress,..}=>("maintenance",progress),
         };
         eprintln!("[DEBUG] turn={turn} {census} {label} paid={paid:?} remaining={:?}",cx.retained_grant());
        }
        artifact_app_laws::fixture_renew_turn(cx,artifact_app_laws::OriginalFixtureCaller::turn_fuel(),original).unwrap();
        while let Some(mut page)=artifact_app_laws::take_fixture_result_page(raw,instance,policy.preparation).unwrap(){
            let token=artifact_app_laws::close_fixture_result_page(&mut page,policy.close);
            assert!(raw.acknowledge_typed_operation_result(token).unwrap());
        }
        let _=app.take_typed_operation_effect();
        let _=app.take_typed_operation_event();
        let _=app.take_typed_operation_completion().await.unwrap();
        let _=app.take_typed_operation_ui_progress();
    }
    assert!(!app.has_pending_typed_operations());
}
