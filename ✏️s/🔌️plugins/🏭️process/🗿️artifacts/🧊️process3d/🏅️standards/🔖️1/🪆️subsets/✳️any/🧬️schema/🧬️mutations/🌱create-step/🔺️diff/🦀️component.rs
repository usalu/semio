//! 🔺️ `create-step` sparse diff construction.
//!
//! 🌉️ Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 4: `steps` composes an
//! `s.stdio.semio.flow` CHILD HANDLE now (see `ProcessWorkingScene`'s doc comment in the artifact
//! root file) — the individual step content this triad used to append lives only inside that
//! unresolved child, unreachable without a `LinkResolver` this ticket doesn't add. This is a
//! DOCUMENTED NO-OP pending that resolver, matching `📐️cad`'s own precedent for per-object
//! mutations on composed content (`addObject`/`patchObject`, ticket wave 3 round 2).

use crate::artifacts::process3d::diff::Process3dDiff;
use crate::artifacts::process3d::mutations::create_step::mutation::CreateStep;
use crate::artifacts::process3d::Process3dSnapshot;

//#region 🔖️Diff
/// 🚧️ Documented no-op — see file doc comment. The `steps` composed child is unresolved here, so
/// no create-family duplicate-id check is possible against real content; surfaced as Warning
/// `mutation.no-op` (the diff genuinely never changes anything) rather than silently succeeding.
pub async fn diff(_payload: &CreateStep, _base: &Process3dSnapshot) -> protocol::MutationOutcome<Process3dDiff> {
    protocol::MutationOutcome::empty().warn("mutation.no-op", "Step creation is a documented no-op pending a link resolver for the composed steps child.".to_string())
}
//#endregion 🔖️Diff
