//! WASM bindings preserving the JS-visible names used by the TypeScript SDK.
//! I/O-style entry points return Promises (`future_to_promise`) so hosts can await them.

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

use crate::guid::Guid;
use crate::kit::KitStore;

#[wasm_bindgen(js_name = generateGuid)]
pub fn wasm_generate_guid() -> String {
    Guid::new_v7().into_string()
}

#[wasm_bindgen(js_name = kitFromJson)]
pub fn wasm_kit_from_json(s: &str) -> js_sys::Promise {
    let s = s.to_string();
    future_to_promise(async move {
        match KitStore::from_json_str(&s) {
            Ok(kit) => {
                let guard = kit.read().map_err(|_| JsValue::from_str("kit lock poisoned"))?;
                serde_wasm_bindgen::to_value(&guard.to_full_dto()).map_err(|e| JsValue::from_str(&e.to_string()))
            }
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    })
}

#[wasm_bindgen(js_name = kitToJson)]
pub fn wasm_kit_to_json(value: JsValue) -> js_sys::Promise {
    future_to_promise(async move {
        let dto: crate::kit::KitFullDto =
            serde_wasm_bindgen::from_value(value).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let kit = KitStore::from_full_dto(dto);
        let guard = kit.read().map_err(|_| JsValue::from_str("kit lock poisoned"))?;
        guard.to_json_pretty().map_err(|e| JsValue::from_str(&e.to_string()))
    })
}

#[wasm_bindgen(js_name = kitValidate)]
pub fn wasm_kit_validate(value: JsValue) -> js_sys::Promise {
    future_to_promise(async move {
        let dto: crate::kit::KitFullDto =
            serde_wasm_bindgen::from_value(value).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let kit = KitStore::from_full_dto(dto);
        match KitStore::validate_async(&kit).await {
            Ok(v) => serde_wasm_bindgen::to_value(&v).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    })
}

#[wasm_bindgen(js_name = kitsAreEqual)]
pub fn wasm_kits_are_equal(a: JsValue, b: JsValue) -> js_sys::Promise {
    future_to_promise(async move {
        let a: crate::kit::KitFullDto =
            serde_wasm_bindgen::from_value(a).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let b: crate::kit::KitFullDto =
            serde_wasm_bindgen::from_value(b).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let ka = KitStore::from_full_dto(a);
        let kb = KitStore::from_full_dto(b);
        let ga = ka.read().map_err(|_| JsValue::from_str("a poisoned"))?;
        let gb = kb.read().map_err(|_| JsValue::from_str("b poisoned"))?;
        Ok(JsValue::from_bool(ga.are_equal(&gb)))
    })
}

#[wasm_bindgen(js_name = flattenDesign)]
pub fn wasm_flatten_design(kit: JsValue, design_guid: &str) -> js_sys::Promise {
    let design_guid = design_guid.to_string();
    future_to_promise(async move {
        let dto: crate::kit::KitFullDto =
            serde_wasm_bindgen::from_value(kit).map_err(|e| JsValue::from_str(&e.to_string()))?;
        let k = KitStore::from_full_dto(dto);
        match KitStore::flatten_design_async(&k, &design_guid).await {
            Ok(rep) => serde_wasm_bindgen::to_value(&rep).map_err(|e| JsValue::from_str(&e.to_string())),
            Err(e) => Err(JsValue::from_str(&e.to_string())),
        }
    })
}

#[wasm_bindgen(js_name = semioRound)]
pub fn wasm_round(value: f64, decimals: i32) -> f64 {
    let m = 10f64.powi(decimals);
    (value * m).round() / m
}

#[wasm_bindgen(js_name = semioNormalizeName)]
pub fn wasm_normalize_name(s: &str) -> String {
    s.trim().to_ascii_lowercase().replace(|c: char| c.is_whitespace(), "-")
}
