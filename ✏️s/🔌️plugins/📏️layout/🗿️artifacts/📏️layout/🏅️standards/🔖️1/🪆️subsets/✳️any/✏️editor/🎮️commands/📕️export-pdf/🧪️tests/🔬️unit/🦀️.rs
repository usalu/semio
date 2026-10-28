//! 📥️ The browser export path of the layout editor, natively: `exportPdf` through the registered, instance-bound app publishes one segmented download that the host drains into a whole pdf.
//!
//! The law rides the editor fixture layer (`unit_tests::context`) and therefore the plugin typed-operation turn API; it lives at the `export-pdf` command node, mounted by the commands companion, and moved here from the export engine crate, whose companion laws drive the export jobs on an original caller and no longer depend on the editor fixtures.

include!("../../../../⚙️engine/📤️export/🧪️tests/🔬️unit/📕️pdf-structure/🦀️.rs");

/// 🌐️ The browser path, natively: `exportPdf` through the registered, instance-bound app, the host's
/// continuation turns (maintenance step → publication unit → result page ACK), the Download lane's page
/// captured exactly where `consumeTypedOperationEffects` captures it, and the chunk drain
/// `drainSegmentedMediaExport` performs — one bounded chunk per call until `None`.
async fn original_layout_export_download(command:semio_s_artifact_layout_layout_editor::LayoutCommand)->(String,String,Option<String>,Vec<u8>){
    use semio_s_artifact_layout_layout_editor_commands::modes::edit::windows::blueprint::config::LayoutBlueprintWindowConfigOwner;
    use semio_framework_plugin::app::TypedOperationResultLane;
    use semio_framework_plugin::{artifact_app_laws, ActionMeta, PluginApp, ViewModel, ViewWindowInstance, WindowConfigOwner};
    const INSTANCE: u32 = 1;
    let mut app = semio_s_artifact_layout_layout_editor::unit_tests::context::layout_app_with_registry().await;
    app.bind_instance_id(INSTANCE).await;
    let before=app.0.document_pack().await.expect("original document pack before export");
    let view = ViewModel { window_instances: vec![ViewWindowInstance { id: "layout-blueprint".into(), window_kind_id: LayoutBlueprintWindowConfigOwner::WINDOW_KIND_ID.into() }], ..ViewModel::new(semio_framework_ui_locale::Locale::En, semio_framework_ui_locale::Terminology::Native) };
    let meta = ActionMeta { view_state: Some(view.for_window_instance("layout-blueprint").expect("blueprint window instance")), ..artifact_app_laws::meta("local") };
    let result = app.0.dispatch_typed(command, &meta, &mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("exportPdf dispatch");
    assert!(result.mutations.is_empty(), "an export never mutates the document");
    let mut download: Option<(u64, String, String, Option<String>, usize)> = None;
    let mut turns = 0usize;
    let mut downloads=0usize;
    while app.has_pending_typed_operations() {
        turns += 1;
        assert!(turns < 65_536, "the export never quiesced");
        app.0.maintenance_step(app.1.cx(), semio_framework_plugin::MountedOwnerPolicyV1::INTERACTIVE_COMPONENT.maintenance).expect("maintenance step");
        app.0.advance_typed_operation_publication(&mut semio_framework_plugin::artifact_app_laws::fixture_identity(), app.1.cx()).await.expect("publication unit");
        if let Some(page) = app.take_typed_operation_result_page(INSTANCE) {
            assert_ne!(page.lane, TypedOperationResultLane::Fault, "export faulted: {}", String::from_utf8_lossy(page.bytes()));
            if page.lane == TypedOperationResultLane::Download {
                downloads+=1;
                assert_eq!(downloads,1,"one original export publishes one download");
                let text = String::from_utf8_lossy(page.bytes()).into_owned();
                let row = semio_framework_pack_json::parse(&text, semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap_or_else(|_| panic!("download page is JSON: {text}"));
                let row = row.as_array().unwrap_or_else(|| panic!("download page is a 4-row array: {text}"));
                download = Some((
                    page.token.operation,
                    row[0].as_str().unwrap_or_default().to_string(),
                    row[1].as_str().unwrap_or_default().to_string(),
                    row[2].as_str().map(str::to_string),
                    row[3].as_u64().unwrap_or_default() as usize,
                ));
            }
            assert!(app.acknowledge_typed_operation_result(page.token).expect("ack"), "the presented page accepts its token");
        }
        let _ = app.take_typed_operation_effect();
        let _ = app.take_typed_operation_event();
        let _ = app.take_typed_operation_completion().await.expect("completion witness");
        let _ = app.take_typed_operation_ui_scope();
    }
    let (operation, filename, mime_type, encoding, declared) = download.expect("exactly one segmented download handle was published");
    
    let mut assembled = Vec::with_capacity(declared);
    while let Some(chunk) = app.take_segmented_download_chunk(operation).await.expect("take one chunk") {
        assert!(!chunk.is_empty() && chunk.len() <= semio_framework_plugin::app::ArtifactOutputChunks::CHUNK_BYTES);
        assembled.extend_from_slice(&chunk);
    }
    assert_eq!(assembled.len(), declared, "the handle declares exactly the drained byte count");
    let after=app.0.document_pack().await.expect("original document pack after export");
    assert_eq!(before.pack,after.pack,"export preserves original document bytes");
    assert_eq!(before.spr,after.spr,"export preserves original document revision");
    app.close_fixture();
    (filename,mime_type,encoding,assembled)
}

#[semio_framework_async_macros::async_test]
async fn export_pdf_publishes_a_segmented_download_the_host_can_drain_into_a_whole_pdf(){let(filename,mime,encoding,assembled)=original_layout_export_download(semio_s_artifact_layout_layout_editor::LayoutCommand::ExportPdf(semio_s_artifact_layout_layout_editor_commands::export_pdf::ExportPdf { page_id: None })).await;assert_eq!(filename,"Demo.pdf");assert_eq!(mime,"application/pdf");assert_eq!(encoding.as_deref(),Some("base64"));let pdf=semio_s_artifact_layout_layout_engine_export::testkit::decode_base64(std::str::from_utf8(&assembled).unwrap()).unwrap();assert_eq!(assert_pdf_structure(&pdf),3,"font stream and two original page streams");assert!(String::from_utf8_lossy(&pdf).contains("/Count 2 >>"));}

#[semio_framework_async_macros::async_test]
async fn every_registered_layout_export_uses_the_actual_retained_host_download(){
 use semio_s_artifact_layout_layout_editor::LayoutCommand;
 use semio_s_artifact_layout_layout_engine_export::LayoutExportKind;
 use semio_s_artifact_layout_layout_editor_commands::{export_png::ExportPng,export_svg::ExportSvg,export_pdf::ExportPdf,export_step::ExportStep,export_idml::ExportIdml,export_package::ExportPackage};
 for(kind,command)in[(LayoutExportKind::Png,LayoutCommand::ExportPng(ExportPng{page_id:None})),(LayoutExportKind::Svg,LayoutCommand::ExportSvg(ExportSvg{page_id:None})),(LayoutExportKind::Pdf,LayoutCommand::ExportPdf(ExportPdf{page_id:None})),(LayoutExportKind::Step,LayoutCommand::ExportStep(ExportStep{page_id:None})),(LayoutExportKind::Idml,LayoutCommand::ExportIdml(ExportIdml{})),(LayoutExportKind::Package,LayoutCommand::ExportPackage(ExportPackage{}))]{
  let(filename,mime,encoding,assembled)=original_layout_export_download(command).await;assert!(filename.ends_with(kind.extension()));assert_eq!(mime,kind.mime_type());assert_eq!(encoding.as_deref(),kind.binary().then_some("base64"));let bytes=if kind.binary(){semio_s_artifact_layout_layout_engine_export::testkit::decode_base64(std::str::from_utf8(&assembled).unwrap()).unwrap()}else{assembled};assert!(!bytes.is_empty());match kind{LayoutExportKind::Png=>assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n")),LayoutExportKind::Svg=>assert!(std::str::from_utf8(&bytes).unwrap().contains("<svg")),LayoutExportKind::Pdf=>{assert_pdf_structure(&bytes);},LayoutExportKind::Step=>assert!(bytes.starts_with(b"ISO-10303-21;")),LayoutExportKind::Idml|LayoutExportKind::Package=>assert!(bytes.starts_with(b"PK\x03\x04"))}
 }
}
