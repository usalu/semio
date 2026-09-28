use super::*;
use crate::mutations::LayoutMutation;
use protocol::{Mutation, MutationDiff};

#[test]
fn update_spread_renames_and_inverse_restores_it() {
    let base = crate::standards::v1::subsets::any::schema::default_document();
    let mutation = LayoutMutation::UpdateSpread(UpdateSpread { id: "spread-1".into(), name: "Opening".into() });
    let next = mutation.diff(&base).diff().apply(&base).expect("spread applies");
    assert_eq!(next.spreads[0].name, "Opening");
    let restored = mutation.inverse(&base)[0].diff(&next).diff().apply(&next).expect("inverse");
    assert_eq!(restored.spreads[0].name, "Spread 1");
}
