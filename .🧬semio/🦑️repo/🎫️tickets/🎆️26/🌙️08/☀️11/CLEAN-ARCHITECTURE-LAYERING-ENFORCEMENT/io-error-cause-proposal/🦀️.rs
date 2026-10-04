//! 🚪️ IO cause retains refusal identity and authored text source data.
use crate::IoError;
use semio_framework_value::{ValueError,ValueRefusalKind,NativeEncodeControl};
use semio_framework_diagnostic::{TextError,TextSpan};
#[test]
fn io_owned_cause_retains_kind_and_text_diagnostics() {
    let kinds=[ValueRefusalKind::InvalidValue,ValueRefusalKind::Canceled,ValueRefusalKind::OwnershipLimit,ValueRefusalKind::AllocationFailed,ValueRefusalKind::WorkLimit,ValueRefusalKind::DepthLimit,ValueRefusalKind::UnsupportedOwner,ValueRefusalKind::InvariantViolated];
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let kind=*kinds.iter().find(|kind|kind.as_str()==row["kind"].as_str().unwrap()).unwrap();
        let message=row["message"].as_str().unwrap().to_owned();
        let mut accept=|_|row["operation"]!="textCanceled";
        let mut control=NativeEncodeControl::new(if row["operation"]=="textOwnership"{0}else{1048576},&mut accept);
        let operation=row["operation"].as_str().unwrap();
        if operation=="textCanceled"||operation=="textOwnership" {
            let refusal=IoError::from_text_error_controlled(TextError::expected(kind,message,TextSpan{line:7,column:11,length:4},"required"),&mut control).unwrap_err();
            assert_eq!(refusal.kind.as_str(),row["expectedKind"].as_str().unwrap());
            continue;
        }
        let original=ValueError::new(kind,message.clone());
        let pointer=original.message.as_ptr();
        let error=if operation=="text" {IoError::from_text_error_controlled(TextError::expected(kind,message.clone(),TextSpan{line:7,column:11,length:4},"required"),&mut control).unwrap()}else{IoError::from_value_error(original)};
        if operation=="value" {assert_eq!(error.cause.message.as_ptr(),pointer);}
        assert_eq!(error.cause.kind,kind);
        assert_eq!(error.cause.message,message);
        if row["operation"]=="text" {
            assert_eq!(error.diagnostics.len(),1);
            assert_eq!(error.diagnostics[0].span,TextSpan{line:7,column:11,length:4});
            assert_eq!(error.diagnostics[0].message,message);
            assert_eq!(error.diagnostics[0].expected.as_ref().unwrap().describe(),"required");
        } else {assert!(error.diagnostics.is_empty());}
    }
    let copy_cases:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧵️stress/🔣️.json")).unwrap();
    for row in copy_cases.as_array().unwrap() {
        let message=row["unit"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        let threshold=row["cancelAfterCompleted"].as_u64().unwrap() as usize;
        let mut observed=Vec::new();
        let mut progress=|event:semio_framework_value::native_encoding::NativeEncodeProgress|{observed.push(event);threshold==0||event.total<=1||event.completed<threshold};
        let mut control=NativeEncodeControl::new(row["maximumBytes"].as_u64().unwrap() as usize,&mut progress);
        let result=IoError::from_text_error_controlled(TextError::expected(ValueRefusalKind::InvariantViolated,message.clone(),TextSpan{line:7,column:11,length:4},"required"),&mut control);
        if row["expectedKind"]=="success" {let output=result.unwrap();assert_eq!(output.cause.kind,ValueRefusalKind::InvariantViolated);assert_eq!(output.cause.message,message);assert_eq!(output.diagnostics[0].message,message);assert_eq!(output.diagnostics[0].span,TextSpan{line:7,column:11,length:4});}
        else {assert_eq!(result.unwrap_err().kind.as_str(),row["expectedKind"].as_str().unwrap());}
        drop(control);
        if threshold!=0 {assert!(observed.iter().any(|event|event.total>1&&event.completed>=threshold));}
        eprintln!("[DEBUG] IO long source {} observed {} controlled frontiers",row["id"].as_str().unwrap(),observed.len());
    }
}

