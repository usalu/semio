#!/usr/bin/env python3
"""🧊 Derives the `mutate-fem3d-1` case fixture ONCE from the committed real structural model.

Provenance, in full:

* Input ``✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/…/📚️examples/🎬️demo/🖼️assets/🗣️example.dsl.semio``
  — the artifact's own committed demo model, and a real one: a sixteen-node, two-storey steel frame
  on an 8 × 10 m grid with four fully clamped column bases, sixteen HEA 200 frame members, two
  materials with their real moduli, a first-floor concrete slab solid, four extra pinned slab-corner
  supports, a dead case carrying an area pressure, a live case carrying a nodal load and an area
  pressure, and an ULS combination at 1.35/1.5.
* Output ``…/🧪️tests/mutate-fem3d-1/🧫️fixtures/🧊️steel-frame.snapshot.json``.

Everything above is carried across unchanged. What is ADDED, and why is the same as for this
artifact's `◻2d` sibling: the vocabulary's `delete-` and `replace-` verbs need a target, and the
committed model REFERENCES every entity it holds. Six unreferenced spares are appended, each taken
from a committed specification vector of this same subset and repointed only onto ids the model
already holds:

    node        `n3`        🌱⚪️create-node/appends-the-column-head-node-n3
    section     `shs120`    🌱create-section/appends-a-square-hollow-profile
    material    `alu`       🌱🧱️create-material/appends-an-aluminium-alloy
    support     `s_spare`   🌱🛡️create-support/clamps-the-column-base-in-all-six-dofs, re-pointed at `n3`
    solid       `sol_spare` 🌱🧊️create-solid/appends-an-extruded-roof-slab, re-pointed at the
                            committed model's own `concrete` material so deleting `alu` leaves
                            nothing dangling
    load case   `wind`      🌱📋️create-load-case/appends-a-wind-case-pushing-on-the-column-head,
                            re-pointed at the committed model's own node `n20_l2`
    combination `sls_spare` 🌱🔗️create-combination/appends-a-serviceability-combination-keyed-by-case-id,
                            re-termed onto the committed model's own `dead`/`live` case ids

The spares are appended LAST, so the committed entities keep their indices and every spare is the
TRAILING member of its collection — which is what makes the inverse of a delete exact in a vocabulary
whose `create-` verbs carry no index.
"""

# region 🔖️Imports
import json
import os
import re
import sys

# endregion 🔖️Imports


# region 🔖️Carrier
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


def read_points(stream, at):
    """📐️ A `[ x,y x,y … ]` carrier point list, returning `(points, next-index)`."""
    if stream[at] != "[":
        raise SystemExit("expected a point list, found %r" % stream[at])
    at += 1
    points = []
    while stream[at] != "]":
        x, _, y = stream[at].partition(",")
        points.append([float(x), float(y)])
        at += 1
    return points, at + 1


# endregion 🔖️Carrier


# region 🔖️Collections
def read_nodes(text):
    return [{"id": row[0], "x": float(row[1]), "y": float(row[2]), "z": float(row[3])} for row in map(tokens_of, section_body(text, "nodes [").strip().split("\n"))]


def read_elements(text):
    out = []
    for line in section_body(text, "elements {").strip().split("\n"):
        kind, fields = tokens_of(line)[0], fields_of(line)
        record = {"kind": kind, "id": fields["id"], "start": fields["start"], "end": fields["end"], "materialId": fields["material-id"], "sectionId": fields["section-id"]}
        if "roll" in fields:
            record["roll"] = float(fields["roll"])
        out.append(record)
    return out


def read_materials(text):
    return [{"id": r[0], "name": r[1], "e": float(r[2]), "g": float(r[3]), "nu": float(r[4]), "rho": float(r[5])} for r in map(tokens_of, section_body(text, "materials [").strip().split("\n"))]


def read_sections(text):
    return [{"id": r[0], "name": r[1], "area": float(r[2]), "iy": float(r[3]), "iz": float(r[4]), "j": float(r[5])} for r in map(tokens_of, section_body(text, "sections [").strip().split("\n"))]


