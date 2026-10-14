#!/usr/bin/env python3
"""🗂️ Moves the CAD model-definition asset embeds out of the interaction and typology engines into per-domain catalog
modules (`✏️editor/🗂️catalog/<group>/🦀️.rs`), one data crate each after the family cut, so no compilation unit embeds
more than its 262,144-byte / 32-file share. Groups follow the model-definition directories; `spatial.shape`'s
interactions split by the category segment of the typology they produce (curve, surface/feature, solid/primitive,
edit/transform/measure). The engines concatenate the catalog slices in their original order.

usage: 🗂️catalogs.py plan|apply
"""
import collections, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 9))
ART = os.path.join(REPO, "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad")
SUB = os.path.join(ART, "🏅️standards/🔖️1/🪆️subsets/✳️any")
ENGINE = os.path.join(SUB, "✏️editor/⚙️engine")
CATALOG = os.path.join(SUB, "✏️editor/🗂️catalog")
DEFS = os.path.join(SUB, "📚️examples/🖼️assets/🏗️modelDefinitions")
GROUPS = collections.OrderedDict([
    ("structure", ("🏛️structure", "🏛️ Structural model definitions: the classic reinforced-concrete structure, its attribute definitions and the concrete definition.")),
    ("building", ("🏢️building", "🏢️ The `aec.building` model definition: its typologies, attribute definitions and interactions.")),
    ("fem", ("📏️fem", "📏️ The structural FEM model definitions: line, surface and solid elements.")),
    ("energy", ("🔥️energy", "🔥️ The `aec.building.energy` model definition: its typologies, attribute definitions and interactions.")),
    ("shape", ("📐️shape", "📐️ The `spatial.shape` model definition and its typologies.")),
    ("shape_curves", ("〰️shape-curves", "〰️ The `spatial.shape` curve interactions.")),
    ("shape_surfaces", ("🗺️shape-surfaces", "🗺️ The `spatial.shape` surface and feature interactions.")),
    ("shape_solids", ("🧊️shape-solids", "🧊️ The `spatial.shape` solid and primitive interactions.")),
    ("shape_edits", ("✂️shape-edits", "✂️ The `spatial.shape` edit, transform, measure and anchor interactions.")),
])
DIR_GROUP = {"🌉️aec.building": "structure", "🏛️aec.building.structure": "structure", "🧱️aec.building.concrete": "structure", "🏢️aec.building": "building",
             "📏️aec.building": "fem", "🗺️aec.building": "fem", "🧊️aec.building": "fem", "🔥️aec.building.energy": "energy", "📐️spatial.shape": "shape"}
SHAPE_CATEGORY = {"curve": "shape_curves", "surface": "shape_surfaces", "feature": "shape_surfaces", "solid": "shape_solids", "primitive": "shape_solids",
                  "edit": "shape_edits", "transform": "shape_edits", "measure": "shape_edits"}
SHAPE_UNPRODUCED = {"〰️constructCurve.json": "shape_curves", "🗺️constructSurface.json": "shape_surfaces", "⚓️createAnchor.json": "shape_edits"}
KINDS = [("interaction", "🕹️interaction/🦀️.rs", "RAW_INTERACTION_ASSETS", "INTERACTIONS", True),
         ("typology", "🧬️typology/🦀️.rs", "RAW_MODEL_DEFINITION_ASSETS", "MODEL_DEFINITIONS", False),
         ("typology", "🧬️typology/🦀️.rs", "RAW_TYPOLOGY_ASSETS", "TYPOLOGIES", True),
         ("typology", "🧬️typology/🦀️.rs", "RAW_ATTRIBUTE_DEFINITION_ASSETS", "ATTRIBUTE_DEFINITIONS", True)]


def entries(text, const, keyed):
    body = re.search(r"const " + const + r": &\[[^\]]*\]\] = &\[|const " + const + r": &\[[^=]*= &\[", text)
    start = body.end()
    depth, j = 1, start
    while depth:
        depth += text[j] == "["
        depth -= text[j] == "]"
        j += 1
    block = text[start:j - 1]
    if keyed:
        rows = re.findall(r'\(\s*"([^"]+)"\s*,\s*include_str!\("([^"]+)"\)\s*,?\s*\)', block)
    else:
        rows = [(None, p) for p in re.findall(r'include_str!\("([^"]+)"\)', block)]
    return body.start(), j, rows


def group_of(path):
    rel = os.path.relpath(path, DEFS).split(os.sep)
    group = DIR_GROUP[rel[0]]
    if group == "shape" and rel[1] == "🕹️interactions":
        name = rel[-1]
        if name in SHAPE_UNPRODUCED:
            return SHAPE_UNPRODUCED[name]
        produces = json.load(open(path, encoding="utf-8")).get("produces", {}).get("typology", "")
        return SHAPE_CATEGORY[produces.split(".")[2]]
    return group


def main():
    mode = sys.argv[1]
    files = {}
    lists = collections.defaultdict(lambda: collections.defaultdict(list))
    edits = collections.defaultdict(list)
    for kind, rel, const, out, keyed in KINDS:
        path = os.path.join(ENGINE, rel)
        text = files.setdefault(path, open(path, encoding="utf-8").read())
        start, end, rows = entries(text, const, keyed)
        order = []
        for key, include in rows:
            asset = os.path.normpath(os.path.join(os.path.dirname(path), include))
            assert os.path.exists(asset), asset
            group = group_of(asset)
            lists[group][out].append((key, asset))
            if group not in order:
                order.append(group)
        item = "&[(&str, &str)]" if keyed else "&[&str]"
        if const == "RAW_INTERACTION_ASSETS":
            order = ["shape_curves", "shape_surfaces", "shape_solids", "shape_edits"] + [g for g in order if not g.startswith("shape")]
        slices = ", ".join(f"crate::editor::cad::catalog::{g}::{out}" for g in order)
        edits[path].append((start, end, f"const {const}: &[{item}] = &[{slices}]"))
        print(f"{const}: {len(rows)} rows -> {order}")
    for path, rows in edits.items():
        text = files[path]
        for start, end, rep in sorted(rows, reverse=True):
            text = text[:start] + rep + text[end:]
        files[path] = text
    roots = {}
    for group, (dirname, doc) in GROUPS.items():
        directory = os.path.join(CATALOG, dirname)
        lines = [f"//! {doc}", ""]
        total = 0
        for out in ("MODEL_DEFINITIONS", "TYPOLOGIES", "ATTRIBUTE_DEFINITIONS", "INTERACTIONS"):
            rows = lists[group].get(out, [])
            keyed = out != "MODEL_DEFINITIONS"
            item = "(&str, &str)" if keyed else "&str"
            lines.append(f"/// 📚️ `{out.lower().replace('_', ' ')}` assets of this catalog, embedded at build time.")
            lines.append(f"pub const {out}: &[{item}] = &[")
            for key, asset in rows:
                inc = os.path.relpath(asset, directory)
                total += os.path.getsize(asset)
                lines.append(f'    ("{key}", include_str!("{inc}")),' if keyed else f'    include_str!("{inc}"),')
            lines.append("];")
            lines.append("")
        roots[os.path.join(directory, "🦀️.rs")] = "\n".join(lines).rstrip() + "\n"
        print(f"{dirname}: {sum(len(v) for v in lists[group].values())} files, {total} bytes")
    if mode == "apply":
        for path, text in list(files.items()) + list(roots.items()):
            os.makedirs(os.path.dirname(path), exist_ok=True)
            open(path, "w", encoding="utf-8").write(text)


main()
