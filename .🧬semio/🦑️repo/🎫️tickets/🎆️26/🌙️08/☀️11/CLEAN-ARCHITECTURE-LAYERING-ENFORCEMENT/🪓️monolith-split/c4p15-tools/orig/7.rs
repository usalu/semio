//! 🧾️ Trusted-catalog descriptor: bundle and descriptor validation, document-open target derivation, native codec provider vocabulary and the retained asset handles of a verified package.

use std::sync::Arc;
use directory::os_store::serialization::ArtifactCodec;
use semio_hub_artifact_authority::{AuthorityError, AuthorityProgress, AuthorityProgressStage, OperationContext};
use std::collections::{BTreeMap, BTreeSet};
use directory::os_component_receipt::ComponentReceiptV1;
use directory::os_directory::schema::{DocumentBrowserActorSourceV1, DocumentOpenBrowserActorV1};
use directory::os_directory::{DocumentOpenArtifactV1, DocumentOpenGrantV1, DocumentOpenPackageV1, DocumentOpenRendererTargetV1, DocumentOpenSurfaceRoleV1, DocumentOpenSurfaceV1};
use semio_framework::{DslValue, PackageDescriptor, Version};
use semio_framework_hash::{Hasher, Sha256};
use semio_framework_plugin_host::PackageRef;
use semio_hub_trusted_catalog_contract::{TRUSTED_BUNDLE_MAX_PROFILES, TRUSTED_CATALOG_MAX_OPEN_TARGETS, TRUSTED_COMPONENT_MAX_BYTES, TRUSTED_DESCRIPTOR_MAX_BYTES, TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION, TRUSTED_PACKAGE_MAX_DEPENDENCIES, TRUSTED_READ_CHUNK_BYTES, TRUSTED_RELATIVE_PATH_MAX_BYTES, TrustedCatalogGenerationRoot, TrustedCatalogRelativePathV1, append_document_open_catalog_field, catalog, catalog_error, decode_digest, dual_hash, ensure_count, sha256, valid_identity, valid_open_identity, valid_package_id, validate_file, validate_identity, verify_digest, verify_length};
use semio_hub_trusted_catalog_contract::schema::{TRUSTED_BUNDLE_SCHEMA_VERSION, TrustedBundleFileV1, TrustedBundleGrantV1, TrustedBundleIdentityV1, TrustedBundleOpenRole, TrustedBundleOpenTargetV1, TrustedBundlePackageRole, TrustedBundlePackageV1, TrustedBundleProfileV1, TrustedBundleRendererTarget, TrustedBundleV1};
use semio_hub_artifact_authority::adapters::{TRUSTED_CATALOG_MAX_CODECS, TRUSTED_CATALOG_MAX_PACKAGES};
use semio_hub_trusted_catalog_contract::plugin_module::{TRUSTED_PLUGIN_MODULE_BUNDLE_MAX_BYTES, TrustedPluginModuleSourceV1, decode_plugin_module_bundle, plugin_module_blob_path, verify_plugin_module_file};
use directory::os_plugin_module_schema::{TrustedPluginModuleBundleV1, TrustedPluginModuleFileV1};
use directory::os_directory::io::binary::artifact_hash::hex_lower;
use directory::os_store;

#[derive(Debug)]
pub struct SelectedTrustedBundleV1 {
    pub package_indices: Vec<usize>,
    pub profile: TrustedBundleProfileV1,
}

/// 🔗️ One explicitly linked native executable for an exact bundle artifact identity.
#[derive(Clone)]
pub struct NativeCodecBinding {
    pub plugin_id: String,
    pub package_id: String,
    pub artifact_kind: String,
    pub factory_id: Option<String>,
    pub codec: ArtifactCodec,
}

impl NativeCodecBinding {
    /// 🪢️ Binds a native executable without deriving package identity from plugin identity.
    pub fn new(plugin_id: impl Into<String>, package_id: impl Into<String>, artifact_kind: impl Into<String>, factory_id: Option<String>, codec: ArtifactCodec) -> Self {
        Self { plugin_id: plugin_id.into(), package_id: package_id.into(), artifact_kind: artifact_kind.into(), factory_id, codec }
    }

    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    pub fn package_id(&self) -> &str {
        &self.package_id
    }

    pub fn artifact_kind(&self) -> &str {
        &self.artifact_kind
    }

    pub fn codec(&self) -> &ArtifactCodec {
        &self.codec
    }

    /// 🪪️ Returns only factory identity retained from an actual verified native receipt.
    pub fn factory_id(&self)->Option<&str>{self.factory_id.as_deref()}
}

/// 🪪️ Borrowed immutable package identity passed from the trusted loader to one native provider.
#[derive(Clone, Copy)]
pub struct NativeCodecProviderPackageV1<'a> {
    pub plugin_id: &'a str,
    pub package_id: &'a str,
    pub version: &'a str,
}

/// 🧬️ One descriptor-committed codec requirement exposed without product-plugin types.
#[derive(Clone, Copy)]
pub struct NativeCodecProviderRequirementV1<'a> {
    pub package: NativeCodecProviderPackageV1<'a>,
    pub artifact_kind: &'a str,
    pub artifact_schema: &'a str,
    pub pack_schema_hash: &'a str,
}

/// 🔌️ Headless Hub provider port. Implementations may supply executable codecs only for the exact
/// verified package closure and cannot publish them outside the loader's atomic registration.
pub trait NativeCodecProviderSourceV1: Sync {
    fn preflight_selection(&self, _selected: &[NativeCodecProviderRequirementV1<'_>]) -> Result<(), AuthorityError> {
        Ok(())
    }

    fn preview(&self, package: NativeCodecProviderPackageV1<'_>, descriptor: &PackageDescriptor, context: &OperationContext<'_>) -> Result<Vec<NativeCodecBinding>, AuthorityError>;
}

/// 💾️ One hash-verified generation file retained by IDENTITY, not by bytes: the opened private
/// generation root, the bundle-relative path and the digests the load verified. A read reopens the
/// path beneath that root and re-verifies length and digests, and the bytes stay resident only while
/// some reader holds them, so the hub's memory is the set of assets in use, never the catalog closure.
pub struct TrustedRetainedFile {
    pub root: Arc<TrustedCatalogGenerationRoot>,
    pub path: TrustedCatalogRelativePathV1,
    pub byte_length: u64,
    pub sha256: [u8; 32],
    pub blake3: Option<[u8; 32]>,
    pub resident: std::sync::Mutex<std::sync::Weak<[u8]>>,
}

impl TrustedRetainedFile {
    pub fn new(root: Arc<TrustedCatalogGenerationRoot>, path: TrustedCatalogRelativePathV1, byte_length: u64, sha256: [u8; 32], blake3: Option<[u8; 32]>) -> Self {
        let resident: std::sync::Weak<[u8]> = std::sync::Weak::<[u8; 0]>::new();
        Self { root, path, byte_length, sha256, blake3, resident: std::sync::Mutex::new(resident) }
    }

