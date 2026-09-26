#!/usr/bin/env python3
"""WG10 s13: the wgpu Shell (both targets) opens a hub document on the hub's canonical checkpoint pair — the one seed
React already opens on — instead of a genesis its own component mints: the pair is fetched through the kernel's
`DirectoryClient::document_canonical_checkpoint_pair`, verified, admitted as the checkpoint of the open's
execution-target lease, loaded into the guest before its actor exists and handed to the actor
(`ArtifactHost::set_document_seed`), which says Hello at the pair's baseline."""
import pathlib

S = pathlib.Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs")
text = S.read_text()


def swap(old, new):
    global text
    assert text.count(old) == 1, (text.count(old), old[:120])
    text = text.replace(old, new)


swap("""/// 🚪️ Where the one frame-pumped document open stands: a hub document's component is being resolved by
/// the serving catalog generation, the guest's app instance is being created, the document's genesis is
/// being loaded into it, or the open settled as cancelled or failed — a settled record stays until the
/// user closes its band, like a settled plugin install.""",
"""/// 🚪️ Where the one frame-pumped document open stands: a hub document's component is being resolved by
/// the serving catalog generation, the guest's app instance is being created, the document's canonical
/// checkpoint pair is being fetched and loaded into it, or the open settled as cancelled or failed — a
/// settled record stays until the user closes its band, like a settled plugin install.""")
swap("""/// 📨️ What one detached open step answered: a hub document's resolved owner — the plugin and app its lease
/// names, and the program to mount (`None` when the local one already is the lease's component) — an app
/// instance, or a seeded document.
enum ShellDocumentOpenAnswer {
    #[cfg(not(target_arch = "wasm32"))]
    Resolved { plugin_id: String, app_id: String, program: Option<ProgramBridgeEntry> },
    Instantiated(u32),
    Seeded,
}""",
"""/// 📨️ What one detached open step answered: a hub document's resolved owner — the plugin and app its lease
/// names, the program to mount (`None` when the local one already is the lease's component) and the
/// checkpoint the lease authorizes — an app instance, or a seeded document with the canonical pair its
/// guest now holds (`None` for a document bound to no hub).
enum ShellDocumentOpenAnswer {
    #[cfg(not(target_arch = "wasm32"))]
    Resolved { plugin_id: String, app_id: String, program: Option<ProgramBridgeEntry>, checkpoint: semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1 },
    Instantiated(u32),
    Seeded(Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>),
}""")
swap("""/// 🎯️ The document half of an open: what the relay asked for.
struct ShellDocumentOpenTarget {
    document_id: String,
    schema: String,
}

/// 🧷️ An open whose host half is prepared — the prior document retired, the socket surface and
/// execution target bound — waiting for the guest to hold the document before its actor binds.
struct ShellPreparedDocumentOpen {
    document_id: String,
    schema: String,
    bindings: Vec<PersistenceBinding>,
    backbone_uri: Option<String>,
    hub_bound: bool,
    plugin: ProgramBridgeEntry,
    session: ActiveSession,
}""",
"""/// 🎯️ The document half of an open: what the relay asked for, and — once a hub resolution read the
/// document's lease — the checkpoint that lease authorizes.
struct ShellDocumentOpenTarget {
    document_id: String,
    schema: String,
    checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
}

/// 🪢️ What a hub document's seed step asks: the document's canonical checkpoint pair, admitted only as the
/// checkpoint its execution-target lease authorizes, over the shell's signed-in directory client.
#[derive(Clone)]
struct ShellHubDocumentSeed {
    client: std::sync::Arc<ShellDirectoryClient>,
    ctx: OperationContext,
    scope: semio_framework_os_kernel::os_directory::DocumentScope,
    checkpoint: semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1,
}

/// 🧷️ An open whose host half is prepared — the prior document retired, the socket surface and
/// execution target bound — waiting for the guest to hold the document before its actor binds.
struct ShellPreparedDocumentOpen {
    document_id: String,
    schema: String,
    bindings: Vec<PersistenceBinding>,
    backbone_uri: Option<String>,
    seed: Option<ShellHubDocumentSeed>,
    plugin: ProgramBridgeEntry,
    session: ActiveSession,
}""")
swap("""/// 🌱️ Loads the genesis the owning component mints for a hub document into the guest instance, off
/// the shell ([`store_sync::os_store::component_document_genesis`]); a document bound to no hub keeps
/// the guest's own document.
async fn seed_document_genesis(plugin: ProgramBridgeEntry, instance_id: u32, schema: String, document_id: String, hub_bound: bool) -> Result<(), String> {
    if !hub_bound {
        return Ok(());
    }
    match store_sync::os_store::component_document_genesis(&schema, &document_id).await.map_err(|error| format!("document genesis: {error}"))? {
        Some(genesis) => plugin.load_app_document_pack(instance_id, &genesis.pack, &genesis.spr).await.map_err(|error| format!("document genesis load: {error}")),
        None => Ok(()),
    }
}""",
"""/// 🪢️ Loads a hub document's canonical checkpoint pair into the guest instance, off the shell — the one
/// seed a native, a wasm32 and a React shell open a hub document on
/// ([`semio_framework_os_kernel::os_directory::client::DirectoryClient::document_canonical_checkpoint_pair`]):
/// fetched, its digests verified, admitted only as the checkpoint the open's lease authorizes, then loaded
/// before the document's actor exists, so every mutation the guest authors names the document and its
/// baseline is the hub's (a door artifact's genesis, a checked-in document's check-in). A document bound to
/// no hub keeps the guest's own document.
async fn seed_hub_document(plugin: ProgramBridgeEntry, instance_id: u32, seed: Option<ShellHubDocumentSeed>) -> Result<Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>, String> {
    let Some(seed) = seed else { return Ok(None) };
    let pair = seed.client.document_canonical_checkpoint_pair(&seed.ctx, &seed.scope, &seed.checkpoint).await.map_err(|error| format!("canonical checkpoint pair: {error}"))?;
    plugin.load_app_document_pack(instance_id, &pair.pack_bytes, &pair.spr_bytes).await.map_err(|error| format!("canonical checkpoint pair load: {error}"))?;
    Ok(Some(pair))
}""")
swap("""    /// 🌱️ A hub document's guest opens on the genesis its owning component mints for `document_id`
    /// ([`store_sync::os_store::component_document_genesis`], the hub's own creation baseline) before
    /// its document actor exists, so every mutation it authors names the document it opened and a later
    /// archive or tail from that actor lands on the same baseline (ticket 26/09/23 slice WG8, gate run 12).
    /// Only a hub binding names a server-minted artifact id; the component refuses a genesis for any other
    /// identity (`artifact genesis identity is not a server-minted artifact id`).
    async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<(), String> {
        let prepared = self.prepare_document_open(document_id, schema, bindings, surface, backbone_uri).await?;
        seed_document_genesis(prepared.plugin.clone(), prepared.session.instance_id, prepared.schema.clone(), prepared.document_id.clone(), prepared.hub_bound).await?;
        self.finish_document_open(prepared).await?;
        self.refresh_ui(UiDirtyScope::Full).await
    }""",
"""    /// 🪢️ A hub document's guest opens on the hub's canonical checkpoint pair ([`seed_hub_document`]) before its
    /// document actor exists, and the actor starts at the same pair ([`Self::install_document_seed`]), so every
    /// mutation it authors names the document it opened and its first Hello asks the hub only for what followed the
    /// pair (ticket 26/09/23: slice WG8 gate run 12 measured the identity-less guest; slice WG10 made the pair the
    /// one seed of every shell).
    async fn open_document(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<(), String> {
        let prepared = self.prepare_document_open(document_id, schema, bindings, surface, backbone_uri, None).await?;
        let pair = seed_hub_document(prepared.plugin.clone(), prepared.session.instance_id, prepared.seed.clone()).await?;
        self.install_document_seed(&prepared, pair)?;
        self.finish_document_open(prepared).await?;
        self.refresh_ui(UiDirtyScope::Full).await
    }

    /// 🪢️ Hands the document's actor the canonical pair its guest was seeded from; nothing for a document bound to
    /// no hub.
    fn install_document_seed(&self, prepared: &ShellPreparedDocumentOpen, pair: Option<semio_framework_os_kernel::os_directory::CanonicalCheckpointPairV1>) -> Result<(), String> {
        let (Some(pair), Some(seed)) = (pair, prepared.seed.as_ref()) else { return Ok(()) };
        if !self.document_host.set_document_seed(&ArtifactDocumentKey::hub(seed.scope.space_id.clone(), seed.scope.document_id.clone()), pair) {
            return Err("the canonical checkpoint pair could not be handed to the document actor".into());
        }
        Ok(())
    }""")
