//! 🧫️ Committed mutation vectors — every `🧫️fixtures/🧬️mutations/<leaf>/<scenario>` bundle is this implementation's own
//! answer, every kind is exercised by an applied vector or a payload-only `🧾️wire-witness`, and every leaf descriptor
//! declares exactly the outcome classes production dispatch reaches from its vectors. The independent Python engine is
//! held to the same bundles by `🪨️mutate-en1996-1`.

use super::{apply_en1996_mutation, inverse_en1996_mutation, En1996Mutation, KINDS};
use crate::standards::v1::subsets::any::io::text::mutations::{decode_en1996_mutation_json};
use crate::standards::v1::subsets::any::schema::snapshot::{En1996Snapshot};
use crate::standards::v1::subsets::any::io::text::snapshot::{decode_en1996_snapshot_json};
use protocol::Mutation;
use std::collections::{BTreeMap, BTreeSet};
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

fn snapshot(path: &Path) -> En1996Snapshot {
    decode_en1996_snapshot_json(&read(path)).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
}

/// 🏷️ A committed outcome's messages in production dispatch's `<Level>:<code>` spelling.
fn committed_messages(outcome: &serde_json::Value) -> Vec<String> {
    let spelled = |message: &serde_json::Value| {
        let level = message["level"].as_str().unwrap_or_default();
        format!("{}{}:{}", level.get(..1).unwrap_or_default().to_uppercase(), level.get(1..).unwrap_or_default(), message["code"].as_str().unwrap_or_default())
    };
    outcome["messages"].as_array().map(|messages| messages.iter().map(spelled).collect()).unwrap_or_default()
}

/// 🚫️ Whether production dispatch refused: it raised an error- or fatal-level message.
fn refused(messages: &[String]) -> bool {
    messages.iter().any(|message| message.starts_with("Error:") || message.starts_with("Fatal:"))
}

/// 🎯️ The mutation addressing a target no document holds — every position past any collection's end, every native id an
/// unknown one — or `None` when its payload addresses nothing.
fn stray(mutation: &En1996Mutation) -> Option<En1996Mutation> {
    let mut payload = serde_json::Value::from(mutation.payload_value());
    let mut addressed = false;
    for (name, value) in payload.as_object_mut()? {
        if (name == "index" || name.ends_with("Index")) && value.is_u64() {
            *value = serde_json::Value::from(u64::from(u32::MAX));
            addressed = true;
        } else if (name == "id" || name.ends_with("Id")) && value.is_string() {
            *value = serde_json::Value::from("∅");
            addressed = true;
        }
    }
    addressed.then(|| mutation.with_payload_value(semio_framework_value::DslValue::from(&payload)).ok()).flatten()
}

/// ⚖️ One bundle's breaches, and the outcome classes it shows production dispatch reaching. Every vector lands on its
/// committed after-snapshot under exactly its committed messages: an `applied` one moves the document by exactly the
/// committed diff and its own inverse restores the before-snapshot, a `no-op` one keeps the document under the committed
/// empty diff, and a `rejected` one is refused at error or fatal level and keeps it. Re-applying the mutation to the
/// after-snapshot shows `no-op` reached when it raises `mutation.no-op` and `rejected` when it is refused, as does the
/// mutation addressing a target no document holds.
fn vector_breaches(bundle: &Path, mutation: &En1996Mutation, reached: &mut BTreeSet<String>) -> Vec<String> {
    let name = bundle.display();
    let apply = |document: &En1996Snapshot, step: &En1996Mutation| apply_en1996_mutation(document, step).unwrap_or_else(|error| panic!("{name}: {error}"));
    let before = snapshot(&bundle.join("📸️snapshot/⬅️before/🔣️.json"));
    let after = snapshot(&bundle.join("📸️snapshot/➡️after/🔣️.json"));
    let outcome = json(&read(&bundle.join("🎯️outcome/🔣️.json")));
    let status = outcome["status"].as_str().unwrap_or_default().to_string();
    let (applied, messages) = apply(&before, mutation);
    let mut breaches = Vec::new();
    if applied != after {
        breaches.push(format!("{name}: the applied document is not the committed after-snapshot"));
    }
    if messages != committed_messages(&outcome) {
        breaches.push(format!("{name}: production dispatch raises {messages:?}, the committed outcome {:?}", committed_messages(&outcome)));
    }
    if (status == "applied") == (applied == before) || refused(&messages) != (status == "rejected") {
        breaches.push(format!("{name}: a {status:?} vector must move the document exactly when applied and be refused exactly when rejected"));
    }
    if status != "rejected" {
        let raised = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::diff(mutation, &before);
        if json(&semio_framework_pack_json::to_json_string(raised.diff())) != json(&read(&bundle.join("🔺️diff/🔣️.json"))) {
            breaches.push(format!("{name}: the produced diff is not the committed diff"));
        }
    }
    if status == "applied" {
        let steps = inverse_en1996_mutation(mutation, &before).expect("valid retained mutation inverse fixture");
        let restored = steps.iter().fold(applied, |document, step| apply(&document, step).0);
        if steps.is_empty() || restored != before {
            breaches.push(format!("{name}: the mutation's own inverse ({} step(s)) does not restore the before-snapshot", steps.len()));
        }
    }
    let again = apply(&after, mutation).1;
    if again.iter().any(|message| message == "Warning:mutation.no-op") {
        reached.insert("no-op".to_string());
    }
    if refused(&again) || stray(mutation).is_some_and(|target| refused(&apply(&before, &target).1)) {
        reached.insert("rejected".to_string());
    }
    reached.insert(status);
    breaches
}

