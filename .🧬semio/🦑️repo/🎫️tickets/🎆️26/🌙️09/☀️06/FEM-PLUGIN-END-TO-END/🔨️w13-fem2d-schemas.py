#!/usr/bin/env python3
"""🧬️ W13 — regenerates the fem2d JSON-Schema surface from one transcription of the Rust shapes.

Two files families are written:

* `🪆️subsets/<subset>/🧬️schema/🧬️mutations/<kind>/🧬️.schema.json` — the per-kind PAYLOAD schema.
  Before this wave every one of them was wrong in the same two ways (W10 finding F7 / W11 finding 8):
  the internally tagged `mutation` discriminator the wire actually carries was missing and declared
  `additionalProperties: false`, so **all 75** committed payloads failed their own schema; and the
  four enum-carrying kinds spelled `FemElement`/`FemLoad` variants as bare PascalCase Rust names
  (`"Bar"`, `"MemberUdl"`) with `kind` as their ONLY property, while `#[value(rename_all =
  "camelCase")]` puts `"bar"`/`"memberUdl"` and the variant's real fields on the wire.
* `🪆️subsets/🌐️any/🧬️schema/📸️snapshot/🔣️.json` — the DOCUMENT schema, whose nine record `$defs`
  were empty stubs, so snapshot validation passed vacuously.

Division of labour between the two, and it is deliberate: a payload schema says what a well-formed
PAYLOAD looks like — shape and types only — because a payload carrying a negative Young's modulus is
perfectly well formed and is refused by the document's own invariant, not by its syntax. The
SNAPSHOT schema is where the plausibility and geometry bounds live, because no valid fem2d document
may contain them. Each guarded field carries an `x-semio-invariant` annotation naming the guard in
`mutations::guards` that enforces it, so the two halves stay findable from each other.

Usage:
    uv run python 🔨️w13-fem2d-schemas.py            # write
    uv run python 🔨️w13-fem2d-schemas.py --check    # report differences, write nothing
"""

# region 🔖️Imports
import json
import os
import sys

# endregion 🔖️Imports


# region 🔖️Paths
REPO = os.path.abspath(os.path.join(os.path.dirname(__file__), *([".."] * 7)))
SUBSETS = os.path.join(REPO, "✏️s", "🔌️plugins", "🏗️fem", "🗿️artifacts", "◻️2d", "🏅️standards", "🔖️1", "🪆️subsets")

KIND_DIR = {
    "create-node": ("🕸️mesh", "⚪️create-node"),
    "delete-node": ("🕸️mesh", "🕳️delete-node"),
    "create-element": ("🕸️mesh", "🧩️create-element"),
    "delete-element": ("🕸️mesh", "🗑️delete-element"),
    "replace-element": ("🕸️mesh", "♻️replace-element"),
    "create-material": ("🧱️material", "🌱️create-material"),
    "delete-material": ("🧱️material", "🗑️delete-material"),
    "replace-material": ("🧱️material", "🔁️replace-material"),
    "create-section": ("🕸️mesh", "📐️create-section"),
    "delete-section": ("🕸️mesh", "✂️delete-section"),
    "replace-section": ("🕸️mesh", "📏️replace-section"),
    "create-support": ("🛡️boundary", "🛡️create-support"),
    "delete-support": ("🛡️boundary", "🗑️delete-support"),
    "replace-support": ("🛡️boundary", "🔁️replace-support"),
    "create-region": ("🕸️mesh", "🗺️create-region"),
    "delete-region": ("🕸️mesh", "🚫️delete-region"),
    "replace-region": ("🕸️mesh", "🔄️replace-region"),
    "create-load-case": ("🏋️load", "📋️create-load-case"),
    "delete-load-case": ("🏋️load", "🗑️delete-load-case"),
    "add-load": ("🏋️load", "➕️add-load"),
    "remove-load": ("🏋️load", "➖️remove-load"),
    "change-load-case-self-weight": ("🏋️load", "⚖️change-load-case-self-weight"),
    "create-combination": ("🏋️load", "🔗️create-combination"),
    "delete-combination": ("🏋️load", "✂️delete-combination"),
    "update-analysis-settings": ("📈️analysis", "🎛️update-analysis-settings"),
}
"""🗂️ Every kind's owning subset and directory — the fem2d half of the vocabulary, all 25."""


