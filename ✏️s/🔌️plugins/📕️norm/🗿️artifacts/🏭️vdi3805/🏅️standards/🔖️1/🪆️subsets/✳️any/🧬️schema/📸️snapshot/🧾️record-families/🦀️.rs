//! 🧾️ Record families of the VDI 3805 snapshot: the building system number and the record family identifier with its DSL field bindings and the family tables. See [`super`].

use semio_s_plugin_norm_compliance::NormError;

/// 🏗️ Parsed building-system number (Anlagenkennzeichen).
#[derive(Clone, Debug, PartialEq, Eq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct BuildingSystemNumber {
    pub system_code: String,
    pub subsystem: String,
    pub sequence: u32,
}

impl BuildingSystemNumber {
    pub fn parse(raw: &str) -> Result<Self, NormError> {
        let parts: Vec<&str> = raw.split('.').collect();
        if parts.len() != 3 {
            return Err(NormError::InvalidValue { field: "building_system_number".into(), reason: "expected SYS.SUB.NNN".into() });
        }
        let sequence: u32 = parts[2].parse().map_err(|_| NormError::InvalidValue { field: "building_system_number.sequence".into(), reason: "must be numeric".into() })?;
        Ok(Self { system_code: parts[0].into(), subsystem: parts[1].into(), sequence })
    }

    pub fn render(&self) -> String {
        format!("{}.{}.{}", self.system_code, self.subsystem, self.sequence)
    }
}

/// 📇️ Record family identifier (010…970.41).
#[derive(Clone, Debug, PartialEq, Eq, Hash, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(transparent)]
pub struct RecordFamilyId(pub String);

/// 🔗️ Hand `DslField` bridge for `RecordFamilyId`: a tuple ("newtype") struct has no named fields
/// for `#[derive(dsl::DslRecord)]` to enumerate, so it binds directly as `Shape::Text` instead of
/// changing its public tuple shape (used pervasively as `.0` across this crate).
impl semio_framework_dsl_record::BorrowedDslField for RecordFamilyId{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Text;}

impl semio_framework_dsl_record::DslField for RecordFamilyId {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{<String as semio_framework_dsl_record::DslField>::shape_controlled(control)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{<String as semio_framework_dsl_record::DslField>::to_value_controlled(&self.0,control)}

    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self,semio_framework_value::ValueError> {
        <String as semio_framework_dsl_record::DslField>::from_value_controlled(value, control).map(Self)
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Text
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Text(self.0.clone())
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Text(s) => Ok(RecordFamilyId(s.clone())),
            other => Err(format!("expected Text, found {other:?}")),
        }
    }
}

