#!/usr/bin/env python3
"""🧪️ W13 — authors the fem2d fixture cases the HARDENED semantics create, and registers them.

W10 gave every one of the 25 kinds three vectors: the pre-existing happy path, a second happy path
on the two-storey braced steel frame, and one edge case pinning the single refusal branch the kind
had. This wave adds branches — referential integrity on six `delete-` verbs, identity and
foreign-key resolution on five `replace-` verbs, the load target on `add-load`, geometry on the two
region verbs, plausibility on the material/section verbs, and bounds on
`update-analysis-settings` — so it adds one vector per NEW branch: **23**, taking fem2d from 75 to
98 committed vectors and every kind to at least three.

Every new case is a REFUSAL, and every one uses the same real model W10 authored (the steel frame),
so the whole corpus keeps talking about one building. The single exception is
`delete-combination`, whose new branch needs a combination that NESTS another; the steel frame has
none, so that case's `⬅️before` is the frame plus one design-envelope combination.

The refusal each case pins is not asserted from this file's opinion: `🔨️w13-fem2d-rules.py` is the
transcription of the guard order the Rust actually runs, and `prepare()` asks it, so a case whose
declared code drifts from the implementation fails to author at all.

Usage:
    uv run python 🔨️w13-fem2d-cases.py             # write cases + patches
    uv run python 🔨️w13-fem2d-cases.py --check     # report differences, write nothing
"""

# region 🔖️Imports
import copy
import hashlib
import importlib.util
import json
import os
import sys

# endregion 🔖️Imports


# region 🔖️Harness
HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets")
CRATE_ENTRY = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "📦️packages", "🦀️rust", "🦀️.rs")
TAXONOMY = os.path.join(REPO, "🧰️framework", "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🔣️taxonomy.json")


def load(name, stem):
    spec = importlib.util.spec_from_file_location(name, os.path.join(HERE, stem))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


w10 = load("w13_w10_cases", "🔨️w10-fem2d-cases.py")
"""📚️ W10's authoring script — reused verbatim for the steel-frame MODEL, the record constructors
and the canonical-JSON writer, so the new vectors are byte-compatible with the 50 it wrote."""

rules = load("w13_rules", "🔨️w13-fem2d-rules.py")
"""⚖️ The transcription of the hardened guard order — the arbiter of every declared refusal."""

MODEL = w10.MODEL
dump = w10.dump
F = w10.F
SUBSET_OF, KIND_DIR = w10.SUBSET_OF, w10.KIND_DIR
node, beam, bar = w10.node, w10.beam, w10.bar
material, section, support = w10.material, w10.section, w10.support
region, nodal, combination, term = w10.region, w10.nodal, w10.combination, w10.term
# endregion 🔖️Harness


# region 🔖️Bases
ENVELOPE_MODEL = copy.deepcopy(MODEL)
ENVELOPE_MODEL["combinations"] = MODEL["combinations"] + [combination("env1", "Design Envelope", [term("uls1", 1.0), term("sls1", 1.0)])]
"""🧮️ The steel frame plus one DESIGN ENVELOPE that nests `uls1` and `sls1`.

The only new base this wave authors, and only because it has to: `delete-combination`'s new
`mutation.target-referenced` branch needs a combination that another combination names, and the
frame W10 authored has three flat combinations and no nesting. An envelope over the ultimate and
serviceability results is the ordinary reason a real model nests one.
"""
# endregion 🔖️Bases


# region 🔖️Cases
def suffix(long_name):
    return hashlib.sha1(long_name.encode("utf-8")).hexdigest()[:6]


def case(kind, emoji, slug, long_name, mutation, note, base=None):
    identity = "%s-%s" % (slug, suffix(long_name))
    return {"kind": kind, "id": identity, "directory": emoji + identity, "module": "tests_" + long_name.replace("-", "_"), "long": long_name, "mutation": mutation, "note": note, "base": MODEL if base is None else base}


WINDOW = [(2.4, 1.0), (3.6, 1.0), (3.6, 2.4), (2.4, 2.4)]
UPPER_BAY = [(0.0, 3.5), (6.0, 3.5), (6.0, 7.0), (0.0, 7.0)]
LOWER_BAY = [(0.0, 0.0), (6.0, 0.0), (6.0, 3.5), (0.0, 3.5)]
DOOR_BESIDE_THE_PANEL = [(7.0, 4.0), (8.0, 4.0), (8.0, 5.5), (7.0, 5.5)]

