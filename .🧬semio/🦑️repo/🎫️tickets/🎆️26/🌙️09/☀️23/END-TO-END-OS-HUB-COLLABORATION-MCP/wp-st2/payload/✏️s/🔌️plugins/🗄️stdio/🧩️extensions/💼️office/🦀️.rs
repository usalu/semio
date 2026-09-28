//! 💼️ `stdio-office` — the Office Open XML docx/pptx/xlsx apps as their own wasm component, over the artifact kinds and codecs the
//! `stdio` package owns.
//!
//! Every registered app monomorphises the whole app runtime and is live code inside its component, so one stdio
//! component cannot assemble all 176 stdio apps; each family ships its bounded fleet as its own package and depends on
//! `stdio` for the kinds it opens (`🗄️stdio/🧪️tests/🚢️shipped-fleet`: every stdio app is shipped by exactly one
//! package, every stdio kind is opened by exactly one package).

#![allow(async_fn_in_trait)]
#![allow(long_running_const_eval)]

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet of the docx/pptx/xlsx editors and viewers — one editor and one viewer per subset.
    pub enum StdioOfficeApps: PluginApp {
        DocxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>>),
        DocxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>>),
        DocxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>>),
        DocxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>>),
        DocxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>>),
        DocxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>>),
        PptxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>>),
        PptxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>>),
        PptxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>>),
        PptxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>>),
        PptxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>>),
        PptxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>>),
        XlsxEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>>),
        XlsxViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>>),
        XlsxStrictEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>>),
        XlsxStrictViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>>),
        XlsxTransitionalEditor(VcsArtifactApp<EditorApp<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>>),
        XlsxTransitionalViewer(VcsArtifactApp<ViewerApp<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the `stdio-office` bundle: every docx/pptx/xlsx subset's editor and viewer (with the owner-mutation roster where
/// the subset's mutation enum derives one), one activation per artifact kind it opens read live from that kind's own
/// `artifact_kind().id`, and the exact-pin runtime dependency on `stdio`, which owns those kinds and their codecs.
pub fn plugin() -> Result<Plugin<StdioOfficeApps>, PluginAssemblyError> {
    Plugin::<StdioOfficeApps>::builder("stdio-office")
        .label("Stdio Office")
        .version(env!("CARGO_PKG_VERSION"))
        .package_id("semio:stdio-office")
        .depends_on("stdio", semio_framework::tree_pin!())
        .editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::DocxEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::base::create_docx_editor())
        .viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::DocxViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::base::create_docx_viewer())
        .editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::DocxStrictEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_editor())
        .viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::DocxStrictViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::strict::create_docx_strict_viewer())
        .editor::<semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalEditor>(semio_s_artifact_stdio_docx::editor::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_editor())
        .viewer::<semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::DocxTransitionalViewer>(semio_s_artifact_stdio_docx::viewer::docx::standards::v_ecma_376::subsets::transitional::create_docx_transitional_viewer())
        .editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::PptxEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::base::create_pptx_editor())
        .viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::PptxViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::base::create_pptx_viewer())
        .editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::PptxStrictEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_editor())
        .viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::PptxStrictViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::strict::create_pptx_strict_viewer())
        .editor::<semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalEditor>(semio_s_artifact_stdio_pptx::editor::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_editor())
        .viewer::<semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::PptxTransitionalViewer>(semio_s_artifact_stdio_pptx::viewer::pptx::standards::v_ecma_376::subsets::transitional::create_pptx_transitional_viewer())
        .editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::XlsxEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_editor())
        .viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::XlsxViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::base::create_xlsx_viewer())
        .editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_editor())
        .viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::XlsxStrictViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::strict::create_xlsx_strict_viewer())
        .editor::<semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalEditor>(semio_s_artifact_stdio_xlsx::editor::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_editor())
        .viewer::<semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::XlsxTransitionalViewer>(semio_s_artifact_stdio_xlsx::viewer::xlsx::standards::v_ecma_376::subsets::transitional::create_xlsx_transitional_viewer())
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_docx::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_pptx::artifact_kind().id })
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_stdio_xlsx::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest {
            id: CapabilityId("artifacts.write".into()),
            scope: "plugin".into(),
            reason: "persist docx/pptx/xlsx editor edits back to the open stdio document".into(),
            optional: false,
        })
        .try_build()
}

#[cfg(feature = "plugin-root")]
semio_framework_plugin::plugin_exports!(plugin, StdioOfficeApps);
