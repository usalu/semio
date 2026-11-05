//! 🏷️ EN 1997 field descriptions and native setter admission.

use semio_s_plugin_norm_app_surface::FieldSpec;

/// 🛡️ Descriptive field metadata admitted by the family's actual native setter vocabulary.
pub fn en1997_field_meta(path: &str) -> Option<FieldSpec> {
    semio_s_plugin_norm_app_surface::admit_list_meta(
        semio_s_plugin_norm_app_surface::admit_field_meta(
        semio_s_artifact_norm_en1997_checks::lookup_field_meta(path),
        semio_s_artifact_norm_en1997_mutations::EDIT_RULES.can_set_field(path),
        ),
        semio_s_artifact_norm_en1997_mutations::EDIT_RULES.can_insert_item(path),
        semio_s_artifact_norm_en1997_mutations::EDIT_RULES.can_remove_item(path, semio_s_artifact_norm_en1997_mutations::ID_REMOVALS.iter().map(|rule| rule.path)),
    )
}
