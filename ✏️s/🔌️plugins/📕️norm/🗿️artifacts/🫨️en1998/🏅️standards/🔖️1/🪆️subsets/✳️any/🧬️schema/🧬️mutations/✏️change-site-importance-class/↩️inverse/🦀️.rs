//! ↩️ Inverse for `change-site-importance-class`.
use crate::change_site_importance_class::ChangeSiteImportanceClass;
use semio_s_artifact_norm_en1998_model::En1998Snapshot;

pub fn inverse<Op>(payload: &ChangeSiteImportanceClass, base: &En1998Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_site_importance_class::ChangeSiteImportanceClass> {
    Ok(vec![Op::from(ChangeSiteImportanceClass { new_importance_class: base.site.importance_class })])
}
