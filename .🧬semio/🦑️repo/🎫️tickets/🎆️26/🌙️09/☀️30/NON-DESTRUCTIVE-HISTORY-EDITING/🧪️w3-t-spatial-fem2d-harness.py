#!/usr/bin/env python3
"""🧪️ W3-T-SPATIAL: registers the fem2d `move-selection` kind with every cross-language harness and catalog that
enumerates the vocabulary — the `🕸️mesh` and `🌐️any` subject adapters (`🦀️.rs`), Gherkin features (`🥒️.feature`),
Python references (`🐍️.py`, which gain an INDEPENDENT implementation of the relative transform), and the mesh / any
oracle catalogs (`🔮️oracles/🔣️.json`). Idempotent: every insertion is marker-guarded.
"""
import json
import os
import re
from collections import OrderedDict

SUBSETS = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets"
MESH_CASE = SUBSETS + "/🕸️mesh/🧪️tests/🕸️mutate-fem2d-1-mesh"
ANY_CASE = SUBSETS + "/🌐️any/🧪️tests/🕸️mutate-fem2d-1-any-mesh"
FIXTURES = SUBSETS + "/🕸️mesh/🧫️fixtures/🧬️mutations/🧭️move-selection"
KIND, DIR = "move-selection", "🧭️move-selection"
FEATURE_PAYLOAD = '{"mutation":"moveSelection","nodeIds":["rc0","rc1","rc2","rc3"],"regionIds":["r1"],"pivotX":11.0,"pivotY":2.8,"dx":0.5,"dy":0.25,"angle":0.0,"sx":1.0,"sy":1.0}'


def scenarios():
    """📂 `(folder, id, class)` of every committed scenario, spec first, then frame, then the refusals in order."""
    order = {"🧭️": 0, "📏️": 1, "⛔️": 2, "⏸️": 3, "🫓️": 4, "🔁️": 5}
    rows = []
    for folder in os.listdir(FIXTURES):
        prefix = next(emoji for emoji in order if folder.startswith(emoji))
        rows.append((order[prefix], folder, folder[len(prefix):], {0: "spec", 1: "frame"}.get(order[prefix], "reject")))
    return [row[1:] for row in sorted(rows)]


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
def vector_arm(key, folder, prefix, with_diff, optional_diff):
    base = f"{prefix}{DIR}/{folder}/"
    lines = [f'        "{key}" => Vector {{', f'            before: include_str!("{base}📸️snapshot/⬅️before/🔣️.json"),', f'            mutation: include_str!("{base}🦠️mutation/🔣️.json"),', f'            after: include_str!("{base}📸️snapshot/➡️after/🔣️.json"),']
    if with_diff:
        lines.append(f'            diff: Some(include_str!("{base}🔺️diff/🔣️.json")),' if optional_diff else f'            diff: include_str!("{base}🔺️diff/🔣️.json"),')
    elif optional_diff:
        lines.append("            diff: None,")
    lines.append(f'            outcome: include_str!("{base}🎯️outcome/🔣️.json"),')
    lines.append("        },")
    return "\n".join(lines) + "\n"


TOUCHES_RUST_OLD = '''    fn touches_one(scenario: &str, kind: &str, before: &Json, after: &Json) -> Result<(), String> {
        let written = match kind {'''
TOUCHES_RUST_NEW = '''    fn touches_one(scenario: &str, kind: &str, before: &Json, after: &Json) -> Result<(), String> {
        if kind == "move-selection" {
            let moved: Vec<&str> = ["nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis"].into_iter().filter(|name| before.get(name) != after.get(name)).collect();
            if moved.is_empty() || moved.iter().any(|name| !["nodes", "regions"].contains(name)) {
                return Err(format!("{scenario}: move-selection writes nodes and regions and nothing else, but {moved:?} moved"));
            }
            return Ok(());
        }
        let written = match kind {'''


