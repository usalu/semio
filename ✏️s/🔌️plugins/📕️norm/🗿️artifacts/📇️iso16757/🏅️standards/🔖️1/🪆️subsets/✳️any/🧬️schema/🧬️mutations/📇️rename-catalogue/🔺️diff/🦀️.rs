//! 🔺️ `rename-catalogue` — sparse diff construction.

use crate::rename_catalogue::mutation::RenameCatalogue;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;
use semio_s_artifact_norm_iso16757_diff::Iso16757Diff;

//#region 🔖️Diff

pub fn diff(payload: &RenameCatalogue, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    if base.catalogue.metadata.names.preferred.text == payload.new_name {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Catalogue already has that name.");
    }
    protocol::MutationOutcome::new(Iso16757Diff { catalogue_name: Some(payload.new_name.clone()), ..Default::default() })
}
