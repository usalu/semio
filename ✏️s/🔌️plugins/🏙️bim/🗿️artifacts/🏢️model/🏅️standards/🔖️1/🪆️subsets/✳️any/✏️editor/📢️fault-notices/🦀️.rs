//! 📢️ Fault notices of the BIM editor: the localized fault notice table. See [`super`].

use super::*;

//#region 🔖️Faults
/// 📣️ Declares the en/de notices of every `bim.*` refusal code a command can raise: each code names the `BimLabels` field that holds its two texts.
macro_rules! fault_notices {
    ($($code:literal => $label:ident;)+) => {
        /// 📣️ The notice of every refusal code, English and German from the one `app_labels!` block.
        pub fn bim_fault_notices() -> &'static [(&'static str, LocalizedLabel)] {
            static NOTICES: std::sync::LazyLock<Vec<(&'static str, LocalizedLabel)>> =
                std::sync::LazyLock::new(|| vec![$(($code, BimLabels::localized(|labels| labels.$label))),+]);
            NOTICES.as_slice()
        }
    };
}

fault_notices! {
    "bim.create.kind-unknown" => fault_create_kind_unknown;
    "bim.create.unsupported" => fault_create_unsupported;
    "bim.create.site-missing" => fault_create_site_missing;
    "bim.create.building-missing" => fault_create_building_missing;
    "bim.create.storey-missing" => fault_create_storey_missing;
    "bim.create.wall-type-missing" => fault_create_wall_type_missing;
    "bim.clash.window-required" => fault_clash_window_required;
    "bim.clash.world-required" => fault_clash_world_required;
    "bim.clash.mode-unknown" => fault_clash_mode_unknown;
    "bim.clash.missing" => fault_clash_missing;
    "bim.clash.inference" => fault_clash_inference;
    "bim.issue.missing" => fault_issue_missing;
    "bim.issue.viewpoint-missing" => fault_issue_viewpoint_missing;
    "bim.create.issue-missing" => fault_create_issue_missing;
    "bim.delete.unsupported" => fault_delete_unsupported;
    "bim.rename.target-missing" => fault_rename_target_missing;
    "bim.rename.unsupported" => fault_rename_unsupported;
    "bim.rename.rejected" => fault_rename_rejected;
    "bim.set.target-missing" => fault_set_target_missing;
    "bim.set.field-unknown" => fault_set_field_unknown;
    "bim.set.read-only" => fault_set_read_only;
    "bim.set.value-invalid" => fault_set_value_invalid;
    "bim.view.window-required" => fault_view_window_required;
    "bim.view.window-unsupported" => fault_view_window_unsupported;
    "bim.view.field-unknown" => fault_view_field_unknown;
    "bim.view.value-invalid" => fault_view_value_invalid;
    "bim.view.storey-missing" => fault_view_storey_missing;
    "bim.view.view-missing" => fault_view_view_missing;
    "bim.view.kind-unknown" => fault_view_kind_unknown;
    "bim.view.building-missing" => fault_view_building_missing;
    "bim.camera.window-required" => fault_camera_window_required;
    "bim.camera.pose-mismatch" => fault_camera_pose_mismatch;
    "bim.camera.invalid" => fault_camera_invalid;
    "bim.tool.storey-missing" => fault_tool_storey_missing;
    "bim.tool.type-missing" => fault_tool_type_missing;
    "bim.tool.rejected" => fault_tool_rejected;
    "bim.tool.input-invalid" => fault_tool_input_invalid;
    "bim.gesture.retained-route" => fault_gesture_retained_route;
    "bim.split.target-missing" => fault_split_target_missing;
    "bim.split.fraction-invalid" => fault_split_fraction_invalid;
    "bim.flip.wall-missing" => fault_flip_wall_missing;
    "bim.attach.wall-missing" => fault_attach_wall_missing;
    "bim.attach.target-missing" => fault_attach_target_missing;
    "bim.create.wall-missing" => fault_create_wall_missing;
    "bim.create.material-missing" => fault_create_material_missing;
    "bim.analyse.inference" => fault_analyse_inference;
    "bim.analyse.cancelled" => fault_analyse_cancelled;
    "bim.export.format-unknown" => fault_export_format_unknown;
    "bim.export.inference" => fault_export_inference;
    "bim.export.failed" => fault_export_failed;
    "bim.export.cancelled" => fault_export_cancelled;
    "bim.export.sheet-missing" => fault_export_sheet_missing;
    "bim.export.sheets-none" => fault_export_sheets_none;
    "bim.create.sheet-missing" => fault_create_sheet_missing;
    "bim.create.view-missing" => fault_create_view_missing;
    "bim.storey.target-missing" => fault_storey_target_missing;
    "bim.storey.no-neighbour" => fault_storey_no_neighbour;
    "bim.property.target-missing" => fault_property_target_missing;
    "bim.property.name-invalid" => fault_property_name_invalid;
    "bim.property.value-invalid" => fault_property_value_invalid;
    "bim.property.missing" => fault_property_missing;
    "bim.place.target-missing" => fault_place_target_missing;
    "bim.place.point-invalid" => fault_place_point_invalid;
    "bim.place.unsupported" => fault_place_unsupported;
    "bim.diagnostic.target-missing" => fault_diagnostic_target_missing;
    "bim.classification.target-missing" => fault_classification_target_missing;
    "bim.classification.invalid" => fault_classification_invalid;
    "bim.classification.missing" => fault_classification_missing;
    "bim.classification.system-missing" => fault_classification_system_missing;
    "bim.classification.edit-invalid" => fault_classification_edit_invalid;
    "bim.classification.entry-has-children" => fault_classification_entry_has_children;
    "bim.classification.no-match" => fault_classification_no_match;
    "bim.template.target-missing" => fault_template_target_missing;
    "bim.template.not-applicable" => fault_template_not_applicable;
    "bim.conditions.source-missing" => fault_conditions_source_missing;
    "bim.conditions.target-missing" => fault_conditions_target_missing;
    "bim.template.edit-invalid" => fault_template_edit_invalid;
    "bim.template.value-invalid" => fault_template_value_invalid;
    "bim.create.family-missing" => fault_create_family_missing;
    "bim.family.missing" => fault_family_missing;
    "bim.family.part-unknown" => fault_family_part_unknown;
    "bim.family.operation-unknown" => fault_family_operation_unknown;
    "bim.family.formula-invalid" => fault_family_formula_invalid;
    "bim.family.kind-unknown" => fault_family_kind_unknown;
    "bim.family.category-unknown" => fault_family_category_unknown;
    "bim.family.shape-unknown" => fault_family_shape_unknown;
    "bim.family.material-missing" => fault_family_material_missing;
    "bim.family.material-unquotable" => fault_family_material_unquotable;
    "bim.family.solid-missing" => fault_family_solid_missing;
    "bim.family.slot-unknown" => fault_family_slot_unknown;
    "bim.family.axis-unknown" => fault_family_axis_unknown;
    "bim.family.axis-unavailable" => fault_family_axis_unavailable;
    "bim.create.component-family-missing" => fault_create_component_family_missing;
    "bim.place.family-unavailable" => fault_place_family_unavailable;
    "bim.override.component-missing" => fault_override_component_missing;
    "bim.override.parameter-missing" => fault_override_parameter_missing;
    "bim.browser.no-match" => fault_browser_no_match;
    "bim.tool.family-missing" => fault_tool_family_missing;
}
//#endregion 🔖️Faults
