//! 🧪️ Every native norm row proves its AOT declaration against current canonical schema bytes and an independent reader invocation.
use super::*;
use semio_framework_value::{DslValue, FromValue, ToValue};

/// 🧬️ Cold oracle for the complete family roster; this reader is never used by interactive preparation.
pub fn norm_compiled_input_findings<A: NormRetainedEditor>() -> Vec<String> {
    use protocol::Mutation;
    let descriptors = A::Mutation::DESCRIPTORS;
    let schemas = A::Mutation::INPUT_SCHEMAS;
    let documents = A::Mutation::INPUT_SCHEMA_DOCUMENTS;
    let compiled = A::Mutation::COMPILED_INPUTS;
    let mut findings = Vec::new();
    if descriptors.len() != schemas.len() || descriptors.len() != documents.len() || descriptors.len() != compiled.len() {
        return vec!["compiled declarations, schema documents and native descriptor rows differ".into()];
    }
    for (index, descriptor) in descriptors.iter().enumerate() {
        let name = format!("{} row {index}", descriptor.semantic_kind);
        let Some(compiled) = compiled[index].as_ref() else {
            findings.push(format!("{name}: no compiled input source")); continue;
        };
        if compiled.semantic_kind != descriptor.semantic_kind || compiled.provenance.owner != descriptor.owner {
            findings.push(format!("{name}: compiled source identity differs"));
        }
        if compiled.schema_sha256 != semio_framework_hash::Sha256::digest(schemas[index].as_bytes()) {
            findings.push(format!("{name}: canonical payload schema fingerprint differs"));
        }
        if compiled.document_sha256.len() != documents[index].len() || compiled.document_sha256.iter().zip(documents[index]).any(|(digest, text)| *digest != semio_framework_hash::Sha256::digest(text.as_bytes())) {
            findings.push(format!("{name}: native referenced document fingerprints differ"));
        }
        if compiled.framework_document_sha256.len() != semio_framework::FRAMEWORK_INPUT_SCHEMA_DOCUMENTS.len() || compiled.framework_document_sha256.iter().zip(semio_framework::FRAMEWORK_INPUT_SCHEMA_DOCUMENTS).any(|(digest, text)| *digest != semio_framework_hash::Sha256::digest(text.as_bytes())) {
            findings.push(format!("{name}: framework document fingerprints differ"));
        }
        let raw = match compiled.definitions_json { Ok(json) => json, Err(error) => { findings.push(format!("{name}: {} at {}: {}", error.code, error.pointer, error.detail)); continue; } };
        let inputs = semio_framework_pack_json::parse(raw, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| error.to_string()).and_then(|value| Vec::<semio_framework::ActionArgDef>::from_value(value).map_err(|error| error.to_string()));
        let inputs = match inputs { Ok(inputs) => inputs, Err(error) => { findings.push(format!("{name}: compiled input JSON admission failed: {error}")); continue; } };
        let expected = semio_framework::mutation_input_defs(schemas[index], &|id: &str| {
            documents[index].iter().find_map(|text| {
                let document = semio_framework_pack_json::parse(text, semio_framework_pack_json::JsonMemberPolicy::Reject).ok()?;
                let document = semio_framework_pack_json::to_dsl_value(&document);
                (document.get("$id").and_then(DslValue::as_str) == Some(id)).then_some(document)
            }).or_else(|| semio_framework::registered_input_schema_document(id))
        });
        match &expected {
            Ok(expected) if expected.to_value() == inputs.to_value() => {},
            Ok(_) => findings.push(format!("{name}: compiled declarations differ from the current synchronous reader")),
            Err(error) => findings.push(format!("{name}: current native input reader refuses: {error}")),
        }
        let raw = match compiled.seed_definitions_json { Ok(json) => json, Err(error) => { findings.push(format!("{name}: seed {} at {}: {}", error.code, error.pointer, error.detail)); continue; } };
        let seeds = semio_framework_pack_json::parse(raw, semio_framework_pack_json::JsonMemberPolicy::Reject).map(|value| semio_framework_pack_json::to_dsl_value(&value)).map_err(|error| error.to_string()).and_then(|value| Vec::<semio_framework::ActionArgShapeSeed>::from_value(value).map_err(|error| error.to_string()));
        let seeds = match seeds { Ok(seeds) => seeds, Err(error) => { findings.push(format!("{name}: compiled seed JSON admission failed: {error}")); continue; } };
        if let Err(error) = semio_framework::validate_action_arg_shape_seeds(&inputs, &seeds) { findings.push(format!("{name}: compiled seed graph is invalid: {error}")); }
        if let Ok(expected) = expected {
            let expected = cold_seeds(&expected);
            if expected.to_value() != seeds.to_value() { findings.push(format!("{name}: compiled seeds differ from the current cold inspector seed oracle")); }
        }

    }
    findings
}


fn cold_seeds(inputs:&[semio_framework::ActionArgDef])->Vec<semio_framework::ActionArgShapeSeed>{
 fn node(schema:&semio_framework::ArgSchema,path:&mut Vec<usize>,key:Option<String>,value:DslValue,output:&mut Vec<semio_framework::ActionArgShapeSeed>){
  let variant_selector=match schema{semio_framework::ArgSchema::Object{fields}=>semio_plugin::app::time_travel::time_travel_variant_selector(fields).and_then(|selector|fields.iter().position(|field|std::ptr::eq(field,selector))),_=>None};
  let singleton=match schema{semio_framework::ArgSchema::Array{items,..}=>Some(DslValue::Array(vec![crate::norm_inspector_seed(items)])),semio_framework::ArgSchema::Reference{many:true,..}=>{let mut item=schema.clone();if let semio_framework::ArgSchema::Reference{many,..}=&mut item{*many=false}Some(DslValue::Array(vec![crate::norm_inspector_seed(&item)]))},_=>None};
  let own=output.len();output.push(semio_framework::ActionArgShapeSeed{schema_path:path.clone(),value,key,children:Vec::new(),variant_selector,singleton});
  match schema{
   semio_framework::ArgSchema::Object{fields}=>for(index,input)in fields.iter().enumerate(){let child=output.len();output[own].children.push(child);path.push(index);node(&input.schema,path,Some(input.key()),input.default.clone().unwrap_or_else(||crate::norm_inspector_seed(&input.schema)),output);path.pop();},
   semio_framework::ArgSchema::Array{items,..}=>{let child=output.len();output[own].children.push(child);path.push(0);node(items,path,None,crate::norm_inspector_seed(items),output);path.pop();},
   _=>{},
  }
 }
 let mut output=Vec::new();
 for(index,input)in inputs.iter().enumerate(){node(&input.schema,&mut vec![index],Some(input.key()),input.default.clone().unwrap_or_else(||crate::norm_inspector_seed(&input.schema)),&mut output)}
 output
}
