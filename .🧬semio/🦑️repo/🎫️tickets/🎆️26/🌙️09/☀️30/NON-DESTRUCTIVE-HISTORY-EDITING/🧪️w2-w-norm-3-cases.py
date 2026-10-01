"""🧪️ W2-W-norm-3 follow-up: restores a norm subset's `mutate-<std>-1` case on its CURRENT vocabulary — the EN 1998 recipe,
generalised (`🧪️w2-w-norm-3-en1998-vectors.py` stays the EN 1998 source).

`python3 🧪️w2-w-norm-3-cases.py <std> <command> [arg]`, `<std>` ∈ en1992 en1993 en1994 en1995 en1997 en1998 en1999 iso16757 vdi3805.
Every case directory is named after the outcome class it witnesses (design §14, option B): `✅apply` for the kind's
applied vector, and for a leaf whose `🔺️diff` refuses by state a `<emoji><slug>` refusal row `<kind>-<slug>` that re-applies
the applied mutation to the after-snapshot it produced (`⛔dupe` re-inserts, `❓gone` re-removes, `🟰noop` re-sets):

* `plan`            — one vector per kind: source (committed vector or wire witness), base, scenario name, path budget.
* `stage <dump>`    — writes every vector's `🦠️mutation` (schema-canonical numbers), `🎯️outcome` and `⬅️before`, one
                      canonical test per vector (`<leaf>/🧪️tests/<scenario>/🦀️.rs`) and the `🔬️fixture` list; removes the
                      superseded fixtures, witnesses, leaf tests and the `🧾️WireWitness` module, and wires a temporary
                      `[DEBUG]` settle test.
* `settle <capture>` — writes the Rust-canonical `⬅️before`, `➡️after` and `🔺️diff` the settle test printed, then unwires it.
* `surface`         — rewrites the oracle registry (catalog, manifest, rationale), the feature, the Python and the Rust
                      adapter from the same vector list.
"""
import copy
import importlib.util
import json
import os
import re
import shutil
import sys

TICKET = os.path.dirname(os.path.abspath(__file__))
NORM = "✏️s/🔌️plugins/📕️norm/🗿️artifacts"
EN1998_CASE = f"{NORM}/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🫨️mutate-en1998-1"
EN1998_ORACLE = f"{NORM}/🫨️en1998/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json"

CONFIG = {
    "en1992": dict(dir="🏛️en1992", label="EN 1992", rust="En1992", base="🛢️liquid-retaining-fem-anchor", dsl="🛢️liquid-retaining-fem-anchor/🛢️liquid-retaining-fem-anchor/🗣️.dsl.semio", pack="🛢️liquid-retaining-fem-anchor/🎒️.pack.semio", what="a liquid-retaining RC wall with a post-installed anchor"),
    "en1993": dict(dir="🔩️en1993", label="EN 1993", rust="En1993", base="🔩️high-strength-connection", dsl="🔩️high-strength-connection/🔩️high-strength-connection/🗣️.dsl.semio", pack="🔩️high-strength-connection/🎒️.pack.semio", what="a high-strength bolted steel connection"),
    "en1994": dict(dir="🧩️en1994", label="EN 1994", rust="En1994", base=None, dsl="🌉️composite-bridge-girder/🌉️composite-bridge-girder/🗣️.dsl.semio", pack="🌉️composite-bridge-girder/🎒️.pack.semio", what="a steel-concrete composite bridge girder"),
    "en1995": dict(dir="🪵️en1995", label="EN 1995", rust="En1995", base=None, dsl="🏠️glulam-floor-beam/🏠️glulam-floor-beam/🗣️.dsl.semio", pack="🏠️glulam-floor-beam/🎒️.pack.semio", what="a glued-laminated timber floor beam"),
    "en1998": dict(dir="🫨️en1998", label="EN 1998", rust="En1998", base=None, dsl="🏢️seismic-rc-frame/🏢️seismic-rc-frame/🗣️.dsl.semio", pack="🏢️seismic-rc-frame/🎒️.pack.semio", what="a seismic reinforced-concrete frame, bridges, silos, tanks, foundations, walls and towers"),
    "en1997": dict(dir="🌍️en1997", label="EN 1997", rust="En1997", base="🎬️demo", dsl="🎬️demo/🗣️.dsl.semio", pack="🎬️demo/📦️.pack.semio", what="a spread foundation, piles, a retaining wall and a slope on layered soil"),
    "en1999": dict(dir="🪶️en1999", label="EN 1999", rust="En1999", base="🏠️aluminium-roof-purlin", dsl="🏠️aluminium-roof-purlin/🏠️aluminium-roof-purlin/🗣️.dsl.semio", pack="🏠️aluminium-roof-purlin/🎒️.pack.semio", what="an aluminium roof purlin with its connections"),
    "iso16757": dict(dir="📇️iso16757", label="ISO 16757", rust="Iso16757", base=None, dsl="🎬️demo/🗣️.dsl.semio", pack="🎬️demo/🎒️.pack.semio", what="a building-services product catalogue"),
    "vdi3805": dict(dir="🏭️vdi3805", label="VDI 3805", rust="Vdi3805", base=None, dsl="🎬️demo/🗣️.dsl.semio", pack=None, what="a VDI 3805 manufacturer product data file"),
}
APPLY = "✅apply"

