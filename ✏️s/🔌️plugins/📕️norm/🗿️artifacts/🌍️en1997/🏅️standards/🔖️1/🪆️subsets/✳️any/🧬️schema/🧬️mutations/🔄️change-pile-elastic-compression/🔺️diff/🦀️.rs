//! 🔺️ `change-pile-elastic-compression` diff: patches `elastic_compression` of the addressed pile row.

use crate::change_pile_elastic_compression::ChangePileElasticCompression;
use semio_s_artifact_norm_en1997_diff::{En1997Diff, En1997PilesPatch, En1997PilesRows};
use semio_s_artifact_norm_en1997_model::En1997Snapshot;

pub fn diff(payload: &ChangePileElasticCompression, base: &En1997Snapshot) -> protocol::MutationOutcome<En1997Diff> {
    if !payload.new_elastic_compression.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(r0) = base.piles.iter().find(|row| row.id == payload.id) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("pile {} missing", payload.id), vec![payload.id.clone()]);
    };
    if r0.elastic_compression == payload.new_elastic_compression {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The pile already has this value.");
    }
    protocol::MutationOutcome::new(En1997Diff { piles: Some(En1997PilesRows::modification(&payload.id, En1997PilesPatch { elastic_compression: Some(payload.new_elastic_compression), ..Default::default() })), ..Default::default() })
}
