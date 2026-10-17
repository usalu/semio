//! 🧪️ Neutral imported image, exact inverse and structural codec contract.
use crate::{JpgSnapshot, JpgImage, JpgMutation};
use crate::standards::v_jfif_1_01::subsets::document::schema::mutations::replace_image::*;
use protocol::{Mutation,DiffAlgebra,OpBinary,OpText};
#[test]
fn imported_image_intent_preserves_every_semantic_owner_and_inverse(){
 let raw:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../🧾️document/🧪️tests/🧫️fixtures/🧬️mutations/🖼️replace-image/🔣️.json")).unwrap();
 let initial:JpgImage=semio_framework_pack_json::from_json_str(&raw["initialImage"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let image:JpgImage=semio_framework_pack_json::from_json_str(&raw["importedImage"].to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
 let base=JpgSnapshot{schema:crate::STDIO_JPG_DOCUMENT_SCHEMA.into(),image:initial};
 let mutation=JpgMutation::ReplaceImage(ReplaceImage{image:image.clone()});
 let applied=protocol::apply_diff(mutation.diff(&base).diff(),&base).unwrap();assert_eq!(applied.image,image);
 let independent:serde_json::Value=serde_json::from_str(&semio_framework_pack_json::to_json_string(&applied.image)).unwrap();
 assert_eq!(independent,raw["importedImage"]);
 let inverse=mutation.inverse(&base).unwrap();let restored=protocol::apply_diff(inverse[0].diff(&applied).diff(),&applied).unwrap();assert_eq!(restored,base);
 assert_eq!(JpgMutation::decode_op(&mutation.encode_op().unwrap()).unwrap(),mutation);
 assert_eq!(JpgMutation::parse_op(&mutation.print_op()).unwrap(),mutation);
 assert!(mutation.diff(&applied).diff().is_empty());
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