#: 🏷️ The shared case vocabulary (design §14): slug → (emoji, outcome status, frozen code, message level).
CLASSES = {
    "dupe": ("⛔", "rejected", "mutation.duplicate-id", "fatal"),
    "gone": ("❓", "rejected", "mutation.target-missing", "error"),
    "noop": ("🟰", "no-op", "mutation.no-op", "warning"),
}

#: ⛔️ The refusal witnesses: every leaf of this scope whose `🔺️diff` gained its state-dependent detection for the
#: outcome law, with the class re-applying its applied mutation to its own after-snapshot witnesses.
REFUSALS = {
    "en1992": {"insert-anchor": "dupe", "remove-anchor": "gone"},
    "en1993": {f"insert-{noun}": "dupe" for noun in ("bridge-fatigue", "cold-formed-member", "crane-runway", "fatigue-detail", "fire-exposure", "joint", "load-case", "material", "member", "member-action", "pile", "plated-panel", "section", "silo-shell", "tension-component", "tower-leg")},
    "en1995": {"insert-connection": "dupe", "insert-member": "dupe"},
    "en1997": {"insert-footing": "dupe", "insert-layer": "dupe", "insert-pile": "dupe"},
    "en1998": {**{f"insert-{noun}": "dupe" for noun in ("building", "bridge", "assessment", "tower", "tank", "retaining-wall", "foundation", "silo")}, "update-site": "noop"},
    "en1999": {"add-member": "dupe"},
}


def load(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


WITNESSES = load("witnesses", os.path.join(TICKET, "🧪️w2-w-norm-3-witnesses.py"))


class Artifact:
    """📕️ One norm subset's coordinates."""

    def __init__(self, std):
        self.std, self.config = std, CONFIG[std]
        self.subset = f"{NORM}/{self.config['dir']}/🏅️standards/🔖️1/🪆️subsets/✳️any"
        self.mutations = f"{self.subset}/🧬️schema/🧬️mutations"
        self.fixtures = f"{self.subset}/🧫️fixtures/🧬️mutations"
        self.case = next(f"{self.subset}/🧪️tests/{name}" for name in os.listdir(f"{self.subset}/🧪️tests") if "mutate-" in name)
        self.oracle = f"{self.subset}/🔮️oracles/🔣️.json"
        self.rust, self.label = self.config["rust"], self.config["label"]
        aggregate = open(f"{self.mutations}/🦀️.rs", encoding="utf-8").read()
        self.kinds = re.search(r"pub const KINDS: &\[&str\] = &\[(.*?)\];", aggregate, re.S).group(1)
        self.kinds = re.findall(r'"([^"]+)"', self.kinds)
        self.external = f'#[value(tag = "mutation"' not in aggregate
        self.leaves = {}
        for leaf in os.listdir(self.mutations):
            descriptor = f"{self.mutations}/{leaf}/🔣️.json"
            if os.path.exists(descriptor) and os.path.exists(f"{self.mutations}/{leaf}/🧬️schema/🔣️.json"):
                self.leaves[json.load(open(descriptor, encoding="utf-8"))["semanticKind"]] = leaf
        assert sorted(self.leaves) == sorted(self.kinds), (sorted(set(self.kinds) ^ set(self.leaves)))

    def budget(self, leaf, scenario):
        size = lambda text: len(text.encode("utf-8"))
        return max(size(f"{self.subset}/🧬️schema/🧬️mutations/{leaf}/🧪️tests/{scenario}") + 11, size(f"{self.subset}/🧫️fixtures/🧬️mutations/{leaf}/{scenario}") + 42)


def kebab(text):
    text = re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "-", text)
    return re.sub(r"-+", "-", re.sub(r"[^a-z0-9]+", "-", text.lower())).strip("-")


def split_emoji(name):
    match = re.match(r"^([^a-z0-9]+)(.*)$", name)
    return (match.group(1), match.group(2)) if match else ("", name)


def canonical(artifact, leaf, wire):
    schema = json.load(open(f"{artifact.mutations}/{leaf}/🧬️schema/🔣️.json", encoding="utf-8"))
    resolve = WITNESSES.schema_types(None, WITNESSES.documents_for(f"{artifact.subset}/🧬️schema"))
    if artifact.external:
        variant, payload = next(iter(wire.items()))
        return {variant: WITNESSES.canonical(payload, schema, schema, resolve)}
    payload = {key: value for key, value in wire.items() if key != "mutation"}
    return {"mutation": schema["properties"]["mutation"]["const"], **WITNESSES.canonical(payload, schema, schema, resolve)}


def payload_of(artifact, wire):
    return next(iter(wire.values())) if artifact.external else {key: value for key, value in wire.items() if key != "mutation"}


