#!/usr/bin/env python3
"""🧊 An INDEPENDENT second implementation of the `s.fem.fem3d` structural model's whole-document
identity, in Python, serving as this case's differential oracle. Relocated out of the artifact-level
`mutate-fem3d-1` case in ticket
`26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`. This
case carries no mutation kind, so unlike its five mutation siblings (`✳️mesh`, `✳️material`,
`✳️boundary`, `✳️load`, `✳️analysis`) it implements no `apply_mutation`/`inverse_mutation` — only
the model shape needed to validate the derived real frame and cross-reference every entity it holds.

**What it was written from.** ``../../🧬️schema/📸️snapshot/🔣️.json`` — `Fem3dSnapshot` is
exactly nine members, `additionalProperties: false`.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half only.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis")
"""🗂️ The nine members `Fem3dSnapshot` declares."""

RECORDS = {
    "nodes": {"id", "x", "y", "z"},
    "materials": {"id", "name", "e", "g", "nu", "rho"},
    "sections": {"id", "name", "area", "iy", "iz", "j"},
    "solids": {"id", "name", "outline", "holes", "baseZ", "height", "layers", "meshSize", "materialId"},
    "supports": {"id", "nodeId", "fixed"},
    "loadCases": {"id", "name", "loads", "selfWeight"},
    "combinations": {"id", "name", "terms"},
}
"""🧱️ The members each record carries, as the committed vectors spell them. `elements` is absent
here on purpose — its shape depends on the element kind, see [`ELEMENTS`]."""

ELEMENTS = {"bar": {"kind", "id", "start", "end", "materialId", "sectionId"}, "frame": {"kind", "id", "start", "end", "materialId", "sectionId", "roll"}}
"""🧩️ The two element variants: a `frame` carries a `roll` about its own axis, a `bar` does not."""

LOADS = {"nodal": {"kind", "id", "nodeId", "dof", "value"}, "memberUdl": {"kind", "id", "elementId", "wx", "wy", "wz"}, "area": {"kind", "id", "solidId", "pressure"}}
"""🏋️ The three load variants, as the committed vectors spell them."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the model to the shape the committed vectors agree on, and to id uniqueness within
    every collection — including load ids within one case."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a fem3d model must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    if set(document["analysis"]) != {"modalCount", "bucklingCount", "deformationScale"}:
        raise AssertionError("analysis must carry exactly the three declared settings, found %r" % sorted(document["analysis"]))
    identifiers = []
    for element in document["elements"]:
        if element.get("kind") not in ELEMENTS or set(element) != ELEMENTS[element["kind"]]:
            raise AssertionError("an element must be a bar or a frame with exactly its declared members, found %r" % element)
        identifiers.append(element["id"])
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("elements carries a duplicate id: %r" % identifiers)
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("a %s record must carry exactly %r, found %r" % (name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s carries a duplicate id: %r" % (name, identifiers))
    for combination in document["combinations"]:
        if not isinstance(combination["terms"], dict):
            raise AssertionError("combination %r carries a case-keyed term MAP, found %r" % (combination["id"], combination["terms"]))
    for case in document["loadCases"]:
        loads = []
        for load in case["loads"]:
            if load.get("kind") not in LOADS or set(load) != LOADS[load["kind"]]:
                raise AssertionError("load %r of case %r is not one of the three declared variants" % (load, case["id"]))
            loads.append(load["id"])
        if len(set(loads)) != len(loads):
            raise AssertionError("case %r carries a duplicate load id: %r" % (case["id"], loads))


def document_of(payload):
    """📥️ Reads a fem3d model out of a snapshot JSON value."""
    document = copy.deepcopy(payload)
    validate(document)
    return document


# endregion 🔖️Document


# region 🔖️Plan
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
def identity_handler(ctx):
    """🔁️ Reads the derived real frame and answers with the whole model.

    This implementation additionally requires, in role, that the model really is the committed
    frame: four ground corners with two storeys above, every element bound to a material and a
    section the model holds, every support and every load bound to something that exists, and every
    combination term naming a real case. A codec that dropped a collection could not satisfy it.
    """
    document = document_of(json_fixture(ctx, "steel-frame"))
    nodes = {node["id"]: node for node in document["nodes"]}
    materials = {record["id"] for record in document["materials"]}
    sections = {record["id"] for record in document["sections"]}
    solids = {record["id"] for record in document["solids"]}
    cases = {record["id"] for record in document["loadCases"]}
    for corner in ("n00", "n20", "n02", "n22"):
        ground, upper = nodes.get("%s_g" % corner), nodes.get("%s_l2" % corner)
        if ground is None or upper is None or ground["z"] != 0.0 or upper["z"] <= ground["z"]:
            raise AssertionError("identity-round-trip: the committed frame stands on four ground corners with two storeys above, %r is not one" % corner)
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
            target = load.get("nodeId") or load.get("solidId") or load.get("elementId")
            pool = nodes if "nodeId" in load else solids if "solidId" in load else {element["id"] for element in document["elements"]}
            if target not in pool:
                raise AssertionError("identity-round-trip: load %r of case %r names %r, which is not in the model" % (load["id"], case["id"], target))
    for combination in document["combinations"]:
        for case_id in combination["terms"]:
            if case_id not in cases:
                raise AssertionError("identity-round-trip: combination %r names case %r, which is not in the model" % (combination["id"], case_id))
    reread = document_of(json.loads(json.dumps(document)))
    if reread != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the model moved it")
    return outcome_of(document)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration in the ORACLE role only — registering this handler as a subject too would make
    the reference its own subject and manufacture a green self-comparison."""
    built = Adapter("python")
    return built.oracle("identity-round-trip", identity_handler)


# endregion 🔖️Registration
