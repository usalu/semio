//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::Plugin;

/// 🔌️ Builds the plugin surface for host registration. `.artifact(…)` (ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1) replaces the old `.setup(engine::register)`
/// escape hatch; `.setup()` itself is gone (W1c) — `FormsPlayApp::app_schema()` now answers the one
/// thing it used to survive for, registered automatically by `.editor(…)` below.
/// `.editor(…)`/`.viewer(…)` (ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET) replace the
/// old single `.document_app(…)` registration with the two role-carrying surfaces for `s.forms.forms@1/*`.
pub fn plugin() -> Result<Plugin, semio_framework_plugin::PluginAssemblyError> {
    Plugin::builder("forms")
        .label("Forms")
        .version("0.1.0")
        .artifact(crate::artifacts::forms::declaration().map_err(semio_framework_plugin::PluginAssemblyError::definition)?)
        .editor::<crate::editor::forms::FormsPlayApp>(crate::editor::forms::create_forms_app())
        .editor_mutation_roster::<crate::editor::forms::FormsPlayApp>()
        .viewer::<crate::viewer::forms::FormsViewer>(crate::viewer::forms::create_forms_viewer())
        .viewer_mutation_roster::<crate::viewer::forms::FormsViewer>()
        .try_build()
}
