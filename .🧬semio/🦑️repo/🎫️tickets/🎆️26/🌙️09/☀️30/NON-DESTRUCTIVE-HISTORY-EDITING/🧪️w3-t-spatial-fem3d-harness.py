#!/usr/bin/env python3
"""🧪️ W3-T-SPATIAL: registers the fem3d `move-selection` kind with every cross-language harness and catalog that
enumerates the vocabulary — the `🕸️mesh` and `🌐️any` subject adapters (`🦀️.rs`: KINDS, spec / hall / rejection
arms, REJECT_VECTORS, the members a transform writes), the Gherkin features (`🥒️.feature`), the Python references
(`🐍️.py`, which gain an INDEPENDENT implementation of the relative transform), and the mesh / any oracle catalogs.
Idempotent: every insertion is marker-guarded.
"""
import json
import os
import re
from collections import OrderedDict

SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets"
MESH_CASE = SUBSETS + "/🕸️mesh/🧪️tests/🕸️mutate-fem3d-1-mesh"
ANY_CASE = SUBSETS + "/🌐️any/🧪️tests/🕸️mutate-fem3d-1-any-mesh"
FIXTURES = SUBSETS + "/🕸️mesh/🧫️fixtures/🧬️mutations/🧭️move-selection"
KIND, DIR = "move-selection", "🧭️move-selection"
FEATURE_PAYLOAD = '{"mutation":"moveSelection","nodeIds":["sc0","sc1","sc2","sc3"],"solidIds":["sol1"],"pivotX":11.0,"pivotY":1.0,"pivotZ":0.0,"dx":0.5,"dy":0.25,"dz":0.0,"axisX":0.0,"axisY":0.0,"axisZ":1.0,"angle":0.0,"sx":1.0,"sy":1.0,"sz":1.0}'


def scenarios():
    """📂 `(folder, id, class)` per committed scenario: spec, hall, then every refusal or no-op."""
    rows = []
    for folder in sorted(os.listdir(FIXTURES)):
        identifier = re.sub(r"^[^\w]+", "", folder)
        klass = "spec" if folder.startswith("🧭️") else "hall" if folder.startswith("🏗️") else "reject"
        rows.append((folder, identifier, klass))
    return rows


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def once(text, old, new, label):
    if new in text:
        return text
    assert text.count(old) == 1, f"{label}: anchor not unique/absent: {old[:90]!r}"
    return text.replace(old, new, 1)


# region 🦀️Rust
def vector_arm(key, folder, prefix, with_diff):
    base = f"{prefix}{DIR}/{folder}/"
    lines = [f'        "{key}" => Vector {{', f'            before: include_str!("{base}📸️snapshot/⬅️before/🔣️.json"),', f'            mutation: include_str!("{base}🦠️mutation/🔣️.json"),', f'            after: include_str!("{base}📸️snapshot/➡️after/🔣️.json"),']
    lines.append(f'            diff: include_str!("{base}🔺️diff/🔣️.json"),' if with_diff else '            diff: "{}",')
    lines.append(f'            outcome: include_str!("{base}🎯️outcome/🔣️.json"),')
    lines.append("        },")
    return "\n".join(lines) + "\n"


TOUCHES_OLD = '''    fn touches_one(scenario: &str, kind: &str, before: &Json, after: &Json) -> Result<(), String> {
        let written = match kind {'''
TOUCHES_NEW = '''    fn touches_one(scenario: &str, kind: &str, before: &Json, after: &Json) -> Result<(), String> {
        if kind == "move-selection" {
            let moved: Vec<&str> = ["nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis"].into_iter().filter(|name| before.get(name) != after.get(name)).collect();
            if moved.is_empty() || moved.iter().any(|name| !["nodes", "solids"].contains(name)) {
                return Err(format!("{scenario}: move-selection writes nodes and solids and nothing else, but {moved:?} moved"));
            }
            return Ok(());
        }
        let written = match kind {'''


def add_arm(text, arm, panic_marker):
    if arm in text:
        return text
    anchor = re.search(r"\n        other => panic!\(\"[^\"]*" + panic_marker + r"[^\n]*\n", text).group(0)
    return text.replace(anchor, "\n" + arm + anchor.lstrip("\n"), 1)


