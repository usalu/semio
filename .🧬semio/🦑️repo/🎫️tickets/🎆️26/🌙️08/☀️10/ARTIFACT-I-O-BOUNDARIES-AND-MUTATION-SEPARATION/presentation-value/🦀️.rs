#[path="../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🚪️io/📝️text/🧩️presentation/🌱️value-text/🦀️.rs"]
mod value_text;
#[test]
fn controlled_presentation_values_agree_with_independent_serde() {
 use semio_framework_value::{DslValue,NativeEncodeControl,ValueRefusalKind};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🚪️io/📝️text/🧩️presentation/🌱️value-text/🧫️fixtures/🔣️.json")).unwrap();
 let mut callbacks=0;
 for expected in fixture["values"].as_array().unwrap() {
  let value=semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&expected.to_string(),semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
  let before=value.clone();let mut accept=|_|{callbacks+=1;true};let mut control=NativeEncodeControl::new(fixture["maximumBytes"].as_u64().unwrap()as usize,&mut accept);
  let output=value_text::generation_value_input_text_v1(&value,&mut control).unwrap();
  assert_eq!(serde_json::from_str::<serde_json::Value>(&output).unwrap(),*expected);assert_eq!(value,before);
 }
 let original=DslValue::String("preserved 🧬️".into());let pointer=original.as_str().unwrap().as_ptr();
 let mut cancel=|_|false;assert_eq!(value_text::generation_value_input_text_v1(&original,&mut NativeEncodeControl::new(65536,&mut cancel)).unwrap_err().kind,ValueRefusalKind::Canceled);
 let mut accept=|_|true;assert_eq!(value_text::generation_value_input_text_v1(&original,&mut NativeEncodeControl::new(0,&mut accept)).unwrap_err().kind,ValueRefusalKind::OwnershipLimit);assert_eq!(original.as_str().unwrap().as_ptr(),pointer);
 println!("[DEBUG] Controlled presentation: values={} callbackObservations={} cancellations=1 allocationRefusals=1 originalPointerPreserved=true independentSerde=true",fixture["values"].as_array().unwrap().len(),callbacks);
}
