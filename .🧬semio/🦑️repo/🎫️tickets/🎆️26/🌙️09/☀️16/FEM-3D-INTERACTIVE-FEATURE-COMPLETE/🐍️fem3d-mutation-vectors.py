#!/usr/bin/env python3
"""🧬 Writes the committed specification vectors — `(before, mutation, after, diff, outcome)` bundles
plus the per-scenario Rust fixture tests — for the four mutation kinds this ticket adds to
`s.fem.fem3d@1` (`replace-node`, `replace-load`, `change-load-case-name`, `replace-combination`).

The AFTER models, diffs and outcomes are computed HERE, in Python, from the before-model and the
mutation payload alone — an independent second application of each kind's documented contract
(target-missing → error, id-mismatch → fatal, unchanged value → no-op warning, otherwise a patched
member). The Rust implementation is then held to these files by the generated tests; nothing in this
script reads the Rust.

Usage: `python3 🐍️fem3d-mutation-vectors.py` (idempotent, rewrites the bundles and tests in place).
"""
import copy
import hashlib
import json
import os
from decimal import Decimal

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/"
MESH = ROOT + "🕸️mesh/"
LOAD = ROOT + "🏋️load/"

DIFF_MEMBERS = ["artifact", "nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis"]
MEMBERS = ["nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis"]

HALL_PROSE = """//! 🏭️ `⬅️before` is the GLULAM WORKSHOP HALL, the second real fem3d model this artifact carries
//! (ticket `26/09/06/FEM-PLUGIN-END-TO-END`): a 12 m span × 12 m long two-bay GL24h portal hall,
//! eaves at 4.2 m and ridge at 6.5 m, braced by M24 steel rods and standing on a 350 mm C25/30
//! raft slab with a machine pit. Every quantity is SI — Pa, m, m², m⁴, kg/m³, N, N/m.
//!
"""


# region 🔖️Json
def read(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def fmt_number(value):
    """🔢 Floats print as plain decimals with at least one fractional digit — the spelling every
    committed fixture already uses (`210000000000.0`, `0.00003692`), never exponent notation."""
    if isinstance(value, bool):
        return "true" if value else "false"
    if isinstance(value, int):
        return str(value)
    text = repr(float(value))
    if "e" in text or "E" in text:
        text = format(Decimal(text), "f")
    if "." not in text:
        text += ".0"
    return text


def dump(value, indent=0):
    """🖨️ Two-space pretty JSON in insertion order, empty containers inline (`[]`/`{}`)."""
    pad = "  " * indent
    inner = "  " * (indent + 1)
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = [f'{inner}"{key}": {dump(item, indent + 1)}' for key, item in value.items()]
        return "{\n" + ",\n".join(rows) + "\n" + pad + "}"
    if isinstance(value, list):
        if not value:
            return "[]"
        rows = [f"{inner}{dump(item, indent + 1)}" for item in value]
        return "[\n" + ",\n".join(rows) + "\n" + pad + "]"
    if value is None:
        return "null"
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    return fmt_number(value)


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text if text.endswith("\n") else text + "\n")


def suffix(kind, slug):
    return hashlib.sha256(f"{kind}/{slug}".encode("utf-8")).hexdigest()[:6]


def find(items, identifier):
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def null_diff():
    return {member: None for member in DIFF_MEMBERS}


def patched_diff(member, identifier, item):
    diff = null_diff()
    diff[member] = {"added": [], "removed": [], "patched": [{"id": identifier, "item": copy.deepcopy(item)}], "reordered": None}
    return diff


def rejected(code, level, path):
    return {"status": "rejected", "code": code, "path": path, "messages": [{"level": level, "code": code}]}


def no_op():
    return {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}


def applied():
    return {"status": "applied"}


# endregion 🔖️Json


# region 🔖️Reference
def load_id(load):
    return load["id"]


def load_target(load, document):
    if load["kind"] == "nodal":
        return ("nodeId", load["nodeId"], find(document["nodes"], load["nodeId"]) is not None)
    if load["kind"] == "memberUdl":
        return ("elementId", load["elementId"], find(document["elements"], load["elementId"]) is not None)
    return ("solidId", load["solidId"], find(document["solids"], load["solidId"]) is not None)


