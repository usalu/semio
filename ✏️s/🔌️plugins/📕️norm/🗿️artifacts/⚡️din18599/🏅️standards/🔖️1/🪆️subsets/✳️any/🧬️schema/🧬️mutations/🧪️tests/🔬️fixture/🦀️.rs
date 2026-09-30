//! 🧫️ Committed mutation vectors — every `🧫️fixtures/🧬️mutations/<leaf>/<scenario>` bundle is this
//! implementation's own answer, and every kind is exercised by an applied vector or a payload-only
//! `🧾️wire-witness`. The independent Python engine is held to the same bundles by `⚡️mutate-din18599-1`.

use super::{apply_din18599_mutation, decode_din18599_mutation_json, inverse_din18599_mutation, Din18599Mutation, KINDS};
use crate::standards::v1::subsets::any::schema::snapshot::{decode_din18599_snapshot_json, Din18599Snapshot};
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

fn snapshot(path: &Path) -> Din18599Snapshot {
    decode_din18599_snapshot_json(&read(path)).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

/// ⚖️ One bundle's breaches: the mutation reaches the committed after-snapshot and diff, or refuses with the committed
/// fatal code and leaves the document untouched, and its own inverse restores the before-snapshot.
fn vector_breaches(bundle: &Path, mutation: &Din18599Mutation) -> Vec<String> {
    let name = bundle.display();
    let before = snapshot(&bundle.join("📸️snapshot/⬅️before/🔣️.json"));
    let after = snapshot(&bundle.join("📸️snapshot/➡️after/🔣️.json"));
    let outcome = json(&read(&bundle.join("🎯️outcome/🔣️.json")));
    let (applied, messages) = apply_din18599_mutation(&before, mutation).unwrap_or_else(|error| panic!("{name}: {error}"));
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
    let raised = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::diff(mutation, &before);
    if json(&pack::json::to_json_string(raised.diff())) != json(&read(&bundle.join("🔺️diff/🔣️.json"))) {
        breaches.push(format!("{name}: the produced diff is not the committed diff"));
    }
    let steps = inverse_din18599_mutation(mutation, &before);
    let restored = steps.iter().fold(applied, |document, step| apply_din18599_mutation(&document, step).map(|(next, _)| next).unwrap_or_else(|error| panic!("{name}: {error}")));
    if steps.is_empty() || restored != before {
        breaches.push(format!("{name}: the mutation's own inverse ({} step(s)) does not restore the before-snapshot", steps.len()));
    }
    breaches
}

/// 💾️ The committed mutation crosses the binary op codec unchanged, framed under its leaf descriptor's protocol tag.
fn binary_breaches(bundle: &Path, mutation: &Din18599Mutation) -> Vec<String> {
    let name = bundle.display();
    let bytes = match <Din18599Mutation as protocol::OpBinary>::encode_op(mutation) {
        Ok(bytes) => bytes,
        Err(error) => return vec![format!("{name}: the binary op encoding failed: {error}")],
    };
    let mut breaches = Vec::new();
    let declared = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::descriptor(mutation).binary_tag.map(u64::from);
    let mut reader = store::pack_rt::ByteReader::new(&bytes);
    let framed = reader.read_u8().ok().and_then(|_| reader.read_varint_u64().ok());
    if framed != declared {
        breaches.push(format!("{name}: the op frame carries tag {framed:?}, the leaf descriptor declares {declared:?}"));
    }
    match <Din18599Mutation as protocol::OpBinary>::decode_op(&bytes) {
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
        let mutation = decode_din18599_mutation_json(&read(&scenario.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", scenario.display()));
        let kind = <Din18599Mutation as protocol::Mutation<Din18599Snapshot>>::descriptor(&mutation).semantic_kind;
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

#[path = "../../📐️change-net-floor-area-m2/🧪️tests/📏️extends-net-floor-area-to-160-m2/🦀️.rs"]
mod tests_change_net_floor_area_m2_extends_net_floor_area_to_160_m2;
#[path = "../../🏷️change-use-class/🧪️tests/🏢️reclassifies-the-building-as-an-office/🦀️.rs"]
mod tests_change_use_class_reclassifies_the_building_as_an_office;
#[path = "../../🌦️update-climate/🧪️tests/🌧️refuses-a-negative-january-irradiance/🦀️.rs"]
mod tests_update_climate_refuses_a_negative_january_irradiance;
