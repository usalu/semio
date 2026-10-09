//! ↩️ Inverse of `SplitBeam` in storage order, replayed reversed by the store: the exact base end offset of an inclined beam is restored first, then its exact base axis as one
//! `PlaceElements`, then the created beam is deleted. Empty when the split is refused.

use super::super::delete_beam::DeleteBeam;
use super::super::elements::Placement;
use super::super::place_elements::PlaceElements;
use super::super::set_beam::SetBeam;
use super::diff::middle;
use super::SplitBeam;
use crate::{Assigned, ModelMutation, ModelSnapshot};

pub fn inverse(payload: &SplitBeam, base: &ModelSnapshot) -> Vec<ModelMutation> {
    let Some(beam) = base.beams.get(&payload.id).filter(|beam| middle(beam, payload.t).is_some()) else {
        return Vec::new();
    };
    let mut rows = vec![
        ModelMutation::DeleteBeam(DeleteBeam { id: payload.new_id.clone() }),
        ModelMutation::PlaceElements(PlaceElements { placements: [(payload.id.clone(), Placement::Beam { axis: beam.axis.clone() })].into() }),
    ];
    if beam.end_top_offset.is_some() {
        rows.push(ModelMutation::SetBeam(SetBeam { id: payload.id.clone(), beam_type: None, top_offset: None, end_top_offset: Some(Assigned::new(beam.end_top_offset)), name: None }));
    }
    rows
}
