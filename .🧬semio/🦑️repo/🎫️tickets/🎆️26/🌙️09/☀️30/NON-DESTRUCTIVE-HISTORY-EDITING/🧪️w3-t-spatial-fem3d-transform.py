#!/usr/bin/env python3
"""🧭️ W3-T-SPATIAL: the fem3d `move-selection` leaf's committed evidence, computed by an INDEPENDENT Python
implementation of its semantics (never by the Rust leaf it witnesses): the fixture bundles (before, mutation, after,
diff, outcome) of every scenario, the per-scenario Rust test mounts, and their registration in the crate root.

Idempotent. Semantics: `p' = c + R(axis, angle)·S(sx, sy, sz)·(p − c) + d` on every named node and every footprint
point of every named solid (a solid follows only a map that keeps its footprint plane and a positive height; one that
cannot is skipped). Guards in order — invariant (non-positive factor, rotation about the zero axis, a target named
twice), target-missing (none exist), no-op (nothing moves), partial (some missing or unfollowable). Committed vectors
use exact dyadic offsets and factors so both languages land on bit-identical doubles.
"""
import hashlib
import json
import math
import os

REPO = "/Users/ueli/Documents/semio"
ARTIFACT = REPO + "/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d"
MESH = ARTIFACT + "/🏅️standards/🔖️1/🪆️subsets/🕸️mesh"
FIXTURES = MESH + "/🧫️fixtures/🧬️mutations"
LEAF = MESH + "/🧬️schema/🧬️mutations/🧭️move-selection"
FRAME = FIXTURES + "/🔁️replace-node/📍️lifts-the-column-head-34351d/📸️snapshot/⬅️before/🔣️.json"
HALL = FIXTURES + "/🔁️replace-node/🏗️hall-lifts-ridge-746bae/📸️snapshot/⬅️before/🔣️.json"
KIND_DIR = "🧭️move-selection"
DIFF_MEMBERS = ["artifact", "nodes", "elements", "materials", "sections", "solids", "supports", "loadCases", "combinations", "analysis"]
AXES = {"x": lambda u, v, w: [w, u, v], "y": lambda u, v, w: [u, w, v], "z": lambda u, v, w: [u, v, w]}
PROJECT = {"x": lambda p: ([p[1], p[2]], p[0]), "y": lambda p: ([p[0], p[2]], p[1]), "z": lambda p: ([p[0], p[1]], p[2])}


def payload(node_ids, solid_ids, pivot=(0.0, 0.0, 0.0), offset=(0.0, 0.0, 0.0), axis=(0.0, 0.0, 1.0), angle=0.0, factors=(1.0, 1.0, 1.0)):
    return {"mutation": "moveSelection", "nodeIds": node_ids, "solidIds": solid_ids, "pivotX": pivot[0], "pivotY": pivot[1], "pivotZ": pivot[2], "dx": offset[0], "dy": offset[1], "dz": offset[2], "axisX": axis[0], "axisY": axis[1], "axisZ": axis[2], "angle": angle, "sx": factors[0], "sy": factors[1], "sz": factors[2]}


SCENARIOS = [
    {"emoji": "🧭️", "slug": "lifts-the-column-head", "module": "lifts_the_column_head_half_a_metre", "base": FRAME, "class": "spec",
     "story": "Lifting the column head n3 half a metre stretches the column it caps; the frame keeps naming the node, so the topology stays and the geometry travels.",
     "payload": payload(["n3"], [], pivot=(0.0, 0.0, 3.5), offset=(0.0, 0.0, 0.5))},
    {"emoji": "🏗️", "slug": "hall-stretches-the-apron", "module": "hall_stretches_the_apron_slab_to_twice_its_width", "base": HALL, "class": "hall",
     "story": "Stretching the portal hall's apron slab to twice its width about the right column line doubles its footprint in x; the gate node the hall never had is skipped with a partial warning.",
     "payload": payload(["gate_9"], ["slab_apron"], pivot=(12.0, 0.0, 0.0), factors=(2.0, 1.0, 1.0))},
    {"emoji": "🚨️", "slug": "no-such-targets", "module": "no_such_targets", "base": FRAME, "class": "reject",
     "story": "A transform resolves its targets on THIS base; when none of them exist there is nothing to move.",
     "payload": payload(["n9"], ["s9"], offset=(1.0, 0.0, 0.0))},
    {"emoji": "⏸️", "slug": "moves-nothing", "module": "an_identity_transform_moves_nothing", "base": FRAME, "class": "reject",
     "story": "The identity transform — no offset, no angle, unit factors — is a no-op on the targets it names.",
     "payload": payload(["n1"], [], pivot=(3.0, 0.0, 1.75))},
    {"emoji": "🫓️", "slug": "flattens-the-raft", "module": "refuses_a_scale_that_flattens_the_raft", "base": HALL, "class": "reject",
     "story": "A zero factor would flatten the raft to a plane no mesh can fill; the payload schema bounds every factor away from zero and the diff refuses it as an invariant.",
     "payload": payload([], ["slab_raft"], factors=(1.0, 1.0, 0.0))},
    {"emoji": "🌀️", "slug": "turns-about-nothing", "module": "refuses_a_rotation_about_the_zero_axis", "base": FRAME, "class": "reject", "invariant": "axis-non-zero",
     "story": "A rotation needs an axis; a non-zero angle about the zero axis has no meaning, which the schema declares as the axis-non-zero invariant.",
     "payload": payload(["n3"], [], axis=(0.0, 0.0, 0.0), angle=0.5)},
    {"emoji": "🔁️", "slug": "names-a-node-twice", "module": "refuses_a_transform_that_names_a_node_twice", "base": FRAME, "class": "reject",
     "story": "Naming a target twice would move it twice in any implementation that iterates the payload; the schema demands unique ids and the diff refuses the repeat as an invariant.",
     "payload": payload(["n2", "n2"], [], offset=(0.0, 0.5, 0.0))},
]


