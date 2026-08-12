//! 🔺️ Sparse diff builder for `ReplacePaneObjects` — removes every object currently in `pane` and
//! adds every object in the replacement list.
use super::mutation::ReplacePaneObjects;
use crate::artifacts::cad::diff::{CadDiff, CadObjectsDelta};
use crate::artifacts::cad::mutations::set_pane_objects_delta;
use crate::artifacts::cad::{cad_pane_objects, CadSnapshot};

//#region 🔖️Diff
pub fn diff(payload: &ReplacePaneObjects, base: &CadSnapshot) -> CadDiff {
    let mut diff = CadDiff::default();
    let removed: Vec<String> = cad_pane_objects(base, payload.pane).iter().map(|object| object.id.clone()).collect();
    let delta = CadObjectsDelta { removed, added: payload.objects.clone(), ..Default::default() };
    set_pane_objects_delta(&mut diff, payload.pane, delta);
    diff
}
//#endregion 🔖️Diff
