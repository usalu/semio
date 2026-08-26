#!/usr/bin/env python3
"""🏗️ An INDEPENDENT second implementation of the `s.fem.fem2d` structural model and all twenty-five
of its typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** What this vocabulary edits is the
MODEL, not the analysis: nine id-keyed collections — nodes, elements, regions, materials, sections,
supports, load cases with their nested loads, combinations, and one settings record. A finite-element
solver (`code_aster`, `OpenSees`, `anastruct`, `PyNite`) computes displacements and forces from a
model; none of them reads `.dsl.semio`, none defines this document, and not one of the twenty-five
kinds asks a solver anything — `replace-section` swaps a profile record, `add-load` appends to a
case's load list, `change-load-case-self-weight` flips a boolean. What a reference genuinely can
adjudicate is the model algebra, and that is what this file implements, from the specification, in
another language.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — `Fem2dSnapshot` is
  exactly those nine members, `additionalProperties: false`, with `FemAnalysisSettings` spelled out
  as `{modalCount, bucklingCount, deformationScale}`.
* the twenty-five committed `(before, mutation, after, outcome)` specification vectors — which is
  where the RECORD shapes and the cascade rules are actually written down, and where each kind's
  internally tagged wire form is given.
* ``…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`` — the twenty-five verbs.

**A defect in the specification, found while writing this and reported rather than worked around.**
Two of the three schema files do not say what they claim. `…/🧬️schema/🧬️mutations/🔣️component.json`
is a verbatim copy of the SNAPSHOT schema with `title` changed to `Fem2dMutation`; and in the
snapshot schema itself every one of `FemNode`, `FemElement`, `FemRegion`, `FemMaterial`,
`FemSection`, `FemSupport`, `FemLoadCase` and `FemCombination` is an EMPTY
`{"title": …, "type": "object"}` with no properties at all. The record shapes below were therefore
read off the twenty-five committed vectors, which agree with one another on every field.

**What the committed vectors settle about cascades, and this implementation follows.** Their own
names are the statement: `removes-node-n3-without-cascading-to-its-support`,
`removes-the-slab-and-keeps-its-material`, `removes-bar-e2-and-keeps-its-end-nodes`,
`removes-the-uls-combination-and-keeps-both-cases`,
`removes-the-live-case-together-with-its-loads`. A delete removes exactly its own entity; only a load
case takes its nested loads with it, because they are nested inside it.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis")
"""🗂️ The nine members `Fem2dSnapshot` declares — and the cross-language projection."""

COLLECTIONS = {
    "node": ("nodes", "node", "newNode"),
    "element": ("elements", "element", "newElement"),
    "region": ("regions", "region", "newRegion"),
    "material": ("materials", "material", "newMaterial"),
    "section": ("sections", "section", "newSection"),
    "support": ("supports", "support", "newSupport"),
    "load-case": ("loadCases", "loadCase", None),
    "combination": ("combinations", "combination", None),
}
"""🗂️ Per noun: its collection, the argument `create-` carries, and the one `replace-` carries when
the vocabulary has a `replace-` for it at all — `node`, `load-case` and `combination` have none."""

REPLACEABLE = ("element", "region", "material", "section", "support")
"""♻️ The five nouns the catalog gives a `replace-` verb."""

KINDS = (
    "create-node",
    "delete-node",
    "create-element",
    "delete-element",
    "replace-element",
    "create-material",
    "delete-material",
    "replace-material",
    "create-section",
    "delete-section",
    "replace-section",
    "create-support",
    "delete-support",
    "replace-support",
    "create-region",
    "delete-region",
    "replace-region",
    "create-load-case",
    "delete-load-case",
    "add-load",
    "remove-load",
    "change-load-case-self-weight",
    "create-combination",
    "delete-combination",
    "update-analysis-settings",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

RECORDS = {
    "nodes": {"id", "x", "y"},
    "elements": {"kind", "id", "start", "end", "materialId", "sectionId"},
    "regions": {"id", "name", "outline", "holes", "thickness", "materialId", "meshSize"},
    "materials": {"id", "name", "e", "nu", "rho"},
    "sections": {"id", "name", "area", "iy"},
    "supports": {"id", "nodeId", "fixed"},
    "loadCases": {"id", "name", "loads", "selfWeight"},
    "combinations": {"id", "name", "terms"},
}
"""🧱️ The members each record carries, as the twenty-five committed vectors spell them."""

LOADS = {"nodal": {"kind", "id", "nodeId", "dof", "value"}, "memberUdl": {"kind", "id", "elementId", "wx", "wy"}, "area": {"kind", "id", "regionId", "pressure"}}
"""🏋️ The three load variants, as the committed `add-load` and load-case vectors spell them."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the model to the shape the twenty-five committed vectors agree on, and to id
    uniqueness within every collection — including load ids within one case."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a fem2d model must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    if set(document["analysis"]) != {"modalCount", "bucklingCount", "deformationScale"}:
        raise AssertionError("analysis must carry exactly the three declared settings, found %r" % sorted(document["analysis"]))
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("a %s record must carry exactly %r, found %r" % (name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s carries a duplicate id: %r" % (name, identifiers))
    for case in document["loadCases"]:
        loads = []
        for load in case["loads"]:
            if load.get("kind") not in LOADS or set(load) != LOADS[load["kind"]]:
                raise AssertionError("load %r of case %r is not one of the three declared variants" % (load, case["id"]))
            loads.append(load["id"])
        if len(set(loads)) != len(loads):
            raise AssertionError("case %r carries a duplicate load id: %r" % (case["id"], loads))


def document_of(payload):
    """📥️ Reads a fem2d model out of a snapshot JSON value."""
    document = copy.deepcopy(payload)
    validate(document)
    return document


def find(items, identifier):
    """🔎️ The index of an id in a collection, or `None`."""
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def noun_of(kind):
    """🏷️ The noun a `create-`/`delete-`/`replace-` kind names."""
    return kind.split("-", 1)[1]


# endregion 🔖️Mutations


# region 🔖️Mutations
def kind_of(mutation):
    """🏷️ The kind an internally tagged mutation payload names."""
    if not isinstance(mutation, dict) or "mutation" not in mutation:
        raise AssertionError("a mutation carries an internally tagged `mutation` member, found %r" % mutation)
    for kind, tag in TAGS.items():
        if tag == mutation["mutation"]:
            return kind
    raise AssertionError("unknown mutation variant %r" % mutation["mutation"])


def case_of(document, identifier, kind):
    """📋️ One load case, or a rejection — a mutation that addressed nothing is never a silent no-op."""
    at = find(document["loadCases"], identifier)
    if at is None:
        raise AssertionError("%s: no load case %r in the model" % (kind, identifier))
    return document["loadCases"][at]


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting model."""
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind == "update-analysis-settings":
        result["analysis"] = copy.deepcopy(mutation["settings"])
    elif kind == "add-load":
        case = case_of(result, mutation["caseId"], kind)
        load = copy.deepcopy(mutation["load"])
        if find(case["loads"], load["id"]) is not None:
            raise AssertionError("%s: case %r already carries a load %r" % (kind, case["id"], load["id"]))
        case["loads"].append(load)
    elif kind == "remove-load":
        case = case_of(result, mutation["caseId"], kind)
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("%s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        case["loads"].pop(at)
    elif kind == "change-load-case-self-weight":
        case_of(result, mutation["caseId"], kind)["selfWeight"] = mutation["newSelfWeight"]
    else:
        noun = noun_of(kind)
        collection, create_argument, replace_argument = COLLECTIONS[noun]
        items = result[collection]
        if kind.startswith("create-"):
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
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the model it applies to.

    Note what the vocabulary can and cannot express: no `create-` verb carries an index, so the
    inverse of a delete is exact only for a TRAILING record — the feature's rows are chosen
    accordingly and say so.
    """
    kind = kind_of(mutation)
    if kind == "update-analysis-settings":
        return {"mutation": TAGS[kind], "settings": copy.deepcopy(document["analysis"])}
    if kind == "add-load":
        return {"mutation": TAGS["remove-load"], "caseId": mutation["caseId"], "loadId": mutation["load"]["id"]}
    if kind == "remove-load":
        case = case_of(document, mutation["caseId"], "inverse of %s" % kind)
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("inverse of %s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        return {"mutation": TAGS["add-load"], "caseId": mutation["caseId"], "load": copy.deepcopy(case["loads"][at])}
    if kind == "change-load-case-self-weight":
        return {"mutation": TAGS[kind], "caseId": mutation["caseId"], "newSelfWeight": case_of(document, mutation["caseId"], "inverse of %s" % kind)["selfWeight"]}
    noun = noun_of(kind)
    collection, create_argument, replace_argument = COLLECTIONS[noun]
    if kind.startswith("create-"):
        return {"mutation": TAGS["delete-%s" % noun], "id": mutation[create_argument]["id"]}
    at = find(document[collection], mutation["id"])
    if at is None:
        raise AssertionError("inverse of %s: %r is not in %s" % (kind, mutation["id"], collection))
    held = copy.deepcopy(document[collection][at])
    if kind.startswith("delete-"):
        return {"mutation": TAGS["create-%s" % noun], create_argument: held}
    return {"mutation": TAGS[kind], "id": mutation["id"], replace_argument: held}


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(scenario, before, after):
    """👁️ Every row below moves the model, so a forward application must move it. A mutation that
    quietly did nothing would otherwise agree with an unchanged model and report a pass."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the model untouched, so nothing was proved" % scenario)


def touches_one(scenario, kind, before, after):
    """🔀️ Each verb writes exactly ONE of the nine members. That is the check an after-snapshot
    comparison cannot make on its own: an implementation that re-derived a sibling collection on
    every edit — renumbering ids, re-sorting sections — would still land on the right value for the
    member it meant to write."""
    if kind == "update-analysis-settings":
        written = "analysis"
    elif kind in ("add-load", "remove-load", "change-load-case-self-weight"):
        written = "loadCases"
    else:
        written = COLLECTIONS[noun_of(kind)][0]
    moved = [name for name in MEMBERS if before[name] != after[name]]
    if moved != [written]:
        raise AssertionError("%s: this verb writes %s and nothing else, but %r moved" % (scenario, written, moved))


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the member and index that failed to come back."""
    if restored == original:
        return
    for name in MEMBERS:
        if restored[name] == original[name]:
            continue
        if name == "analysis":
            raise AssertionError("inverse-%s: analysis came back as %r, not %r" % (kind, restored[name], original[name]))
        was = [record["id"] for record in original[name]]
        now = [record["id"] for record in restored[name]]
        if was != now:
            raise AssertionError("inverse-%s: %s came back as %r, not %r" % (kind, name, now, was))
        for at, (left, right) in enumerate(zip(original[name], restored[name])):
            if left != right:
                raise AssertionError("inverse-%s: %s[%d] (%s) came back as %s, not %s" % (kind, name, at, left["id"], json.dumps(right, sort_keys=True)[:200], json.dumps(left, sort_keys=True)[:200]))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member."""
    for name in MEMBERS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-snapshot says %s" % (kind, name, json.dumps(produced[name], sort_keys=True)[:300], json.dumps(committed[name], sort_keys=True)[:300]))


# endregion 🔖️Laws


# region 🔖️Plan
def doc_string(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to the real derived timber portal frame."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "timber-portal-frame"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("mutate-%s" % kind, document, applied)
        touches_one("mutate-%s" % kind, kind, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived frame and then its OWN computed inverse.

    The projection carries BOTH models; projecting only the restored one would make all
    twenty-five rows project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "timber-portal-frame"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("inverse-%s" % kind, document, applied)
        restored = apply_mutation(applied, inverse_mutation(document, mutation))
        restores(kind, restored, document)
        return outcome_of({"mutated": applied, "restored": restored})

    return handler


def spec_vector_handler(kind):
    """📐️ Replays the committed handcrafted `(before, mutation, after)` triple for one kind."""

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        if kind_of(mutation) != kind:
            raise AssertionError("spec-vector-%s: the committed vector carries a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        observable("spec-vector-%s" % kind, before, applied)
        touches_one("spec-vector-%s" % kind, kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the derived real frame and answers with the whole model.

    This implementation additionally requires, in role, that the model really is the committed frame:
    a ridge above both eaves, every element bound to a material and a section the model holds, every
    support and every load bound to something that exists, and every combination term naming a real
    case. A codec that dropped a collection could not satisfy it.
    """
    document = document_of(json_fixture(ctx, "timber-portal-frame"))
    nodes = {node["id"]: node for node in document["nodes"]}
    materials = {record["id"] for record in document["materials"]}
    sections = {record["id"] for record in document["sections"]}
    regions = {record["id"] for record in document["regions"]}
    cases = {record["id"] for record in document["loadCases"]}
    if "ridge" not in nodes or nodes["ridge"]["y"] <= max(nodes["p0_l2"]["y"], nodes["p8_l2"]["y"]):
        raise AssertionError("identity-round-trip: the committed frame has its ridge above both eaves, found %r" % nodes.get("ridge"))
    for element in document["elements"]:
        for end in ("start", "end"):
            if element[end] not in nodes:
                raise AssertionError("identity-round-trip: element %r names %s %r, which is not a node" % (element["id"], end, element[end]))
        if element["materialId"] not in materials or element["sectionId"] not in sections:
            raise AssertionError("identity-round-trip: element %r names a material or section the model does not hold" % element["id"])
    for support in document["supports"]:
        if support["nodeId"] not in nodes:
            raise AssertionError("identity-round-trip: support %r names node %r, which is not in the model" % (support["id"], support["nodeId"]))
    for case in document["loadCases"]:
        for load in case["loads"]:
            target = load.get("nodeId") or load.get("regionId") or load.get("elementId")
            pool = nodes if "nodeId" in load else regions if "regionId" in load else {element["id"] for element in document["elements"]}
            if target not in pool:
                raise AssertionError("identity-round-trip: load %r of case %r names %r, which is not in the model" % (load["id"], case["id"], target))
    for combination in document["combinations"]:
        for term in combination["terms"]:
            if term["caseId"] not in cases:
                raise AssertionError("identity-round-trip: combination %r names case %r, which is not in the model" % (combination["id"], term["caseId"]))
    reread = document_of(json.loads(json.dumps(document)))
    if reread != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the model moved it")
    return outcome_of(document)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
        built = built.oracle("spec-vector-%s" % kind, spec_vector_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)


# endregion 🔖️Registration