def apply(document, mutation):
    """🧬 The independent application: `(after, diff, outcome)` from `(before, mutation)`."""
    before = copy.deepcopy(document)
    after = copy.deepcopy(document)
    tag = mutation["mutation"]
    if tag == "replaceNode":
        at = find(before["nodes"], mutation["id"])
        if at is None:
            return before, null_diff(), rejected("mutation.target-missing", "error", [mutation["id"]])
        new = mutation["newNode"]
        if new["id"] != mutation["id"]:
            return before, null_diff(), rejected("mutation.id-mismatch", "fatal", [mutation["id"], new["id"]])
        if before["nodes"][at] == new:
            return before, null_diff(), no_op()
        after["nodes"][at] = copy.deepcopy(new)
        return after, patched_diff("nodes", mutation["id"], new), applied()
    if tag == "replaceLoad":
        case_at = find(before["loadCases"], mutation["caseId"])
        if case_at is None:
            return before, null_diff(), rejected("mutation.target-missing", "error", [mutation["caseId"]])
        case = before["loadCases"][case_at]
        load_at = find(case["loads"], mutation["loadId"])
        if load_at is None:
            return before, null_diff(), rejected("mutation.target-missing", "error", [mutation["loadId"]])
        new = mutation["newLoad"]
        if load_id(new) != mutation["loadId"]:
            return before, null_diff(), rejected("mutation.id-mismatch", "fatal", [mutation["loadId"], load_id(new)])
        _, target, resolves = load_target(new, before)
        if not resolves:
            return before, null_diff(), rejected("mutation.target-missing", "error", [target])
        if case["loads"][load_at] == new:
            return before, null_diff(), no_op()
        after["loadCases"][case_at]["loads"][load_at] = copy.deepcopy(new)
        return after, patched_diff("loadCases", mutation["caseId"], after["loadCases"][case_at]), applied()
    if tag == "changeLoadCaseName":
        case_at = find(before["loadCases"], mutation["caseId"])
        if case_at is None:
            return before, null_diff(), rejected("mutation.target-missing", "error", [mutation["caseId"]])
        if before["loadCases"][case_at]["name"] == mutation["newName"]:
            return before, null_diff(), no_op()
        after["loadCases"][case_at]["name"] = mutation["newName"]
        return after, patched_diff("loadCases", mutation["caseId"], after["loadCases"][case_at]), applied()
    if tag == "replaceCombination":
        at = find(before["combinations"], mutation["id"])
        if at is None:
            return before, null_diff(), rejected("mutation.target-missing", "error", [mutation["id"]])
        new = mutation["newCombination"]
        if new["id"] != mutation["id"]:
            return before, null_diff(), rejected("mutation.id-mismatch", "fatal", [mutation["id"], new["id"]])
        for case_id in new["terms"]:
            if find(before["loadCases"], case_id) is None:
                return before, null_diff(), rejected("mutation.target-missing", "error", [case_id])
        if before["combinations"][at] == new:
            return before, null_diff(), no_op()
        after["combinations"][at] = copy.deepcopy(new)
        return after, patched_diff("combinations", mutation["id"], new), applied()
    raise AssertionError(f"unknown mutation {tag}")


# endregion 🔖️Reference


# region 🔖️Rust
def rust_header(kind, folder, prose_lines, hall):
    lines = [f"//! 🧪️ `{kind}` fixture — `{folder}`.", "//!", "//! Source of truth is the committed JSON bundle beside this file (contract D1, ticket", "//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/", "//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are", "//! asserted by the shared codec-matrix harness, not here.", "//!"]
    text = "\n".join(lines) + "\n"
    if hall:
        text += HALL_PROSE
    for line in prose_lines:
        text += f"//! {line}\n"
    return text


def rust_consts(kind_dir, folder, with_diff):
    base = f'include_str!("../../../../../🧫️fixtures/🧬️mutations/{kind_dir}/{folder}/'
    text = f'const BEFORE: &str = {base}📸️snapshot/⬅️before/🔣️.json");\n'
    text += f'const AFTER: &str = {base}📸️snapshot/➡️after/🔣️.json");\n'
    text += f'const MUTATION: &str = {base}🦠️mutation/🔣️.json");\n'
    if with_diff:
        text += f'const DIFF: &str = {base}🔺️diff/🔣️.json");\n'
    text += f'const OUTCOME: &str = {base}🎯️outcome/🔣️.json");\n'
    return text


COMMON = """
fn before() -> Fem3dSnapshot {
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}
fn expected_after() -> Fem3dSnapshot {
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}
fn mutation() -> Fem3dMutation {
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}
"""

USES = """
use crate::standards::v1::subsets::any::schema::mutations::Fem3dMutation;
use crate::standards::v1::subsets::any::schema::mutations::{apply_fem3d_mutation, inverse_fem3d_mutation};
use crate::Fem3dSnapshot;

"""


