//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Vdi3805 editor: which value-tree path raises which concrete kind.

use semio_framework_value::{DslValue, FromValue, ToValue};
use semio_s_plugin_norm_app_surface::{get_value_at_path, set_value_at_path, IdRemoval, InsertItemRule, NormEdit, NormEditRules, SelectorKey, SetFieldRule};
use crate::{change_curve_points::ChangeCurvePoints, change_geometry_parameters::ChangeGeometryParameters, resize_geometry::ResizeGeometry, Vdi3805Mutation};
use semio_s_artifact_norm_vdi3805_model::{CharacteristicCurve, ParametricGeometry, Vdi3805Snapshot};

/// 📚 Every path a Vdi3805 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field_arguments: &[],
    move_item: &[],
    set_field: &[
        SetFieldRule { path: "catalog.products[].configuration", kind: "change-product-configuration", selectors: &[SelectorKey::Id("id")], value: "newConfiguration" },
        SetFieldRule { path: "catalog.file", kind: "change-manufacturer-file", selectors: &[], value: "newManufacturerFile" },
        SetFieldRule { path: "catalog.products[].title", kind: "rename-product", selectors: &[SelectorKey::Id("id")], value: "newTitle" },
        SetFieldRule { path: "correctionAsOf", kind: "change-correction-as-of", selectors: &[], value: "newCorrectionAsOf" },
        SetFieldRule { path: "strictMode", kind: "change-strict-mode", selectors: &[], value: "newStrictMode" },
        SetFieldRule { path: "limits", kind: "change-limits", selectors: &[], value: "newLimits" },
    ],
    insert_item: &[
        InsertItemRule { path: "catalog.products", kind: "add-product", selectors: &[], index: "index", item: "product" },
    ],
    remove_item: &[

    ],
};

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[
        IdRemoval { path: "catalog.products", kind: "remove-product", selectors: &[], id: "id" },
];

/// 🗺️ The id of the map row a `geometry[id=…]` / `curves[id=…]` path addresses and the sub-path inside the row.
fn map_row(path: &str, collection: &str) -> Option<(String, String)> {
    let rest = path.strip_prefix(collection)?.strip_prefix("[id=")?;
    let (id, tail) = rest.split_once(']')?;
    Some((id.to_owned(), tail.trim_start_matches('.').to_owned()))
}

fn edited_row<T: FromValue>(document: &Vdi3805Snapshot, path: &str, value: &DslValue) -> Result<T, String> {
    let mut tree = document.to_value();
    set_value_at_path(&mut tree, path, value.clone())?;
    let row_path = &path[..=path.find(']').ok_or_else(|| format!("path '{path}' has no row selector"))?];
    T::from_value(get_value_at_path(&tree, row_path)?.clone()).map_err(|error| format!("{error:?}"))
}

fn resolve_map_edit(document: &Vdi3805Snapshot, path: &str, value: &DslValue) -> Option<Result<Vec<Vdi3805Mutation>, String>> {
    if let Some((id, leaf)) = map_row(path, "geometry") {
        let row = edited_row::<ParametricGeometry>(document, path, value);
        return Some(row.and_then(|row| match leaf.split('.').next() {
            Some("bbox") => Ok(vec![Vdi3805Mutation::ResizeGeometry(ResizeGeometry { id, new_bbox: row.bbox })]),
            Some("parameters") => Ok(vec![Vdi3805Mutation::ChangeGeometryParameters(ChangeGeometryParameters { id, new_parameters: row.parameters })]),
            _ => Err(format!("geometry leaf '{leaf}' has no editor setter")),
        }));
    }
    let (id, leaf) = map_row(path, "curves")?;
    Some(edited_row::<CharacteristicCurve>(document, path, value).and_then(|row| match leaf.split('.').next() {
        Some("points") => Ok(vec![Vdi3805Mutation::ChangeCurvePoints(ChangeCurvePoints { id, new_points: row.points })]),
        _ => Err(format!("curve leaf '{leaf}' has no editor setter")),
    }))
}

/// 🖊️ Whether `path` has an editor setter: a leaf inside a typed rule of [`EDIT_RULES`], a bounding-box or parameter leaf of a `geometry[id=…]` row or a point of a `curves[id=…]` row.
pub fn can_set_field(path: &str) -> bool {
    EDIT_RULES.can_set_field(path)
        || map_row(path, "geometry").is_some_and(|(_, leaf)| leaf.starts_with("bbox") || leaf.starts_with("parameters"))
        || map_row(path, "curves").is_some_and(|(_, leaf)| leaf.starts_with("points"))
}

/// 🎯️ Resolves an editor edit into concrete kind mutations: the `geometry[id=…]` and `curves[id=…]` rows of the id-keyed maps first, then row-id removals, then every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Vdi3805Snapshot, edit: &NormEdit) -> Result<Vec<Vdi3805Mutation>, String> {
    if let NormEdit::SetField { path, value } = edit {
        if let Some(resolved) = resolve_map_edit(document, path, value) {
            return resolved;
        }
    }
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)
}
