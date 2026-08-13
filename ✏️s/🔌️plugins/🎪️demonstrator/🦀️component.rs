//! 🔌️ Plugin root contract for the demonstrator multi-pane bundle.

use semio_framework_plugin::Plugin;

const PLUGIN_ID: &str = "demonstrator";
const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
const PLUGIN_VERSION: &str = "0.1.0";

/// 🔌️ Builds the demonstrator plugin: `.artifact(...)` declares the owned `playground` artifact
/// (ticket 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, replacing the old side-effecting
/// per-leaf `register()` the artifact used to expose from its now-deleted engine dir), then
/// `apps::bundle` layers the six entwerfen-mit-bestand surfaces' document codecs + apps onto the
/// already-built `Plugin` (ticket 26/08/13/UNIFIED-STATE-ARCHITECTURE-AND-DEMONSTRATOR-RESTORATION
/// D3 dissolved the former `🎪️panes/` facet into `🎛️apps`). Playground has no app-scope
/// config/presence schema of its own, so unlike `🗒️note` this plugin needs no narrowed `.setup()`
/// call at all.
pub fn plugin() -> Plugin {
    let plugin = Plugin::builder(PLUGIN_ID)
        .label(PLUGIN_LABEL)
        .version(PLUGIN_VERSION)
        .artifact(crate::artifacts::playground::declaration())
        .build();
    crate::apps::bundle(plugin)
}
