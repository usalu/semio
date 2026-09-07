#!/usr/bin/env python3
"""🔌️ W12 — wire every fem3d vector into the cross-language differential.

W11 left 50 of its 75 vectors Rust-only because extending the Python reference alone would have
broken the feature (§5 of `📓️w11-fem3d-cases.md`). This script closes that: for each of the five
subsets it

1. upgrades the independent Python reference (`🌐️any/🧪️tests/*/🐍️.py`) to the hardened refusal
   rules — referential integrity, identity-preserving replaces, resolved foreign keys, physical
   plausibility, solid geometry and analysis bounds — and relaxes `observable()` so a vector that is
   MEANT not to move the model can be replayed;
2. adds a `hall-vector-<kind>` and a `reject-<vector-id>` handler to the reference's `adapter()`;
3. adds the mirror SUBJECT handlers to the Rust adapter (`<subset>/🧪️tests/*/🦀️.rs`);
4. adds the two matching `Scenario Outline`s and their `Examples` rows to the `🥒️.feature`.

The reference stays an independent re-implementation: the rules below are written from the record
shapes the snapshot schema and the committed vectors state, in the reference's own idiom, and no
Rust was copied into it — the two implementations are compared, not shared.

Usage: uv run python 🔨️w12-wire-fem3d-differential.py
"""

import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "🧊️3d", "🏅️standards", "🔖️1", "🪆️subsets")

CASES = {
    "🕸️mesh": ("🕸️mutate-fem3d-1-mesh", "🕸️mutate-fem3d-1-any-mesh"),
    "🧱️material": ("🧱️mutate-fem3d-1-material", "🧱️mutate-fem3d-1-any-material"),
    "🛡️boundary": ("🛡️mutate-fem3d-1-boundary", "🛡️mutate-fem3d-1-any-boundary"),
    "🏋️load": ("🏋️mutate-fem3d-1-load", "🏋️mutate-fem3d-1-any-load"),
    "📈️analysis": ("📈️mutate-fem3d-1-analysis", "📈️mutate-fem3d-1-any-analysis"),
}


# region 🔖️Inventory
def leading_emoji(name):
    emoji = ""
    for character in name:
        if character.isalnum() or character in "-_.":
            break
        emoji += character
    return emoji


def inventory(subset):
    """🗂️ Per kind directory: the happy vector, the hall vector and every refusal/no-op vector."""
    root = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations")
    found = []
    for kind_directory in sorted(os.listdir(root)):
        tests = os.path.join(root, kind_directory, "🧪️tests")
        if not os.path.isdir(tests):
            continue
        kind = None
        halls, rejects, happy = [], [], []
        for case in sorted(os.listdir(tests)):
            outcome = json.load(open(os.path.join(tests, case, "🎯️outcome", "🔣️.json"), encoding="utf-8"))
            mutation = json.load(open(os.path.join(tests, case, "🦠️mutation", "🔣️.json"), encoding="utf-8"))
            kind = kind_of(mutation["mutation"])
            no_op = any(entry["code"] == "mutation.no-op" for entry in outcome.get("messages", []))
            if case.startswith("🏗️"):
                halls.append(case)
            elif outcome["status"] == "rejected" or no_op:
                rejects.append(case)
            else:
                happy.append(case)
        assert len(halls) == 1 and len(happy) == 1, (subset, kind_directory, halls, happy)
        found.append({"kind": kind, "directory": kind_directory, "happy": happy[0], "hall": halls[0], "rejects": rejects})
    return found


def kind_of(tag):
    out = ""
    for character in tag:
        out += ("-" + character.lower()) if character.isupper() else character
    return out


def identifier_of(case):
    return case[len(leading_emoji(case)) :].lstrip("️")


# endregion 🔖️Inventory


# region 🔖️Reference
REFERENCE_DOC = '''* the committed `🎯️outcome` vectors — where the REFUSALS are written down: an identity that is
  already taken, a target that does not resolve, a `replace-` that would rename its target, a
  `delete-` whose target still has referrers, and a value or footprint no solver could assemble.
'''