    pub async fn read(&self, context: &OperationContext<'_>) -> Result<Arc<[u8]>, AuthorityError> {
        if let Some(bytes) = self.resident.lock().unwrap_or_else(std::sync::PoisonError::into_inner).upgrade() {
            return Ok(bytes);
        }
        let bytes = self.root.read_regular(&self.path, self.byte_length, context).await?;
        verify_length(self.byte_length, bytes.len())?;
        let verified = match self.blake3 {
            Some(blake3) => dual_hash(&bytes, context).await? == (self.sha256, blake3),
            None => sha256(&bytes, context).await? == self.sha256,
        };
        if !verified {
            return Err(catalog("retained trusted file differs from the digests its catalog verified"));
        }
        let bytes: Arc<[u8]> = bytes.into();
        *self.resident.lock().unwrap_or_else(std::sync::PoisonError::into_inner) = Arc::downgrade(&bytes);
        Ok(bytes)
    }
}

/// 🧱️ One verified execution-target asset (a component or a closed browser actor), read on demand.
/// Its length is known without reading; [`Self::read`] yields exactly the bytes the catalog verified.
#[derive(Clone)]
pub struct TrustedCatalogAsset {
    pub source: TrustedCatalogAssetSource,
}

#[derive(Clone)]
enum TrustedCatalogAssetSource {
    Retained(Arc<TrustedRetainedFile>),
    Resident(Arc<[u8]>),
}

impl TrustedCatalogAsset {
    /// 🧷️ An asset whose bytes a caller already holds in memory (a fixture catalog built in-process).
    pub fn resident(bytes: Arc<[u8]>) -> Self {
        Self { source: TrustedCatalogAssetSource::Resident(bytes) }
    }

    pub fn retained(file: TrustedRetainedFile) -> Self {
        Self { source: TrustedCatalogAssetSource::Retained(Arc::new(file)) }
    }

    /// 📏️ The verified byte length, known without reading.
    pub fn byte_length(&self) -> u64 {
        match &self.source {
            TrustedCatalogAssetSource::Retained(file) => file.byte_length,
            TrustedCatalogAssetSource::Resident(bytes) => bytes.len() as u64,
        }
    }

    /// 📖️ The verified bytes; a retained asset is reread and re-verified unless a reader still holds it.
    pub async fn read(&self, context: &OperationContext<'_>) -> Result<Arc<[u8]>, AuthorityError> {
        match &self.source {
            TrustedCatalogAssetSource::Retained(file) => file.read(context).await,
            TrustedCatalogAssetSource::Resident(bytes) => Ok(Arc::clone(bytes)),
        }
    }

    /// 🚰️ The verified bytes as a chunked stream with no request deadline: a retained asset is reopened beneath its
    /// generation root and hashed while it streams (see [`TrustedCatalogAssetStream`]).
    pub fn stream(&self) -> Result<TrustedCatalogAssetStream, AuthorityError> {
        match &self.source {
            TrustedCatalogAssetSource::Retained(file) => {
                let (reader, length) = file.root.open_regular(&file.path)?.into_reader();
                if length != file.byte_length {
                    return Err(catalog("retained trusted file differs from the length its catalog verified"));
                }
                Ok(TrustedCatalogAssetStream {
                    byte_length: file.byte_length,
                    source: TrustedCatalogAssetStreamSource::Retained { reader, streamed: 0, sha256: Sha256::new(), blake3: Hasher::new(), expected: (file.sha256, file.blake3), held: None },
                })
            }
            TrustedCatalogAssetSource::Resident(bytes) => Ok(TrustedCatalogAssetStream { byte_length: bytes.len() as u64, source: TrustedCatalogAssetStreamSource::Resident { bytes: Arc::clone(bytes), offset: 0 } }),
        }
    }
}

/// 🚰️ One verified asset served in bounded chunks. Serving has no absolute deadline — bytes flow while the file is
/// hashed, so a large module on a loaded host is slow, never refused — and the final chunk is released only once the
/// streamed length, SHA-256 and BLAKE3 equal what the catalog verified: a file changed on disk after verification ends
/// the stream in an error before its last bytes, so a reader holding the declared length never completes it. A
/// resident asset streams the bytes the catalog already verified. Dropping the stream stops reading.
pub struct TrustedCatalogAssetStream {
    byte_length: u64,
    source: TrustedCatalogAssetStreamSource,
}

enum TrustedCatalogAssetStreamSource {
    Retained { reader: tokio::fs::File, streamed: u64, sha256: Sha256, blake3: Hasher, expected: ([u8; 32], Option<[u8; 32]>), held: Option<Vec<u8>> },
    Resident { bytes: Arc<[u8]>, offset: usize },
    Finished,
}

impl TrustedCatalogAssetStream {
    /// 📏️ The verified byte length the stream delivers in full or not at all.
    pub fn byte_length(&self) -> u64 {
        self.byte_length
    }

    /// 📦️ The next chunk, `None` after the last one; an error ends the stream.
    pub async fn next_chunk(&mut self) -> Option<Result<Vec<u8>, AuthorityError>> {
        let outcome = match &mut self.source {
            TrustedCatalogAssetStreamSource::Finished => return None,
            TrustedCatalogAssetStreamSource::Resident { bytes, offset } => {
                let end = offset.saturating_add(TRUSTED_READ_CHUNK_BYTES).min(bytes.len());
                let chunk = (*offset < end).then(|| bytes[*offset..end].to_vec());
                *offset = end;
                chunk.map(Ok)
            }
            TrustedCatalogAssetStreamSource::Retained { reader, streamed, sha256, blake3, expected, held } => match Self::next_retained(self.byte_length, reader, streamed, sha256, blake3, *expected, held).await {
                Some(Ok((chunk, verified_last))) => {
                    if verified_last {
                        self.source = TrustedCatalogAssetStreamSource::Finished;
                    }
                    return Some(Ok(chunk));
                }
                refused => refused.map(|chunk| chunk.map(|(chunk, _)| chunk)),
            },
        };
        if !matches!(outcome, Some(Ok(_))) {
            self.source = TrustedCatalogAssetStreamSource::Finished;
        }
        outcome
    }

    /// 🔐️ The next held-back chunk of a retained file, `true` beside the last one, which is released only after the
    /// streamed length and digests equal the verified ones.
    async fn next_retained(byte_length: u64, reader: &mut tokio::fs::File, streamed: &mut u64, sha256: &mut Sha256, blake3: &mut Hasher, expected: ([u8; 32], Option<[u8; 32]>), held: &mut Option<Vec<u8>>) -> Option<Result<(Vec<u8>, bool), AuthorityError>> {
        use tokio::io::AsyncReadExt;
        loop {
            let remaining = byte_length.saturating_sub(*streamed);
            let mut chunk = vec![0u8; usize::try_from(remaining.saturating_add(1)).map_or(TRUSTED_READ_CHUNK_BYTES, |limit| limit.min(TRUSTED_READ_CHUNK_BYTES))];
            let read = match reader.read(&mut chunk).await {
                Ok(read) => read,
                Err(error) => return Some(Err(catalog_error(error))),
            };
            if read == 0 {
                if *streamed != byte_length {
                    return Some(Err(catalog("retained trusted file length changed while streaming")));
                }
                let digests = (std::mem::replace(sha256, Sha256::new()).finalize(), *blake3.finalize().as_bytes());
                if digests.0 != expected.0 || expected.1.is_some_and(|verified| verified != digests.1) {
                    return Some(Err(catalog("retained trusted file differs from the digests its catalog verified")));
                }
                return held.take().map(|chunk| Ok((chunk, true)));
            }
            *streamed = streamed.saturating_add(read as u64);
            if *streamed > byte_length {
                return Some(Err(catalog("retained trusted file grew beyond its verified length while streaming")));
            }
            chunk.truncate(read);
            sha256.update(&chunk);
            blake3.update(&chunk);
            if let Some(previous) = held.replace(chunk) {
                return Some(Ok((previous, false)));
            }
        }
    }
}

/// 🧩️ One package's verified plugin module: its canonical manifest, addressed by the manifest's SHA-256,
/// and every file it lists, retained by identity and reread and re-verified on use.
pub struct VerifiedPluginModule {
    pub bundle_sha256: String,
    pub manifest_bytes: Arc<[u8]>,
    pub bundle: TrustedPluginModuleBundleV1,
    pub files: BTreeMap<String, TrustedCatalogAsset>,
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
    pub plugin_id: String,
    pub package: PackageRef,
    pub version: String,
    pub dependencies: Vec<String>,
    pub plugin_module: VerifiedPluginModule,
    pub component_sha256: [u8; 32],
    pub descriptor_sha256: [u8; 32],
    pub component: TrustedCatalogAsset,
    pub descriptor_bytes: Arc<[u8]>,
    pub receipt: ComponentReceiptV1,
    pub browser_actor: DocumentOpenBrowserActorV1,
    pub browser_actor_asset: Option<TrustedCatalogAsset>,
    pub descriptor: Arc<PackageDescriptor>,
}

impl VerifiedTrustedPackage {
    /// 🪪️ Returns the registry plugin identity, kept separate from `package_ref().package`.
    pub fn plugin_id(&self) -> &str {
        &self.plugin_id
    }

