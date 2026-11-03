//! 🔗 `change-effect-influence` diff: replaces the addressed effect with the changed copy.

use crate::change_effect_influence::ChangeEffectInfluence;
use semio_s_artifact_norm_en1990_diff::En1990RowEdit as _;
use semio_s_artifact_norm_en1990_diff::{En1990Diff, En1990EffectEdit};
use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn diff(payload: &ChangeEffectInfluence, base: &En1990Snapshot) -> protocol::MutationOutcome<En1990Diff> {
    if !payload.new_influence.is_finite() {
        return protocol::MutationOutcome::fatal("mutation.invariant", "The value must be finite.", Vec::<String>::new());
    }
    let Some(row) = base.effects.get(payload.index) else {
        return protocol::MutationOutcome::error("mutation.target-missing", format!("Member effect {} does not exist.", payload.index), [payload.index.to_string()]);
    };
    if row.influence == payload.new_influence {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "The member effect already has this value.");
    }
    let mut next = row.clone();
    next.influence = payload.new_influence;
    protocol::MutationOutcome::new(En1990Diff { effects: vec![En1990EffectEdit::replace(payload.index, String::new(), next)], ..Default::default() })
}