def read_solids(text):
    out = []
    for line in section_body(text, "solids [").strip().split("\n"):
        parts = tokens_of(line)
        outline, at = read_points(parts, 2)
        holes, at = read_points(parts, at)
        out.append({"id": parts[0], "name": parts[1], "outline": outline, "holes": holes, "baseZ": float(parts[at]), "height": float(parts[at + 1]), "layers": int(parts[at + 2]), "meshSize": float(parts[at + 3]), "materialId": parts[at + 4]})
    return out


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
        return {"kind": "area", "id": fields["id"], "solidId": fields["solid-id"], "pressure": float(fields["pressure"])}
    if kind == "memberUdl":
        return {"kind": "memberUdl", "id": fields["id"], "elementId": fields["element-id"], "wx": float(fields["wx"]), "wy": float(fields["wy"]), "wz": float(fields["wz"])}
    raise SystemExit("unknown load kind %r" % kind)


def read_load_cases(text):
    """📋️ A header row, a braced load block, then the boolean on a line of its own."""
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
    """🔗️ `combinations [id:TEXT name:TEXT terms:MAP]` — a header row then a braced `case=factor` map."""
    body = section_body(text, "combinations [").split("\n")
    out, at = [], 0
    while at < len(body):
        line = body[at].strip()
        if not line:
            at += 1
            continue
        parts = tokens_of(line)
        combination = {"id": parts[0], "name": parts[1], "terms": {}}
        at += 1
        while body[at].strip() != "}":
            for key, value in (pair.split("=", 1) for pair in tokens_of(body[at])):
                combination["terms"][key] = float(value)
            at += 1
        at += 1
        out.append(combination)
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
    """🧬️ Writes the derived model: the committed frame, plus six unreferenced committed spares."""
    text = open(source, encoding="utf-8").read()
    model = {
        "nodes": read_nodes(text),
        "elements": read_elements(text),
        "materials": read_materials(text),
        "sections": read_sections(text),
        "solids": read_solids(text),
        "supports": read_supports(text),
        "loadCases": read_load_cases(text),
        "combinations": read_combinations(text),
        "analysis": read_analysis(text),
    }
    if len(model["nodes"]) != 16 or len(model["elements"]) != 16 or len(model["supports"]) != 8:
        raise SystemExit("the committed example is expected to be the sixteen-node, sixteen-member, eight-support frame")

    spare_node = committed(subset_root, "🌱⚪️create-node", "appends-the-column-head-node-n3", "node")
    spare_support = dict(committed(subset_root, "🌱🛡️create-support", "clamps-the-column-base-in-all-six-dofs", "support"), id="s_spare", nodeId=spare_node["id"])
    spare_solid = dict(committed(subset_root, "🌱🧊️create-solid", "appends-an-extruded-roof-slab", "solid"), id="sol_spare", materialId=model["materials"][-1]["id"])
    spare_case = committed(subset_root, "🌱📋️create-load-case", "appends-a-wind-case-pushing-on-the-column-head", "loadCase")
    spare_case = dict(spare_case, loads=[dict(load, nodeId="n20_l2") for load in spare_case["loads"]])
    spare_combination = committed(subset_root, "🌱🔗️create-combination", "appends-a-serviceability-combination-keyed-by-case-id", "combination")
    spare_combination = dict(spare_combination, id="sls_spare", terms={case["id"]: factor for case, factor in zip(model["loadCases"], spare_combination["terms"].values())})
    model["nodes"].append(spare_node)
    model["sections"].append(committed(subset_root, "🌱create-section", "appends-a-square-hollow-profile", "section"))
    model["materials"].append(committed(subset_root, "🌱🧱️create-material", "appends-an-aluminium-alloy", "material"))
    model["supports"].append(spare_support)
    model["solids"].append(spare_solid)
    model["loadCases"].append(spare_case)
    model["combinations"].append(spare_combination)

    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(model, ensure_ascii=False, indent=2) + "\n")
    print("wrote %s (%d nodes, %d elements, %d materials, %d sections, %d solids, %d supports, %d cases, %d combinations)" % (target, len(model["nodes"]), len(model["elements"]), len(model["materials"]), len(model["sections"]), len(model["solids"]), len(model["supports"]), len(model["loadCases"]), len(model["combinations"])))


# endregion 🔖️Derivation


if __name__ == "__main__":
    derive(sys.argv[1], sys.argv[2], sys.argv[3])