    /// 📦️ Returns the independently attested package id and component BLAKE3 identity.
    pub fn package_ref(&self) -> &PackageRef {
        &self.package
    }

    /// 🏷️ Returns the exact decoded descriptor version string.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// 🔐️ Returns the independently checked component SHA-256.
    pub const fn component_sha256(&self) -> &[u8; 32] {
        &self.component_sha256
    }

    /// 🔐️ Returns the raw committed descriptor-byte SHA-256.
    pub const fn descriptor_sha256(&self) -> &[u8; 32] {
        &self.descriptor_sha256
    }

    /// 🧱️ Returns the verified component, read on demand.
    pub fn component(&self) -> &TrustedCatalogAsset {
        &self.component
    }

    /// 📜️ Returns the exact bytes decoded into `descriptor()`.
    pub fn descriptor_bytes(&self) -> &[u8] {
        &self.descriptor_bytes
    }

    /// 🗂️ Returns the decoded existing `PackageDescriptor`.
    pub fn descriptor(&self) -> &PackageDescriptor {
        &self.descriptor
    }

    /// 🧾️ Returns the shipped receipt binding the semantic descriptor to the component bytes.
    pub fn receipt(&self) -> &ComponentReceiptV1 {
        &self.receipt
    }

    /// 🧩️ Returns the package's verified plugin module.
    pub fn plugin_module(&self) -> &VerifiedPluginModule {
        &self.plugin_module
    }
}

/// 🧱 The exact verified component, raw descriptor bytes and closed actor bound to one current
/// selection. It is produced only by [`VerifiedTrustedCatalog::assets_for_current_selection`] and
/// carries no path, origin or catalog handle; the component and actor are read on demand.
pub struct VerifiedExecutionTargetAssets {
    pub selection: VerifiedDocumentOpenSelectionV1,
    pub component: TrustedCatalogAsset,
    pub descriptor: Arc<[u8]>,
    pub browser_actor: Option<TrustedCatalogAsset>,
}

/// 🧬 One exact document-open choice retained only after the complete catalog verifies.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VerifiedDocumentOpenSelectionV1 {
    pub package: DocumentOpenPackageV1,
    pub artifact: DocumentOpenArtifactV1,
    pub parent_dialect: semio_framework::ArtifactDialect,
    pub surface: DocumentOpenSurfaceV1,
    pub grant: DocumentOpenGrantV1,
    pub browser_actor: DocumentOpenBrowserActorV1,
}

/// 🧩️ Reads, verifies and retains one package's plugin module: the manifest against its record and the
/// package's own verified digests, then every file against the manifest. A file several modules share
/// (the vendored imports, the fonts) is one content-addressed file, verified once per load.
pub async fn verify_plugin_module(
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
    let receipt_sha256 = hex_lower(&Sha256::digest(&record.receipt.encode_canonical()));
    let source = TrustedPluginModuleSourceV1 { plugin_id: &record.plugin_id, package_id: &record.package_id, version: &record.version, component_sha256, descriptor_byte_sha256: descriptor_sha256, receipt_sha256: &receipt_sha256 };
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
pub struct CodecKey {
    pub plugin_id: String,
    pub package_id: String,
    pub artifact_kind: String,
    pub artifact_schema: String,
}

impl CodecKey {
    pub fn from_parts(plugin_id: &str, package_id: &str, artifact_kind: &str, artifact_schema: &str) -> Self {
        Self { plugin_id: plugin_id.to_string(), package_id: package_id.to_string(), artifact_kind: artifact_kind.to_string(), artifact_schema: artifact_schema.to_string() }
    }
}

/// 🗂️ The one pairing rule between a verified descriptor's artifact kinds and its surfaces, shared by
/// publication ([`descriptor_open_targets`]) and verification ([`validate_descriptor_open_target`]). A
/// plugin-level kind (`PluginBuilder::artifact_kind`, the channel GIS still uses) is opened only by the
/// surfaces whose own dialect names it, even when a sibling app lists it as an input (GIS's terrain editor
/// lists the map). Any other kind is opened by the editor that declares it itself (where a plugin migrated
/// onto the declaration tree, ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, stitches its spec)
/// and by the viewers of that editor's dialect, the read-only surface of the same documents. A HOSTED kind
/// (`PluginManifest::hosted_artifact_kinds`) opens in the host surfaces whose dialect names it (a family hosting its owner's
/// kind, `PluginBuilder::host_artifact`) or whose io presents it — an embedded editor of another plugin's artifact
/// (demonstrator's cad editor presents `3d.cad`) — and in the viewers of a presenting editor's dialect.
fn app_opens_kind(descriptor: &PackageDescriptor, app: &semio_framework::AppDefinition, artifact_kind: &str, artifact_schema: &str) -> bool {
    let declares = |kinds: &[semio_framework::ArtifactKindSpec]| kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema);
    if declares(&descriptor.manifest.artifact_kinds) {
        return app.dialect.artifact_kind == artifact_kind;
    }
    if descriptor.manifest.hosted_artifact_kinds.iter().any(|kind| kind.id == artifact_kind && kind.schema == artifact_schema) {
        let presents = |candidate: &semio_framework::AppDefinition| candidate.io.artifact.id == artifact_kind && candidate.io.artifact_schema == artifact_schema;
        return app.dialect.artifact_kind == artifact_kind
            || presents(app)
            || (app.role == semio_framework::AppRole::Viewer && descriptor.manifest.apps.iter().any(|editor| editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect && presents(editor)));
    }
    match app.role {
        semio_framework::AppRole::Editor => declares(&app.artifact_kinds),
        semio_framework::AppRole::Viewer => declares(&app.artifact_kinds) || descriptor.manifest.apps.iter().any(|editor| editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect && declares(&editor.artifact_kinds)),
    }
}

/// 🎯️ Every document-open surface one verified descriptor declares, in app then kind declaration order:
/// each (editor or viewer, kind) pair [`app_opens_kind`] admits, in the app's first window kind, rendered by
/// the catalog's closed wasm actor, with the grant its role fixes (a viewer reads and observes, never writes),
/// so a read-only member opens the same document through its viewer. `os-hub trusted-catalog open-targets`
/// answers exactly this list, so a publisher never re-derives the rule.
pub fn descriptor_open_targets(descriptor: &PackageDescriptor) -> Vec<semio_hub_trusted_catalog_contract::schema::TrustedDescriptorOpenTargetV1> {
    if descriptor.execution != semio_framework::ExecutionMode::Isolated {
        return Vec::new();
    }
    let mut targets = Vec::new();
    for app in &descriptor.manifest.apps {
        if app.id != semio_framework::surface_app_id(&app.dialect, app.role) {
            continue;
        }
        let role = match app.role {
            semio_framework::AppRole::Viewer => TrustedBundleOpenRole::Viewer,
            semio_framework::AppRole::Editor => TrustedBundleOpenRole::Editor,
        };
        let mut seen = BTreeSet::new();
        let editors = descriptor.manifest.apps.iter().filter(|editor| app.role == semio_framework::AppRole::Viewer && editor.role == semio_framework::AppRole::Editor && editor.dialect == app.dialect);
        let owned = descriptor.manifest.artifact_kinds.iter().chain(app.artifact_kinds.iter()).chain(editors.flat_map(|editor| editor.artifact_kinds.iter())).map(|kind| (kind.id.as_str(), kind.schema.as_str()));
        let hosted = descriptor.manifest.hosted_artifact_kinds.iter().map(|kind| (kind.id.as_str(), kind.schema.as_str()));
        for (kind_id, kind_schema) in owned.chain(hosted) {
            if !seen.insert((kind_id, kind_schema)) || !app_opens_kind(descriptor, app, kind_id, kind_schema) {
                continue;
            }
            targets.push(semio_hub_trusted_catalog_contract::schema::TrustedDescriptorOpenTargetV1 {
                artifact_kind: kind_id.to_string(),
                artifact_schema: kind_schema.to_string(),
                surface_id: app.id.clone(),
                app_id: app.id.clone(),
                window_kind_id: app.window_kinds.first().id.clone(),
                role,
                renderer_target: TrustedBundleRendererTarget::Wasm,
                parent_dialect: app.dialect.clone(),
                grant: TrustedBundleGrantV1 { read: true, write: matches!(role, TrustedBundleOpenRole::Editor), observe: true },
            });
        }
    }
    targets
}

