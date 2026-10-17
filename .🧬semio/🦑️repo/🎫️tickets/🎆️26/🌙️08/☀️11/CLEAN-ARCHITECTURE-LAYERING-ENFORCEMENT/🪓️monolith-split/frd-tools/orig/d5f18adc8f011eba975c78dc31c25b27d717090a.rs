//! 🫳️ Borrowed and owned GLTF field metadata share one portable six-carrier contract.
use semio_framework_dsl_record::DslField;
#[allow(unused_imports)]
use crate::standards::v2_0::subsets::any::schema::snapshot::*;
#[allow(unused_imports)]
use semio_framework_value::{ValueError, ValueRefusalKind};
use crate::standards::v2_0::subsets::any::io::binary::snapshot::owned_pack::{json};
use crate::standards::v2_0::subsets::any::io::binary::snapshot::owned_pack::*;
use semio_framework_dsl_record::{BorrowedDslField,BorrowedShape,Shape};

fn borrowed_kind(shape:BorrowedShape)->&'static str{match shape{BorrowedShape::List(_)=>"list",BorrowedShape::UInt=>"uint",BorrowedShape::Record(_)=>"record",BorrowedShape::Enum(_)=>"enum",BorrowedShape::Text=>"text",_=>panic!("unexpected borrowed GLTF carrier shape")}}
fn owned_kind(shape:&Shape)->&'static str{match shape{Shape::List(_)=>"list",Shape::UInt=>"uint",Shape::Record(_)=>"record",Shape::Enum(_)=>"enum",Shape::Text=>"text",_=>panic!("unexpected owned GLTF carrier shape")}}
fn observed<T:DslField+BorrowedDslField>(owner:&str)->serde_json::Value{
 let BorrowedShape::Record(borrowed)=T::SHAPE else{panic!("GLTF carrier must declare a record")};
 let Shape::Record(owned)=T::shape() else{panic!("GLTF carrier must declare a record")};
 let borrowed=borrowed();let owned=(owned.ordinary)();
 assert_eq!(borrowed.keyword,owned.keyword.as_deref());
 assert_eq!(borrowed.layout,owned.layout);
 let fields=borrowed.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"optional":field.optional,"kind":borrowed_kind(field.shape)})).collect::<Vec<_>>();
 let owned=owned.fields.iter().map(|field|serde_json::json!({"id":field.id,"key":field.key,"optional":field.optional,"kind":owned_kind(&field.shape)})).collect::<Vec<_>>();
 assert_eq!(fields,owned,"{owner} owned/borrowed metadata differ");
 serde_json::json!({"owner":owner,"fields":fields})
}
#[test]
fn borrowed_gltf_carriers_match_the_neutral_owned_metadata_and_serde_oracle(){
 let expected:serde_json::Value=serde_json::from_str(runtime_fixture_text("../../../../../🚪️io/💾️binary/📸️snapshot/📦️pack/🧪️tests/🫳️borrowed-carriers/🔣️.json")).unwrap();
 let actual=serde_json::json!({"cases":[observed::<GltfJson>("json"),observed::<GltfMorphTarget>("morph-target"),observed::<GltfPrimitive>("primitive"),observed::<GltfCameraProjection>("camera-projection"),observed::<GltfImage>("image"),observed::<GltfTexture>("texture")]});
 assert_eq!(actual,expected);
 assert_eq!(serde_json::from_slice::<serde_json::Value>(&serde_json::to_vec(&actual).unwrap()).unwrap(),expected);
}

fn runtime_fixture_text(relative: &str) -> &'static str {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
    Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
}