def patch_mesh_rust(path):
    text = read(path)
    rows = scenarios()
    kinds = re.search(r"^const KINDS: &\[&str\] = &\[(.*)\];$", text, re.M)
    if f'"{KIND}"' not in kinds.group(1):
        text = text.replace(kinds.group(0), kinds.group(0)[:-2] + f', "{KIND}"];', 1)
    text = once(text, '    ("frame-vector-replace-node", "replace-node"),\n];', f'    ("frame-vector-replace-node", "replace-node"),\n    ("spec-vector-{KIND}", "{KIND}"),\n    ("frame-vector-{KIND}", "{KIND}"),\n];', "COMMITTED")
    refusals = [row for row in rows if row[2] == "reject"]
    added = "".join(f'    ("reject-{KIND}-{index}", "{KIND}"),\n' for index in range(1, len(refusals) + 1))
    text = once(text, '    ("reject-replace-node-2", "replace-node"),\n];', '    ("reject-replace-node-2", "replace-node"),\n' + added + "];", "REFUSED")
    arms = vector_arm(f"spec-vector-{KIND}", rows[0][0], "../../🧫️fixtures/🧬️mutations/", True, True) + vector_arm(f"frame-vector-{KIND}", rows[1][0], "../../🧫️fixtures/🧬️mutations/", True, True)
    arms += "".join(vector_arm(f"reject-{KIND}-{index}", folder, "../../🧫️fixtures/🧬️mutations/", False, True) for index, (folder, _, _) in enumerate(refusals, 1))
    anchor = '        other => panic!("🕸️mutate-fem2d-1-mesh: no committed specification vector is registered for scenario {other:?}"),\n'
    text = once(text, anchor, arms + anchor, "vector arms")
    text = once(text, TOUCHES_RUST_OLD, TOUCHES_RUST_NEW, "touches_one")
    write(path, text)


def patch_any_rust(path):
    text = read(path)
    rows = scenarios()
    kinds = re.search(r"^const KINDS: &\[&str\] = &\[(.*)\];$", text, re.M)
    if f'"{KIND}"' not in kinds.group(1):
        text = text.replace(kinds.group(0), kinds.group(0)[:-2] + f', "{KIND}"];', 1)
    arm = vector_arm(KIND, rows[0][0], "../../../🕸️mesh/🧫️fixtures/🧬️mutations/", True, False)
    anchor = re.search(r"\n        other => panic!\([^\n]*\n", text).group(0)[1:]
    text = once(text, anchor, arm + anchor, "any vector arm")
    text = once(text, TOUCHES_RUST_OLD, TOUCHES_RUST_NEW, "any touches_one")
    write(path, text)
# endregion 🦀️Rust


