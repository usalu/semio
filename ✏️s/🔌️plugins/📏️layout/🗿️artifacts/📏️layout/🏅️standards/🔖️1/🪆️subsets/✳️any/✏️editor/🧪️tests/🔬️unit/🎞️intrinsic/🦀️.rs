//! 🎞️ Real VCS ports preserve the configured form owner through an admitted Layout import.
use super::*;
use semio_framework_plugin::{MediaArtifact,MediaArtifactDescriptor,MediaArtifactError,MediaWireFormat,PluginApp};
use semio_framework_value::{DslValue,Number,ToValue,FromValue,ValueRefusalKind};
use semio_s_artifact_forms_forms::{FormsSnapshot,FormStep,FormQuestion};
use semio_s_artifact_forms_forms::op::FormMutation;
use semio_s_artifact_forms_forms::editor::forms::{FormsPlayApp,create_forms_app};

type LiveForms=semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<FormsPlayApp>,semio_s_artifact_stdio_semio::SemioMembers>;
struct LiveFormsOwner(LiveForms);
impl Drop for LiveFormsOwner{
 fn drop(&mut self){if !std::thread::panicking(){semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut self.0);}}
}
fn live_forms_manifest()->semio_framework_plugin::App{semio_framework_plugin::App{definition:create_forms_app(),examples:Vec::new()}}
fn live_intrinsic_value(value:&serde_json::Value)->DslValue{
 match value["kind"].as_str().unwrap(){
  "null"=>DslValue::Null,
  "boolean"=>DslValue::Bool(value["value"].as_bool().unwrap()),
  "unsigned"=>DslValue::Number(Number::UInt(value["value"].as_str().unwrap().parse().unwrap())),
  "signed"=>DslValue::Number(Number::Int(value["value"].as_str().unwrap().parse().unwrap())),
  "float"=>DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(value["bits"].as_str().unwrap(),16).unwrap()))),
  "text"=>DslValue::String(value["value"].as_str().unwrap().into()),
  "bytes"=>DslValue::Bytes(value["value"].as_array().unwrap().iter().map(|byte|u8::try_from(byte.as_u64().unwrap()).unwrap()).collect()),
  "array"=>DslValue::Array(value["items"].as_array().unwrap().iter().map(live_intrinsic_value).collect()),
  "object"=>DslValue::Object(value["members"].as_array().unwrap().iter().map(|member|(member["name"].as_str().unwrap().into(),live_intrinsic_value(&member["value"]))).collect()),
  other=>panic!("unknown closed live fixture value {other}"),
 }
}
fn live_question(id:String,value:DslValue)->FormQuestion{
 FormQuestion{id,label:"Literal configured value".into(),kind:"text".into(),description:None,required:None,placeholder:None,default:Some(value),min:None,max:None,step:None,unit:None,text:None,options:None,fields:None,schema:None,src:None,accept:None,fixture_slug:None,params:None,condition:None}
}
#[semio_framework_async_macros::async_test]
async fn live_vcs_forms_dictionary_out_to_layout_fields_in_retains_complete_intrinsic_owner(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🧾️dictionary/🔣️.json")).unwrap();
 let contract=&fixture["liveVcs"];
 let mut values:Vec<DslValue>=fixture["dictionary"]["entries"].as_array().unwrap().iter().map(|entry|live_intrinsic_value(&entry["value"])).collect();
 values.extend(fixture["floatWords"].as_array().unwrap().iter().map(|word|DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(word.as_str().unwrap(),16).unwrap())))));
 let entries:Vec<crate::FormDictionaryEntry>=contract["questionIds"].as_array().unwrap().iter().zip(values).map(|(id,value)|crate::FormDictionaryEntry{question_id:id.as_str().unwrap().into(),value}).collect();
 let expected=crate::FormDictionary{entries};
 expected.validate().expect("unique actual question identities");
 let steps=vec![FormStep{id:"live-step".into(),title:"Intrinsic defaults".into(),description:None,blocks:expected.entries.iter().map(|entry|live_question(entry.question_id.clone(),entry.value.clone())).collect()}];
 let snapshot=semio_s_artifact_forms_forms::forms_snapshot_with_state("forms.form".into(),"live-form".into(),"1".into(),Some("Full intrinsic defaults".into()),&steps);
 let mut envelope=store::create_document_envelope::<FormsSnapshot,FormMutation>("forms.form","live-form",snapshot,None).into_owners();
 envelope.dialect=Some(<FormsPlayApp as semio_framework_plugin::ArtifactEditor>::DIALECT.into());
 let files=store::print_document_pack(&envelope).await.expect("actual typed form document pack");
 let mut producer=LiveFormsOwner(semio_framework_plugin::artifact_app_laws::new_app_with_registry_and_members::<semio_framework_plugin::EditorApp<FormsPlayApp>,semio_s_artifact_stdio_semio::SemioMembers>(live_forms_manifest).await);
 producer.0.bind_instance_id(context::INSTANCE).await;
 semio_framework_plugin::artifact_app_laws::load_document(&mut producer.0, &files).await.expect("real VCS form load");
 assert_eq!(semio_s_artifact_forms_forms::schema::configured_dictionary(&producer.0.snapshot().unwrap()).unwrap(),expected);
 let artifact=producer.0.produce_media(contract["output"].as_str().unwrap()).await.expect("actual dictionary output");
 assert_eq!(artifact.descriptor.port_id.as_deref(),Some("dictionary:out"));
 assert_eq!(artifact.descriptor.media_type,Some(MediaType{class:MediaClass::Data,form:MediaForm::Value}));
 assert!(matches!(&artifact.descriptor.wire,MediaWireFormat::Intrinsic{schema}if schema=="form.dictionary"));
 assert_eq!(artifact.descriptor.blob_hash,None);
 let descriptor_bytes=store::pack_rt::encode_wire_value(&artifact.descriptor.to_value());
 let descriptor=MediaArtifactDescriptor::from_value(store::pack_rt::decode_wire_value(&descriptor_bytes).expect("descriptor wire input")).expect("typed descriptor");
 let transported=MediaArtifact{descriptor,data:artifact.data};
 let mut target=context::layout_app_with_registry().await;
 let before=target.snapshot().expect("actual Layout owner");
 let revision=target.test_document_revision();
 let mut malformed=transported.clone();malformed.data.push(0);
 let error=target.consume_media(contract["input"].as_str().unwrap(),malformed).await.unwrap_err();
 assert!(matches!(error,MediaArtifactError::Value(error)if error.kind==ValueRefusalKind::InvalidValue));
 assert_eq!(target.test_document_revision(),revision);
 assert_eq!(target.snapshot().unwrap(),before);
 target.consume_media(contract["input"].as_str().unwrap(),transported).await.expect("real reserved import job and document emit");
 assert_ne!(target.test_document_revision(),revision);
 let mut complete=before;complete.data_fields=Some(expected);
 assert_eq!(target.snapshot().expect("completed target snapshot"),complete);
 semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut target.0);
 semio_framework_plugin::artifact_app_laws::close_registered_fixture_app(&mut producer.0);
}
