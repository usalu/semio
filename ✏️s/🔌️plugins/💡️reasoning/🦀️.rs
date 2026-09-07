//! 🔌️ Plugin root contract — typestate `Plugin::builder` registration for this owner.

use semio_framework_plugin::__semio_dispatch_PluginApp;
use semio_framework_plugin::kernel::{ActivationEvent, CapabilityId, CapabilityRequest};
use semio_framework_plugin::plugin_app_close_prelude::*;
use semio_framework_plugin::{ExecutionMode, Plugin, PluginApp};

//#region 🗃️Apps
/// 🗃️ Closed runtime app fleet for the declaration-owned reasoning surfaces.
semio_framework_dispatch_macros::dyn_enum_close! {
    pub enum ReasoningApps: PluginApp {}
}
//#endregion 🗃️Apps

/// 🔌️ Builds the plugin surface for host registration. `.activation(…)`/`.execution(…)`/
/// `.requests(…)` (ticket 26/08/17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME M6-remaining,
/// `📓️design-abi.md` §3/§6) are this crate's migration proof, mirroring `🗒️note`'s shape. No
/// `.handler(…)` and no `🧩️extensions/` dir anywhere in this crate, so `Isolated` (the SDK default)
/// is honest.
pub fn plugin() -> Result<Plugin<ReasoningApps>, PluginAssemblyError> {
    Plugin::<ReasoningApps>::builder("reasoning")
        .label("Mindmap")
        .version("0.1.0")
        .package_id("semio:reasoning")
        .declare_artifact(crate::artifacts::wires::artifact())
        .editor_mutation_roster::<crate::editor::wires::ReasoningWiresPlayApp>()
        .viewer_mutation_roster::<crate::viewer::wires::WiresViewer>()
        .activation(ActivationEvent::OnArtifactKind { kind: crate::artifacts::wires::artifact_kind().id })
        .execution(ExecutionMode::Isolated)
        .requests(CapabilityRequest { id: CapabilityId("documents.write".into()), scope: "plugin".into(), reason: "persist reasoning wires edits to the open document".into(), optional: false })
        .try_build()
}

//#region 🧪️SurfaceTests
/// 🧪️ Contract §2.5 surface-pair proofs, using the canonical `semio_framework_plugin::testkit`
/// functions (ticket 26/08/16 lane 0-F closed this SDK gap — see `📓️w0-f-report.md`).
#[cfg(test)]
mod surface_tests {
    use semio_framework_plugin::testkit::{assert_editor_and_viewer_share_dialect, assert_viewer_never_mutates};

    #[semio_framework_async_macros::async_test]
    async fn wires_viewer_never_mutates() {
        assert_viewer_never_mutates::<crate::viewer::wires::WiresViewer>();
    }

    #[semio_framework_async_macros::async_test]
    async fn wires_editor_and_viewer_share_dialect() {
        assert_editor_and_viewer_share_dialect::<crate::editor::wires::ReasoningWiresPlayApp, crate::viewer::wires::WiresViewer>();
    }
}
//#endregion 🧪️SurfaceTests

//#region 🪪️IdentityTests
/// 🪪️ One law joining every authority that names this plugin, driven by the language-agnostic tuple
/// `🧪️fixtures/🧫️plugin-identity/🔣️.json` (mirrored from the TypeScript side by the registry's
/// `🪪️plugin-identity.test.ts`, which runs the same join for all 59 rows).
#[cfg(test)]
mod identity_tests {
    const IDENTITY_FIXTURE: &str = include_str!("🧪️fixtures/🧫️plugin-identity/🔣️.json");
    const COMPONENT_MANIFEST: &str = include_str!("📦️packages/🦀️rust/Cargo.toml");
    const DEPLOYMENT_CATALOG: &str = include_str!("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json");
    const GENERATED_REGISTRY: &str = include_str!("../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json");