def patch_rust(path, prefix, with_hall):
    text = read(path)
    rows = scenarios()
    kinds = re.search(r"^const KINDS: &\[&str\] = &\[(.*)\];$", text, re.M)
    if f'"{KIND}"' not in kinds.group(1):
        text = text.replace(kinds.group(0), kinds.group(0)[:-2] + f', "{KIND}"];', 1)
    spec = [row for row in rows if row[2] == "spec"][0]
    text = add_arm(text, vector_arm(KIND, spec[0], prefix, True), "no committed specification vector")
    if with_hall:
        hall = [row for row in rows if row[2] == "hall"][0]
        text = add_arm(text, vector_arm(KIND, hall[0], prefix, True), "no committed hall vector")
        for folder, identifier, klass in rows:
            if klass != "reject":
                continue
            current = re.search(r"^const REJECT_VECTORS: &\[&str\] = &\[(.*)\];$", text, re.M)
            if f'"{identifier}"' not in current.group(1):
                text = text.replace(current.group(0), current.group(0)[:-2] + f', "{identifier}"];', 1)
            text = add_arm(text, vector_arm(identifier, folder, prefix, folder.startswith("⏸️")), "no committed rejection vector")
    text = once(text, TOUCHES_OLD, TOUCHES_NEW, "touches_one")
    write(path, text)
# endregion 🦀️Rust


# region 🥒️Feature
def table_bounds(text, tag):
    start = text.index(f"  @id-{tag}\n")
    examples = text.index("    Examples:\n", start) + len("    Examples:\n")
    end = examples
    while end < len(text) and text[end:end + 6] == "    | ":
        end = text.index("\n", end) + 1
    return examples, end


def append_rows(text, tag, new_rows):
    if f"  @id-{tag}\n" not in text:
        return text
    start, end = table_bounds(text, tag)
    cells = [[cell.strip() for cell in line.strip().strip("|").split(" | ")] for line in text[start:end].splitlines()]
    existing = {row[0] for row in cells}
    additions = [row for row in new_rows if row[0] not in existing]
    if not additions:
        return text
    cells.extend(additions)
    widths = [max(len(row[at]) for row in cells) for at in range(len(cells[0]))]
    rebuilt = "".join("    | " + " | ".join(cell.ljust(width) for cell, width in zip(row, widths)) + " |\n" for row in cells)
    return text[:start] + rebuilt + text[end:]


def patch_feature(path, with_hall):
    text = read(path)
    rows = scenarios()
    text = append_rows(text, "mutate", [[KIND, FEATURE_PAYLOAD]])
    text = append_rows(text, "inverse", [[KIND, FEATURE_PAYLOAD]])
    if with_hall:
        spec = [row for row in rows if row[2] == "spec"][0]
        hall = [row for row in rows if row[2] == "hall"][0]
        text = append_rows(text, "spec-vector", [[KIND, DIR, spec[0]]])
        text = append_rows(text, "hall-vector", [[KIND, DIR, hall[0]]])
        text = append_rows(text, "reject", [[identifier, DIR, folder] for folder, identifier, klass in rows if klass == "reject"])
    write(path, text)
# endregion 🥒️Feature


