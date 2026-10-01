#!/usr/bin/env python3
"""🧭️ W3-T-SPATIAL: the fem2d `move-selection` leaf's committed evidence, computed by an INDEPENDENT Python
implementation of its semantics (never by the Rust leaf it witnesses): the fixture quintets (before, mutation, after,
diff, outcome) of every scenario, the per-scenario Rust test mounts, and their registration in the crate root.

Idempotent: files are rewritten from the same inputs, root insertions are marker-guarded. Semantics:
`p' = c + R(angle)·S(sx, sy)·(p − c) + d` on every named node and every outline and hole point of every named
region; guards in order — invariant (non-positive factor, a target named twice), target-missing (none exist),
no-op (nothing moves), partial (some missing, warning addressed at the missing ids). Committed vectors use exact
dyadic offsets and factors so both languages land on bit-identical doubles.
"""
import hashlib
import json
import math
import os

REPO = "/Users/ueli/Documents/semio"
ARTIFACT = REPO + "/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d"
MESH = ARTIFACT + "/🏅️standards/🔖️1/🪆️subsets/🕸️mesh"
FIXTURES = MESH + "/🧫️fixtures/🧬️mutations"
LEAF = MESH + "/🧬️schema/🧬️mutations/🧭️move-selection"
TIMBER = FIXTURES + "/🔁️replace-node/🕹️raises-the-ridge-e53b00/📸️snapshot/⬅️before/🔣️.json"
STEEL = FIXTURES + "/🔁️replace-node/📍️widens-the-canopy-553d69/📸️snapshot/⬅️before/🔣️.json"
KIND_DIR = "🧭️move-selection"
DIFF_MEMBERS = ["artifact", "nodes", "elements", "regions", "materials", "sections", "supports", "loadCases", "combinations", "analysis"]

TIMBER_PROSE = """//! 🪵️ The model is the timber portal frame every fem2d mutation subset case shares (8.0 m span, eaves at 5.6 m,
//! ridge at 7.6 m, an RC first-floor slab on four corner nodes and one spare region), in SI base units."""
STEEL_PROSE = """//! 🏢️ The model is the two-storey braced steel frame (6.0 m bay, 3.5 m storeys, HEB 200 columns,
//! IPE 270/IPE 240 beams, a CHS 88.9x4.0 brace, an RC infill panel with a window opening), the
//! SECOND real-world fem2d model — the first is the timber portal frame the subset-level
//! differential cases share. Every value is in SI base units."""


def payload(node_ids, region_ids, pivot=(0.0, 0.0), offset=(0.0, 0.0), angle=0.0, factors=(1.0, 1.0)):
    return {"mutation": "moveSelection", "nodeIds": node_ids, "regionIds": region_ids, "pivotX": pivot[0], "pivotY": pivot[1], "dx": offset[0], "dy": offset[1], "angle": angle, "sx": factors[0], "sy": factors[1]}


SCENARIOS = [
    {"emoji": "🧭️", "slug": "shifts-the-slab", "module": "shifts_the_first_floor_slab_and_its_corner_nodes_right_and_up", "base": TIMBER, "prose": TIMBER_PROSE, "class": "spec",
     "story": "Shifting the first-floor slab 0.5 m right and 0.25 m up carries its outline and its four corner nodes together; every element on those nodes travels along.",
     "payload": payload(["rc0", "rc1", "rc2", "rc3"], ["r1"], pivot=(11.0, 2.8), offset=(0.5, 0.25))},
    {"emoji": "📏️", "slug": "stretches-the-panel", "module": "stretches_the_spare_panel_and_the_canopy_tip_to_twice_their_width", "base": STEEL, "prose": STEEL_PROSE, "class": "frame",
     "story": "Stretching to twice the width about the right column line doubles the spare side panel and the canopy cantilever; the node the frame never had is skipped with a partial warning.",
     "payload": payload(["n7", "n99"], ["panel_spare"], pivot=(6.0, 3.5), factors=(2.0, 1.0))},
    {"emoji": "⛔️", "slug": "rejects-a-missing", "module": "rejects_transforming_a_node_and_a_region_the_steel_frame_never_had", "base": STEEL, "prose": STEEL_PROSE, "class": "reject",
     "story": "A transform resolves its targets on THIS base; when none of them exist there is nothing to move.",
     "payload": payload(["n99"], ["r9"], offset=(1.0, 0.0))},
    {"emoji": "⏸️", "slug": "moves-nothing", "module": "an_identity_transform_moves_nothing", "base": STEEL, "prose": STEEL_PROSE, "class": "reject",
     "story": "The identity transform — no offset, no angle, unit factors — is a no-op on the targets it names.",
     "payload": payload(["n1"], [], pivot=(3.0, 1.75))},
    {"emoji": "🫓️", "slug": "denies-a-flat-scale", "module": "refuses_a_scale_that_flattens_the_wall_to_zero_height", "base": STEEL, "prose": STEEL_PROSE, "class": "reject",
     "story": "A zero factor would collapse the wall to a line no mesh can fill; the payload schema bounds the factor away from zero and the diff refuses it as an invariant.",
     "payload": payload([], ["wall1"], pivot=(3.0, 0.0), factors=(1.0, 0.0))},
    {"emoji": "🔁️", "slug": "denies-a-twice-named", "module": "refuses_a_transform_that_names_a_node_twice", "base": STEEL, "prose": STEEL_PROSE, "class": "reject",
     "story": "Naming a target twice would move it twice in any implementation that iterates the payload; the schema demands unique ids and the diff refuses the repeat as an invariant.",
     "payload": payload(["n3", "n3"], [], offset=(0.0, 0.5))},
]