def tag_of(kind):
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


def title_of(kind):
    return "".join(word[:1].upper() + word[1:] for word in kind.split("-"))


# endregion 🔖️Paths


# region 🔖️Records
STRING = {"type": "string"}
NUMBER = {"type": "number", "format": "double"}
POINT = {"type": "array", "items": {"type": "number", "format": "double"}, "minItems": 2, "maxItems": 2}
DOF = {"type": "string", "enum": ["Tx", "Ty", "Tz", "Rx", "Ry", "Rz"]}


def record(title, properties, invariants=None):
    """🧱️ One closed record — every declared field required, nothing else admitted."""
    body = {"title": title, "type": "object", "additionalProperties": False, "required": list(properties), "properties": properties}
    if invariants:
        body["x-semio-invariant"] = invariants
    return body


def bounded(base, invariant, **bounds):
    """📏️ A numeric field carrying its own document-level bound and the guard that enforces it."""
    field = dict(base)
    field.update(bounds)
    field["x-semio-invariant"] = invariant
    return field


def variant(tag, properties):
    """🔀️ One internally tagged enum variant — `kind` plus the variant's own fields."""
    fields = {"kind": {"const": tag}}
    fields.update(properties)
    return {"type": "object", "additionalProperties": False, "required": list(fields), "properties": fields}


def fem_node(bounded_document):
    finite = "mutations::guards::node_geometry — a node coordinate is finite" if bounded_document else None
    coordinate = bounded(NUMBER, finite) if bounded_document else NUMBER
    return record("FemNode", {"id": STRING, "x": coordinate, "y": coordinate})


def fem_element(_bounded_document=False):
    ends = {"id": STRING, "start": STRING, "end": STRING, "materialId": STRING, "sectionId": STRING}
    return {"title": "FemElement", "oneOf": [variant("bar", ends), variant("beam", ends)]}


def fem_material(bounded_document):
    if not bounded_document:
        return record("FemMaterial", {"id": STRING, "name": STRING, "e": NUMBER, "nu": NUMBER, "rho": NUMBER})
    guard = "mutations::guards::material_plausibility"
    return record(
        "FemMaterial",
        {
            "id": STRING,
            "name": STRING,
            "e": bounded(NUMBER, "%s — a positive Young's modulus" % guard, exclusiveMinimum=0.0),
            "nu": bounded(NUMBER, "%s — a Poisson ratio in the admissible open interval" % guard, exclusiveMinimum=-1.0, exclusiveMaximum=0.5),
            "rho": bounded(NUMBER, "%s — a positive density" % guard, exclusiveMinimum=0.0),
        },
    )


def fem_section(bounded_document):
    if not bounded_document:
        return record("FemSection", {"id": STRING, "name": STRING, "area": NUMBER, "iy": NUMBER})
    guard = "mutations::guards::section_plausibility"
    return record(
        "FemSection",
        {"id": STRING, "name": STRING, "area": bounded(NUMBER, "%s — a positive area" % guard, exclusiveMinimum=0.0), "iy": bounded(NUMBER, "%s — a positive second moment of area" % guard, exclusiveMinimum=0.0)},
    )


def fem_region(bounded_document):
    outline = {"type": "array", "items": POINT}
    holes = {"type": "array", "items": {"type": "array", "items": POINT}}
    if not bounded_document:
        return record("FemRegion", {"id": STRING, "name": STRING, "outline": outline, "holes": holes, "thickness": NUMBER, "materialId": STRING, "meshSize": NUMBER})
    guard = "mutations::guards::region_geometry"
    outline = dict(outline, minItems=3, **{"x-semio-invariant": "%s — an outline of at least three points enclosing non-zero area" % guard})
    holes = dict(holes, items={"type": "array", "items": POINT, "minItems": 3}, **{"x-semio-invariant": "%s — every hole a real polygon lying inside the outline" % guard})
    return record(
        "FemRegion",
        {
            "id": STRING,
            "name": STRING,
            "outline": outline,
            "holes": holes,
            "thickness": bounded(NUMBER, "%s — a positive thickness" % guard, exclusiveMinimum=0.0),
            "materialId": STRING,
            "meshSize": bounded(NUMBER, "%s — a positive mesh size" % guard, exclusiveMinimum=0.0),
        },
    )