# region 🐍️Python
PY_APPLY = '''    if kind == "move-selection":
        return move_selection(result, mutation)
'''
PY_FUNCTIONS = '''def move_selection(result, mutation):
    """🧭️ The relative gumball transform, written from the leaf's schema alone: every named node and every outline and
    hole point of every named region is scaled by `(sx, sy)` and rotated by `angle` about the pivot, then offset by
    `(dx, dy)`. Guards in order: a non-positive factor or a target named twice (Fatal invariant), no named target
    exists (Error target-missing), nothing moves (Warning no-op). Missing targets beside present ones are skipped."""
    nodes, regions = mutation["nodeIds"], mutation["regionIds"]
    targets = nodes + regions
    if mutation["sx"] <= 0.0 or mutation["sy"] <= 0.0:
        fatal(INVARIANT, targets, "A move-selection needs positive scale factors.")
    for ids in (nodes, regions):
        for at, identifier in enumerate(ids):
            if identifier in ids[:at]:
                fatal(INVARIANT, [identifier], 'A move-selection names "%s" twice.' % identifier)
    if not any(node["id"] in nodes for node in result["nodes"]) and not any(region["id"] in regions for region in result["regions"]):
        error(TARGET_MISSING, targets, "None of the named nodes and regions exist.")
    sine, cosine = math.sin(mutation["angle"]), math.cos(mutation["angle"])

    def mapped(x, y):
        u, v = (x - mutation["pivotX"]) * mutation["sx"], (y - mutation["pivotY"]) * mutation["sy"]
        return mutation["pivotX"] + u * cosine - v * sine + mutation["dx"], mutation["pivotY"] + u * sine + v * cosine + mutation["dy"]

    moved = False
    for node in result["nodes"]:
        if node["id"] in nodes:
            x, y = mapped(node["x"], node["y"])
            moved |= (x, y) != (node["x"], node["y"])
            node["x"], node["y"] = x, y
    for region in result["regions"]:
        if region["id"] in regions:
            outline = [list(mapped(*point)) for point in region["outline"]]
            holes = [[list(mapped(*point)) for point in hole] for hole in region["holes"]]
            moved |= outline != region["outline"] or holes != region["holes"]
            region["outline"], region["holes"] = outline, holes
    if not moved:
        warn(NO_OP, "The transform moves none of the named nodes and regions.")
    validate(result)
    return result


def inverse_steps(document, mutation):
    """↩️ The steps that undo one application: a move-selection restores every BASE node and region it moves with
    one whole-record replacement each; every other kind undoes with its one computed inverse."""
    if kind_of(mutation) != "move-selection":
        return [inverse_mutation(document, mutation)]
    moved = apply_mutation(document, mutation)
    steps = [{"mutation": "replaceNode", "id": node["id"], "newNode": copy.deepcopy(node)} for node, after in zip(document["nodes"], moved["nodes"]) if node != after]
    steps += [{"mutation": "replaceRegion", "id": region["id"], "newRegion": copy.deepcopy(region)} for region, after in zip(document["regions"], moved["regions"]) if region != after]
    return steps


def apply_steps(document, steps):
    """🧮️ Applies every step in order."""
    for step in steps:
        document = apply_mutation(document, step)
    return document


'''
PY_TOUCHES_OLD = '''def touches_one(scenario, kind, before, after):
    """🔀️ Each verb writes exactly ONE of the nine members. That is the check an after-snapshot
    comparison cannot make on its own: an implementation that re-derived a sibling collection on
    every edit — renumbering ids, re-sorting sections — would still land on the right value for the
    member it meant to write."""
'''
PY_TOUCHES_NEW = PY_TOUCHES_OLD + '''    if kind == "move-selection":
        moved = [name for name in MEMBERS if before[name] != after[name]]
        if not moved or any(name not in ("nodes", "regions") for name in moved):
            raise AssertionError("%s: move-selection writes nodes and regions and nothing else, but %r moved" % (scenario, moved))
        return
'''


def patch_python(path, with_refusals):
    text = read(path)
    text = re.sub(r'^KINDS = \((.*)\)$', lambda match: match.group(0) if f'"{KIND}"' in match.group(1) else f'KINDS = ({match.group(1)}, "{KIND}")', text, count=1, flags=re.M)
    if with_refusals:
        count = len([row for row in scenarios() if row[2] == "reject"])
        text = re.sub(r'^REFUSALS = \{(.*)\}$', lambda match: match.group(0) if f'"{KIND}"' in match.group(1) else f'REFUSALS = {{{match.group(1)}, "{KIND}": {count}}}', text, count=1, flags=re.M)
    if "\nimport math\n" not in text:
        text = once(text, "\nimport copy\n", "\nimport copy\nimport math\n", "import math")
    text = once(text, '''    kind = kind_of(mutation)
    result = copy.deepcopy(document)
''', '''    kind = kind_of(mutation)
    result = copy.deepcopy(document)
''' + PY_APPLY, "apply dispatch")
    text = once(text, "def inverse_mutation(document, mutation):", PY_FUNCTIONS + "def inverse_mutation(document, mutation):", "functions")
    text = once(text, PY_TOUCHES_OLD, PY_TOUCHES_NEW, "touches_one")
    text = text.replace("restored = apply_mutation(applied, inverse_mutation(document, mutation))", "restored = apply_steps(applied, inverse_steps(document, mutation))")
    text = text.replace("restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)", "restores(kind, apply_steps(applied, inverse_steps(before, mutation)), before)")
    write(path, text)