def sources(artifact):
    """🧫️ The committed evidence per kind: an applied vector (its mutation, before and after) or a wire witness (mutation only)."""
    found = {}
    if not os.path.isdir(artifact.fixtures):
        return found
    for leaf in os.listdir(artifact.fixtures):
        for scenario in sorted(os.listdir(f"{artifact.fixtures}/{leaf}")):
            stem = f"{artifact.fixtures}/{leaf}/{scenario}"
            if not os.path.exists(f"{stem}/🦠️mutation/🔣️.json"):
                continue
            if os.path.exists(f"{stem}/🎯️outcome/🔣️.json") and json.load(open(f"{stem}/🎯️outcome/🔣️.json", encoding="utf-8")).get("status") != "applied":
                continue
            kind = next((kind for kind, directory in artifact.leaves.items() if directory == leaf), None)
            if kind is None or (kind in found and found[kind]["before"] is not None):
                continue
            read = lambda tail: json.load(open(f"{stem}/{tail}", encoding="utf-8")) if os.path.exists(f"{stem}/{tail}") else None
            found[kind] = {"wire": read("🦠️mutation/🔣️.json"), "before": read("📸️snapshot/⬅️before/🔣️.json"), "after": read("📸️snapshot/➡️after/🔣️.json"), "old": None if scenario == "🧾️wire-witness" else scenario}
    return found


def with_second_member(base):
    base = copy.deepcopy(base)
    first = base["members"][0]
    second = {**copy.deepcopy(first), "id": first["id"] + "-2"}
    base["members"] = [first, second] + [member for member in base["members"][1:] if member["id"] != second["id"]]
    return base


#: ✍️ Vectors whose committed evidence would not move the document (the value equals the base) or needs a richer base.
OVERRIDES = {
    ("en1992", "change-member-stirrup-spacing"): lambda payload, base: ({**payload, "newSpacing": 0.2}, base),
    ("en1992", "reorder-members"): lambda payload, base: ({**payload, "fromIndex": 1, "toIndex": 0}, with_second_member(base)),
    ("en1999", "change-weld-throat"): lambda payload, base: ({**payload, "newThroat": 0.006}, base),
    ("en1994", "change-beam-stud-spacing-m"): lambda payload, base: ({**payload, "newSpacingM": 0.15}, base),
    ("en1999", "change-cold-formed"): lambda payload, base: (amend(payload, "coldFormed", thickness=0.004), base),
    ("en1999", "change-fire-scenarios"): lambda payload, base: (amend(payload, "fireScenarios", thetaA=250.0), base),
    ("en1999", "change-shells"): lambda payload, base: (amend(payload, "shells", thickness=0.012), base),
    ("en1999", "change-connections"): lambda payload, base: (amend(payload, "connections", welds={**payload["connections"][0]["welds"], "throat": 0.006}), base),
    ("en1999", "change-fatigue-details"): lambda payload, base: (amend(payload, "fatigueDetails", deltaSigmaEd=40000000.0), base),
}


def amend(payload, collection, **fields):
    payload = copy.deepcopy(payload)
    payload[collection][0].update(fields)
    return payload


def outcome(slug):
    _emoji, status, code, level = CLASSES[slug]
    return {"status": status, "code": code, "messages": [{"level": level, "code": code}]}


def plan(artifact, dump=None):
    evidence = sources(artifact)
    base = json.load(open(os.path.join(dump, artifact.std, artifact.config["base"] + ".json"), encoding="utf-8")) if dump and artifact.config["base"] else None
    rows = []
    for kind in artifact.kinds:
        leaf, source = artifact.leaves[kind], evidence.get(kind)
        assert source is not None, f"{artifact.std}: no committed evidence for {kind}"
        payload = payload_of(artifact, source["wire"])
        before = source["before"] if source["before"] is not None else base
        if (artifact.std, kind) in OVERRIDES:
            payload, before = OVERRIDES[(artifact.std, kind)](payload, before)
            source = {**source, "wire": {next(iter(source["wire"])): payload} if artifact.external else {"mutation": source["wire"]["mutation"], **payload}, "after": None}
        assert artifact.budget(leaf, APPLY) <= 240, f"{artifact.std}: {leaf}/{APPLY} exceeds the path budget"
        wire = canonical(artifact, leaf, source["wire"])
        rows.append({"row": kind, "kind": kind, "leaf": leaf, "scenario": APPLY, "wire": wire, "before": before, "outcome": {"status": "applied"}, "origin": "vector" if source["old"] else "witness"})
        slug = REFUSALS.get(artifact.std, {}).get(kind)
        if slug is None:
            continue
        if source["after"] is None or source["old"] != APPLY:
            print(f"{artifact.std}: {kind}-{slug} waits for the settled {APPLY} after-snapshot")
            continue
        scenario = CLASSES[slug][0] + slug
        assert artifact.budget(leaf, scenario) <= 240, f"{artifact.std}: {leaf}/{scenario} exceeds the path budget"
        rows.append({"row": f"{kind}-{slug}", "kind": kind, "leaf": leaf, "scenario": scenario, "wire": wire, "before": source["after"], "outcome": outcome(slug), "origin": "vector"})
    return rows


