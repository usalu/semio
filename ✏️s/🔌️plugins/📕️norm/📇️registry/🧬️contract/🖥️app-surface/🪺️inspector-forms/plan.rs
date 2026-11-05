//! 🧭️ Inspector intents bind actual native mutation schemas and admit candidates through the same typed edit plan.
use super::*;
use semio_framework::{ActionArgDef,ArgSchema,mutation_input_defs,registered_input_schema_document};
use semio_framework::action_staging::StagedFormRequest;
use semio_framework_value::DslValue;
use semio_framework_plugin::app::time_travel::time_travel_variant_selector;
use crate::{NormEdit, NormRetainedEditor, get_value_at_path, parse_path, PathSegment, resolve_list_index, resolve_norm_edits};
use semio_framework_value::{ToValue, retirement::RetireOwned};

fn inputs<A: NormRetainedEditor>(kind: &str) -> Result<Vec<ActionArgDef>, String> {
    use protocol::Mutation;
    let index = A::Mutation::DESCRIPTORS.iter().position(|descriptor| descriptor.semantic_kind == kind).ok_or_else(|| format!("unknown mutation kind: {kind}"))?;
    let schema = A::Mutation::INPUT_SCHEMAS.get(index).ok_or_else(|| format!("missing mutation schema: {kind}"))?;
    let documents = A::Mutation::INPUT_SCHEMA_DOCUMENTS.get(index).copied().unwrap_or_default();
    mutation_input_defs(schema, &|id: &str| {
        documents.iter().find_map(|text| {
            let document = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
            let document = semio_framework_pack_json::to_dsl_value(&document);
            (document.get("$id").and_then(DslValue::as_str) == Some(id)).then_some(document)
        }).or_else(|| registered_input_schema_document(id))
    }).map_err(|error| error.to_string())
}
fn descend(mut input: ActionArgDef, value: &DslValue, path: &str) -> Result<ActionArgDef, String> {
    let mut cursor = value;
    for segment in parse_path(path)? {
        match segment {
            PathSegment::Field(key) => {
                let ArgSchema::Object { fields } = &input.schema else { return Err("variant parent has no object input schema".into()) };
                input = fields.iter().find(|input| input.key() == key).cloned().ok_or_else(|| format!("unknown variant input: {key}"))?;
                cursor = cursor.get(&key).ok_or_else(|| format!("missing variant field: {key}"))?;
            },
            segment @ (PathSegment::Index(_) | PathSegment::Id(_)) => {
                let ArgSchema::Array { items, .. } = &input.schema else { return Err("variant row has no array input schema".into()) };
                let values = cursor.as_array().ok_or_else(|| "variant row is not an array".to_string())?;
                cursor = &values[resolve_list_index(values, &segment, path)?];
                input.schema = *items.clone();
            },
        }
    }
    Ok(input)
}

/// 🧭️ Schema-bound insertion and discriminated replacement plans; ordinary supplied scalar edits retain their direct route.
pub fn norm_inspector_form_plan<A: NormRetainedEditor>(document: &A::Snapshot, request: &StagedFormRequest) -> Result<Option<NormInspectorFormPlan>, String> {
    let path = request.args.get("path").and_then(DslValue::as_str).ok_or_else(|| "inspector path is absent".to_string())?;
    let rules = A::inspector_rules();
    match request.token.action.as_str() {
        "insertItem" => {
            let rule = rules.insertion_binding(path).ok_or_else(|| "this list has no insertion operation".to_string())?;
            let index = request.args.get("index").and_then(DslValue::as_u64).and_then(|index| usize::try_from(index).ok()).ok_or_else(|| "insertion index is invalid".to_string())?;
            let input = inputs::<A>(rule.kind)?.into_iter().find(|input| input.key() == rule.item).ok_or_else(|| "insertion has no typed item input".to_string())?;
            let seed = input.default.clone().unwrap_or_else(|| norm_inspector_seed(&input.schema));
            Ok(Some(NormInspectorFormPlan { target: NormInspectorFormTarget::Insert { path: path.into(), index }, input, seed, draft_key: "value".into() }))
        },
        "setField" => {
            let Some(variant) = request.args.get("value").and_then(DslValue::as_str) else { return Ok(None) };
            let binding = rules.setter_binding(path).ok_or_else(|| "this field has no setter operation".to_string())?;
            if binding.relative.is_empty() { return Ok(None); }
            let (parent, selector) = binding.relative.rsplit_once('.').unwrap_or(("", binding.relative.as_str()));
            let input = inputs::<A>(binding.rule.kind)?.into_iter().find(|input| input.key() == binding.rule.value).ok_or_else(|| "setter has no typed value input".to_string())?;
            let tree = document.to_value();
            let current = get_value_at_path(&tree, &binding.boundary)?;
            let input = if parent.is_empty() { input } else { descend(input, current, parent)? };
            let ArgSchema::Object { fields } = &input.schema else { return Ok(None) };
            if time_travel_variant_selector(fields).is_none_or(|input| input.key() != selector) { return Ok(None); }
            let boundary = if parent.is_empty() { binding.boundary } else { format!("{}.{}", binding.boundary, parent) };
            let current = get_value_at_path(&tree, &boundary)?;
            NormInspectorFormPlan::variant(input, current, selector, variant, boundary).map(Some)
        },
        _ => Err("unsupported inspector form action".into()),
    }
}

/// 🛡️ A readiness probe applies actual native operations to a private typed projection and retires its admitted operations.
pub fn admit_norm_inspector_args<A>(document: &A::Snapshot, action: &str, args: &DslValue) -> Result<(), String>
where
    A: NormRetainedEditor,
    A::Mutation: protocol::ColdMutation<A::Snapshot> + RetireOwned,
{
    let path = args.get("path").and_then(DslValue::as_str).ok_or_else(|| "inspector path is absent".to_string())?.to_string();
    let value = args.get("value").ok_or_else(|| "inspector value is absent".to_string())?.clone();
    let edit = match action {
        "insertItem" => NormEdit::InsertItem { path, index: args.get("index").and_then(DslValue::as_u64).and_then(|index| usize::try_from(index).ok()).ok_or_else(|| "insertion index is invalid".to_string())?, value: Some(value) },
        "setField" => NormEdit::SetField { path, value },
        _ => return Err("unsupported inspector form action".into()),
    };
    let admitted = resolve_norm_edits(document, &[edit], A::inspector_resolve)?;
    for operation in admitted { protocol::Mutation::retire_cold(operation); }
    Ok(())
}
