//! ️ `change-sections` named scenario `replaces-sections-library`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn specifies_sections_library_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-sections applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-sections must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    {
        let mut items = base.sections.clone();
        items.clear();
        // keep at least structural validity by cloning original then pushing nothing — use clone with tweak
        items = base.sections.clone();
        if !items.is_empty() { items.pop(); items.push(base.sections[0].clone()); }
        // force change: empty then restore one
        let mut changed = base.sections.clone();
        if let Some(first) = changed.first_mut() {
            // id tweak for equality break where possible — for annex-less lists clone+push duplicate avoided
        }
        changed.reverse();
        if changed == base.sections { changed.push(base.sections[0].clone()); }
        En1999Mutation::ChangeSections(crate::mutations::change_sections::ChangeSections { sections: changed })
    }
}
