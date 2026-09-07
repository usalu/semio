#!/usr/bin/env python3
"""🏗️ W12 — author the refusal vectors the hardened fem3d rule set creates.

W11 gave every one of the 25 kinds three vectors: the pre-existing one, a glulam-workshop-hall happy
path and one edge branch. W12's hardening adds four new refusal families — `mutation.id-mismatch`,
`mutation.target-referenced`, `mutation.invariant` (geometry) and `mutation.invariant`
(plausibility/bounds) — and every one of them needs a vector of its own. This script writes them,
mounts them in the crate, registers them in the five subset oracle catalogs and in the repo
taxonomy's `members-of-tests`.

Every `⬅️before` is the SAME glulam workshop hall W11 introduced, read back off disk from one of its
own committed vectors rather than re-derived here, so there is exactly one copy of that model's
truth in the repository.

Subcommands: `cases` (write the bundles), `patch` (mounts + catalogs + taxonomy), `manifest`.
Idempotent — every patch is anchored and skipped when already applied.

Usage: uv run python 🔨️w12-author-fem3d-cases.py cases patch
"""

import copy
import hashlib
import importlib.util
import json
import os
import sys
import textwrap

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets")
CRATE = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "📦️packages", "🦀️rust", "🦀️.rs")
TAXONOMY = os.path.join(REPO, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json")
HALL = os.path.join(SUBSETS, "🕸️mesh", "🧬️schema", "🧬️mutations", "🧩️create-element", "🧪️tests", "🏗️hall-new-tie-074a69", "📸️snapshot", "⬅️before", "🔣️.json")

_spec = importlib.util.spec_from_file_location("w12rules", os.path.join(HERE, "🔨️w12-fem3d-rules.py"))
rules = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(rules)


# region 🔖️Vocabulary
SEMANTICS = {
    "delete-element": ("delete", "element", "DeletedElement"),
    "replace-element": ("replace", "element", "ReplacedElement"),
    "create-section": ("create", "section", "CreatedSection"),
    "delete-section": ("delete", "section", "DeletedSection"),
    "replace-section": ("replace", "section", "ReplacedSection"),
    "create-solid": ("create", "solid", "CreatedSolid"),
    "delete-solid": ("delete", "solid", "DeletedSolid"),
    "replace-solid": ("replace", "solid", "ReplacedSolid"),
    "create-material": ("create", "material", "CreatedMaterial"),
    "delete-material": ("delete", "material", "DeletedMaterial"),
    "replace-material": ("replace", "material", "ReplacedMaterial"),
    "replace-support": ("replace", "support", "ReplacedSupport"),
    "delete-load-case": ("delete", "load-case", "DeletedLoadCase"),
    "add-load": ("add", "load", "AddedLoad"),
    "update-analysis-settings": ("update", "analysis-settings", "UpdatedAnalysisSettings"),
}

INVERSE_VARIANT = {
    "delete-element": "CreateElement",
    "replace-element": "ReplaceElement",
    "create-section": "DeleteSection",
    "delete-section": "CreateSection",
    "replace-section": "ReplaceSection",
    "create-solid": "DeleteSolid",
    "delete-solid": "CreateSolid",
    "replace-solid": "ReplaceSolid",
    "create-material": "DeleteMaterial",
    "delete-material": "CreateMaterial",
    "replace-material": "ReplaceMaterial",
    "replace-support": "ReplaceSupport",
    "delete-load-case": "CreateLoadCase",
    "add-load": "RemoveLoad",
    "update-analysis-settings": "UpdateAnalysisSettings",
}

KIND_DIRECTORY = {
    "delete-element": ("🕸️mesh", "🗑️delete-element"),
    "replace-element": ("🕸️mesh", "♻️replace-element"),
    "create-section": ("🕸️mesh", "📐️create-section"),
    "delete-section": ("🕸️mesh", "✂️delete-section"),
    "replace-section": ("🕸️mesh", "📏️replace-section"),
    "create-solid": ("🕸️mesh", "🧊️create-solid"),
    "delete-solid": ("🕸️mesh", "🚫️delete-solid"),
    "replace-solid": ("🕸️mesh", "🔄️replace-solid"),
    "create-material": ("🧱️material", "🌱️create-material"),
    "delete-material": ("🧱️material", "🗑️delete-material"),
    "replace-material": ("🧱️material", "🔁️replace-material"),
    "replace-support": ("🛡️boundary", "🔁️replace-support"),
    "delete-load-case": ("🏋️load", "🗑️delete-load-case"),
    "add-load": ("🏋️load", "➕️add-load"),
    "update-analysis-settings": ("📈️analysis", "🎛️update-analysis-settings"),
}
# endregion 🔖️Vocabulary


# region 🔖️Model
def hall():
    with open(HALL, encoding="utf-8") as handle:
        return json.load(handle)


def record(document, collection, identifier):
    at = rules.find(document[collection], identifier)
    assert at is not None, (collection, identifier)
    return copy.deepcopy(document[collection][at])


def with_field(base, **fields):
    out = copy.deepcopy(base)
    out.update(fields)
    return out


# endregion 🔖️Model


# region 🔖️Cases
def cases():
    """📚️ The twenty-one refusal vectors, each on the shared glulam workshop hall."""
    model = hall()
    built = []

    def case(kind, emoji, slug, mutation, note, before=None):
        built.append({"kind": kind, "emoji": emoji, "slug": slug, "mutation": mutation, "note": note, "before": before or copy.deepcopy(model)})

    case(
        "delete-element",
        "⛓️",
        "rafter-under-udl",
        {"mutation": "deleteElement", "id": "raf_l_1"},
        "The middle frame's left rafter carries the roof dead UDL and the snow UDL. Striking it would leave both member loads pointing at nothing, so the verb refuses and names them; releasing the loads first is the caller's move.",
    )
    case(
        "replace-element",
        "🪪️",
        "renames-brace",
        {"mutation": "replaceElement", "id": "brc_0", "newElement": with_field(record(model, "elements", "brc_0"), id="brc_9")},
        "A `replace-` selects its target by `id` and carries a whole new record; a new record under a different id would silently rename the brace and orphan anything naming `brc_0`. The identity is not the replace's to change.",
    )
    case(
        "replace-element",
        "🚨️",
        "dangling-sec",
        {"mutation": "replaceElement", "id": "brc_0", "newElement": with_field(record(model, "elements", "brc_0"), sectionId="sec_x")},
        "`replace-element` now resolves the same four foreign keys `create-element` does, in the same order — start, end, material, section. Start, end and material all resolve here; the section is the first to miss.",
    )
    case(
        "create-section",
        "🧨️",
        "zero-area",
        {"mutation": "createSection", "section": {"id": "sec_void", "name": "Zero Area Profile", "area": 0.0, "iy": 0.0001, "iz": 0.0001, "j": 0.0001}},
        "A cross-section of zero area carries no axial stiffness at all — `EA/L` is zero and the element's stiffness matrix is singular. The document refuses the value rather than letting the solver discover it.",
    )
    case(
        "delete-section",
        "⛓️",
        "purlin-in-use",
        {"mutation": "deleteSection", "id": "sec_pur"},
        "The C24 purlin profile is shared by all six eaves and ridge purlins. Deleting it would leave six elements without section properties, so the refusal names every one of them.",
    )
    case(
        "replace-section",
        "🪪️",
        "renames-purlin",
        {"mutation": "replaceSection", "id": "sec_pur", "newSection": with_field(record(model, "sections", "sec_pur"), id="sec_pur_v2")},
        "Re-grading a profile is a `replace-`; re-naming its identity is not. Six elements point at `sec_pur` and the vocabulary carries no verb that re-points them, so the rename is refused.",
    )
    case(
        "replace-section",
        "🧨️",
        "negative-iy",
        {"mutation": "replaceSection", "id": "sec_tie", "newSection": with_field(record(model, "sections", "sec_tie"), iy=-1.03e-08)},
        "A negative second moment of area is not a slender section, it is an unphysical one — the bending stiffness would come out negative and the tie rod would push the roof up.",
    )
    case(
        "create-solid",
        "📐️",
        "sliver-outline",
        {"mutation": "createSolid", "solid": {"id": "slab_sliver", "name": "Collinear Yard Strip", "outline": [[16.0, 0.0], [20.0, 0.0], [24.0, 0.0]], "holes": [], "baseZ": -0.2, "height": 0.2, "layers": 1, "meshSize": 1.0, "materialId": "c25_30"}},
        "Three points on one line close a ring of zero area. `resolve_geometry` could not triangulate it and the extrusion would have no volume, so the footprint is refused at the document boundary instead of at solve time.",
    )
    case(
        "delete-solid",
        "⛓️",
        "raft-under-load",
        {"mutation": "deleteSolid", "id": "slab_raft"},
        "The raft slab carries the 2.5 kPa floor pressure of the dead case. Deleting it would leave that area load addressing nothing.",
    )
    case(
        "replace-solid",
        "🪪️",
        "renames-apron",
        {"mutation": "replaceSolid", "id": "slab_apron", "newSolid": with_field(record(model, "solids", "slab_apron"), id="slab_apron_v2")},
        "The door apron is unreferenced, so this rename would break nothing today — and is refused all the same. The rule is about what the verb MEANS, not about whether this particular row happens to be safe.",
    )
    case(
        "replace-solid",
        "📐️",
        "zero-height",
        {"mutation": "replaceSolid", "id": "slab_apron", "newSolid": with_field(record(model, "solids", "slab_apron"), height=0.0)},
        "A solid extruded by nothing is a surface, and this artifact has no surface element. Zero and negative heights are refused together.",
    )
    case(
        "replace-solid",
        "🚨️",
        "dangling-mat",
        {"mutation": "replaceSolid", "id": "slab_apron", "newSolid": with_field(record(model, "solids", "slab_apron"), materialId="c40_50")},
        "`replace-solid` now resolves `materialId` exactly as `create-solid` does. Upgrading the apron to a C40/50 that was never added to the model is refused, not silently accepted.",
    )
    case(
        "create-material",
        "🧨️",
        "nu-at-a-half",
        {"mutation": "createMaterial", "material": {"id": "rubber", "name": "Incompressible Elastomer", "e": 5000000.0, "g": 1670000.0, "nu": 0.5, "rho": 1100.0}},
        "A Poisson ratio of exactly 0.5 is incompressible: the `Tet4` constitutive matrix divides by `1 - 2*nu` and blows up. The admissible interval is open at both ends, (-1, 0.5), and 0.5 is outside it.",
    )
    case(
        "delete-material",
        "⛓️",
        "glulam-in-use",
        {"mutation": "deleteMaterial", "id": "gl24h"},
        "GL24h is the grade of every column and rafter in the hall. The refusal lists all twelve so a caller can see the size of what it asked for.",
    )
    case(
        "replace-material",
        "🪪️",
        "renames-c24",
        {"mutation": "replaceMaterial", "id": "c24", "newMaterial": with_field(record(model, "materials", "c24"), id="c24_v2")},
        "This is finding 3 of W11's report turned into a refusal: `replace-material { id: \"c24\", newMaterial: { id: \"c24_v2\" } }` used to rename the grade and orphan every purlin pointing at `c24`.",
    )
    case(
        "replace-material",
        "🧨️",
        "negative-e",
        {"mutation": "replaceMaterial", "id": "gl32c", "newMaterial": with_field(record(model, "materials", "gl32c"), e=-13500000000.0)},
        "A negative Young's modulus inverts the whole load path — the stiffer the member, the more it would deflect. The spare GL32c grade is unreferenced and the value is refused anyway.",
    )
    case(
        "replace-support",
        "🪪️",
        "renames-pin",
        {"mutation": "replaceSupport", "id": "sup_ext_a", "newSupport": with_field(record(model, "supports", "sup_ext_a"), id="sup_ext_z")},
        "The reserved set-out pin is a leaf of the reference graph, so renaming it orphans nothing — and the identity contract is still the identity contract.",
    )
    case(
        "replace-support",
        "🚨️",
        "dangling-node",
        {"mutation": "replaceSupport", "id": "sup_ext_a", "newSupport": with_field(record(model, "supports", "sup_ext_a"), nodeId="n_ext_z")},
        "`replace-support` now resolves `nodeId` exactly as `create-support` does. Moving a support onto a node the model does not carry is refused.",
    )
    case(
        "delete-load-case",
        "⛓️",
        "dead-in-combos",
        {"mutation": "deleteLoadCase", "id": "dead"},
        "All three combinations weight the dead case. Deleting it would leave three combinations superposing a result that no longer exists, so the refusal names them in document order.",
    )
    case(
        "add-load",
        "🚨️",
        "no-such-member",
        {"mutation": "addLoad", "caseId": "snow", "load": {"kind": "memberUdl", "id": "ld_snow_x", "elementId": "raf_l_9", "wx": 0.0, "wy": 0.0, "wz": -4320.0}},
        "This is finding 1 of W11's report turned into a refusal. `create-load-case` always resolved a carried load's target; `add-load` accepted the same payload against a member that does not exist. Both verbs now resolve it the same way.",
    )
    case(
        "update-analysis-settings",
        "🧨️",
        "zero-modes",
        {"mutation": "updateAnalysisSettings", "settings": {"modalCount": 0, "bucklingCount": 4, "deformationScale": 150.0}},
        "An eigen solve for zero modes has nothing to return. The bound is one, not zero, and the refusal carries no target because analysis settings are one inseparable document facet with no id of their own.",
    )
    return built


# endregion 🔖️Cases


# region 🔖️Emission
HALL_NOTE = """//! 🏭️ `⬅️before` is the GLULAM WORKSHOP HALL, the second real fem3d model this artifact carries
//! (ticket `26/09/06/FEM-PLUGIN-END-TO-END`): a 12 m span × 12 m long two-bay GL24h portal hall,
//! eaves at 4.2 m and ridge at 6.5 m, braced by M24 steel rods and standing on a 350 mm C25/30
//! raft slab with a machine pit. Every quantity is SI — Pa, m, m², m⁴, kg/m³, N, N/m."""

LEVEL_VARIANT = {"fatal": "Fatal", "error": "Error"}
LEVEL_PROSE = {
    "mutation.id-mismatch": "renaming a record is an identity breach, the same Fatal level a duplicate identity raises",
    "mutation.target-referenced": "a live referrer is an Error, the same level a missed target raises — the request is answerable, just not now",
    "mutation.invariant": "an unphysical value is Fatal: no later mutation could rescue a document that took it",
    "mutation.target-missing": "a missed target is an Error, not the Fatal a duplicate identity raises",
}


def directory_name(entry):
    digest = hashlib.sha1(("%s/%s" % (entry["kind"], entry["slug"])).encode("utf-8")).hexdigest()[:6]
    return "%s%s-%s" % (entry["emoji"], entry["slug"], digest)


def scenario_id(entry):
    return directory_name(entry)[len(entry["emoji"]) :]


def module_name(entry):
    return "tests_edge_" + entry["slug"].replace("-", "_")


def rust(entry, outcome):
    kind = entry["kind"]
    verb, noun, record_name = SEMANTICS[kind]
    label = "%s/%s" % (kind, scenario_id(entry))
    level = outcome["messages"][0]["level"]
    path = ", ".join("\\\"%s\\\"" % segment for segment in outcome["path"]) or "no target at all"
    targets = ", ".join('"%s".to_string()' % segment for segment in outcome["path"])
    expected_target = "vec![%s]" % targets if outcome["path"] else "Vec::<String>::new()"
    note = "\n".join("//! " + line for line in textwrap.wrap(entry["note"], 108))
    return f'''//! 🧪️ `{kind}` fixture — `{directory_name(entry)}`.
//!
//! Source of truth is the committed JSON bundle beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
{HALL_NOTE}
//!
{note}

use crate::artifacts::fem3d::mutations::Fem3dMutation;
use crate::artifacts::fem3d::mutations::{{apply_fem3d_mutation, inverse_fem3d_mutation}};
use crate::artifacts::fem3d::Fem3dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem3dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Fem3dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Fem3dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}

/// ▶️ A refused mutation leaves the document byte-identical to the committed `after`, which is the
/// committed `before` again. `vcs::apply_mutation` is deliberately policy-agnostic — it applies the
/// (empty) diff and returns `Ok`, so REJECTION IS NOT VISIBLE IN THE RESULT, only in the messages.
#[test]
fn rejection_leaves_the_document_at_the_committed_after() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem3d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{label}: a refused mutation must leave every one of the nine members untouched");
}}

/// 🚨️ The branch this vector pins, level and address together.
#[test]
fn the_refusal_is_the_declared_diagnostic() {{
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem3d::diff::Fem3dDiff::default(), "{label}: a refused mutation must carry the empty diff");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "{label}: exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "{outcome["code"]}", "{label}: the refusal is reported as {outcome["code"]}");
    assert_eq!(messages[0].level, protocol::Severity::{LEVEL_VARIANT[level]}, "{label}: {LEVEL_PROSE[outcome["code"]]}");
    assert_eq!(messages[0].target, {expected_target}, "{label}: the diagnostic addresses exactly {path}");
}}

/// ↩️ The inverse is computed from `before` and the mutation payload alone, never from the verdict,
/// so a refused request still has the one well-formed undo step its kind always emits.
#[test]
fn inverse_has_the_declared_shape() {{
    let inverse = inverse_fem3d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "{label}: this kind always emits exactly one undo step, got {{inverse:?}}");
    assert!(matches!(inverse[0], Fem3dMutation::{INVERSE_VARIANT[kind]}(_)), "{label}: the inverse of {kind} is a {INVERSE_VARIANT[kind]}, got {{:?}}", inverse[0]);
}}

/// 🎯️ The declared outcome — status, code, level and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "{label}: this vector declares a rejected outcome");
    let produced = <Fem3dMutation as protocol::Mutation<Fem3dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "{label}: the declared code must match the emitted one");
    let declared_path: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared_path, message.target, "{label}: the declared path must match the emitted target");
}}

/// 🔣️ Both committed snapshots and the committed mutation are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Fem3dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{label}} JSON is not canonical");
    }}
    let reencoded = dsl::ToValue::to_value(&mutation());
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not canonical");
}}

/// 🪪️ The fixture is bound to this very kind's own semantic descriptor, never a sibling's.
#[test]
fn semantics_bind_the_declared_kind() {{
    let semantics = <Fem3dMutation as protocol::SemanticMutation<Fem3dSnapshot>>::semantics(&mutation());
    assert_eq!((semantics.verb, semantics.entity, semantics.kind, semantics.record), ("{verb}", "{noun}", "{kind}", "{record_name}"), "{label}: the fixture must be bound to {kind}'s own descriptor");
}}
'''


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(value, handle, ensure_ascii=False, indent=2)
        handle.write("\n")


def write_cases():
    written = []
    for entry in cases():
        kind = entry["kind"]
        subset, kind_directory = KIND_DIRECTORY[kind]
        name = directory_name(entry)
        root = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", kind_directory, "🧪️tests", name)
        after, outcome = rules.apply_mutation(entry["before"], entry["mutation"])
        assert rules.rejected(outcome), (kind, name, outcome)
        assert after == entry["before"], (kind, name)
        committed = {"status": "rejected", "code": outcome["code"], "path": outcome["path"], "messages": outcome["messages"]}
        os.makedirs(os.path.join(root, "🔺️diff"), exist_ok=True)
        dump(os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json"), entry["before"])
        dump(os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json"), after)
        dump(os.path.join(root, "🦠️mutation", "🔣️.json"), entry["mutation"])
        dump(os.path.join(root, "🎯️outcome", "🔣️.json"), committed)
        open(os.path.join(root, "🔺️diff", "🚫️.absent"), "w", encoding="utf-8").close()
        with open(os.path.join(root, "🦀️.rs"), "w", encoding="utf-8") as handle:
            handle.write(rust(entry, outcome))
        written.append((kind, name, outcome["code"], len(name)))
    for kind, name, code, length in written:
        print("%-26s %-28s %-28s %d chars" % (kind, name, code, length))
    print("%d case bundles written" % len(written))
    return written


# endregion 🔖️Emission


# region 🔖️Registration
def patch_crate():
    with open(CRATE, encoding="utf-8") as handle:
        text = handle.read()
    added = 0
    for entry in cases():
        subset, kind_directory = KIND_DIRECTORY[entry["kind"]]
        name = directory_name(entry)
        path = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/%s/🧬️schema/🧬️mutations/%s/🧪️tests/%s/🦀️.rs" % (subset, kind_directory, name)
        if path in text:
            continue
        anchor = "../../🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/%s/🧬️schema/🧬️mutations/%s/🦀️.rs\"]\n                                    mod component;\n                                    pub use component::*;\n" % (subset, kind_directory)
        at = text.index(anchor) + len(anchor)
        block = '                                    #[cfg(test)]\n                                    #[path = "%s"]\n                                    mod %s;\n' % (path, module_name(entry))
        text = text[:at] + block + text[at:]
        added += 1
    with open(CRATE, "w", encoding="utf-8") as handle:
        handle.write(text)
    print("crate mounts added:", added)


def patch_catalogs():
    grouped = {}
    for entry in cases():
        subset, _ = KIND_DIRECTORY[entry["kind"]]
        grouped.setdefault(subset, []).append((entry["kind"], directory_name(entry), scenario_id(entry)))
    for subset, entries in grouped.items():
        path = os.path.join(SUBSETS, subset, "🔮️oracle", "🔣️.json")
        with open(path, encoding="utf-8") as handle:
            catalog = json.load(handle)
        added = 0
        for kind, name, identifier in entries:
            for vector in catalog["mutationCatalogs"][0]["vectors"]:
                if vector["mutationId"] != kind or any(scenario["directoryName"] == name for scenario in vector["scenarios"]):
                    continue
                vector["scenarios"].insert(0, {"id": identifier, "directoryName": name})
                added += 1
        with open(path, "w", encoding="utf-8") as handle:
            json.dump(catalog, handle, ensure_ascii=False, indent=2)
            handle.write("\n")
        print("%s catalog entries added: %d" % (subset, added))


def patch_taxonomy():
    with open(TAXONOMY, encoding="utf-8") as handle:
        taxonomy = json.load(handle)
    names = taxonomy["semanticDirectoryMemberKinds"]["members-of-tests"]["memberNames"]
    added = 0
    for entry in cases():
        name = directory_name(entry)
        if name not in names:
            names.append(name)
            added += 1
    with open(TAXONOMY, "w", encoding="utf-8") as handle:
        json.dump(taxonomy, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print("taxonomy members-of-tests rows added:", added, "total", len(names), "duplicates", len(names) - len(set(names)))


# endregion 🔖️Registration


def main():
    commands = sys.argv[1:] or ["manifest"]
    for command in commands:
        if command == "cases":
            write_cases()
        elif command == "patch":
            patch_crate()
            patch_catalogs()
            patch_taxonomy()
        elif command == "manifest":
            for entry in cases():
                print("%-26s %s" % (entry["kind"], directory_name(entry)))
        else:
            raise SystemExit("unknown command %r" % command)
    return 0


if __name__ == "__main__":
    sys.exit(main())