    /// 🪪️ The language-agnostic identity tuple, read once and shared by every authority assertion.
    fn identity_fixture() -> serde_json::Value {
        serde_json::from_str(IDENTITY_FIXTURE).expect("plugin-identity fixture is JSON")
    }

    /// 🏗️ The whole guest assembly, named — `plugin()` runs `try_build`'s own `semio:<plugin_id>`
    /// package-identity preflight plus every artifact-declaration preflight.
    fn assembled_plugin() -> semio_framework_plugin::Plugin<crate::ReasoningApps> {
        match crate::plugin().await {
            Ok(plugin) => plugin,
            Err(error) => panic!("plugin assembly rejected: {error:?}"),
        }
    }

    /// 🪪️ Every authority that names this plugin must name the SAME identity: the Cargo component
    /// package, the root `builder(…)`/`package_id(…)` pair through the assembled manifest, the
    /// hand-authored deployment catalog row (id + physical module directory), the generated registry
    /// row (`pluginId`/`packageId`/`packageName`) and the canonical `s.<plugin>.<artifact>` owner
    /// segment the assembly gate `plugin-assembly.surface-dependency-gate` derives via
    /// `ArtifactKindId::plugin()`. The Cargo crate NAME (`semio-s-plugin-reasoning-mindmap`) and the
    /// playground VARIANT (`reasoning-wires`) are deliberately other names and are pinned separately,
    /// so a future rename can never silently conflate them with the identity again (the 2026-09-05
    /// regression: `semio:reasoning-mindmap` in Cargo against a `builder("reasoning")` root and
    /// `s.reasoning.*` artifact kinds).
    #[semio_framework_async_macros::async_test]
    async fn plugin_identity_is_the_same_in_every_authority() {
        let fixture = identity_fixture();
        let plugin_id = fixture["pluginId"].as_str().expect("fixture pluginId");
        let package_id = fixture["packageId"].as_str().expect("fixture packageId");
        assert_eq!(package_id, format!("semio:{plugin_id}"));

        let manifest = assembled_plugin().await.manifest;
        assert_eq!(manifest.plugin_id, plugin_id);
        assert!(COMPONENT_MANIFEST.contains(&format!("package = \"{package_id}\"")), "Cargo component package is not {package_id}");
        assert!(COMPONENT_MANIFEST.contains(&format!("name = \"{}\"", fixture["packageName"].as_str().expect("fixture packageName"))));
        assert!(
            COMPONENT_MANIFEST.contains(&format!("variant = \"{}\"", fixture["playgroundVariant"].as_str().expect("fixture playgroundVariant"))),
            "the playground variant row is the OTHER name and must stay declared"
        );

        let prefix = fixture["artifactKindPrefix"].as_str().expect("fixture artifactKindPrefix");
        assert_eq!(prefix, format!("s.{plugin_id}."));
        for app in &manifest.apps {
            assert!(app.id.starts_with(prefix), "{} is not owned by {plugin_id} under the canonical s.<plugin>.<artifact> grammar", app.id);
        }

        let catalog: serde_json::Value = serde_json::from_str(DEPLOYMENT_CATALOG).expect("deployment catalog is JSON");
        let row = catalog["modules"]
            .as_array()
            .expect("deployment catalog modules")
            .iter()
            .find(|entry| entry["pluginId"] == plugin_id)
            .unwrap_or_else(|| panic!("deployment catalog has no row for {plugin_id}"));
        assert_eq!(row["directoryName"], fixture["moduleDirectoryName"]);

        let registry: serde_json::Value = serde_json::from_str(GENERATED_REGISTRY).expect("generated registry is JSON");
        let entry = registry
            .as_array()
            .expect("generated registry rows")
            .iter()
            .find(|entry| entry["pluginId"] == plugin_id)
            .unwrap_or_else(|| panic!("generated registry has no row for {plugin_id}"));
        assert_eq!(entry["packageId"], package_id);
        assert_eq!(entry["packageName"], fixture["packageName"]);
    }
}
//#endregion 🪪️IdentityTests