def fem_support(_bounded_document=False):
    return record("FemSupport", {"id": STRING, "nodeId": STRING, "fixed": {"type": "array", "items": DOF}})


def fem_load(_bounded_document=False):
    return {
        "title": "FemLoad",
        "oneOf": [
            variant("nodal", {"id": STRING, "nodeId": STRING, "dof": DOF, "value": NUMBER}),
            variant("memberUdl", {"id": STRING, "elementId": STRING, "wx": NUMBER, "wy": NUMBER}),
            variant("area", {"id": STRING, "regionId": STRING, "pressure": NUMBER}),
        ],
    }


def fem_load_case(bounded_document):
    return record("FemLoadCase", {"id": STRING, "name": STRING, "loads": {"type": "array", "items": fem_load(bounded_document)}, "selfWeight": {"type": "boolean"}})


def fem_combination(_bounded_document=False):
    term = record("FemCombinationTerm", {"caseId": STRING, "factor": NUMBER})
    return record("FemCombination", {"id": STRING, "name": STRING, "terms": {"type": "array", "items": term}})


def fem_analysis_settings(bounded_document):
    count = {"type": "integer", "format": "uint32", "minimum": 0}
    if not bounded_document:
        return record("FemAnalysisSettings", {"modalCount": count, "bucklingCount": count, "deformationScale": NUMBER})
    guard = "mutations::guards::analysis_bounds"
    return record(
        "FemAnalysisSettings",
        {
            "modalCount": bounded(count, "%s — at least one modal mode" % guard, minimum=1),
            "bucklingCount": bounded(count, "%s — at least one buckling mode" % guard, minimum=1),
            "deformationScale": bounded(NUMBER, "%s — a positive deformation exaggeration" % guard, exclusiveMinimum=0.0),
        },
    )


def fem_camera(_bounded_document=False):
    return record("FemCamera", {"x": NUMBER, "y": NUMBER, "zoom": NUMBER})


# endregion 🔖️Records


# region 🔖️Payloads
ARGUMENT = {
    "create-node": ("node", fem_node),
    "create-element": ("element", fem_element),
    "create-material": ("material", fem_material),
    "create-section": ("section", fem_section),
    "create-support": ("support", fem_support),
    "create-region": ("region", fem_region),
    "create-load-case": ("loadCase", fem_load_case),
    "create-combination": ("combination", fem_combination),
    "replace-element": ("newElement", fem_element),
    "replace-material": ("newMaterial", fem_material),
    "replace-section": ("newSection", fem_section),
    "replace-support": ("newSupport", fem_support),
    "replace-region": ("newRegion", fem_region),
}
"""🎒️ The record argument each `create-`/`replace-` verb carries, and the shape it carries."""


def payload_properties(kind):
    """🧾️ One kind's payload properties, tag first, in the Rust struct's own field order."""
    fields = {"mutation": {"const": tag_of(kind), "description": "The internally tagged discriminator `#[value(tag = \"mutation\")]` puts on the wire."}}
    if kind in ARGUMENT and kind.startswith("create-"):
        name, shape = ARGUMENT[kind]
        fields[name] = shape(False)
    elif kind in ARGUMENT:
        name, shape = ARGUMENT[kind]
        fields["id"] = STRING
        fields[name] = shape(False)
    elif kind.startswith("delete-"):
        fields["id"] = STRING
    elif kind == "add-load":
        fields["caseId"] = STRING
        fields["load"] = fem_load(False)
    elif kind == "remove-load":
        fields["caseId"] = STRING
        fields["loadId"] = STRING
    elif kind == "change-load-case-self-weight":
        fields["caseId"] = STRING
        fields["newSelfWeight"] = {"type": "boolean"}
    elif kind == "update-analysis-settings":
        fields["settings"] = fem_analysis_settings(False)
    else:
        raise AssertionError("no payload shape declared for %r" % kind)
    return fields


