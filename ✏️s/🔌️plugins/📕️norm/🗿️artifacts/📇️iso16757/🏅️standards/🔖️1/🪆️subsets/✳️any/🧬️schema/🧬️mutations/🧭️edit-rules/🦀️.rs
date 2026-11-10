//! 🧭️ The `setField` / `insertItem` / `removeItem` / `applyRemedy` vocabulary of the Iso16757 editor: which value-tree path raises which concrete kind.

use semio_s_plugin_norm_app_surface::{IdRemoval, InsertItemRule, NormEdit, NormEditRules, RemoveItemRule, SelectorKey, SetFieldArgument, SetFieldArgumentSource, SetFieldRule};
use crate::Iso16757Mutation;
use semio_s_artifact_norm_iso16757_model::Iso16757Snapshot;

/// 📚 Every path a Iso16757 editor edit resolves, one rule per kind that sets, inserts or removes through it.
pub const EDIT_RULES: NormEditRules = NormEditRules {
    set_field_arguments: &[
        SetFieldArgument { path: "scriptLimits.maxSteps", kind: "change-script-limits", argument: "newMaxRecursion", source: SetFieldArgumentSource::Sibling("maxRecursion") },
        SetFieldArgument { path: "scriptLimits.maxSteps", kind: "change-script-limits", argument: "newTimeoutMs", source: SetFieldArgumentSource::Sibling("timeoutMs") },
        SetFieldArgument { path: "scriptLimits.maxRecursion", kind: "change-script-limits", argument: "newMaxSteps", source: SetFieldArgumentSource::Sibling("maxSteps") },
        SetFieldArgument { path: "scriptLimits.maxRecursion", kind: "change-script-limits", argument: "newTimeoutMs", source: SetFieldArgumentSource::Sibling("timeoutMs") },
        SetFieldArgument { path: "scriptLimits.timeoutMs", kind: "change-script-limits", argument: "newMaxSteps", source: SetFieldArgumentSource::Sibling("maxSteps") },
        SetFieldArgument { path: "scriptLimits.timeoutMs", kind: "change-script-limits", argument: "newMaxRecursion", source: SetFieldArgumentSource::Sibling("maxRecursion") },
    ],
    move_item: &[],
    set_field: &[
        SetFieldRule { path: "selection.classId", kind: "change-selection-class", selectors: &[], value: "newClassId" },
        SetFieldRule { path: "catalogue.manufacturer.names.preferred.text", kind: "rename-manufacturer", selectors: &[], value: "newName" },
        SetFieldRule { path: "catalogue.products[].names.preferred.text", kind: "rename-product", selectors: &[SelectorKey::Id("id")], value: "newName" },
        SetFieldRule { path: "catalogue.metadata.names.preferred.text", kind: "rename-catalogue", selectors: &[], value: "newName" },
        SetFieldRule { path: "exchangeProcess", kind: "change-exchange-process", selectors: &[], value: "newExchangeProcess" },
        SetFieldRule { path: "catalogue.productGroups[].names.preferred.text", kind: "rename-product-group", selectors: &[SelectorKey::Id("id")], value: "newName" },
        SetFieldRule { path: "partNumberRule", kind: "replace-part-number-rule", selectors: &[], value: "newRule" },
        SetFieldRule { path: "selection.seriesId", kind: "change-selection-series", selectors: &[], value: "newSeriesId" },
        SetFieldRule { path: "scriptLimits.maxSteps", kind: "change-script-limits", selectors: &[], value: "newMaxSteps" },
        SetFieldRule { path: "scriptLimits.maxRecursion", kind: "change-script-limits", selectors: &[], value: "newMaxRecursion" },
        SetFieldRule { path: "scriptLimits.timeoutMs", kind: "change-script-limits", selectors: &[], value: "newTimeoutMs" },
    ],
    insert_item: &[
        InsertItemRule { path: "dictionary.subjects", kind: "introduce-subject", selectors: &[], index: "index", item: "subject" },
        InsertItemRule { path: "catalogue.productClasses", kind: "introduce-product-class", selectors: &[], index: "index", item: "productClass" },
        InsertItemRule { path: "catalogue.propertyDefinitions", kind: "introduce-property-definition", selectors: &[], index: "index", item: "propertyDefinition" },
        InsertItemRule { path: "catalogue.productSeries", kind: "introduce-product-series", selectors: &[], index: "index", item: "productSeries" },
        InsertItemRule { path: "catalogue.products", kind: "introduce-product", selectors: &[], index: "index", item: "product" },
        InsertItemRule { path: "catalogue.productIndexes", kind: "introduce-product-index", selectors: &[], index: "index", item: "productIndex" },
        InsertItemRule { path: "selection.constraints", kind: "add-selection-constraint", selectors: &[], index: "index", item: "constraint" },
        InsertItemRule { path: "catalogue.productGroups", kind: "introduce-product-group", selectors: &[], index: "index", item: "productGroup" },
    ],
    remove_item: &[
        RemoveItemRule { path: "selection.constraints", kind: "remove-selection-constraint", selectors: &[], index: "index" },
    ],
};