# region 🐍️Python
PY_FUNCTIONS = '''def move_selection(result, mutation):
    """🧭️ The relative gumball transform, written from the leaf's schema alone: every named node and every footprint
    point of every named solid is scaled by `(sx, sy, sz)`, rotated by `angle` about the axis through the pivot
    (Rodrigues), then offset by `(dx, dy, dz)`; a solid follows only a map that keeps its footprint plane and a positive
    height. Refuses a non-positive factor, a rotation about the zero axis, a target named twice and a payload none of
    whose targets exist; an identity changes nothing."""
    nodes, solids = mutation["nodeIds"], mutation["solidIds"]
    if min(mutation["sx"], mutation["sy"], mutation["sz"]) <= 0.0:
        raise AssertionError("move-selection: every scale factor must be positive")
    length = math.sqrt(mutation["axisX"] ** 2 + mutation["axisY"] ** 2 + mutation["axisZ"] ** 2)
    if mutation["angle"] != 0.0 and length <= 1e-12:
        raise AssertionError("move-selection: a rotation needs a non-zero axis")
    for ids in (nodes, solids):
        if len(set(ids)) != len(ids):
            raise AssertionError("move-selection: a target is named twice")
    if not any(node["id"] in nodes for node in result["nodes"]) and not any(solid["id"] in solids for solid in result["solids"]):
        raise AssertionError("move-selection: none of the named nodes and solids exist")
    pivot = [mutation["pivotX"], mutation["pivotY"], mutation["pivotZ"]]

    def mapped(point):
        s = [(point[0] - pivot[0]) * mutation["sx"], (point[1] - pivot[1]) * mutation["sy"], (point[2] - pivot[2]) * mutation["sz"]]
        if mutation["angle"] != 0.0:
            a = [mutation["axisX"] / length, mutation["axisY"] / length, mutation["axisZ"] / length]
            sine, cosine = math.sin(mutation["angle"]), math.cos(mutation["angle"])
            cross = [a[1] * s[2] - a[2] * s[1], a[2] * s[0] - a[0] * s[2], a[0] * s[1] - a[1] * s[0]]
            along = (a[0] * s[0] + a[1] * s[1] + a[2] * s[2]) * (1.0 - cosine)
            s = [s[at] * cosine + cross[at] * sine + a[at] * along for at in range(3)]
        return [pivot[0] + s[0] + mutation["dx"], pivot[1] + s[1] + mutation["dy"], pivot[2] + s[2] + mutation["dz"]]

    lift = {"x": lambda u, v, w: [w, u, v], "y": lambda u, v, w: [u, w, v], "z": lambda u, v, w: [u, v, w]}
    project = {"x": lambda p: ([p[1], p[2]], p[0]), "y": lambda p: ([p[0], p[2]], p[1]), "z": lambda p: ([p[0], p[1]], p[2])}
    for node in result["nodes"]:
        if node["id"] in nodes:
            node["x"], node["y"], node["z"] = mapped([node["x"], node["y"], node["z"]])
    for solid in result["solids"]:
        if solid["id"] not in solids:
            continue
        up, onto = lift[solid["axis"]], project[solid["axis"]]
        placed = [onto(mapped(up(point[0], point[1], solid["baseZ"]))) for point in solid["outline"]]
        holes = [[onto(mapped(up(point[0], point[1], solid["baseZ"]))) for point in hole] for hole in solid["holes"]]
        bases = [w for _, w in placed] + [w for hole in holes for _, w in hole]
        _, top = onto(mapped(up(solid["outline"][0][0], solid["outline"][0][1], solid["baseZ"] + solid["height"])))
        if max(bases) - min(bases) > 1e-9 or not top - bases[0] > 0.0:
            continue
        solid["outline"], solid["holes"], solid["height"], solid["baseZ"] = [uv for uv, _ in placed], [[uv for uv, _ in hole] for hole in holes], top - bases[0], bases[0]
    return result


def inverse_steps(document, mutation):
    """↩️ The steps that undo one application: a move-selection restores every BASE node and solid it moves with one
    whole-record replacement each; every other kind undoes with its one computed inverse."""
    if kind_of(mutation) != "move-selection":
        return [inverse_mutation(document, mutation)]
    moved = apply_mutation(document, mutation)
    steps = [{"mutation": "replaceNode", "id": node["id"], "newNode": copy.deepcopy(node)} for node, after in zip(document["nodes"], moved["nodes"]) if node != after]
    steps += [{"mutation": "replaceSolid", "id": solid["id"], "newSolid": copy.deepcopy(solid)} for solid, after in zip(document["solids"], moved["solids"]) if solid != after]
    return steps


def apply_steps(document, steps):
    """🧮️ Applies every step in order."""
    for step in steps:
        document = apply_mutation(document, step)
    return document


'''
PY_TOUCHES_OLD = '''    member it meant to write."""
    if kind == "update-analysis-settings":
        written = "analysis"'''
PY_TOUCHES_NEW = '''    member it meant to write."""
    if kind == "move-selection":
        moved = [name for name in MEMBERS if before[name] != after[name]]
        if not moved or any(name not in ("nodes", "solids") for name in moved):
            raise AssertionError("%s: move-selection writes nodes and solids and nothing else, but %r moved" % (scenario, moved))
        return
    if kind == "update-analysis-settings":
        written = "analysis"'''


