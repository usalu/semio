"""🧩️ S15 (1/3): wires the trusted plugin module bundle into the hub trusted catalog — schema v3, loader, publication,
generation framing, index/serving API, fixture helpers. Anchored, assert-once replacements; ran once 10:24."""
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

# 1. JSON schema: required pluginModule, schemaVersion 3.
edit(f"{ROOT}/🧬️schema/🔣️.json", [
    ('"required": ["pluginId", "packageId", "version", "role", "dependencies", "component", "descriptor", "executionProtocol", "browserActor", "nativeCodecs", "openTargets"],',
     '"required": ["pluginId", "packageId", "version", "role", "dependencies", "component", "descriptor", "executionProtocol", "browserActor", "pluginModule", "nativeCodecs", "openTargets"],'),
    ('''        "browserActor": { "$ref": "#/$defs/TrustedBundleBrowserActorV1" },
        "nativeCodecs":''', '''        "browserActor": { "$ref": "#/$defs/TrustedBundleBrowserActorV1" },
        "pluginModule": { "$ref": "#/$defs/TrustedBundlePluginModuleV1" },
        "nativeCodecs":'''),
    ('"schemaVersion": { "const": 2 },', '"schemaVersion": { "const": 3 },'),
])

# 2. Rust schema projection.
edit(f"{ROOT}/🧬️schema/🦀️.rs", [
    ('''    pub browser_actor: TrustedBundleBrowserActorV1,
    pub native_codecs: Vec<TrustedBundleCodecV1>,''', '''    pub browser_actor: TrustedBundleBrowserActorV1,
    pub plugin_module: TrustedBundlePluginModuleV1,
    pub native_codecs: Vec<TrustedBundleCodecV1>,'''),
])

