//! 🔗 `change-effects` diff — replaces the whole collection: every base row is removed back to front, then every new row is inserted in order.

use super::ChangeEffects;
use crate::diff::En1990RowEdit as _;
use crate::diff::{En1990Diff, En1990EffectEdit};
use crate::En1990Snapshot;
use protocol::MutationOutcome;

pub fn diff(mutation: &ChangeEffects, base: &En1990Snapshot) -> MutationOutcome<En1990Diff> {
    if base.effects == mutation.new_effects {
        return MutationOutcome::empty().warning("mutation.no-op", "effects already has this value.");
    }
    let removed = (0..base.effects.len()).rev().map(|index| En1990EffectEdit::remove(index, String::new()));
    let inserted = mutation.new_effects.iter().cloned().enumerate().map(|(index, row)| En1990EffectEdit::insert(index, row));
    MutationOutcome::new(En1990Diff { effects: removed.chain(inserted).collect(), ..En1990Diff::default() })
}
