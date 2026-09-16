#!/usr/bin/env python3
"""🧪 Registers the four new fem3d mutation kinds with every cross-language harness that enumerates
the vocabulary: the subset-owned and `✳️any`-owned Rust subject adapters (`🦀️.rs`), the Gherkin
features (`🥒️.feature`) and the Python references (`🐍️.py`) of `🕸️mesh` and `🏋️load`.

Idempotent: every insertion is guarded by a marker check, so re-running changes nothing.
"""
import os
import re

ROOT = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/"
MESH_FIX = ROOT + "🕸️mesh/🧫️fixtures/🧬️mutations/"
LOAD_FIX = ROOT + "🏋️load/🧫️fixtures/🧬️mutations/"

STEEL_FRAME_PARAMS = {
    "replace-node": '{"mutation":"replaceNode","id":"n3","newNode":{"id":"n3","x":0.0,"y":0.0,"z":4.0}}',
    "replace-load": '{"mutation":"replaceLoad","caseId":"live","loadId":"l2","newLoad":{"kind":"nodal","id":"l2","nodeId":"n20_l1","dof":"Tz","value":-7500.0}}',
    "change-load-case-name": '{"mutation":"changeLoadCaseName","caseId":"wind","newName":"Wind +X"}',
    "replace-combination": '{"mutation":"replaceCombination","id":"uls","newCombination":{"id":"uls","name":"ULS","terms":{"dead":1.35,"live":1.5,"wind":0.9}}}',
}
KIND_DIRS = {"replace-node": "🔁️replace-node", "replace-load": "🔁️replace-load", "change-load-case-name": "🏷️change-load-case-name", "replace-combination": "🔁️replace-combination"}
MESH_KINDS = ["replace-node"]
LOAD_KINDS = ["replace-load", "change-load-case-name", "replace-combination"]


def scenarios(fixtures, kind):
    """📂 `(folder, scenario-id, class)` per committed scenario, class ∈ spec/hall/reject."""
    rows = []
    for folder in sorted(os.listdir(fixtures + KIND_DIRS[kind])):
        identifier = re.sub(r"^[^\w]+", "", folder)
        if folder.startswith("🏗️"):
            klass = "hall"
        elif folder.startswith(("🚨️", "🪪️", "⏸️")):
            klass = "reject"
        else:
            klass = "spec"
        rows.append((folder, identifier, klass))
    return rows


