#!/usr/bin/env python3
"""🏗️ Derives the `mutate-fem2d-1` case fixture ONCE from the committed real structural model.

Provenance, in full:

* Input ``✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻2d/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio``
  — the artifact's own committed demo model, and a real one: a twelve-node timber-and-steel portal
  frame with a ridge at 7.6 m, nine beam elements, four supports, three materials (S235 steel, C24
  timber, C30/37 concrete) with their real moduli and densities, four sections with their real areas
  and second moments, a first-floor slab region, a dead case carrying an area pressure, a live case
  carrying a nodal load and an area pressure, and an ULS combination at 1.35/1.5.
* Output ``…/🧪️tests/mutate-fem2d-1/🧫️fixtures/🏗️timber-portal-frame.snapshot.json``.

Everything above is carried across unchanged. What is ADDED, and why: the vocabulary's `delete-` and
`replace-` verbs need a target, and the committed model REFERENCES every entity it holds — every
material by an element or the slab region, every section by an element, every node by an element or a
support, the slab by two area loads, and both load cases by the ULS combination's terms. Deleting a
referenced entity asks a question no committed document answers, so seven UNREFERENCED spares are
appended, each taken from a committed specification vector of this same subset:

    node        `n3`         🌱⚪️create-node/appends-node-n3
    section     `ipe300`     🌱create-section/appends-the-ipe300-profile
    material    `c30`        🌱🧱️create-material/appends-concrete-c30
    support     `s_spare`    🌱🛡️create-support/adds-a-vertical-roller-at-node-n2, re-pointed at `n3`
    region      `slab_spare` 🌱🗺️create-region/appends-a-solid-rectangular-slab, re-pointed at the
                             committed model's own `concrete` material so deleting `c30` leaves
                             nothing dangling
    load case   `snow`       🌱📋️create-load-case/appends-a-live-case-carrying-one-nodal-load,
                             re-pointed at the committed model's own node `p8_l1`
    combination `uls_spare`  🌱🔗️create-combination/appends-an-uls-combination-over-both-cases,
                             re-termed onto the committed model's own `dead`/`live` case ids

The only edits are those repointings, and each one points at an id this file already holds. The
spares are appended LAST, so the committed entities keep their indices and every spare is the
TRAILING member of its collection — which is also what makes the inverse of a delete exact in a
vocabulary whose `create-` verbs carry no index.

What the committed vectors settle, and this reading follows: their own names state the cascade rules
this vocabulary has — `removes-node-n3-without-cascading-to-its-support`,
`removes-the-slab-and-keeps-its-material`, `removes-bar-e2-and-keeps-its-end-nodes`,
`removes-the-uls-combination-and-keeps-both-cases`,
`removes-the-live-case-together-with-its-loads`. A delete removes exactly its own entity; only a load
case takes its nested loads with it, because they are nested inside it.
"""

# region 🔖️Imports
import json
import os
import re
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
UNIT = re.compile(r"^(-?\d+(?:\.\d+)?(?:[eE][-+]?\d+)?)(?:[A-Za-z][A-Za-z0-9/]*)?$")
"""🔢️ A carrier quantity: a number with an optional unit suffix (`0m`, `210000000000Pa`, `7850kg/m3`)."""


def number(token):
    """🔢️ The numeric value of a carrier quantity."""
    matched = UNIT.match(token)
    if matched is None:
        raise SystemExit("not a carrier quantity: %r" % token)
    return float(matched.group(1))


def tokens_of(line):
    """✂️ Splits a carrier row into tokens, keeping a `"quoted phrase"` whole."""
    return [part[1:-1] if part.startswith('"') else part for part in re.findall(r'"[^"]*"|\S+', line)]


def section_body(text, header):
    """🧱️ The body of a `name … { … }` carrier section, matched by brace depth."""
    at = text.index(header)
    start = text.index("{", at) + 1
    depth, cursor = 1, start
    while depth:
        if text[cursor] == "{":
            depth += 1
        elif text[cursor] == "}":
            depth -= 1
        cursor += 1
    return text[start : cursor - 1]


