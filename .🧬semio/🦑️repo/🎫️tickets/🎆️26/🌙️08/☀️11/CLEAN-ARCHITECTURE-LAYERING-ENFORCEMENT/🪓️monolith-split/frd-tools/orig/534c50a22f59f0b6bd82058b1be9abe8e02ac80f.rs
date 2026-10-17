//! 🖊️ Typed stroke endings, joins and bounded editable dash patterns.
#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub enum StrokeCap { Butt, Round, Square }

impl StrokeCap {
    pub const fn as_str(self) -> &'static str {
        match self { Self::Butt => "butt", Self::Round => "round", Self::Square => "square" }
    }
    pub fn parse(value: &str) -> Result<Self, &'static str> {
        match value { "butt" => Ok(Self::Butt), "round" => Ok(Self::Round), "square" => Ok(Self::Square), _ => Err("Invalid stroke cap") }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, semio_framework_value::RetainedClone, semio_framework_value::RetireOwned, semio_framework_value::ToValue, semio_framework_value::FromValue, semio_framework_dsl_record_derive::DslScalar, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner=semio_framework_pack_json)]
#[cfg_attr(any(test, feature = "testkit"), derive(serde::Serialize, serde::Deserialize))]
#[value(rename_all = "camelCase")]
#[cfg_attr(any(test, feature = "testkit"), serde(rename_all = "camelCase"))]
pub enum StrokeJoin { Miter, Round, Bevel }

impl StrokeJoin {
    pub const fn as_str(self) -> &'static str {
        match self { Self::Miter => "miter", Self::Round => "round", Self::Bevel => "bevel" }
    }
    pub fn parse(value: &str) -> Result<Self, &'static str> {
        match value { "miter" => Ok(Self::Miter), "round" => Ok(Self::Round), "bevel" => Ok(Self::Bevel), _ => Err("Invalid stroke join") }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn stroke_enums_match_neutral_cases_and_codecs() {
        use semio_framework_value::FromValue;
use semio_framework_value::ToValue;
        let cases: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🧪️tests/🧫️fixtures/🖊️stroke/🎚️enums/🔣️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let value = case["value"].clone();
            let valid = case["valid"] == true;
            let wire = semio_framework_pack_json::to_dsl_value(&semio_framework_pack_json::parse(&value.to_string(), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap());
            if case["kind"] == "cap" {
                let parsed = serde_json::from_value::<super::StrokeCap>(value.clone());
                assert_eq!(parsed.is_ok(), valid, "{case}");
                assert_eq!(super::StrokeCap::from_value(wire).is_ok(), valid, "{case}");
                assert_eq!(value.as_str().and_then(|value| super::StrokeCap::parse(value).ok()).is_some(), valid, "{case}");
                if let Ok(cap) = parsed {
                    assert_eq!(super::StrokeCap::parse(value.as_str().unwrap()).unwrap(), cap);
                    assert_eq!(serde_json::to_value(cap).unwrap(), value);
                    assert_eq!(super::StrokeCap::from_value(cap.to_value()).unwrap(), cap);
                }
            } else {
                let parsed = serde_json::from_value::<super::StrokeJoin>(value.clone());
                assert_eq!(parsed.is_ok(), valid, "{case}");
                assert_eq!(super::StrokeJoin::from_value(wire).is_ok(), valid, "{case}");
                assert_eq!(value.as_str().and_then(|value| super::StrokeJoin::parse(value).ok()).is_some(), valid, "{case}");
                if let Ok(join) = parsed {
                    assert_eq!(super::StrokeJoin::parse(value.as_str().unwrap()).unwrap(), join);
                    assert_eq!(serde_json::to_value(join).unwrap(), value);
                    assert_eq!(super::StrokeJoin::from_value(join.to_value()).unwrap(), join);
                }
            }
        }
        eprintln!("[TRACE] stroke cap and join codecs matched all shared enum cases");
    }

    #[test]
    fn dash_pattern_fixtures() {
        let cases: serde_json::Value = serde_json::from_str(runtime_fixture_text("../../../🧪️tests/🧫️fixtures/🖊️stroke/🔣️.json")).unwrap();
        for case in cases.as_array().unwrap() {
            let parsed = {let mut accepted=|_|true;let mut control=semio_framework_value::NativeDecodeControl::new(4096,&mut accepted);semio_s_artifact_draw_drawing_io::text::dash::decode_dash_text(case["value"].as_str().unwrap(),&mut control)};
            if case["error"] == true { assert!(parsed.is_err(), "{case}"); }
            else { assert_eq!(parsed.unwrap(), serde_json::from_value::<Option<Vec<f64>>>(case["dash"].clone()).unwrap(), "{case}"); }
        }
        eprintln!("[TRACE] stroke dash parsing matched all shared cases");
    }

    fn runtime_fixture_text(relative: &str) -> &'static str {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(relative);
        Box::leak(std::fs::read_to_string(&path).unwrap_or_else(|error| panic!("fixture {}: {error}", path.display())).into_boxed_str())
    }
}