TEST = '''//! 🧪️ Committed vector `{leaf}` / `{scenario}`: the canonical wire of the op reaches the committed after-snapshot and diff, and its own inverse restores the before-snapshot.
use crate::{{{rust}Diff, {rust}Mutation, {rust}Snapshot}};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🦠️mutation/🔣️.json");
const DIFF: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🔺️diff/🔣️.json");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🎯️outcome/🔣️.json");
fn before() -> {rust}Snapshot {{ pack::json::from_json_str(BEFORE).expect("before") }}
fn mutation() -> {rust}Mutation {{ pack::json::from_json_str(MUTATION).expect("mutation") }}
fn apply(mutation: &{rust}Mutation, base: &{rust}Snapshot) -> {rust}Snapshot {{
    let raised = <{rust}Mutation as protocol::Mutation<{rust}Snapshot>>::diff(mutation, base);
    assert!(raised.messages().is_empty(), "{kind} raised {{:?}}", raised.messages());
    <{rust}Diff as protocol::MutationDiff<{rust}Snapshot>>::apply(raised.diff(), base).expect("apply")
}}
#[test]
fn mutation_is_the_canonical_wire() {{
    let _: {rust}Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}}
#[test]
fn applies_to_committed_after_and_diff() {{
    let base = before();
    let raised = <{rust}Mutation as protocol::Mutation<{rust}Snapshot>>::diff(&mutation(), &base);
    assert_eq!(*raised.diff(), pack::json::from_json_str::<{rust}Diff>(DIFF).expect("diff"));
    let after = apply(&mutation(), &base);
    assert_ne!(after, base, "{kind} must move the document");
    assert_eq!(after, pack::json::from_json_str::<{rust}Snapshot>(AFTER).expect("after"));
}}
#[test]
fn inverse_restores_before() {{
    let base = before();
    let inverse = <{rust}Mutation as protocol::Mutation<{rust}Snapshot>>::inverse(&mutation(), &base);
    assert!(!inverse.is_empty(), "{kind} changes the document, so its inverse must not be empty");
    let restored = inverse.iter().fold(apply(&mutation(), &base), |snapshot, step| apply(step, &snapshot));
    assert_eq!(restored, base);
}}
#[test]
fn declared_outcome_holds() {{
    assert_eq!(serde_json::from_str::<serde_json::Value>(OUTCOME).expect("outcome")["status"], "applied");
}}
'''

REFUSAL_TEST = '''//! 🧪️ Committed vector `{leaf}` / `{scenario}`: re-applying the op to the after-snapshot it already produced is refused with `{code}` ({level}) and leaves the document untouched.
use crate::{{{rust}Diff, {rust}Mutation, {rust}Snapshot}};
const BEFORE: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🦠️mutation/🔣️.json");
const DIFF_ABSENT: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🔺️diff/🚫️.absent");
const OUTCOME: &str = include_str!("../../../../../🧫️fixtures/🧬️mutations/{leaf}/{scenario}/🎯️outcome/🔣️.json");
fn before() -> {rust}Snapshot {{ pack::json::from_json_str(BEFORE).expect("before") }}
fn mutation() -> {rust}Mutation {{ pack::json::from_json_str(MUTATION).expect("mutation") }}
#[test]
fn mutation_is_the_canonical_wire() {{
    let _: {rust}Mutation = store::os_store::test_support::assert_wire_witness(MUTATION);
}}
#[test]
fn refuses_with_the_declared_code() {{
    let raised = <{rust}Mutation as protocol::Mutation<{rust}Snapshot>>::diff(&mutation(), &before());
    let outcome: serde_json::Value = serde_json::from_str(OUTCOME).expect("outcome");
    let raised_codes: Vec<(String, String)> = raised.messages().iter().map(|message| (format!("{{:?}}", message.level).to_lowercase(), message.code.0.clone())).collect();
    assert_eq!(raised_codes, vec![(outcome["messages"][0]["level"].as_str().expect("level").to_string(), outcome["code"].as_str().expect("code").to_string())]);
    assert_eq!(outcome["status"], "{status}");
    assert_eq!(*raised.diff(), {rust}Diff::default());
    assert!(DIFF_ABSENT.is_empty());
}}
#[test]
fn leaves_the_document_untouched() {{
    let raised = <{rust}Mutation as protocol::Mutation<{rust}Snapshot>>::diff(&mutation(), &before());
    let after = <{rust}Diff as protocol::MutationDiff<{rust}Snapshot>>::apply(raised.diff(), &before()).expect("apply");
    assert_eq!(after, before());
    assert_eq!(pack::json::from_json_str::<{rust}Snapshot>(AFTER).expect("after"), before());
}}
'''

