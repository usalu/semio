extern crate serde_json;extern crate quote;extern crate proc_macro2;use quote::quote;use std::{collections::HashSet,fs,path::PathBuf};#[derive(Debug)]struct MutationSourceAuthority{owner:String}//#region 🔣️MutationLeafJson
#[derive(Debug, PartialEq, Eq)]
enum MutationLeafInvertibility { SelfInvertible, ExplicitMutation, Plan, NonInvertible }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafDiffParticipation { Detect, ApplyOnly, Plan, None }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafOutcomeClass { Applied, Info, Warning, Error, Fatal }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafComposition { Atomic, Composite }

#[derive(Debug, PartialEq, Eq)]
enum MutationLeafLanguageSurface { Rust, Typescript, Graphql, Protobuf, JsonSchema, Text, Binary }

#[derive(Debug, PartialEq, Eq)]
struct MutationLeafJson {
    schema_version: u32,
    owner: String,
    semantic_kind: String,
    display_name: String,
    emoji: String,
    aggregate_variant: String,
    payload_schema: String,
    text_opcode: Option<String>,
    binary_tag: Option<u32>,
    invertibility: MutationLeafInvertibility,
    diff_participation: MutationLeafDiffParticipation,
    outcome_classes: Vec<MutationLeafOutcomeClass>,
    composition: MutationLeafComposition,
    required_language_surfaces: Vec<MutationLeafLanguageSurface>,
}

const MUTATION_LEAF_DESCRIPTOR_KEYS: [&str; 14] = ["schemaVersion", "owner", "semanticKind", "displayName", "emoji", "aggregateVariant", "payloadSchema", "textOpcode", "binaryTag", "invertibility", "diffParticipation", "outcomeClasses", "composition", "requiredLanguageSurfaces"];

fn parse_mutation_leaf_descriptor(raw: &[u8], authority: &MutationSourceAuthority) -> Result<MutationLeafJson, String> {
    mutation_leaf_reject_duplicate_keys(raw)?;
    let value: serde_json::Value = serde_json::from_slice(raw).map_err(|error| format!("malformed mutation descriptor JSON: {error}"))?;
    let object = value.as_object().ok_or_else(|| "mutation descriptor must be an object".to_string())?;
    if object.len() != MUTATION_LEAF_DESCRIPTOR_KEYS.len() || MUTATION_LEAF_DESCRIPTOR_KEYS.iter().any(|key| !object.contains_key(*key)) || object.keys().any(|key| !MUTATION_LEAF_DESCRIPTOR_KEYS.contains(&key.as_str())) { return Err("mutation descriptor must contain exactly the fourteen schema fields".to_string()); }
    let string = |key| mutation_leaf_string(object.get(key).unwrap(), key);
    let schema_version = mutation_leaf_u32(object.get("schemaVersion").unwrap(), "schemaVersion")?;
    if schema_version != 1 { return Err("schemaVersion must equal 1".to_string()); }
    let owner = string("owner")?;
    if owner != authority.owner { return Err("descriptor owner does not exactly match source owner".to_string()); }
    let semantic_kind = string("semanticKind")?;
    if !mutation_leaf_kebab(&semantic_kind) { return Err("semanticKind must be lowercase kebab-case".to_string()); }
    let display_name = string("displayName")?;
    let emoji = string("emoji")?;
    let aggregate_variant = string("aggregateVariant")?;
    if !mutation_leaf_pascal(&aggregate_variant) { return Err("aggregateVariant must be ASCII PascalCase".to_string()); }
    let payload_schema = string("payloadSchema")?;
    let text_opcode = match object.get("textOpcode").unwrap() { serde_json::Value::Null => None, value => { let value = mutation_leaf_string(value, "textOpcode")?; if !mutation_leaf_kebab(&value) { return Err("textOpcode must be lowercase kebab-case or null".to_string()); } Some(value) } };
    let binary_tag = match object.get("binaryTag").unwrap() { serde_json::Value::Null => None, value => Some(mutation_leaf_u32(value, "binaryTag")?) };
    let invertibility = match string("invertibility")?.as_str() { "self" => MutationLeafInvertibility::SelfInvertible, "explicit-mutation" => MutationLeafInvertibility::ExplicitMutation, "plan" => MutationLeafInvertibility::Plan, "non-invertible" => MutationLeafInvertibility::NonInvertible, _ => return Err("invertibility is not a schema enum value".to_string()) };
    let diff_participation = match string("diffParticipation")?.as_str() { "detect" => MutationLeafDiffParticipation::Detect, "apply-only" => MutationLeafDiffParticipation::ApplyOnly, "plan" => MutationLeafDiffParticipation::Plan, "none" => MutationLeafDiffParticipation::None, _ => return Err("diffParticipation is not a schema enum value".to_string()) };
    let outcome_classes = mutation_leaf_outcomes(object.get("outcomeClasses").unwrap())?;
    let composition = match string("composition")?.as_str() { "atomic" => MutationLeafComposition::Atomic, "composite" => MutationLeafComposition::Composite, _ => return Err("composition is not a schema enum value".to_string()) };
    let required_language_surfaces = mutation_leaf_surfaces(object.get("requiredLanguageSurfaces").unwrap())?;
    Ok(MutationLeafJson { schema_version, owner, semantic_kind, display_name, emoji, aggregate_variant, payload_schema, text_opcode, binary_tag, invertibility, diff_participation, outcome_classes, composition, required_language_surfaces })
}