INTEGRITY = '''

# region 🔖️Integrity
CASCADE_EXEMPT = ("delete-node",)
"""🕳️ The one verb whose committed vector states that it removes a node under a live element and
leaves the element naming it. Every other `delete-` refuses while a referrer is alive; this one is
specified permissive, so the reference is permissive with it too."""


def every_load(document):
    """🏋️ Every load in the model, case by case, in document order."""
    for case in document["loadCases"]:
        for load in case["loads"]:
            yield load


def referrers(document, collection, identifier):
    """🔗️ Every record id that would be left naming nothing if `identifier` left `collection`.

    Read straight off the record shapes the vectors state: an element names two nodes, a material
    and a section; a support names a node; a solid names a material; the three load variants name a
    node, an element and a solid respectively; a combination weights load cases by id.
    """
    found = []
    if collection == "nodes":
        found += [item["id"] for item in document["elements"] if identifier in (item["start"], item["end"])]
        found += [item["id"] for item in document["supports"] if item["nodeId"] == identifier]
        found += [load["id"] for load in every_load(document) if load["kind"] == "nodal" and load["nodeId"] == identifier]
    elif collection == "elements":
        found += [load["id"] for load in every_load(document) if load["kind"] == "memberUdl" and load["elementId"] == identifier]
    elif collection == "sections":
        found += [item["id"] for item in document["elements"] if item["sectionId"] == identifier]
    elif collection == "solids":
        found += [load["id"] for load in every_load(document) if load["kind"] == "area" and load["solidId"] == identifier]
    elif collection == "materials":
        found += [item["id"] for item in document["elements"] if item["materialId"] == identifier]
        found += [item["id"] for item in document["solids"] if item["materialId"] == identifier]
    elif collection == "loadCases":
        found += [item["id"] for item in document["combinations"] if identifier in item["terms"]]
    return found


def resolves(document, collection, identifier, kind):
    """🔎️ A foreign key the model must already carry — the same demand every verb makes of the same
    reference, whichever verb carries it."""
    if find(document[collection], identifier) is None:
        raise AssertionError("%s: %r is not in %s" % (kind, identifier, collection))


def resolve_element(document, element, kind):
    """🔩️ An element's four foreign keys, in the order the vectors report them."""
    resolves(document, "nodes", element["start"], kind)
    resolves(document, "nodes", element["end"], kind)
    resolves(document, "materials", element["materialId"], kind)
    resolves(document, "sections", element["sectionId"], kind)


def resolve_load(document, load, kind):
    """🎯️ The one thing a load hangs on, per variant."""
    if load["kind"] == "nodal":
        resolves(document, "nodes", load["nodeId"], kind)
    elif load["kind"] == "memberUdl":
        resolves(document, "elements", load["elementId"], kind)
    else:
        resolves(document, "solids", load["solidId"], kind)


def finite(*values):
    """🔢️ JSON cannot spell a non-finite literal, so this only ever fires on a computed value — it is
    stated because the rule is about the model, not about the carrier."""
    return all(isinstance(value, (int, float)) and value == value and abs(value) != float("inf") for value in values)


def ring_area(ring):
    """📏️ Twice the shoelace sum, halved — the signed area of a closed ring."""
    return sum(ring[at][0] * ring[(at + 1) % len(ring)][1] - ring[(at + 1) % len(ring)][0] * ring[at][1] for at in range(len(ring))) / 2.0


def encloses(ring, point):
    """🎯️ Crossing-count containment; a point exactly on an edge is undefined and never authored."""
    inside = False
    for at, corner in enumerate(ring):
        other = ring[(at + 1) % len(ring)]
        if (corner[1] > point[1]) != (other[1] > point[1]) and point[0] < corner[0] + (point[1] - corner[1]) / (other[1] - corner[1]) * (other[0] - corner[0]):
            inside = not inside
    return inside


def admissible(collection, record, kind):
    """🧨️ The values a record must carry to describe something a solver could assemble at all — a
    positive stiffness, a Poisson ratio strictly inside (-1, 0.5), a footprint with area, a positive
    extrusion through at least one layer, and holes that stay inside their outline."""
    if collection == "nodes":
        if not finite(record["x"], record["y"], record["z"]):
            raise AssertionError("%s: node %r must sit at a finite position" % (kind, record["id"]))
    elif collection == "materials":
        if not finite(record["e"], record["g"], record["nu"], record["rho"]) or min(record["e"], record["g"], record["rho"]) <= 0.0:
            raise AssertionError("%s: material %r must carry a positive finite e, g and rho" % (kind, record["id"]))
        if not -1.0 < record["nu"] < 0.5:
            raise AssertionError("%s: material %r must carry a Poisson ratio in (-1, 0.5), not %r" % (kind, record["id"], record["nu"]))
    elif collection == "sections":
        if not finite(record["area"], record["iy"], record["iz"], record["j"]) or min(record["area"], record["iy"], record["iz"], record["j"]) <= 0.0:
            raise AssertionError("%s: section %r must carry a positive finite area, iy, iz and j" % (kind, record["id"]))
    elif collection == "solids":
        if len(record["outline"]) < 3 or not all(finite(point[0], point[1]) for point in record["outline"]) or ring_area(record["outline"]) == 0.0:
            raise AssertionError("%s: solid %r needs a closed outline of at least three points and non-zero area" % (kind, record["id"]))
        if not finite(record["baseZ"], record["height"], record["meshSize"]) or record["height"] <= 0.0 or record["meshSize"] <= 0.0 or record["layers"] < 1:
            raise AssertionError("%s: solid %r needs a positive height, mesh size and layer count" % (kind, record["id"]))
        for hole in record["holes"]:
            if len(hole) < 3 or ring_area(hole) == 0.0 or not all(finite(point[0], point[1]) and encloses(record["outline"], point) for point in hole):
                raise AssertionError("%s: solid %r carries a hole that is degenerate or leaves its outline" % (kind, record["id"]))


def bounded(settings, kind):
    """⚙️ An eigen solve for fewer than one mode returns nothing, and a non-positive display scale
    collapses or mirrors the results view."""
    if settings["modalCount"] < 1 or settings["bucklingCount"] < 1:
        raise AssertionError("%s: at least one modal and one buckling factor are needed, not %r and %r" % (kind, settings["modalCount"], settings["bucklingCount"]))
    if not finite(settings["deformationScale"]) or settings["deformationScale"] <= 0.0:
        raise AssertionError("%s: the deformation scale must be finite and positive, not %r" % (kind, settings["deformationScale"]))


# endregion 🔖️Integrity
'''