SETTLE = '''//! 🧾️ [DEBUG] temporary vector settle (W2-W-norm-3): prints each staged vector's canonical before, after and diff.
use crate::{{{rust}Diff, {rust}Mutation, {rust}Snapshot}};

#[test]
fn debug_settle_vectors() {{
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    for leaf in std::fs::read_dir(&root).expect("fixtures").flatten() {{
        for scenario in std::fs::read_dir(leaf.path()).expect("leaf").flatten() {{
            let directory = scenario.path();
            let (Ok(before), Ok(mutation)) = (std::fs::read_to_string(directory.join("📸️snapshot/⬅️before/🔣️.json")), std::fs::read_to_string(directory.join("🦠️mutation/🔣️.json"))) else {{ continue }};
            let base: {rust}Snapshot = pack::json::from_json_str(&before).expect("before");
            let op: {rust}Mutation = pack::json::from_json_str(&mutation).expect("mutation");
            let raised = <{rust}Mutation as protocol::Mutation<{rust}Snapshot>>::diff(&op, &base);
            let codes = raised.messages().iter().map(|message| format!("{{:?}}:{{}}", message.level, message.code.0)).collect::<Vec<_>>().join(",");
            let after = <{rust}Diff as protocol::MutationDiff<{rust}Snapshot>>::apply(raised.diff(), &base);
            match after {{
                Ok(after) if raised.messages().is_empty() => println!("[DEBUG] VECTOR {{}}/{{}} {{}} {{}} {{}}", leaf.file_name().to_string_lossy(), scenario.file_name().to_string_lossy(), pack::json::to_json_string(&base), pack::json::to_json_string(&after), pack::json::to_json_string(raised.diff())),
                _ => println!("[DEBUG] REFUSED {{}}/{{}} {{}} {{codes}}", leaf.file_name().to_string_lossy(), scenario.file_name().to_string_lossy(), pack::json::to_json_string(&base)),
            }}
        }}
    }}
}}
'''


def pretty(value):
    return json.dumps(value, ensure_ascii=False, indent=2) + "\n"


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def module(row):
    return "vector_" + row.replace("-", "_")


def region(source, name, body):
    pattern = re.compile(r"\n//#region " + re.escape(name) + r"\n.*?//#endregion " + re.escape(name) + r"\n", re.S)
    source = pattern.sub("\n", source).rstrip("\n") + "\n"
    return source + (f"\n//#region {name}\n{body}//#endregion {name}\n" if body else "")


def rewire(artifact, fixture=True, settle=False):
    path = f"{artifact.mutations}/🦀️.rs"
    source = open(path, encoding="utf-8").read()
    source = region(source, "🧾️WireWitness", "")
    source = re.sub(r'\n#\[cfg\(test\)\]\n#\[path = "🧪️tests/🔬️fixture/🦀️.rs"\]\nmod fixture_tests;\n', "\n", source)
    source = region(source, "🧫️Vectors", '#[cfg(test)]\n#[path = "🧪️tests/🔬️fixture/🦀️.rs"]\nmod fixture_tests;\n' if fixture else "")
    source = region(source, "🧾️Settle", '#[cfg(test)]\n#[path = "🧪️tests/🧪️settle/🦀️.rs"]\nmod settle;\n' if settle else "")
    write(path, source)


def stage(artifact, dump):
    rows = plan(artifact, dump)
    shutil.rmtree(artifact.fixtures, ignore_errors=True)
    shutil.rmtree(f"{artifact.mutations}/🧪️tests/🧪️wire-witness", ignore_errors=True)
    modules = []
    for row in rows:
        tests = f"{artifact.mutations}/{row['leaf']}/🧪️tests"
        if row["outcome"]["status"] == "applied":
            shutil.rmtree(tests, ignore_errors=True)
        stem = f"{artifact.fixtures}/{row['leaf']}/{row['scenario']}"
        write(f"{stem}/🦠️mutation/🔣️.json", pretty(row["wire"]))
        write(f"{stem}/🎯️outcome/🔣️.json", pretty(row["outcome"]))
        write(f"{stem}/📸️snapshot/⬅️before/🔣️.json", pretty(row["before"]))
        if row["outcome"]["status"] == "applied":
            write(f"{tests}/{row['scenario']}/🦀️.rs", TEST.format(leaf=row["leaf"], scenario=row["scenario"], kind=row["kind"], rust=artifact.rust))
        else:
            write(f"{tests}/{row['scenario']}/🦀️.rs", REFUSAL_TEST.format(leaf=row["leaf"], scenario=row["scenario"], rust=artifact.rust, status=row["outcome"]["status"], code=row["outcome"]["code"], level=row["outcome"]["messages"][0]["level"]))
        modules.append(f'#[path = "../../{row["leaf"]}/🧪️tests/{row["scenario"]}/🦀️.rs"]\nmod {module(row["row"])};\n')
    write(f"{artifact.mutations}/🧪️tests/🔬️fixture/🦀️.rs", f"//! 🧫️ One canonical test per committed {artifact.label} specification vector.\n" + "".join(modules))
    write(f"{artifact.mutations}/🧪️tests/🧪️settle/🦀️.rs", SETTLE.format(rust=artifact.rust))
    rewire(artifact, fixture=False, settle=True)
    json.dump(rows, open(os.path.join(TICKET, "🗑️generated", "w2w-norm3", f"plan-{artifact.std}.json"), "w", encoding="utf-8"), ensure_ascii=False)
    print(f"{artifact.std}: staged {len(rows)} vectors ({sum(row['origin'] == 'vector' for row in rows)} from committed vectors, {sum(row['origin'] == 'witness' for row in rows)} from wire witnesses)")


