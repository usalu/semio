//! 🗃️ S Home launcher app command — `apply-local-catalog-document`: the host hands back one studio it keeps on this device.
//!
//! A guest has no filesystem, so the studios this device keeps (persisted into a folder, bound to a file, or imported) live
//! in the host's local document catalog (`os.config.local-catalog`, persisted local-only) and in their own folder lane. On
//! every Home mount the host re-hydrates the guest with each kept studio's own pack/spr pair through this chrome-audience
//! view action. It is IO-owning retained work in two stages (`HomeCatalogWork`, editor): [`validate`] decodes the pair and
//! checks it is the named studio's event-sourced document, writing nothing; [`commit`] admits it into the local catalog
//! under its own id and manifest name (idempotent: a repeated page re-lists nothing twice) and publishes
//! `change-catalog-generation`. It never asks the host to keep the studio again — only the user's own persist, bind and
//! import commits do — so a re-hydration can never loop back into an admission. The admitted studio replaces the ephemeral
//! draft of the same id, if one is still listed: the host now keeps it.
//!
//! [`keep_on_device`] is the guest half of the lane: the request a persist, bind or import commit hands the host.
//! @see ✏️s/🔌️plugins/🪐️space/🗿️artifacts/🏠️home/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️.rs `HomeCatalogWork`
//! @see 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗂️local-catalog/🟦️.ts

use crate::editor::home::config::{local_studio_id_is_admissible, HomeConfig, HomeConfigMutation};
use crate::standards::v1::subsets::any::schema::mutations::change_catalog_generation;
use crate::standards::v1::subsets::any::schema::mutations::text::SHomeMutation;
use crate::SHomeSnapshot;
use semio_framework_plugin::{ArtifactView, ConfigView, Effect, Emit, Fault, FaultOrigin};

#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::DslRecord)]
#[dsl(keyword = "apply-local-catalog-document")]
pub struct ApplyLocalCatalogDocument {
    pub document_id: String,
    pub pack: String,
    pub spr: String,
}

pub fn handle(_payload: &ApplyLocalCatalogDocument, _doc: &ArtifactView<'_, SHomeSnapshot>, _cfg: &ConfigView<'_, HomeConfig>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    Err(Fault::new(FaultOrigin::App, "s.home.apply-local-catalog-document.requires-retained-job", "re-hydrating a kept studio writes the local catalog and runs only as the retained catalog job"))
}

fn decoded(payload: &ApplyLocalCatalogDocument) -> Result<(Vec<u8>, Vec<u8>), Fault> {
    let invalid = |part: &str| Fault::new(FaultOrigin::App, "s.home.apply-local-catalog-document.encoding-invalid", format!("the kept studio's {part} is not base64"));
    Ok((protocol::base64_standard_decode(&payload.pack).map_err(|_| invalid("pack"))?, protocol::base64_standard_decode(&payload.spr).map_err(|_| invalid("spr"))?))
}

/// 🔎️ Stage one — the pair must decode into the named studio's space document; writes nothing.
pub fn validate(payload: &ApplyLocalCatalogDocument) -> Result<(), Fault> {
    if !local_studio_id_is_admissible(&payload.document_id) {
        return Err(Fault::new(FaultOrigin::App, "s.home.apply-local-catalog-document.id-invalid", "the kept studio's id is empty, oversized or carries control characters"));
    }
    let (pack, spr) = decoded(payload)?;
    let parsed: store::ParsedDocumentText<semio_framework_artifact_space_space::SpaceSnapshot, semio_framework_artifact_space_space::SpaceMutation> = semio_framework_plugin::resolve_ready(store::parse_document_pack(&pack, &spr))
        .map_err(|error| Fault::new(FaultOrigin::App, "s.home.apply-local-catalog-document.not-a-studio", format!("the kept studio's pair is not a studio document: {error}")))?;
    let envelope = parsed.into_envelope();
    let named = envelope.schema == semio_framework_artifact_space_space::S_SPACE_SCHEMA && envelope.id == payload.document_id;
    envelope.retire_unadopted();
    if !named {
        return Err(Fault::new(FaultOrigin::App, "s.home.apply-local-catalog-document.mismatch", format!("the kept pair is not the studio document {}", payload.document_id)));
    }
    Ok(())
}

/// 💾️ Stage two — the one catalog write: admits the kept studio under its own id and name, then bumps the catalog generation.
pub fn commit(payload: &ApplyLocalCatalogDocument, doc: &ArtifactView<'_, SHomeSnapshot>) -> Result<Emit<SHomeMutation, HomeConfigMutation>, Fault> {
    let (pack, spr) = decoded(payload)?;
    semio_framework_os::host::import_os_space_from_pack(&pack, &spr, &semio_framework_plugin::resolve_ready(crate::catalog_port()))
        .map_err(|error| Fault::new(FaultOrigin::App, "s.home.apply-local-catalog-document.catalog-refused", format!("the local catalog refused the kept studio: {error:?}")))?;
    let draft_port = semio_framework_plugin::resolve_ready(crate::draft_backbone_port());
    semio_framework_plugin::resolve_ready(crate::ephemeral_draft_catalog()).discard_draft(&draft_port, &payload.document_id);
    Ok(Emit::mutations(vec![change_catalog_generation(doc.snapshot.catalog_generation + 1)]))
}

/// 📮️ Asks the host to keep one local studio on this device (`os.local-catalog.admit`) with the studio's own event-sourced
/// pack/spr pair: `storage` is `folder` or `file`, an empty `target` means the device's own data folder. The host writes the
/// studio into that lane, records it in its local catalog and hands it back through `applyLocalCatalogDocument`.
pub fn keep_on_device(space_id: &str, storage: &str, target: &str) -> Result<Effect, Fault> {
    let refused = |code: &'static str, message: String| Fault::new(FaultOrigin::App, code, message);
    let document = semio_framework_plugin::resolve_ready(crate::resolve_studio_document(space_id)).ok_or_else(|| refused("s.home.keep-on-device.unknown-studio", format!("no local studio {space_id} exists")))?;
    let files = semio_framework_os::export_backbone_pack(&document).map_err(|error| refused("s.home.keep-on-device.export-failed", format!("studio {space_id} could not be exported: {error:?}")))?;
    let name = semio_framework_os::materialize_backbone_snapshot(&document, &[]).map_err(|error| refused("s.home.keep-on-device.export-failed", format!("studio {space_id} could not be read: {error:?}")))?.name;
    let args = pack::json_to_dsl_value(&pack::json!({
        "documentId": space_id,
        "schema": semio_framework_artifact_space_space::S_SPACE_SCHEMA,
        "name": name.trim(),
        "storage": storage,
        "target": target.trim(),
        "pack": protocol::base64_standard_encode(&files.pack),
        "spr": protocol::base64_standard_encode(&files.spr),
    }));
    Ok(Effect::ReplayShellCommand { action_id: "os.local-catalog.admit".into(), args: Some(args) })
}
