//! 🧭️ The leaf edits of the EN1992 editor: the typed rule table is the crate `semio-s-artifact-norm-en1992-mutations-edit-rules`. See [`semio_s_artifact_norm_en1992_mutations_edit_rules`].

pub use semio_s_artifact_norm_en1992_mutations_edit_rules::{EDIT_RULES, ID_REMOVALS};

use semio_s_plugin_norm_app_surface::NormEdit;
use crate::En1992Mutation;
use semio_s_artifact_norm_en1992_model::En1992Snapshot;

/// 🎯️ Resolves an editor edit into concrete kind mutations: row-id removals by [`ID_REMOVALS`], every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &En1992Snapshot, edit: &NormEdit) -> Result<Vec<En1992Mutation>, String> {
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)
}