swap("""    async fn prepare_document_open(&mut self, document_id: String, schema: String, bindings: Vec<PersistenceBinding>, surface: Option<String>, backbone_uri: Option<String>) -> Result<ShellPreparedDocumentOpen, String> {""",
"""    async fn prepare_document_open(
        &mut self,
        document_id: String,
        schema: String,
        bindings: Vec<PersistenceBinding>,
        surface: Option<String>,
        backbone_uri: Option<String>,
        checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
    ) -> Result<ShellPreparedDocumentOpen, String> {""")
swap("""        self.bind_document_execution_target(&document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id).await?;
        let hub_bound = bindings.iter().any(|binding| matches!(binding, PersistenceBinding::Hub { .. }));
        Ok(ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, hub_bound, plugin, session })
    }""",
"""        let seed = self.bind_document_execution_target(&document_id, &schema, &bindings, &plugin, &session.app, &window_kind_id, checkpoint).await?;
        Ok(ShellPreparedDocumentOpen { document_id, schema, bindings, backbone_uri, seed, plugin, session })
    }""")
swap("""    /// 🪪️ The kind identity of a hub document whose schema this process resolves no codec for (neither
    /// linked nor a mounted component's, [`store_sync::os_store::document_kind_codec`]): the
    /// hub-selected execution target's lease for the exact scope and the surface this shell requests,
    /// bound onto the document host only when it names the package this shell mounted
    /// ([`document_execution_target_admitted`]). A kind with a linked codec, or a document with no hub
    /// binding, needs none. Both targets walk this one body.
    async fn bind_document_execution_target(&mut self, document_id: &str, schema: &str, bindings: &[PersistenceBinding], program: &ProgramBridgeEntry, app: &AppDefinition, window_kind_id: &str) -> Result<(), String> {
        let Some(space_id) = bindings.iter().find_map(|binding| match binding {
            PersistenceBinding::Hub { space_id, .. } => Some(space_id.clone()),
            PersistenceBinding::Folder { .. } => None,
        }) else {
            return Ok(());
        };
        if store_sync::os_store::document_kind_codec(schema).await.map_err(|error| format!("document codec registry: {error}"))?.is_some() {
            return Ok(());
        }
        let client = self.directory_client.clone().ok_or("document open requires a signed-in hub session")?;""",
"""    /// 🪪️ A hub document's execution-target lease for the exact scope and the surface this shell requests: its
    /// `checkpoint` is what the document's canonical pair must be ([`ShellHubDocumentSeed`]), and for a kind this
    /// process resolves no codec for (neither linked nor a mounted component's,
    /// [`store_sync::os_store::document_kind_codec`]) it is also the kind identity, bound onto the document host
    /// only when it names the package this shell mounted ([`document_execution_target_admitted`]). A hub
    /// resolution that already read the lease hands its `checkpoint` in, and a kind that resolves a codec then
    /// needs no second read. A document with no hub binding has no seed. Both targets walk this one body.
    #[allow(clippy::too_many_arguments)]
    async fn bind_document_execution_target(
        &mut self,
        document_id: &str,
        schema: &str,
        bindings: &[PersistenceBinding],
        program: &ProgramBridgeEntry,
        app: &AppDefinition,
        window_kind_id: &str,
        checkpoint: Option<semio_framework_os_kernel::os_directory::DocumentOpenCheckpointV1>,
    ) -> Result<Option<ShellHubDocumentSeed>, String> {
        let Some(space_id) = bindings.iter().find_map(|binding| match binding {
            PersistenceBinding::Hub { space_id, .. } => Some(space_id.clone()),
            PersistenceBinding::Folder { .. } => None,
        }) else {
            return Ok(None);
        };
        let client = self.directory_client.clone().ok_or("document open requires a signed-in hub session")?;
        let scope = semio_framework_os_kernel::os_directory::DocumentScope::new(space_id.as_str(), document_id);
        let codec_resolves = store_sync::os_store::document_kind_codec(schema).await.map_err(|error| format!("document codec registry: {error}"))?.is_some();
        if let (true, Some(checkpoint)) = (codec_resolves, checkpoint) {
            return Ok(Some(ShellHubDocumentSeed { client, ctx: self.directory_ctx(), scope, checkpoint }));
        }""")
