import pathlib

path = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = path.read_text()


def replace(old, new, count=1):
    global text
    assert text.count(old) == count, (text.count(old), old[:100])
    text = text.replace(old, new)


replace("""/// 📨️ What one detached open step answered: a hub document's resolved program (`None` when the local one
/// already is the lease's component), an app instance, or a seeded document.
enum ShellDocumentOpenAnswer {
    #[cfg(not(target_arch = "wasm32"))]
    Resolved(Option<ProgramBridgeEntry>),""", """/// 📨️ What one detached open step answered: a hub document's resolved owner — the plugin and app its lease
/// names, and the program to mount (`None` when the local one already is the lease's component) — an app
/// instance, or a seeded document.
enum ShellDocumentOpenAnswer {
    #[cfg(not(target_arch = "wasm32"))]
    Resolved { plugin_id: String, app_id: String, program: Option<ProgramBridgeEntry> },""")

replace("""    /// Answers `false` for a relay that names no hub document of an explicit app, which opens as before.
    #[cfg(not(target_arch = "wasm32"))]
    fn begin_document_resolution(&mut self, target: &OpenArtifactRelayTarget) -> bool {
        let (Some(plugin_id), Some(app_id), Some(document_id), Some(schema), Some(space_id), Some(client)) =
            (target.plugin_id.clone(), target.app_id.clone(), target.document_id.clone(), target.schema.clone(), self.open_space_id.clone(), self.directory_client.clone())
        else {
            return false;
        };
        let local = self.plugins.iter().find(|entry| entry.plugin_id == plugin_id).cloned();
        let local_app = local.as_ref().and_then(|program| program.manifest.apps.iter().find(|app| app.id == app_id).cloned());
        let surface_id = local_app.as_ref().map_or_else(|| semio_framework::manifest::surface_app_id(&target.dialect, target.role), |app| semio_framework::manifest::surface_app_id(&app.dialect, app.role));
        let label = local_app.map_or_else(|| app_id.clone(), |app| app.label.resolve(self.active_terminology(), self.active_locale()).to_string());""", """    /// The lease names the plugin and app that open the document, so a relay that names none (the creation
    /// door's, a Space index row's) resolves exactly like one that does. Answers `false` for a relay that names
    /// no hub document, which opens as before.
    #[cfg(not(target_arch = "wasm32"))]
    fn begin_document_resolution(&mut self, target: &OpenArtifactRelayTarget) -> bool {
        let (Some(document_id), Some(schema), Some(space_id), Some(client)) = (target.document_id.clone(), target.schema.clone(), self.open_space_id.clone(), self.directory_client.clone()) else {
            return false;
        };
        let local_app = target.app_id.as_ref().and_then(|app_id| self.plugins.iter().flat_map(|program| program.manifest.apps.iter()).find(|app| app.id == *app_id).cloned());
        let surface_id = local_app.as_ref().map_or_else(|| semio_framework::manifest::surface_app_id(&target.dialect, target.role), |app| semio_framework::manifest::surface_app_id(&app.dialect, app.role));
        let label = local_app.map_or_else(|| target.app_id.clone().unwrap_or_else(|| target.artifact_ref.clone()), |app| app.label.resolve(self.active_terminology(), self.active_locale()).to_string());
        let local_shas: Vec<(String, Option<String>)> = self.plugins.iter().map(|program| (program.plugin_id.clone(), program.component_sha256.clone())).collect();""")
replace("""        let local_sha = local.as_ref().and_then(|program| program.component_sha256.clone());
        let resolving_plugin = plugin_id.clone();
        let pending = ShellDetached::spawn(async move {
            let store = semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore::for_user();
            let resolved = client
                .resolve_execution_target_module(&ctx, &intent, local_sha.as_deref(), &store, |step| reported.store(document_open_resolve_step_code(step), std::sync::atomic::Ordering::Release))
                .await
                .map_err(|error| format!("document execution target: {error}"))?;
            let Some(files) = resolved.files else { return Ok(ShellDocumentOpenAnswer::Resolved(None)) };
            crate::program_bridge::load_resolved_program(&resolving_plugin, &files.component, &files.descriptor, &resolved.lease.component.sha256).await.map(|program| ShellDocumentOpenAnswer::Resolved(Some(program)))
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Resolving, plugin_id, app_id, Some(ShellDocumentOpenTarget { document_id, schema }), Some(pending));""", """        let expected_plugin = target.plugin_id.clone();
        let pending = ShellDetached::spawn(async move {
            let store = semio_framework_os_kernel::os_directory::client::ExecutionTargetModuleStore::for_user();
            let lease = client.document_execution_target_manifest(&ctx, &intent).await.map_err(|error| format!("document execution target: {error}"))?;
            let plugin_id = lease.package.plugin_id.clone();
            if expected_plugin.as_ref().is_some_and(|expected| *expected != plugin_id) {
                return Err(format!("the hub opens this document with {plugin_id}, not {}", expected_plugin.unwrap_or_default()));
            }
            let local_sha = local_shas.into_iter().find(|(local, _)| *local == plugin_id).and_then(|(_, sha)| sha);
            let resolved = client
                .resolve_execution_target_module(&ctx, &intent, local_sha.as_deref(), &store, |step| reported.store(document_open_resolve_step_code(step), std::sync::atomic::Ordering::Release))
                .await
                .map_err(|error| format!("document execution target: {error}"))?;
            let app_id = resolved.lease.surface.app_id.clone();
            let program = match resolved.files {
                Some(files) => Some(crate::program_bridge::load_resolved_program(&plugin_id, &files.component, &files.descriptor, &resolved.lease.component.sha256).await?),
                None => None,
            };
            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program })
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Resolving, target.plugin_id.clone().unwrap_or_default(), target.app_id.clone().unwrap_or_default(), Some(ShellDocumentOpenTarget { document_id, schema }), Some(pending));""")
replace("""            Ok(ShellDocumentOpenAnswer::Resolved(resolved)) => {
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                self.continue_resolved_open(opening, resolved).await;
            }""", """            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program }) => {
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                opening.plugin_id = plugin_id;
                opening.app_id = app_id;
                self.continue_resolved_open(opening, program).await;
            }""")
path.write_text(text)
print("ok")