def folder(scenario):
    return f'{scenario["emoji"]}{scenario["slug"]}-{hashlib.sha256(scenario["story"].encode("utf-8")).hexdigest()[:6]}'


# region 🧮️Reference
def mapped(p, point):
    pivot = [p["pivotX"], p["pivotY"], p["pivotZ"]]
    s = [(point[0] - pivot[0]) * p["sx"], (point[1] - pivot[1]) * p["sy"], (point[2] - pivot[2]) * p["sz"]]
    if p["angle"] != 0.0:
        length = math.sqrt(p["axisX"] ** 2 + p["axisY"] ** 2 + p["axisZ"] ** 2)
        a = [p["axisX"] / length, p["axisY"] / length, p["axisZ"] / length]
        sin, cos = math.sin(p["angle"]), math.cos(p["angle"])
        cross = [a[1] * s[2] - a[2] * s[1], a[2] * s[0] - a[0] * s[2], a[0] * s[1] - a[1] * s[0]]
        along = (a[0] * s[0] + a[1] * s[1] + a[2] * s[2]) * (1.0 - cos)
        s = [s[i] * cos + cross[i] * sin + a[i] * along for i in range(3)]
    return [pivot[0] + s[0] + p["dx"], pivot[1] + s[1] + p["dy"], pivot[2] + s[2] + p["dz"]]


def map_solid(p, solid):
    lift, project = AXES[solid["axis"]], PROJECT[solid["axis"]]
    base = None

    def onto(point):
        nonlocal base
        uv, w = project(mapped(p, lift(point[0], point[1], solid["baseZ"])))
        if base is None:
            base = w
        elif abs(base - w) > 1e-9:
            return None
        return uv

    outline = [onto(point) for point in solid["outline"]]
    holes = [[onto(point) for point in hole] for hole in solid["holes"]]
    if any(point is None for point in outline) or any(point is None for hole in holes for point in hole):
        return None
    _, top = project(mapped(p, lift(solid["outline"][0][0], solid["outline"][0][1], solid["baseZ"] + solid["height"])))
    height = top - base
    if not height > 0.0:
        return None
    moved = dict(solid)
    moved.update({"outline": outline, "holes": holes, "baseZ": base, "height": height})
    return {key: moved[key] for key in solid}


def transform(document, p, invariant_id):
    targets = p["nodeIds"] + p["solidIds"]
    rejected = lambda code, path: (document, None, dict({"status": "rejected", "code": code, "path": path, "messages": [{"level": "fatal" if code == "mutation.invariant" else "error", "code": code}]}, **({"invariant": invariant_id} if invariant_id else {})))
    if p["sx"] <= 0.0 or p["sy"] <= 0.0 or p["sz"] <= 0.0:
        return rejected("mutation.invariant", targets)
    if p["angle"] != 0.0 and math.sqrt(p["axisX"] ** 2 + p["axisY"] ** 2 + p["axisZ"] ** 2) <= 1e-12:
        return rejected("mutation.invariant", targets)
    for ids in (p["nodeIds"], p["solidIds"]):
        for at, identifier in enumerate(ids):
            if identifier in ids[:at]:
                return rejected("mutation.invariant", [identifier])
    nodes = [node for node in document["nodes"] if node["id"] in p["nodeIds"]]
    solids = [solid for solid in document["solids"] if solid["id"] in p["solidIds"]]
    if not nodes and not solids:
        return rejected("mutation.target-missing", targets)
    after = json.loads(json.dumps(document))
    patched_nodes, patched_solids, unfollowed = [], [], []
    for node in after["nodes"]:
        if node["id"] in p["nodeIds"]:
            x, y, z = mapped(p, [node["x"], node["y"], node["z"]])
            if (x, y, z) != (node["x"], node["y"], node["z"]):
                node["x"], node["y"], node["z"] = x, y, z
                patched_nodes.append({"id": node["id"], "item": node})
    for at, solid in enumerate(after["solids"]):
        if solid["id"] in p["solidIds"]:
            moved = map_solid(p, solid)
            if moved is None:
                unfollowed.append(solid["id"])
            elif moved != solid:
                after["solids"][at] = moved
                patched_solids.append({"id": moved["id"], "item": moved})
    if not patched_nodes and not patched_solids:
        return document, None, {"status": "no-op", "messages": [{"level": "warning", "code": "mutation.no-op"}]}
    diff = {member: None for member in DIFF_MEMBERS}
    if patched_nodes:
        diff["nodes"] = {"added": [], "removed": [], "patched": patched_nodes, "reordered": None}
    if patched_solids:
        diff["solids"] = {"added": [], "removed": [], "patched": patched_solids, "reordered": None}
    present = {node["id"] for node in nodes}
    skipped = [identifier for identifier in p["nodeIds"] if identifier not in present] + [identifier for identifier in p["solidIds"] if identifier not in {solid["id"] for solid in solids} or identifier in unfollowed]
    return after, diff, {"status": "applied", "messages": [{"level": "warning", "code": "mutation.partial", "target": skipped}] if skipped else []}