/// 🏠️ The package whose native codec one document-open target of `host` binds: `host` itself when it declares that exact
/// codec, else — for a kind `host` hosts — the ONE declared dependency of `host` in this catalog that declares the exact
/// codec (the load then holds it to the hosted row's explicit owner). Codec rows never move: a host carries none for the
/// kinds it hosts, and no owner is ever read off a kind id.
pub fn open_target_codec_package<'a>(packages: &'a [TrustedBundlePackageV1], host: &'a TrustedBundlePackageV1, target: &TrustedBundleOpenTargetV1) -> Result<&'a TrustedBundlePackageV1, AuthorityError> {
    let binds = |package: &TrustedBundlePackageV1| package.native_codecs.iter().any(|codec| codec.artifact_kind == target.artifact_kind && codec.artifact_schema == target.artifact_schema && codec.pack_schema_hash == target.pack_schema_hash);
    if binds(host) {
        return Ok(host);
    }
    let mut owners = packages.iter().filter(|package| package.plugin_id != host.plugin_id && host.dependencies.iter().any(|dependency| dependency.plugin_id == package.plugin_id) && binds(package));
    let owner = owners.next().ok_or_else(|| catalog("trusted document-open target binds no exact native codec of its own package or of a declared dependency in the catalog"))?;
    if owners.next().is_some() {
        return Err(catalog("trusted document-open target binds an exact native codec more than one declared dependency declares"));
    }
    Ok(owner)
}

/// 🏠️ The distinct HOSTED codec identities of `host`'s `selected` open targets, each with the declared dependency that owns
/// it ([`open_target_codec_package`]). A document a host creates or opens names the host as its descriptor owner (the host's
/// component ships the surface), so the catalog executes it under the host's own identity: the owner's native codec when the
/// hub links one, and the host's component for genesis and every guest codec call — never the owner's component, which need
/// not ship the surface (the `stdio` component ships none of its families' apps).
pub fn hosted_codec_targets<'a>(packages: &'a [TrustedBundlePackageV1], host: &'a TrustedBundlePackageV1, selected: impl Fn(&TrustedBundleOpenTargetV1) -> bool) -> Result<Vec<(&'a TrustedBundlePackageV1, &'a TrustedBundleOpenTargetV1)>, AuthorityError> {
    let mut hosted: Vec<(&TrustedBundlePackageV1, &TrustedBundleOpenTargetV1)> = Vec::new();
    for target in host.open_targets.iter().filter(|target| selected(target)) {
        let owner = open_target_codec_package(packages, host, target)?;
        if owner.plugin_id != host.plugin_id && !hosted.iter().any(|(_, known)| known.artifact_kind == target.artifact_kind && known.artifact_schema == target.artifact_schema && known.pack_schema_hash == target.pack_schema_hash) {
            hosted.push((owner, target));
        }
    }
    Ok(hosted)
}

/// 🌱️ The owner-preferred creation rule over two candidate tiers — the owners' editors, then the hosts' editors: the first
/// non-empty tier decides by [`most_general_dialect`], so a host never makes an owner's kind ambiguous, a whole-standard
/// editor creates for its subset editors, and two most general editors (two owners, two standards) never pick one.
pub fn owner_preferred_creation<T>(owned: Vec<T>, hosted: Vec<T>, dialect: impl Fn(&T) -> &semio_framework::ArtifactDialect) -> MostGeneralDialect<T> {
    most_general_dialect(if owned.is_empty() { hosted } else { owned }, dialect)
}

/// 🌳️ What [`most_general_dialect`] answers over the surfaces of one role that share a document.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MostGeneralDialect<T> {
    One(T),
    Ambiguous,
    Empty,
}

impl<T> MostGeneralDialect<T> {
    /// 🎯️ The one answer, when there is one.
    pub fn one(self) -> Option<T> {
        match self {
            Self::One(candidate) => Some(candidate),
            Self::Ambiguous | Self::Empty => None,
        }
    }
}

/// 🌳️ The ONE most-general-dialect rule ([fixture](../../../🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/🧫️fixtures/🌳️most-general-dialect/🔣️.json),
/// read by the guest's `artifact_codec_owner` too): the candidate no other candidate covers ([`dialect_covers`]); several
/// uncovered candidates are ambiguous and refused rather than resolved by order.
pub fn most_general_dialect<T>(candidates: Vec<T>, dialect: impl Fn(&T) -> &semio_framework::ArtifactDialect) -> MostGeneralDialect<T> {
    let covered = candidates.iter().map(|candidate| candidates.iter().any(|other| dialect_covers(dialect(other), dialect(candidate)))).collect::<Vec<_>>();
    let mut uncovered = candidates.into_iter().zip(covered).filter_map(|(candidate, covered)| (!covered).then_some(candidate));
    match (uncovered.next(), uncovered.next()) {
        (Some(one), None) => MostGeneralDialect::One(one),
        (Some(_), Some(_)) => MostGeneralDialect::Ambiguous,
        (None, _) => MostGeneralDialect::Empty,
    }
}

/// 🌳️ Whether `general` is the whole-standard dialect (`<kind>@<standard>/*`) of `subset`'s own kind and standard: every
/// document of a subset of a standard is a document of the whole standard, so the whole-standard surface covers it.
fn dialect_covers(general: &semio_framework::ArtifactDialect, subset: &semio_framework::ArtifactDialect) -> bool {
    let any = semio_framework::SubsetId::ANY.0;
    general.subset == any && subset.subset != any && general.artifact_kind == subset.artifact_kind && general.standard == subset.standard
}

/// 🏷️ The declaration of `selection`'s exact kind among `kinds`.
pub fn declared_kind<'a>(mut kinds: impl Iterator<Item = &'a semio_framework::ArtifactKindSpec>, selection: &VerifiedDocumentOpenSelectionV1) -> Option<&'a semio_framework::ArtifactKindSpec> {
    kinds.find(|kind| kind.id == selection.artifact.kind && kind.schema == selection.artifact.schema)
}