CASES = [
    case(
        "delete-element",
        "🔗️",
        "blocks-udl",
        "refuses-to-delete-the-floor-beam-two-member-udls-still-load",
        {"mutation": "deleteElement", "id": "b1"},
        "🔗️ The floor beam carries the dead-case and imposed-case member UDLs. Deleting it used to be\naccepted and left both loads pointing at nothing; it is now `mutation.target-referenced`, and the\ndiagnostic names the target first and then every referrer as `<case>/<load>`, so the caller can\noffer to strip them.",
    ),
    case(
        "replace-element",
        "🪪️",
        "denies-rename",
        "refuses-to-rename-the-roof-beam-through-a-replace-element",
        {"mutation": "replaceElement", "id": "b2", "newElement": beam("b3", "n5", "n6", "steel_s355", "ipe240")},
        "🪪️ A `replace-` selects by `id` and carries a record that has its own `id`; letting them differ\nis a silent RENAME that orphans every referrer. It is now `mutation.id-mismatch`, FATAL — renaming\na member is `delete-` plus `create-` plus re-pointing the loads, never a whole-value swap.",
    ),
    case(
        "replace-element",
        "🚫️",
        "dangling-start",
        "refuses-to-replace-the-brace-onto-a-start-node-that-does-not-exist",
        {"mutation": "replaceElement", "id": "br1", "newElement": bar("br1", "n42", "n3", "steel_s355", "chs889")},
        "🚫️ `replace-element` now resolves the SAME four foreign keys `create-element` does, through the\nsame `guards::element_references`. The `start` node is read first, so `n42` is the address the\ndiagnostic carries even though the rest of the member is well formed.",
    ),
    case(
        "create-material",
        "⚗️",
        "denies-poisson",
        "refuses-an-elastomeric-bearing-at-the-incompressible-poisson-limit",
        {"mutation": "createMaterial", "material": material("bearing_rubber", "Elastomeric Bearing", 5000000.0, 0.5, 1100.0)},
        "⚗️ An elastomer at ν = 0.5 is incompressible, and the plane constitutive matrix built from it is\nsingular — the solve would fail far from this edit with no diagnostic pointing back at it. The\nadmissible interval is the open (-1, 0.5), so the limit itself is refused.",
    ),
    case(
        "delete-material",
        "🔗️",
        "blocks-in-use",
        "refuses-to-delete-the-s355-grade-seven-members-are-made-of",
        {"mutation": "deleteMaterial", "id": "steel_s355"},
        "🔗️ Every one of the frame's seven members is S355. Deleting the grade used to be accepted and\nleft all seven pointing at a material that no longer existed; it is now\n`mutation.target-referenced`, listing all seven.",
    ),
    case(
        "replace-material",
        "🪪️",
        "denies-rename",
        "refuses-to-regrade-the-concrete-by-renaming-it-through-a-replace",
        {"mutation": "replaceMaterial", "id": "concrete_c30", "newMaterial": material("concrete_c35", "Concrete C35/45", 34000000000.0, 0.2, 2500.0)},
        "🪪️ Upgrading the panel concrete by renaming the row would orphan both regions that name\n`concrete_c30` while looking like an edit to one record. `mutation.id-mismatch`, FATAL.",
    ),
    case(
        "replace-material",
        "⚗️",
        "denies-zero-modulus",
        "refuses-a-concrete-row-whose-modulus-was-left-at-zero",
        {"mutation": "replaceMaterial", "id": "concrete_c30", "newMaterial": material("concrete_c30", "Concrete C30/37", 0.0, 0.2, 2500.0)},
        "⚗️ The half-filled form: the grade was renamed and the modulus never entered. A zero Young's\nmodulus gives the panel no stiffness at all, so the SAME `guards::material_plausibility` bound\n`create-material` runs refuses it here too — the twins finally agree.",
    ),
    case(
        "create-section",
        "⚗️",
        "denies-zero-area",
        "refuses-an-ipe-100-profile-whose-area-was-left-at-zero",
        {"mutation": "createSection", "section": section("ipe100", "IPE 100", 0.0, 0.000001710)},
        "⚗️ A zero area gives the member zero axial stiffness and a singular row in the assembled system.\nThe inertia is right, which is exactly why this is worth refusing: nothing downstream would have\nlooked wrong until the solve.",
    ),
    case(
        "delete-section",
        "🔗️",
        "blocks-in-use",
        "refuses-to-delete-the-heb-200-profile-four-columns-still-carry",
        {"mutation": "deleteSection", "id": "heb200"},
        "🔗️ All four columns are HEB 200. `mutation.target-referenced` names the profile and then `c1`\nthrough `c4`.",
    ),
    case(
        "replace-section",
        "🪪️",
        "denies-rename",
        "refuses-to-rename-the-roof-beam-profile-through-a-replace-section",
        {"mutation": "replaceSection", "id": "ipe240", "newSection": section("ipe240a", "IPE 240 A", 0.003912, 0.00003892)},
        "🪪️ The roof beam names `ipe240`; renaming the profile row through a replace would leave it\npointing at nothing. `mutation.id-mismatch`, FATAL.",
    ),
    case(
        "replace-section",
        "⚗️",
        "denies-zero-iy",
        "refuses-a-roof-beam-profile-whose-second-moment-was-left-at-zero",
        {"mutation": "replaceSection", "id": "ipe240", "newSection": section("ipe240", "IPE 240", 0.003912, 0.0)},
        "⚗️ A zero second moment of area gives a `beam` no bending stiffness while its `area` still says it\nis there — the most quietly wrong section a model can hold. The same bound `create-section` runs.",
    ),
    case(
        "replace-support",
        "🪪️",
        "denies-rename",
        "refuses-to-rename-the-roof-tie-support-through-a-replace-support",
        {"mutation": "replaceSupport", "id": "sup_tie", "newSupport": support("sup_tie_b", "n6", ["Tx"])},
        "🪪️ Nothing in this vocabulary names a support id, so a rename here orphans no referrer — and it is\nstill refused, because a `replace-` that changes identity is not a replacement. The rule is the\nverb's, not the noun's.",
    ),
    case(
        "replace-support",
        "👻️",
        "dangling-node",
        "refuses-to-move-the-roof-tie-onto-a-node-that-does-not-exist",
        {"mutation": "replaceSupport", "id": "sup_tie", "newSupport": support("sup_tie", "n42", ["Tx"])},
        "👻️ `replace-support` now resolves `node_id` through the SAME `guards::node_reference`\n`create-support` uses. A support on a node that is not there restrains nothing and silently\nremoves a boundary condition from the model.",
    ),
    case(
        "create-region",
        "📐️",
        "denies-two-point",
        "refuses-a-floor-slab-region-outlined-by-only-two-points",
        {"mutation": "createRegion", "region": region("slab_l1", "First Floor Slab", [(0.0, 3.5), (6.0, 3.5)], [], 0.2, "concrete_c30", 0.5)},
        "📐️ Two points are a line, not a polygon: there is no area to mesh and\n`fem2d_engine::meshing::build_nodes_and_elements` would return an empty triangulation with no\ndiagnostic naming this edit. `guards::region_geometry` refuses it at the source.",
    ),
    case(
        "create-region",
        "🕳️",
        "denies-loose-hole",
        "refuses-an-upper-bay-panel-whose-door-opening-lies-outside-it",
        {"mutation": "createRegion", "region": region("panel_door", "Upper Bay Panel", UPPER_BAY, [DOOR_BESIDE_THE_PANEL], 0.2, "concrete_c30", 0.5)},
        "🕳️ The door opening was drawn beside the panel instead of in it — the bay spans x ∈ [0, 6] and the\nopening sits at x ∈ [7, 8]. A hole must be cut FROM its outline; touching the boundary is allowed\n(a notch), leaving it is not.",
    ),
    case(
        "delete-region",
        "🔗️",
        "blocks-in-use",
        "refuses-to-delete-the-infill-panel-the-wind-case-still-presses-on",
        {"mutation": "deleteRegion", "id": "wall1"},
        "🔗️ The wind case puts 640 Pa over `wall1`. Deleting the panel used to be accepted and left the area\nload addressing a region that no longer existed.",
    ),
    case(
        "replace-region",
        "🪪️",
        "denies-rename",
        "refuses-to-rename-the-infill-panel-through-a-replace-region",
        {"mutation": "replaceRegion", "id": "wall1", "newRegion": region("wall2", "Infill Wall Panel", LOWER_BAY, [WINDOW], 0.2, "concrete_c30", 0.5)},
        "🪪️ Renaming the panel would orphan the wind case's area load while presenting itself as a geometry\nedit. `mutation.id-mismatch`, FATAL.",
    ),
    case(
        "replace-region",
        "👻️",
        "dangling-mat",
        "refuses-to-repour-the-infill-panel-in-a-grade-the-model-lacks",
        {"mutation": "replaceRegion", "id": "wall1", "newRegion": region("wall1", "Infill Wall Panel", LOWER_BAY, [WINDOW], 0.2, "concrete_c50", 0.5)},
        "👻️ `replace-region` now resolves `material_id` through the SAME `guards::material_reference`\n`create-region` uses. `concrete_c50` was never added to the catalogue.",
    ),
    case(
        "replace-region",
        "📐️",
        "denies-zero-thick",
        "refuses-an-infill-panel-whose-thickness-was-set-to-zero",
        {"mutation": "replaceRegion", "id": "wall1", "newRegion": region("wall1", "Infill Wall Panel", LOWER_BAY, [WINDOW], 0.0, "concrete_c30", 0.5)},
        "📐️ A plane-stress region's thickness multiplies every term of its element stiffness; at zero the\npanel is present in the mesh and absent from the structure. The same geometry bound\n`create-region` runs.",
    ),
    case(
        "delete-load-case",
        "🔗️",
        "blocks-in-use",
        "refuses-to-delete-the-dead-case-three-combinations-still-weight",
        {"mutation": "deleteLoadCase", "id": "dead"},
        "🔗️ All three combinations weight the dead case. Deleting it used to be accepted and left every one\nof them with a term naming a case that no longer existed — a combination that then silently\nevaluates to less than it says.",
    ),
    case(
        "add-load",
        "👻️",
        "dangling-node",
        "refuses-to-push-a-wind-load-at-a-node-the-frame-does-not-have",
        {"mutation": "addLoad", "caseId": "wind", "load": nodal("lw4", "n42", "Tx", 7500.0)},
        "👻️ The asymmetry this closes: `create-load-case` resolved every load it carried, `add-load`\nresolved none, so the very same nodal load was refused arriving inside a new case and accepted\nattached to an existing one. Both now go through `guards::load_reference`.",
    ),
    case(
        "delete-combination",
        "🔗️",
        "blocks-in-use",
        "refuses-to-delete-an-uls-combination-a-design-envelope-nests",
        {"mutation": "deleteCombination", "id": "uls1"},
        "🔗️ This is the one case whose model is not the plain steel frame: the frame's three combinations\nare flat, and this branch needs one that NESTS another, so `⬅️before` adds a design envelope over\n`uls1` and `sls1`. A combination term may name a load case OR another combination\n(`create-combination` resolves both), which is exactly what makes this reference possible.",
        base=ENVELOPE_MODEL,
    ),
    case(
        "update-analysis-settings",
        "🚫️",
        "denies-zero-modes",
        "refuses-an-analysis-configured-to-extract-zero-modal-modes",
        {"mutation": "updateAnalysisSettings", "settings": {"modalCount": 0, "bucklingCount": 4, "deformationScale": F(120.0)}},
        "🚫️ Before this wave `update-analysis-settings` had NO rejection branch at all — its only guard was\nthe equality no-op, so zero modes, four billion modes and a negative deformation scale were all\naccepted unconditionally (W10 finding F1). It now carries bounds, and this is the first vector in\nthe corpus that pins them. The analysis facet has no id, so the diagnostic's target is empty.",
    ),
]
# endregion 🔖️Cases


