#[derive(semio_framework_dsl_record_derive::DslScalar)]
enum AuthoredBorrowedScalar{Ready,#[dsl(key="finished")]Done}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="leaf")]
struct AuthoredBorrowedLeaf{text:String}
#[derive(semio_framework_dsl_record_derive::DslEnum)]
enum AuthoredBorrowedVariant{Document{text:String},#[dsl(key="vacant")]Empty,#[dsl(key="alias")]Leaf(Box<AuthoredBorrowedLeaf>)}
#[derive(semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword="owner",layout="lines")]
struct AuthoredBorrowedOwner{
 #[dsl(key="identity",positional,defines="owner")]id:String,
 maybe:Option<i64>,
 #[dsl(list)]list:Vec<u64>,
 #[dsl(tuple)]tuple:Vec<f64>,
 #[dsl(statements)]many:Vec<AuthoredBorrowedVariant>,
 #[dsl(statements,block)]nested:Vec<AuthoredBorrowedVariant>,
 map:std::collections::BTreeMap<String,bool>,
 #[dsl(statements)]optional:Option<AuthoredBorrowedVariant>,
 #[dsl(statements)]required:Box<AuthoredBorrowedVariant>,
 #[dsl(base64)]bytes:Vec<u8>,
 #[dsl(table)]rows:Vec<AuthoredBorrowedLeaf>,
 #[dsl(unit="m")]length:f64,
 #[dsl(angle="°")]rotation:f64,
 #[dsl(refs="owner")]reference:String,
 #[dsl(key="language-key")]language:String,
 #[dsl(lang_from="language")]code:String,
 #[dsl(lang="jack")]jack:String,
 #[dsl(coord)]coord:[f64;3],
 #[dsl(dir)]direction:[f64;3],
 scalar:AuthoredBorrowedScalar,
 recursive:std::collections::BTreeMap<String,AuthoredBorrowedOwner>,
}
fn authored_borrowed_kind(shape:crate::BorrowedShape)->&'static str{
 use crate::BorrowedShape as H;
 match shape{
 H::Text=>"Text",H::Int=>"Int",H::UInt=>"UInt",H::Float=>"Float",H::Bool=>"Bool",
 H::List(inner)=>{assert!(matches!(inner(),H::UInt));"List(UInt)"},
 H::Tuple(inner,None)=>{assert!(matches!(inner(),H::Float));"Tuple(Float)"},
 H::Statements(_)=>"Statements",
 H::Block(inner)=>{assert!(matches!(inner(),H::Statements(_)));"Block(Statements)"},
 H::Map(inner)=>match inner(){H::Bool=>"Map(Bool)",H::Record(make)=>{assert_eq!(make().keyword,Some("owner"));"Map(Record)"},_=>panic!("authored map shape")},
 H::Bytes64=>"Bytes64",H::Table(make)=>{assert_eq!(make().keyword,Some("leaf"));"Table"},
 H::Quantity(unit)=>{assert_eq!(unit.symbol,"m");"Quantity(m)"},H::Angle(unit)=>{assert_eq!(unit.symbol,"°");"Angle(°)"},
 H::Ref("owner")=>"Ref(owner)",H::EmbedFrom("language-key")=>"EmbedFrom(language-key)",H::Embed("jack")=>"Embed(jack)",H::Coord(3)=>"Coord(3)",H::Dir=>"Dir",H::Enum(_)=>"Enum",
 _=>panic!("closed authored schema shape"),
 }
}
#[test]
fn child_authored_borrowed_schema_preserves_static_metadata_and_lazy_owner_edges(){
 use crate::{BorrowedDslField,BorrowedDslRecord,BorrowedDslVariants,BorrowedShape as H};
 const SPEC:crate::BorrowedRecordSpec=<AuthoredBorrowedOwner as BorrowedDslRecord>::RECORD;
 let fixture:serde_json::Value=serde_json::from_str(include_str!("@BORROWED_AUTHORED_FIXTURE@")).unwrap();
 assert_eq!(SPEC.keyword,fixture["record"]["keyword"].as_str());assert_eq!(SPEC.layout,RecordLayout::Lines);assert_eq!(SPEC.fields.len(),fixture["record"]["fields"].as_array().unwrap().len());
 for(field,expected)in SPEC.fields.iter().zip(fixture["record"]["fields"].as_array().unwrap()){
  assert_eq!(field.id as u64,expected["id"].as_u64().unwrap());assert_eq!(field.key,expected["key"].as_str().unwrap());assert_eq!(field.position.map(u64::from),expected["position"].as_u64());assert_eq!(field.optional,expected["optional"].as_bool().unwrap());assert_eq!(field.defines,expected["defines"].as_str());assert!(!field.flatten&&!field.is_call_name);assert_eq!(authored_borrowed_kind(field.shape),expected["kind"].as_str().unwrap());
 }
 let H::Quantity(length)=SPEC.fields[11].shape else{panic!("quantity owner")};let H::Angle(angle)=SPEC.fields[12].shape else{panic!("angle owner")};assert!(std::ptr::eq(length,semio_framework_dsl::unit_by_symbol("m").unwrap()));assert!(std::ptr::eq(angle,semio_framework_dsl::unit_by_symbol("°").unwrap()));
 let H::Enum(labels)=<AuthoredBorrowedScalar as BorrowedDslField>::SHAPE else{panic!("scalar labels")};for((key,ordinal),expected)in labels.iter().zip(fixture["scalar"].as_array().unwrap()){assert_eq!(*key,expected["key"].as_str().unwrap());assert_eq!(*ordinal as u64,expected["ordinal"].as_u64().unwrap());}assert_eq!(labels.len(),2);
 let variants=<AuthoredBorrowedVariant as BorrowedDslVariants>::VARIANTS;assert_eq!(variants.len(),3);
 for((key,make),expected)in variants.iter().zip(fixture["variants"].as_array().unwrap()){assert_eq!(*key,expected["key"].as_str().unwrap());let spec=make();assert_eq!(spec.keyword,expected["recordKeyword"].as_str());assert_eq!(spec.fields.len(),expected["fields"].as_array().unwrap().len());for(field,expected)in spec.fields.iter().zip(expected["fields"].as_array().unwrap()){assert_eq!(field.id as u64,expected["id"].as_u64().unwrap());assert_eq!(field.key,expected["key"].as_str().unwrap());assert_eq!(authored_borrowed_kind(field.shape),expected["kind"].as_str().unwrap());}}
 let values=[AuthoredBorrowedVariant::Document{text:String::new()},AuthoredBorrowedVariant::Empty,AuthoredBorrowedVariant::Leaf(Box::new(AuthoredBorrowedLeaf{text:String::new()}))];
 const PRODUCER:crate::BorrowedRecordSpecProducer=crate::BorrowedRecordSpecProducer::of::<AuthoredBorrowedOwner>();
 let mut accepted=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(0,&mut accepted);
 let(_,requests)=observe_requests(||{
  for _ in 0..fixture["readRepetitions"].as_u64().unwrap(){
   let produced=PRODUCER.encode(&mut control).unwrap();assert_eq!(produced.fields.as_ptr(),SPEC.fields.as_ptr());
   let repeated=<AuthoredBorrowedOwner as BorrowedDslRecord>::RECORD;assert_eq!(repeated.fields.as_ptr(),SPEC.fields.as_ptr());for field in repeated.fields{std::hint::black_box(authored_borrowed_kind(field.shape));}
   let H::Map(inner)=repeated.fields[20].shape else{panic!("recursive owner")};let H::Record(make)=inner()else{panic!("lazy record")};assert_eq!(make().fields.as_ptr(),SPEC.fields.as_ptr());
   for(index,value)in values.iter().enumerate(){let(key,ordinal,spec)=value.projected_borrowed_variant_identity();assert_eq!(ordinal,index);assert_eq!(key,variants[index].0);assert_eq!(spec.fields.as_ptr(),variants[index].1().fields.as_ptr());}
  }
 });
 assert_eq!(control.owned_bytes(),0);
 let mut refused=|_|false;let mut canceled=semio_framework_value::NativeEncodeControl::new(0,&mut refused);assert_eq!(PRODUCER.encode(&mut canceled).err().unwrap().kind,semio_framework_value::ValueRefusalKind::Canceled);assert_eq!(canceled.owned_bytes(),0);
 assert_eq!(requests.bytes,fixture["expectedAllocatedBytes"].as_u64().unwrap()as usize);assert_eq!(requests.length,0);
 eprintln!("[DEBUG] actual authored static schema fields21 kinds11 refinements7 variants3 repeated256 actual allocator requests0; recursive/source pointers preserved");
}
