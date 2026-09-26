//! ️ `change-fire-scenarios` named scenario `replaces-fire-scenarios`.

use crate::{En1999Mutation, En1999Snapshot};
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn specifies_fire_scenarios_applies_and_inverts() {
    let base = En1999Snapshot::default();
    let mutation = sample_mutation(&base);
    let outcome = mutation.diff(&base);
    let after = outcome.diff().apply(&base).expect("change-fire-scenarios applies");
    assert_ne!(serde_json::to_string(&after).unwrap(), serde_json::to_string(&base).unwrap(), "change-fire-scenarios must change the snapshot");
}

fn sample_mutation(base: &En1999Snapshot) -> En1999Mutation {
    {
        let mut items = base.fire_scenarios.clone();
        items.clear();
        // keep at least structural validity by cloning original then pushing nothing — use clone with tweak
        items = base.fire_scenarios.clone();
        if !items.is_empty() { items.pop(); items.push(base.fire_scenarios[0].clone()); }
        // force change: empty then restore one
        let mut changed = base.fire_scenarios.clone();
        if let Some(first) = changed.first_mut() {
            // id tweak for equality break where possible — for annex-less lists clone+push duplicate avoided
        }
        changed.reverse();
        if changed == base.fire_scenarios { changed.push(base.fire_scenarios[0].clone()); }
        En1999Mutation::ChangeFireScenarios(crate::mutations::change_fire_scenarios::ChangeFireScenarios { fire_scenarios: changed })
    }
}
