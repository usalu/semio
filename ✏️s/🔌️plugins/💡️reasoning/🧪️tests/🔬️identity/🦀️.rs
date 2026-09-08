
const IDENTITY_FIXTURE: &str = include_str!("../../🧪️fixtures/🧫️plugin-identity/🔣️.json");
const COMPONENT_MANIFEST: &str = include_str!("../../📦️packages/🦀️rust/Cargo.toml");
const DEPLOYMENT_CATALOG: &str = include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment/🗺️catalog.json");
const GENERATED_REGISTRY: &str = include_str!("../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json");

/// 🪪️ The language-agnostic identity tuple, read once and shared by every authority assertion.
fn identity_fixture() -> serde_json::Value {
    serde_json::from_str(IDENTITY_FIXTURE).expect("plugin-identity fixture is JSON")
}

/// 🏗️ The whole guest assembly, named — `plugin()` runs `try_build`'s own `semio:<plugin_id>`
/// package-identity preflight plus every artifact-declaration preflight.
fn assembled_plugin() -> semio_framework_plugin::Plugin<super::ReasoningApps> {
    match super::plugin() {
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

    let manifest = assembled_plugin().manifest;
    assert_eq!(manifest.plugin_id, plugin_id);
    assert!(COMPONENT_MANIFEST.contains(&format!("package = \"{package_id}\"")), "Cargo component package is not {package_id}");
    assert!(COMPONENT_MANIFEST.contains(&format!("name = \"{}\"", fixture["packageName"].as_str().expect("fixture packageName"))));
    assert!(COMPONENT_MANIFEST.contains(&format!("variant = \"{}\"", fixture["playgroundVariant"].as_str().expect("fixture playgroundVariant"))), "the playground variant row is the OTHER name and must stay declared");

    let prefix = fixture["artifactKindPrefix"].as_str().expect("fixture artifactKindPrefix");
    assert_eq!(prefix, format!("s.{plugin_id}."));
    for app in &manifest.apps {
        assert!(app.id.starts_with(prefix), "{} is not owned by {plugin_id} under the canonical s.<plugin>.<artifact> grammar", app.id);
    }

    let catalog: serde_json::Value = serde_json::from_str(DEPLOYMENT_CATALOG).expect("deployment catalog is JSON");
    let row = catalog["modules"].as_array().expect("deployment catalog modules").iter().find(|entry| entry["pluginId"] == plugin_id).unwrap_or_else(|| panic!("deployment catalog has no row for {plugin_id}"));
    assert_eq!(row["directoryName"], fixture["moduleDirectoryName"]);

    let registry: serde_json::Value = serde_json::from_str(GENERATED_REGISTRY).expect("generated registry is JSON");
    let entry = registry.as_array().expect("generated registry rows").iter().find(|entry| entry["pluginId"] == plugin_id).unwrap_or_else(|| panic!("generated registry has no row for {plugin_id}"));
    assert_eq!(entry["packageId"], package_id);
    assert_eq!(entry["packageName"], fixture["packageName"]);
}
