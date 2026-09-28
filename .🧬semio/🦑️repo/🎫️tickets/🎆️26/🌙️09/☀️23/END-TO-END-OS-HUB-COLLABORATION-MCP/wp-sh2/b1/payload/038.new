//! 💾 S Home launcher app command — `persist-locally`: keeps an ephemeral local-only studio in a folder on disk.
//!
//! Without a folder the command opens its dialog. With one it is IO-owning retained work in two stages (`HomeCatalogWork`,
//! editor): [`validate`] resolves the studio and the folder without writing anything, [`commit`] opens the folder backbone,
//! writes the studio's event-sourced document into it, admits the folder-backed studio into the local catalog under its own
//! id and name, retires the ephemeral draft and publishes `change-catalog-generation`. The job checkpoints between the
//! stages, so a cancellation before [`commit`] writes nothing, and every refusal is named (`s.home.persist-locally.*`). On a
//! guest without a filesystem (every `wasm32` shell) the commit asks the host's local document catalog to keep the studio
//! in the folder instead (`os.local-catalog.admit`, storage `folder`) and the host hands the kept studio back.
//! Share/collaboration stay blocked; hub promotion is a separate command.
//! @see ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs `HomeCatalogWork`

use crate::editor::home::config::{local_studio_id_is_admissible, HomeConfig, HomeConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "persist-locally")]
pub struct PersistLocally {
    pub space_id: String,
    pub folder_path: Option<String>,
}

/// 🗨️ The folder dialog a folder-less persist opens, seeded with the studio id.
pub fn folder_dialog(space_id: &str) -> Emit<SHomeMutation, HomeConfigMutation> {
    let args = Some(pack::json_to_dsl_value(&pack::json!({ "spaceId": space_id })));
    Emit::effect(Effect::OpenDialog { req: semio_framework_plugin::RequestId(127), dialog_id: "persistLocally".into(), args })
}

pub fn handle(payload: &PersistLocally, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    match payload.folder_path.as_deref().map(str::trim) {
        None | Some("") => Ok(folder_dialog(&payload.space_id)),
        Some(_) => Err(Fault::new(FaultOrigin::App, "s.home.persist-locally.requires-retained-job", "persisting a studio writes the filesystem and the local catalog and runs only as the retained persist job")),
    }
}

/// 🔎️ Stage one — the ephemeral studio must exist and the folder must be named on a host with a filesystem; writes nothing.
pub fn validate(space_id: &str, folder_path: &str) -> Result<(), Fault> {
    if !local_studio_id_is_admissible(space_id) {
        return Err(Fault::new(FaultOrigin::App, "s.home.persist-locally.studio-invalid", "the studio id is empty, oversized or carries control characters"));
    }
    if folder_path.trim().is_empty() || folder_path.chars().any(char::is_control) {
        return Err(Fault::new(FaultOrigin::App, "s.home.persist-locally.path-invalid", "the folder path is empty or carries control characters"));
    }
    semio_framework_plugin::resolve_ready(crate::resolve_studio_document(space_id))
        .map(|_| ())
        .ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.persist-locally.unknown-studio", format!("no local studio {space_id} exists")))
}

/// 💾️ Stage two — the one folder + catalog write, the draft retirement, then the catalog generation bump.
#[cfg(not(target_arch = "wasm32"))]
pub fn commit(space_id: &str, folder_path: &str, doc: &ArtifactView<'_, SHomeSnapshot>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    use semio_framework_os::{document_backbone_ref, encode_backbone_payload, OsBackbonePort as _};
    let refused = |error: semio_framework_os::VcsError| Fault::new(FaultOrigin::App, "s.home.persist-locally.io-failed", format!("persisting {space_id} into {folder_path} failed: {error:?}"));
    let uri = format!("folder://{folder_path}");
    let port = semio_framework_os::open_folder_space_backbone(folder_path).map_err(refused)?;
    let mut document = semio_framework_plugin::resolve_ready(crate::resolve_studio_document(space_id)).ok_or_else(|| Fault::new(FaultOrigin::App, "s.home.persist-locally.unknown-studio", format!("no local studio {space_id} exists")))?;
    document.backbone = Some(semio_framework_plugin::resolve_ready(document_backbone_ref(&uri)));
    port.write(&uri, &encode_backbone_payload(&document).map_err(refused)?).map_err(refused)?;
    semio_framework_plugin::resolve_ready(crate::register_studio_port(space_id, port));
    semio_framework_os::host::admit_os_space_document(document, &semio_framework_plugin::resolve_ready(crate::catalog_port())).map_err(refused)?;
    let draft_port = semio_framework_plugin::resolve_ready(crate::draft_backbone_port());
    semio_framework_plugin::resolve_ready(crate::ephemeral_draft_catalog()).discard_draft(&draft_port, space_id);
    Ok(Emit::mutations(vec![crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation(doc.snapshot.catalog_generation + 1)]))
}

/// 📮️ Stage two on a guest without a filesystem — the host keeps the studio in the folder: one `os.local-catalog.admit`
/// request with the studio's own pack/spr pair; the host writes it, records it in its local catalog and hands it back
/// through `applyLocalCatalogDocument`, which lists it as persisted and retires the draft.
#[cfg(target_arch = "wasm32")]
pub fn commit(space_id: &str, folder_path: &str, _doc: &ArtifactView<'_, SHomeSnapshot>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Ok(Emit::effect(super::apply_local_catalog_document::keep_on_device(space_id, "folder", folder_path)?))
}

#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
