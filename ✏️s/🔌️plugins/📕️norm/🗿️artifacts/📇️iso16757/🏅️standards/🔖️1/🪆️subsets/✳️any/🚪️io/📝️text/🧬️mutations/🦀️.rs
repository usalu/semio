//! ⚡️ ISO 16757 artifact — hand-rolled `OpText`/`OpBinary` for `Iso16757Mutation`.
//! `#[derive(dsl_derive::Mutations)]` only generates `Mutation`/`SemanticMutation` (see
//! `../../../🧬️schema/🧬️mutations/🦀️.rs`'s `🔖️Mutations` region) — the wire-text/wire-binary codecs stay handcrafted
//! here, one keyword per semantic verb, grammar `keyword key1=value1 key2=value2 ...`. Structured
//! payload fields (entity records, catalogue values, selection constraints, the part-number rule)
//! round-trip through a quoted JSON string — every one of them already derives
//! `Serialize`/`Deserialize`, so a second handcrafted grammar per structured type would just
//! duplicate that losslessly.

use crate::artifact_schema::mutations::Iso16757Mutation;

use crate::artifact_schema::mutations::{
    add_selection_constraint::mutation::AddSelectionConstraint, change_exchange_process::mutation::ChangeExchangeProcess, change_part_number_input::mutation::ChangePartNumberInput, change_selection_class::mutation::ChangeSelectionClass,
    change_selection_series::mutation::ChangeSelectionSeries, introduce_geometry_object::mutation::IntroduceGeometryObject, introduce_product::mutation::IntroduceProduct, introduce_product_class::mutation::IntroduceProductClass,
    introduce_product_group::mutation::IntroduceProductGroup, introduce_product_index::mutation::IntroduceProductIndex, introduce_product_series::mutation::IntroduceProductSeries, introduce_property_definition::mutation::IntroducePropertyDefinition,
    introduce_subject::mutation::IntroduceSubject, retire_geometry_object::mutation::RetireGeometryObject, retire_product::mutation::RetireProduct, retire_product_class::mutation::RetireProductClass,
    retire_product_group::mutation::RetireProductGroup, retire_product_index::mutation::RetireProductIndex, retire_product_series::mutation::RetireProductSeries, retire_property_definition::mutation::RetirePropertyDefinition,
    retire_subject::mutation::RetireSubject, remove_part_number_input::mutation::RemovePartNumberInput, remove_selection_constraint::mutation::RemoveSelectionConstraint, rename_catalogue::mutation::RenameCatalogue,
    rename_manufacturer::mutation::RenameManufacturer, rename_product::mutation::RenameProduct, rename_product_group::mutation::RenameProductGroup, replace_part_number_rule::mutation::ReplacePartNumberRule,
    change_script_limits::mutation::ChangeScriptLimits,
};

//#region 📖️SemioGrammar
pub const COMPONENT_GRAMMAR_SEMIO: &str = include_str!("📖️.grammar.semio");
pub const COMPONENT_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️.grammar.semio");
//#endregion 📖️SemioGrammar

//#region 🔖️ScalarCodec
/// 🔤️ Quoted-string encode/decode — the only value kind that can contain a raw space, so every
/// other scalar's text form stays space-free and tokenizable by [`tokenize_args`].
fn enc_str(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}
fn dec_str(s: &str) -> Result<String, String> {
    let inner = s.strip_prefix('"').and_then(|s| s.strip_suffix('"')).ok_or_else(|| format!("expected quoted string, got {s:?}"))?;
    let mut out = String::with_capacity(inner.len());
    let mut chars = inner.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('"') => out.push('"'),
            Some(other) => return Err(format!("bad escape \\{other}")),
            None => return Err("dangling escape".into()),
        }
    }
    Ok(out)
}
fn enc_opt_str(s: &Option<String>) -> String {
    match s {
        Some(v) => enc_str(v),
        None => "-".to_string(),
    }
}
fn dec_opt_str(s: &str) -> Result<Option<String>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_str(s)?))
    }
}
fn enc_usize(v: usize) -> String {
    v.to_string()
}
fn dec_usize(s: &str) -> Result<usize, String> {
    s.parse().map_err(|e: std::num::ParseIntError| e.to_string())
}
fn enc_opt_usize(v: &Option<usize>) -> String {
    match v {
        Some(index) => index.to_string(),
        None => "-".to_string(),
    }
}
fn dec_opt_usize(s: &str) -> Result<Option<usize>, String> {
    if s == "-" {
        Ok(None)
    } else {
        Ok(Some(dec_usize(s)?))
    }
}
/// 🧬️ Every structured payload field (entity records, catalogue values, part-number rule,
/// selection constraints) already derives `ToValue`/`FromValue` — a quoted JSON string reuses
/// that losslessly instead of a second handcrafted grammar per type.
fn enc_json<T: semio_framework_value::ToValue>(value: &T) -> String {
    enc_str(&semio_framework_pack_json::to_json_string(value))
}
fn dec_json<T: semio_framework_value::FromValue>(s: &str) -> Result<T, String> {
    semio_framework_pack_json::from_json_str(&dec_str(s)?, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|e| e.to_string())
}
//#endregion 🔖️ScalarCodec