def canonical_tests(tag):
    return f"""
/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{tag}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = dsl::ToValue::to_value(&mutation());
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{tag}: committed mutation JSON is not canonical");
}}
"""


def semantics_test(tag, kind, verb, entity, record):
    return f"""
/// 🪪️ The fixture is bound to this very kind's own semantic descriptor, never a sibling's.
#[test]
fn semantics_bind_the_declared_kind() {{
    let semantics = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("{verb}", "{entity}", "{kind}", "{record}"), "{tag}: the fixture must be bound to {kind}'s own descriptor");
}}
"""


def diff_tests(tag, identity):
    produced = "the committed all-null" if identity else "the committed sparse"
    replay = "Replaying the committed identity delta on `before` reproduces the committed `after`." if identity else "Replaying the committed delta on `before` reproduces the committed `after` on its own."
    return f"""
/// 🔺️ The produced delta is exactly {produced} `🔺️diff/🔣️.json`.
#[test]
fn produces_committed_diff() {{
    let outcome = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let produced = dsl::ToValue::to_value(outcome.diff());
    let committed: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    assert_eq!(produced, committed, "{tag}: produced diff differs from the committed 🔺️diff/🔣️.json");
}}

/// 🔣️ The committed diff is itself canonical and decodes to the artifact's own diff type.
#[test]
fn committed_diff_is_canonical() {{
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let reencoded = dsl::ToValue::to_value(&decoded);
    let original: dsl::DslValue = dsl::json::from_json_str(DIFF).expect("committed diff reparses");
    assert_eq!(reencoded, original, "{tag}: committed diff JSON is not canonical");
}}

/// 🩹 {replay}
#[test]
fn committed_diff_applies_to_after() {{
    let decoded: crate::standards::v1::subsets::any::schema::diff::Fem3dDiff = dsl::json::from_json_str(DIFF).expect("committed diff decodes");
    let produced = <crate::standards::v1::subsets::any::schema::diff::Fem3dDiff as protocol::MutationDiff<Fem3dSnapshot>>::apply(&decoded, &before()).expect("committed diff applies to the before-snapshot");
    assert_eq!(produced, expected_after(), "{tag}: committed diff did not carry before to after");
}}
"""


def applied_rust(spec, tag, written, extra_assert):
    kind = spec["kind"]
    others = [m for m in MEMBERS if m != written]
    field = {"nodes": "nodes", "elements": "elements", "materials": "materials", "sections": "sections", "solids": "solids", "supports": "supports", "loadCases": "load_cases", "combinations": "combinations", "analysis": "analysis"}
    untouched = "\n".join(f'    assert_eq!(snapshot.{field[m]}, base.{field[m]}, "{tag}: {field[m]} must not move when this verb runs");' for m in others)
    text = f"""
/// ▶️ The mutation carries the committed model from `before` to exactly the committed `after`.
#[test]
fn applies_to_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("{kind} applies to its committed before-snapshot");
    assert_eq!(snapshot, expected_after(), "{tag}: applied state differs from committed after-snapshot");
{extra_assert}
}}

/// 🔀️ This verb writes exactly ONE of the nine members; an implementation that re-derived a sibling
/// collection on every edit would still land on the right value for the member it meant to write.
#[test]
fn touches_only_its_own_member() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("forward applies");
{untouched}
    assert_ne!(snapshot.{field[written]}, base.{field[written]}, "{tag}: {field[written]} is the one member this verb writes");
}}

/// ↩️ Applying the computed inverse after the forward step lands back on the committed `before`.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{tag}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome holds: applied, with no diagnostic at all.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "{tag}: this vector declares an applied outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert!(produced.messages().is_empty(), "{tag}: a clean application raises no diagnostic, got {{:?}}", produced.messages());
}}
"""
    return text + diff_tests(tag, False)