# region 🔖️Rust
HEADER = """//! 🧪️ `{kind}` fixture — `{directory}`.
//!
//! Source of truth is the committed JSON quartet beside this file (contract D1, ticket
//! `26/08/20/COMPOSE-TO-PUZZLE5D-MIGRATION`). The `.op.semio`/`.spr.semio`/`.dsl.semio`/
//! `.pack.semio`/`.patch.semio` encodings are derived from it by `fixtures generate` and are
//! asserted by the shared codec-matrix harness, not here.
//!
//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening){model_note}.
//! Every value is in SI base units.
//!
//! 🛡️ This vector pins a branch the 26/09/06/FEM-PLUGIN-END-TO-END hardening wave ADDED; before it
//! the payload below was accepted (see `📓️w13-fem2d-semantics.md` for the per-kind rule table).
//!
{note}

use crate::artifacts::fem2d::mutations::Fem2dMutation;
use crate::artifacts::fem2d::mutations::{{apply_fem2d_mutation, inverse_fem2d_mutation}};
use crate::artifacts::fem2d::Fem2dSnapshot;

const BEFORE: &str = include_str!("📸️snapshot/⬅️before/🔣️.json");
const AFTER: &str = include_str!("📸️snapshot/➡️after/🔣️.json");
const MUTATION: &str = include_str!("🦠️mutation/🔣️.json");
const OUTCOME: &str = include_str!("🎯️outcome/🔣️.json");

fn before() -> Fem2dSnapshot {{
    dsl::json::from_json_str(BEFORE).expect("before snapshot decodes")
}}
fn expected_after() -> Fem2dSnapshot {{
    dsl::json::from_json_str(AFTER).expect("after snapshot decodes")
}}
fn mutation() -> Fem2dMutation {{
    dsl::json::from_json_str(MUTATION).expect("mutation decodes")
}}

/// ▶️ A refused `{kind}` still applies cleanly — the refusal is carried as a diagnostic beside an
/// EMPTY diff (§C2 LAW 1/2), so `apply` is a no-op rather than an `Err`. The document therefore
/// comes out byte-identical to the committed `after`, which is the committed `before`.
#[test]
fn rejection_leaves_the_document_untouched() {{
    let base = before();
    let mut snapshot = base.clone();
    apply_fem2d_mutation(&mut snapshot, &mutation()).expect("an empty diff still applies cleanly");
    assert_eq!(snapshot, expected_after(), "{label}: applied state differs from committed after-snapshot");
    assert_eq!(snapshot, base, "{label}: a refused mutation must leave the snapshot exactly where it was");
}}

/// 🚨️ The refusal is the diagnostic this kind's own diff builder raises, at its own level.
#[test]
fn the_refusal_is_the_declared_diagnostic() {{
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    assert_eq!(produced.diff(), &crate::artifacts::fem2d::diff::Fem2dDiff::default(), "{label}: a rejecting {kind} must carry the empty diff, never a half-built delta");
    let messages = produced.messages();
    assert_eq!(messages.len(), 1, "exactly one diagnostic is expected, got {{messages:?}}");
    assert_eq!(messages[0].code.0, "{code}", "{label}: the refusal is reported as {code}");
    assert_eq!(messages[0].level, protocol::Severity::{severity}, "{severity_note}");
    assert_eq!(messages[0].target, {target}, "{target_note}");
    let semantics = <Fem2dMutation as protocol::SemanticMutation<Fem2dSnapshot>>::semantics(&mutation());
    assert_eq!(semantics.kind, "{kind}", "the fixture must be bound to {kind}'s own descriptor");
}}

/// ↩️ {inverse_doc}
#[test]
fn inverse_of_the_refused_mutation() {{
    let inverse = inverse_fem2d_mutation(&before(), &mutation());
    assert_eq!(inverse.len(), 1, "{label}: {kind} undoes with exactly one step, got {{inverse:?}}");
    let Fem2dMutation::{variant}(undo) = &inverse[0] else {{
        panic!("{kind}'s inverse must be a {variant}, got {{:?}}", inverse[0]);
    }};
{inverse_assert}}}

/// 🎯️ The declared rejection — status, code and path — is exactly what the diff builder emits.
#[test]
fn declared_outcome_holds() {{
    let outcome: dsl::DslValue = dsl::json::from_json_str(OUTCOME).expect("outcome decodes");
    assert_eq!(outcome.get("status").and_then(dsl::DslValue::as_str), Some("rejected"), "{label} declares a rejected outcome");
    let produced = <Fem2dMutation as protocol::Mutation<Fem2dSnapshot>>::diff(&mutation(), &before());
    let message = produced.messages().first().expect("a rejected outcome carries a diagnostic");
    assert_eq!(outcome.get("code").and_then(dsl::DslValue::as_str), Some(message.code.0.as_str()), "the declared code must match the emitted one");
    let declared: Vec<String> = outcome.get("path").and_then(dsl::DslValue::as_array).expect("a rejected outcome declares a path").iter().map(|entry| entry.as_str().expect("path segments are strings").to_string()).collect();
    assert_eq!(declared, message.target, "the declared path must match the emitted target");
}}

/// 🔣️ Both committed snapshots are already canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    for (label, text) in [("before", BEFORE), ("after", AFTER)] {{
        let decoded: Fem2dSnapshot = dsl::json::from_json_str(text).expect("snapshot decodes");
        let reencoded = dsl::ToValue::to_value(&decoded);
        let original: dsl::DslValue = dsl::json::from_json_str(text).expect("snapshot reparses");
        assert_eq!(reencoded, original, "{label}: committed {{label}} JSON is not canonical");
    }}
    assert_eq!(BEFORE, AFTER, "{label} changes nothing: the two committed snapshots must be byte-identical");
    let decoded_mutation = mutation();
    let reencoded = dsl::ToValue::to_value(&decoded_mutation);
    let original: dsl::DslValue = dsl::json::from_json_str(MUTATION).expect("mutation reparses");
    assert_eq!(reencoded, original, "{label}: committed mutation JSON is not canonical");
}}
"""

