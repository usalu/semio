//! WASM bindings preserving the JS-visible names used by the TypeScript SDK.
//! All functions delegate to the OO API on `Kit`/`KitRef`; the bindings only
//! marshal JSON through `serde-wasm-bindgen`.

use wasm_bindgen::prelude::*;

use crate::guid::Guid;
use crate::kit::Kit;

#[wasm_bindgen(js_name = generateGuid)]
pub fn wasm_generate_guid() -> String {
    Guid::new_v7().into_string()
}

#[wasm_bindgen(js_name = kitFromJson)]
pub fn wasm_kit_from_json(s: &str) -> std::result::Result<JsValue, JsValue> {
    match Kit::from_json_str(s) {
        Ok(kit) => {
            let guard = kit.read().map_err(|_| JsValue::from_str("kit lock poisoned"))?;
            serde_wasm_bindgen::to_value(&guard.to_dto()).map_err(|e| JsValue::from_str(&e.to_string()))
        }
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

#[wasm_bindgen(js_name = kitToJson)]
pub fn wasm_kit_to_json(value: JsValue) -> std::result::Result<String, JsValue> {
    let dto: crate::kit::KitDto =
        serde_wasm_bindgen::from_value(value).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let kit = Kit::from_dto(dto);
    let guard = kit.read().map_err(|_| JsValue::from_str("kit lock poisoned"))?;
    guard.to_json_pretty().map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = kitValidate)]
pub fn wasm_kit_validate(value: JsValue) -> std::result::Result<JsValue, JsValue> {
    let dto: crate::kit::KitDto =
        serde_wasm_bindgen::from_value(value).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let kit = Kit::from_dto(dto);
    let guard = kit.read().map_err(|_| JsValue::from_str("kit lock poisoned"))?;
    serde_wasm_bindgen::to_value(&guard.validate()).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen(js_name = kitsAreEqual)]
pub fn wasm_kits_are_equal(a: JsValue, b: JsValue) -> std::result::Result<bool, JsValue> {
    let a: crate::kit::KitDto =
        serde_wasm_bindgen::from_value(a).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let b: crate::kit::KitDto =
        serde_wasm_bindgen::from_value(b).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let ka = Kit::from_dto(a);
    let kb = Kit::from_dto(b);
    let ga = ka.read().map_err(|_| JsValue::from_str("a poisoned"))?;
    let gb = kb.read().map_err(|_| JsValue::from_str("b poisoned"))?;
    Ok(ga.are_equal(&gb))
}

#[wasm_bindgen(js_name = flattenDesign)]
pub fn wasm_flatten_design(kit: JsValue, design_guid: &str) -> std::result::Result<JsValue, JsValue> {
    let dto: crate::kit::KitDto =
        serde_wasm_bindgen::from_value(kit).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let kit = Kit::from_dto(dto);
    let guard = kit.read().map_err(|_| JsValue::from_str("kit lock poisoned"))?;
    match guard.flatten_design(design_guid) {
        Ok(rep) => serde_wasm_bindgen::to_value(&rep).map_err(|e| JsValue::from_str(&e.to_string())),
        Err(e) => Err(JsValue::from_str(&e.to_string())),
    }
}

/// Tiny utility namespace preserved for callers that imported it.
#[wasm_bindgen(js_name = semioRound)]
pub fn wasm_round(value: f64, decimals: i32) -> f64 {
    let m = 10f64.powi(decimals);
    (value * m).round() / m
}

#[wasm_bindgen(js_name = semioNormalizeName)]
pub fn wasm_normalize_name(s: &str) -> String {
    s.trim().to_ascii_lowercase().replace(|c: char| c.is_whitespace(), "-")
}
