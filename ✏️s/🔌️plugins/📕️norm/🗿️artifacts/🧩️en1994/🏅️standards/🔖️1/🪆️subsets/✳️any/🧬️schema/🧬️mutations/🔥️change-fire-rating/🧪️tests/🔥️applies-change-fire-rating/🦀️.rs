use crate::artifact_schema::mutations::change_fire_rating::ChangeFireRating;
use crate::{En1994Mutation, En1994Snapshot};
use protocol::Mutation;

#[test]
fn applies_change_fire_rating() {
    let base = En1994Snapshot::default();
    let op = En1994Mutation::ChangeFireRating(ChangeFireRating { new_fire_rating: "r90".into() });
    let (next, _) = protocol::apply_mutation(&base, &op).expect("apply");
    let _ = next;
}
