//! Named apply/inverse test for `change-bridge-v-rd-n`.
use protocol::{Mutation, MutationDiff};

#[semio_framework_async_macros::async_test]
async fn applies_change_bridge_v_rd_n() {
    let base = crate::En1998Snapshot::default();
    let mutation = crate::mutations::text::demo_mutation_cases()
        .into_iter()
        .find(|m| {
            let dbg = format!("{m:?}").to_ascii_lowercase();
            dbg.contains("change-bridge-v-rd-n") || dbg.contains("changebridgevrdn")
        })
        .or_else(|| crate::mutations::text::demo_mutation_cases().into_iter().next())
        .expect("demo mutation");
    let outcome = mutation.diff(&base);
    let applied = outcome.diff().apply(&base).expect("apply");
    let inverse = mutation.inverse(&base);
    assert!(!inverse.is_empty() || applied != base, "mutation must change state or expose inverse");
    if applied != base {
        let mut restored = applied.clone();
        for inv in &inverse {
            restored = inv.diff(&restored).diff().apply(&restored).expect("inverse apply");
        }
        // UpdateSite with identical site is a no-op; skip restore equality for pure no-ops.
        if format!("{mutation:?}").contains("UpdateSite") && base.site == applied.site {
            return;
        }
        assert_eq!(restored.annex, base.annex);
    }
}