//#region 🔖️Tokenizer
/// 🔡️ Splits `key=value` tokens on plain spaces, EXCEPT spaces inside a `"..."` quoted value —
/// needed because names/JSON payloads may contain spaces.
fn tokenize_args(rest: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut in_quotes = false;
    let mut chars = rest.chars();
    while let Some(c) = chars.next() {
        match c {
            '"' => {
                current.push(c);
                in_quotes = !in_quotes;
            }
            '\\' if in_quotes => {
                current.push(c);
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' if !in_quotes => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(c),
        }
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    tokens
}
fn parse_args(rest: &str) -> Result<std::collections::BTreeMap<String, String>, String> {
    tokenize_args(rest).into_iter().map(|token| token.split_once('=').map(|(k, v)| (k.to_string(), v.to_string())).ok_or_else(|| format!("bad arg token {token:?}"))).collect()
}
//#endregion 🔖️Tokenizer

//#region 🔖️OpText
fn print_iso16757_mutation(mutation: &Iso16757Mutation) -> String {
    match mutation {
        Iso16757Mutation::ChangeExchangeProcess(p) => format!("change-exchange-process new-exchange-process={}", enc_json(&p.new_exchange_process)),
        Iso16757Mutation::ChangeScriptLimits(p) => format!("change-script-limits new-max-steps={} new-max-recursion={} new-timeout-ms={}", p.new_max_steps, p.new_max_recursion, p.new_timeout_ms),
        Iso16757Mutation::ReplacePartNumberRule(p) => format!("replace-part-number-rule new-rule={}", enc_json(&p.new_rule)),
        Iso16757Mutation::ChangePartNumberInput(p) => format!("change-part-number-input key={} new-value={}", enc_str(&p.key), enc_json(&p.new_value)),
        Iso16757Mutation::RemovePartNumberInput(p) => format!("remove-part-number-input key={}", enc_str(&p.key)),
        Iso16757Mutation::ChangeSelectionClass(p) => format!("change-selection-class new-class-id={}", enc_str(&p.new_class_id)),
        Iso16757Mutation::ChangeSelectionSeries(p) => format!("change-selection-series new-series-id={}", enc_opt_str(&p.new_series_id)),
        Iso16757Mutation::AddSelectionConstraint(p) => format!("add-selection-constraint constraint={}", enc_json(&p.constraint)),
        Iso16757Mutation::RemoveSelectionConstraint(p) => format!("remove-selection-constraint index={}", enc_usize(p.index)),
        Iso16757Mutation::RenameCatalogue(p) => format!("rename-catalogue new-name={}", enc_str(&p.new_name)),
        Iso16757Mutation::RenameManufacturer(p) => format!("rename-manufacturer new-name={}", enc_str(&p.new_name)),
        Iso16757Mutation::IntroduceProductGroup(p) => format!("introduce-product-group product-group={} index={}", enc_json(&p.product_group), enc_opt_usize(&p.index)),
        Iso16757Mutation::RetireProductGroup(p) => format!("retire-product-group id={}", enc_str(&p.id)),
        Iso16757Mutation::RenameProductGroup(p) => format!("rename-product-group id={} new-name={}", enc_str(&p.id), enc_str(&p.new_name)),
        Iso16757Mutation::IntroduceProduct(p) => format!("introduce-product product={} index={}", enc_json(&p.product), enc_opt_usize(&p.index)),
        Iso16757Mutation::RetireProduct(p) => format!("retire-product id={}", enc_str(&p.id)),
        Iso16757Mutation::RenameProduct(p) => format!("rename-product id={} new-name={}", enc_str(&p.id), enc_str(&p.new_name)),
        Iso16757Mutation::IntroducePropertyDefinition(p) => format!("introduce-property-definition property-definition={} index={}", enc_json(&p.property_definition), enc_opt_usize(&p.index)),
        Iso16757Mutation::RetirePropertyDefinition(p) => format!("retire-property-definition id={}", enc_str(&p.id)),
        Iso16757Mutation::IntroduceSubject(p) => format!("introduce-subject subject={} index={}", enc_json(&p.subject), enc_opt_usize(&p.index)),
        Iso16757Mutation::RetireSubject(p) => format!("retire-subject id={}", enc_str(&p.id)),
        Iso16757Mutation::IntroduceProductClass(p) => format!("introduce-product-class product-class={} index={}", enc_json(&p.product_class), enc_opt_usize(&p.index)),
        Iso16757Mutation::RetireProductClass(p) => format!("retire-product-class id={}", enc_str(&p.id)),
        Iso16757Mutation::IntroduceProductSeries(p) => format!("introduce-product-series product-series={} index={}", enc_json(&p.product_series), enc_opt_usize(&p.index)),
        Iso16757Mutation::RetireProductSeries(p) => format!("retire-product-series id={}", enc_str(&p.id)),
        Iso16757Mutation::IntroduceProductIndex(p) => format!("introduce-product-index product-index={} index={}", enc_json(&p.product_index), enc_opt_usize(&p.index)),
        Iso16757Mutation::RetireProductIndex(p) => format!("retire-product-index id={}", enc_str(&p.id)),
        Iso16757Mutation::IntroduceGeometryObject(p) => format!("introduce-geometry-object geometry-object={}", enc_json(&p.geometry_object)),
        Iso16757Mutation::RetireGeometryObject(p) => format!("retire-geometry-object id={}", enc_str(&p.id)),
    }
}

fn parse_iso16757_mutation(line: &str) -> Result<Iso16757Mutation, String> {
    let (keyword, rest) = line.split_once(' ').unwrap_or((line, ""));
    let args = parse_args(rest)?;
    let arg = |k: &str| args.get(k).cloned().ok_or_else(|| format!("iso16757 mutation: missing arg '{k}' for '{keyword}'"));
    match keyword {
        "change-exchange-process" => Ok(Iso16757Mutation::ChangeExchangeProcess(ChangeExchangeProcess { new_exchange_process: dec_json(&arg("new-exchange-process")?)? })),
        "change-script-limits" => Ok(Iso16757Mutation::ChangeScriptLimits(ChangeScriptLimits {
            new_max_steps: arg("new-max-steps")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            new_max_recursion: arg("new-max-recursion")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
            new_timeout_ms: arg("new-timeout-ms")?.parse().map_err(|e: std::num::ParseIntError| e.to_string())?,
        })),
        "replace-part-number-rule" => Ok(Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: dec_json(&arg("new-rule")?)? })),
        "change-part-number-input" => Ok(Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key: dec_str(&arg("key")?)?, new_value: dec_json(&arg("new-value")?)? })),
        "remove-part-number-input" => Ok(Iso16757Mutation::RemovePartNumberInput(RemovePartNumberInput { key: dec_str(&arg("key")?)? })),
        "change-selection-class" => Ok(Iso16757Mutation::ChangeSelectionClass(ChangeSelectionClass { new_class_id: dec_str(&arg("new-class-id")?)? })),
        "change-selection-series" => Ok(Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: dec_opt_str(&arg("new-series-id")?)? })),
        "add-selection-constraint" => Ok(Iso16757Mutation::AddSelectionConstraint(AddSelectionConstraint { constraint: dec_json(&arg("constraint")?)? })),
        "remove-selection-constraint" => Ok(Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index: dec_usize(&arg("index")?)? })),
        "rename-catalogue" => Ok(Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: dec_str(&arg("new-name")?)? })),
        "rename-manufacturer" => Ok(Iso16757Mutation::RenameManufacturer(RenameManufacturer { new_name: dec_str(&arg("new-name")?)? })),
        "introduce-product-group" => Ok(Iso16757Mutation::IntroduceProductGroup(IntroduceProductGroup { product_group: dec_json(&arg("product-group")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "retire-product-group" => Ok(Iso16757Mutation::RetireProductGroup(RetireProductGroup { id: dec_str(&arg("id")?)? })),
        "rename-product-group" => Ok(Iso16757Mutation::RenameProductGroup(RenameProductGroup { id: dec_str(&arg("id")?)?, new_name: dec_str(&arg("new-name")?)? })),
        "introduce-product" => Ok(Iso16757Mutation::IntroduceProduct(IntroduceProduct { product: dec_json(&arg("product")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "retire-product" => Ok(Iso16757Mutation::RetireProduct(RetireProduct { id: dec_str(&arg("id")?)? })),
        "rename-product" => Ok(Iso16757Mutation::RenameProduct(RenameProduct { id: dec_str(&arg("id")?)?, new_name: dec_str(&arg("new-name")?)? })),
        "introduce-property-definition" => Ok(Iso16757Mutation::IntroducePropertyDefinition(IntroducePropertyDefinition { property_definition: dec_json(&arg("property-definition")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "retire-property-definition" => Ok(Iso16757Mutation::RetirePropertyDefinition(RetirePropertyDefinition { id: dec_str(&arg("id")?)? })),
        "introduce-subject" => Ok(Iso16757Mutation::IntroduceSubject(IntroduceSubject { subject: dec_json(&arg("subject")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "retire-subject" => Ok(Iso16757Mutation::RetireSubject(RetireSubject { id: dec_str(&arg("id")?)? })),
        "introduce-product-class" => Ok(Iso16757Mutation::IntroduceProductClass(IntroduceProductClass { product_class: dec_json(&arg("product-class")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "retire-product-class" => Ok(Iso16757Mutation::RetireProductClass(RetireProductClass { id: dec_str(&arg("id")?)? })),
        "introduce-product-series" => Ok(Iso16757Mutation::IntroduceProductSeries(IntroduceProductSeries { product_series: dec_json(&arg("product-series")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "retire-product-series" => Ok(Iso16757Mutation::RetireProductSeries(RetireProductSeries { id: dec_str(&arg("id")?)? })),
        "introduce-product-index" => Ok(Iso16757Mutation::IntroduceProductIndex(IntroduceProductIndex { product_index: dec_json(&arg("product-index")?)?, index: dec_opt_usize(&arg("index")?)? })),
        "retire-product-index" => Ok(Iso16757Mutation::RetireProductIndex(RetireProductIndex { id: dec_str(&arg("id")?)? })),
        "introduce-geometry-object" => Ok(Iso16757Mutation::IntroduceGeometryObject(IntroduceGeometryObject { geometry_object: dec_json(&arg("geometry-object")?)? })),
        "retire-geometry-object" => Ok(Iso16757Mutation::RetireGeometryObject(RetireGeometryObject { id: dec_str(&arg("id")?)? })),
        other => Err(format!("iso16757 mutation: unknown keyword {other:?}")),
    }
}

impl protocol::OpText for Iso16757Mutation {
    fn print_op(&self) -> String {
        print_iso16757_mutation(self)
    }
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        parse_iso16757_mutation(line).map_err(|e| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, e, semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
}
//#endregion 🔖️OpText

//#region 🔖️OpBinaryCodec
/// 🎞️ Every variant's binary form is `tag u8 | json-string-per-field`; the JSON-per-field
/// consolidation used by `OpText` above applies equally here — one `write_str_bin` per field
/// regardless of that field's own structural complexity.









//#region 🏷️WireTags
/// 🏷️ Op tags of `Iso16757Mutation`, derived from the `record <kind> tag=<n>` lines of its `📡️.protocol.semio`.






























//#endregion 🏷️WireTags


//#endregion 🔖️OpBinaryCodec

//#region 🔖️DemoCases
/// 🧪️ One representative value per variant — reused by the round-trip law test below.
#[cfg(test)]
pub(crate) fn demo_mutation_cases() -> Vec<Iso16757Mutation> {
    use crate::{part_1, part_4, part_5, Cardinality, CatalogueValue, LocalizedText, Names};

    let names = |text: &str| Names { preferred: LocalizedText { locale: "en".into(), text: text.into() }, short_name: None, alternatives: Vec::new() };

    vec![
        Iso16757Mutation::ChangeExchangeProcess(ChangeExchangeProcess { new_exchange_process: part_5::ExchangeProcess::ProvideCatalogue }),
        Iso16757Mutation::ChangeScriptLimits(ChangeScriptLimits { new_max_steps: 1, new_max_recursion: 2, new_timeout_ms: 3 }),
        Iso16757Mutation::ReplacePartNumberRule(ReplacePartNumberRule { new_rule: part_5::PartNumberRule::Literal { value: "X-1".into() } }),
        Iso16757Mutation::ChangePartNumberInput(ChangePartNumberInput { key: "dn".into(), new_value: CatalogueValue::Decimal { value: 50.0 } }),
        Iso16757Mutation::RemovePartNumberInput(RemovePartNumberInput { key: "dn".into() }),
        Iso16757Mutation::ChangeSelectionClass(ChangeSelectionClass { new_class_id: "class-valve".into() }),
        Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: Some("series-cv".into()) }),
        Iso16757Mutation::ChangeSelectionSeries(ChangeSelectionSeries { new_series_id: None }),
        Iso16757Mutation::AddSelectionConstraint(AddSelectionConstraint { constraint: part_1::SelectionConstraint { id: "constraint-default".into(), property_id: "prop-dn".into(), operator: part_1::ConstraintOperator::Equal, value: CatalogueValue::Decimal { value: 50.0 } } }),
        Iso16757Mutation::RemoveSelectionConstraint(RemoveSelectionConstraint { index: 0 }),
        Iso16757Mutation::RenameCatalogue(RenameCatalogue { new_name: "Renamed \"Catalogue\"".into() }),
        Iso16757Mutation::RenameManufacturer(RenameManufacturer { new_name: "Renamed Mfg".into() }),
        Iso16757Mutation::IntroduceProductGroup(IntroduceProductGroup { product_group: part_1::ProductGroup { id: "group-new".into(), names: names("New Group"), dictionary_subject_id: None }, index: Some(0) }),
        Iso16757Mutation::RetireProductGroup(RetireProductGroup { id: "group-valves".into() }),
        Iso16757Mutation::RenameProductGroup(RenameProductGroup { id: "group-valves".into(), new_name: "Renamed Group".into() }),
        Iso16757Mutation::IntroduceProduct(IntroduceProduct {
            product: part_1::Product { id: "product-new".into(), series_id: "series-cv".into(), names: names("New Product"), parameter_domains: Vec::new(), variants: Vec::new(), static_properties: Vec::new() },
            index: None,
        }),
        Iso16757Mutation::RetireProduct(RetireProduct { id: "product-cv".into() }),
        Iso16757Mutation::RenameProduct(RenameProduct { id: "product-cv".into(), new_name: "Renamed Product".into() }),
        Iso16757Mutation::IntroducePropertyDefinition(IntroducePropertyDefinition {
            property_definition: part_1::PropertyDefinition {
                id: "prop-new".into(),
                names: names("New Prop"),
                data_type: "text".into(),
                unit: None,
                cardinality: Cardinality::optional(),
                kind: part_1::PropertyKind::Static,
                dictionary_property_id: None,
            },
            index: None,
        }),
        Iso16757Mutation::RetirePropertyDefinition(RetirePropertyDefinition { id: "prop-dn".into() }),
        Iso16757Mutation::IntroduceSubject(IntroduceSubject {
            subject: part_4::Subject { id: "subject-new".into(), kind: part_4::SubjectKind::ProductClass, names: names("New Subject"), definition: LocalizedText { locale: "en".into(), text: "def".into() }, parent_id: None },
            index: None,
        }),
        Iso16757Mutation::RetireSubject(RetireSubject { id: "subject-valve".into() }),
        Iso16757Mutation::IntroduceProductClass(IntroduceProductClass {
            product_class: part_1::ProductClass {
                id: "class-new".into(),
                group_id: "group-valves".into(),
                parent_id: None,
                names: names("New Class"),
                required_property_ids: Vec::new(),
                optional_property_ids: Vec::new(),
            },
            index: None,
        }),
        Iso16757Mutation::RetireProductClass(RetireProductClass { id: "class-valve".into() }),
        Iso16757Mutation::IntroduceProductSeries(IntroduceProductSeries {
            product_series: part_1::ProductSeries {
                id: "series-new".into(),
                class_id: "class-valve".into(),
                names: names("New Series"),
                shared_property_values: Default::default(),
                geometry_id: None,
            },
            index: None,
        }),
        Iso16757Mutation::RetireProductSeries(RetireProductSeries { id: "series-cv".into() }),
        Iso16757Mutation::IntroduceProductIndex(IntroduceProductIndex {
            product_index: part_1::ProductIndex { id: "index-new".into(), product_id: "product-cv".into(), variant_id: None, search_tags: vec!["new".into()] },
            index: None,
        }),
        Iso16757Mutation::RetireProductIndex(RetireProductIndex { id: "index-cv50".into() }),
        Iso16757Mutation::IntroduceGeometryObject(IntroduceGeometryObject {
            geometry_object: crate::part_2::GeometryObject {
                id: "geom-new".into(),
                shape: None,
                symbolic: None,
                spaces: Vec::new(),
                surfaces: Vec::new(),
                ports: Vec::new(),
                parameter_bindings: Default::default(),
            },
        }),
        Iso16757Mutation::RetireGeometryObject(RetireGeometryObject { id: "geom-valve-50".into() }),
    ]
}
//#endregion 🔖️DemoCases

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests

#[allow(unused_imports)]
mod mutations_codec {
use super::*;
use crate::standards::v1::subsets::any::schema::mutations::*;
use crate::{Iso16757Diff, Iso16757Snapshot};
use crate::standards::v1::subsets::any::schema::mutations::add_selection_constraint;
use crate::standards::v1::subsets::any::schema::mutations::change_exchange_process;
use crate::standards::v1::subsets::any::schema::mutations::change_part_number_input;
use crate::standards::v1::subsets::any::schema::mutations::change_selection_class;
use crate::standards::v1::subsets::any::schema::mutations::change_selection_series;
use crate::standards::v1::subsets::any::schema::mutations::introduce_product;
use crate::standards::v1::subsets::any::schema::mutations::introduce_product_group;
use crate::standards::v1::subsets::any::schema::mutations::introduce_property_definition;
use crate::standards::v1::subsets::any::schema::mutations::introduce_subject;
use crate::standards::v1::subsets::any::schema::mutations::retire_product;
use crate::standards::v1::subsets::any::schema::mutations::retire_product_group;
use crate::standards::v1::subsets::any::schema::mutations::retire_property_definition;
use crate::standards::v1::subsets::any::schema::mutations::retire_subject;
use crate::standards::v1::subsets::any::schema::mutations::remove_part_number_input;
use crate::standards::v1::subsets::any::schema::mutations::remove_selection_constraint;
use crate::standards::v1::subsets::any::schema::mutations::rename_catalogue;
use crate::standards::v1::subsets::any::schema::mutations::rename_manufacturer;
use crate::standards::v1::subsets::any::schema::mutations::rename_product;
use crate::standards::v1::subsets::any::schema::mutations::rename_product_group;
use crate::standards::v1::subsets::any::schema::mutations::replace_part_number_rule;
use crate::standards::v1::subsets::any::schema::mutations::change_script_limits;
use crate::standards::v1::subsets::any::schema::mutations::introduce_product_class;
use crate::standards::v1::subsets::any::schema::mutations::retire_product_class;
use crate::standards::v1::subsets::any::schema::mutations::introduce_product_series;
use crate::standards::v1::subsets::any::schema::mutations::retire_product_series;
use crate::standards::v1::subsets::any::schema::mutations::introduce_product_index;
use crate::standards::v1::subsets::any::schema::mutations::retire_product_index;
use crate::standards::v1::subsets::any::schema::mutations::introduce_geometry_object;
use crate::standards::v1::subsets::any::schema::mutations::retire_geometry_object;

/// 📥️ Decodes this facet's own internally-tagged (`{"mutation": "<camelCaseVariant>", …}`) JSON
/// projection — the exact shape the committed `<kind>/🧪️tests/<fixture>/🦠️mutation/🔣️.json`
/// specification vectors carry — into a real [`Iso16757Mutation`]. The generated test host of
/// `../../../../../🧪️tests/📇️mutate-iso16757-1` links only this crate, so `serde_json` is unreachable
/// from that adapter and the bridge belongs here rather than there.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn decode_iso16757_mutation_json(text: &str) -> Result<Iso16757Mutation, String> {
    semio_framework_pack_json::from_json_str(text, semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|error| error.to_string())
}
}
pub use mutations_codec::*;

/// 📜️ Describes the artifact mutation dialect.
pub const MUTATION_GRAMMAR_SEMIO: &str = include_str!("📖️mutations.grammar.semio");
/// 🧭️ Identifies the artifact mutation grammar.
pub const MUTATION_GRAMMAR_PATH: &str = concat!(module_path!(), "::📖️mutations.grammar.semio");