def settle(artifact, capture):
    rows = {f"{row['leaf']}/{row['scenario']}": row for row in json.load(open(os.path.join(TICKET, "🗑️generated", "w2w-norm3", f"plan-{artifact.std}.json"), encoding="utf-8"))}
    vector = re.compile(r"^\[DEBUG\] VECTOR (\S+) (\{.*\}) (\{.*\}) (\{.*\})$")
    refusal = re.compile(r"^\[DEBUG\] REFUSED (\S+) (\{.*\}) (\S*)$")
    settled, failures = 0, []
    for line in open(capture, encoding="utf-8"):
        line = line.rstrip("\n")
        applied, refused = vector.match(line), refusal.match(line)
        match = applied or refused
        if not match:
            continue
        row, stem = rows.get(match.group(1)), f"{artifact.fixtures}/{match.group(1)}"
        if row is None:
            failures.append(f"unplanned {match.group(1)}")
            continue
        expected = row["outcome"]
        if applied and expected["status"] == "applied":
            write(f"{stem}/📸️snapshot/⬅️before/🔣️.json", pretty(json.loads(match.group(2))))
            write(f"{stem}/📸️snapshot/➡️after/🔣️.json", pretty(json.loads(match.group(3))))
            write(f"{stem}/🔺️diff/🔣️.json", pretty(json.loads(match.group(4))))
            settled += 1
        elif refused and expected["status"] != "applied" and f"{expected['messages'][0]['level'].capitalize()}:{expected['code']}" in match.group(3).split(","):
            write(f"{stem}/📸️snapshot/⬅️before/🔣️.json", pretty(json.loads(match.group(2))))
            write(f"{stem}/📸️snapshot/➡️after/🔣️.json", pretty(json.loads(match.group(2))))
            write(f"{stem}/🔺️diff/🚫️.absent", "")
            settled += 1
        else:
            failures.append(f"{match.group(1)} declared {expected['status']} {expected.get('code', '')}, Rust: {line[:300]}")
    if not failures and settled == len(rows):
        shutil.rmtree(f"{artifact.mutations}/🧪️tests/🧪️settle", ignore_errors=True)
        rewire(artifact, fixture=True, settle=False)
    print(f"{artifact.std}: settled {settled} of {len(rows)}, failures {len(failures)}")
    for failure in failures:
        print("  ", failure[:400])


def surface(artifact):
    rows = json.load(open(os.path.join(TICKET, "🗑️generated", "w2w-norm3", f"plan-{artifact.std}.json"), encoding="utf-8"))
    std, number, label, rust = artifact.std, artifact.std, artifact.label, artifact.rust
    kinds = [row["kind"] for row in rows if row["row"] == row["kind"]]
    descriptors = {row["kind"]: json.load(open(f"{artifact.mutations}/{row['leaf']}/🔣️.json", encoding="utf-8")) for row in rows}
    template = json.load(open(EN1998_ORACLE, encoding="utf-8"))
    current = json.load(open(artifact.oracle, encoding="utf-8"))
    oracle = current if "mutationCatalogs" in current and current.get("oracles") else json.loads(json.dumps(template, ensure_ascii=False).replace("en1998", std).replace("EN 1998", label).replace("🫨️mutate-", artifact.case.rsplit("/", 1)[1].split("mutate-")[0] + "mutate-"))
    catalog = oracle["mutationCatalogs"][0]
    catalog["kinds"] = kinds
    catalog["vectors"] = [{"mutationId": kind, "sourceMutationDirectoryName": artifact.leaves[kind], "mutationDirectoryName": artifact.leaves[kind], "scenarios": [{"id": "apply" if row["row"] == kind else row["row"], "directoryName": row["scenario"]} for row in rows if row["kind"] == kind]} for kind in kinds]
    manifest = oracle["mutationManifests"][0]
    shape = copy.deepcopy(manifest["mutations"][0])
    manifest["mutations"] = [{**shape, "id": kind, "capability": catalog["capability"], "outcomes": descriptors[kind]["outcomeClasses"], "productionDispatch": {"operation": kind, "bridgeVersion": 1, "variant": descriptors[kind]["aggregateVariant"]}, "oracleRequirements": [{"capability": catalog["capability"], "qualifyingKind": "verified-native-second-implementation"}]} for kind in kinds]
    entry = oracle["oracles"][0]
    entry["rationale"] = re.sub(r"all \d+ kinds of", f"all {len(kinds)} kinds of", entry["rationale"])
    entry["rationale"] = re.sub(r"for its \d+-kind mutation vocabulary", f"for its {len(kinds)}-kind mutation vocabulary", entry["rationale"])
    entry["rationale"] = re.sub(r"with \d+ committed fixture vector\(s\)", f"with {len(rows)} committed fixture vector(s)", entry["rationale"])
    entry.setdefault("nativeSecondImplementation", {}).setdefault("fixtureCoverage", {})["vectors"] = len(rows)
    write(artifact.oracle, json.dumps(oracle, ensure_ascii=False, indent=2) + "\n")
    feature(artifact, rows)
    python(artifact, rows)
    adapter(artifact, rows)
    print(f"{std}: surfaced {len(kinds)} kinds")


