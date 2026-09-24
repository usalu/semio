"""🧩️ S15 (2/3): the trusted plugin module bundle in the hub fixtures and tests, the integration-fixtures GIS profile,
the os-hub binary laws, the hub routes and the loader/serving law. Anchored, assert-once replacements; ran once 10:25."""
import json, re, sys
ROOT = "/Users/ueli/Documents/semio/🌎️hub/🗿️artifact-authority/🔏️trusted-catalog"

def edit(path, pairs):
    s = open(path, encoding="utf-8").read()
    for old, new in pairs:
        n = s.count(old)
        if n != 1:
            sys.exit(f"{path}: anchor count {n}: {old[:160]!r}")
        s = s.replace(old, new)
    open(path, "w", encoding="utf-8").write(s)

# 5. The neutral two-package fixture: v3 records naming (placeholder) plugin module manifests.
p = f"{ROOT}/🧫️fixtures/👥️two-package/🔣️.json"
s = open(p, encoding="utf-8").read()
if s.count('"schemaVersion": 2,') != 1: sys.exit("two-package schemaVersion anchor")
s = s.replace('"schemaVersion": 2,', '"schemaVersion": 3,')
for plugin, digest in (("fixture.editor", "51"), ("fixture.base", "52")):
    anchor = f'        "descriptor": {{\n          "path": "descriptors/{plugin.split(".")[1]}.descriptor.semio",'
    if s.count(anchor) != 1: sys.exit(f"two-package descriptor anchor {plugin}")
    s = s.replace(anchor, f'        "pluginModule": {{ "path": "plugin-module-{plugin}.json", "byteLength": 1, "sha256": "{digest * 32}", "blake3": "{str(int(digest) + 10) * 32}" }},\n' + anchor)
open(p, "w", encoding="utf-8").write(s)
json.load(open(p, encoding="utf-8"))

