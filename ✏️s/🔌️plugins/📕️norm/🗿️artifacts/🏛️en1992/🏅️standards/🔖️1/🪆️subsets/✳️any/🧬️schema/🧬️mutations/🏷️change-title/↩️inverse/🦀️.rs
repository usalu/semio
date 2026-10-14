use crate::change_title::ChangeTitle;

use semio_s_artifact_norm_en1992_model::En1992Snapshot;

pub fn inverse<Op>(_payload: &ChangeTitle, base: &En1992Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_title::ChangeTitle> {
    Ok((|| {
    vec![Op::from(ChangeTitle { new_title: base.title.clone() })]

    })())
}