/// 🆔️ Every collection of this family removed by row id.
pub const ID_REMOVALS: &[IdRemoval] = &[
        IdRemoval { path: "dictionary.subjects", kind: "retire-subject", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productClasses", kind: "retire-product-class", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productIndexes", kind: "retire-product-index", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productSeries", kind: "retire-product-series", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.products", kind: "retire-product", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.productGroups", kind: "retire-product-group", selectors: &[], id: "id" },
        IdRemoval { path: "catalogue.propertyDefinitions", kind: "retire-property-definition", selectors: &[], id: "id" },
];

/// 🧱️ The concrete mutation of `kind` built from its payload value.
fn payload_of(kind: &str, payload: semio_framework_value::DslValue) -> Result<Iso16757Mutation, String> {
    <Iso16757Mutation as protocol::Mutation<Iso16757Snapshot>>::from_payload_value(kind, payload).map_err(|error| error.to_string())
}

/// 🧱️ The field-wise write of a whole facet: `leaf` of `facet` set to `value`, the facet value tree returned.
fn facet_with(facet: semio_framework_value::DslValue, leaf: &str, value: &semio_framework_value::DslValue) -> Result<semio_framework_value::DslValue, String> {
    let mut tree = facet;
    semio_s_plugin_norm_app_surface::set_value_at_path(&mut tree, leaf, value.clone())?;
    Ok(tree)
}

/// 🚦️ `scriptLimits.<leaf>` raises `change-script-limits` with the three limits validated together: the written leaf and the two siblings of the base document.
fn resolve_script_limits(document: &Iso16757Snapshot, leaf: &str, value: &semio_framework_value::DslValue) -> Result<Vec<Iso16757Mutation>, String> {
    use semio_framework_value::{DslValue, ToValue};
    let tree = facet_with(document.script_limits.to_value(), leaf, value)?;
    let field = |name: &str| tree.get(name).cloned().ok_or_else(|| format!("script limits lack {name}"));
    let payload = DslValue::object([("newMaxSteps".to_string(), field("maxSteps")?), ("newMaxRecursion".to_string(), field("maxRecursion")?), ("newTimeoutMs".to_string(), field("timeoutMs")?)]);
    payload_of("change-script-limits", payload).map(|mutation| vec![mutation])
}

/// 📐️ `geometry.objects.<id>.<leaf>` replaces the whole object as one atomic pair, retire then introduce, so every numeric leaf of a geometry object (space bounds, port positions, surface bounds) is one undoable edit.
fn resolve_geometry(document: &Iso16757Snapshot, rest: &str, value: &semio_framework_value::DslValue) -> Result<Vec<Iso16757Mutation>, String> {
    use semio_framework_value::{DslValue, FromValue, ToValue};
    let (id, leaf) = rest.split_once('.').ok_or_else(|| format!("geometry.objects.{rest} is not a leaf"))?;
    let object = document.geometry.objects.get(id).ok_or_else(|| format!("unknown geometry object '{id}'"))?;
    let replaced = facet_with(object.to_value(), leaf, value)?;
    if semio_s_artifact_norm_iso16757_model::part_2::GeometryObject::from_value(replaced.clone()).map_err(|error| error.to_string())? == *object {
        return Err(format!("'{id}.{leaf}' already has this value"));
    }
    let retire = payload_of("retire-geometry-object", DslValue::object([("id".to_string(), DslValue::String(id.to_string()))]))?;
    let introduce = payload_of("introduce-geometry-object", DslValue::object([("geometryObject".to_string(), replaced)]))?;
    Ok(vec![retire, introduce])
}

/// ➕ The geometry object a new row stands for: the placed copy of the last object (a document-valid starting point, id renewed by the insert tool) or, for an empty catalogue, a unit box of 0.1 m.
fn new_geometry_object(document: &Iso16757Snapshot, value: Option<&semio_framework_value::DslValue>) -> Result<semio_s_artifact_norm_iso16757_model::part_2::GeometryObject, String> {
    use semio_framework_value::FromValue;
    use semio_s_artifact_norm_iso16757_model::part_2::{GeometryNode, GeometryObject};
    match value {
        Some(value) => GeometryObject::from_value(value.clone()).map_err(|error| error.to_string()),
        None => {
            let id = (document.geometry.objects.len() + 1..).map(|n| format!("geometry-object-{n}")).find(|id| !document.geometry.objects.contains_key(id)).unwrap_or_default();
            let parameters = ["width", "depth", "height"].into_iter().map(|name| (name.to_string(), 0.1)).collect();
            Ok(GeometryObject { id, shape: Some(GeometryNode::Primitive { kind: "box".into(), parameters }), symbolic: None, spaces: Vec::new(), surfaces: Vec::new(), ports: Vec::new(), parameter_bindings: Default::default() })
        }
    }
}

/// ➕ `insertItem` into `geometry.objects` raises `introduce-geometry-object`; an id the catalogue already holds is refused.
fn resolve_geometry_insert(document: &Iso16757Snapshot, value: Option<&semio_framework_value::DslValue>) -> Result<Vec<Iso16757Mutation>, String> {
    use semio_framework_value::{DslValue, ToValue};
    let object = new_geometry_object(document, value)?;
    if object.id.trim().is_empty() || document.geometry.objects.contains_key(&object.id) {
        return Err(format!("geometry object id '{}' is empty or taken", object.id));
    }
    payload_of("introduce-geometry-object", DslValue::object([("geometryObject".to_string(), object.to_value())])).map(|mutation| vec![mutation])
}

/// ➖ `removeItem` of `geometry.objects.<id>` (or `geometry.objects[id=<id>]`) raises `retire-geometry-object`.
fn resolve_geometry_remove(document: &Iso16757Snapshot, path: &str) -> Result<Option<Vec<Iso16757Mutation>>, String> {
    use semio_framework_value::DslValue;
    let id = path.strip_prefix("geometry.objects.").or_else(|| path.strip_prefix("geometry.objects[id=").and_then(|rest| rest.strip_suffix(']')));
    let Some(id) = id.filter(|id| !id.is_empty() && !id.contains(['.', '[', ']'])) else { return Ok(None) };
    if !document.geometry.objects.contains_key(id) {
        return Err(format!("unknown geometry object '{id}'"));
    }
    payload_of("retire-geometry-object", DslValue::object([("id".to_string(), DslValue::String(id.to_string()))])).map(|mutation| Some(vec![mutation]))
}

/// 📐️ The numeric geometry leaves an editor sets: primitive dimensions, placement translation and rotation of any shape node, space and surface bound corners and port positions and directions.
fn geometry_leaf(path: &str) -> bool {
    let Some((id, leaf)) = path.strip_prefix("geometry.objects.").and_then(|rest| rest.split_once('.')) else { return false };
    let indexed = leaf.ends_with(']');
    let node = leaf.starts_with("shape.") || leaf.starts_with("symbolic.");
    !id.is_empty()
        && ((node && (leaf.contains(".parameters.") || (indexed && (leaf.contains(".translation[") || leaf.contains(".rotationDeg["))))) || (indexed && ((leaf.starts_with("spaces[") || leaf.starts_with("surfaces[")) && leaf.contains(".bounds.") || leaf.starts_with("ports[") && (leaf.contains(".position[") || leaf.contains(".direction[")))))
}

/// 🖊️ Whether `path` has an editor setter: a leaf inside a typed rule of [`EDIT_RULES`] or a numeric geometry leaf resolved by [`resolve_edit`] as one atomic object replacement.
pub fn can_set_field(path: &str) -> bool {
    EDIT_RULES.can_set_field(path) || geometry_leaf(path)
}

/// 🎯️ Resolves an editor edit into concrete kind mutations: the numeric geometry and script-limit leaves by their facet, row-id removals first, every other path through [`EDIT_RULES`].
pub fn resolve_edit(document: &Iso16757Snapshot, edit: &NormEdit) -> Result<Vec<Iso16757Mutation>, String> {
    match edit {
        NormEdit::SetField { path, value } => {
            if let Some(rest) = path.strip_prefix("geometry.objects.") {
                return resolve_geometry(document, rest, value);
            }
            if let Some(leaf) = path.strip_prefix("scriptLimits.") {
                return resolve_script_limits(document, leaf, value);
            }
        }
        NormEdit::InsertItem { path, value, .. } if path == "geometry.objects" => return resolve_geometry_insert(document, value.as_ref()),
        NormEdit::RemoveItem { path, .. } => {
            if let Some(mutations) = resolve_geometry_remove(document, path)? {
                return Ok(mutations);
            }
        }
        _ => {}
    }
    EDIT_RULES.resolve_with_id_removals(ID_REMOVALS, document, edit)
}