# 6. Trusted-catalog unit tests.
edit(f"{ROOT}/🧪️tests/🔬️unit/🦀️.rs", [
    ("use crate::artifact_authority::trusted_catalog::schema::{TrustedBundleBrowserActorV1, TrustedBundleCodecV1, TrustedBundleComponentV1, TrustedBundleProfileOpenTargetV1};",
     "use crate::artifact_authority::trusted_catalog::schema::{TrustedBundleBrowserActorV1, TrustedBundleCodecV1, TrustedBundleComponentV1, TrustedBundlePluginModuleV1, TrustedBundleProfileOpenTargetV1};"),
    ('''        if record["browserActor"]["kind"] == "closed-browser-actor" {
            record["browserActor"]["sourceDescriptorByteSha256"] = record["descriptor"]["sha256"].clone();
        }
        std::fs::write(self.root.join(record["descriptor"]["path"].as_str().expect("descriptor path")), bytes).expect("replace descriptor");''', '''        if record["browserActor"]["kind"] == "closed-browser-actor" {
            record["browserActor"]["sourceDescriptorByteSha256"] = record["descriptor"]["sha256"].clone();
        }
        attach_fixture_plugin_module(&self.root, record, &bytes);
        std::fs::write(self.root.join(record["descriptor"]["path"].as_str().expect("descriptor path")), bytes).expect("replace descriptor");'''),
    ('''fn fixture_json() -> serde_json::Value {''', '''/// 🧩️ (Re)writes one fixture record's plugin module for its current descriptor bytes.
fn attach_fixture_plugin_module(root: &Path, record: &mut serde_json::Value, descriptor: &[u8]) {
    record["pluginModule"] = write_fixture_plugin_module(root, record["pluginId"].as_str().unwrap(), record["packageId"].as_str().unwrap(), record["version"].as_str().unwrap(), record["component"]["sha256"].as_str().unwrap(), descriptor).expect("fixture plugin module");
}

fn fixture_json() -> serde_json::Value {'''),
    ('''            browser_actor: serde_json::from_value(synthetic_browser_actor(&"11".repeat(32), &"13".repeat(32), "packages/gis/browser/closed-actor.mjs")).unwrap(),''', '''            browser_actor: serde_json::from_value(synthetic_browser_actor(&"11".repeat(32), &"13".repeat(32), "packages/gis/browser/closed-actor.mjs")).unwrap(),
            plugin_module: TrustedBundlePluginModuleV1 { path: "packages/gis/plugin-module.json".into(), byte_length: 1, sha256: "14".repeat(32), blake3: "15".repeat(32) },'''),
    ('''            browser_actor: TrustedBundleBrowserActorV1::None {},''', '''            browser_actor: TrustedBundleBrowserActorV1::None {},
            plugin_module: TrustedBundlePluginModuleV1 { path: "packages/stdio/plugin-module.json".into(), byte_length: 1, sha256: "24".repeat(32), blake3: "25".repeat(32) },'''),
    ('''    let mut bundle = TrustedBundleV1 {
        schema_version: 2,''', '''    let mut bundle = TrustedBundleV1 {
        schema_version: 3,'''),
    ('''        if index == 0 {
            bundle["packages"][index]["browserActor"]["sourceDescriptorByteSha256"] = bundle["packages"][index]["descriptor"]["sha256"].clone();
        }
        let path = root.join(bundle["packages"][index]["descriptor"]["path"].as_str().expect("descriptor path"));''', '''        if index == 0 {
            bundle["packages"][index]["browserActor"]["sourceDescriptorByteSha256"] = bundle["packages"][index]["descriptor"]["sha256"].clone();
        }
        attach_fixture_plugin_module(&root, &mut bundle["packages"][index], &bytes);
        let path = root.join(bundle["packages"][index]["descriptor"]["path"].as_str().expect("descriptor path"));'''),
    ('''    let bundle = serde_json::json!({
        "schemaVersion": 2,
        "profiles": [{ "id": "frozen-gis-test",''', '''    let mut bundle = serde_json::json!({
        "schemaVersion": 3,
        "profiles": [{ "id": "frozen-gis-test",'''),
    ('''    let mut fixture = FixtureDirectory { bundle_path: root.join("trusted-catalog.json"), root, bundle, schema: "gis.map".into() };''', '''    attach_fixture_plugin_module(&root, &mut bundle["packages"][0], &bytes);
    let mut fixture = FixtureDirectory { bundle_path: root.join("trusted-catalog.json"), root, bundle, schema: "gis.map".into() };'''),
    ('''                if record["browserActor"]["kind"] == "closed-browser-actor" {
                    record["browserActor"]["sourceDescriptorByteSha256"] = record["descriptor"]["sha256"].clone();
                }
                std::fs::write(&originals[index].0, bytes).expect("write resealed complete candidate descriptor");''', '''                if record["browserActor"]["kind"] == "closed-browser-actor" {
                    record["browserActor"]["sourceDescriptorByteSha256"] = record["descriptor"]["sha256"].clone();
                }
                attach_fixture_plugin_module(&fixture.root, record, &bytes);
                std::fs::write(&originals[index].0, bytes).expect("write resealed complete candidate descriptor");'''),
])

# 7. Publication tests copy the plugin module manifest and every content-addressed file.
edit(f"{ROOT}/🧪️tests/📤️publication/🦀️.rs", [
    ('''        let mut paths = vec![record["component"]["path"].as_str().unwrap(), record["descriptor"]["path"].as_str().unwrap()];
        if let Some(path) = record["browserActor"]["path"].as_str() {
            paths.push(path);
        }
        for path in paths {''', '''        let mut paths = vec![record["component"]["path"].as_str().unwrap().to_owned(), record["descriptor"]["path"].as_str().unwrap().to_owned(), record["pluginModule"]["path"].as_str().unwrap().to_owned()];
        if let Some(path) = record["browserActor"]["path"].as_str() {
            paths.push(path.to_owned());
        }
        let manifest: serde_json::Value = serde_json::from_slice(&std::fs::read(fixture.root.join(record["pluginModule"]["path"].as_str().unwrap())).unwrap()).unwrap();
        paths.extend(manifest["files"].as_array().unwrap().iter().map(|file| plugin_module::plugin_module_blob_path(file["sha256"].as_str().unwrap())));
        for path in &paths {'''),
    ('for field in ["component", "descriptor", "browserActor", "bundle"] {', 'for field in ["component", "descriptor", "browserActor", "pluginModule", "bundle"] {'),
])
print("hub tests and fixtures wired")