swap("""        let lease = client.document_execution_target_manifest(&self.directory_command_ctx(), &intent).await.map_err(|error| format!("document execution target: {error}"))?;
        document_execution_target_admitted(&lease, &program.plugin_id, program.package_id.as_deref(), program.component_sha256.as_deref(), schema, &surface_id)?;
        if !self.document_host.set_document_execution_target_lease(&ArtifactDocumentKey::hub(space_id, document_id), lease) {
            return Err("document execution target is already bound for this document".into());
        }
        Ok(())
    }""",
"""        let lease = client.document_execution_target_manifest(&self.directory_command_ctx(), &intent).await.map_err(|error| format!("document execution target: {error}"))?;
        let checkpoint = lease.checkpoint.clone();
        if !codec_resolves {
            document_execution_target_admitted(&lease, &program.plugin_id, program.package_id.as_deref(), program.component_sha256.as_deref(), schema, &surface_id)?;
            if !self.document_host.set_document_execution_target_lease(&ArtifactDocumentKey::hub(space_id, document_id), lease) {
                return Err("document execution target is already bound for this document".into());
            }
        }
        Ok(Some(ShellHubDocumentSeed { client, ctx: self.directory_ctx(), scope, checkpoint }))
    }""")
swap("""            (Some(document_id), Some(schema)) => Some(ShellDocumentOpenTarget { document_id, schema }),""",
     """            (Some(document_id), Some(schema)) => Some(ShellDocumentOpenTarget { document_id, schema, checkpoint: None }),""")
