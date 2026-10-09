//! 🧪️ Public canonical package and typed Flow cache witnesses.
#[path = "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧬️schema/📦️package/🦀️.rs"]
pub mod space_package;
pub use space_package::{admit_space_package_declaration,SpacePackageDeclaration,SpaceArtifactPackage,SpacePackageError};
#[path = "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🚪️io/📝️text/📦️package/🦀️.rs"]
pub mod space_io;
#[path = "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧬️schema/📦️package/🦀️.rs"]
pub mod collection_package;
pub use collection_package::{admit_collection_package_declaration,CollectionPackageDeclaration,CollectionArtifactPackage,CollectionPackageError};
#[path = "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🚪️io/📝️text/📦️package/🦀️.rs"]
pub mod collection_io;
#[cfg(feature="flow")]
#[path = "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📔️registry/🌱️seed/🦀️.rs"]
pub mod flow_seed;
#[cfg(feature="flow")]
#[path = "../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/🚪️io/📥️evaluation-response/🦀️.rs"]
pub mod flow_io;

#[cfg(test)]
mod tests {
 use super::*;
 #[cfg(feature="flow")]
 use neural_engine::ColdRetire;
 #[test]
 fn canonical_packages_match_independent_serde_json(){
  let ours=space_io::read_builtin_space_package().unwrap();
  let oracle:serde_json::Value=serde_json::from_str(space_io::SPACE_PACKAGE_DECLARATION_JSON).unwrap();
  assert_eq!(ours.id,oracle["id"].as_str().unwrap());assert_eq!(ours,space_package::package_descriptor().unwrap());
  let ours=collection_io::read_builtin_collection_package().unwrap();
  let oracle:serde_json::Value=serde_json::from_str(collection_io::COLLECTION_PACKAGE_DECLARATION_JSON).unwrap();
  assert_eq!(ours.id,oracle["id"].as_str().unwrap());assert_eq!(ours,collection_package::package_descriptor().unwrap());
  println!("[DEBUG] native package identities=2 independent serde JSON matches");
 }
 #[test]
 fn package_identity_and_unknown_fields_are_refused_before_admission(){
  for source in [space_io::SPACE_PACKAGE_DECLARATION_JSON.replace("os.space","alien"),space_io::SPACE_PACKAGE_DECLARATION_JSON.replacen("{","{\"unknown\":true,",1),space_io::SPACE_PACKAGE_DECLARATION_JSON.replace("\"id\":","\"id\":\"alien\",\"id\":")]{assert!(space_io::decode_space_package_json(&source).is_err());}
  let source=collection_io::COLLECTION_PACKAGE_DECLARATION_JSON.replace("os.collection","alien");assert!(collection_io::decode_collection_package_json(&source).is_err());
  println!("[DEBUG] native package refusal identities+unknown+duplicate verified");
 }
 #[cfg(feature="flow")]
 #[test]
 fn physical_flow_response_to_typed_cache_matches_independent_json(){
  let cache=neural_engine::NeuralCache::new();
  for source in [r#"{"schema":"number","value":42.0}"#,r#"{"schema":"exact","large":9223372036854775807,"nested":{"text":"Grüße","flag":true}}"#]{
   let output=flow_io::decode_flow_node_output_json(source).unwrap();
   flow_seed::seed_flow_eval_node_cache(&cache,17,output);
   let found=cache.get(17).unwrap();
   let oracle:serde_json::Value=serde_json::from_str(source).unwrap();
   let observed:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&found)).unwrap();
   assert_eq!(observed,oracle);found.retire_cold();
  }
  for source in ["[1,2]","{",r#"{"value":42,"value":43}"#]{assert!(flow_io::decode_flow_node_output_json(source).is_err());}
  assert_eq!(cache.len(),1);cache.retire_cold();
  println!("[DEBUG] native Flow cache outputs=2 refused=3 exact i64 preserved");
 }
}