def fields_of(line):
    """🔧️ A `key=value` row — the shape `elements` and the load blocks use."""
    return dict(part.split("=", 1) for part in tokens_of(line) if "=" in part)


# endregion 🔖️Carrier


# region 🔖️Collections
def read_nodes(text):
    return [{"id": row[0], "x": number(row[1]), "y": number(row[2])} for row in map(tokens_of, section_body(text, "nodes [").strip().split("\n"))]


def read_elements(text):
    out = []
    for line in section_body(text, "elements {").strip().split("\n"):
        parts = tokens_of(line)
        fields = fields_of(line)
        out.append({"kind": parts[0], "id": fields["id"], "start": fields["start"], "end": fields["end"], "materialId": fields["material-id"], "sectionId": fields["section-id"]})
    return out


def read_points(token_stream, at):
    """📐️ A `[ x,y x,y … ]` carrier point list, returning `(points, next-index)`."""
    if token_stream[at] != "[":
        raise SystemExit("expected a point list, found %r" % token_stream[at])
    at += 1
    points = []
    while token_stream[at] != "]":
        x, _, y = token_stream[at].partition(",")
        points.append([float(x), float(y)])
        at += 1
    return points, at + 1


def read_regions(text):
    out = []
    for line in section_body(text, "regions [").strip().split("\n"):
        parts = tokens_of(line)
        outline, at = read_points(parts, 2)
        holes, at = read_points(parts, at)
        out.append({"id": parts[0], "name": parts[1], "outline": outline, "holes": holes, "thickness": number(parts[at]), "materialId": parts[at + 1], "meshSize": number(parts[at + 2])})
    return out


def read_materials(text):
    return [{"id": row[0], "name": row[1], "e": number(row[2]), "nu": number(row[3]), "rho": number(row[4])} for row in map(tokens_of, section_body(text, "materials [").strip().split("\n"))]


def read_sections(text):
    return [{"id": row[0], "name": row[1], "area": number(row[2]), "iy": number(row[3])} for row in map(tokens_of, section_body(text, "sections [").strip().split("\n"))]


def read_supports(text):
    out = []
    for line in section_body(text, "supports [").strip().split("\n"):
        parts = tokens_of(line)
        out.append({"id": parts[0], "nodeId": parts[1], "fixed": parts[parts.index("[") + 1 : parts.index("]")]})
    return out


def read_load(line):
    """🏋️ One load row of a load case's `loads:BLOCK` column."""
    kind, fields = tokens_of(line)[0], fields_of(line)
    if kind == "nodal":
        return {"kind": "nodal", "id": fields["id"], "nodeId": fields["node-id"], "dof": fields["dof"], "value": float(fields["value"])}
    if kind == "area":
        return {"kind": "area", "id": fields["id"], "regionId": fields["region-id"], "pressure": float(fields["pressure"])}
    if kind == "memberUdl":
        return {"kind": "memberUdl", "id": fields["id"], "elementId": fields["element-id"], "wx": float(fields["wx"]), "wy": float(fields["wy"])}
    raise SystemExit("unknown load kind %r" % kind)


def read_load_cases(text):
    """📋️ `load-cases [id:TEXT name:TEXT loads:BLOCK self-weight:BOOL]` — a header row, a braced load
    block, then the boolean on a line of its own."""
    body = section_body(text, "load-cases [").split("\n")
    out, at = [], 0
    while at < len(body):
        line = body[at].strip()
        if not line:
            at += 1
            continue
        parts = tokens_of(line)
        case = {"id": parts[0], "name": parts[1], "loads": [], "selfWeight": False}
        at += 1
        while body[at].strip() != "}":
            case["loads"].append(read_load(body[at]))
            at += 1
        at += 1
        case["selfWeight"] = body[at].strip() == "true"
        at += 1
        out.append(case)
    return out