/// 📤️ Answers `os-hub trusted-catalog open-targets`: decodes one bounded descriptor exactly as the loader
/// does and serializes [`descriptor_open_targets`] as `TrustedCatalogDescriptorOpenTargetsV1`.
pub fn descriptor_open_targets_answer(descriptor_bytes: &[u8]) -> Result<Vec<u8>, AuthorityError> {
    if descriptor_bytes.is_empty() || descriptor_bytes.len() as u64 > TRUSTED_DESCRIPTOR_MAX_BYTES {
        return Err(catalog("descriptor bytes are empty or exceed the trusted descriptor bound"));
    }
    let descriptor = decode_package_descriptor(descriptor_bytes)?;
    let targets = descriptor_open_targets(&descriptor);
    if targets.len() > TRUSTED_CATALOG_MAX_OPEN_TARGETS {
        return Err(AuthorityError::ResourceLimit("trusted document-open target count"));
    }
    let mut bytes = serde_json::to_vec(&semio_hub_trusted_catalog_contract::schema::TrustedCatalogDescriptorOpenTargetsV1 { schema: semio_hub_trusted_catalog_contract::schema::TRUSTED_CATALOG_DESCRIPTOR_OPEN_TARGETS_SCHEMA, targets }).map_err(catalog_error)?;
    bytes.push(b'\n');
    Ok(bytes)
}

pub fn validate_descriptor_open_target(descriptor: &PackageDescriptor, target: &TrustedBundleOpenTargetV1) -> Result<semio_framework::ArtifactDialect, AuthorityError> {
    let expected_role = match target.role {
        TrustedBundleOpenRole::Viewer => semio_framework::AppRole::Viewer,
        TrustedBundleOpenRole::Editor => semio_framework::AppRole::Editor,
    };
    let app = descriptor.manifest.apps.iter().find(|app| app.id == target.app_id).ok_or_else(|| catalog("document-open target app is absent from the verified descriptor"))?;
    if !app_opens_kind(descriptor, app, &target.artifact_kind, &target.artifact_schema)
        || app.id != target.surface_id
        || app.id != semio_framework::surface_app_id(&app.dialect, app.role)
        || app.role != expected_role
        || app.dialect != target.parent_dialect
        || [&app.dialect.artifact_kind, &app.dialect.standard, &app.dialect.subset].into_iter().any(|value| !valid_open_identity(value))
        || !app.window_kinds.iter().any(|window| window.id == target.window_kind_id)
        || descriptor.execution != semio_framework::ExecutionMode::Isolated
        || target.renderer_target != TrustedBundleRendererTarget::Wasm
    {
        return Err(catalog("document-open target surface, app, window, role, renderer, or artifact differs from the verified descriptor"));
    }
    Ok(app.dialect.clone())
}

pub fn document_open_target_sort_key(target: &VerifiedDocumentOpenSelectionV1) -> [&str; 18] {
    let role = match target.surface.role {
        DocumentOpenSurfaceRoleV1::Viewer => "viewer",
        DocumentOpenSurfaceRoleV1::Editor => "editor",
    };
    let renderer = match target.surface.renderer_target {
        DocumentOpenRendererTargetV1::React => "react",
        DocumentOpenRendererTargetV1::Wgpu => "wgpu",
        DocumentOpenRendererTargetV1::Wasm => "wasm",
    };
    [
        &target.package.plugin_id,
        &target.package.package_id,
        &target.package.version,
        &target.package.component_sha256,
        &target.package.component_blake3,
        &target.package.descriptor_byte_sha256,
        &target.artifact.kind,
        &target.artifact.schema,
        &target.artifact.pack_schema_hash,
        &target.parent_dialect.artifact_kind,
        &target.parent_dialect.standard,
        &target.parent_dialect.subset,
        &target.surface.surface_id,
        &target.surface.app_id,
        &target.surface.window_kind_id,
        role,
        renderer,
        if target.grant.write { "111" } else { "101" },
    ]
}

pub fn sort_open_targets(targets: &mut [VerifiedDocumentOpenSelectionV1]) {
    targets.sort_by(|left, right| document_open_target_sort_key(left).cmp(&document_open_target_sort_key(right)));
}

pub fn selected_closure_digest(identities: &[TrustedBundleIdentityV1]) -> Result<[u8; 32], AuthorityError> {
    let mut encoded = b"semio/hub/trusted-profile-selected-closure/v1\0".to_vec();
    encoded.extend_from_slice(&u32::try_from(identities.len()).map_err(catalog_error)?.to_be_bytes());
    for identity in identities {
        for value in [identity.plugin_id.as_bytes(), identity.package_id.as_bytes(), identity.version.as_bytes()] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
    }
    Ok(Sha256::digest(&encoded))
}

pub fn package_actor_renderer(package: &TrustedBundlePackageV1) -> &'static str {
    if package.open_targets.iter().any(|target| matches!(target.renderer_target, TrustedBundleRendererTarget::Wasm)) {
        "wasm"
    } else {
        "react"
    }
}

/// 🧮️ Frames resolved dependency identities in native tuple order for every profile generation.
pub fn append_trusted_profile_dependencies(encoded: &mut Vec<u8>, dependencies: &[TrustedBundleIdentityV1]) -> Result<(), AuthorityError> {
    let mut dependencies = dependencies.iter().collect::<Vec<_>>();
    dependencies.sort();
    encoded.extend_from_slice(&u32::try_from(dependencies.len()).map_err(catalog_error)?.to_be_bytes());
    for dependency in dependencies {
        for value in [dependency.plugin_id.as_bytes(), dependency.package_id.as_bytes(), dependency.version.as_bytes()] {
            append_document_open_catalog_field(encoded, value)?;
        }
    }
    Ok(())
}

