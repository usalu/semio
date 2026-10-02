"""🧪️ W2-W-norm-2 (outcome law): witnesses each leaf's state-dependent refusal or no-op with a committed vector, keeps
every leaf descriptor's `outcomeClasses` equal to what its diff can produce, and keeps a catalog's scenarios and its
feature's Examples tables equal to the fixture tree. Writes ONLY inside the given norm artifact directory.

  python3 🧪️w2w-norm-2-outcomes.py refusals <artifact-dir-name> <kind>=<slug> ...   e.g. ⚖️en1990 insert-effect=dupe
                                                     (`<kind>=rule:<lexeme>` names the out-of-bounds value)
  python3 🧪️w2w-norm-2-outcomes.py classes <artifact-dir-name>
  python3 🧪️w2w-norm-2-outcomes.py sync <artifact-dir-name> <catalog-id> <case-dir-name>
  python3 🧪️w2w-norm-2-outcomes.py suite <artifact-dir-name>                       one of my four crates' vector suite

A refusal or no-op vector re-applies the leaf's canonical `✅apply` mutation to that vector's own after-snapshot: an
insert then repeats an id the document already holds (`⛔dupe`), a remove addresses the record it already removed
(`❓gone`) and a change sets the value the field already has (`🟰noop`). Before and after are therefore both the
canonical after-snapshot, byte for byte. A `📏clamp` vector is the canonical insert asked for a position past the list's
end: it lands where the canonical one (an append) does, under a `mutation.clamped` warning. A `🚫rule` vector is the
canonical change asked for a value below the bound its leaf schema states (`-1.0`, or the lexeme `rule:<lexeme>` names): a
`mutation.invariant` refusal on the canonical before-snapshot.
"""

import json
import os
import re
import shutil
import sys

ROOT = "/Users/ueli/Documents/semio"
WITNESS = "🧾️wire-witness"
CANONICAL = "✅apply"
#: 🎯️ The vocabulary slug → (case directory, committed outcome).
OUTCOMES = {
    "dupe": ("⛔dupe", {"status": "rejected", "code": "mutation.duplicate-id", "messages": [{"level": "fatal", "code": "mutation.duplicate-id"}]}),
    "gone": ("❓gone", {"status": "rejected", "code": "mutation.target-missing", "messages": [{"level": "error", "code": "mutation.target-missing"}]}),
    "noop": ("🟰noop", {"status": "no-op", "code": "mutation.no-op", "messages": [{"level": "warning", "code": "mutation.no-op"}]}),
    "clamp": ("📏clamp", {"status": "applied", "code": "mutation.clamped", "messages": [{"level": "warning", "code": "mutation.clamped"}]}),
    "rule": ("🚫rule", {"status": "rejected", "code": "mutation.invariant", "messages": [{"level": "fatal", "code": "mutation.invariant"}]}),
}
#: 🚫️ The value a `🚫rule` vector sets: below the positive or non-negative bound its leaf schema states.
OUT_OF_BOUNDS = "-1.0"
#: 📏️ The position a `📏clamp` vector asks for: past the end of every committed example collection.
PAST_END = 99


def subset(artifact):
    return f"{ROOT}/✏️s/🔌️plugins/📕️norm/🗿️artifacts/{artifact}/🏅️standards/🔖️1/🪆️subsets/✳️any"


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    text = json.dumps(value, ensure_ascii=False, indent=2) + "\n"
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def leaves(artifact):
    """🍃️ semantic kind → leaf directory name, read off each leaf descriptor."""
    root = f"{subset(artifact)}/🧬️schema/🧬️mutations"
    found = {}
    for name in sorted(os.listdir(root)):
        descriptor = f"{root}/{name}/🔣️.json"
        if os.path.isfile(descriptor) and "semanticKind" in (data := load(descriptor)):
            found[data["semanticKind"]] = name
    return found