def read(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def write(path, text):
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(text)


def replace_once(text, old, new, label):
    if new in text:
        return text
    assert old in text, f"{label}: anchor not found: {old[:80]!r}"
    return text.replace(old, new, 1)


# region 🔖️Rust
def vector_arm(kind, folder, prefix, with_diff):
    base = f'{prefix}{KIND_DIRS[kind]}/{folder}/'
    lines = [f'        "{{key}}" => Vector {{', f'            before: include_str!("{base}📸️snapshot/⬅️before/🔣️.json"),', f'            mutation: include_str!("{base}🦠️mutation/🔣️.json"),', f'            after: include_str!("{base}📸️snapshot/➡️after/🔣️.json"),']
    lines.append(f'            diff: include_str!("{base}🔺️diff/🔣️.json"),' if with_diff else '            diff: "{}",')
    lines.append(f'            outcome: include_str!("{base}🎯️outcome/🔣️.json"),')
    lines.append("        },")
    return "\n".join(lines) + "\n"


def patch_rust(path, kinds, fixtures, prefix, with_hall):
    text = read(path)
    for kind in kinds:
        rows = scenarios(fixtures, kind)
        spec = [row for row in rows if row[2] == "spec"]
        hall = [row for row in rows if row[2] == "hall"]
        rejects = [row for row in rows if row[2] == "reject"]
        assert len(spec) == 1 and len(hall) == 1, (kind, rows)
        kinds_line = re.search(r"^const KINDS: &\[&str\] = &\[(.*)\];$", text, re.M)
        if f'"{kind}"' not in kinds_line.group(1):
            text = text.replace(kinds_line.group(0), kinds_line.group(0)[:-2] + f', "{kind}"];', 1)
        spec_arm = vector_arm(kind, spec[0][0], prefix, True).replace("{key}", kind)
        anchor = re.search(r'\n        other => panic!\("[^"]*no committed specification vector[^\n]*\n', text).group(0)
        if spec_arm not in text:
            text = text.replace(anchor, "\n" + spec_arm + anchor.lstrip("\n"), 1)
        if with_hall:
            hall_arm = vector_arm(kind, hall[0][0], prefix, True).replace("{key}", kind)
            anchor = re.search(r'\n        other => panic!\("[^"]*no committed hall vector[^\n]*\n', text).group(0)
            if hall_arm not in text:
                text = text.replace(anchor, "\n" + hall_arm + anchor.lstrip("\n"), 1)
            rejects_line = re.search(r"^const REJECT_VECTORS: &\[&str\] = &\[(.*)\];$", text, re.M)
            for folder, identifier, _ in rejects:
                if f'"{identifier}"' not in rejects_line.group(1):
                    current = re.search(r"^const REJECT_VECTORS: &\[&str\] = &\[(.*)\];$", text, re.M).group(0)
                    text = text.replace(current, current[:-2] + f', "{identifier}"];', 1)
                arm = vector_arm(kind, folder, prefix, folder.startswith("⏸️")).replace("{key}", identifier)
                anchor = re.search(r'\n        other => panic!\("[^"]*no committed rejection vector[^\n]*\n', text).group(0)
                if arm not in text:
                    text = text.replace(anchor, "\n" + arm + anchor.lstrip("\n"), 1)
    write(path, text)


# endregion 🔖️Rust


# region 🔖️Feature
def table_rows(text, tag):
    """📋 The Examples table of the `@id-<tag>` outline: (start, end, rows) over the raw text."""
    start = text.index(f"  @id-{tag}\n")
    examples = text.index("    Examples:\n", start) + len("    Examples:\n")
    end = examples
    while end < len(text) and text[end:end + 6] == "    | ":
        end = text.index("\n", end) + 1
    return examples, end


def format_row(cells, widths):
    return "    | " + " | ".join(cell.ljust(width) for cell, width in zip(cells, widths)) + " |\n"


def append_rows(text, tag, new_rows):
    start, end = table_rows(text, tag)
    rows = [line for line in text[start:end].splitlines()]
    cells = [[cell.strip() for cell in row.strip().strip("|").split(" | ")] for row in rows]
    existing_ids = {row[0] for row in cells}
    additions = [row for row in new_rows if row[0] not in existing_ids]
    if not additions:
        return text
    cells.extend(additions)
    widths = [max(len(row[at]) for row in cells) for at in range(len(cells[0]))]
    rebuilt = "".join(format_row(row, widths) for row in cells)
    return text[:start] + rebuilt + text[end:]


def patch_feature(path, kinds, fixtures, with_hall):
    text = read(path)
    for kind in kinds:
        rows = scenarios(fixtures, kind)
        text = append_rows(text, "mutate", [[kind, STEEL_FRAME_PARAMS[kind]]])
        text = append_rows(text, "inverse", [[kind, STEEL_FRAME_PARAMS[kind]]])
        if with_hall:
            spec = [row for row in rows if row[2] == "spec"][0]
            hall = [row for row in rows if row[2] == "hall"][0]
            text = append_rows(text, "spec-vector", [[kind, KIND_DIRS[kind], spec[0]]])
            text = append_rows(text, "hall-vector", [[kind, KIND_DIRS[kind], hall[0]]])
            text = append_rows(text, "reject", [[identifier, KIND_DIRS[kind], folder] for folder, identifier, klass in rows if klass == "reject"])
    write(path, text)


# endregion 🔖️Feature


# region 🔖️Python
APPLY_BRANCHES = '''    elif kind == "replace-load":
        case = case_of(result, mutation["caseId"], kind)
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("%s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        load = copy.deepcopy(mutation["newLoad"])
        if load["id"] != mutation["loadId"]:
            raise AssertionError("%s: a replace selects %r and may not rename it to %r" % (kind, mutation["loadId"], load["id"]))
        if case["loads"][at] != load:
            resolve_load(result, load, kind)
        case["loads"][at] = load
    elif kind == "change-load-case-name":
        case_of(result, mutation["caseId"], kind)["name"] = mutation["newName"]
'''

INVERSE_BRANCHES = '''    if kind == "replace-load":
        case = case_of(document, mutation["caseId"], "inverse of %s" % kind)
        at = find(case["loads"], mutation["loadId"])
        if at is None:
            raise AssertionError("inverse of %s: case %r carries no load %r" % (kind, case["id"], mutation["loadId"]))
        return {"mutation": TAGS[kind], "caseId": mutation["caseId"], "loadId": mutation["loadId"], "newLoad": copy.deepcopy(case["loads"][at])}
    if kind == "change-load-case-name":
        return {"mutation": TAGS[kind], "caseId": mutation["caseId"], "newName": case_of(document, mutation["caseId"], "inverse of %s" % kind)["name"]}
'''

RESOLVE_LOAD = '''

def resolve_load(document, load, kind):
    """🎯️ A load names exactly one carrier — a node, an element or a solid — and it must be there."""
    if load["kind"] == "nodal":
        target, collection = load["nodeId"], "nodes"
    elif load["kind"] == "memberUdl":
        target, collection = load["elementId"], "elements"
    else:
        target, collection = load["solidId"], "solids"
    if find(document[collection], target) is None:
        raise AssertionError("%s: load %r names %r, which is not in %s" % (kind, load["id"], target, collection))
'''


def patch_python(path, kinds, fixtures):
    text = read(path)
    kinds_line = re.search(r"^KINDS = \((.*)\)$", text, re.M)
    for kind in kinds:
        if f'"{kind}"' not in kinds_line.group(1):
            current = re.search(r"^KINDS = \((.*)\)$", text, re.M).group(0)
            text = text.replace(current, current[:-1] + f', "{kind}")', 1)
    text = replace_once(text, '    "combination": ("combinations", "combination", None),', '    "combination": ("combinations", "combination", "newCombination"),', "collections")
    if any(kind in ("replace-load", "change-load-case-name") for kind in kinds):
        text = replace_once(text, '    else:\n        noun = noun_of(kind)\n        collection, create_argument, replace_argument = COLLECTIONS[noun]\n        items = result[collection]\n', APPLY_BRANCHES + '    else:\n        noun = noun_of(kind)\n        collection, create_argument, replace_argument = COLLECTIONS[noun]\n        items = result[collection]\n', "apply")
        text = replace_once(text, '    noun = noun_of(kind)\n    collection, create_argument, replace_argument = COLLECTIONS[noun]\n    if kind.startswith("create-"):', INVERSE_BRANCHES + '    noun = noun_of(kind)\n    collection, create_argument, replace_argument = COLLECTIONS[noun]\n    if kind.startswith("create-"):', "inverse")
        text = replace_once(text, '    elif kind in ("add-load", "remove-load", "change-load-case-self-weight"):\n        written = "loadCases"', '    elif kind in ("add-load", "remove-load", "replace-load", "change-load-case-self-weight", "change-load-case-name"):\n        written = "loadCases"', "touches")
        if "def resolve_load(" not in text:
            text = replace_once(text, "\n\ndef apply_mutation(document, mutation):", RESOLVE_LOAD + "\n\ndef apply_mutation(document, mutation):", "resolve_load")
    rejects_tuple = re.search(r"^REJECT_VECTORS = \(\n((?:    \"[^\"]+\",\n)+)\)$", text, re.M)
    if rejects_tuple:
        for kind in kinds:
            for folder, identifier, klass in scenarios(fixtures, kind):
                if klass == "reject" and f'"{identifier}"' not in text:
                    current = re.search(r"^REJECT_VECTORS = \(\n((?:    \"[^\"]+\",\n)+)\)$", text, re.M)
                    text = text.replace(current.group(0), current.group(0)[:-1] + f'    "{identifier}",\n)', 1)
    write(path, text)


# endregion 🔖️Python


def main():
    mesh = ROOT + "🕸️mesh/🧪️tests/🕸️mutate-fem3d-1-mesh/"
    load = ROOT + "🏋️load/🧪️tests/🏋️mutate-fem3d-1-load/"
    any_mesh = ROOT + "🌐️any/🧪️tests/🕸️mutate-fem3d-1-any-mesh/"
    any_load = ROOT + "🌐️any/🧪️tests/🏋️mutate-fem3d-1-any-load/"
    patch_rust(mesh + "🦀️.rs", MESH_KINDS, MESH_FIX, "../../🧫️fixtures/🧬️mutations/", True)
    patch_rust(load + "🦀️.rs", LOAD_KINDS, LOAD_FIX, "../../🧫️fixtures/🧬️mutations/", True)
    patch_rust(any_mesh + "🦀️.rs", MESH_KINDS, MESH_FIX, "../../../🕸️mesh/🧫️fixtures/🧬️mutations/", False)
    patch_rust(any_load + "🦀️.rs", LOAD_KINDS, LOAD_FIX, "../../../🏋️load/🧫️fixtures/🧬️mutations/", False)
    patch_feature(mesh + "🥒️.feature", MESH_KINDS, MESH_FIX, True)
    patch_feature(load + "🥒️.feature", LOAD_KINDS, LOAD_FIX, True)
    patch_feature(any_mesh + "🥒️.feature", MESH_KINDS, MESH_FIX, False)
    patch_feature(any_load + "🥒️.feature", LOAD_KINDS, LOAD_FIX, False)
    patch_python(mesh + "🐍️.py", MESH_KINDS, MESH_FIX)
    patch_python(load + "🐍️.py", LOAD_KINDS, LOAD_FIX)
    patch_python(any_mesh + "🐍️.py", MESH_KINDS, MESH_FIX)
    patch_python(any_load + "🐍️.py", LOAD_KINDS, LOAD_FIX)
    print("harnesses patched")


if __name__ == "__main__":
    main()
