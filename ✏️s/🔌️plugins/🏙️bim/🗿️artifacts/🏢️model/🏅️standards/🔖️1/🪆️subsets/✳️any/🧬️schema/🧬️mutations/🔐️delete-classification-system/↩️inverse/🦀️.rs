//! ↩️ Inverse of `DeleteClassificationSystem`: one concrete `SetElementClassification` per removed classification and, last in storage order so that the store (which replays the vector reversed) recreates the
//! system first, the `CreateClassificationSystem` carrying the full removed record; none when the system is absent or the cascade is refused.

use crate::standards::v1::subsets::any::schema::mutations::create_classification_system::CreateClassificationSystem;
use semio_s_artifact_bim_model_mut_any_88::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification;
use crate::standards::v1::subsets::any::schema::mutations::delete_classification_system::DeleteClassificationSystem;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteClassificationSystem, base: &ModelSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v1::subsets::any::schema::mutations::create_classification_system::CreateClassificationSystem> + From<semio_s_artifact_bim_model_mut_any_88::standards::v1::subsets::any::schema::mutations::set_element_classification::SetElementClassification>,
{
    let Some(system) = base.classification_systems.get(&payload.id) else {
        return Vec::new();
    };
    let holders = semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::classified_by(base, &payload.id);
    if holders.len() > semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::INVERSE_ROWS {
        return Vec::new();
    }
    holders
        .into_iter()
        .map(|(holder, code)| Op::from(SetElementClassification { id: holder, system: payload.id.clone(), code }))
        .chain(std::iter::once(Op::from(CreateClassificationSystem { id: payload.id.clone(), system: system.clone() })))
        .collect()
}