def no_op_rust(spec, tag):
    kind = spec["kind"]
    return f"""
/// ▶️ A no-op still APPLIES; it simply writes nothing, so `after` is the committed `before` again.
#[test]
fn applies_to_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("{kind}'s identity diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{tag}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{tag}: a no-op must leave every one of the nine members exactly as it found them");
}}

/// ⚠️ The branch this vector pins: a Warning `mutation.no-op` with an EMPTY diff and no target
/// address — `MutationOutcome::empty().warn(..)` is the 2-arg builder, which attaches none.
#[test]
fn a_redundant_write_is_a_warning_not_a_rejection() {{
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "{tag}: a no-op must carry the identity diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{tag}: exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "mutation.no-op", "{tag}: a redundant write is reported as no-op, never as target-missing");
    assert_eq!(messages[0].level, protocol::Severity::Warning, "{tag}: a no-op is a Warning — the mutation still applies");
    assert!(messages[0].target.is_empty(), "{tag}: the 2-arg warn builder attaches no target address");
}}

/// ↩️ The inverse of a no-op is itself a no-op, so the round trip is the identity twice over.
#[test]
fn inverse_restores_before() {{
    let base = before();
    let mutation = mutation();
    let inverse = inverse_fem3d_mutation(&base, &mutation);
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation).expect("forward applies");
    for step in &inverse {{
        apply_fem3d_mutation(&mut snapshot, step).expect("inverse step applies");
    }}
    assert_eq!(snapshot, base, "{tag}: inverse did not restore the before-snapshot");
}}

/// 🎯️ The declared outcome holds, code and level together.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("applied"), "{tag}: a no-op declares an applied outcome");
    let declared = outcome.get("messages").and_then(dsl::DslValue::as_array).expect("the declared outcome carries messages");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(declared.len(), produced.messages().len(), "{tag}: the declared diagnostic count must match the emitted one");
    assert_eq!(declared[0].get("code").and_then(dsl::DslValue::as_str), Some(produced.messages()[0].code.0.as_str()), "{tag}: the declared code must match the emitted one");
    assert_eq!(declared[0].get("level").and_then(dsl::DslValue::as_str), Some("warn"), "{tag}: the declared level must name the Warning the builder raises");
}}
""" + diff_tests(tag, True)


def rejected_rust(spec, tag, outcome, level_prose):
    kind = spec["kind"]
    variant = spec["variant"]
    code = outcome["code"]
    severity = {"error": "Error", "fatal": "Fatal"}[outcome["messages"][0]["level"]]
    path = ", ".join(f'"{segment}".to_string()' for segment in outcome["path"])
    quoted = ", ".join(f'\\"{segment}\\"' for segment in outcome["path"])
    return f"""
/// ▶️ A refused mutation leaves the document byte-identical to the committed `after`, which is the
/// committed `before` again. `vcs::apply_mutation` is deliberately policy-agnostic — it applies the
/// (empty) diff and returns `Ok`, so REJECTION IS NOT VISIBLE IN THE RESULT, only in the messages.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{tag}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{tag}: a refused mutation must leave every one of the nine members untouched");
}}

/// 🚨️ The branch this vector pins, level and address together.
#[test]
fn the_refusal_is_the_declared_diagnostic() {{
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::standards::v1::subsets::any::schema::diff::Fem3dDiff::default(), "{tag}: a refused mutation must carry the empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{tag}: exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "{code}", "{tag}: the refusal is reported as {code}");
    assert_eq!(messages[0].level, protocol::Severity::{severity}, "{tag}: {level_prose}");
    assert_eq!(messages[0].target, vec![{path}], "{tag}: the diagnostic addresses exactly {quoted}");
}}

/// ↩️ The inverse is computed from `before` and the mutation payload alone, never from the verdict,
/// so a refused request still has the undo step its kind emits whenever the target exists.
#[test]
fn inverse_has_the_declared_shape() {{
    let inverse = inverse_fem3d_mutation(&before(), &mutation());
    for step in &inverse {{
        assert!(matches!(step, Fem3dMutation::{variant}(_)), "{tag}: the inverse of {kind} is a {variant}, got {{step:?}}");
    }}
    assert!(inverse.len() <= 1, "{tag}: this kind never emits more than one undo step, got {{inverse:?}}");
}}

/// 🎯️ The declared outcome — status, code, level and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "{tag}: this vector declares a rejected outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "{tag}: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "{tag}: the declared path must match the emitted target");
}}
"""


# endregion 🔖️Rust