fn mutation_leaf_string(value: &serde_json::Value, key: &str) -> Result<String, String> { value.as_str().filter(|value| !value.is_empty()).map(str::to_owned).ok_or_else(|| format!("{key} must be a nonempty string")) }

fn mutation_leaf_u32(value: &serde_json::Value, key: &str) -> Result<u32, String> {
    let number = value.as_f64().ok_or_else(|| format!("{key} must be an integer"))?;
    if !number.is_finite() || number.fract() != 0.0 || number < 0.0 || number > u32::MAX as f64 { return Err(format!("{key} must be a u32 integer")); }
    Ok(number as u32)
}

fn mutation_leaf_kebab(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty() && bytes[0].is_ascii_lowercase() && bytes.contains(&b'-') && bytes.split(|byte| *byte == b'-').all(|part| !part.is_empty() && part.iter().all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit()))
}

fn mutation_leaf_pascal(value: &str) -> bool { let bytes = value.as_bytes(); !bytes.is_empty() && bytes[0].is_ascii_uppercase() && bytes.iter().all(|byte| byte.is_ascii_alphanumeric()) }

fn mutation_leaf_outcomes(value: &serde_json::Value) -> Result<Vec<MutationLeafOutcomeClass>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "outcomeClasses must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    values.iter().map(|value| { let value = mutation_leaf_string(value, "outcomeClasses")?; if !seen.insert(value.clone()) { return Err("outcomeClasses must not contain duplicates".to_string()); } match value.as_str() { "applied" => Ok(MutationLeafOutcomeClass::Applied), "info" => Ok(MutationLeafOutcomeClass::Info), "warning" => Ok(MutationLeafOutcomeClass::Warning), "error" => Ok(MutationLeafOutcomeClass::Error), "fatal" => Ok(MutationLeafOutcomeClass::Fatal), _ => Err("outcomeClasses contains a non-schema enum value".to_string()) } }).collect()
}