# endregion 🧮️Reference


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def text(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(value)


def test_file(scenario, name, diff):
    rel = f"../../../../../🧫️fixtures/🧬️mutations/{KIND_DIR}/{name}"
    constants = [f'const BEFORE: &str = include_str!("{rel}/📸️snapshot/⬅️before/🔣️.json");', f'const AFTER: &str = include_str!("{rel}/📸️snapshot/➡️after/🔣️.json");', f'const MUTATION: &str = include_str!("{rel}/🦠️mutation/🔣️.json");']
    if diff is not None:
        constants.append(f'const DIFF: &str = include_str!("{rel}/🔺️diff/🔣️.json");')
    constants.append(f'const OUTCOME: &str = include_str!("{rel}/🎯️outcome/🔣️.json");')
    header = f"""//! 🧪️ `move-selection` fixture — `{name}`.
//!
//! Source of truth is the committed JSON bundle, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem3d-transform.py`.
//!
//! {scenario["emoji"]} {scenario["story"]}

use super::laws;

{chr(10).join(constants)}
"""
    if diff is not None:
        return header + """
/// ▶️ The leaf carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}

/// ↩️ The computed inverse — whole-record replacements of every moved node and solid — restores `before` exactly.
#[test]
fn inverse_restores_before() {
    laws::inverse_restores(BEFORE, MUTATION);
}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
#[test]
fn declared_outcome_holds() {
    laws::declared_outcome(BEFORE, MUTATION, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, Some(DIFF));
}
"""
    return header + """
/// ⛔️ The refused or no-op leaf leaves the document byte-identical, emits the declared diagnostic and inverts to nothing.
#[test]
fn refusal_leaves_the_document_untouched() {
    laws::refusal(BEFORE, MUTATION, AFTER, OUTCOME);
}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {
    laws::canonical(BEFORE, AFTER, MUTATION, None);
}
"""


def main():
    mounts = []
    for scenario in SCENARIOS:
        name = folder(scenario)
        with open(scenario["base"], encoding="utf-8") as handle:
            before = json.load(handle)
        after, diff, outcome = transform(before, scenario["payload"], scenario.get("invariant"))
        root = f"{FIXTURES}/{KIND_DIR}/{name}"
        dump(f"{root}/📸️snapshot/⬅️before/🔣️.json", before)
        dump(f"{root}/📸️snapshot/➡️after/🔣️.json", after)
        dump(f"{root}/🦠️mutation/🔣️.json", scenario["payload"])
        dump(f"{root}/🎯️outcome/🔣️.json", outcome)
        if outcome["status"] == "no-op":
            dump(f"{root}/🔺️diff/🔣️.json", {member: None for member in DIFF_MEMBERS})
        elif diff is None:
            text(f"{root}/🔺️diff/🚫️.absent", "")
        else:
            dump(f"{root}/🔺️diff/🔣️.json", diff)
        text(f"{LEAF}/🧪️tests/{name}/🦀️.rs", test_file(scenario, name, diff))
        mounts.append((name, scenario["module"]))
        print(name, outcome["status"], outcome.get("code", [message["code"] for message in outcome.get("messages", [])]))
    lines = []
    for name, module in mounts:
        lines += ["                            #[cfg(test)]", f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/🧪️tests/{name}/🦀️.rs"]', f"                            mod tests_{module};"]
    block = "\n".join(["                        #[path = \".\"]", "                        pub mod move_selection {",
                       f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/🦀️.rs"]', "                            mod component;",
                       f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/🔺️diff/🦀️.rs"]', "                            pub mod diff;",
                       f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/↩️inverse/🦀️.rs"]', "                            pub mod inverse;",
                       "                            pub use component::*;", *lines, "                        }"]) + "\n"
    root_file = ARTIFACT + "/🦀️.rs"
    with open(root_file, encoding="utf-8") as handle:
        source = handle.read()
    start = source.find("                        #[path = \".\"]\n                        pub mod move_selection {")
    if start >= 0:
        end = source.index("                        }\n", source.index("pub mod move_selection {", start)) + len("                        }\n")
        source = source[:start] + block + source[end:]
    else:
        anchor = "                            mod tests_renames_node;\n                        }\n"
        assert source.count(anchor) == 1, "replace_node mount anchor"
        source = source.replace(anchor, anchor + block, 1)
    with open(root_file, "w", encoding="utf-8") as handle:
        handle.write(source)


if __name__ == "__main__":
    main()
