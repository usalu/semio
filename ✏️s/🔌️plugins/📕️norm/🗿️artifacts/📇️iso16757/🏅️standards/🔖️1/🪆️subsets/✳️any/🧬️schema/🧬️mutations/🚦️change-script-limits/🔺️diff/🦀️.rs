//! 🔺️ `change-script-limits` — sparse diff construction.

use super::mutation::ChangeScriptLimits;
use crate::{part_5::ScriptLimits, Iso16757Snapshot};
use crate::diff::{Iso16757Diff, Iso16757ScriptLimitsPatch};

//#region 🔖️Diff

pub fn diff(payload: &ChangeScriptLimits, base: &Iso16757Snapshot) -> protocol::MutationOutcome<Iso16757Diff> {
    let limits = &base.script_limits;
    if limits.max_steps == payload.new_max_steps && limits.max_recursion == payload.new_max_recursion && limits.timeout_ms == payload.new_timeout_ms {
        return protocol::MutationOutcome::empty().warning("mutation.no-op", "Script limits already have these values.");
    }
    protocol::MutationOutcome::new(Iso16757Diff {
        script_limits: Some(Iso16757ScriptLimitsPatch {
            max_steps: (limits.max_steps != payload.new_max_steps).then_some(payload.new_max_steps),
            max_recursion: (limits.max_recursion != payload.new_max_recursion).then_some(payload.new_max_recursion),
            timeout_ms: (limits.timeout_ms != payload.new_timeout_ms).then_some(payload.new_timeout_ms),
        }),
        ..Default::default()
    })
}