fn mutation_leaf_surfaces(value: &serde_json::Value) -> Result<Vec<MutationLeafLanguageSurface>, String> {
    let values = value.as_array().filter(|values| !values.is_empty()).ok_or_else(|| "requiredLanguageSurfaces must be a nonempty array".to_string())?;
    let mut seen = HashSet::new();
    let surfaces: Vec<_> = values.iter().map(|value| { let value = mutation_leaf_string(value, "requiredLanguageSurfaces")?; if !seen.insert(value.clone()) { return Err("requiredLanguageSurfaces must not contain duplicates".to_string()); } match value.as_str() { "rust" => Ok(MutationLeafLanguageSurface::Rust), "typescript" => Ok(MutationLeafLanguageSurface::Typescript), "graphql" => Ok(MutationLeafLanguageSurface::Graphql), "protobuf" => Ok(MutationLeafLanguageSurface::Protobuf), "json-schema" => Ok(MutationLeafLanguageSurface::JsonSchema), "text" => Ok(MutationLeafLanguageSurface::Text), "binary" => Ok(MutationLeafLanguageSurface::Binary), _ => Err("requiredLanguageSurfaces contains a non-schema enum value".to_string()) } }).collect::<Result<_, _>>()?;
    if !surfaces.iter().any(|surface| matches!(surface, MutationLeafLanguageSurface::Rust)) { return Err("requiredLanguageSurfaces must contain rust".to_string()); }
    Ok(surfaces)
}

fn mutation_leaf_reject_duplicate_keys(raw: &[u8]) -> Result<(), String> {
    let mut index = mutation_leaf_skip_ws(raw, 0);
    if raw.get(index) != Some(&b'{') { return Err("mutation descriptor must be a JSON object".to_string()); }
    index += 1;
    let mut keys = HashSet::new();
    loop {
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) == Some(&b'}') { return Ok(()); }
        let key_start = index;
        index = mutation_leaf_string_end(raw, index).ok_or_else(|| "malformed mutation descriptor JSON key".to_string())?;
        let key: String = serde_json::from_slice(&raw[key_start..index]).map_err(|_| "malformed mutation descriptor JSON key".to_string())?;
        if !keys.insert(key) { return Err("mutation descriptor has a duplicate key".to_string()); }
        index = mutation_leaf_skip_ws(raw, index);
        if raw.get(index) != Some(&b':') { return Err("malformed mutation descriptor JSON key separator".to_string()); }
        index = mutation_leaf_json_value_end(raw, mutation_leaf_skip_ws(raw, index + 1)).ok_or_else(|| "malformed mutation descriptor JSON value".to_string())?;
        index = mutation_leaf_skip_ws(raw, index);
        match raw.get(index) { Some(b',') => index += 1, Some(b'}') => return Ok(()), _ => return Err("malformed mutation descriptor JSON object".to_string()) }
    }
}

fn mutation_leaf_skip_ws(raw: &[u8], mut index: usize) -> usize { while raw.get(index).is_some_and(|byte| byte.is_ascii_whitespace()) { index += 1; } index }
fn mutation_leaf_string_end(raw: &[u8], mut index: usize) -> Option<usize> { if raw.get(index) != Some(&b'\"') { return None; } index += 1; while let Some(byte) = raw.get(index) { match byte { b'\"' => return Some(index + 1), b'\\' => index += 2, 0..=0x1f => return None, _ => index += 1 } } None }
fn mutation_leaf_json_value_end(raw: &[u8], index: usize) -> Option<usize> {
    match raw.get(index)? { b'\"' => mutation_leaf_string_end(raw, index), b'{' => mutation_leaf_balanced_end(raw, index, b'{', b'}'), b'[' => mutation_leaf_balanced_end(raw, index, b'[', b']'), _ => { let end = raw[index..].iter().position(|byte| matches!(*byte, b',' | b'}' | b']') || byte.is_ascii_whitespace()).map(|offset| index + offset).unwrap_or(raw.len()); (end > index).then_some(end) } }
}
fn mutation_leaf_balanced_end(raw: &[u8], mut index: usize, open: u8, close: u8) -> Option<usize> { let mut depth = 0usize; while let Some(byte) = raw.get(index) { if *byte == b'\"' { index = mutation_leaf_string_end(raw, index)?; continue; } if *byte == open { depth += 1; } else if *byte == close { depth -= 1; if depth == 0 { return Some(index + 1); } } index += 1; } None }