# 8. The integration-fixtures GIS profile and the os-hub binary's stdio bundle.
HUB = "/Users/ueli/Documents/semio/🌎️hub"
edit(f"{HUB}/🧪️tests/🔏️trusted-catalog-profile/🦀️.rs", [
    ("""    let mut bundle = serde_json::json!({
        "schemaVersion": 2,""", """    let plugin_module = super::write_fixture_plugin_module(root, &descriptor.manifest.plugin_id, &descriptor.package_id, &descriptor.manifest.version, &component_sha256, &descriptor_bytes)?;
    let mut bundle = serde_json::json!({
        "schemaVersion": 3,"""),
    ("""            "nativeCodecs": native_codecs, "openTargets": [target]
        }, stdio_record]""", """            "pluginModule": plugin_module,
            "nativeCodecs": native_codecs, "openTargets": [target]
        }, stdio_record]"""),
])
edit(f"{HUB}/🧪️tests/🔬️bin-unit/🦀️.rs", [
    ("""    let mut bundle = serde_json::json!({
        "schemaVersion": 2,
        "profiles": [{
            "id": "stdio-native-openable-v1",""", """    let mut bundle = serde_json::json!({
        "schemaVersion": 3,
        "profiles": [{
            "id": "stdio-native-openable-v1","""),
    ("""    std::fs::write(stage.join("closed-actor.mjs"), component).expect("synthetic actor, never executed");""", """    std::fs::write(stage.join("closed-actor.mjs"), component).expect("synthetic actor, never executed");
    let plugin_module_files = vec![("stdio/🌉️bridge.js".to_owned(), b"export {};\\n".to_vec()), ("stdio/🔣️.json".to_owned(), b"{}\\n".to_vec()), ("stdio/🛂️.descriptor.semio".to_owned(), descriptor_bytes.clone())];
    let plugin_module_source = semio_hub::artifact_authority::trusted_catalog::plugin_module::TrustedPluginModuleSourceV1 { plugin_id: "stdio", package_id: "semio:stdio", version: &*version, component_sha256: &component_sha256, descriptor_byte_sha256: &descriptor_sha256 };
    let plugin_module = semio_hub::artifact_authority::trusted_catalog::plugin_module::write_plugin_module_bundle(&stage, "plugin-module-stdio.json", plugin_module_source, "stdio", &plugin_module_files).expect("stdio plugin module");
    bundle["packages"][0]["pluginModule"] = serde_json::to_value(plugin_module).expect("stdio plugin module record");"""),
])
print("profile + bin-unit wired")

# 9. Hub routes: the generation's plugin module index, manifests and files.
edit(f"{HUB}/🏗️bootstrap/🦀️.rs", [
    ("use semio_hub::artifact_authority::trusted_catalog::{NativeCodecProviderSourceV1, TrustedCatalogAsset, TrustedCatalogLoader, VerifiedDocumentOpenSelectionV1, VerifiedExecutionTargetAssets, VerifiedTrustedCatalog};",
     "use semio_hub::artifact_authority::trusted_catalog::{NativeCodecProviderSourceV1, TrustedCatalogAsset, TrustedCatalogLoader, VerifiedDocumentOpenSelectionV1, VerifiedExecutionTargetAssets, VerifiedTrustedCatalog};\nuse semio_hub::artifact_authority::trusted_catalog::plugin_module::plugin_module_content_type;"),
    ("//#endregion 🔖️Extensions\n", """//#endregion 🔖️Extensions

//#region 🔖️PluginModules
/// 🧊️ A plugin module manifest and its files are content-addressed and immutable.
const TRUSTED_PLUGIN_MODULE_CACHE_CONTROL: &str = "public, max-age=31536000, immutable";

/// 📇️ `GET /trusted-catalog/plugin-modules`: every plugin module of the current catalog generation. As
/// public as the catalog it indexes: a module is the trusted code a shell loads, never document data.
async fn get_trusted_plugin_module_index(State(state): State<HubState>) -> Response {
    let Some(catalog) = state.verified_catalog.as_ref() else { return StatusCode::SERVICE_UNAVAILABLE.into_response() };
    match serde_json::to_vec(&catalog.plugin_module_index()) {
        Ok(bytes) => (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json"), (axum::http::header::CACHE_CONTROL, "no-store")], bytes).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}

/// 🧩️ `GET /trusted-catalog/plugin-modules/{bundleSha256}`: one plugin module manifest by its content address.
async fn get_trusted_plugin_module_manifest(Path(bundle_sha256): Path<String>, State(state): State<HubState>) -> Response {
    let Some(module) = state.verified_catalog.as_ref().and_then(|catalog| catalog.plugin_module(&bundle_sha256)) else { return StatusCode::NOT_FOUND.into_response() };
    (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, "application/json"), (axum::http::header::CACHE_CONTROL, TRUSTED_PLUGIN_MODULE_CACHE_CONTROL)], module.manifest_bytes().to_vec()).into_response()
}

/// 📖️ `GET /trusted-catalog/plugin-modules/{bundleSha256}/{*path}`: one file of that module, reread and
/// re-verified against the generation it was verified in. A file that no longer verifies is never served.
async fn get_trusted_plugin_module_file(Path((bundle_sha256, path)): Path<(String, String)>, State(state): State<HubState>) -> Response {
    let Some(asset) = state.verified_catalog.as_ref().and_then(|catalog| catalog.plugin_module(&bundle_sha256)).and_then(|module| module.file(&path)) else { return StatusCode::NOT_FOUND.into_response() };
    let control = ExecutionTargetAssetReadControl;
    let context = OperationContext::new(control.now_ms().saturating_add(DOCUMENT_EXECUTION_TARGET_DEADLINE_MS), AuthorityLimits::maximum(), &control);
    match asset.read(&context).await {
        Ok(bytes) => (StatusCode::OK, [(axum::http::header::CONTENT_TYPE, plugin_module_content_type(&path)), (axum::http::header::CACHE_CONTROL, TRUSTED_PLUGIN_MODULE_CACHE_CONTROL)], bytes.to_vec()).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
//#endregion 🔖️PluginModules
"""),
    ("""        .route("/%F0%9F%A7%A9%EF%B8%8Fextension-modules/{extension_id}/{*rest}", get(get_extension_asset))
""", """        .route("/%F0%9F%A7%A9%EF%B8%8Fextension-modules/{extension_id}/{*rest}", get(get_extension_asset))
        .route("/trusted-catalog/plugin-modules", get(get_trusted_plugin_module_index))
        .route("/trusted-catalog/plugin-modules/{bundle_sha256}", get(get_trusted_plugin_module_manifest))
        .route("/trusted-catalog/plugin-modules/{bundle_sha256}/{*path}", get(get_trusted_plugin_module_file))
"""),
])
print("hub routes wired")