def table(rows):
    cells = [("id", "dir", "fixture")] + [(row["row"], row["leaf"], row["scenario"]) for row in rows]
    width = [max(len(entry[i]) for entry in cells) for i in range(3)]
    return "".join("      | " + " | ".join(cell.ljust(width[i]) for i, cell in enumerate(entry)) + " |\n" for entry in cells)


def refusal_note(refusals):
    if not refusals:
        return ""
    classes = sorted({row["scenario"] for row in refusals})
    return f"""

  The {len(refusals)} refusal rows ({", ".join(f"`{name}`" for name in classes)}) re-apply a kind's applied mutation to the
  after-snapshot it produced: re-inserting an id the collection now holds must be refused `mutation.duplicate-id`
  (Fatal), re-removing a member that is gone `mutation.target-missing` (Error) and re-setting a value the document
  already has must report `mutation.no-op` (Warning). Both sides must refuse under the committed code and leave the
  document bit-identical; a refusal has nothing to undo, so these rows are `mutate-` only."""


def feature(artifact, rows):
    std, label = artifact.std, artifact.label
    applied = [row for row in rows if row["row"] == row["kind"]]
    refusals = [row for row in rows if row["row"] != row["kind"]]
    verbs = {}
    for row in applied:
        verbs.setdefault(row["kind"].split("-")[0], 0)
        verbs[row["kind"].split("-")[0]] += 1
    shape = ", ".join(f"{count} `{verb}`" for verb, count in sorted(verbs.items(), key=lambda item: -item[1]))
    pack = f"\n    And its committed binary twin asset://{artifact.config['pack']}" if artifact.config["pack"] else ""
    twin = " The Rust side additionally proves it PARSED the document: the committed binary\n  twin must decode to the same document as the text, through a separately written codec." if artifact.config["pack"] else ""
    text = f"""@capability-{std}-1-mutate
@oracle-{std}-1-python-independent
@comparison-ordered-json-v1
@mutations-{std}-1-any
Feature: Apply every typed {label} mutation against an independent Python implementation
  `s.norm.{std}` is a semio-NATIVE artifact and no third party reads or writes it, so the second producer a
  differential comparison needs is a second IMPLEMENTATION: the shared norm reference engine
  (`✏️s/🔌️plugins/📕️norm/🔮️oracles/🏃️execution/🐍️.py`), which `🐍️.py` beside this file feeds with this subset's
  kind list, vectors and carrier. It is written from the repository's own specification of what a semantic
  mutation means (the verb table, the `new<Field>` naming mechanic, the addressing convention and the derivation
  rules) and imports nothing from the Rust it judges.

  Both implementations read the SAME committed bytes: every `(before, mutation, after, outcome)` path below is a
  declared `shared://` fixture, so neither side holds a transcription that could drift. The {len(applied)} `✅apply` vectors
  cover every kind of the current vocabulary ({shape}) on {artifact.config['what']}; each vector's after-snapshot and
  diff were written by production dispatch and its mutation is the canonical Rust wire. Each side asserts the same
  laws in role — the applied document must BE the committed after-snapshot, an `applied` vector must move the
  document, and the mutation followed by its OWN computed inverse must restore the before-snapshot exactly, list
  position included. `parity` adds that two implementations, in two languages, reach the same document.{refusal_note(refusals)}

  `inverse-` projects BOTH the mutated and the restored document, so the mutated half distinguishes the rows.

  ⚠️ Honest boundary — the CARRIER. `identity-round-trip` reads the committed
  `asset://{artifact.config['dsl']}`. The carrier has no published grammar: the committed
  `📖️component.grammar.semio` is the repository-wide `payload = OCTET+` placeholder, so the two sides are compared
  at the envelope preamble, the ordered lines and the digest and length of what each re-emitted.{twin}

  @id-mutate
  @level-exhaustive
  @mode-differential
  Scenario Outline: Apply <id> to its committed specification vector
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When both implementations apply the committed mutation to the committed before-snapshot
    Then each reaches the committed after-snapshot under the committed outcome status and the two agree
    Examples:
{table(rows)}
  @id-inverse
  @level-exhaustive
  @mode-differential
  Scenario Outline: Undoing <id> restores its committed before-snapshot
    Given the committed before-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/⬅️before/🔣️.json
    And the committed mutation payload shared://🧬️mutations/<dir>/<fixture>/🦠️mutation/🔣️.json
    And the committed after-snapshot shared://🧬️mutations/<dir>/<fixture>/📸️snapshot/➡️after/🔣️.json
    And the committed outcome shared://🧬️mutations/<dir>/<fixture>/🎯️outcome/🔣️.json
    When each implementation applies the committed mutation and then its OWN computed inverse
    Then both restore the before-snapshot and agree on the mutated and the restored document
    Examples:
{table(applied)}
  @id-identity-round-trip
  @level-long
  @mode-round-trip
  Scenario: Re-emit the real committed {label} document from the parsed carrier
    Given the real committed text artifact asset://{artifact.config['dsl']}{pack}
    When each implementation parses the artifact and prints it back to its canonical carrier bytes
    Then both reproduce the committed file byte for byte and agree on the parsed fields and the digest of what they emitted
"""
    write(f"{artifact.case}/🥒️.feature", text)


