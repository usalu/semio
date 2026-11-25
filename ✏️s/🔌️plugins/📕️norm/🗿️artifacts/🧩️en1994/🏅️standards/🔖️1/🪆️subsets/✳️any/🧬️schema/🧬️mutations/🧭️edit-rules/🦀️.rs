//! 🧭️ The leaf edits of the EN1994 editor: the typed rule table is the crate `semio-s-artifact-norm-en1994-mutations-edit-rules`. See [`semio_s_artifact_norm_en1994_mutations_edit_rules`].

pub use semio_s_artifact_norm_en1994_mutations_edit_rules::{EDIT_RULES, ID_REMOVALS};

use semio_framework_value::{DslValue, ToValue};
use semio_s_plugin_norm_app_surface::{NormEdit, coerce_like, get_value_at_path};
use crate::En1994Mutation;
use semio_s_artifact_norm_en1994_model::En1994Snapshot;

/// 🪢 A row list whose rows are owned by the rows of another list through a key: removing the owner removes its rows in the same batch.
pub struct OwnedRows {
    pub owner: &'static str,
    pub rows: &'static str,
    pub key: &'static str,
}

/// 🪢 The stud rows belong to their beam (`beamId`), see `remove-beam`.
pub const OWNED_ROWS: &[OwnedRows] = &[OwnedRows { owner: "beams", rows: "studs", key: "beamId" }];

const CATALOGUE: [&str; 11] = ["HEB300", "HEB320", "HEB340", "HEB360", "HEB400", "HEB450", "HEB500", "HEB550", "HEB600", "HEB650", "HEB700"];

fn catalogue_designation(height_m: f64) -> Option<&'static str> {
    CATALOGUE.iter().copied().filter_map(|designation| semio_s_artifact_norm_en1994_model::SteelSection::from_catalogue(designation).map(|section| (designation, section.height_m))).min_by(|a, b| (a.1 - height_m).abs().total_cmp(&(b.1 - height_m).abs())).map(|(designation, _)| designation)
}

/// 🔀️ Conforms a handle or panel write to the leaf it addresses: a number written to a flag becomes a flag, a count is rounded, and a number written to `steel.designation` is the catalogue pick by section height.
fn conform(document: &En1994Snapshot, path: &str, value: &DslValue) -> DslValue {
    let tree = document.to_value();
    match (get_value_at_path(&tree, path), value) {
        (Ok(DslValue::Bool(_)), DslValue::Number(number)) => DslValue::Bool(number.as_f64() >= 0.5),
        (Ok(DslValue::String(_)), DslValue::Number(number)) if path.ends_with(".steel.designation") => catalogue_designation(number.as_f64()).map_or_else(|| value.clone(), |designation| DslValue::String(designation.to_string())),
        (Ok(current), _) => coerce_like(current, value),
        _ => value.clone(),
    }
}

/// 🎯️ Resolves an editor edit into the concrete per-leaf kind mutations it denotes through [`EDIT_RULES`]; every editable leaf owns one kind and a nested action removal is raised by [`ID_REMOVALS`].
pub fn resolve_edit(document: &En1994Snapshot, edit: &NormEdit) -> Result<Vec<En1994Mutation>, String> {
    match edit {
        NormEdit::SetField { path, value } => EDIT_RULES.resolve(document, &NormEdit::SetField { path: path.clone(), value: conform(document, path, value) }),
        _ => EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit),
    }
}
