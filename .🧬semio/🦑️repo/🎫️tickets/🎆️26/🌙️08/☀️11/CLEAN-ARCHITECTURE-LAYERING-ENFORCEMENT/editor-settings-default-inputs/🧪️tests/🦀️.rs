
#[derive(serde::Serialize,serde::Deserialize)]
#[serde(default,rename_all="camelCase",deny_unknown_fields)]
struct SettingsOracle {font_px:f64,line_height:f64,show_line_numbers:bool,tab_size:usize}
impl Default for SettingsOracle {
 fn default()->Self {
  let corpus:Value=serde_json::from_str(include_str!("../🧫️fixtures/⚙️settings/🔣️.json")).unwrap();let defaults=&corpus["defaults"];
  Self {font_px:defaults["fontPx"].as_f64().unwrap(),line_height:defaults["lineHeight"].as_f64().unwrap(),show_line_numbers:defaults["showLineNumbers"].as_bool().unwrap(),tab_size:defaults["tabSize"].as_u64().unwrap() as usize}
 }
}

#[test]
fn canonical_scene_settings_defaults_match_the_independent_serde_oracle_and_each_cancellation_boundary() {
 let corpus:Value=serde_json::from_str(include_str!("../🧫️fixtures/⚙️settings/🔣️.json")).unwrap();
 for row in corpus["cases"].as_array().unwrap() {
  let oracle=serde_json::from_value::<SettingsOracle>(row["input"].clone());assert_eq!(oracle.is_ok(),row["accepted"].as_bool().unwrap(),"{} serde",row["id"]);
  if let Ok(oracle)=oracle {assert_eq!(serde_json::to_value(oracle).unwrap(),row["expected"],"{} fixture oracle",row["id"]);}
  let input=serde_json::json!({"settings":row["input"]});
  for format in [IntrinsicFormat::Body,IntrinsicFormat::Document] {
   let bytes=encoded(DslValue::from(&input),format);let mut accept=|_|true;let mut control=NativeDecodeControl::new(16_777_216,&mut accept);
   let actual=decode(&bytes,format,&Default::default(),&mut control);assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap(),"{} {format:?}",row["id"]);
   if let Ok(scene)=actual {assert_eq!(Value::from(scene.to_value()),serde_json::json!({"settings":row["expected"]}),"{} {format:?}",row["id"]);}
  }
  let mut calls=0;let mut accept=|_|{calls+=1;true};let mut control=NativeDecodeControl::new(16_777_216,&mut accept);
  let actual=from_json(&input.to_string(),&mut control);assert_eq!(actual.is_ok(),row["accepted"].as_bool().unwrap(),"{} JSON",row["id"]);drop(control);
  if row["accepted"].as_bool().unwrap() {for stop in 1..=calls {let mut observed=0;let mut cancel=|_|{observed+=1;observed!=stop};let mut control=NativeDecodeControl::new(16_777_216,&mut cancel);assert!(from_json(&input.to_string(),&mut control).is_err(),"{} cancellation {stop}",row["id"]);}}
 }
 eprintln!("[DEBUG] Editor eight controlled-default schedules agree with serde_json across JSON, Body and Document and every JSON cancellation boundary");
}