def refusals(artifact, requests):
    """⛔️ Writes one refusal or no-op bundle per `<kind>=<slug>` from the kind's canonical applied vector."""
    known = leaves(artifact)
    fixtures = f"{subset(artifact)}/🧫️fixtures/🧬️mutations"
    for request in requests:
        kind, request_slug = request.split("=")
        slug, _, lexeme = request_slug.partition(":")
        directory, outcome = OUTCOMES[slug]
        source, target = f"{fixtures}/{known[kind]}/{CANONICAL}", f"{fixtures}/{known[kind]}/{directory}"
        assert load(f"{source}/🎯️outcome/🔣️.json")["status"] == "applied", source
        if os.path.exists(target):
            shutil.rmtree(target)
        clamped, ruled = slug == "clamp", slug == "rule"
        for role in ("⬅️before", "➡️after"):
            os.makedirs(f"{target}/📸️snapshot/{role}")
            snapshot = role if clamped else "⬅️before" if ruled else "➡️after"
            shutil.copyfile(f"{source}/📸️snapshot/{snapshot}/🔣️.json", f"{target}/📸️snapshot/{role}/🔣️.json")
        os.makedirs(f"{target}/🦠️mutation")
        shutil.copyfile(f"{source}/🦠️mutation/🔣️.json", f"{target}/🦠️mutation/🔣️.json")
        if clamped:
            text = open(f"{source}/🦠️mutation/🔣️.json", encoding="utf-8").read()
            assert len(re.findall(r'"index": \d+', text)) == 1, source
            open(f"{target}/🦠️mutation/🔣️.json", "w", encoding="utf-8").write(re.sub(r'"index": \d+', f'"index": {PAST_END}', text))
        if ruled:
            text = open(f"{source}/🦠️mutation/🔣️.json", encoding="utf-8").read()
            value = r'("new[A-Za-z0-9]*": )-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?'
            assert len(re.findall(value, text)) == 1, source
            open(f"{target}/🦠️mutation/🔣️.json", "w", encoding="utf-8").write(re.sub(value, lambda match: match.group(1) + (lexeme or OUT_OF_BOUNDS), text))
        dump(f"{target}/🎯️outcome/🔣️.json", outcome)
        if clamped:
            shutil.copytree(f"{source}/🔺️diff", f"{target}/🔺️diff")
        elif outcome["status"] == "rejected":
            os.makedirs(f"{target}/🔺️diff")
            open(f"{target}/🔺️diff/🚫️.absent", "w").close()
        else:
            dump(f"{target}/🔺️diff/🔣️.json", {key: None for key in load(f"{source}/🔺️diff/🔣️.json")})
        print(f"{artifact}: {known[kind]}/{directory}")


def reached(source):
    """🎯️ The outcome classes one diff source can produce, in vocabulary order."""
    return ["applied"] + (["no-op"] if "mutation.no-op" in source else []) + (["rejected"] if re.search(r"MutationOutcome::(error|fatal)\(", source) else [])


def classes(artifact):
    """🏷️ Every leaf descriptor's `outcomeClasses` (and the owner-manifest row mirroring it) from the leaf's diff."""
    root = f"{subset(artifact)}/🧬️schema/🧬️mutations"
    found = {}
    for kind, leaf in leaves(artifact).items():
        diff = f"{root}/{leaf}/🔺️diff/🦀️.rs"
        found[kind] = reached(open(diff if os.path.exists(diff) else f"{root}/{leaf}/🦀️.rs", encoding="utf-8").read())
        descriptor = load(f"{root}/{leaf}/🔣️.json")
        if descriptor["outcomeClasses"] != found[kind]:
            print(f"{artifact} {kind}: {descriptor['outcomeClasses']} -> {found[kind]}")
            descriptor["outcomeClasses"] = found[kind]
            dump(f"{root}/{leaf}/🔣️.json", descriptor)
    manifest = load(f"{subset(artifact)}/🔮️oracles/🔣️.json")
    for owner in manifest.get("mutationManifests", []):
        for row in owner["mutations"]:
            row["outcomes"] = found[row["id"]]
    dump(f"{subset(artifact)}/🔮️oracles/🔣️.json", manifest)


def slug(directory):
    return re.search(r"[a-z0-9][a-z0-9-]*$", directory).group(0)


def scenarios(artifact, leaf):
    """🧫️ A leaf's vector directories, the canonical applied one first."""
    names = sorted(name for name in os.listdir(f"{subset(artifact)}/🧫️fixtures/🧬️mutations/{leaf}") if name != WITNESS)
    return sorted(names, key=lambda name: name != CANONICAL)


def aligned(cells):
    widths = [max(len(row[column]) for row in cells) for column in range(len(cells[0]))]
    return "".join("      | " + " | ".join(cell.ljust(width) for cell, width in zip(row, widths)) + " |\n" for row in cells)