# 10. Loader law: the index, every served file, and refusal of a file tampered after verification.
law = '''
#[tokio::test]
async fn a_loaded_catalog_indexes_and_serves_every_verified_plugin_module_file_and_refuses_a_tampered_one() {
    let fixture = prepared_fixture();
    let provider = FixtureProviderSource::new(vec![fixture.binding()]);
    let control = TestControl::new();
    let catalog = TrustedCatalogLoader::load_fixture(&fixture.bundle_path, "fixture", &provider, &control.context()).await.expect("fixture catalog");
    let index = catalog.plugin_module_index();
    plugin_module::validate_plugin_module_index(&index).expect("canonical index");
    assert_eq!(index.generation_id, catalog.generation_id());
    assert_eq!(index.modules.iter().map(|entry| entry.plugin_id.as_str()).collect::<Vec<_>>(), vec!["fixture.base", "fixture.editor"]);
    assert_eq!(index.modules[1].dependencies, vec!["fixture.base".to_owned()]);
    for entry in &index.modules {
        let module = catalog.plugin_module(&entry.bundle_sha256).expect("indexed module");
        assert_eq!(hex_lower(&Sha256::digest(module.manifest_bytes())), entry.bundle_sha256);
        assert_eq!(module.manifest_bytes().len() as u64, entry.bundle_byte_length);
        assert_eq!(module.bundle().entry, entry.entry);
        for file in &module.bundle().files {
            let bytes = module.file(&file.path).expect("listed file").read(&control.context()).await.expect("verified file");
            assert_eq!(hex_lower(&Sha256::digest(&bytes)), file.sha256);
        }
    }
    assert!(catalog.plugin_module(&"00".repeat(32)).is_none());
    let editor = catalog.plugin_module(&index.modules[1].bundle_sha256).expect("editor module");
    let bridge = editor.bundle().files.iter().find(|file| file.path.ends_with("🌉️bridge.js")).expect("editor entry");
    assert!(editor.file("fixture.editor/unlisted.js").is_none());
    std::fs::write(fixture.root.join(plugin_module::plugin_module_blob_path(&bridge.sha256)), b"tampered").expect("tamper entry");
    assert!(editor.file(&bridge.path).expect("editor entry").read(&control.context()).await.is_err(), "a module file tampered after verification is never served");
}
'''
p = f"{ROOT}/🧪️tests/🔬️unit/🦀️.rs"
with open(p, "a", encoding="utf-8") as handle:
    handle.write(law)
print("loader law appended")
