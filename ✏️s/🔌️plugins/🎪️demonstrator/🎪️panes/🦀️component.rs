//! 🎪️ The demonstrator's six panes, assembled onto an already-built plugin.
//!
//! Export registration runs for all six panes BEFORE any app is registered, matching the order the
//! pre-consolidation bundle used: the OS mesh/solid/dwg registries are process-global, so a pane's
//! handlers must be in place before the host can resolve a document it hands back.

use semio_framework_plugin::Plugin;

/// 🎁️ Layers every pane's host exports + app onto `bundle` (the demonstrator root builds `bundle` via
/// `Plugin::builder(...).artifact(playground::declaration()).build()`, ticket
/// 26/08/12/ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE M1, then hands it here).
pub fn bundle(bundle: Plugin) -> Plugin {
    crate::panes::generator::register_exports();
    crate::panes::koordinator::register_exports();
    crate::panes::aggregator::register_exports();
    crate::panes::aussuchen::register_exports();
    crate::panes::bearbeiten::register_exports();
    crate::panes::verfolgen::register_exports();
    let bundle = crate::panes::generator::register_app(bundle);
    let bundle = crate::panes::koordinator::register_app(bundle);
    let bundle = crate::panes::aggregator::register_app(bundle);
    let bundle = crate::panes::aussuchen::register_app(bundle);
    let bundle = crate::panes::bearbeiten::register_app(bundle);
    crate::panes::verfolgen::register_app(bundle)
}

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    const PLUGIN_ID: &str = "demonstrator";
    const PLUGIN_LABEL: &str = "Entwerfen mit Bestand";
    const PLUGIN_VERSION: &str = "0.1.0";

    /// 🧪️ `bundle()` only layers panes onto a plugin it is handed — these tests exercise it in
    /// isolation with a bare `Plugin::new`, matching exactly the identity fields the real
    /// `crate::plugin()` passes through `Plugin::builder(...).build()`.
    fn test_bundle() -> Plugin {
        bundle(Plugin::new(PLUGIN_ID, PLUGIN_LABEL, PLUGIN_VERSION))
    }

    /// 🧪️ Bundle identity is the plugin's wire identity — the registry rows, playground variants and
    /// wasm component filename all key on `"demonstrator"`, so a rename here is a breaking change.
    #[test]
    fn bundle_keeps_its_plugin_identity() {
        let manifest = test_bundle().manifest;
        assert_eq!(manifest.plugin_id, PLUGIN_ID);
        assert_eq!(manifest.label, PLUGIN_LABEL);
        assert_eq!(manifest.version, PLUGIN_VERSION);
    }

    /// 🧪️ The six registered app ids, in registration order, are exactly the six `app = "…"` values
    /// the playground rows in `Cargo.toml` name. This is the demonstrator's whole observable surface:
    /// it owns no document schema, so this list — not a wire-format diff — is what the consolidation
    /// had to preserve byte-for-byte.
    #[test]
    fn bundle_registers_the_six_demonstrator_panes() {
        let ids: Vec<String> = test_bundle().manifest.apps.iter().map(|app| app.id.clone()).collect();
        assert_eq!(ids, vec!["procedural3d-play", "cad-play", "puzzle3d-play", "sourcing-curate", "process3d-play", "gis2d-play"]);
    }

    /// 🧪️ Every registered app declares at least one document schema, so the host can route a document
    /// to a pane — a pane whose codec registration was dropped would surface as an empty list here.
    #[test]
    fn every_pane_declares_a_document_schema() {
        for app in test_bundle().manifest.apps {
            assert!(!app.document.is_empty(), "app {} declares no document schema", app.id);
        }
    }
}
//#endregion 🧪️Tests