APPLY_OLD = '''    if kind == "update-analysis-settings":
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind == "add-load":
        case = case_of(result, mutation["caseId"], kind)
        load = copy.deepcopy(mutation["load"])
        if find(case["loads"], load["id"]) is not None:
            raise AssertionError("%s: case %r already carries a load %r" % (kind, case["id"], load["id"]))
        case["loads"].append(load)
'''

APPLY_NEW = '''    if kind == "update-analysis-settings":
        bounded(mutation["settings"], kind)
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind == "add-load":
        case = case_of(result, mutation["caseId"], kind)
        load = copy.deepcopy(mutation["load"])
        if find(case["loads"], load["id"]) is not None:
            raise AssertionError("%s: case %r already carries a load %r" % (kind, case["id"], load["id"]))
        resolve_load(result, load, kind)
        case["loads"].append(load)
'''

APPLY_TAIL_OLD = '''        if kind.startswith("create-"):
            record = copy.deepcopy(mutation[create_argument])
            if find(items, record["id"]) is not None:
                raise AssertionError("%s: %r is already in %s" % (kind, record["id"], collection))
            items.append(record)
        elif kind.startswith("delete-"):
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            items.pop(at)
        else:
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            items[at] = copy.deepcopy(mutation[replace_argument])
'''