/// 🧾️ The committed applied vector of the leaf `kind`: its mutation and its before-snapshot.
pub(crate) fn applied_vector(kind: &str) -> (En1996Mutation, En1996Snapshot) {
    let leaf = directories(&root()).into_iter().find(|leaf| leaf.file_name().is_some_and(|name| name.to_string_lossy().trim_start_matches(|c: char| !c.is_ascii_alphabetic()) == kind)).unwrap_or_else(|| panic!("no committed fixtures for {kind}"));
    let bundle = leaf.join("✅apply");
    let mutation = decode_en1996_mutation_json(&read(&bundle.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", bundle.display()));
    (mutation, snapshot(&bundle.join("📸️snapshot/⬅️before/🔣️.json")))
}

/// 🎯️ The canonical assertion one vector's own test makes: its bundle holds with no breach.
fn assert_vector(leaf: &str, scenario: &str) {
    let bundle = root().join(leaf).join(scenario);
    let mutation = decode_en1996_mutation_json(&read(&bundle.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", bundle.display()));
    let breaches = vector_breaches(&bundle, &mutation, &mut BTreeSet::new());
    assert!(breaches.is_empty(), "{}", breaches.join("\n"));
}

/// 💾️ The committed mutation crosses the binary op codec unchanged, framed under its leaf descriptor's protocol tag.
fn binary_breaches(bundle: &Path, mutation: &En1996Mutation) -> Vec<String> {
    let name = bundle.display();
    let bytes = match <En1996Mutation as protocol::OpBinary>::encode_op(mutation) {
        Ok(bytes) => bytes,
        Err(error) => return vec![format!("{name}: the binary op encoding failed: {error}")],
    };
    let mut breaches = Vec::new();
    let declared = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::descriptor(mutation).binary_tag.map(u64::from);
    let mut reader = store::pack_rt::ByteReader::new(&bytes);
    let framed = reader.read_u8().ok().and_then(|_| reader.read_varint_u64().ok());
    if framed != declared {
        breaches.push(format!("{name}: the op frame carries tag {framed:?}, the leaf descriptor declares {declared:?}"));
    }
    match <En1996Mutation as protocol::OpBinary>::decode_op(&bytes) {
        Ok(decoded) if &decoded == mutation => {}
        Ok(decoded) => breaches.push(format!("{name}: the binary op decodes to {decoded:?}")),
        Err(error) => breaches.push(format!("{name}: the binary op does not decode: {error}")),
    }
    breaches
}

#[test]
fn committed_vectors_are_this_implementations_answer() {
    let (mut covered, mut breaches, mut classes) = (BTreeSet::new(), Vec::new(), BTreeMap::new());
    for scenario in directories(&root()).iter().flat_map(|leaf| directories(leaf)) {
        let mutation = decode_en1996_mutation_json(&read(&scenario.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", scenario.display()));
        let descriptor = <En1996Mutation as protocol::Mutation<En1996Snapshot>>::descriptor(&mutation);
        breaches.extend(binary_breaches(&scenario, &mutation));
        if scenario.ends_with("🧾️wire-witness") {
            covered.insert(descriptor.semantic_kind);
            continue;
        }
        let declared: BTreeSet<String> = descriptor.outcome_classes.iter().map(|class| class.as_str().to_string()).collect();
        let (canonical, reached, _) = classes.entry(descriptor.semantic_kind).or_insert_with(|| (false, BTreeSet::new(), declared));
        let found = vector_breaches(&scenario, &mutation, reached);
        let applied = json(&read(&scenario.join("🎯️outcome/🔣️.json")))["status"] == "applied";
        if found.is_empty() && applied {
            covered.insert(descriptor.semantic_kind);
        }
        *canonical |= applied;
        breaches.extend(found);
    }
    for (kind, (canonical, reached, declared)) in &classes {
        if (*canonical && reached != declared) || !reached.is_subset(declared) {
            breaches.push(format!("{kind}: the descriptor declares {declared:?}, production dispatch reaches {reached:?} from the committed vectors"));
        }
    }
    assert!(breaches.is_empty(), "{}", breaches.join("\n"));
    let missing: Vec<&str> = KINDS.iter().copied().filter(|kind| !covered.contains(kind)).collect();
    assert!(missing.is_empty(), "kinds without a committed applied vector or wire witness: {missing:?}");
}

//#region 🧫️CanonicalVectorTests
#[path = "../../🌍️change-annex/🧪️tests/🟰noop/🦀️.rs"]
mod change_annex_noop;
#[path = "../../🎭️change-design-situation/🧪️tests/🟰noop/🦀️.rs"]
mod change_design_situation_noop;
#[path = "../../🏢️change-storeys/🧪️tests/🟰noop/🦀️.rs"]
mod change_storeys_noop;
#[path = "../../🏭️change-masonry-class/🧪️tests/🟰noop/🦀️.rs"]
mod change_masonry_class_noop;
//#endregion 🧫️CanonicalVectorTests
