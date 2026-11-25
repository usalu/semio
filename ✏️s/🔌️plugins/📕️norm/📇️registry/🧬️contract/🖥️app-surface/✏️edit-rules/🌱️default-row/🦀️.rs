//! 🌱️ The default row of a collection, derived schema-first from the item schema of its insert mutation.

use super::{get_value_at_path, object_id_field, parse_path, InsertItemRule, NormEditRules};
use semio_framework::{mutation_input_defs, registered_input_schema_document, ActionArgDef, ArgSchema, ReferenceIdType};
use semio_framework_value::DslValue;
use semio_s_plugin_norm_inspector_forms::norm_inspector_seed;

/// 🧬️ The typed inputs of the mutation `kind` of `M`, read from its declared input schema and the schema documents it references.
pub fn mutation_inputs<D, M: protocol::Mutation<D>>(kind: &str) -> Result<Vec<ActionArgDef>, String> {
    let index = M::DESCRIPTORS.iter().position(|descriptor| descriptor.semantic_kind == kind).ok_or_else(|| format!("unknown mutation kind: {kind}"))?;
    let schema = M::INPUT_SCHEMAS.get(index).ok_or_else(|| format!("missing mutation schema: {kind}"))?;
    let documents = M::INPUT_SCHEMA_DOCUMENTS.get(index).copied().unwrap_or_default();
    mutation_input_defs(schema, &|id: &str| {
        documents.iter().find_map(|text| {
            let document = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
            let document = semio_framework_pack_json::to_dsl_value(&document);
            (document.get("$id").and_then(DslValue::as_str) == Some(id)).then_some(document)
        }).or_else(|| registered_input_schema_document(id))
    }).map_err(|error| error.to_string())
}

/// 🔗️ The ids of the rows of one collection template (`buildings[].storeys`), over every parent row.
fn template_ids(tree: &DslValue, template: &str) -> Vec<String> {
    let parts: Vec<&str> = template.split("[]").collect();
    let mut level = vec![tree.clone()];
    for (n, part) in parts.iter().enumerate() {
        let part = part.trim_start_matches('.');
        let mut next = Vec::new();
        for value in &level {
            let reached = if part.is_empty() { Some(value) } else { get_value_at_path(value, part).ok() };
            let Some(reached) = reached else { continue };
            let rows: Vec<DslValue> = match reached {
                DslValue::Array(rows) => rows.clone(),
                DslValue::Object(entries) if n + 1 == parts.len() => entries.iter().map(|(_, row)| row.clone()).collect(),
                _ => Vec::new(),
            };
            next.extend(rows);
        }
        level = next;
    }
    level.iter().filter_map(|row| object_id_field(row).map(str::to_string)).collect()
}

fn collect_links(schema: &ArgSchema, links: &mut Vec<Vec<String>>) {
    match schema {
        ArgSchema::Reference { kinds, many: false, id_type: ReferenceIdType::String, .. } => links.push(kinds.clone()),
        ArgSchema::Object { fields } => fields.iter().filter(|field| field.required && field.key() != "id").for_each(|field| collect_links(&field.schema, links)),
        ArgSchema::Array { items, min_items: Some(count), .. } if *count > 0 => collect_links(items, links),
        _ => {}
    }
}

impl NormEditRules {
    /// 🔗️ The insert rules whose collection holds rows of one of `kinds`: the rule names the kind by its item or by its mutation kind without the verb (`insert-roof` -> `roof`), compared without case and separators.
    fn rules_of_kinds(&self, kinds: &[String]) -> Vec<&InsertItemRule> {
        let plain = |name: &str| name.chars().filter(|c| c.is_alphanumeric()).flat_map(char::to_lowercase).collect::<String>();
        let named = |rule: &InsertItemRule| {
            let stem = ["insert-", "add-", "introduce-"].iter().find_map(|prefix| rule.kind.strip_prefix(prefix)).unwrap_or(rule.kind);
            kinds.iter().any(|kind| plain(kind) == plain(rule.item) || plain(kind) == plain(stem))
        };
        self.insert_item.iter().filter(|rule| named(rule)).collect()
    }

