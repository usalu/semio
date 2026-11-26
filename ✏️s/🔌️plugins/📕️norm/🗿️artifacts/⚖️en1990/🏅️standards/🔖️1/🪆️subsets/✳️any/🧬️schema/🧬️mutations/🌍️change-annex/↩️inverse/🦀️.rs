//! ↩️ `change-annex` inverse.

use crate::change_annex::ChangeAnnex;

use semio_s_artifact_norm_en1990_model::En1990Snapshot;

pub fn inverse<Op>(mutation: &ChangeAnnex, base: &En1990Snapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> where Op: From<crate::change_annex::ChangeAnnex> {
    Ok(match previous(mutation, base) {
        Some(inverse) => vec![Op::from(inverse)],
        None => Vec::new(),
    })
}

fn previous(_mutation: &ChangeAnnex, base: &En1990Snapshot) -> Option<crate::change_annex::ChangeAnnex> {
    Some(ChangeAnnex { new_annex: base.annex })
}