def payload_schema(kind):
    """🧬️ One kind's complete payload schema — the wire shape, tag included."""
    fields = payload_properties(kind)
    return {
        "$schema": "http://json-schema.org/draft-07/schema#",
        "title": title_of(kind),
        "description": "The `%s` mutation payload as it travels the wire: internally tagged with `\"mutation\": \"%s\"`, camelCase throughout. Structural only — the plausibility and geometry bounds this kind enforces live in the SNAPSHOT schema, because they are properties of a valid document rather than of a well-formed payload; each is annotated there with the `mutations::guards` function that raises them." % (kind, tag_of(kind)),
        "type": "object",
        "additionalProperties": False,
        "required": list(fields),
        "properties": fields,
    }


def mutation_schema():
    """🧬️ The aggregate `Fem2dMutation` schema — the closed 25-variant tagged union.

    Before this wave the committed file was a verbatim copy of the SNAPSHOT schema with `title`
    changed to `Fem2dMutation`, so it demanded that a mutation carry `nodes`/`elements`/… : it
    described the document, not the vocabulary, and every real payload failed it. Both fem2d and
    fem3d shipped that copy, and both oracle catalogs report it as a defect in their rationale.
    """
    defs = {}
    for kind in sorted(KIND_DIR):
        fields = payload_properties(kind)
        defs[title_of(kind)] = {"title": title_of(kind), "type": "object", "additionalProperties": False, "required": list(fields), "properties": fields}
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://semio.tech/schema/s/fem/fem2d/mutation.json",
        "title": "Fem2dMutation",
        "description": "The closed semantic mutation vocabulary of the fem2d document: exactly twenty-five variants, discriminated by the internally tagged `mutation` member. Each variant is the same payload shape the kind's own `<kind>/🧬️.schema.json` declares.",
        "oneOf": [{"$ref": "#/$defs/%s" % title_of(kind)} for kind in sorted(KIND_DIR)],
        "$defs": defs,
    }


# endregion 🔖️Payloads


# region 🔖️Snapshot
SNAPSHOT_MEMBERS = [
    ("nodes", "FemNode"),
    ("elements", "FemElement"),
    ("regions", "FemRegion"),
    ("materials", "FemMaterial"),
    ("sections", "FemSection"),
    ("supports", "FemSupport"),
    ("loadCases", "FemLoadCase"),
    ("combinations", "FemCombination"),
]

DEFS = {
    "FemCamera": fem_camera,
    "FemAnalysisSettings": fem_analysis_settings,
    "FemNode": fem_node,
    "FemElement": fem_element,
    "FemRegion": fem_region,
    "FemMaterial": fem_material,
    "FemSection": fem_section,
    "FemSupport": fem_support,
    "FemLoadCase": fem_load_case,
    "FemCombination": fem_combination,
}
"""🗂️ The `$defs` the snapshot schema declares, in its own committed order."""


def snapshot_schema():
    """📸️ The document schema — nine members, and record `$defs` that finally say something."""
    properties = {}
    for member, definition in SNAPSHOT_MEMBERS:
        properties[member] = {"type": "array", "items": {"$ref": "#/$defs/%s" % definition}, "x-semio-state": "artifact"}
    properties["analysis"] = {"$ref": "#/$defs/FemAnalysisSettings", "x-semio-state": "artifact"}
    return {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "$id": "https://semio.tech/schema/s/fem/fem2d/snapshot.json",
        "title": "Fem2dSnapshot",
        "description": "A fem2d structural document: eight id-keyed collections and one inseparable analysis-settings facet. Every record `$def` below carries the fields the Rust struct declares AND, where the vocabulary enforces one, the bound that makes the value admissible — annotated with the `mutations::guards` function that raises it, so a reader can get from the document invariant to the code that keeps it.",
        "type": "object",
        "additionalProperties": False,
        "required": [member for member, _ in SNAPSHOT_MEMBERS] + ["analysis"],
        "properties": properties,
        "$defs": {name: shape(True) for name, shape in DEFS.items()},
    }


# endregion 🔖️Snapshot


