//! 🧪 Named mutation test for `change-envelope-volume`.

use crate::mutations::change_envelope_volume;
use crate::{Din16798Mutation, Din16798Snapshot};
use protocol::MutationDiff;

#[semio_framework_async_macros::async_test]
async fn applies_change_envelope_volume() {
    let base = Din16798Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(&mutation, &base);
    assert_eq!(outcome.worst_level(), None, "change-envelope-volume should apply cleanly on the default subject");
    let after = MutationDiff::apply(outcome.diff(), &base).expect("applies");
    assert_ne!(after, base, "change-envelope-volume must change the snapshot");
    let inverse = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::inverse(&mutation, &base);
    let mut restored = after;
    for step in &inverse {
        let undo = <Din16798Mutation as protocol::Mutation<Din16798Snapshot>>::diff(step, &restored);
        restored = MutationDiff::apply(undo.diff(), &restored).expect("inverse applies");
    }
    assert_eq!(restored, base, "change-envelope-volume inverse restores the base snapshot");
}

fn sample_mutation(base: &Din16798Snapshot) -> Din16798Mutation {
    Din16798Mutation::ChangeEnvelopeVolume(change_envelope_volume::ChangeEnvelopeVolume { new_envelope_volume_m3: base.envelope_volume_m3 + 1.0 })
}