swap("""        let prepared = match self.prepare_document_open(document.document_id, document.schema, bindings, surface, None).await {""",
     """        let prepared = match self.prepare_document_open(document.document_id, document.schema, bindings, surface, None, document.checkpoint).await {""")
swap("""        let pending = ShellDetached::spawn({
            let (plugin, instance_id, schema, document_id, hub_bound) = (prepared.plugin.clone(), prepared.session.instance_id, prepared.schema.clone(), prepared.document_id.clone(), prepared.hub_bound);
            async move { seed_document_genesis(plugin, instance_id, schema, document_id, hub_bound).await.map(|()| ShellDocumentOpenAnswer::Seeded) }
        });
        let mut opening = opening.unwrap_or_else(|| ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Seeding, session.plugin_id.clone(), session.app.id.clone(), None, None));
        opening.phase = ShellDocumentOpenPhase::Seeding;
        opening.pending = Some(pending);""",
"""        let seed_cancel = prepared.seed.as_ref().map(|seed| seed.ctx.cancel.clone());
        let pending = ShellDetached::spawn({
            let (plugin, instance_id, seed) = (prepared.plugin.clone(), prepared.session.instance_id, prepared.seed.clone());
            async move { seed_hub_document(plugin, instance_id, seed).await.map(ShellDocumentOpenAnswer::Seeded) }
        });
        let mut opening = opening.unwrap_or_else(|| ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Seeding, session.plugin_id.clone(), session.app.id.clone(), None, None));
        opening.phase = ShellDocumentOpenPhase::Seeding;
        opening.cancel = seed_cancel;
        opening.pending = Some(pending);""")
swap("""            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program })
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Resolving, target.plugin_id.clone().unwrap_or_default(), target.app_id.clone().unwrap_or_default(), Some(ShellDocumentOpenTarget { document_id, schema }), Some(pending));""",
"""            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program, checkpoint: resolved.lease.checkpoint.clone() })
        });
        let mut opening = ShellDocumentOpening::new(label, ShellDocumentOpenPhase::Resolving, target.plugin_id.clone().unwrap_or_default(), target.app_id.clone().unwrap_or_default(), Some(ShellDocumentOpenTarget { document_id, schema, checkpoint: None }), Some(pending));""")
swap("""            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program }) => {
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                opening.plugin_id = plugin_id;
                opening.app_id = app_id;""",
"""            Ok(ShellDocumentOpenAnswer::Resolved { plugin_id, app_id, program, checkpoint }) => {
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                opening.plugin_id = plugin_id;
                opening.app_id = app_id;
                if let Some(document) = opening.document.as_mut() {
                    document.checkpoint = Some(checkpoint);
                }""")
swap("""            Ok(ShellDocumentOpenAnswer::Seeded) => {
                let Some(prepared) = opening.prepared.take() else {
                    self.document_opening = Some(opening);
                    self.fail_document_opening("seeded document open lost its prepared owner".to_string());
                    return true;
                };
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                match self.finish_document_open(prepared).await {""",
"""            Ok(ShellDocumentOpenAnswer::Seeded(pair)) => {
                let Some(prepared) = opening.prepared.take() else {
                    self.document_opening = Some(opening);
                    self.fail_document_opening("seeded document open lost its prepared owner".to_string());
                    return true;
                };
                if opening.cancel_requested {
                    opening.phase = ShellDocumentOpenPhase::Cancelled;
                    self.document_opening = Some(opening);
                    return true;
                }
                if let Err(error) = self.install_document_seed(&prepared, pair) {
                    self.document_opening = Some(opening);
                    self.fail_document_opening(error);
                    return true;
                }
                match self.finish_document_open(prepared).await {""")
S.write_text(text)
print("shell edited")
