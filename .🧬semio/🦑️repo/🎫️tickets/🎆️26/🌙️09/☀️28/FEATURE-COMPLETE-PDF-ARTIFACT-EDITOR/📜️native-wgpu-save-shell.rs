//! 💾 Owns one detached native Save, its progress and local cancellation.
use super::*;
use semio_renderer_wgpu_program_bridge::{NativeMediaDeclaration, NativeSaveOutcome};
use std::sync::{Arc, atomic::{AtomicU64, Ordering}};

pub struct NativeMediaSaveTask {
    pub id: u64,
    pub file: String,
    pub plugin_id: String,
    app: String,
    instance: u32,
    package: Option<String>,
    component: Option<String>,
    pub cancel: CancelToken,
    progress: Arc<AtomicU64>,
    observed: u64,
    answer: ShellDetached<Result<NativeSaveOutcome, String>>,
}

impl Drop for NativeMediaSaveTask {
    fn drop(&mut self) { self.cancel.cancel_now(); }
}

impl NativeMediaSaveTask {
    pub fn completed(&self) -> u64 { self.progress.load(Ordering::Acquire) }
}

impl OpeningState {
    pub fn start_media_save(
        &mut self, program: ProgramBridgeEntry, app: String, instance: u32, declaration: NativeMediaDeclaration,
    ) -> bool {
        if self.native_media_save.is_some() { return false; }
        let id = self.import_transfer_next;
        self.import_transfer_next = id.wrapping_add(1).max(1);
        let file = declaration.filename().to_owned();
        let (plugin_id, package, component) = (program.plugin_id.clone(), program.package_id.clone(), program.component_sha256.clone());
        let cancel = CancelToken::root_now();
        let progress = Arc::new(AtomicU64::new(0));
        let worker_cancel = cancel.clone();
        let worker_progress = Arc::clone(&progress);
        let answer = ShellDetached::spawn(async move {
            if worker_cancel.is_cancelled_now() { return Ok(NativeSaveOutcome::Cancelled); }
            let owner = program.read_app_document_identity(instance).await.map_err(|error| error.to_string())?;
            if owner.parent_document_id.is_none() { return Err("Save requires an attached document".into()); }
            if worker_cancel.is_cancelled_now() { return Ok(NativeSaveOutcome::Cancelled); }
            let extension = declaration.extension().unwrap_or("dat");
            let request = ui_host::NativeFileDialogRequest::save(declaration.filename(), [extension]);
            let Some(path) = ui_host::select_native_paths(request).await.into_iter().next() else {
                return Ok(NativeSaveOutcome::Cancelled);
            };
            program.save_native_media(&owner, &declaration, &path, || worker_cancel.is_cancelled_now(),
                |done| worker_progress.store(done, Ordering::Release)).await
        });
        self.native_media_save = Some(NativeMediaSaveTask { id, file, plugin_id, app, instance, package, component,
            cancel, progress, observed: 0, answer });
        true
    }

    /// ⏱️ Keeps progress and cancellation with the original mounted session until settlement.
    pub fn poll_media_save(
        &mut self, docs: &DocumentState, plugins: &PluginState,
    ) -> (bool, Option<(String, Result<NativeSaveOutcome, String>)>) {
        let Some(task) = self.native_media_save.as_mut() else { return (false, None); };
        let mounted = docs.session.iter().chain(docs.spawned_session.iter()).any(|session|
            session.plugin_id == task.plugin_id && session.app.id == task.app && session.instance_id == task.instance);
        let unchanged = plugins.plugins.iter().any(|plugin| plugin.plugin_id == task.plugin_id &&
            plugin.package_id == task.package && plugin.component_sha256 == task.component);
        if !mounted || !unchanged { task.cancel.cancel_now(); }
        if let Some(answer) = task.answer.take() {
            let task = self.native_media_save.take().expect("settled native Save");
            return (true, Some((task.file.clone(), answer)));
        }
        let now = task.completed();
        let changed = task.observed != now;
        task.observed = now;
        (changed, None)
    }

    pub fn cancel_media_save(&self, id: u64) -> bool {
        let Some(task) = self.native_media_save.as_ref().filter(|owner| owner.id == id) else { return false; };
        task.cancel.cancel_now();
        true
    }
}