fn fixture_value(value: &serde_json::Value) -> semio_framework_value::DslValue {
    use semio_framework_value::{DslValue,Number};
    match value {
        serde_json::Value::Null=>DslValue::Null,
        serde_json::Value::Bool(value)=>DslValue::Bool(*value),
        serde_json::Value::Number(value)=>DslValue::Number(value.as_u64().map(Number::UInt).or_else(||value.as_i64().map(Number::Int)).unwrap()),
        serde_json::Value::String(value)=>DslValue::String(value.clone()),
        serde_json::Value::Array(values)=>DslValue::Array(values.iter().map(fixture_value).collect()),
        serde_json::Value::Object(fields)=>DslValue::Object(fields.iter().map(|(key,value)|(key.clone(),fixture_value(value))).collect()),
    }
}
#[test]
fn io_cause_controlled_wire_retains_all_authorities_and_source_data() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let Some(expected)=row.get("expectedWire")else{continue};
        let expected=fixture_value(expected);
        let mut accept=|_|true;
        let decoded=IoError::from_value_controlled(&expected,&mut semio_framework_value::NativeDecodeControl::new(1048576,&mut accept)).unwrap();
        assert_eq!(decoded.cause.kind.as_str(),row["kind"].as_str().unwrap());
        assert_eq!(decoded.cause.message,row["message"].as_str().unwrap());
        let mut accept=|_|true;
        let encoded=decoded.to_value_controlled(&mut NativeEncodeControl::new(1048576,&mut accept)).unwrap();
        let mut accept=|_|true;
        let roundtrip=IoError::from_value_controlled(&encoded,&mut semio_framework_value::NativeDecodeControl::new(1048576,&mut accept)).unwrap();
        assert_eq!(roundtrip,decoded);
        assert_eq!(fixture_output(&encoded),row["expectedWire"]);
    }
}
#[test]
fn io_cause_wire_refuses_closed_members_and_hostile_controls() {
    use semio_framework_value::{DslValue,NativeDecodeControl};
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔁️wire/🔣️.json")).unwrap();
    for row in rows.as_array().unwrap() {
        let input=DslValue::Object(row["members"].as_array().unwrap().iter().map(|member|(member[0].as_str().unwrap().to_owned(),fixture_value(&member[1]))).collect());
        let accept=row["accept"].as_bool().unwrap();
        let maximum=usize::try_from(row["maximumBytes"].as_u64().unwrap()).unwrap();
        let refusal=if row["operation"]=="encode" {
            let mut progress=|_|accept;
            IoError::from_value_error(ValueError::new(ValueRefusalKind::WorkLimit,"canceled spoofed prose")).to_value_controlled(&mut NativeEncodeControl::new(maximum,&mut progress)).unwrap_err()
        } else {
            let mut progress=|_|accept;
            IoError::from_value_controlled(&input,&mut NativeDecodeControl::new(maximum,&mut progress)).unwrap_err()
        };
        assert_eq!(refusal.kind.as_str(),row["expectedKind"].as_str().unwrap());
    }
}

fn fixture_output(value: &semio_framework_value::DslValue) -> serde_json::Value {
    use semio_framework_value::{DslValue,Number};
    match value {
        DslValue::Null=>serde_json::Value::Null,
        DslValue::Bool(value)=>serde_json::Value::Bool(*value),
        DslValue::Number(Number::UInt(value))=>serde_json::Value::Number((*value).into()),
        DslValue::Number(Number::Int(value))=>serde_json::Value::Number((*value).into()),
        DslValue::Number(Number::Float(value))=>serde_json::Value::Number(serde_json::Number::from_f64(*value).unwrap()),
        DslValue::String(value)=>serde_json::Value::String(value.clone()),
        DslValue::Array(values)=>serde_json::Value::Array(values.iter().map(fixture_output).collect()),
        DslValue::Object(fields)=>serde_json::Value::Object(fields.iter().map(|(key,value)|(key.clone(),fixture_output(value))).collect()),
        _=>panic!("closed IO fixture primitive"),
    }
}