INVERSE = {
    "delete-element": ("CreateElement", "`delete-element`'s inverse is BASE-derived and the target is still there — a refusal does not remove it — so the inverse is the `create-element` that would put the very same member back.", '    assert_eq!(crate::artifacts::fem2d::element_id(&undo.element), "{target}", "the inverse recreates exactly the element the delete was refused for");\n'),
    "delete-material": ("CreateMaterial", "`delete-material`'s inverse is BASE-derived and the target survives the refusal, so the inverse is the `create-material` that would put the grade back.", '    assert_eq!(undo.material.id, "{target}", "the inverse recreates exactly the material the delete was refused for");\n'),
    "delete-section": ("CreateSection", "`delete-section`'s inverse is BASE-derived and the target survives the refusal, so the inverse is the `create-section` that would put the profile back.", '    assert_eq!(undo.section.id, "{target}", "the inverse recreates exactly the section the delete was refused for");\n'),
    "delete-region": ("CreateRegion", "`delete-region`'s inverse is BASE-derived and the target survives the refusal, so the inverse is the `create-region` that would put the panel back.", '    assert_eq!(undo.region.id, "{target}", "the inverse recreates exactly the region the delete was refused for");\n'),
    "delete-load-case": ("CreateLoadCase", "`delete-load-case`'s inverse is BASE-derived and the target survives the refusal, so the inverse is the `create-load-case` that would put the case AND its loads back.", '    assert_eq!(undo.load_case.id, "{target}", "the inverse recreates exactly the case the delete was refused for");\n    assert!(!undo.load_case.loads.is_empty(), "the inverse carries the case\'s loads too — they are members of the case, not a separate collection");\n'),
    "delete-combination": ("CreateCombination", "`delete-combination`'s inverse is BASE-derived and the target survives the refusal, so the inverse is the `create-combination` that would put it back.", '    assert_eq!(undo.combination.id, "{target}", "the inverse recreates exactly the combination the delete was refused for");\n'),
    "replace-element": ("ReplaceElement", "`replace-element`'s inverse is BASE-derived: it restores the element `before` holds, addressed by the SELECTED id — never by the id the refused replacement carried.", '    assert_eq!(undo.id, "{target}", "the inverse addresses the selected id, not the one the replacement carried");\n    assert_eq!(undo.new_element.as_ref(), before().elements.iter().find(|item| crate::artifacts::fem2d::element_id(item) == "{target}").expect("the target survives a refusal"), "the inverse restores the pre-mutation element");\n'),
    "replace-material": ("ReplaceMaterial", "`replace-material`'s inverse is BASE-derived: it restores the material `before` holds, addressed by the SELECTED id.", '    assert_eq!(undo.id, "{target}", "the inverse addresses the selected id, not the one the replacement carried");\n    assert_eq!(&undo.new_material, before().materials.iter().find(|item| item.id == "{target}").expect("the target survives a refusal"), "the inverse restores the pre-mutation material");\n'),
    "replace-section": ("ReplaceSection", "`replace-section`'s inverse is BASE-derived: it restores the section `before` holds, addressed by the SELECTED id.", '    assert_eq!(undo.id, "{target}", "the inverse addresses the selected id, not the one the replacement carried");\n    assert_eq!(&undo.new_section, before().sections.iter().find(|item| item.id == "{target}").expect("the target survives a refusal"), "the inverse restores the pre-mutation section");\n'),
    "replace-support": ("ReplaceSupport", "`replace-support`'s inverse is BASE-derived: it restores the support `before` holds, addressed by the SELECTED id.", '    assert_eq!(undo.id, "{target}", "the inverse addresses the selected id, not the one the replacement carried");\n    assert_eq!(&undo.new_support, before().supports.iter().find(|item| item.id == "{target}").expect("the target survives a refusal"), "the inverse restores the pre-mutation support");\n'),
    "replace-region": ("ReplaceRegion", "`replace-region`'s inverse is BASE-derived: it restores the region `before` holds, addressed by the SELECTED id.", '    assert_eq!(undo.id, "{target}", "the inverse addresses the selected id, not the one the replacement carried");\n    assert_eq!(&undo.new_region, before().regions.iter().find(|item| item.id == "{target}").expect("the target survives a refusal"), "the inverse restores the pre-mutation region");\n'),
    "create-material": ("DeleteMaterial", "`create-material`'s inverse is PAYLOAD-derived, not base-derived: it is a `delete-material` of the id it was asked to create, even when the create itself was refused.", '    assert_eq!(undo.id, "{target}", "the inverse addresses exactly the id the payload carried");\n'),
    "create-section": ("DeleteSection", "`create-section`'s inverse is PAYLOAD-derived: a `delete-section` of the id it was asked to create, even when the create itself was refused.", '    assert_eq!(undo.id, "{target}", "the inverse addresses exactly the id the payload carried");\n'),
    "create-region": ("DeleteRegion", "`create-region`'s inverse is PAYLOAD-derived: a `delete-region` of the id it was asked to create, even when the create itself was refused.", '    assert_eq!(undo.id, "{target}", "the inverse addresses exactly the id the payload carried");\n'),
    "add-load": ("RemoveLoad", "`add-load`'s inverse is BASE-derived in its GUARD only: the case exists, so it emits the `remove-load` that would detach the load — a step that is itself a `mutation.target-missing` refusal here, because the refused add never attached anything.", '    assert_eq!(undo.case_id, "{case}", "the inverse addresses the case the payload named");\n    assert_eq!(undo.load_id, "{target}", "the inverse addresses the load the payload carried");\n'),
    "update-analysis-settings": ("UpdateAnalysisSettings", "`update-analysis-settings` ALWAYS emits exactly one inverse step carrying `base.analysis`, refused or not — it has no branch that can collapse to `Vec::new()`.", '    assert_eq!(undo.settings, before().analysis, "the inverse carries the settings the document already had");\n'),
}
"""↩️ Per kind: the inverse variant, why it has that shape, and the assertion that pins it."""

