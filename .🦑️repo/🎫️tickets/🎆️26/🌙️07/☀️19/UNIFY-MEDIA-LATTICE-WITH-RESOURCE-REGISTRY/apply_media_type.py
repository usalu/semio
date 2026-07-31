#!/usr/bin/env python3
"""🧬️ Mechanical fan-out: adds media_type/schema/export_formats/import_formats to every
ResourceKindSpec literal across the ~25 plugin crates, and threads MediaClass/MediaForm/MediaType
into each file's semio_framework_plugin import block. One-shot script for the
UNIFY-MEDIA-LATTICE-WITH-RESOURCE-REGISTRY ticket; not meant to be reused."""
import re
import sys

ROOT = "/Users/ueli/Documents/semio/"

# id -> (class, form, export_formats, import_formats)
MAPPING = {
    "2d.note": ("TwoD", "Document", [], []),
    "animate.present.deck": ("Presentation", "Deck", [], []),
    "form.dictionary": ("Data", "Value", [], []),
    "computation.imperative": ("Computation", "Imperative", [], []),
    "graph.trinity": ("Graph", "Trinity", [], []),
    "2d.puzzle": ("TwoD", "Design", ["Svg", "Png"], ["Svg", "Png"]),
    "3d.puzzle": ("ThreeD", "Design", ["Glb", "Obj", "Stl"], ["Glb", "Obj"]),
    "5d.puzzle": ("Kit", "Design", [], []),
    "3d.lowpoly": ("ThreeD", "Mesh", ["Glb", "Obj", "Stl"], ["Glb", "Obj"]),
    "3d.mesh": ("ThreeD", "Mesh", ["Glb", "Obj", "Stl"], ["Glb", "Obj"]),
    "2d.layout": ("TwoD", "Vector", ["Svg", "Png"], ["Svg", "Png"]),
    "graph.wires": ("Graph", "Dag", [], []),
    "3d.cad": ("ThreeD", "Brep", ["Step", "Obj", "Stl", "Glb"], ["Step", "Obj", "Stl"]),
    "text.document": ("Text", "Document", [], []),
    "vcs.document": ("Data", "Value", [], []),
    "3d.remodel": ("ThreeD", "Mesh", ["Glb", "Obj", "Stl"], ["Glb", "Obj"]),
    "2d.raster": ("TwoD", "Raster", ["Svg", "Png"], ["Svg", "Png"]),
    "computation.sequence": ("Computation", "Sequence", [], []),
    "catalogue.kinds": ("Kit", "Type", [], []),
    "catalogue.sourcing": ("Kit", "Kit", [], []),
    "graph.dag": ("Graph", "Dag", [], []),
    "2d.drawing": ("TwoD", "Vector", ["Svg", "Png"], ["Svg", "Png"]),
    "2d.map": ("TwoD", "Vector", ["Svg", "Png"], ["Svg", "Png"]),
    "2d.procedural": ("TwoD", "Flow", [], []),
    "3d.procedural": ("ThreeD", "Flow", [], []),
    "2d.shooting": ("TwoD", "Raster", ["Svg", "Png"], ["Svg", "Png"]),
    "3d.process": ("ThreeD", "Brep", ["Step", "Obj", "Stl", "Glb"], ["Step", "Obj", "Stl"]),
    "computation.flow": ("Computation", "Flow", [], []),
}

FILES = [
    "animate/plugin/rs/lib.rs",
    "forms/plugin/rs/lib.rs",
    "imperative/plugin/rs/lib.rs",
    "trinity/plugin/rs/lib.rs",
    "puzzle/plugin/rs/lib.rs",
    "lowpoly/plugin/rs/lib.rs",
    "layout/plugin/rs/lib.rs",
    "reasoning/mindmap/plugin/rs/lib.rs",
    "norm/plugin/rs/lib.rs",
    "cad/plugin/rs/lib.rs",
    "writer/plugin/rs/lib.rs",
    "vcs/plugin/rs/lib.rs",
    "remodel/plugin/rs/lib.rs",
    "raster/plugin/rs/lib.rs",
    "sequence/plugin/rs/lib.rs",
    "sourcing/plugin/rs/lib.rs",
    "infinite/board/port/directed/dag/plugin/rs/lib.rs",
    "draw/plugin/rs/lib.rs",
    "gis/plugin/rs/lib.rs",
    "procedural/plugin/rs/lib.rs",
    "shooting/plugin/rs/lib.rs",
    "process/plugin/rs/lib.rs",
    "flow/plugin/rs/lib.rs",
]