# 3. Loader, publication, generation framing, index/serving API.
edit(f"{ROOT}/🦀️.rs", [
    ("use schema::{publication_revision, TrustedBundleFileV1, TrustedBundleGrantV1, TrustedBundleIdentityV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageRole, TrustedBundlePackageV1, TrustedBundleProfileV1, TrustedBundleRendererTarget, TrustedBundleV1, TrustedCatalogCurrentPointerV1,",
     "use schema::{publication_revision, TrustedBundleFileV1, TrustedBundleGrantV1, TrustedBundleIdentityV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageRole, TrustedBundlePackageV1, TrustedBundleProfileV1, TrustedBundleRendererTarget, TrustedBundleV1, TrustedPluginModuleBundleV1, TrustedPluginModuleFileV1, TrustedPluginModuleIndexEntryV1, TrustedPluginModuleIndexV1, TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA, TrustedCatalogCurrentPointerV1,"),
    ('''#[path = "🧩️plugin-module/🦀️.rs"]
pub mod plugin_module;
''', '''#[path = "🧩️plugin-module/🦀️.rs"]
pub mod plugin_module;
use plugin_module::{decode_plugin_module_bundle, plugin_module_blob_path, verify_plugin_module_file, TrustedPluginModuleSourceV1, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES};
'''),
    ('''        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let (verified, _) = TrustedCatalogLoader::verify_selected(&generation, relative, bundle_bytes, &command.profile_id, providers, context).await?;
        if verified.generation_id() != command.generation_id { return Err(catalog("trusted publication candidate generation differs from the verified profile")); }
        for package in &verified.packages {''', '''        let bundle: TrustedBundleV1 = serde_json::from_slice(&bundle_bytes).map_err(catalog_error)?;
        let (verified, _) = TrustedCatalogLoader::verify_selected(&generation, relative, bundle_bytes, &command.profile_id, providers, context).await?;
        if verified.generation_id() != command.generation_id { return Err(catalog("trusted publication candidate generation differs from the verified profile")); }
        let mut published_module_files = BTreeSet::new();
        for package in &verified.packages {'''),
    ('''            if let Some(actor) = record.browser_actor.file() { files.push(actor); }
            for file in files {''', '''            if let Some(actor) = record.browser_actor.file() { files.push(actor); }
            files.push(TrustedBundleFileV1 { path: record.plugin_module.path.clone(), byte_length: record.plugin_module.byte_length, sha256: record.plugin_module.sha256.clone() });
            for file in &package.plugin_module.bundle.files {
                if published_module_files.insert(file.sha256.clone()) { files.push(TrustedBundleFileV1 { path: plugin_module_blob_path(&file.sha256), byte_length: file.byte_length, sha256: file.sha256.clone() }); }
            }
            for file in files {'''),
    ('''/// 🧬️ One fully verified package retained in dependency-first order.
pub struct VerifiedTrustedPackage {
    plugin_id: String,
    package: PackageRef,
    version: String,''', '''/// 🧩️ One package's verified plugin module: its canonical manifest, addressed by the manifest's SHA-256,
/// and every file it lists, retained by identity and reread and re-verified on use.
pub struct VerifiedPluginModule {
    bundle_sha256: String,
    manifest_bytes: Arc<[u8]>,
    bundle: TrustedPluginModuleBundleV1,
    files: BTreeMap<String, TrustedCatalogAsset>,
}

impl VerifiedPluginModule {
    /// 🔐️ The content address: the SHA-256 of the canonical manifest bytes.
    pub fn bundle_sha256(&self) -> &str {
        &self.bundle_sha256
    }

    /// 📜️ The exact canonical manifest bytes the catalog verified.
    pub fn manifest_bytes(&self) -> &[u8] {
        &self.manifest_bytes
    }

    /// 🧩️ The decoded manifest.
    pub fn bundle(&self) -> &TrustedPluginModuleBundleV1 {
        &self.bundle
    }

    /// 📖️ The verified file at one module-relative path, read on demand.
    pub fn file(&self, path: &str) -> Option<&TrustedCatalogAsset> {
        self.files.get(path)
    }
}

/// 🧬️ One fully verified package retained in dependency-first order.
pub struct VerifiedTrustedPackage {
    plugin_id: String,
    package: PackageRef,
    version: String,
    dependencies: Vec<String>,
    plugin_module: VerifiedPluginModule,'''),
    ('''    /// 🗂️ Returns the decoded existing `PackageDescriptor`.
    pub fn descriptor(&self) -> &PackageDescriptor {
        &self.descriptor
    }
}''', '''    /// 🗂️ Returns the decoded existing `PackageDescriptor`.
    pub fn descriptor(&self) -> &PackageDescriptor {
        &self.descriptor
    }

    /// 🧩️ Returns the package's verified plugin module.
    pub fn plugin_module(&self) -> &VerifiedPluginModule {
        &self.plugin_module
    }
}'''),
    ('''    /// 🧬 Returns the neutral SHA-256 identity of the sorted immutable open-target projection.
    pub fn generation_id(&self) -> &str {
        &self.generation_id
    }
''', '''    /// 🧬 Returns the neutral SHA-256 identity of the sorted immutable open-target projection.
    pub fn generation_id(&self) -> &str {
        &self.generation_id
    }

    /// 📇️ Every plugin module of this generation in ascending plugin order: what
    /// `GET /trusted-catalog/plugin-modules` answers.
    pub fn plugin_module_index(&self) -> TrustedPluginModuleIndexV1 {
        let mut modules = self
            .packages
            .iter()
            .map(|package| TrustedPluginModuleIndexEntryV1 {
                plugin_id: package.plugin_id.clone(),
                package_id: package.package.package.0.clone(),
                version: package.version.clone(),
                component_sha256: hex_lower(&package.component_sha256),
                descriptor_byte_sha256: hex_lower(&package.descriptor_sha256),
                dependencies: package.dependencies.clone(),
                bundle_sha256: package.plugin_module.bundle_sha256.clone(),
                bundle_byte_length: package.plugin_module.manifest_bytes.len() as u64,
                entry: package.plugin_module.bundle.entry.clone(),
            })
            .collect::<Vec<_>>();
        modules.sort_by(|left, right| left.plugin_id.as_bytes().cmp(right.plugin_id.as_bytes()));
        TrustedPluginModuleIndexV1 { schema: TRUSTED_PLUGIN_MODULE_INDEX_SCHEMA.into(), generation_id: self.generation_id.clone(), modules }
    }

    /// 🧩️ The verified plugin module whose manifest has this SHA-256, when this generation carries it.
    pub fn plugin_module(&self, bundle_sha256: &str) -> Option<&VerifiedPluginModule> {
        self.packages.iter().map(|package| &package.plugin_module).find(|module| module.bundle_sha256 == bundle_sha256)
    }
'''),
    ('''            browser_actor: DocumentOpenBrowserActorV1,
            browser_actor_asset: Option<TrustedCatalogAsset>,
        }

        let mut staged = Vec::with_capacity(order.len());''', '''            browser_actor: DocumentOpenBrowserActorV1,
            browser_actor_asset: Option<TrustedCatalogAsset>,
            plugin_module: VerifiedPluginModule,
        }

        let mut staged = Vec::with_capacity(order.len());
        let mut plugin_module_files = BTreeMap::new();'''),
    ('''            } else {
                None
            };
            report_package_progress(context, position, 3, total_units)?;
            staged.push(StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset });
        }

        for stage in staged {
            let StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset } = stage;''', '''            } else {
                None
            };
            let plugin_module = verify_plugin_module(root, record, &hex_lower(&component_sha256), &hex_lower(&descriptor_sha256), &mut resolved_paths, &mut plugin_module_files, context).await?;
            report_package_progress(context, position, 3, total_units)?;
            staged.push(StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module });
        }

        for stage in staged {
            let StagedTrustedPackage { position, record, component, component_sha256, component_blake3, descriptor_bytes, descriptor_sha256, descriptor, browser_actor, browser_actor_asset, plugin_module } = stage;'''),
    ('''            packages.push(VerifiedTrustedPackage {
                plugin_id: record.plugin_id.clone(),
                package: package_ref,
                version: record.version.clone(),''', '''            packages.push(VerifiedTrustedPackage {
                plugin_id: record.plugin_id.clone(),
                package: package_ref,
                version: record.version.clone(),
                dependencies: record.dependencies.iter().map(|dependency| dependency.plugin_id.clone()).collect(),
                plugin_module,'''),
    ('''#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CodecKey {''', '''/// 🧩️ Reads, verifies and retains one package's plugin module: the manifest against its record and the
/// package's own verified digests, then every file against the manifest. A file several modules share
/// (the vendored imports, the fonts) is one content-addressed file, verified once per load.
async fn verify_plugin_module(
    root: &Arc<TrustedCatalogGenerationRoot>,
    record: &TrustedBundlePackageV1,
    component_sha256: &str,
    descriptor_sha256: &str,
    resolved_paths: &mut BTreeSet<TrustedCatalogRelativePathV1>,
    verified_files: &mut BTreeMap<String, (TrustedPluginModuleFileV1, TrustedCatalogAsset)>,
    context: &OperationContext<'_>,
) -> Result<VerifiedPluginModule, AuthorityError> {
    let manifest_path = TrustedCatalogRelativePathV1::parse(&record.plugin_module.path)?;
    if !resolved_paths.insert(manifest_path.clone()) {
        return Err(catalog("trusted plugin module path is already used by the selected closure"));
    }
    let manifest_bytes = root.read_regular(&manifest_path, TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, context).await?;
    verify_length(record.plugin_module.byte_length, manifest_bytes.len())?;
    let (manifest_sha256, manifest_blake3) = dual_hash(&manifest_bytes, context).await?;
    verify_digest(&record.plugin_module.sha256, manifest_sha256, "plugin module sha256")?;
    verify_digest(&record.plugin_module.blake3, manifest_blake3, "plugin module blake3")?;
    let source = TrustedPluginModuleSourceV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version, component_sha256, descriptor_byte_sha256: descriptor_sha256 };
    let bundle = decode_plugin_module_bundle(&manifest_bytes, source)?;
    let mut files = BTreeMap::new();
    for file in &bundle.files {
        context.checkpoint()?;
        let asset = match verified_files.get(&file.sha256) {
            Some((known, asset)) if known.byte_length == file.byte_length && known.blake3 == file.blake3 => asset.clone(),
            Some(_) => return Err(catalog("trusted plugin module files disagree about one content address")),
            None => {
                let path = TrustedCatalogRelativePathV1::parse(&plugin_module_blob_path(&file.sha256))?;
                if !resolved_paths.insert(path.clone()) {
                    return Err(catalog("trusted plugin module file path is already used by the selected closure"));
                }
                let bytes = root.read_regular(&path, file.byte_length, context).await?;
                let (sha256, blake3) = dual_hash(&bytes, context).await?;
                verify_plugin_module_file(file, bytes.len(), sha256, blake3)?;
                drop(bytes);
                let asset = TrustedCatalogAsset::retained(TrustedRetainedFile::new(Arc::clone(root), path, file.byte_length, sha256, Some(blake3)));
                verified_files.insert(file.sha256.clone(), (file.clone(), asset.clone()));
                asset
            }
        };
        files.insert(file.path.clone(), asset);
    }
    Ok(VerifiedPluginModule { bundle_sha256: hex_lower(&manifest_sha256), manifest_bytes: manifest_bytes.into(), bundle, files })
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct CodecKey {'''),
    ('''        package.browser_actor.append_generation(&mut encoded)?;
        append_trusted_profile_dependencies(&mut encoded, &package.dependencies)?;''', '''        package.browser_actor.append_generation(&mut encoded)?;
        append_document_open_catalog_field(&mut encoded, package.plugin_module.path.as_bytes())?;
        encoded.extend_from_slice(&package.plugin_module.byte_length.to_be_bytes());
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.sha256, "profile plugin module sha256")?.as_slice())?;
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.blake3, "profile plugin module blake3")?.as_slice())?;
        append_trusted_profile_dependencies(&mut encoded, &package.dependencies)?;'''),
    ("    if bundle.schema_version != 2 || bundle.packages.is_empty()", "    if bundle.schema_version != 3 || bundle.packages.is_empty()"),
    ('''        if let Some(file) = package.browser_actor.file() {
            if !paths.insert(file.path) {
                return Err(catalog("trusted browser actor path is reused across package records"));
            }
        }''', '''        if let Some(file) = package.browser_actor.file() {
            if !paths.insert(file.path) {
                return Err(catalog("trusted browser actor path is reused across package records"));
            }
        }
        let module = &package.plugin_module;
        if module.path.is_empty() || module.path.len() > TRUSTED_RELATIVE_PATH_MAX_BYTES || !(1..=TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES).contains(&module.byte_length) {
            return Err(catalog("trusted plugin module record is empty or exceeds its fixed boundary"));
        }
        decode_digest(&module.sha256, "plugin module sha256")?;
        decode_digest(&module.blake3, "plugin module blake3")?;
        if !paths.insert(module.path.clone()) {
            return Err(catalog("trusted plugin module path is reused across package records"));
        }'''),
])
print("hub trusted-catalog wired")