SEVERITY_NOTE = {
    "mutation.duplicate-id": "a duplicate identity is an invariant breach — Fatal, and no merge policy may absorb it",
    "mutation.id-mismatch": "a replacement that renames its target is an identity breach — Fatal, and no merge policy may absorb it",
    "mutation.invariant": "an inadmissible payload is wrong against every base, not just this one — Fatal",
    "mutation.target-missing": "a missing target is an Error, the level a merge policy may still choose to tolerate",
    "mutation.target-referenced": "a live referrer is a property of THIS base, so it is an Error rather than a Fatal",
}


def rust_targets(target):
    if not target:
        return "Vec::<String>::new()", "the analysis facet has no id, so the diagnostic carries no address"
    listed = ", ".join('"%s".to_string()' % item for item in target)
    if len(target) == 1:
        return "vec![%s]" % listed, "the diagnostic addresses the offending id"
    return "vec![%s]" % listed, "the diagnostic addresses the target first, then every referrer"


def render_rust(entry):
    kind = entry["kind"]
    variant, inverse_doc, assertion = INVERSE[kind]
    target, note = rust_targets(entry["target"])
    model_note = "" if entry["base"] is MODEL else ", plus the one design-envelope combination this branch needs (see below)"
    return HEADER.format(
        kind=kind,
        directory=entry["directory"],
        note="\n".join("//! " + line for line in entry["note"].split("\n")),
        label=entry["label"],
        code=entry["code"],
        severity="Fatal" if entry["level"] == rules.FATAL else "Error",
        severity_note=SEVERITY_NOTE[entry["code"]],
        target=target,
        target_note=note,
        inverse_doc=inverse_doc,
        variant=variant,
        inverse_assert=assertion.format(target=entry["inverse_target"], case=entry["mutation"].get("caseId", "")),
        model_note=model_note,
    )