def read_combinations(text):
    out = []
    for line in section_body(text, "combinations [").strip().split("\n"):
        parts = tokens_of(line)
        terms, at = [], parts.index("[") + 1
        while parts[at] != "]":
            case_id = parts[at].split("=", 1)[1]
            factor = float(parts[at + 1].split("=", 1)[1])
            terms.append({"caseId": case_id, "factor": factor})
            at += 2
        out.append({"id": parts[0], "name": parts[1], "terms": terms})
    return out


def read_analysis(text):
    fields = fields_of(section_body(text, "analysis {").strip())
    return {"modalCount": int(fields["modal-count"]), "bucklingCount": int(fields["buckling-count"]), "deformationScale": float(fields["deformation-scale"])}


# endregion 🔖️Collections


# region 🔖️Derivation
def committed(root, kind_dir, fixture, member):
    """🧫️ One entity, taken verbatim from a committed specification vector's mutation payload."""
    path = os.path.join(root, "🧬️schema/🧬️mutations", kind_dir, "🧪️tests", fixture, "🦠️mutation/🔣️component.json")
    return json.load(open(path, encoding="utf-8"))[member]


def derive(source, subset_root, target):
    """🧬️ Writes the derived model: the committed frame, plus five unreferenced committed spares."""
    text = open(source, encoding="utf-8").read()
    model = {
        "nodes": read_nodes(text),
        "elements": read_elements(text),
        "regions": read_regions(text),
        "materials": read_materials(text),
        "sections": read_sections(text),
        "supports": read_supports(text),
        "loadCases": read_load_cases(text),
        "combinations": read_combinations(text),
        "analysis": read_analysis(text),
    }
    if len(model["nodes"]) != 12 or len(model["elements"]) != 9 or len(model["loadCases"]) != 2:
        raise SystemExit("the committed example is expected to be the twelve-node, nine-element, two-case frame")

    spare_node = committed(subset_root, "🌱⚪️create-node", "appends-node-n3", "node")
    spare_support = dict(committed(subset_root, "🌱🛡️create-support", "adds-a-vertical-roller-at-node-n2", "support"), id="s_spare", nodeId=spare_node["id"])
    spare_region = dict(committed(subset_root, "🌱🗺️create-region", "appends-a-solid-rectangular-slab", "region"), id="slab_spare", materialId=model["materials"][-1]["id"])
    spare_case = committed(subset_root, "🌱📋️create-load-case", "appends-a-live-case-carrying-one-nodal-load", "loadCase")
    spare_case = dict(spare_case, id="snow", name="Snow", loads=[dict(load, nodeId="p8_l1") for load in spare_case["loads"]])
    spare_combination = committed(subset_root, "🌱🔗️create-combination", "appends-an-uls-combination-over-both-cases", "combination")
    spare_combination = dict(spare_combination, id="uls_spare", terms=[{"caseId": case["id"], "factor": term["factor"]} for case, term in zip(model["loadCases"], spare_combination["terms"])])
    model["nodes"].append(spare_node)
    model["sections"].append(committed(subset_root, "🌱create-section", "appends-the-ipe300-profile", "section"))
    model["materials"].append(committed(subset_root, "🌱🧱️create-material", "appends-concrete-c30", "material"))
    model["supports"].append(spare_support)
    model["regions"].append(spare_region)
    model["loadCases"].append(spare_case)
    model["combinations"].append(spare_combination)

    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(model, ensure_ascii=False, indent=2) + "\n")
    print("wrote %s (%d nodes, %d elements, %d materials, %d sections, %d supports, %d cases, %d combinations)" % (target, len(model["nodes"]), len(model["elements"]), len(model["materials"]), len(model["sections"]), len(model["supports"]), len(model["loadCases"]), len(model["combinations"])))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2], sys.argv[3])
