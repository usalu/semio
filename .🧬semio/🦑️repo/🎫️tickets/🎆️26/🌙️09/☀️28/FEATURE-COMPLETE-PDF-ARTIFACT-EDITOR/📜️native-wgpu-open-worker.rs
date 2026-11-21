//! 📂 Owns one picked natural file until its fresh program and archive can be mounted.
use super::*;
use semio_renderer_wgpu_program_bridge::{NATIVE_DOCUMENT_FILE_MAXIMUM, NativeMediaDeclaration, ProgramFault};
use std::{io::Read, path::{Path, PathBuf}};

pub struct NativeOpenedFile {
    pub program: ProgramBridgeEntry,
    pub source_instance: u32,
    pub instance: u32,
    pub document_id: String,
    pub folder: PathBuf,
    published: bool,
}

impl NativeOpenedFile {
    pub fn publish(mut self) { self.published = true; }
}

impl Drop for NativeOpenedFile {
    fn drop(&mut self) {
        if !self.published { self.program.destroy_app(self.instance); }
    }
}

impl OpeningState {
    pub fn start_native_file_open(
        &mut self, program: ProgramBridgeEntry, app: String, source_instance: u32,
        declaration: NativeMediaDeclaration, folder: PathBuf, label: String,
    ) -> bool {
        if self.document_opening.as_ref().is_some_and(ShellDocumentOpening::running) || self.native_media_save.is_some() {
            return false;
        }
        let plugin_id = program.plugin_id.clone();
        let app_id = app.clone();
        let cancel = CancelToken::root_now();
        let worker_cancel = cancel.clone();
        let pending = ShellDetached::spawn(async move {
            open_native_file(program, app, source_instance, declaration, folder, worker_cancel).await
                .map(ShellDocumentOpenAnswer::NativeFile).map_err(ProgramFault::from)
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Instantiating, plugin_id, app_id, None, Some(pending));
        opening.cancel = Some(cancel);
        self.document_opening = Some(opening);
        true
    }
}

async fn open_native_file(
    program: ProgramBridgeEntry, app: String, source_instance: u32, declaration: NativeMediaDeclaration,
    folder: PathBuf, cancel: CancelToken,
) -> Result<Option<NativeOpenedFile>, String> {
    if cancel.is_cancelled_now() { return Ok(None); }
    let extension = declaration.extension().ok_or("Open File has no declared extension")?;
    let Some(path) = ui_host::select_native_paths(ui_host::NativeFileDialogRequest::open([extension], false)).await
        .into_iter().next() else { return Ok(None); };
    let Some(bytes) = read_selected_file(&path, || cancel.is_cancelled_now()).await? else { return Ok(None); };
    let instance = program.create_app(&app).await.map_err(|error| error.to_string())?;
    let mut opened = NativeOpenedFile { program, source_instance, instance, document_id: String::new(), folder, published: false };
    if cancel.is_cancelled_now() { return Ok(None); }
    opened.program.import_native_file(instance, &declaration, bytes).await?;
    if cancel.is_cancelled_now() { return Ok(None); }
    let owner = opened.program.read_app_document_identity(instance).await.map_err(|error| error.to_string())?;
    let document = owner.parent_document_id.as_deref().ok_or("Open File did not create a document identity")?;
    let archive = opened.program.read_app_document_archive(instance).await?;
    let schema = opened.program.manifest.apps.iter().find(|candidate| candidate.id == app)
        .map(|candidate| candidate.io.artifact_schema.as_str()).ok_or("Open File lost its declared editor")?;
    let encoded = protocol::encode_document_archive_bytes(&archive).map_err(|error| error.to_string())?;
    if cancel.is_cancelled_now() || opened.program.read_app_document_identity(instance).await.map_err(|error| error.to_string())? != owner {
        return Ok(None);
    }
    if cancel.is_cancelled_now() { return Ok(None); }
    let storage = store_sync::sync::FolderEventLogStorage::new(opened.folder.clone());
    storage.create_archive_if_absent(document, schema, &encoded).await.map_err(|error| error.to_string())?;
    let readback = storage.read_archive(document).await.map_err(|error| error.to_string())?;
    if readback.as_deref() != Some(encoded.as_slice()) { return Err("Open File could not verify the local document archive".into()); }
    if cancel.is_cancelled_now() { return Ok(None); }
    opened.document_id = document.to_owned();
    Ok(Some(opened))
}

async fn read_selected_file(path: &Path, cancelled: impl Fn() -> bool) -> Result<Option<Vec<u8>>, String> {
    let file = std::fs::File::open(path).map_err(|error| error.to_string())?;
    let metadata = file.metadata().map_err(|error| error.to_string())?;
    if !metadata.is_file() || metadata.len() == 0 || metadata.len() > NATIVE_DOCUMENT_FILE_MAXIMUM as u64 {
        return Err("Open File has an inadmissible input extent".into());
    }
    let mut input = file.take(NATIVE_DOCUMENT_FILE_MAXIMUM as u64 + 1);
    let mut data = Vec::with_capacity(metadata.len() as usize);
    let mut page = [0; 4096];
    loop {
        if cancelled() { return Ok(None); }
        let count = input.read(&mut page).map_err(|error| error.to_string())?;
        if count == 0 { break; }
        if data.len() + count > NATIVE_DOCUMENT_FILE_MAXIMUM { return Err("Open File exceeded its input cap".into()); }
        data.extend_from_slice(&page[..count]);
        semio_framework_async::yield_once().await;
    }
    if data.len() as u64 != metadata.len() { return Err("Open File changed size while being read".into()); }
    Ok(Some(data))
}

pub fn native_local_documents() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os("S_DATA_DIR").map(PathBuf::from).filter(|path| path.is_absolute()) {
        return Some(path.join("documents"));
    }
    #[cfg(target_os = "windows")]
    let root = std::env::var_os("LOCALAPPDATA").map(PathBuf::from);
    #[cfg(target_os = "macos")]
    let root = std::env::var_os("HOME").map(|home| PathBuf::from(home).join("Library/Application Support"));
    #[cfg(not(any(target_os = "windows", target_os = "macos")))]
    let root = std::env::var_os("XDG_DATA_HOME").map(PathBuf::from).filter(|path| path.is_absolute())
        .or_else(|| std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share")));
    root.filter(|path| path.is_absolute()).map(|path| path.join("semio/documents"))
}