def patch_python(path):
    text = read(path)
    text = re.sub(r'^KINDS = \((.*)\)$', lambda match: match.group(0) if f'"{KIND}"' in match.group(1) else f'KINDS = ({match.group(1)}, "{KIND}")', text, count=1, flags=re.M)
    if "\nimport math\n" not in text:
        text = once(text, "import copy\nimport json\n", "import copy\nimport json\nimport math\n", "import math")
    dispatch = '''    if kind == "move-selection":
        result = move_selection(copy.deepcopy(document), mutation)
        validate(result)
        return result
'''
    if dispatch not in text:
        head = "    kind = kind_of(mutation)\n    refuse(document, kind, mutation)\n" if "    kind = kind_of(mutation)\n    refuse(document, kind, mutation)\n" in text else "    kind = kind_of(mutation)\n    result = copy.deepcopy(document)\n"
        tail = head.split("\n", 1)[1]
        text = once(text, head, "    kind = kind_of(mutation)\n" + dispatch + tail, "apply dispatch")
    text = once(text, "def inverse_mutation(document, mutation):", PY_FUNCTIONS + "def inverse_mutation(document, mutation):", "functions")
    text = once(text, PY_TOUCHES_OLD, PY_TOUCHES_NEW, "touches_one")
    text = text.replace("restored = apply_mutation(applied, inverse_mutation(document, mutation))", "restored = apply_steps(applied, inverse_steps(document, mutation))")
    text = text.replace("restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)", "restores(kind, apply_steps(applied, inverse_steps(before, mutation)), before)")
    write(path, text)
# endregion 🐍️Python


# region 🔮️Oracles
def patch_mesh_oracle(path):
    data = json.loads(read(path), object_pairs_hook=OrderedDict)
    catalog = data["mutationCatalogs"][0]
    if KIND not in catalog["kinds"]:
        catalog["kinds"].append(KIND)
    if not any(vector["mutationId"] == KIND for vector in catalog["vectors"]):
        catalog["vectors"].append(OrderedDict([("mutationId", KIND), ("sourceMutationDirectoryName", DIR), ("mutationDirectoryName", DIR), ("scenarios", [OrderedDict([("id", ident), ("directoryName", folder)]) for folder, ident, _ in scenarios()])]))
    write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")


def patch_any_oracle(path):
    data = json.loads(read(path), object_pairs_hook=OrderedDict)
    mesh = next(catalog for catalog in data["mutationCatalogs"] if catalog["id"].endswith("-mesh"))
    if KIND not in mesh["kinds"]:
        mesh["kinds"].append(KIND)
    manifests = data["mutationManifests"][0]["mutations"]
    if not any(manifest["id"] == KIND for manifest in manifests):
        template = json.loads(json.dumps(next(manifest for manifest in manifests if manifest["id"] == "replace-node")), object_pairs_hook=OrderedDict)
        template["id"] = KIND
        template["payloadSchema"] = template["payloadSchema"].replace("🔁️replace-node", DIR).replace("#ReplaceNode", "#MoveSelection")
        template["productionDispatch"] = OrderedDict([("operation", KIND), ("bridgeVersion", 1), ("variant", "MoveSelection")])
        template["oracleRequirements"] = [requirement for requirement in template["oracleRequirements"] if "carrier" not in requirement["capability"]]
        manifests.append(template)
    write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
# endregion 🔮️Oracles


def main():
    patch_rust(MESH_CASE + "/🦀️.rs", "../../🧫️fixtures/🧬️mutations/", True)
    patch_rust(ANY_CASE + "/🦀️.rs", "../../../🕸️mesh/🧫️fixtures/🧬️mutations/", False)
    patch_feature(MESH_CASE + "/🥒️.feature", True)
    patch_feature(ANY_CASE + "/🥒️.feature", False)
    patch_python(MESH_CASE + "/🐍️.py")
    patch_python(ANY_CASE + "/🐍️.py")
    patch_mesh_oracle(SUBSETS + "/🕸️mesh/🔮️oracles/🔣️.json")
    patch_any_oracle(SUBSETS + "/🌐️any/🔮️oracles/🔣️.json")
    print("registered", KIND, [row[0] for row in scenarios()])


if __name__ == "__main__":
    main()