# id literal used for the id: "..." field (regex-escaped). norm uses `format!(...)` for id, not a plain string.
BLOCK_RE = re.compile(
    r'(?P<head>\.resource_kind\(ResourceKindSpec \{\n'
    r'(?P<indent>[ \t]+)id: (?P<idexpr>[^\n]+),\n'
    r'[ \t]+name: [^\n]+,\n'
    r'[ \t]+source_format: (?P<schemaexpr>[^\n]+),\n'
    r'[ \t]+component_kind: [^\n]+,\n'
    r'[ \t]+dimension: [^\n]+,\n'
    r'[ \t]+media_capability: [^\n]+,\n)'
    r'(?P<indent2>[ \t]+)\}\)'
)

NORM_ID_RE = re.compile(r'format!\("computation\.norm\.\{variant\}\.document"')


def os_media_format_list(names, indent):
    if not names:
        return "vec![]"
    inner = ", ".join(f"OsMediaFormat::{n}" for n in names)
    return f"vec![{inner}]"


def resolve_key(idexpr):
    idexpr = idexpr.strip()
    if idexpr.startswith('"') and idexpr.endswith('".into()'):
        return idexpr[1:-len('".into()')]
    if 'norm.{variant}' in idexpr or "computation.norm" in idexpr:
        return "computation.norm.*"
    return None


def process(path):
    with open(path, "r", encoding="utf-8") as f:
        content = f.read()
    original = content

    def repl(match):
        idexpr = match.group("idexpr")
        indent = match.group("indent")
        key = resolve_key(idexpr)
        if key == "computation.norm.*":
            cls, form, exp, imp = ("Computation", "Value", [], [])
            schema_expr = 'format!("norm.{variant}.document", variant = $variant)'
        else:
            if key not in MAPPING:
                raise SystemExit(f"no mapping for id expr: {idexpr!r} in {path}")
            cls, form, exp, imp = MAPPING[key]
            schema_expr = match.group("schemaexpr")
        exp_v = os_media_format_list(exp, indent)
        imp_v = os_media_format_list(imp, indent)
        insert = (
            f"{indent}media_type: MediaType {{ class: MediaClass::{cls}, form: MediaForm::{form} }},\n"
            f"{indent}schema: {schema_expr},\n"
            f"{indent}export_formats: {exp_v},\n"
            f"{indent}import_formats: {imp_v},\n"
        )
        return match.group("head") + insert + match.group("indent2") + "})"

    new_content, count = BLOCK_RE.subn(repl, content)
    if count == 0:
        print(f"WARNING: no ResourceKindSpec block matched in {path}", file=sys.stderr)
        return False

    # Thread MediaClass/MediaForm/MediaType into the import block: insert right before the first
    # standalone "OsMediaCapability" import token, if not already imported (idempotent on rerun).
    idx = new_content.find("OsMediaCapability")
    if idx == -1:
        raise SystemExit(f"OsMediaCapability not found for import patch in {path}")
    window = new_content[max(0, idx - 200):idx]
    if "MediaClass" not in window:
        new_content = new_content[:idx] + "MediaClass, MediaForm, MediaType, " + new_content[idx:]

    if new_content != original:
        with open(path, "w", encoding="utf-8") as f:
            f.write(new_content)
        print(f"patched {count} block(s) + import in {path}")
        return True
    return False


if __name__ == "__main__":
    any_changed = False
    for rel in FILES:
        p = ROOT + rel
        any_changed |= process(p)
    if not any_changed:
        print("no files changed", file=sys.stderr)
        sys.exit(1)