# region 🔖️Descriptors
OUTCOME_CLASSES = {
    "create-node": ["applied", "fatal"],
    "delete-node": ["applied", "error"],
    "create-element": ["applied", "error", "fatal"],
    "delete-element": ["applied", "error"],
    "replace-element": ["applied", "warning", "error", "fatal"],
    "create-material": ["applied", "fatal"],
    "delete-material": ["applied", "error"],
    "replace-material": ["applied", "warning", "error", "fatal"],
    "create-section": ["applied", "fatal"],
    "delete-section": ["applied", "error"],
    "replace-section": ["applied", "warning", "error", "fatal"],
    "create-support": ["applied", "error", "fatal"],
    "delete-support": ["applied", "error"],
    "replace-support": ["applied", "warning", "error", "fatal"],
    "create-region": ["applied", "error", "fatal"],
    "delete-region": ["applied", "error"],
    "replace-region": ["applied", "warning", "error", "fatal"],
    "create-load-case": ["applied", "error", "fatal"],
    "delete-load-case": ["applied", "error"],
    "add-load": ["applied", "warning", "error"],
    "remove-load": ["applied", "error"],
    "change-load-case-self-weight": ["applied", "warning", "error"],
    "create-combination": ["applied", "error", "fatal"],
    "delete-combination": ["applied", "error"],
    "update-analysis-settings": ["applied", "warning", "fatal"],
}
"""🎯️ Every outcome class each kind's diff builder can actually reach, after this wave.

The committed values were wrong in both directions and for a traceable reason: they were produced
by the v2-manifest scaffolder in
`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts`, which scans a
`🔺️diff/🦀️.rs` for `MutationOutcome::error`, `::empty` and `::new` and has no case for `::fatal`
or for the chained `.warn(…)` — so `create-node`, `create-section` and `create-material` were
recorded as `["applied"]` although each raises a Fatal `mutation.duplicate-id`, and every no-op was
recorded as `info` although it is raised at `Warning`. Both spellings are in the derive's own enum
(`✨️derive/🦀️.rs:625`) and in `outcomeClassesOf`, which maps `warning`→applied and
`fatal`→rejected, so declaring them is the truthful thing and nothing downstream has to change.
"""


def patch_descriptors(check):
    """🪪️ Rewrites only the `outcomeClasses` member of each kind's leaf descriptor."""
    tally = {}
    for kind, (subset, directory) in sorted(KIND_DIR.items()):
        path = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", directory, "🔣️.json")
        descriptor = json.load(open(path, encoding="utf-8"))
        descriptor["outcomeClasses"] = OUTCOME_CLASSES[kind]
        state = write(path, descriptor, check)
        tally[state] = tally.get(state, 0) + 1
    return tally


# endregion 🔖️Descriptors


# region 🔖️Emit
def write(path, payload, check):
    text = json.dumps(payload, indent=2, ensure_ascii=False) + "\n"
    existing = open(path, encoding="utf-8").read() if os.path.exists(path) else None
    if existing == text:
        return "same"
    if check:
        return "differs"
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)
    return "written"


def main():
    check = "--check" in sys.argv[1:]
    tally = {}
    for kind, (subset, directory) in sorted(KIND_DIR.items()):
        path = os.path.join(SUBSETS, subset, "🧬️schema", "🧬️mutations", directory, "🧬️.schema.json")
        assert os.path.isdir(os.path.dirname(path)), path
        state = write(path, payload_schema(kind), check)
        tally[state] = tally.get(state, 0) + 1
    state = write(os.path.join(SUBSETS, "🌐️any", "🧬️schema", "📸️snapshot", "🔣️.json"), snapshot_schema(), check)
    tally[state] = tally.get(state, 0) + 1
    state = write(os.path.join(SUBSETS, "🌐️any", "🧬️schema", "🧬️mutations", "🔣️.json"), mutation_schema(), check)
    tally[state] = tally.get(state, 0) + 1
    print("25 per-kind payload schemas + snapshot schema + aggregate mutation schema: %s" % ", ".join("%s %d" % (key, value) for key, value in sorted(tally.items())))
    print("25 leaf descriptors (outcomeClasses): %s" % ", ".join("%s %d" % item for item in sorted(patch_descriptors(check).items())))
    return 1 if check and tally.get("differs") else 0


if __name__ == "__main__":
    sys.exit(main())
# endregion 🔖️Emit