    /// 🔗️ The collections (insert path templates) a link of one of `kinds` points into; empty for a kind no document collection holds (a catalogue reference).
    pub fn link_collections(&self, kinds: &[String]) -> Vec<&'static str> {
        self.rules_of_kinds(kinds).into_iter().map(|rule| rule.path).collect()
    }

    /// 🔗️ The ids a link of one of `kinds` can point at in `tree`, collection by collection.
    pub fn link_ids(&self, tree: &DslValue, kinds: &[String]) -> Vec<String> {
        self.rules_of_kinds(kinds).into_iter().flat_map(|rule| template_ids(tree, rule.path)).collect()
    }

    /// 🔗️ The kinds each required link of a new row of `path` may point at, in field order: the prerequisites of the first row of the collection.
    pub fn required_links<D, M: protocol::Mutation<D>>(&self, path: &str) -> Result<Vec<Vec<String>>, String> {
        let rule = self.insertion_binding(path).ok_or_else(|| format!("no kind inserts into '{path}'"))?;
        let input = mutation_inputs::<D, M>(rule.kind)?.into_iter().find(|input| input.key() == rule.item).ok_or_else(|| format!("insertion '{}' has no typed item input", rule.kind))?;
        let mut links = Vec::new();
        collect_links(&input.schema, &mut links);
        Ok(links)
    }

    /// 🔗️ The existing id a link of one of `kinds` points at: the first id of [`NormEditRules::link_ids`]; `None` when no document collection holds the kind (a catalogue reference keeps its seed); a refusal naming the kinds when the collection exists and is empty.
    fn linked_id(&self, tree: &DslValue, kinds: &[String]) -> Result<Option<DslValue>, String> {
        if self.rules_of_kinds(kinds).is_empty() {
            return Ok(None);
        }
        self.link_ids(tree, kinds).into_iter().next().map(|id| Some(DslValue::String(id))).ok_or_else(|| format!("the row links to a {} and the document has none yet: insert one first into {} (the insert palette where it has a tool, else the inspector list or the insertItem command)", kinds.join(" or "), self.link_collections(kinds).iter().map(|path| format!("'{path}'")).collect::<Vec<_>>().join(" or ")))
    }

    fn link(&self, tree: &DslValue, schema: &ArgSchema, seed: &mut DslValue) -> Result<(), String> {
        match (schema, seed) {
            (ArgSchema::Reference { kinds, many: false, id_type: ReferenceIdType::String, .. }, seed) => {
                if let Some(id) = self.linked_id(tree, kinds)? {
                    *seed = id;
                }
            }
            (ArgSchema::Object { fields }, DslValue::Object(entries)) => {
                for (key, value) in entries.iter_mut() {
                    if let Some(field) = fields.iter().find(|field| field.key() == *key && field.key() != "id") {
                        self.link(tree, &field.schema, value)?;
                    }
                }
            }
            (ArgSchema::Array { items, .. }, DslValue::Array(rows)) => rows.iter_mut().try_for_each(|row| self.link(tree, items, row))?,
            _ => {}
        }
        Ok(())
    }

    /// 🌱️ The default row of the collection at `path`: the typed seed of the item input of its insert mutation (schema defaults, first enum options, required fields only) whose required links point at existing rows of `tree`; a link with nothing to point at refuses the insertion.
    pub fn default_row<D, M: protocol::Mutation<D>>(&self, tree: &DslValue, path: &str) -> Result<DslValue, String> {
        let rule = self.insertion_binding(path).ok_or_else(|| format!("no kind inserts into '{path}'"))?;
        let input = mutation_inputs::<D, M>(rule.kind)?.into_iter().find(|input| input.key() == rule.item).ok_or_else(|| format!("insertion '{}' has no typed item input", rule.kind))?;
        let mut row = input.default.clone().unwrap_or_else(|| norm_inspector_seed(&input.schema));
        self.link(tree, &input.schema, &mut row)?;
        Ok(row)
    }

    /// 🌱️ The row an insertion without a value places: the [`NormEditRules::default_row`] under a fresh `<collection>-<n>` id where the row has one.
    pub fn seeded_row<D, M: protocol::Mutation<D>>(&self, tree: &DslValue, path: &str) -> Result<DslValue, String> {
        let mut row = self.default_row::<D, M>(tree, path)?;
        let taken: Vec<String> = match get_value_at_path(tree, path) {
            Ok(DslValue::Array(rows)) => rows.iter().filter_map(|row| object_id_field(row)).map(str::to_string).collect(),
            _ => Vec::new(),
        };
        let name = parse_path(path).ok().and_then(|segments| segments.into_iter().rev().find_map(|segment| match segment { super::PathSegment::Field(name) => Some(name), _ => None })).unwrap_or_else(|| "row".to_string());
        if let DslValue::Object(entries) = &mut row {
            if let Some((_, id)) = entries.iter_mut().find(|(key, _)| key == "id") {
                *id = DslValue::String((taken.len() + 1..).map(|n| format!("{name}-{n}")).find(|id| !taken.contains(id)).unwrap_or_default());
            }
        }
        Ok(row)
    }
}