APPLY_TAIL_NEW = '''        if kind.startswith("create-"):
            record = copy.deepcopy(mutation[create_argument])
            if find(items, record["id"]) is not None:
                raise AssertionError("%s: %r is already in %s" % (kind, record["id"], collection))
            carried(result, collection, record, kind)
            items.append(record)
        elif kind.startswith("delete-"):
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            blockers = [] if kind in CASCADE_EXEMPT else referrers(result, collection, mutation["id"])
            if blockers:
                raise AssertionError("%s: %r in %s is still referenced by %r" % (kind, mutation["id"], collection, blockers))
            items.pop(at)
        else:
            at = find(items, mutation["id"])
            if at is None:
                raise AssertionError("%s: %r is not in %s" % (kind, mutation["id"], collection))
            record = copy.deepcopy(mutation[replace_argument])
            if record["id"] != mutation["id"]:
                raise AssertionError("%s: a replace selects %r and may not rename it to %r" % (kind, mutation["id"], record["id"]))
            if items[at] != record:
                carried(result, collection, record, kind)
            items[at] = record
'''

CARRIED = '''

def carried(document, collection, record, kind):
    """🚚️ Everything a record brings with it — the foreign keys it names and the values it must be
    admissible under — demanded identically of the `create-` and the `replace-` of one noun."""
    if collection == "elements":
        resolve_element(document, record, kind)
    elif collection == "supports":
        resolves(document, "nodes", record["nodeId"], kind)
    elif collection == "solids":
        resolves(document, "materials", record["materialId"], kind)
    elif collection == "loadCases":
        for load in record["loads"]:
            resolve_load(document, load, kind)
    elif collection == "combinations":
        for case_id in sorted(record["terms"]):
            resolves(document, "loadCases", case_id, kind)
    admissible(collection, record, kind)

'''

OBSERVABLE_OLD = '''def observable(scenario, before, after):
    """👁️ Every row below moves the model, so a forward application must move it. A mutation that
    quietly did nothing would otherwise agree with an unchanged model and report a pass."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the model untouched, so nothing was proved" % scenario)
'''

OBSERVABLE_NEW = '''def observable(scenario, before, after, moves=True):
    """👁️ A forward row moves the model, so a forward application must move it — a mutation that
    quietly did nothing would otherwise agree with an unchanged model and report a pass. The law is
    two-sided: pass `moves=False` for a row whose whole claim is that NOTHING happens (a refusal or
    a declared no-op), where a model that moved is the failure."""
    if moves and before == after:
        raise AssertionError("%s: the forward mutation left the model untouched, so nothing was proved" % scenario)
    if not moves and before != after:
        raise AssertionError("%s: the vector claims nothing happens, but the model moved" % scenario)
'''

REJECT_HANDLER = '''

def reject_handler():
    """🚫️ Replays a committed vector whose whole claim is that the model does NOT move — either the
    request is refused outright, or it is a declared no-op. Both implementations project the same
    two facts: whether the request was refused, and the model it left behind.

    The refusal REASON is deliberately not projected. Each implementation words it in its own
    language; what the differential can hold them to is the verdict and the document.
    """

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        after = document_of(json_fixture(ctx, "➡️after"))
        mutation = json_fixture(ctx, "🦠️mutation")
        kind = kind_of(mutation)
        if kind not in KINDS:
            raise AssertionError("reject: %s is not one of this subset's kinds" % kind)
        equals_committed(kind, before, after)
        try:
            applied = apply_mutation(before, mutation)
        except AssertionError:
            return outcome_of({"refused": True, "model": before})
        observable("reject-%s" % kind, before, applied, moves=False)
        return outcome_of({"refused": False, "model": applied})

    return handler
'''

REGISTRATION_OLD = '''    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        built = built.oracle("spec-vector-%s" % kind, spec_vector_handler(kind))
    return built
'''

REGISTRATION_NEW = '''    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        built = built.oracle("spec-vector-%s" % kind, spec_vector_handler(kind))
        built = built.oracle("hall-vector-%s" % kind, spec_vector_handler(kind))
    for identifier in REJECT_VECTORS:
        built = built.oracle("reject-%s" % identifier, reject_handler())
    return built
'''