# endregion 🐍️Python


# region 🥒️Feature
def table_rows(text, header):
    """📏️ The column widths of the Examples table that starts with `header`."""
    at = text.index(header)
    return at


def add_row(text, after_row, row):
    if row.strip() in text:
        return text
    assert text.count(after_row) >= 1, after_row
    return text.replace(after_row, after_row + row, 1)


def patch_feature(path, full):
    text = read(path)
    mutate_old = '    | replace-node    | {"mutation":"replaceNode","id":"ridge","newNode":{"id":"ridge","x":4.0,"y":8.2}}'
    lines = text.split("\n")
    out = []
    row = f"    | {KIND:<15} | {FEATURE_PAYLOAD} |"
    lines = [line for line in lines if line != row]
    for line in lines:
        out.append(line)
        if line.startswith(mutate_old):
            out.append(row)
    text = "\n".join(out)
    if full:
        rows = scenarios()
        text = add_row(text, "    | replace-node    | 🔁️replace-node    | 🕹️raises-the-ridge-e53b00        |\n", f"    | {KIND:<15} | {DIR:<17} | {rows[0][0]} |\n")
        text = add_row(text, "    | replace-node    | 🔁️replace-node    | 📍️widens-the-canopy-553d69  |\n", f"    | {KIND:<15} | {DIR:<17} | {rows[1][0]} |\n")
        rejects = "".join(f"    | {KIND}-{index:<3} | {DIR:<17} | {folder} |\n" for index, (folder, _, _) in enumerate([row for row in rows if row[2] == "reject"], 1))
        text = add_row(text, "    | replace-node-2    | 🔁️replace-node    | 🪪️denies-rename-e69720       |", "\n" + rejects.rstrip("\n"))
        text = text.replace("`replace-region`, `replace-node`) have a subset-owned test", "`replace-region`, `replace-node`, `move-selection`) have a subset-owned test")
    write(path, text)
# endregion 🥒️Feature


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
    mesh = next(catalog for catalog in data["mutationCatalogs"] if catalog["id"] == "fem2d-1-any-mesh")
    if KIND not in mesh["kinds"]:
        mesh["kinds"].append(KIND)
    manifests = data["mutationManifests"][0]["mutations"]
    if not any(manifest["id"] == KIND for manifest in manifests):
        template = json.loads(json.dumps(next(manifest for manifest in manifests if manifest["id"] == "replace-node")), object_pairs_hook=OrderedDict)
        template["id"] = KIND
        template["payloadSchema"] = f"../🧬️schema/🧬️mutations/{DIR}/🦀️.rs#MoveSelection"
        template["productionDispatch"] = OrderedDict([("operation", KIND), ("bridgeVersion", 1), ("variant", "MoveSelection")])
        template["oracleRequirements"] = [requirement for requirement in template["oracleRequirements"] if requirement["capability"] != "fem2d-1-mutate-carrier"]
        manifests.append(template)
    write(path, json.dumps(data, indent=2, ensure_ascii=False) + "\n")
# endregion 🔮️Oracles


def main():
    patch_mesh_rust(MESH_CASE + "/🦀️.rs")
    patch_any_rust(ANY_CASE + "/🦀️.rs")
    patch_python(MESH_CASE + "/🐍️.py", True)
    patch_python(ANY_CASE + "/🐍️.py", True)
    patch_feature(MESH_CASE + "/🥒️.feature", True)
    patch_feature(ANY_CASE + "/🥒️.feature", False)
    patch_mesh_oracle(SUBSETS + "/🕸️mesh/🔮️oracles/🔣️.json")
    patch_any_oracle(SUBSETS + "/🌐️any/🔮️oracles/🔣️.json")
    print("registered", KIND, [row[0] for row in scenarios()])


if __name__ == "__main__":
    main()