def python(artifact, rows):
    template = open(f"{EN1998_CASE}/🐍️.py", encoding="utf-8").read()
    head, rest = template.split("KINDS = [", 1)
    _, tail = rest.split("#: 🗣️ The real committed EN 1998 document", 1)
    tail = "#: 🗣️ The real committed EN 1998 document" + tail
    applied = [row for row in rows if row["row"] == row["kind"]]
    body = "KINDS = [\n" + "".join(f'    "{row["kind"]}",\n' for row in applied) + "]\n\n#: 🧫️ The committed specification vectors: each kind's `✅apply` vector as (triad directory, fixture), and each refusal row\n#: `<kind>-<slug>` as (kind, triad directory, fixture).\nVECTORS = {\n" + "".join(f'    "{row["row"]}": ("{row["leaf"]}", "{row["scenario"]}"),\n' if row["row"] == row["kind"] else f'    "{row["row"]}": ("{row["kind"]}", "{row["leaf"]}", "{row["scenario"]}"),\n' for row in rows) + "}\n\n"
    text = (head + body + tail).replace("EN 1998", artifact.label)
    text = re.sub(r'DSL_ASSET = "[^"]*"', f'DSL_ASSET = "asset://{artifact.config["dsl"]}"', text)
    text = re.sub(r'ENVELOPE = "[^"]*"', f'ENVELOPE = "norm.{artifact.std}.dsl"', text)
    write(f"{artifact.case}/🐍️.py", text)


def adapter(artifact, rows):
    std, rust, label = artifact.std, artifact.rust, artifact.label
    text = open(f"{EN1998_CASE}/🦀️.rs", encoding="utf-8").read()
    head, rest = text.split("const KINDS: &[&str] = &[", 1)
    _, rest = rest.split("];", 1)
    text = head + "const KINDS: &[&str] = &[\n" + "".join(f'    "{row["kind"]}",\n' for row in rows if row["row"] == row["kind"]) + "];" + rest
    head, rest = text.split("const ROWS: &[&str] = &[", 1)
    _, rest = rest.split("];", 1)
    text = head + "const ROWS: &[&str] = &[\n" + "".join(f'    "{row["row"]}",\n' for row in rows if row["row"] != row["kind"]) + "];" + rest
    head, rest = text.split("    match kind {\n", 1)
    _, tail = rest.split("        other => panic!(", 1)
    arms = "".join(f'''        "{row['row']}" => (
            include_str!("../../🧫️fixtures/🧬️mutations/{row['leaf']}/{row['scenario']}/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/{row['leaf']}/{row['scenario']}/🦠️mutation/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/{row['leaf']}/{row['scenario']}/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../🧫️fixtures/🧬️mutations/{row['leaf']}/{row['scenario']}/🎯️outcome/🔣️.json"),
        ),
''' for row in rows)
    text = head + "    match kind {\n" + arms + "        other => panic!(" + tail
    text = re.sub(r"current \d+-kind `En1998Mutation`", f"current {len(artifact.kinds)}-kind `En1998Mutation`", text)
    text = re.sub(r'const DSL_ASSET: &str = "[^"]*";', f'const DSL_ASSET: &str = "asset://{artifact.config["dsl"]}";', text)
    if artifact.config["pack"]:
        text = re.sub(r'const PACK_ASSET: &str = "[^"]*";', f'const PACK_ASSET: &str = "asset://{artifact.config["pack"]}";', text)
    else:
        text = re.sub(r'/// 🎒️ The same document in its binary envelope[^\n]*\n#\[cfg\(feature = "sut"\)\]\nconst PACK_ASSET: &str = "[^"]*";\n', "", text)
        text = re.sub(r'        let twin = decode_en1998_pack\(&ctx\.fixture_bytes\(super::PACK_ASSET\)\?\)\?;\n        if twin != parsed \{\n.*?\n        \}\n', "", text, flags=re.S)
        text = text.replace(" pack\n//! and JSON codecs", " JSON\n//! codec").replace("the DSL,\n//! pack and JSON codecs", "the DSL, pack and JSON codecs")
    text = text.replace("En1998", rust).replace("en1998", std).replace("EN 1998", label)
    write(f"{artifact.case}/🦀️.rs", text)


if __name__ == "__main__":
    artifact = Artifact(sys.argv[1])
    command = sys.argv[2]
    if command == "plan":
        for row in plan(artifact, sys.argv[3] if len(sys.argv) > 3 else None):
            print(f"{artifact.budget(row['leaf'], row['scenario']):4d} {row['origin']:8s} {row['kind']:45s} {row['scenario']}")
    elif command == "stage":
        stage(artifact, sys.argv[3])
    elif command == "settle":
        settle(artifact, sys.argv[3])
    elif command == "surface":
        surface(artifact)