def sync(artifact, catalog_id, case):
    """📇️ The catalog's scenarios (canonical first, then `<kind>-<slug>` rows) and the feature's two Examples tables."""
    path = f"{subset(artifact)}/🔮️oracles/🔣️.json"
    manifest = load(path)
    catalog = next(entry for entry in manifest["mutationCatalogs"] if entry["id"] == catalog_id)
    mutate, inverse = [("id", "dir", "fixture")], [("id", "dir", "fixture")]
    for vector in catalog["vectors"]:
        names = scenarios(artifact, vector["mutationDirectoryName"])
        vector["scenarios"] = [{"id": slug(name) if name == names[0] else f"{vector['mutationId']}-{slug(name)}", "directoryName": name} for name in names]
        for scenario in vector["scenarios"]:
            row = (vector["mutationId"] if scenario["directoryName"] == names[0] else scenario["id"], vector["mutationDirectoryName"], scenario["directoryName"])
            mutate.append(row)
            if scenario["directoryName"] == names[0]:
                inverse.append(row)
    dump(path, manifest)
    feature_path = f"{subset(artifact)}/🧪️tests/{case}/🥒️.feature"
    lines = open(feature_path, encoding="utf-8").read().splitlines(keepends=True)
    out, role, index = [], None, 0
    while index < len(lines):
        line = lines[index]
        role = "mutate" if "@id-mutate" in line else "inverse" if "@id-inverse" in line else role
        out.append(line)
        index += 1
        if line.strip() == "Examples:":
            while index < len(lines) and lines[index].lstrip().startswith("|"):
                index += 1
            out.append(aligned(mutate if role == "mutate" else inverse))
    with open(feature_path, "w", encoding="utf-8") as handle:
        handle.write("".join(out))
    print(f"{artifact}: {len(mutate) - 1} mutate rows, {len(inverse) - 1} inverse rows")


#: 🦀️ Each crate's names for the vector-suite template: (lowercase id, aggregate, snapshot, differential case).
CRATES = {
    "🪨️en1996": ("en1996", "En1996Mutation", "En1996Snapshot", "🪨️mutate-en1996-1"),
    "🌬️din16798": ("din16798", "Din16798Mutation", "Din16798Snapshot", "🌬️mutate-din16798-1"),
    "⚡️din18599": ("din18599", "Din18599Mutation", "Din18599Snapshot", "⚡️mutate-din18599-1"),
    "🧱️din4108": ("din4108", "Din4108Mutation", "Din4108Snapshot", "🧱️mutate-din4108-1"),
}

#: 📝️ What each refusal or no-op case witnesses, for its canonical test's docstring.
WITNESSES = {
    "dupe": "re-applying the canonical insert repeats an id the document already holds, a `mutation.duplicate-id`",
    "gone": "re-applying the canonical remove addresses the record it already removed, a `mutation.target-missing`",
    "noop": "re-applying the canonical change sets the value the field already has, a `mutation.no-op`",
    "clamp": "the canonical insert asked for a position past the list's end lands last under a `mutation.clamped` warning",
    "rule": "the canonical change asked for a value below its leaf schema's bound is a `mutation.invariant` refusal",
}

SUITE = """//! 🧫️ Committed mutation vectors — every `🧫️fixtures/🧬️mutations/<leaf>/<scenario>` bundle is this implementation's own
//! answer, every kind is exercised by an applied vector or a payload-only `🧾️wire-witness`, and every leaf descriptor
//! declares exactly the outcome classes production dispatch reaches from its vectors. The independent Python engine is
//! held to the same bundles by `@CASE@`.

use super::{apply_@ID@_mutation, decode_@ID@_mutation_json, inverse_@ID@_mutation, @MUTATION@, KINDS};
use crate::standards::v1::subsets::any::schema::snapshot::{decode_@ID@_snapshot_json, @SNAPSHOT@};
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

fn snapshot(path: &Path) -> @SNAPSHOT@ {
    decode_@ID@_snapshot_json(&read(path)).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
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
fn stray(mutation: &@MUTATION@) -> Option<@MUTATION@> {
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
    addressed.then(|| mutation.with_payload_value(dsl::DslValue::from(&payload)).ok()).flatten()
}

/// ⚖️ One bundle's breaches, and the outcome classes it shows production dispatch reaching. Every vector lands on its
/// committed after-snapshot under exactly its committed messages: an `applied` one moves the document by exactly the
/// committed diff and its own inverse restores the before-snapshot, a `no-op` one keeps the document under the committed
/// empty diff, and a `rejected` one is refused at error or fatal level and keeps it. Re-applying the mutation to the
/// after-snapshot shows `no-op` reached when it raises `mutation.no-op` and `rejected` when it is refused, as does the
/// mutation addressing a target no document holds.
fn vector_breaches(bundle: &Path, mutation: &@MUTATION@, reached: &mut BTreeSet<String>) -> Vec<String> {
    let name = bundle.display();
    let apply = |document: &@SNAPSHOT@, step: &@MUTATION@| apply_@ID@_mutation(document, step).unwrap_or_else(|error| panic!("{name}: {error}"));
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
        let raised = <@MUTATION@ as protocol::Mutation<@SNAPSHOT@>>::diff(mutation, &before);
        if json(&pack::json::to_json_string(raised.diff())) != json(&read(&bundle.join("🔺️diff/🔣️.json"))) {
            breaches.push(format!("{name}: the produced diff is not the committed diff"));
        }
    }
    if status == "applied" {
        let steps = inverse_@ID@_mutation(mutation, &before);
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

/// 🎯️ The canonical assertion one vector's own test makes: its bundle holds with no breach.
fn assert_vector(leaf: &str, scenario: &str) {
    let bundle = root().join(leaf).join(scenario);
    let mutation = decode_@ID@_mutation_json(&read(&bundle.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", bundle.display()));
    let breaches = vector_breaches(&bundle, &mutation, &mut BTreeSet::new());
    assert!(breaches.is_empty(), "{}", breaches.join("\\n"));
}

/// 💾️ The committed mutation crosses the binary op codec unchanged, framed under its leaf descriptor's protocol tag.
fn binary_breaches(bundle: &Path, mutation: &@MUTATION@) -> Vec<String> {
    let name = bundle.display();
    let bytes = match <@MUTATION@ as protocol::OpBinary>::encode_op(mutation) {
        Ok(bytes) => bytes,
        Err(error) => return vec![format!("{name}: the binary op encoding failed: {error}")],
    };
    let mut breaches = Vec::new();
    let declared = <@MUTATION@ as protocol::Mutation<@SNAPSHOT@>>::descriptor(mutation).binary_tag.map(u64::from);
    let mut reader = store::pack_rt::ByteReader::new(&bytes);
    let framed = reader.read_u8().ok().and_then(|_| reader.read_varint_u64().ok());
    if framed != declared {
        breaches.push(format!("{name}: the op frame carries tag {framed:?}, the leaf descriptor declares {declared:?}"));
    }
    match <@MUTATION@ as protocol::OpBinary>::decode_op(&bytes) {
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
        let mutation = decode_@ID@_mutation_json(&read(&scenario.join("🦠️mutation/🔣️.json"))).unwrap_or_else(|error| panic!("{}: {error}", scenario.display()));
        let descriptor = <@MUTATION@ as protocol::Mutation<@SNAPSHOT@>>::descriptor(&mutation);
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
    assert!(breaches.is_empty(), "{}", breaches.join("\\n"));
    let missing: Vec<&str> = KINDS.iter().copied().filter(|kind| !covered.contains(kind)).collect();
    assert!(missing.is_empty(), "kinds without a committed applied vector or wire witness: {missing:?}");
}
"""