def folder(scenario):
    digest = hashlib.sha256(scenario["story"].encode("utf-8")).hexdigest()[:6]
    return f'{scenario["emoji"]}{scenario["slug"]}-{digest}'


# region 🧮️Reference
def mapped(p, x, y):
    s, c = math.sin(p["angle"]), math.cos(p["angle"])
    u, v = (x - p["pivotX"]) * p["sx"], (y - p["pivotY"]) * p["sy"]
    return p["pivotX"] + u * c - v * s + p["dx"], p["pivotY"] + u * s + v * c + p["dy"]


def transform(document, p):
    """🧮️ `(after, diff, outcome, messages)` of one move-selection on `document`."""
    targets = p["nodeIds"] + p["regionIds"]
    if p["sx"] <= 0.0 or p["sy"] <= 0.0:
        return document, None, {"status": "rejected", "code": "mutation.invariant", "path": targets}
    for ids in (p["nodeIds"], p["regionIds"]):
        for at, identifier in enumerate(ids):
            if identifier in ids[:at]:
                return document, None, {"status": "rejected", "code": "mutation.invariant", "path": [identifier]}
    nodes = [node for node in document["nodes"] if node["id"] in p["nodeIds"]]
    regions = [region for region in document["regions"] if region["id"] in p["regionIds"]]
    if not nodes and not regions:
        return document, None, {"status": "rejected", "code": "mutation.target-missing", "path": targets}
    after = json.loads(json.dumps(document))
    patched_nodes, patched_regions = [], []
    for node in after["nodes"]:
        if node["id"] in p["nodeIds"]:
            x, y = mapped(p, node["x"], node["y"])
            if (x, y) != (node["x"], node["y"]):
                node["x"], node["y"] = x, y
                patched_nodes.append({"id": node["id"], "item": node})
    for region in after["regions"]:
        if region["id"] in p["regionIds"]:
            outline = [list(mapped(p, *point)) for point in region["outline"]]
            holes = [[list(mapped(p, *point)) for point in hole] for hole in region["holes"]]
            if outline != region["outline"] or holes != region["holes"]:
                region["outline"], region["holes"] = outline, holes
                patched_regions.append({"id": region["id"], "item": region})
    if not patched_nodes and not patched_regions:
        return document, None, {"status": "no-op", "messages": [{"level": "warning", "code": "mutation.no-op"}]}
    diff = {member: None for member in DIFF_MEMBERS}
    if patched_nodes:
        diff["nodes"] = {"added": [], "removed": [], "patched": patched_nodes, "reordered": None}
    if patched_regions:
        diff["regions"] = {"added": [], "removed": [], "patched": patched_regions, "reordered": None}
    present = {node["id"] for node in nodes} | {region["id"] for region in regions}
    missing = [identifier for identifier in targets if identifier not in present]
    messages = [{"level": "warning", "code": "mutation.partial", "target": missing}] if missing else []
    return after, diff, {"status": "applied", "messages": messages}
# endregion 🧮️Reference