# region 🔖️Scenarios
def emit(spec, scenario):
    """🏭 Writes one scenario: the fixture bundle under `🧫️fixtures/🧬️mutations/<kind>/<folder>/` and
    its Rust test under `🧬️schema/🧬️mutations/<kind>/🧪️tests/<folder>/🦀️.rs`."""
    kind, kind_dir, subset = spec["kind"], spec["dir"], spec["subset"]
    folder = f'{scenario["emoji"]}{scenario["slug"]}-{suffix(kind, scenario["slug"])}'
    before = read(scenario["before"])
    after, diff, outcome = apply(before, scenario["mutation"])
    assert outcome["status"] == scenario["expect"] or (scenario["expect"] == "no-op" and outcome.get("messages")), (folder, outcome)
    bundle = f"{subset}🧫️fixtures/🧬️mutations/{kind_dir}/{folder}/"
    with open(scenario["before"], encoding="utf-8") as handle:
        before_text = handle.read()
    write(bundle + "📸️snapshot/⬅️before/🔣️.json", before_text)
    write(bundle + "📸️snapshot/➡️after/🔣️.json", before_text if after == before else dump(after))
    write(bundle + "🦠️mutation/🔣️.json", dump(scenario["mutation"]))
    write(bundle + "🎯️outcome/🔣️.json", dump(outcome))
    is_reject = outcome["status"] == "rejected"
    if is_reject:
        write(bundle + "🔺️diff/🚫️.absent", "")
    else:
        write(bundle + "🔺️diff/🔣️.json", dump(diff))
    tag = f"{kind}/{scenario['slug']}-{suffix(kind, scenario['slug'])}"
    text = rust_header(kind, folder, scenario["prose"], scenario["hall"]) + USES + rust_consts(kind_dir, folder, not is_reject) + COMMON
    if is_reject:
        text += rejected_rust(spec, tag, outcome, scenario["level_prose"])
    elif outcome.get("messages"):
        text += no_op_rust(spec, tag)
    else:
        text += applied_rust(spec, tag, spec["written"], scenario["extra_assert"].replace("{tag}", tag))
    text += canonical_tests(tag) + semantics_test(tag, kind, spec["verb"], spec["entity"], spec["record"])
    write(f"{subset}🧬️schema/🧬️mutations/{kind_dir}/🧪️tests/{folder}/🦀️.rs", text)
    return folder, outcome["status"], bool(outcome.get("messages"))


MESH_SPEC_BEFORE = MESH + "🧫️fixtures/🧬️mutations/♻️replace-element/🔄️rolls-the-column-50f732/📸️snapshot/⬅️before/🔣️.json"
MESH_HALL_BEFORE = MESH + "🧫️fixtures/🧬️mutations/♻️replace-element/🏗️hall-strut-d0e4b7/📸️snapshot/⬅️before/🔣️.json"
LOAD_SPEC_LOADS_BEFORE = LOAD + "🧫️fixtures/🧬️mutations/➖️remove-load/➖️drops-the-trailing-member-b73b25/📸️snapshot/⬅️before/🔣️.json"
LOAD_SPEC_CASES_BEFORE = LOAD + "🧫️fixtures/🧬️mutations/🔗️create-combination/🔗️appends-a-8ede20/📸️snapshot/⬅️before/🔣️.json"
LOAD_SPEC_COMBOS_BEFORE = LOAD + "🧫️fixtures/🧬️mutations/✂️delete-combination/✂️removes-the-182f7b/📸️snapshot/⬅️before/🔣️.json"
LOAD_HALL_BEFORE = LOAD + "🧫️fixtures/🧬️mutations/⚖️change-load-case-self-weight/🏗️hall-crane-sw-978370/📸️snapshot/⬅️before/🔣️.json"

LEVEL_MISSING = "a missed target is an Error, not the Fatal a duplicate identity raises"
LEVEL_RENAME = "renaming a record is an identity breach, the same Fatal level a duplicate identity raises"


def node(identifier, x, y, z):
    return {"id": identifier, "x": x, "y": y, "z": z}