/// 🎯️ The whole SET, in one canonical order, so a generation id names every creatable kind it
/// admits — not merely the first (ticket 26/09/18 slice TC3b). The count is framed ahead of the
/// rows exactly as the selected closure's is, so adding a target can never collide with a
/// different bundle whose rows happen to concatenate identically.
pub fn trusted_profile_generation(bundle: &TrustedBundleV1, profile: &TrustedBundleProfileV1) -> Result<String, AuthorityError> {
    let mut encoded = Vec::new();
    encoded.extend_from_slice(b"semio/hub/trusted-profile-generation/v2\0");
    append_document_open_catalog_field(&mut encoded, profile.id.as_bytes())?;
    encoded.extend_from_slice(&u32::try_from(profile.selected_closure.len()).map_err(catalog_error)?.to_be_bytes());
    for identity in &profile.selected_closure {
        let package = bundle
            .packages
            .iter()
            .find(|package| package.plugin_id == identity.plugin_id && package.package_id == identity.package_id && package.version == identity.version)
            .ok_or_else(|| catalog("profile generation package is outside its selected closure"))?;
        let role = match package.role {
            TrustedBundlePackageRole::Plugin => b"plugin".as_slice(),
            TrustedBundlePackageRole::Extension => b"extension".as_slice(),
        };
        for value in [
            package.plugin_id.as_bytes(),
            package.package_id.as_bytes(),
            package.version.as_bytes(),
            role,
            decode_digest(&package.component.sha256, "profile component sha256")?.as_slice(),
            decode_digest(&package.component.blake3, "profile component blake3")?.as_slice(),
            decode_digest(&package.descriptor.sha256, "profile descriptor sha256")?.as_slice(),
            package.receipt.encode_canonical().as_slice(),
            package.execution_protocol.app_channel_version.to_be_bytes().as_slice(),
        ] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
        package.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &package.component.sha256, descriptor_byte_sha256: &package.descriptor.sha256 }, package_actor_renderer(package))?;
        package.browser_actor.append_generation(&mut encoded)?;
        append_document_open_catalog_field(&mut encoded, package.plugin_module.path.as_bytes())?;
        encoded.extend_from_slice(&package.plugin_module.byte_length.to_be_bytes());
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.sha256, "profile plugin module sha256")?.as_slice())?;
        append_document_open_catalog_field(&mut encoded, decode_digest(&package.plugin_module.blake3, "profile plugin module blake3")?.as_slice())?;
        append_trusted_profile_dependencies(&mut encoded, &package.dependencies)?;
        let mut codecs = package.native_codecs.iter().collect::<Vec<_>>();
        codecs.sort_by(|left, right| (&left.artifact_kind, &left.artifact_schema, &left.pack_schema_hash).cmp(&(&right.artifact_kind, &right.artifact_schema, &right.pack_schema_hash)));
        encoded.extend_from_slice(&u32::try_from(codecs.len()).map_err(catalog_error)?.to_be_bytes());
        for codec in codecs {
            append_document_open_catalog_field(&mut encoded, codec.artifact_kind.as_bytes())?;
            append_document_open_catalog_field(&mut encoded, codec.artifact_schema.as_bytes())?;
            append_document_open_catalog_field(&mut encoded, decode_digest(&codec.pack_schema_hash, "profile codec pack schema hash")?.as_slice())?;
        }
    }
    let mut selected = profile.open_targets.iter().collect::<Vec<_>>();
    selected.sort_by(|left, right| {
        (&left.package.plugin_id, &left.package.package_id, &left.package.version, &left.target.artifact_kind, &left.target.artifact_schema, &left.target.surface_id, left.target.role as u8)
            .cmp(&(&right.package.plugin_id, &right.package.package_id, &right.package.version, &right.target.artifact_kind, &right.target.artifact_schema, &right.target.surface_id, right.target.role as u8))
    });
    encoded.extend_from_slice(&u32::try_from(selected.len()).map_err(catalog_error)?.to_be_bytes());
    for selection in selected {
        let package = bundle
            .packages
            .iter()
            .find(|package| package.plugin_id == selection.package.plugin_id && package.package_id == selection.package.package_id && package.version == selection.package.version)
            .ok_or_else(|| catalog("profile generation open-target package is absent"))?;
        let target = &selection.target;
        let role = match target.role {
            TrustedBundleOpenRole::Viewer => b"viewer".as_slice(),
            TrustedBundleOpenRole::Editor => b"editor".as_slice(),
        };
        let renderer = match target.renderer_target {
            TrustedBundleRendererTarget::React => b"react".as_slice(),
            TrustedBundleRendererTarget::Wgpu => b"wgpu".as_slice(),
            TrustedBundleRendererTarget::Wasm => b"wasm".as_slice(),
        };
        for value in [
            package.plugin_id.as_bytes(),
            package.package_id.as_bytes(),
            package.version.as_bytes(),
            decode_digest(&package.component.sha256, "open target component sha256")?.as_slice(),
            decode_digest(&package.component.blake3, "open target component blake3")?.as_slice(),
            decode_digest(&package.descriptor.sha256, "open target descriptor sha256")?.as_slice(),
            package.execution_protocol.app_channel_version.to_be_bytes().as_slice(),
            target.artifact_kind.as_bytes(),
            target.artifact_schema.as_bytes(),
            decode_digest(&target.pack_schema_hash, "open target pack schema hash")?.as_slice(),
            target.parent_dialect.artifact_kind.as_bytes(),
            target.parent_dialect.standard.as_bytes(),
            target.parent_dialect.subset.as_bytes(),
            target.surface_id.as_bytes(),
            target.app_id.as_bytes(),
            target.window_kind_id.as_bytes(),
            role,
            renderer,
            [u8::from(target.grant.read), u8::from(target.grant.write), u8::from(target.grant.observe)].as_slice(),
        ] {
            append_document_open_catalog_field(&mut encoded, value)?;
        }
    }
    Ok(hex_lower(&Sha256::digest(&encoded)))
}

/// 🧬️ Recomputes the two canonical profile digests without trusting carried digest fields.
pub fn trusted_profile_digests_json(bundle_bytes: &[u8], profile_id: &str) -> Result<(String, String), AuthorityError> {
    let bundle: TrustedBundleV1 = serde_json::from_slice(bundle_bytes).map_err(catalog_error)?;
    let profile = bundle.profiles.iter().find(|profile| profile.id == profile_id).ok_or_else(|| catalog("trusted profile digest source is missing"))?;
    Ok((hex_lower(&selected_closure_digest(&profile.selected_closure)?), trusted_profile_generation(&bundle, profile)?))
}