# endregion 🔖️Rust


# region 🔖️Emit
def inverse_target(entry):
    """🪪️ The id the inverse step must address, per the verb's own inverse shape."""
    kind, mutation = entry["kind"], entry["mutation"]
    if kind.startswith("create-"):
        for key in ("material", "section", "region"):
            if key in mutation:
                return mutation[key]["id"]
    if kind == "add-load":
        return mutation["load"]["id"]
    if kind == "update-analysis-settings":
        return ""
    return mutation["id"]


def prepare():
    for entry in CASES:
        entry["label"] = "%s/%s" % (entry["kind"], entry["id"])
        entry["inverse_target"] = inverse_target(entry)
        try:
            rules.apply_hardened(entry["base"], entry["mutation"])
        except rules.Reject as refusal:
            entry["code"], entry["level"], entry["target"] = refusal.code, refusal.level, refusal.target
        else:
            raise AssertionError("%s: the hardened rules ACCEPT this payload, so it is not a refusal case" % entry["label"])
        assert entry["level"] in (rules.FATAL, rules.ERROR), entry["label"]
        entry["outcome"] = {"status": "rejected", "code": entry["code"], "path": entry["target"]}
    identities = [entry["directory"] for entry in CASES]
    assert len(set(identities)) == len(identities), "case directory names must be unique"
    modules = [entry["module"] for entry in CASES]
    assert len(set(modules)) == len(modules), "case module names must be unique"
    return CASES