def scenarios():
    hall_mesh = read(MESH_HALL_BEFORE)
    hall_load = read(LOAD_HALL_BEFORE)
    ridge = hall_mesh["nodes"][find(hall_mesh["nodes"], "ap_1")]
    wx0 = hall_load["loadCases"][find(hall_load["loadCases"], "wind_x")]["loads"][0]
    sls = hall_load["combinations"][find(hall_load["combinations"], "sls_char")]
    crane = hall_load["loadCases"][find(hall_load["loadCases"], "crane_spare")]
    combos = read(LOAD_SPEC_COMBOS_BEFORE)["combinations"]
    first_combo = combos[0]
    retuned_wx0 = copy.deepcopy(wx0)
    for key in ("wx", "wy", "wz", "value", "pressure"):
        if key in retuned_wx0:
            retuned_wx0[key] = retuned_wx0[key] * 1.25
    replace_node = {
        "kind": "replace-node", "dir": "🔁️replace-node", "subset": MESH, "variant": "ReplaceNode", "verb": "replace", "entity": "node", "record": "ReplacedNode", "written": "nodes",
        "scenarios": [
            {"emoji": "📍️", "slug": "lifts-the-column-head", "before": MESH_SPEC_BEFORE, "hall": False, "expect": "applied",
             "mutation": {"mutation": "replaceNode", "id": "n3", "newNode": node("n3", 0.0, 0.0, 4.0)},
             "prose": ["Only `z` changes: the column head `n3` rises from 3.5 m to 4.0 m while the frame `f1` that names it keeps naming it — a", "replace never renames, so the topology stays exactly where it was and the geometry travels."],
             "extra_assert": '    assert_eq!(snapshot.nodes.len(), 3, "{tag}: a replacement must not change the node count");\n    assert_eq!(snapshot.nodes[2].z, 4.0, "{tag}: the lifted column head must survive the round trip exactly");'},
            {"emoji": "🏗️", "slug": "hall-lifts-ridge", "before": MESH_HALL_BEFORE, "hall": True, "expect": "applied",
             "mutation": {"mutation": "replaceNode", "id": "ap_1", "newNode": node("ap_1", ridge["x"], ridge["y"], ridge["z"] + 0.3)},
             "prose": ["The middle apex `ap_1` rises 300 mm — the one gesture the transform gumball spells when it drags a node — while every", "rafter, tie and load that names `ap_1` keeps naming it."],
             "extra_assert": '    let apex = snapshot.nodes.iter().find(|node| node.id == "ap_1").expect("the apex survives");\n    assert_eq!(apex.z, ' + fmt_number(ridge["z"] + 0.3) + ', "{tag}: the ridge rises by exactly 0.3 m");'},
            {"emoji": "🚨️", "slug": "no-such-node", "before": MESH_SPEC_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceNode", "id": "n9", "newNode": node("n9", 1.0, 1.0, 1.0)},
             "prose": ["`n9` is not in the model, so the first guard refuses before the payload is even read."], "level_prose": LEVEL_MISSING},
            {"emoji": "🪪️", "slug": "renames-node", "before": MESH_SPEC_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceNode", "id": "n2", "newNode": node("n2b", 6.0, 0.0, 0.0)},
             "prose": ["`n2` is a leaf of the reference graph, so renaming it would orphan nothing — and the identity contract is still the", "identity contract: a replace never renames."], "level_prose": LEVEL_RENAME},
            {"emoji": "⏸️", "slug": "same-node", "before": MESH_SPEC_BEFORE, "hall": False, "expect": "no-op",
             "mutation": {"mutation": "replaceNode", "id": "n1", "newNode": node("n1", 0.0, 0.0, 0.0)},
             "prose": ["Replacing the base node with the position it already holds is a no-op WARNING, never a rejection: the verb still", "applies, it simply writes nothing."]},
        ],
    }
    spec_loads = read(LOAD_SPEC_LOADS_BEFORE)
    g2 = spec_loads["loadCases"][0]["loads"][1]
    retuned_g2 = dict(g2)
    retuned_g2["wz"] = -12.0
    replace_load = {
        "kind": "replace-load", "dir": "🔁️replace-load", "subset": LOAD, "variant": "ReplaceLoad", "verb": "replace", "entity": "load", "record": "ReplacedLoad", "written": "loadCases",
        "scenarios": [
            {"emoji": "🔁️", "slug": "retunes-the-rafter-udl", "before": LOAD_SPEC_LOADS_BEFORE, "hall": False, "expect": "applied",
             "mutation": {"mutation": "replaceLoad", "caseId": "dead", "loadId": "g2", "newLoad": retuned_g2},
             "prose": ["Only `wz` changes: the member UDL `g2` on the column `f1` goes from -8 N/m to -12 N/m in place — the nodal load `g1`", "beside it keeps its slot and its value."],
             "extra_assert": '    assert!(matches!(&snapshot.load_cases[0].loads[1], crate::FemLoad::MemberUdl { wz, .. } if *wz == -12.0), "{tag}: the retuned UDL must survive the round trip exactly");\n    assert_eq!(snapshot.load_cases[0].loads.len(), 2, "{tag}: a replacement must not change the load count");'},
            {"emoji": "🏗️", "slug": "hall-retunes-wx", "before": LOAD_HALL_BEFORE, "hall": True, "expect": "applied",
             "mutation": {"mutation": "replaceLoad", "caseId": "wind_x", "loadId": wx0["id"], "newLoad": retuned_wx0},
             "prose": ["The first wind load of `wind_x` is scaled by 1.25 in place; the case's other loads and every other case stay untouched."],
             "extra_assert": '    let wind = snapshot.load_cases.iter().find(|case| case.id == "wind_x").expect("the wind case survives");\n    assert_eq!(wind.loads.len(), 3, "{tag}: a replacement must not change the load count");'},
            {"emoji": "🚨️", "slug": "no-such-load", "before": LOAD_SPEC_LOADS_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceLoad", "caseId": "dead", "loadId": "g7", "newLoad": {"kind": "nodal", "id": "g7", "nodeId": "n3", "dof": "Tz", "value": -1.0}},
             "prose": ["The case exists but carries no load `g7`, so the second guard refuses on the load id."], "level_prose": LEVEL_MISSING},
            {"emoji": "🚨️", "slug": "no-such-case", "before": LOAD_SPEC_LOADS_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceLoad", "caseId": "live", "loadId": "g2", "newLoad": retuned_g2},
             "prose": ["`live` is not a load case of this model, so the first guard refuses on the case id before any load is looked up."], "level_prose": LEVEL_MISSING},
            {"emoji": "🪪️", "slug": "renames-load", "before": LOAD_SPEC_LOADS_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceLoad", "caseId": "dead", "loadId": "g2", "newLoad": dict(retuned_g2, id="g2b")},
             "prose": ["A replace never renames: the replacement's own id must be the addressed one."], "level_prose": LEVEL_RENAME},
            {"emoji": "🚨️", "slug": "dangling-node", "before": LOAD_SPEC_LOADS_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceLoad", "caseId": "dead", "loadId": "g1", "newLoad": {"kind": "nodal", "id": "g1", "nodeId": "n7", "dof": "Tz", "value": -15.0}},
             "prose": ["The replacement re-aims the nodal load at `n7`, which the model does not carry — the SAME per-variant resolution", "`add-load` runs, so a load can never point at a node that is not there."], "level_prose": LEVEL_MISSING},
            {"emoji": "⏸️", "slug": "same-load", "before": LOAD_SPEC_LOADS_BEFORE, "hall": False, "expect": "no-op",
             "mutation": {"mutation": "replaceLoad", "caseId": "dead", "loadId": "g2", "newLoad": g2},
             "prose": ["Replacing the UDL with the value it already holds is a no-op WARNING, never a rejection: the verb still applies, it", "simply writes nothing."]},
        ],
    }
    change_name = {
        "kind": "change-load-case-name", "dir": "🏷️change-load-case-name", "subset": LOAD, "variant": "ChangeLoadCaseName", "verb": "change", "entity": "load-case", "record": "ChangedLoadCaseName", "written": "loadCases",
        "scenarios": [
            {"emoji": "🏷️", "slug": "renames-the-wind-case", "before": LOAD_SPEC_CASES_BEFORE, "hall": False, "expect": "applied",
             "mutation": {"mutation": "changeLoadCaseName", "caseId": "wind", "newName": "Wind Y"},
             "prose": ["Only `name` changes: the id `wind` every combination term resolves through is untouched, so a rename orphans nothing."],
             "extra_assert": '    assert_eq!(snapshot.load_cases[1].name, "Wind Y", "{tag}: the new name must survive the round trip exactly");\n    assert_eq!(snapshot.load_cases[1].id, "wind", "{tag}: the id every combination resolves through is untouched");'},
            {"emoji": "🏗️", "slug": "hall-renames-crane", "before": LOAD_HALL_BEFORE, "hall": True, "expect": "applied",
             "mutation": {"mutation": "changeLoadCaseName", "caseId": "crane_spare", "newName": "Crane Runway (reserved)"},
             "prose": ["The spare crane case gets its human-readable label; its loads and its self-weight flag stay exactly as they were."],
             "extra_assert": '    let crane = snapshot.load_cases.iter().find(|case| case.id == "crane_spare").expect("the crane case survives");\n    assert_eq!(crane.name, "Crane Runway (reserved)", "{tag}: the new name must survive the round trip exactly");\n    assert_eq!(crane.self_weight, ' + ("true" if crane["selfWeight"] else "false") + ', "{tag}: a rename never touches the self-weight flag");'},
            {"emoji": "🚨️", "slug": "no-such-case", "before": LOAD_SPEC_CASES_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "changeLoadCaseName", "caseId": "snow", "newName": "Snow"},
             "prose": ["`snow` is not a load case of this model, so the one guard refuses on the case id."], "level_prose": LEVEL_MISSING},
            {"emoji": "⏸️", "slug": "same-name", "before": LOAD_SPEC_CASES_BEFORE, "hall": False, "expect": "no-op",
             "mutation": {"mutation": "changeLoadCaseName", "caseId": "dead", "newName": "Dead"},
             "prose": ["Renaming a case to the name it already carries is a no-op WARNING, never a rejection: the verb still applies, it", "simply writes nothing."]},
        ],
    }
    reweighted = copy.deepcopy(first_combo)
    reweighted["terms"] = {case: factor * 1.1 for case, factor in first_combo["terms"].items()}
    retuned_sls = copy.deepcopy(sls)
    retuned_sls["name"] = "SLS Frequent"
    retuned_sls["terms"] = dict(sorted({**sls["terms"], "snow": 0.5}.items()))
    dangling = copy.deepcopy(first_combo)
    dangling["terms"] = dict(sorted({**first_combo["terms"], "seismic": 1.0}.items()))
    replace_combination = {
        "kind": "replace-combination", "dir": "🔁️replace-combination", "subset": LOAD, "variant": "ReplaceCombination", "verb": "replace", "entity": "combination", "record": "ReplacedCombination", "written": "combinations",
        "scenarios": [
            {"emoji": "🔁️", "slug": "reweights-the-terms", "before": LOAD_SPEC_COMBOS_BEFORE, "hall": False, "expect": "applied",
             "mutation": {"mutation": "replaceCombination", "id": first_combo["id"], "newCombination": reweighted},
             "prose": ["Every factor of the first combination is scaled by 1.1 in place; its id and the cases it weights are the same cases."],
             "extra_assert": '    assert_eq!(snapshot.combinations.len(), ' + str(len(combos)) + ', "{tag}: a replacement must not change the combination count");\n    assert_eq!(snapshot.combinations[0].terms.len(), ' + str(len(first_combo["terms"])) + ', "{tag}: a re-weighting keeps every term");'},
            {"emoji": "🏗️", "slug": "hall-retunes-sls", "before": LOAD_HALL_BEFORE, "hall": True, "expect": "applied",
             "mutation": {"mutation": "replaceCombination", "id": "sls_char", "newCombination": retuned_sls},
             "prose": ["The characteristic SLS combination becomes the frequent one: a new name and a 0.5 snow factor, the dead term untouched."],
             "extra_assert": '    let frequent = snapshot.combinations.iter().find(|combination| combination.id == "sls_char").expect("the combination survives");\n    assert_eq!(frequent.name, "SLS Frequent", "{tag}: the new name must survive the round trip exactly");\n    assert_eq!(frequent.terms.get("snow"), Some(&0.5), "{tag}: the retuned snow factor must survive the round trip exactly");'},
            {"emoji": "🚨️", "slug": "no-such-combo", "before": LOAD_SPEC_COMBOS_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceCombination", "id": "ghost", "newCombination": dict(reweighted, id="ghost")},
             "prose": ["`ghost` is not a combination of this model, so the first guard refuses before the terms are resolved."], "level_prose": LEVEL_MISSING},
            {"emoji": "🪪️", "slug": "renames-combo", "before": LOAD_SPEC_COMBOS_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceCombination", "id": first_combo["id"], "newCombination": dict(reweighted, id=first_combo["id"] + "_b")},
             "prose": ["A replace never renames: the replacement's own id must be the addressed one."], "level_prose": LEVEL_RENAME},
            {"emoji": "🚨️", "slug": "dangling-term", "before": LOAD_SPEC_COMBOS_BEFORE, "hall": False, "expect": "rejected",
             "mutation": {"mutation": "replaceCombination", "id": first_combo["id"], "newCombination": dangling},
             "prose": ["The replacement weights `seismic`, a case the model does not carry — the SAME per-term resolution `create-combination`", "runs, so the twins accept exactly the same terms."], "level_prose": LEVEL_MISSING},
            {"emoji": "⏸️", "slug": "same-combination", "before": LOAD_SPEC_COMBOS_BEFORE, "hall": False, "expect": "no-op",
             "mutation": {"mutation": "replaceCombination", "id": first_combo["id"], "newCombination": first_combo},
             "prose": ["Replacing the combination with the value it already holds is a no-op WARNING, never a rejection: the verb still", "applies, it simply writes nothing."]},
        ],
    }
    return [replace_node, replace_load, change_name, replace_combination]


# endregion 🔖️Scenarios


def main():
    summary = {}
    for spec in scenarios():
        rows = []
        for scenario in spec["scenarios"]:
            folder, status, messages = emit(spec, scenario)
            rows.append((folder, scenario["slug"], status, messages, scenario["hall"]))
        summary[spec["kind"]] = rows
    print(json.dumps(summary, ensure_ascii=False, indent=1))


if __name__ == "__main__":
    main()