def suite(artifact):
    """🧫️ Renders the crate's vector suite from the shared template and mounts one canonical test per refusal, no-op or
    clamp vector, writing that test where it is missing; the existing canonical mounts are kept."""
    identifier, mutation, snapshot, case = CRATES[artifact]
    sources = f"{subset(artifact)}/🧬️schema/🧬️mutations"
    path = f"{sources}/🧪️tests/🔬️fixture/🦀️.rs"
    current = open(path, encoding="utf-8").read()
    mounts = re.findall(r'#\[path = "[^"]+"\]\nmod \w+;\n', current)
    known = {kind: leaf for kind, leaf in leaves(artifact).items()}
    for kind, leaf in known.items():
        directory = f"{subset(artifact)}/🧫️fixtures/🧬️mutations/{leaf}"
        for scenario in sorted(os.listdir(directory)) if os.path.isdir(directory) else []:
            if scenario in (WITNESS, CANONICAL):
                continue
            mount = f'#[path = "../../{leaf}/🧪️tests/{scenario}/🦀️.rs"]\n'
            test = f"{sources}/{leaf}/🧪️tests/{scenario}/🦀️.rs"
            if not os.path.exists(test):
                os.makedirs(os.path.dirname(test), exist_ok=True)
                emoji = scenario[: len(scenario) - len(slug(scenario))]
                with open(test, "w", encoding="utf-8") as handle:
                    handle.write(f"//! {emoji} Canonical test of the committed `{kind}` vector `{scenario}` — {WITNESSES[slug(scenario)]}.\n\n#[test]\nfn committed_vector_holds() {{\n    super::assert_vector(\"{leaf}\", \"{scenario}\");\n}}\n")
            if not any(existing.startswith(mount) for existing in mounts):
                mounts.append(f"{mount}mod {kind.replace('-', '_')}_{slug(scenario).replace('-', '_')};\n")
    body = SUITE.replace("@ID@", identifier).replace("@MUTATION@", mutation).replace("@SNAPSHOT@", snapshot).replace("@CASE@", case)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(body + "\n//#region 🧫️CanonicalVectorTests\n" + "".join(mounts) + "//#endregion 🧫️CanonicalVectorTests\n")
    print(f"{artifact}: suite rendered, {len(mounts)} canonical test mounts")


if __name__ == "__main__":
    command, artifact = sys.argv[1], sys.argv[2]
    {"refusals": lambda: refusals(artifact, sys.argv[3:]), "classes": lambda: classes(artifact), "sync": lambda: sync(artifact, *sys.argv[3:5]), "suite": lambda: suite(artifact)}[command]()
