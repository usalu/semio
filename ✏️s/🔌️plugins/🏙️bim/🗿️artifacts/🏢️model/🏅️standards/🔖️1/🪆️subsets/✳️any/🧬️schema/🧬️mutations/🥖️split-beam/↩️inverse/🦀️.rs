//! ↩️ Inverse of `SplitBeam` in storage order, replayed reversed by the store: the exact base end offset of an inclined beam is restored first, then its exact base axis as one
//! `PlaceElements`, then the created beam is deleted. Empty when the split is refused.

use semio_s_artifact_bim_model_mut_any_67::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam;
use semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::Placement;
use semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements;
use semio_s_artifact_bim_model_mut_any_170::standards::v1::subsets::any::schema::mutations::set_beam::SetBeam;
use crate::standards::v1::subsets::any::schema::mutations::split_beam::diff::middle;
use crate::standards::v1::subsets::any::schema::mutations::split_beam::SplitBeam;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &SplitBeam, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<semio_s_artifact_bim_model_mut_any_67::standards::v1::subsets::any::schema::mutations::delete_beam::DeleteBeam> + From<semio_s_artifact_bim_model_mut_any_84::standards::v1::subsets::any::schema::mutations::place_elements::PlaceElements> + From<semio_s_artifact_bim_model_mut_any_170::standards::v1::subsets::any::schema::mutations::set_beam::SetBeam>,
{
    let Some(beam) = base.beams.get(&payload.id).filter(|beam| middle(beam, payload.t).is_some()) else {
        return Vec::new();
    };
    let mut rows = vec![
        Op::from(DeleteBeam { id: payload.new_id.clone() }),
        Op::from(PlaceElements { placements: [(payload.id.clone(), Placement::Beam { axis: beam.axis.clone() })].into() }),
    ];
    if beam.end_top_offset.is_some() {
        rows.push(Op::from(SetBeam { id: payload.id.clone(), beam_type: None, top_offset: None, end_top_offset: Some(Assigned::new(beam.end_top_offset)), name: None }));
    }
    rows
}
