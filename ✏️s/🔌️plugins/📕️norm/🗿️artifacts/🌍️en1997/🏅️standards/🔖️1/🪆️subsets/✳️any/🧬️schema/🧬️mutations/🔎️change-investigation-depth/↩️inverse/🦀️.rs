use crate::change_investigation_depth::ChangeInvestigationDepth;

use semio_s_artifact_norm_en1997_model::En1997Snapshot;
pub fn inverse<Op>(_payload: &ChangeInvestigationDepth, base: &En1997Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_investigation_depth::ChangeInvestigationDepth> {
    Ok((|| {
    vec![Op::from(ChangeInvestigationDepth { new_investigation_depth: base.investigation_depth })]

    })())
}