def patch_reference(path, reject_vectors):
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    if "REJECT_VECTORS" in text:
        return "already patched"
    anchor = "* ``…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`` — this subset's own verbs.\n"
    assert anchor in text
    text = text.replace(anchor, anchor + REFERENCE_DOC, 1)
    assert "# endregion 🔖️Document" in text
    text = text.replace("\n# endregion 🔖️Document\n", "\n# endregion 🔖️Document\n" + INTEGRITY, 1)
    assert APPLY_OLD in text
    text = text.replace(APPLY_OLD, APPLY_NEW, 1)
    assert APPLY_TAIL_OLD in text
    text = text.replace(APPLY_TAIL_OLD, APPLY_TAIL_NEW, 1)
    text = text.replace("\ndef inverse_mutation(document, mutation):", CARRIED + "\ndef inverse_mutation(document, mutation):", 1)
    assert OBSERVABLE_OLD in text
    text = text.replace(OBSERVABLE_OLD, OBSERVABLE_NEW, 1)
    text = text.replace("\n# endregion 🔖️Handlers", REJECT_HANDLER + "\n# endregion 🔖️Handlers", 1)
    assert REGISTRATION_OLD in text
    text = text.replace(REGISTRATION_OLD, REGISTRATION_NEW, 1)
    listing = "\n".join('    "%s",' % identifier for identifier in reject_vectors)
    block = '\nREJECT_VECTORS = (\n%s\n)\n"""🚫️ The committed vectors of this subset that claim the model does NOT move — every refusal this\nvocabulary can raise plus every declared no-op, named by the scenario id the feature\'s `@id-reject`\nrows carry."""\n' % listing
    text = text.replace('\nTAGS = {kind: tag_of(kind) for kind in KINDS}\n', '\nTAGS = {kind: tag_of(kind) for kind in KINDS}\n' + block, 1)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return "patched"


# endregion 🔖️Reference


# region 🔖️Feature
def feature_block(tag, title, rows, extra):
    lines = ["", "  @id-%s" % tag, "  @level-exhaustive", "  @mode-differential", "  Scenario Outline: %s" % title]
    lines += [
        "    Given the committed before-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/⬅️before/🔣️.json",
        "    And the committed mutation asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/🦠️mutation/🔣️.json",
        "    And the committed after-model asset://🧬️schema/🧬️mutations/<dir>/🧪️tests/<fixture>/📸️snapshot/➡️after/🔣️.json",
        "    When the committed mutation is applied to the committed before-model",
        "    Then %s" % extra,
        "    Examples:",
    ]
    widths = [max(len(row[column]) for row in rows + [("id", "dir", "fixture")]) for column in range(3)]
    header = ("id", "dir", "fixture")
    for row in [header] + rows:
        lines.append("    | " + " | ".join(row[column].ljust(widths[column]) for column in range(3)) + " |")
    return "\n".join(lines) + "\n"


def patch_feature(path, entries):
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    if "@id-hall-vector" in text:
        return "already patched"
    hall_rows = [(entry["kind"], entry["directory"], entry["hall"]) for entry in entries]
    reject_rows = [(identifier_of(case), entry["directory"], case) for entry in entries for case in entry["rejects"]]
    text = text.rstrip("\n") + "\n"
    text += feature_block(
        "hall-vector",
        "Replay the committed <id> glulam-hall vector through both implementations",
        hall_rows,
        "each implementation lands on the committed after-model in role, only the member this verb writes moved, and the two agree",
    )
    text += feature_block(
        "reject",
        "Refuse the committed <id> vector in both implementations and leave the model where it was",
        reject_rows,
        "both implementations refuse it, or declare it a no-op, and leave the committed before-model exactly as it was",
    )
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return "patched"


# endregion 🔖️Feature