def dump(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(json.dumps(value, indent=2, ensure_ascii=False) + "\n")


def text(path, value):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        handle.write(value)


def test_file(scenario, name, outcome, diff):
    rel = f"../../../../../🧫️fixtures/🧬️mutations/{KIND_DIR}/{name}"
    constants = [f'const BEFORE: &str = include_str!("{rel}/📸️snapshot/⬅️before/🔣️.json");', f'const AFTER: &str = include_str!("{rel}/📸️snapshot/➡️after/🔣️.json");', f'const MUTATION: &str = include_str!("{rel}/🦠️mutation/🔣️.json");']
    if diff is not None:
        constants.append(f'const DIFF: &str = include_str!("{rel}/🔺️diff/🔣️.json");')
    constants.append(f'const OUTCOME: &str = include_str!("{rel}/🎯️outcome/🔣️.json");')
    header = f"""//! 🧪️ `move-selection` fixture — `{name}`.
//!
//! Source of truth is the committed JSON quintet, computed by the independent Python reference
//! `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️30/NON-DESTRUCTIVE-HISTORY-EDITING/🧪️w3-t-spatial-fem2d-transform.py`.
//!
{scenario["prose"]}
//!
//! {scenario["emoji"]} {scenario["story"]}

use super::laws;

{chr(10).join(constants)}
"""
    if diff is not None:
        body = f"""
/// ▶️ The leaf carries `before` to exactly the committed `after` and produces exactly the committed delta.
#[test]
fn applies_to_committed_after_with_the_committed_diff() {{
    laws::forward(BEFORE, MUTATION, AFTER, DIFF);
}}

/// ↩️ The computed inverse — whole-record replacements of every moved node and region — restores `before` exactly.
#[test]
fn inverse_restores_before() {{
    laws::inverse_restores(BEFORE, MUTATION);
}}

/// 🎯️ The declared outcome — status and ordered diagnostics — is what the leaf emits.
#[test]
fn declared_outcome_holds() {{
    laws::declared_outcome(BEFORE, MUTATION, OUTCOME);
}}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    laws::canonical(BEFORE, AFTER, MUTATION, Some(DIFF));
}}
"""
    else:
        body = f"""
/// ⛔️ The refused or no-op leaf leaves the document byte-identical, emits the declared diagnostic and inverts to nothing.
#[test]
fn refusal_leaves_the_document_untouched() {{
    laws::refusal(BEFORE, MUTATION, AFTER, OUTCOME);
}}

/// 🔣️ Every committed JSON file is canonical: decode→encode is a fixed point.
#[test]
fn committed_json_is_canonical() {{
    laws::canonical(BEFORE, AFTER, MUTATION, None);
}}
"""
    return header + body


def main():
    mounts = []
    for scenario in SCENARIOS:
        name = folder(scenario)
        with open(scenario["base"], encoding="utf-8") as handle:
            before = json.load(handle)
        after, diff, outcome = transform(before, scenario["payload"])
        root = f"{FIXTURES}/{KIND_DIR}/{name}"
        dump(f"{root}/📸️snapshot/⬅️before/🔣️.json", before)
        dump(f"{root}/📸️snapshot/➡️after/🔣️.json", after)
        dump(f"{root}/🦠️mutation/🔣️.json", scenario["payload"])
        dump(f"{root}/🎯️outcome/🔣️.json", outcome)
        if diff is None:
            text(f"{root}/🔺️diff/🚫️.absent", "")
        else:
            dump(f"{root}/🔺️diff/🔣️.json", diff)
        text(f"{LEAF}/🧪️tests/{name}/🦀️.rs", test_file(scenario, name, outcome, diff))
        mounts.append((name, scenario["module"]))
        print(name, outcome["status"], outcome.get("code", [m["code"] for m in outcome.get("messages", [])]))
    lines = []
    for name, module in mounts:
        lines.append("                            #[cfg(test)]")
        lines.append(f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/🧪️tests/{name}/🦀️.rs"]')
        lines.append(f"                            mod tests_{module};")
    block = "\n".join([
        "                        #[path = \".\"]",
        "                        pub mod move_selection {",
        f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/🦀️.rs"]',
        "                            mod component;",
        f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/🔺️diff/🦀️.rs"]',
        "                            pub mod diff;",
        f'                            #[path = "🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/{KIND_DIR}/↩️inverse/🦀️.rs"]',
        "                            pub mod inverse;",
        "                            pub use component::*;",
        *lines,
        "                        }",
    ]) + "\n"
    root_file = ARTIFACT + "/🦀️.rs"
    with open(root_file, encoding="utf-8") as handle:
        source = handle.read()
    start = source.find("                        #[path = \".\"]\n                        pub mod move_selection {")
    if start >= 0:
        end = source.index("                        }\n", source.index("pub mod move_selection {", start)) + len("                        }\n")
        source = source[:start] + block + source[end:]
    else:
        anchor = "                            mod tests_refuses_to_relabel_a_steel_frame_node_through_a_replace_node;\n                        }\n"
        assert source.count(anchor) == 1, "replace_node mount anchor"
        source = source.replace(anchor, anchor + block, 1)
    with open(root_file, "w", encoding="utf-8") as handle:
        handle.write(source)


if __name__ == "__main__":
    main()