impl RecordFamilyId {
    pub const R010: &'static str = "010";
    pub const R020: &'static str = "020";
    pub const R030: &'static str = "030";
    pub const R040: &'static str = "040";
    pub const R050: &'static str = "050";
    pub const R060: &'static str = "060";
    pub const R070: &'static str = "070";
    pub const R080: &'static str = "080";
    pub const R090: &'static str = "090";
    pub const R100: &'static str = "100";
    pub const R110: &'static str = "110";
    pub const R120: &'static str = "120";
    pub const R130: &'static str = "130";
    pub const R140: &'static str = "140";
    pub const R150: &'static str = "150";
    pub const R160: &'static str = "160";
    pub const R170: &'static str = "170";
    pub const R180: &'static str = "180";
    pub const R190: &'static str = "190";
    pub const R200: &'static str = "200";
    pub const R210: &'static str = "210";
    pub const R220: &'static str = "220";
    pub const R230: &'static str = "230";
    pub const R240: &'static str = "240";
    pub const R250: &'static str = "250";
    pub const R260: &'static str = "260";
    pub const R270: &'static str = "270";
    pub const R280: &'static str = "280";
    pub const R290: &'static str = "290";
    pub const R300: &'static str = "300";
    pub const R310: &'static str = "310";
    pub const R320: &'static str = "320";
    pub const R330: &'static str = "330";
    pub const R340: &'static str = "340";
    pub const R350: &'static str = "350";
    pub const R360: &'static str = "360";
    pub const R370: &'static str = "370";
    pub const R380: &'static str = "380";
    pub const R390: &'static str = "390";
    pub const R400: &'static str = "400";
    pub const R410: &'static str = "410";
    pub const R420: &'static str = "420";
    pub const R430: &'static str = "430";
    pub const R440: &'static str = "440";
    pub const R450: &'static str = "450";
    pub const R460: &'static str = "460";
    pub const R470: &'static str = "470";
    pub const R480: &'static str = "480";
    pub const R490: &'static str = "490";
    pub const R500: &'static str = "500";
    pub const R510: &'static str = "510";
    pub const R520: &'static str = "520";
    pub const R530: &'static str = "530";
    pub const R540: &'static str = "540";
    pub const R550: &'static str = "550";
    pub const R560: &'static str = "560";
    pub const R570: &'static str = "570";
    pub const R580: &'static str = "580";
    pub const R590: &'static str = "590";
    pub const R600: &'static str = "600";
    pub const R610: &'static str = "610";
    pub const R620: &'static str = "620";
    pub const R630: &'static str = "630";
    pub const R640: &'static str = "640";
    pub const R650: &'static str = "650";
    pub const R660: &'static str = "660";
    pub const R670: &'static str = "670";
    pub const R680: &'static str = "680";
    pub const R690: &'static str = "690";
    pub const R700: &'static str = "700";
    pub const R710: &'static str = "710";
    pub const R720: &'static str = "720";
    pub const R730: &'static str = "730";
    pub const R740: &'static str = "740";
    pub const R750: &'static str = "750";
    pub const R760: &'static str = "760";
    pub const R770: &'static str = "770";
    pub const R780: &'static str = "780";
    pub const R790: &'static str = "790";
    pub const R800: &'static str = "800";
    pub const R810: &'static str = "810";
    pub const R820: &'static str = "820";
    pub const R830: &'static str = "830";
    pub const R840: &'static str = "840";
    pub const R850: &'static str = "850";
    pub const R860: &'static str = "860";
    pub const R870: &'static str = "870";
    pub const R880: &'static str = "880";
    pub const R890: &'static str = "890";
    pub const R900: &'static str = "900";
    pub const R910: &'static str = "910";
    pub const R920: &'static str = "920";
    pub const R930: &'static str = "930";
    pub const R940: &'static str = "940";
    pub const R950: &'static str = "950";
    pub const R960: &'static str = "960";
    pub const R970_41: &'static str = "970.41";

    pub fn all_known() -> &'static [&'static str] {
        &[
            Self::R010,
            Self::R020,
            Self::R030,
            Self::R040,
            Self::R050,
            Self::R060,
            Self::R070,
            Self::R080,
            Self::R090,
            Self::R100,
            Self::R110,
            Self::R120,
            Self::R130,
            Self::R140,
            Self::R150,
            Self::R160,
            Self::R170,
            Self::R180,
            Self::R190,
            Self::R200,
            Self::R210,
            Self::R220,
            Self::R230,
            Self::R240,
            Self::R250,
            Self::R260,
            Self::R270,
            Self::R280,
            Self::R290,
            Self::R300,
            Self::R310,
            Self::R320,
            Self::R330,
            Self::R340,
            Self::R350,
            Self::R360,
            Self::R370,
            Self::R380,
            Self::R390,
            Self::R400,
            Self::R410,
            Self::R420,
            Self::R430,
            Self::R440,
            Self::R450,
            Self::R460,
            Self::R470,
            Self::R480,
            Self::R490,
            Self::R500,
            Self::R510,
            Self::R520,
            Self::R530,
            Self::R540,
            Self::R550,
            Self::R560,
            Self::R570,
            Self::R580,
            Self::R590,
            Self::R600,
            Self::R610,
            Self::R620,
            Self::R630,
            Self::R640,
            Self::R650,
            Self::R660,
            Self::R670,
            Self::R680,
            Self::R690,
            Self::R700,
            Self::R710,
            Self::R720,
            Self::R730,
            Self::R740,
            Self::R750,
            Self::R760,
            Self::R770,
            Self::R780,
            Self::R790,
            Self::R800,
            Self::R810,
            Self::R820,
            Self::R830,
            Self::R840,
            Self::R850,
            Self::R860,
            Self::R870,
            Self::R880,
            Self::R890,
            Self::R900,
            Self::R910,
            Self::R920,
            Self::R930,
            Self::R940,
            Self::R950,
            Self::R960,
            Self::R970_41,
        ]
    }
}