# region 🔖️Subject
def vector_arm(kind_directory, case, with_diff):
    parts = [
        '            before: include_str!("../../🧬️schema/🧬️mutations/%s/🧪️tests/%s/📸️snapshot/⬅️before/🔣️.json"),' % (kind_directory, case),
        '            mutation: include_str!("../../🧬️schema/🧬️mutations/%s/🧪️tests/%s/🦠️mutation/🔣️.json"),' % (kind_directory, case),
        '            after: include_str!("../../🧬️schema/🧬️mutations/%s/🧪️tests/%s/📸️snapshot/➡️after/🔣️.json"),' % (kind_directory, case),
    ]
    if with_diff:
        parts.append('            diff: include_str!("../../🧬️schema/🧬️mutations/%s/🧪️tests/%s/🔺️diff/🔣️.json"),' % (kind_directory, case))
    else:
        parts.append('            diff: "{}",')
    parts.append('            outcome: include_str!("../../🧬️schema/🧬️mutations/%s/🧪️tests/%s/🎯️outcome/🔣️.json"),' % (kind_directory, case))
    return "\n".join(parts)


def patch_subject(path, case_name, entries):
    with open(path, encoding="utf-8") as handle:
        text = handle.read()
    if "fn hall_vector_of(" in text:
        return "already patched"

    hall_arms = "\n".join('        "%s" => Vector {\n%s\n        },' % (entry["kind"], vector_arm(entry["directory"], entry["hall"], True)) for entry in entries)
    reject_arms = "\n".join(
        '        "%s" => Vector {\n%s\n        },' % (identifier_of(case), vector_arm(entry["directory"], case, os.path.exists(os.path.join(os.path.dirname(path), "..", "..", "🧬️schema", "🧬️mutations", entry["directory"], "🧪️tests", case, "🔺️diff", "🔣️.json"))))
        for entry in entries
        for case in entry["rejects"]
    )
    reject_ids = ", ".join('"%s"' % identifier_of(case) for entry in entries for case in entry["rejects"])

    tables = f'''
/// 🏭️ One kind's committed GLULAM WORKSHOP HALL vector — the second real model this artifact
/// carries (ticket `26/09/06/FEM-PLUGIN-END-TO-END`), a 12 m span two-bay GL24h portal hall on a
/// C25/30 raft slab. Same five members as [`vector`], a different structural system.
fn hall_vector_of(kind: &str) -> Vector {{
    match kind {{
{hall_arms}
        other => panic!("{case_name}: no committed hall vector is registered for kind {{other:?}}"),
    }}
}}

/// 🚫️ Every committed vector of this subset whose whole claim is that the model does NOT move — a
/// refusal, or a declared no-op. Keyed by the scenario id, not by the kind, because a kind can
/// refuse in several different ways. A rejection bundle carries no `🔺️diff/🔣️.json` at all, so the
/// `diff` member is the empty object and no handler reads it.
const REJECT_VECTORS: &[&str] = &[{reject_ids}];

fn reject_vector_of(identifier: &str) -> Vector {{
    match identifier {{
{reject_arms}
        other => panic!("{case_name}: no committed rejection vector is registered for {{other:?}}"),
    }}
}}
'''

    anchor = "//#endregion 🔖️Fixtures"
    assert anchor in text
    text = text.replace(anchor, tables.rstrip("\n") + "\n" + anchor, 1)

    handlers = f'''
    /// 🏭️ Replays one committed glulam-hall vector — the same five laws [`spec_vector`] applies, on
    /// the second real model, so no verb's evidence rests on a single structural system.
    pub fn hall_vector(kind: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {{
        move |_ctx: &Context| {{
            let committed = hall_vector_of(kind);
            let report = report_of(&format!("hall-vector-{{kind}}"), committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            if let Some(first) = law::divergence(applied, member(&report, "expectedSnapshot")?) {{
                return Err(format!("hall-vector-{{kind}}: the applied model is not the committed after-snapshot — {{first}}"));
            }}
            if let Some(first) = law::divergence(member(&report, "diff")?, &canonical(committed.diff)) {{
                return Err(format!("hall-vector-{{kind}}: the produced delta is not the committed 🔺️diff — {{first}}"));
            }}
            declared_outcome_holds(kind, &members(&report, "messages")?, &canonical(committed.outcome))?;
            law::mutation_is_observable(kind, applied, member(&report, "base")?, UNOBSERVABLE)?;
            touches_one(&format!("hall-vector-{{kind}}"), kind, member(&report, "base")?, applied)?;
            law::inverse_restores(kind, member(&report, "inverseSnapshot")?, member(&report, "base")?)?;
            Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied.clone()))
        }}
    }}

    /// 🚫️ Replays one committed vector whose claim is that NOTHING happens. The projection carries
    /// the two facts both implementations can produce — whether the request was refused, and the
    /// model it left behind. The refusal's wording is not projected: each implementation words it in
    /// its own language, and what the differential holds them to is the verdict and the document.
    pub fn reject(identifier: &'static str) -> impl Fn(&Context) -> Result<Outcome, String> {{
        move |_ctx: &Context| {{
            let committed = reject_vector_of(identifier);
            let report = report_of(&format!("reject-{{identifier}}"), committed.before, committed.mutation, committed.after)?;
            let applied = member(&report, "snapshot")?;
            let messages = members(&report, "messages")?;
            declared_outcome_holds(identifier, &messages, &canonical(committed.outcome))?;
            if let Some(first) = law::divergence(applied, member(&report, "base")?) {{
                return Err(format!("reject-{{identifier}}: a refused or no-op mutation must leave the model exactly where it was — {{first}}"));
            }}
            if let Some(first) = law::divergence(applied, member(&report, "expectedSnapshot")?) {{
                return Err(format!("reject-{{identifier}}: the committed after-model is not the committed before-model — {{first}}"));
            }}
            let refused = messages.iter().any(|message| {{ let level = message.str("level"); level == "error" || level == "fatal" }});
            let projection = Json::Object(vec![("refused".to_string(), Json::Bool(refused)), ("model".to_string(), applied.clone())]);
            Ok(Outcome::with_raw(projection.to_string().into_bytes(), projection))
        }}
    }}
'''
    handler_anchor = "    //#endregion 🔖️Handlers"
    assert handler_anchor in text
    text = text.replace(handler_anchor, handlers.rstrip("\n") + "\n" + handler_anchor, 1)

    text = text.replace("    use super::{canonical, vector, DERIVED_ASSET, UNOBSERVABLE};", "    use super::{canonical, hall_vector_of, reject_vector_of, vector, DERIVED_ASSET, UNOBSERVABLE};", 1)

    registration_old = '''            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
        }
        return built;'''
    registration_new = '''            built = built.subject(&format!("spec-vector-{kind}"), subject::spec_vector(kind));
            built = built.subject(&format!("hall-vector-{kind}"), subject::hall_vector(kind));
        }
        for identifier in REJECT_VECTORS {
            built = built.subject(&format!("reject-{identifier}"), subject::reject(identifier));
        }
        return built;'''
    assert registration_old in text
    text = text.replace(registration_old, registration_new, 1)
    text = text.replace("        let _ = (KINDS, UNOBSERVABLE, vector as fn(&str) -> Vector);", "        let _ = (KINDS, UNOBSERVABLE, REJECT_VECTORS, vector as fn(&str) -> Vector, hall_vector_of as fn(&str) -> Vector, reject_vector_of as fn(&str) -> Vector);", 1)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return "patched"


# endregion 🔖️Subject


def main():
    for subset, (case_name, reference_case) in CASES.items():
        entries = inventory(subset)
        reject_ids = [identifier_of(case) for entry in entries for case in entry["rejects"]]
        reference = os.path.join(SUBSETS, "🌐️any", "🧪️tests", reference_case, "🐍️.py")
        feature = os.path.join(SUBSETS, subset, "🧪️tests", case_name, "🥒️.feature")
        subject = os.path.join(SUBSETS, subset, "🧪️tests", case_name, "🦀️.rs")
        print("%-12s kinds=%-2d hall=%-2d reject=%-2d  reference:%-15s feature:%-15s subject:%s" % (subset, len(entries), len(entries), len(reject_ids), patch_reference(reference, reject_ids), patch_feature(feature, entries), patch_subject(subject, case_name, entries)))
    return 0


if __name__ == "__main__":
    sys.exit(main())
