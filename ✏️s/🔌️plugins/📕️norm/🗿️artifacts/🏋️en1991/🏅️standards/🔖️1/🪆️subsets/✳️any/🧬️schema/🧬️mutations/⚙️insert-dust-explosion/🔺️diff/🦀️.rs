//! ⚙ `insert-dust-explosion` diff: inserts the dust explosion row; an id the list holds is a `mutation.duplicate-id`.

use crate::insert_dust_explosion::InsertDustExplosion;
use semio_s_artifact_norm_en1991_diff::{En1991Diff, En1991DustExplosionDelta};
use semio_s_artifact_norm_en1991_model::En1991Snapshot;

pub fn diff(payload: &InsertDustExplosion, base: &En1991Snapshot) -> protocol::MutationOutcome<En1991Diff> {
    if base.dust_explosions.iter().any(|existing| existing.id == payload.item.id) {
        let key = payload.item.id.clone();
        return protocol::MutationOutcome::fatal("mutation.duplicate-id", format!("Dust explosion {key} already exists."), [key]);
    }
    if payload.index > base.dust_explosions.len() {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Insert index {} is past the end ({} rows).", payload.index, base.dust_explosions.len()), Vec::<String>::new());
    }
    protocol::MutationOutcome::new(En1991Diff { dust_explosions: En1991DustExplosionDelta::insertion(payload.index, payload.item.clone()), ..Default::default() })
}