fn emit_mutation_leaf_descriptor(descriptor: &MutationLeafJson) -> proc_macro2::TokenStream {
    let schema_version = descriptor.schema_version; let owner = &descriptor.owner; let semantic_kind = &descriptor.semantic_kind; let display_name = &descriptor.display_name; let emoji = &descriptor.emoji; let aggregate_variant = &descriptor.aggregate_variant; let payload_schema = &descriptor.payload_schema;
    let text_opcode = descriptor.text_opcode.as_ref().map(|value| quote!(::core::option::Option::Some(#value))).unwrap_or_else(|| quote!(::core::option::Option::None)); let binary_tag = descriptor.binary_tag.map(|value| quote!(::core::option::Option::Some(#value))).unwrap_or_else(|| quote!(::core::option::Option::None));
    let invertibility = match &descriptor.invertibility { MutationLeafInvertibility::SelfInvertible => quote!(::semio_framework_os_kernel::MutationInvertibility::SelfInvertible), MutationLeafInvertibility::ExplicitMutation => quote!(::semio_framework_os_kernel::MutationInvertibility::ExplicitMutation), MutationLeafInvertibility::Plan => quote!(::semio_framework_os_kernel::MutationInvertibility::Plan), MutationLeafInvertibility::NonInvertible => quote!(::semio_framework_os_kernel::MutationInvertibility::NonInvertible) };
    let diff_participation = match &descriptor.diff_participation { MutationLeafDiffParticipation::Detect => quote!(::semio_framework_os_kernel::MutationDiffParticipation::Detect), MutationLeafDiffParticipation::ApplyOnly => quote!(::semio_framework_os_kernel::MutationDiffParticipation::ApplyOnly), MutationLeafDiffParticipation::Plan => quote!(::semio_framework_os_kernel::MutationDiffParticipation::Plan), MutationLeafDiffParticipation::None => quote!(::semio_framework_os_kernel::MutationDiffParticipation::None) };
    let outcome_classes = descriptor.outcome_classes.iter().map(|value| match value { MutationLeafOutcomeClass::Applied => quote!(::semio_framework_os_kernel::MutationOutcomeClass::Applied), MutationLeafOutcomeClass::Info => quote!(::semio_framework_os_kernel::MutationOutcomeClass::Info), MutationLeafOutcomeClass::Warning => quote!(::semio_framework_os_kernel::MutationOutcomeClass::Warning), MutationLeafOutcomeClass::Error => quote!(::semio_framework_os_kernel::MutationOutcomeClass::Error), MutationLeafOutcomeClass::Fatal => quote!(::semio_framework_os_kernel::MutationOutcomeClass::Fatal) });
    let composition = match &descriptor.composition { MutationLeafComposition::Atomic => quote!(::semio_framework_os_kernel::MutationComposition::Atomic), MutationLeafComposition::Composite => quote!(::semio_framework_os_kernel::MutationComposition::Composite) };
    let required_language_surfaces = descriptor.required_language_surfaces.iter().map(|value| match value { MutationLeafLanguageSurface::Rust => quote!(::semio_framework_os_kernel::MutationLanguageSurface::Rust), MutationLeafLanguageSurface::Typescript => quote!(::semio_framework_os_kernel::MutationLanguageSurface::Typescript), MutationLeafLanguageSurface::Graphql => quote!(::semio_framework_os_kernel::MutationLanguageSurface::Graphql), MutationLeafLanguageSurface::Protobuf => quote!(::semio_framework_os_kernel::MutationLanguageSurface::Protobuf), MutationLeafLanguageSurface::JsonSchema => quote!(::semio_framework_os_kernel::MutationLanguageSurface::JsonSchema), MutationLeafLanguageSurface::Text => quote!(::semio_framework_os_kernel::MutationLanguageSurface::Text), MutationLeafLanguageSurface::Binary => quote!(::semio_framework_os_kernel::MutationLanguageSurface::Binary) });
    quote!(::semio_framework_os_kernel::MutationLeafDescriptor { schema_version: #schema_version, owner: #owner, semantic_kind: #semantic_kind, display_name: #display_name, emoji: #emoji, aggregate_variant: #aggregate_variant, payload_schema: #payload_schema, text_opcode: #text_opcode, binary_tag: #binary_tag, invertibility: #invertibility, diff_participation: #diff_participation, outcome_classes: &[#(#outcome_classes),*], composition: #composition, required_language_surfaces: &[#(#required_language_surfaces),*] })
}

#[cfg(test)]
mod mutation_leaf_json_tests {
    use super::*;
    fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../../🧪️tests/🧬️mutation-leaf-json/🧫️fixtures/🔣️cases.json")).unwrap() }
    fn authority(owner: &str) -> MutationSourceAuthority { MutationSourceAuthority { workspace_root: PathBuf::new(), mutation_root: PathBuf::new(), owner: owner.to_string(), source_path: PathBuf::new(), descriptor_path: PathBuf::new(), taxonomy_path: PathBuf::new() } }
    #[test]
    fn parses_mutation_leaf_json_fixture() {
        let fixture = fixture(); let authority = authority(fixture["authorityOwner"].as_str().unwrap());
        for vector in fixture["cases"].as_array().unwrap() { let result = parse_mutation_leaf_descriptor(vector["raw"].as_str().unwrap().as_bytes(), &authority); assert_eq!(result.is_ok(), vector["parserAccepted"].as_bool().unwrap(), "{}: {result:?}", vector["name"]); if let Err(error) = result { assert!(error.contains(vector["diagnostic"].as_str().unwrap()), "{}: {error}", vector["name"]); } }
    }
    #[test]
    fn emits_all_core_descriptor_fields() {
        let fixture = fixture(); let authority = authority(fixture["authorityOwner"].as_str().unwrap()); let descriptor = parse_mutation_leaf_descriptor(fixture["cases"][0]["raw"].as_str().unwrap().as_bytes(), &authority).unwrap(); let emitted = emit_mutation_leaf_descriptor(&descriptor).to_string();
        for field in ["schema_version", "owner", "semantic_kind", "display_name", "emoji", "aggregate_variant", "payload_schema", "text_opcode", "binary_tag", "invertibility", "diff_participation", "outcome_classes", "composition", "required_language_surfaces"] { assert!(emitted.contains(field), "missing {field}: {emitted}"); }
        assert!(emitted.contains("MutationLeafDescriptor") && emitted.contains("ExplicitMutation") && emitted.contains("JsonSchema"));
    }
}
//#endregion 🔣️MutationLeafJsonfn main(){let f:serde_json::Value=serde_json::from_str(include_str!("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧪️tests/🧬️mutation-leaf-json/🧫️fixtures/🔣️cases.json")).unwrap();let a=MutationSourceAuthority{owner:f["authorityOwner"].as_str().unwrap().into()};let out=PathBuf::from(std::env::args().nth(1).unwrap());let mut n=0;for v in f["cases"].as_array().unwrap(){let r=parse_mutation_leaf_descriptor(v["raw"].as_str().unwrap().as_bytes(),&a);assert_eq!(r.is_ok(),v["parserAccepted"].as_bool().unwrap(),"{}: {r:?}",v["name"]);if let Err(e)=r{assert!(e.contains(v["diagnostic"].as_str().unwrap()));}else{fs::write(out.join(format!("{n}.tokens")),emit_mutation_leaf_descriptor(&r.unwrap()).to_string()).unwrap();n+=1;}}println!("[DEBUG] parser vectors={} emitted={n}",f["cases"].as_array().unwrap().len());}