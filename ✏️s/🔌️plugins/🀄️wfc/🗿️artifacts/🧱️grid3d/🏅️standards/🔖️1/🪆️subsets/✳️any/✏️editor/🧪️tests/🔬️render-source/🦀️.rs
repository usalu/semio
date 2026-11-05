//! 🪪️ Actual mounted render authority controls the instance-local completed grid3d preview.
use super::*;
#[semio_framework_async_macros::async_test]
async fn mounted_render_uses_canonical_render_revision_and_local_transient() {
    let snapshot=crate::examples::blocks::snapshot();
    let history=semio_framework_plugin::HistoryView::empty();
    let context=semio_framework_plugin::AppRenderOperationContext{app_instance_id:1,base_revision:semio_framework_job::RevisionId(1),generation:semio_framework_job::Generation(1),canonical_base_revision:[7;32],mounted_policy:semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT};
    let doc=ArtifactView::with_render_context(&snapshot,&history,semio_framework_plugin::ChildContentView::EMPTY,context,"grid3d",None).await;
    assert!(doc.operation_optional().is_none());
    let no_config=NoConfig::default();
    let config=ConfigView{snapshot:&no_config,window:None};
    let view=semio_framework_plugin::ViewModel::new(semio_framework_ui_locale::Locale::En,semio_framework_ui_locale::Terminology::Native);
    let payload=fill_tool::Grid3dFillPayload{source_revision:vec![7;32],assignments:vec![fill_tool::Grid3dFillCell{x:0,y:0,z:0,tile_id:Some(snapshot.tiles[0].id.clone())}],done:true,..Default::default()};
    let transient=Grid3dTransient{payload_json:String::from_utf8(payload.encode()).unwrap()};
    let empty=Grid3dEditor::render_bodies(preview::BODY_KEY,&doc,&config,&view,&Grid3dTransient::default()).unwrap();
    let fresh=Grid3dEditor::render_bodies(preview::BODY_KEY,&doc,&config,&view,&transient).unwrap();
    let empty=serde_json::to_value(&empty.root).unwrap();
    assert_ne!(serde_json::to_value(&fresh.root).unwrap(),empty);
    let changed=ArtifactView::with_render_context(&snapshot,&history,semio_framework_plugin::ChildContentView::EMPTY,semio_framework_plugin::AppRenderOperationContext{canonical_base_revision:[8;32],..context},"grid3d",None).await;
    let stale=Grid3dEditor::render_bodies(preview::BODY_KEY,&changed,&config,&view,&transient).unwrap();
    assert_eq!(serde_json::to_value(&stale.root).unwrap(),empty);
}

/// 🛡️ Completed background commands cannot publish unfinished or stale results.
#[test]
fn completion_refusals_keep_their_localized_domain_codes() {
    let snapshot=crate::examples::blocks::snapshot();
    let history=semio_framework_plugin::HistoryView::empty();
    let doc=ArtifactView::with_operation(&snapshot,&history,semio_framework_plugin::AppOperationContext{app_instance_id:1,parent_document_id:"grid3d".into(),operation_id:1,generation:1,canonical_base_revision:[7;32],retained:semio_framework_plugin::artifact_app_laws::fixture_retained_turn().grant,authoring_seed:"fill".into()});
    let no_config=NoConfig::default();
    let config=ConfigView{snapshot:&no_config,window:None};
    for (payload,code) in [(fill_tool::Grid3dFillPayload{source_revision:vec![7;32],done:false,..Default::default()},"wfc.grid3d.fill.invalid"),(fill_tool::Grid3dFillPayload{source_revision:vec![8;32],done:true,..Default::default()},"wfc.grid3d.fill.stale")] {
        let command=Grid3dEditorCommand::CommitFill{payload_json:String::from_utf8(payload.encode()).unwrap()};
        let fault=grid3d_command_emit(&command,&doc,&config,None).unwrap_err();
        assert_eq!(fault.code,semio_framework_plugin::FaultCode::new(code));
        assert!(Grid3dEditor::fault_notices().iter().any(|(notice,_)|*notice==code));
    }
}
