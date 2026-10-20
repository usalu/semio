//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};
use semio_framework_plugin::PluginBuilder;

//#region 🗃️Apps
semio_framework_dispatch_macros::dyn_enum_close! {
    /// 🗃️ Closed runtime app fleet for the layout editor and viewer surfaces.
    pub enum LayoutApps: PluginApp {
        Editor(VcsArtifactApp<EditorApp<semio_s_artifact_layout_layout_editor::LayoutPlayApp>>),
        Viewer(VcsArtifactApp<ViewerApp<semio_s_artifact_layout_layout_viewer::LayoutViewer>>),
    }
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. The first act binds the Semio drawing codec to the layout background drawing slot: the layout artifact
/// reads its SQLite DDL, decodes and renders its background child only through that slot, so the declaration below must find it bound. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `LayoutPlayApp::app_schema()` now answers the
/// one thing it used to survive for, registered automatically by `.editor(…)` below. `.editor(…)` +
/// `.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the old single
/// `.document_app(…)` registration — the subset's mutation-capable and read-only surfaces are now two
/// independently addressable apps sharing one `LAYOUT_DIALECT` coordinate.
/// `.activation(…)`/`.execution(…)`/`.requests(…)` (ticket
/// 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M1, `📓️design-abi.md` §3/§6): the host
/// activates one instance whenever a `"2d.layout"` artifact
/// (`semio_s_artifact_layout_layout_model::artifact_kind().id`) is opened, this plugin's actor runs `Isolated`
/// (no cross-plugin extension attachment, no self-tick/`pending_effects` loop found by grep — the
/// SDK default holds), and it asks the broker for document write access because `LayoutPlayApp`
/// persists edits back to the open document. No quota declared: layout's ~20 `Effect` call sites
/// (`DispatchAction`/`DownloadMediaExport`) are per-turn UI/export effects with no evidence of
/// long-running computation, large held buffers, or high-frequency timers. `.asset(…)` declares the fifteen built-in text
/// fonts as package assets the host preloads into `Event::InstanceOpen.assets` (see `LayoutPlayApp::default`, which installs
/// them into the layout engine when an instance opens).
pub fn plugin() -> Result<Plugin<LayoutApps>, PluginAssemblyError> {
    semio_hub_layout_drawing::bind_layout_background_drawing().map_err(|error| PluginAssemblyError::new("layout.background-drawing", error.to_string()))?;
    let mut builder = PluginBuilder::<_, LayoutApps>::new("layout")
        .label("Layout")
        .version("0.1.0")
        .package_id("semio:layout")
        .artifact(semio_s_artifact_layout_layout::declaration().map_err(PluginAssemblyError::definition)?)
        .editor::<semio_s_artifact_layout_layout_editor::LayoutPlayApp>(semio_s_artifact_layout_layout_editor::create_layout_app())
        .editor_mutation_roster::<semio_s_artifact_layout_layout_editor::LayoutPlayApp>()
        .viewer::<semio_s_artifact_layout_layout_viewer::LayoutViewer>(semio_s_artifact_layout_layout_viewer::create_layout_viewer())
        .viewer_mutation_roster::<semio_s_artifact_layout_layout_viewer::LayoutViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: semio_s_artifact_layout_layout_model::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("artifacts.write".into()), scope: "plugin".into(), reason: "persist layout edits to the open document".into(), optional: false });
    for declaration in semio_s_artifact_layout_layout_editor::text_font_asset_declarations() {
        builder = builder.asset(declaration);
    }
    builder.try_build()
}

//#region 🧪️SurfaceTests
#[cfg(test)]
#[path = "🧪️tests/🔬️surface/🦀️.rs"]
mod surface_tests;
//#endregion 🧪️SurfaceTests
