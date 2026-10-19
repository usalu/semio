//! ↩️ Inverse of `DeleteZone`: one concrete row per removed record and removed property or classification (the shared cascade, the zone created
//! first when the store replays the vector reversed) and, in front of them so they replay after the zone exists, one `SetSpace` per former member
//! that gives the space its zone back.

use semio_s_artifact_bim_model_mut_any_4::standards::v1::subsets::any::schema::mutations::set_space::SetSpace;
use crate::standards::v1::subsets::any::schema::mutations::delete_zone::DeleteZone;
use semio_s_artifact_bim_model_authored::standards::v1::subsets::any::schema::diff::algebra::Assigned;
use semio_s_artifact_bim_model_tests_2::standards::v1::subsets::any::schema::diff::patches::site_and_spaces::SpacePatch;
use semio_s_artifact_bim_model_snapshot::standards::v1::subsets::any::schema::snapshot::ModelSnapshot;

pub fn inverse<Op>(payload: &DeleteZone, base: &ModelSnapshot) -> Vec<Op>
where
    Op: semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::RemovalInverse + From<semio_s_artifact_bim_model_mut_any_4::standards::v1::subsets::any::schema::mutations::set_space::SetSpace>,
{
    let removal = match semio_s_artifact_bim_model_diff::standards::v1::subsets::any::schema::mutations::cascade::closure(base, std::slice::from_ref(&payload.id)) {
        Ok(removal) if base.zones.contains_key(&payload.id) && !base.area_schemes.values().any(|row| row.zones.contains(&payload.id)) => removal,
        _ => return Vec::new(),
    };
    let belonging = semio_s_artifact_bim_model_ramp_runs::standards::v1::subsets::any::schema::mutations::elements::zone_members(base, &payload.id).into_iter().map(|space| Op::from(SetSpace::from_patch(space, SpacePatch { zone: Some(Assigned::new(Some(payload.id.clone()))), ..Default::default() })));
    belonging.chain(removal.inverse(base)).collect()
}