/// 🎯️ An open target carries TWO artifact-kind ids from two deliberately distinct spaces, and they
/// are never equal in a real bundle: `artifact_kind` is the manifest `ArtifactKindSpec::id`
/// (`stdio.json` — `validate_descriptor_open_target` requires a manifest kind with exactly that id
/// and schema), while `parent_dialect` is the owning app's `Dialect`, whose kind is the descriptor
/// id (`s.stdio.json` — the same function requires `app.dialect == target.parent_dialect`). Every
/// stdio artifact ships both spellings (`📜️native-codec-factories.json`: `artifact_kind` is
/// `stdio.<x>` for all of them; every `Viewer::builder(…)` dialect is `s.stdio.<x>`). This loop
/// used to refuse a target whose two spellings differed, which made every real stdio bundle
/// unloadable and left `artifactAuthority` permanently not-ready. Binding is enforced where it is
/// meaningful: to a native codec of the same package below, and to the descriptor's own app and
/// artifact kind in `validate_descriptor_open_target`.
pub fn validate_bundle(bundle: &TrustedBundleV1, profile_id: &str) -> Result<SelectedTrustedBundleV1, AuthorityError> {
    if bundle.schema_version != TRUSTED_BUNDLE_SCHEMA_VERSION || bundle.packages.is_empty() || bundle.packages.len() > TRUSTED_CATALOG_MAX_PACKAGES || bundle.profiles.is_empty() || bundle.profiles.len() > TRUSTED_BUNDLE_MAX_PROFILES || !valid_identity(profile_id) {
        return Err(catalog("trusted bundle shape or version is invalid"));
    }
    let mut plugins = BTreeMap::new();
    let mut package_ids = BTreeSet::new();
    let mut paths = BTreeSet::new();
    for (index, package) in bundle.packages.iter().enumerate() {
        let identity = TrustedBundleIdentityV1 { plugin_id: package.plugin_id.clone(), package_id: package.package_id.clone(), version: package.version.clone() };
        validate_identity(&identity)?;
        if plugins.insert(package.plugin_id.as_str(), index).is_some() || !package_ids.insert(package.package_id.as_str()) {
            return Err(catalog("trusted plugin or package identity is duplicated"));
        }
        Version::parse(&package.version).map_err(catalog_error)?;
        ensure_count(package.dependencies.len(), TRUSTED_PACKAGE_MAX_DEPENDENCIES, "trusted dependency count")?;
        ensure_count(package.native_codecs.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
        ensure_count(package.open_targets.len(), TRUSTED_CATALOG_MAX_OPEN_TARGETS, "trusted document-open target count")?;
        validate_file(&TrustedBundleFileV1 { path: package.component.path.clone(), byte_length: package.component.byte_length, sha256: package.component.sha256.clone() }, TRUSTED_COMPONENT_MAX_BYTES)?;
        decode_digest(&package.component.blake3, "component blake3")?;
        validate_file(&package.descriptor, TRUSTED_DESCRIPTOR_MAX_BYTES)?;
        if !paths.insert(package.component.path.clone()) || !paths.insert(package.descriptor.path.clone()) {
            return Err(catalog("trusted file path is reused across package records"));
        }
        package.browser_actor.validate(DocumentBrowserActorSourceV1 { component_sha256: &package.component.sha256, descriptor_byte_sha256: &package.descriptor.sha256 }, package_actor_renderer(package))?;
        if let Some(file) = package.browser_actor.file() {
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
        }
        let mut dependencies = BTreeSet::new();
        for dependency in &package.dependencies {
            validate_identity(dependency)?;
            Version::parse(&dependency.version).map_err(catalog_error)?;
            if !dependencies.insert(dependency) || dependency.plugin_id == package.plugin_id {
                return Err(catalog("trusted dependency identity is duplicated or self-referential"));
            }
        }
        let mut codec_identities = BTreeSet::new();
        for codec in &package.native_codecs {
            if !valid_identity(&codec.artifact_kind) || !valid_identity(&codec.artifact_schema) || decode_digest(&codec.pack_schema_hash, "pack schema hash")? == [0; 32] || !codec_identities.insert((codec.artifact_kind.as_str(), codec.artifact_schema.as_str())) {
                return Err(catalog("trusted native codec identity is empty, zero, or duplicated"));
            }
        }
        let mut open_target_keys = BTreeSet::new();
        for target in &package.open_targets {
            let expected_grant = TrustedBundleGrantV1 { read: true, write: matches!(target.role, TrustedBundleOpenRole::Editor), observe: true };
            if !valid_open_identity(&target.artifact_kind)
                || !valid_open_identity(&target.artifact_schema)
                || !valid_open_identity(&target.surface_id)
                || !valid_open_identity(&target.app_id)
                || !valid_open_identity(&target.window_kind_id)
                || [&target.parent_dialect.artifact_kind, &target.parent_dialect.standard, &target.parent_dialect.subset].into_iter().any(|value| !valid_open_identity(value))
            {
                return Err(catalog("trusted document-open target names an empty, padded or control-bearing identity"));
            }
            if target.grant != expected_grant {
                return Err(catalog("trusted document-open target grant differs from the one its role fixes"));
            }
            if decode_digest(&target.pack_schema_hash, "open target pack schema hash")? == [0; 32] {
                return Err(catalog("trusted document-open target pack schema hash is zero"));
            }
            open_target_codec_package(&bundle.packages, package, target)?;
            if !open_target_keys.insert((target.artifact_kind.as_str(), target.artifact_schema.as_str(), target.surface_id.as_str(), target.role as u8)) {
                return Err(catalog("trusted document-open target key is duplicated within one package"));
            }
        }
    }
    for package in &bundle.packages {
        for dependency in &package.dependencies {
            let index = *plugins.get(dependency.plugin_id.as_str()).ok_or_else(|| catalog("trusted bundle dependency is incomplete"))?;
            let expected = &bundle.packages[index];
            if expected.package_id != dependency.package_id || expected.version != dependency.version {
                return Err(catalog("trusted bundle dependency identity conflicts with its package record"));
            }
        }
    }
    let mut profiles = BTreeSet::new();
    let mut selected = None;
    for profile in &bundle.profiles {
        if !valid_identity(&profile.id)
            || profile.selected_closure.is_empty()
            || profile.selected_closure.len() > TRUSTED_CATALOG_MAX_PACKAGES
            || !profiles.insert(profile.id.as_str())
            || decode_digest(&profile.selected_closure_sha256, "selected closure sha256")? == [0; 32]
            || decode_digest(&profile.generation_id, "profile generation id")? == [0; 32]
        {
            return Err(catalog("trusted bundle profile is empty, oversized, or duplicated"));
        }
        let mut prior_plugin = None;
        let mut closure = BTreeSet::new();
        for identity in &profile.selected_closure {
            validate_identity(identity)?;
            if prior_plugin.is_some_and(|prior: &str| prior >= identity.plugin_id.as_str()) {
                return Err(catalog("trusted bundle selected closure is not in canonical plugin order"));
            }
            prior_plugin = Some(identity.plugin_id.as_str());
            let index = *plugins.get(identity.plugin_id.as_str()).ok_or_else(|| catalog("trusted bundle selected closure is incomplete"))?;
            let expected = &bundle.packages[index];
            if expected.package_id != identity.package_id || expected.version != identity.version || !closure.insert(index) {
                return Err(catalog("trusted bundle selected closure conflicts with its package record"));
            }
        }
        if hex_lower(&selected_closure_digest(&profile.selected_closure)?) != profile.selected_closure_sha256 {
            return Err(catalog("trusted bundle selected closure digest differs"));
        }
        if profile.open_targets.is_empty() || profile.open_targets.len() > TRUSTED_CATALOG_MAX_OPEN_TARGETS {
            return Err(catalog("trusted profile declares no document-open target or exceeds the generation ceiling"));
        }
        let mut profile_target_keys = BTreeSet::new();
        for selection in &profile.open_targets {
            validate_identity(&selection.package)?;
            let target_index = *plugins.get(selection.package.plugin_id.as_str()).ok_or_else(|| catalog("trusted profile open target package is absent"))?;
            let target_package = &bundle.packages[target_index];
            if !closure.contains(&target_index) || target_package.package_id != selection.package.package_id || target_package.version != selection.package.version || !target_package.open_targets.contains(&selection.target) {
                return Err(catalog("trusted profile open target is outside its selected closure"));
            }
            if !profile_target_keys.insert((
                selection.package.plugin_id.as_str(),
                selection.package.package_id.as_str(),
                selection.target.artifact_kind.as_str(),
                selection.target.artifact_schema.as_str(),
                selection.target.surface_id.as_str(),
                selection.target.role as u8,
            )) {
                return Err(catalog("trusted profile declares the same document-open target twice"));
            }
        }
        if profile.id == "local-stdio-gis-open-v1" {
            let identities = profile.selected_closure.iter().map(|identity| (identity.plugin_id.as_str(), identity.package_id.as_str())).collect::<Vec<_>>();
            let gis = bundle.packages.iter().find(|package| package.plugin_id == "gis");
            let stdio = bundle.packages.iter().find(|package| package.plugin_id == "stdio");
            if identities != [("gis", "semio:gis"), ("stdio", "semio:stdio")]
                || bundle.packages.len() != 2
                || gis.is_none_or(|package| {
                    package.native_codecs.len() != 2
                        || package.dependencies.as_slice() != std::slice::from_ref(&profile.selected_closure[1])
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gismap" && codec.artifact_schema == "gis.map")
                        || !package.native_codecs.iter().any(|codec| codec.artifact_kind == "s.gis.gisterrain" && codec.artifact_schema == "gis.terrain")
                })
                || stdio.is_none_or(|package| package.native_codecs.len() != 29 || !package.dependencies.is_empty())
                || profile.open_targets.len() != bundle.packages.iter().map(|package| package.open_targets.len()).sum::<usize>()
            {
                return Err(catalog("local stdio plus GIS profile is not its exact closed two-package native-codec closure opening every package target"));
            }
        }
        if profile.id == profile_id {
            selected = Some((profile, closure));
        }
    }
    let (profile, closure) = selected.ok_or_else(|| catalog("selected trusted bundle profile is missing"))?;
    let mut indegree = BTreeMap::new();
    let mut dependents: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &index in &closure {
        let package = &bundle.packages[index];
        indegree.insert(index, package.dependencies.len());
        for dependency in &package.dependencies {
            let dependency_index = *plugins.get(dependency.plugin_id.as_str()).ok_or_else(|| catalog("selected dependency closure is incomplete"))?;
            let dependency_package = &bundle.packages[dependency_index];
            if !closure.contains(&dependency_index) || dependency_package.package_id != dependency.package_id || dependency_package.version != dependency.version {
                return Err(catalog("selected dependency identity conflicts with its package record"));
            }
            dependents.entry(dependency_index).or_default().push(index);
        }
    }
    let mut ready: BTreeSet<(String, usize)> = indegree.iter().filter(|(_, degree)| **degree == 0).map(|(index, _)| (bundle.packages[*index].plugin_id.clone(), *index)).collect();
    let mut order = Vec::with_capacity(closure.len());
    while let Some((_, index)) = ready.pop_first() {
        order.push(index);
        for dependent in dependents.get(&index).into_iter().flatten() {
            let degree = indegree.get_mut(dependent).expect("selected dependent has indegree");
            *degree -= 1;
            if *degree == 0 {
                ready.insert((bundle.packages[*dependent].plugin_id.clone(), *dependent));
            }
        }
    }
    if order.len() != closure.len() {
        return Err(catalog("selected dependency closure contains a cycle"));
    }
    Ok(SelectedTrustedBundleV1 { package_indices: order, profile: profile.clone() })
}

pub fn validate_native_bindings(bindings: &[NativeCodecBinding]) -> Result<BTreeMap<CodecKey, &NativeCodecBinding>, AuthorityError> {
    ensure_count(bindings.len(), TRUSTED_CATALOG_MAX_CODECS, "trusted codec count")?;
    let mut map = BTreeMap::new();
    for binding in bindings {
        if !valid_identity(&binding.plugin_id) || !valid_package_id(&binding.package_id) || !valid_identity(&binding.artifact_kind) || !valid_identity(&binding.codec.schema) {
            return Err(catalog("native codec binding identity is invalid"));
        }
        let key = CodecKey::from_parts(&binding.plugin_id, &binding.package_id, &binding.artifact_kind, &binding.codec.schema);
        if map.insert(key, binding).is_some() {
            return Err(catalog("native codec binding identity is duplicated"));
        }
    }
    Ok(map)
}

/// 🪪️ A decoded semantic descriptor must be exactly the package its trust record names, speaking this hub's app channel,
/// and the record's shipped receipt must bind exactly that descriptor to exactly the record's component bytes. A refusal
/// names the package and every field that differs; a catalog published for another app channel is named as such, because
/// the operator's remedy (publish the catalog again with this tree) differs from a tampered descriptor's.
pub fn validate_descriptor(record: &TrustedBundlePackageV1, descriptor: &PackageDescriptor, packages: &[TrustedBundlePackageV1]) -> Result<(), AuthorityError> {
    let differing: Vec<&str> = [
        (descriptor.descriptor_version != 1, "descriptorVersion"),
        (!valid_package_id(&descriptor.package_id) || descriptor.package_id != record.package_id, "packageId"),
        (!record.role.matches(descriptor.role), "role"),
        (descriptor.manifest.plugin_id != record.plugin_id, "pluginId"),
        (descriptor.manifest.version != record.version, "version"),
        (descriptor.execution_protocol != record.execution_protocol, "executionProtocol"),
        (record.receipt.wasm_bytes != record.component.byte_length, "receipt wasmBytes"),
    ]
    .into_iter()
    .filter_map(|(differs, field)| differs.then_some(field))
    .collect();
    if !differing.is_empty() {
        return Err(catalog(&format!("package {} descriptor identity does not exactly match its trust record: {}", record.package_id, differing.join(", "))));
    }
    if descriptor.execution_protocol.app_channel_version != directory::os_spr::CHANNEL_VERSION {
        return Err(catalog(&format!(
            "package {} was published for app channel {} but this hub speaks app channel {}: publish the trusted catalog again with this hub's tree",
            record.package_id,
            descriptor.execution_protocol.app_channel_version,
            directory::os_spr::CHANNEL_VERSION
        )));
    }
    record.receipt.binds(directory::os_component_receipt::ComponentProfileV1::Release, &record.plugin_id, &record.package_id, &record.component.sha256, &record.descriptor.sha256).map_err(|refusal| catalog(&format!("package {} {refusal}", record.package_id)))?;
    if descriptor.manifest.dependencies.len() != record.dependencies.len() {
        return Err(catalog("decoded manifest dependency count does not match the trust record"));
    }
    let records: BTreeMap<&str, &TrustedBundlePackageV1> = packages.iter().map(|package| (package.plugin_id.as_str(), package)).collect();
    let mut manifest_dependencies = BTreeSet::new();
    for dependency in &descriptor.manifest.dependencies {
        if !manifest_dependencies.insert(dependency.plugin_id.as_str()) {
            return Err(catalog("decoded manifest dependency is duplicated"));
        }
        let expected = record.dependencies.iter().find(|entry| entry.plugin_id == dependency.plugin_id).ok_or_else(|| catalog("decoded manifest dependency is absent from the trust record"))?;
        let package = records.get(expected.plugin_id.as_str()).ok_or_else(|| catalog("decoded manifest dependency package is missing"))?;
        let version = Version::parse(&package.version).map_err(catalog_error)?;
        if expected.package_id != package.package_id || expected.version != package.version || !dependency.version.matches(&version) {
            return Err(catalog("decoded manifest dependency version or package identity conflicts"));
        }
    }
    let mut manifest_kinds = BTreeSet::new();
    for kind in &descriptor.manifest.artifact_kinds {
        if !manifest_kinds.insert(kind.id.as_str()) {
            return Err(catalog("decoded manifest artifact kind is duplicated"));
        }
        if !record.native_codecs.iter().any(|codec| codec.artifact_kind == kind.id && codec.artifact_schema == kind.schema) {
            return Err(catalog("decoded manifest artifact kind is absent from the trust record"));
        }
    }
    let mut hosted_kinds = BTreeSet::new();
    for kind in &descriptor.manifest.hosted_artifact_kinds {
        if !hosted_kinds.insert((kind.id.as_str(), kind.schema.as_str())) || manifest_kinds.contains(kind.id.as_str()) {
            return Err(catalog("decoded manifest hosted kind is duplicated or also owned"));
        }
        if kind.owner.is_empty() || kind.owner == record.plugin_id {
            return Err(catalog("decoded manifest hosted kind names no other owner package"));
        }
        if !manifest_dependencies.contains(kind.owner.as_str()) || !records.contains_key(kind.owner.as_str()) {
            return Err(catalog("decoded manifest hosted kind's owner is not a declared dependency present in the catalog"));
        }
    }
    Ok(())
}

pub fn decode_package_descriptor(bytes: &[u8]) -> Result<PackageDescriptor, AuthorityError> {
    let mut options = os_store::PackDecodeOptions::default();
    options.limits.max_file_len = TRUSTED_DESCRIPTOR_MAX_BYTES;
    options.limits.max_segment_len = TRUSTED_DESCRIPTOR_MAX_BYTES;
    options.limits.max_symbols = 131_072;
    options.limits.max_items = 262_144;
    options.limits.max_depth = 64;
    options.limits.max_total_alloc = TRUSTED_DESCRIPTOR_MAX_MATERIALIZATION;
    let value = os_pack::pack_rt::decode_wire_value_with_options(bytes, &options).map_err(catalog_error)?;
    reject_duplicate_descriptor_fields(&value)?;
    let canonical = os_pack::pack_rt::encode_wire_value(&value);
    if canonical != bytes {
        return Err(catalog("package descriptor is not its exact canonical schema projection"));
    }
    let descriptor: PackageDescriptor = semio_framework_value::FromValue::from_value(value).map_err(catalog_error)?;
    let projection = semio_framework_value::ToValue::to_value(&descriptor);
    if os_pack::pack_rt::encode_wire_value(&projection) != bytes { return Err(catalog("package descriptor is not its exact canonical schema projection")); }
    Ok(descriptor)
}

fn reject_duplicate_descriptor_fields(value: &DslValue) -> Result<(), AuthorityError> {
    match value {
        DslValue::Array(items) => {
            for item in items {
                reject_duplicate_descriptor_fields(item)?;
            }
        }
        DslValue::Object(entries) => {
            let mut keys = BTreeSet::new();
            for (key, item) in entries {
                if !keys.insert(key.as_str()) {
                    return Err(catalog("package descriptor contains a duplicate object field"));
                }
                reject_duplicate_descriptor_fields(item)?;
            }
        }
        _ => {}
    }
    Ok(())
}

pub fn report_package_progress(context: &OperationContext<'_>, package_position: usize, package_phase: u64, total_units: u64) -> Result<(), AuthorityError> {
    let position = u64::try_from(package_position).map_err(catalog_error)?;
    let completed_units = position.checked_mul(4).and_then(|units| units.checked_add(package_phase)).ok_or_else(|| catalog("catalog progress overflow"))?;
    context.report(AuthorityProgress { stage: AuthorityProgressStage::CatalogLoading, completed_units, total_units })
}
