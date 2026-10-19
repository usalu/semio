//! 🧮️ Sheet attributes of the VDI 3805 snapshot: the attribute assembly from native records and the native record, the heating attribute sets, the generic attributes and the sheet attribute accessors. See [`super`].

use super::product_identity_and_links::ExtensionBag;
use super::record_families::RecordFamilyId;
use super::sheet_registry::SheetId;
use std::collections::BTreeMap;

pub fn attributes_from_records(sheet: SheetId, records: &[NativeRecord]) -> SheetAttributes {
    let mut kv: BTreeMap<String, String> = BTreeMap::new();
    for record in records {
        if record.family.0.as_str() == "210" {
            let mut it = record.fields.iter().skip(1);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "110" && record.fields.len() > 4 {
            let mut it = record.fields.iter().skip(4);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        } else if record.family.0.as_str() == "200" && record.fields.len() > 5 {
            let mut it = record.fields.iter().skip(5);
            while let (Some(k), Some(v)) = (it.next(), it.next()) {
                if parse_f64(k).is_none() {
                    kv.insert(k.to_lowercase(), v.clone());
                }
            }
        }
        if record.family.0 == RecordFamilyId::R100 {
            if let Some(group) = record.fields.get(2) {
                kv.entry("product_group".into()).or_insert_with(|| group.clone());
            }
        }
    }
    match sheet.0 {
        2 => {
            let dn = kv.get("dn").and_then(|s| s.parse().ok()).unwrap_or(0);
            let kvs_m3_h = kv.get("kvs").and_then(|s| parse_f64(s)).unwrap_or(0.0);
            SheetAttributes::ValveHeating(ValveHeatingAttributes::from_kvs_m3_h(
                dn,
                kvs_m3_h,
                kv.get("pressure_class").cloned().unwrap_or_default(),
                kv.get("connection_type").cloned().unwrap_or_default(),
                kv.get("authority_min").and_then(|s| parse_f64(s)).unwrap_or(0.0),
                kv.get("authority_max").and_then(|s| parse_f64(s)).unwrap_or(0.0),
            ))
        }
        3 => SheetAttributes::Radiator(RadiatorAttributes {
            standard_output_w: kv.get("standard_output_w").or(kv.get("phi")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            heat_exponent_n: kv.get("n").or(kv.get("heat_exponent_n")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            length_m: kv.get("length_m").or(kv.get("l")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            height_m: kv.get("height_m").or(kv.get("h")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            depth_m: kv.get("depth_m").or(kv.get("d")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            connection_type: kv.get("connection_type").cloned().unwrap_or_default(),
        }),
        5 => SheetAttributes::PumpHeating(PumpHeatingAttributes {
            dn_suction: kv.get("dn_suction").and_then(|s| s.parse().ok()).unwrap_or(0),
            dn_discharge: kv.get("dn_discharge").or(kv.get("dn")).and_then(|s| s.parse().ok()).unwrap_or(0),
            nominal_flow_m3_s: kv.get("q").or(kv.get("nominal_flow_m3_s")).and_then(|s| parse_f64(s)).map(|q| if q > 1.0 { q / 3600.0 } else { q }).unwrap_or(0.0),
            nominal_head_m: kv.get("h").or(kv.get("nominal_head_m")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            motor_power_w: kv.get("p").or(kv.get("motor_power_w")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            hydraulic_efficiency: kv.get("eta").or(kv.get("hydraulic_efficiency")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            qh_curve_ref: kv.get("qh_curve_ref").cloned(),
        }),
        6 => SheetAttributes::HeatGenerator(HeatGeneratorAttributes {
            nominal_heat_output_w: kv.get("qn").or(kv.get("nominal_heat_output_w")).and_then(|s| parse_f64(s)).unwrap_or(0.0),
            fuel_type: kv.get("fuel_type").cloned().unwrap_or_default(),
            flow_temp_max_c: kv.get("flow_temp_max_c").and_then(|s| parse_f64(s)).unwrap_or(0.0),
            return_temp_min_c: kv.get("return_temp_min_c").and_then(|s| parse_f64(s)).unwrap_or(0.0),
        }),
        _ => SheetAttributes::Generic(GenericAttributes::from_map(&kv)),
    }
}

pub(crate) fn parse_f64(raw: &str) -> Option<f64> {
    raw.replace(',', ".").parse().ok()
}

/// 📄️ One semicolon-delimited native record.
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct NativeRecord {
    pub family: RecordFamilyId,
    pub fields: Vec<String>,
    pub extensions: ExtensionBag,
}

/// 🎛 Sheet-2 heating control valve attributes (VDI 3805 Blatt 2).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct ValveHeatingAttributes {
    pub dn: u16,
    pub kvs_m3_s: f64,
    pub pressure_class: String,
    pub connection_type: String,
    pub authority_min: f64,
    pub authority_max: f64,
}

impl ValveHeatingAttributes {
    pub fn kvs_m3_h(&self) -> f64 {
        self.kvs_m3_s * 3600.0
    }

    pub fn from_kvs_m3_h(dn: u16, kvs_m3_h: f64, pressure_class: impl Into<String>, connection_type: impl Into<String>, authority_min: f64, authority_max: f64) -> Self {
        Self { dn, kvs_m3_s: kvs_m3_h / 3600.0, pressure_class: pressure_class.into(), connection_type: connection_type.into(), authority_min, authority_max }
    }
}

/// ♨️ Sheet-3 radiator / heating-surface attributes (VDI 3805 Blatt 3; Φ at 75/65/20 °C).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct RadiatorAttributes {
    pub standard_output_w: f64,
    pub heat_exponent_n: f64,
    pub length_m: f64,
    pub height_m: f64,
    pub depth_m: f64,
    pub connection_type: String,
}

/// 💧 Sheet-5 heating pump attributes (VDI 3805 Blatt 5).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct PumpHeatingAttributes {
    pub dn_suction: u16,
    pub dn_discharge: u16,
    pub nominal_flow_m3_s: f64,
    pub nominal_head_m: f64,
    pub motor_power_w: f64,
    pub hydraulic_efficiency: f64,
    pub qh_curve_ref: Option<String>,
}

/// 🔥 Sheet-6 heat-generator attributes (VDI 3805 Blatt 6).
#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct HeatGeneratorAttributes {
    pub nominal_heat_output_w: f64,
    pub fuel_type: String,
    pub flow_temp_max_c: f64,
    pub return_temp_min_c: f64,
}

/// 🧩 Single generic Blatt attribute (mirrors a native 210 key/value[/unit] pair).
#[derive(Clone, Debug, PartialEq, Default, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GenericAttribute {
    pub key: String,
    pub value: String,
    #[cfg_attr(test, serde(default, skip_serializing_if = "Option::is_none"))]
    #[value(default)]
    pub unit: Option<String>,
}

/// 📋 Generic sheet attributes as an ordered key/value/unit catalogue mirroring native records.
#[derive(Clone, Debug, PartialEq, Default, semio_framework_dsl_record_derive::DslRecord, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
pub struct GenericAttributes {
    #[dsl(table)]
    #[cfg_attr(test, serde(default))]
    #[value(default)]
    pub entries: Vec<GenericAttribute>,
}

impl GenericAttributes {
    pub fn from_map(kv: &BTreeMap<String, String>) -> Self {
        Self {
            entries: kv
                .iter()
                .map(|(k, v)| GenericAttribute {
                    key: k.clone(),
                    value: v.clone(),
                    unit: None,
                })
                .collect(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.entries.iter().find(|e| e.key.eq_ignore_ascii_case(key)).map(|e| e.value.as_str())
    }

    pub fn ensure_keys(&mut self, keys: &[&str]) {
        for key in keys {
            if self.get(key).is_none() {
                self.entries.push(GenericAttribute {
                    key: (*key).to_string(),
                    value: String::new(),
                    unit: None,
                });
            }
        }
    }
}

/// 🎛 Typed per-sheet mandatory attribute payload — replaces the untyped parameters map.
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(tag = "kind", rename_all = "camelCase"))]
#[value(tag = "kind", rename_all = "camelCase")]
pub enum SheetAttributes {
    ValveHeating(ValveHeatingAttributes),
    Radiator(RadiatorAttributes),
    PumpHeating(PumpHeatingAttributes),
    HeatGenerator(HeatGeneratorAttributes),
    Generic(GenericAttributes),
}

impl SheetAttributes {
    pub fn dn(&self) -> Option<u16> {
        match self {
            Self::ValveHeating(a) => Some(a.dn),
            Self::PumpHeating(a) => Some(a.dn_discharge),
            _ => None,
        }
    }
}

impl semio_framework_dsl_record::BorrowedDslField for SheetAttributes{const SHAPE:semio_framework_dsl_record::BorrowedShape=semio_framework_dsl_record::BorrowedShape::Value;}

impl semio_framework_dsl_record::DslField for SheetAttributes {
    fn shape_controlled<C:semio_framework_dsl_record::NativeSchemaControl>(control:&mut C)->Result<semio_framework_dsl_record::Shape,semio_framework_value::ValueError>{control.checkpoint()?;Ok(semio_framework_dsl_record::Shape::Value)}
    fn to_value_controlled(&self,control:&mut semio_framework_value::NativeEncodeControl<'_>)->Result<semio_framework_dsl_record::FieldValue,semio_framework_value::ValueError>{<Self as semio_framework_value::ToValue>::to_value_controlled(self,control).map(semio_framework_dsl_record::FieldValue::Value)}

    fn from_value_controlled(value: &semio_framework_dsl_record::FieldValue, control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<Self,semio_framework_value::ValueError> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(value) => sheet_attributes_controlled(value, control),
            _ => Err(vdi3805_invalid("expected VDI3805 sheet attribute value")),
        }
    }
    fn shape() -> semio_framework_dsl_record::Shape {
        semio_framework_dsl_record::Shape::Value
    }
    fn to_value(&self) -> semio_framework_dsl_record::FieldValue {
        semio_framework_dsl_record::FieldValue::Value(semio_framework_value::ToValue::to_value(self))
    }
    fn from_value(value: &semio_framework_dsl_record::FieldValue) -> Result<Self, String> {
        match value {
            semio_framework_dsl_record::FieldValue::Value(dsl_value) => {
                semio_framework_value::FromValue::from_value(dsl_value.clone()).map_err(|error|error.to_string())
            }
            other => Err(format!("expected Value, found {other:?}")),
        }
    }
}

/// 🚫️ An invalid VDI 3805 attribute value.
fn vdi3805_invalid(message: impl Into<String>) -> semio_framework_value::ValueError {
    semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, message)
}

pub fn sheet_attribute_object<'a>(value: &'a semio_framework_value::DslValue, allowed: &[&str], control: &mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>) -> Result<&'a [(String, semio_framework_value::DslValue)],semio_framework_value::ValueError> {
    let semio_framework_value::DslValue::Object(fields) = value else { return Err(vdi3805_invalid("expected VDI3805 attribute object")); };
    if fields.len() > allowed.len() { return Err(vdi3805_invalid("VDI3805 attribute object has excess fields")); }
    for (position, (key, _)) in fields.iter().enumerate() {
        control.step()?;
        if !allowed.contains(&key.as_str()) || fields[..position].iter().any(|(prior, _)| prior == key) { return Err(vdi3805_invalid("VDI3805 attribute object has unknown or duplicate fields")); }
    }
    Ok(fields)
}

pub fn sheet_attribute_field<'a>(fields: &'a [(String, semio_framework_value::DslValue)], key: &str) -> Result<&'a semio_framework_value::DslValue,semio_framework_value::ValueError> {
    fields.iter().find(|(name, _)| name == key).map(|(_, value)| value).ok_or_else(||vdi3805_invalid(format!("VDI3805 attribute requires {key}")))
}

pub fn sheet_attribute_real(fields: &[(String, semio_framework_value::DslValue)], key: &str) -> Result<f64,semio_framework_value::ValueError> {
    sheet_attribute_field(fields, key)?.as_f64().ok_or_else(||vdi3805_invalid(format!("VDI3805 attribute {key} requires a number")))
}

pub fn sheet_attribute_u16(fields: &[(String, semio_framework_value::DslValue)], key: &str) -> Result<u16,semio_framework_value::ValueError> {
    let word = sheet_attribute_field(fields, key)?.as_u64().ok_or_else(||vdi3805_invalid("expected unsigned VDI3805 attribute"))?;
    u16::try_from(word).map_err(|_|vdi3805_invalid("VDI3805 attribute exceeds u16"))
}
