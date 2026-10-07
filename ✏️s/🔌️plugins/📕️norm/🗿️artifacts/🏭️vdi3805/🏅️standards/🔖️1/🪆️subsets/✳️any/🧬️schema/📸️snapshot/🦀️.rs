//! 🧬️ Vdi3805 snapshot schema — artifact-lane fields only.

use crate::{CatalogIndex, CharacteristicCurve, EditionId, EditionProfileChoice, ManufacturerCatalog, ParametricGeometry, SecurityLimits, SheetId, NativeRecord, SheetAttributes, RecordFamilyId, ValveHeatingAttributes, RadiatorAttributes, PumpHeatingAttributes, HeatGeneratorAttributes, GenericAttributes};
use framework_schema::ArtifactSchema;
use std::collections::BTreeMap;


#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslRecord, ArtifactSchema, value_derive::ToValue, value_derive::FromValue)]
#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase"))]
#[value(rename_all = "camelCase")]
#[dsl(id = "norm.vdi3805", layout = "lines")]
#[artifact_schema(id = "s.norm.vdi3805")]
pub struct Vdi3805Snapshot {
    #[state(artifact)]
    pub catalog: ManufacturerCatalog,
    #[state(artifact)]
    pub edition_profile: BTreeMap<String, EditionProfileChoice>,
    #[state(artifact)]
    pub correction_as_of: EditionId,
    #[state(artifact)]
    pub strict_mode: bool,
    #[state(artifact)]
    pub index: CatalogIndex,
    #[state(artifact)]
    pub geometry: BTreeMap<String, ParametricGeometry>,
    #[state(artifact)]
    pub curves: BTreeMap<String, CharacteristicCurve>,
    #[state(artifact)]
    pub limits: SecurityLimits,
}


impl Default for Vdi3805Snapshot {
    fn default() -> Self {
        crate::reference_fixture()
    }
}








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
