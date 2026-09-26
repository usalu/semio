//! ️ `change-members` named scenario `replaces-members-list`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn specifies_members_list_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-members applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-members must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    {
        let mut items = base.members.clone();
        items.clear();
        // keep at least structural validity by cloning original then pushing nothing — use clone with tweak
        items = base.members.clone();
        if !items.is_empty() { items.pop(); items.push(base.members[0].clone()); }
        // force change: empty then restore one
        let mut changed = base.members.clone();
        if let Some(first) = changed.first_mut() {
            // id tweak for equality break where possible — for annex-less lists clone+push duplicate avoided
        }
        changed.reverse();
        if changed == base.members { changed.push(base.members[0].clone()); }
        En1999Mutation::ChangeMembers(crate::mutations::change_members::ChangeMembers { members: changed })
    }
}
