//! 🧫️ Committed mutation vectors — every `🧫️fixtures/🧬️mutations/<leaf>/<scenario>` bundle is this
//! implementation's own answer, and every kind is exercised by an applied vector or a payload-only
//! `🧾️wire-witness`. The independent Python engine is held to the same bundles by `🧱️mutate-din4108-1`.

use super::{apply_din4108_mutation, decode_din4108_mutation_json, inverse_din4108_mutation, Din4108Mutation, KINDS};
use crate::standards::v1::subsets::any::schema::snapshot::{decode_din4108_snapshot_json, Din4108Snapshot};
use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

fn read(path: &Path) -> String {
    std::fs::read_to_string(path).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn json(text: &str) -> serde_json::Value {
    serde_json::from_str(text).expect("committed JSON")
}

fn directories(path: &Path) -> Vec<PathBuf> {
    let mut found: Vec<PathBuf> = std::fs::read_dir(path).expect("fixture directory").map(|entry| entry.expect("entry").path()).filter(|path| path.is_dir()).collect();
    found.sort();
    found
}

fn snapshot(path: &Path) -> Din4108Snapshot {
    decode_din4108_snapshot_json(&read(path)).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// ⚖️ One bundle's breaches: the mutation reaches the committed after-snapshot and diff, or refuses with the committed
/// fatal code and leaves the document untouched, and its own inverse restores the before-snapshot.
fn vector_breaches(bundle: &Path, mutation: &Din4108Mutation) -> Vec<String> {
    let name = bundle.display();
    let before = snapshot(&bundle.join("📸️snapshot/⬅️before/🔣️.json"));
    let after = snapshot(&bundle.join("📸️snapshot/➡️after/🔣️.json"));
    let outcome = json(&read(&bundle.join("🎯️outcome/🔣️.json")));
    let (applied, messages) = apply_din4108_mutation(&before, mutation).unwrap_or_else(|error| panic!("{name}: {error}"));
    let mut breaches = Vec::new();
    if applied != after {
        breaches.push(format!("{name}: the applied document is not the committed after-snapshot"));
    }
    if outcome["status"] == "rejected" {
        if applied != before || !messages.contains(&format!("Fatal:{}", outcome["code"].as_str().unwrap_or_default())) {
            breaches.push(format!("{name}: the committed refusal {} is not raised, got {messages:?}", outcome["code"]));
        }
        return breaches;
    }
    if !messages.is_empty() || applied == before {
        breaches.push(format!("{name}: an applied vector must move the document without a diagnostic, got {messages:?}"));
    }
    let raised = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::diff(mutation, &before);
    if json(&pack::json::to_json_string(raised.diff())) != json(&read(&bundle.join("🔺️diff/🔣️.json"))) {
        breaches.push(format!("{name}: the produced diff is not the committed diff"));
    }
    let steps = inverse_din4108_mutation(mutation, &before);
    let restored = steps.iter().fold(applied, |document, step| apply_din4108_mutation(&document, step).map(|(next, _)| next).unwrap_or_else(|error| panic!("{name}: {error}")));
    if steps.is_empty() || restored != before {
        breaches.push(format!("{name}: the mutation's own inverse ({} step(s)) does not restore the before-snapshot", steps.len()));
    }
    breaches
}

/// 🎯️ The canonical assertion one vector's own test makes: its bundle holds with no breach.
fn assert_vector(leaf: &str, scenario: &str) {
    let bundle = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations").join(leaf).join(scenario);
    let mutation = decode_din4108_mutation_json(&read(&bundle.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", bundle.display()));
    let breaches = vector_breaches(&bundle, &mutation);
    assert!(breaches.is_empty(), "{}", breaches.join("\n"));
}

/// 💾️ The committed mutation crosses the binary op codec unchanged, framed under its leaf descriptor's protocol tag.
fn binary_breaches(bundle: &Path, mutation: &Din4108Mutation) -> Vec<String> {
    let name = bundle.display();
    let bytes = match <Din4108Mutation as protocol::OpBinary>::encode_op(mutation) {
        Ok(bytes) => bytes,
        Err(error) => return vec![format!("{name}: the binary op encoding failed: {error}")],
    };
    let mut breaches = Vec::new();
    let declared = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::descriptor(mutation).binary_tag.map(u64::from);
    let mut reader = store::pack_rt::ByteReader::new(&bytes);
    let framed = reader.read_u8().ok().and_then(|_| reader.read_varint_u64().ok());
    if framed != declared {
        breaches.push(format!("{name}: the op frame carries tag {framed:?}, the leaf descriptor declares {declared:?}"));
    }
    match <Din4108Mutation as protocol::OpBinary>::decode_op(&bytes) {
        Ok(decoded) if &decoded == mutation => {}
        Ok(decoded) => breaches.push(format!("{name}: the binary op decodes to {decoded:?}")),
        Err(error) => breaches.push(format!("{name}: the binary op does not decode: {error}")),
    }
    breaches
}

#[test]
fn committed_vectors_are_this_implementations_answer() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    let (mut covered, mut breaches) = (BTreeSet::new(), Vec::new());
    for scenario in directories(&root).iter().flat_map(|leaf| directories(leaf)) {
        let mutation = decode_din4108_mutation_json(&read(&scenario.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", scenario.display()));
        let kind = <Din4108Mutation as protocol::Mutation<Din4108Snapshot>>::descriptor(&mutation).semantic_kind;
        breaches.extend(binary_breaches(&scenario, &mutation));
        if scenario.ends_with("🧾️wire-witness") {
            covered.insert(kind);
            continue;
        }
        let found = vector_breaches(&scenario, &mutation);
        if found.is_empty() && json(&read(&scenario.join("🎯️outcome/🔣️.json")))["status"] == "applied" {
            covered.insert(kind);
        }
        breaches.extend(found);
    }
    assert!(breaches.is_empty(), "{}", breaches.join("\n"));
    let missing: Vec<&str> = KINDS.iter().copied().filter(|kind| !covered.contains(kind)).collect();
    assert!(missing.is_empty(), "kinds without a committed applied vector or wire witness: {missing:?}");
}

//#region 🧫️CanonicalVectorTests
#[path = "../../🌦️change-climate-zone/🧪️tests/🗺️moves-to-zone-3/🦀️.rs"]
mod change_climate_zone;
#[path = "../../🗂️change-usage/🧪️tests/🏢️sets-nonresidential/🦀️.rs"]
mod change_usage;
#[path = "../../🌡️change-t-int-c/🧪️tests/🌡️sets-t-int-to-21-point-5/🦀️.rs"]
mod change_t_int_c;
#[path = "../../💧️change-rh-int/🧪️tests/💧️raises-rh-to-0-point-55/🦀️.rs"]
mod change_rh_int;
#[path = "../../💨️change-airtightness-n50/🧪️tests/💨️tightens-n50-to-1-point-0/🦀️.rs"]
mod change_airtightness_n50;
#[path = "../../🌬️change-has-mechanical-ventilation/🧪️tests/🌬️disables-mechanical-ventilation/🦀️.rs"]
mod change_has_mechanical_ventilation;
#[path = "../../✅️change-bb2-details-conform/🧪️tests/❌️declares-bb2-non-conforming/🦀️.rs"]
mod change_bb2_details_conform;
#[path = "../../➕️insert-zone/🧪️tests/➕️appends-extra-zone/🦀️.rs"]
mod insert_zone;
#[path = "../../➖️remove-zone/🧪️tests/🚫️removes-first-zone/🦀️.rs"]
mod remove_zone;
#[path = "../../📐️change-zone-floor-area/🧪️tests/📐️sets-floor-area-to-90/🦀️.rs"]
mod change_zone_floor_area;
#[path = "../../🧱change-zone-heaviness/🧪️tests/🧱sets-heaviness-light/🦀️.rs"]
mod change_zone_heaviness;
#[path = "../../🌙change-zone-night-ventilation/🧪️tests/🌙sets-night-ventilation-high/🦀️.rs"]
mod change_zone_night_ventilation;
#[path = "../../🪟insert-zone-window/🧪️tests/🪟appends-extra-window/🦀️.rs"]
mod insert_zone_window;
#[path = "../../🚫️remove-zone-window/🧪️tests/🚫️removes-east-window/🦀️.rs"]
mod remove_zone_window;
#[path = "../../📏change-zone-window-area/🧪️tests/📏grows-south-window/🦀️.rs"]
mod change_zone_window_area;
#[path = "../../☀️change-zone-window-g-value/🧪️tests/☀️sets-g-value-0-point-6/🦀️.rs"]
mod change_zone_window_g_value;
#[path = "../../⛱️change-zone-window-shading-fc/🧪️tests/⛱️tightens-shading-fc/🦀️.rs"]
mod change_zone_window_shading_fc;
#[path = "../../🏠️insert-element/🧪️tests/🏠️appends-extra-wall/🦀️.rs"]
mod insert_element;
#[path = "../../🚫️remove-element/🧪️tests/🚫️removes-first-element/🦀️.rs"]
mod remove_element;
#[path = "../../📐️change-element-area/🧪️tests/📐️grows-wall-area/🦀️.rs"]
mod change_element_area;
#[path = "../../↔️change-element-adjacent/🧪️tests/↔️sets-adjacent-unheated/🦀️.rs"]
mod change_element_adjacent;
#[path = "../../🏷️change-element-kind/🧪️tests/🏷️retags-as-opaque-frame/🦀️.rs"]
mod change_element_kind;
#[path = "../../➕️insert-layer/🧪️tests/➕️inserts-layer-into-wall/🦀️.rs"]
mod insert_layer;
#[path = "../../➖️remove-layer/🧪️tests/➖️removes-eps-layer/🦀️.rs"]
mod remove_layer;
#[path = "../../🔀️reorder-layers/🧪️tests/🧭️swaps-first-two-layers/🦀️.rs"]
mod reorder_layers;
#[path = "../../📏️change-layer-thickness/🧪️tests/📏️thickens-eps-to-0-point-2/🦀️.rs"]
mod change_layer_thickness;
#[path = "../../🌡change-layer-lambda/🧪️tests/🌡️sets-eps-lambda/🦀️.rs"]
mod change_layer_lambda;
#[path = "../../💧change-layer-mu/🧪️tests/💧raises-eps-mu/🦀️.rs"]
mod change_layer_mu;
#[path = "../../🧽️change-layer-material-id/🧪️tests/🧽️retags-eps-material/🦀️.rs"]
mod change_layer_material_id;
#[path = "../../🌉️insert-thermal-bridge/🧪️tests/🌉️appends-extra-bridge/🦀️.rs"]
mod insert_thermal_bridge;
#[path = "../../🧊remove-thermal-bridge/🧪️tests/🧊removes-first-bridge/🦀️.rs"]
mod remove_thermal_bridge;
#[path = "../../🔘change-thermal-bridge-psi/🧪️tests/🔘lowers-psi/🦀️.rs"]
mod change_thermal_bridge_psi;
#[path = "../../↔️change-thermal-bridge-length/🧪️tests/↔️shortens-bridge/🦀️.rs"]
mod change_thermal_bridge_length;
#[path = "../../🧭change-element-orientation-deg/🧪️tests/🧭turns-north-wall-south/🦀️.rs"]
mod change_element_orientation_deg;
#[path = "../../📐change-element-inclination-deg/🧪️tests/📐tilts-north-wall-to-45-degrees/🦀️.rs"]
mod change_element_inclination_deg;
#[path = "../../📈️change-element-delta-ug/🧪️tests/📈️raises-glazing-delta-ug/🦀️.rs"]
mod change_element_delta_ug;
#[path = "../../📈️change-element-delta-uf/🧪️tests/📈️raises-frame-delta-uf/🦀️.rs"]
mod change_element_delta_uf;
#[path = "../../📈️change-element-delta-ur/🧪️tests/📈️raises-roof-delta-ur/🦀️.rs"]
mod change_element_delta_ur;
#[path = "../../🏷change-thermal-bridge-bb2-type/🧪️tests/🏷️reclassifies-reveal-bridge/🦀️.rs"]
mod change_thermal_bridge_bb2_type;
#[path = "../../🧭change-zone-window-orientation/🧪️tests/🧭turns-south-window-west/🦀️.rs"]
mod change_zone_window_orientation;
#[path = "../../📐change-zone-window-inclination-deg/🧪️tests/📐tilts-south-window-to-60-degrees/🦀️.rs"]
mod change_zone_window_inclination_deg;
#[path = "../../🏷️change-layer-application-type/🧪️tests/🏷️reclassifies-eps-as-wab/🦀️.rs"]
mod change_layer_application_type;
#[path = "../../🏷️change-layer-compressive-class/🧪️tests/🏷️raises-eps-compressive-class/🦀️.rs"]
mod change_layer_compressive_class;
//#endregion 🧫️CanonicalVectorTests
