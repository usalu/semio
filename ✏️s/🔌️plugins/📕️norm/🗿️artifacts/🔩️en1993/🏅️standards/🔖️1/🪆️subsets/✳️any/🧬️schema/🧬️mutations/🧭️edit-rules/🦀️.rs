//! 🧭️ The leaf edits of the EN1993 editor: the typed rule table is the crate `semio-s-artifact-norm-en1993-mutations-edit-rules`. See [`semio_s_artifact_norm_en1993_mutations_edit_rules`].

pub use semio_s_artifact_norm_en1993_mutations_edit_rules::EDIT_RULES;

//#region 🧭️LeafEdits
use semio_framework_value::DslValue;
use semio_s_artifact_norm_en1993_model::{rolled_heb_catalogue, En1993Snapshot};
use semio_s_plugin_norm_app_surface::NormEdit;

use crate::{insert_section, En1993Mutation};

fn choose_section(document: &En1993Snapshot, member_path: &str, height: f64) -> Result<Vec<En1993Mutation>, String> {
    let section = rolled_heb_catalogue().into_iter().min_by(|a, b| (a.h - height).abs().total_cmp(&(b.h - height).abs())).ok_or("empty section catalogue")?;
    let mut out = Vec::new();
    if !document.sections.iter().any(|s| s.id == section.id) {
        out.push(En1993Mutation::InsertSection(insert_section::InsertSection { index: None, section: section.clone() }));
    }
    out.extend(EDIT_RULES.resolve(document, &NormEdit::SetField { path: member_path.to_string(), value: DslValue::String(section.id) })?);
    Ok(out)
}

/// 🧭️ Resolves every edit of the EN 1993 editor through the typed [`EDIT_RULES`]: one kind per leaf, no whole-row fallback;
/// a numeric write to `members[…].sectionId` is a section-height pick from the HEB catalogue and inserts the section when the document lacks it;
/// the enum leaves (`platedPanels[…].stiffenerShape`, `members[…].warpingSupport`) take their token directly.
pub fn resolve(document: &En1993Snapshot, edit: &NormEdit) -> Result<Vec<En1993Mutation>, String> {
    if let NormEdit::SetField { path, value: DslValue::Number(height) } = edit {
        if path.starts_with("members[") && path.ends_with(".sectionId") {
            return choose_section(document, path, height.as_f64());
        }
    }
    EDIT_RULES.resolve(document, edit)
}
//#endregion 🧭️LeafEdits