def case_root(entry):
    return os.path.join(SUBSETS, SUBSET_OF[entry["kind"]], "🧬️schema", "🧬️mutations", KIND_DIR[entry["kind"]], "🧪️tests", entry["directory"])


def write(path, text, check):
    existing = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    if existing == text:
        return "same"
    if check:
        return "differs"
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return "written"


def emit(check):
    prepare()
    tally = {}
    siblings = {}
    for entry in CASES:
        root = case_root(entry)
        assert len(entry["directory"]) <= 28, (entry["directory"], len(entry["directory"]))
        siblings.setdefault(os.path.dirname(root), []).append(entry["directory"])
        base = dump(entry["base"]) + "\n"
        for path, text in [
            (os.path.join(root, "🦠️mutation", "🔣️.json"), dump(entry["mutation"]) + "\n"),
            (os.path.join(root, "🎯️outcome", "🔣️.json"), dump(entry["outcome"]) + "\n"),
            (os.path.join(root, "📸️snapshot", "⬅️before", "🔣️.json"), base),
            (os.path.join(root, "📸️snapshot", "➡️after", "🔣️.json"), base),
            (os.path.join(root, "🔺️diff", "🚫️.absent"), ""),
            (os.path.join(root, "🦀️.rs"), render_rust(entry)),
        ]:
            state = write(path, text, check)
            tally[state] = tally.get(state, 0) + 1
    for parent, added in siblings.items():
        present = sorted(set(added) | set(os.listdir(parent))) if os.path.isdir(parent) else sorted(added)
        emojis = [w10.leading_emoji(name) for name in present]
        assert len(set(emojis)) == len(emojis), (parent, sorted(zip(emojis, present)))
    print("cases: %d, files: %s" % (len(CASES), ", ".join("%s %d" % item for item in sorted(tally.items()))))
    return tally


if __name__ == "__main__":
    check = "--check" in sys.argv[1:]
    tally = emit(check)
    sys.exit(1 if check and tally.get("differs") else 0)
# endregion 🔖️Emit