# 4. Fixture helpers beside the headless stdio fixture package.
edit(f"{ROOT}/🦀️.rs", [
    ('''/// 🧫️ Shares headless Stdio metadata between native GIS fixtures; synthetic bytes are never executed.''', '''/// 🧫️ The files of a never-executed fixture plugin module: its entry, both descriptor forms (the packed one
/// the package's own descriptor) and one vendored import shared by every fixture module.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn fixture_plugin_module_files(plugin_id: &str, descriptor_bytes: &[u8]) -> Vec<(String, Vec<u8>)> {
    vec![
        (format!("{plugin_id}/🌉️bridge.js"), b"export async function createActorApi() {}\\n".to_vec()),
        (format!("{plugin_id}/🔣️.json"), serde_json::json!({ "manifest": { "pluginId": plugin_id } }).to_string().into_bytes()),
        (format!("{plugin_id}/🛂️.descriptor.semio"), descriptor_bytes.to_vec()),
        ("🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/io.js".to_owned(), b"export const streams = {};\\n".to_vec()),
    ]
}

/// 🧫️ Writes one package's fixture plugin module beneath `root` and answers the `pluginModule` record naming it.
#[cfg(any(test, feature = "integration-fixtures"))]
pub fn write_fixture_plugin_module(root: &Path, plugin_id: &str, package_id: &str, version: &str, component_sha256: &str, descriptor_bytes: &[u8]) -> Result<serde_json::Value, AuthorityError> {
    let descriptor_byte_sha256 = hex_lower(&Sha256::digest(descriptor_bytes));
    let source = TrustedPluginModuleSourceV1 { plugin_id, package_id, version, component_sha256, descriptor_byte_sha256: &descriptor_byte_sha256 };
    let record = plugin_module::write_plugin_module_bundle(root, &format!("plugin-module-{plugin_id}.json"), source, plugin_id, &fixture_plugin_module_files(plugin_id, descriptor_bytes))?;
    serde_json::to_value(record).map_err(catalog_error)
}

/// 🧫️ Shares headless Stdio metadata between native GIS fixtures; synthetic bytes are never executed.'''),
    ('''    let record = serde_json::json!({
        "pluginId": "stdio", "packageId": "semio:stdio", "version": version, "role": "plugin", "dependencies": [],
        "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
        "component": { "path": "stdio-component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
        "descriptor": { "path": "stdio-descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
        "browserActor": { "kind": "none" }, "nativeCodecs": codecs, "openTargets": [],
    });''', '''    let plugin_module = write_fixture_plugin_module(root, "stdio", "semio:stdio", &version, &component_sha256, &bytes)?;
    let record = serde_json::json!({
        "pluginId": "stdio", "packageId": "semio:stdio", "version": version, "role": "plugin", "dependencies": [],
        "executionProtocol": { "appChannelVersion": descriptor.execution_protocol.app_channel_version },
        "component": { "path": "stdio-component.wasm", "byteLength": component.len(), "sha256": component_sha256, "blake3": hex_lower(component_blake3.finalize().as_bytes()) },
        "descriptor": { "path": "stdio-descriptor.semio", "byteLength": bytes.len(), "sha256": hex_lower(&Sha256::digest(&bytes)) },
        "browserActor": { "kind": "none" }, "pluginModule": plugin_module, "nativeCodecs": codecs, "openTargets": [],
    });'''),
])
print("steps 1-4 wired; steps 5-10 live in s15-apply-plugin-module-hub-2.py, step 3 fixes in -hub-3.py")
