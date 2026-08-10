"""Transform 5d example DSL assets for design-parity catalogs / fastener xy / part anchor."""
from __future__ import annotations
import re
from pathlib import Path

puzzle = Path(open("/tmp/puzzle_path.txt").read().strip())
examples = puzzle / "🗿️artifacts/🖐️5d/📚️examples"

def transform_grip_list(list_body: str) -> str:
    """Convert old grip-kind + grip-2d + grip-3d sequences into unified GripTemplate records."""
    # Split roughly on grip-kind= starts that begin a template (not the nested grip-kind inside grip-2d)
    # Old pattern per template:
    #   grip-kind=ID grip-2d=angle=...rad grip-kind=ID radius=R grip-3d=position=@X,Y,Z direction=^A,B,C radius=R
    pattern = re.compile(
        r"grip-kind=(?P<kind>[^\s]+)\s+"
        r"grip-2d=angle=(?P<angle>[^\s]+)\s+"
        r"grip-kind=(?P<kind2>[^\s]+)\s+"
        r"radius=(?P<r2d>[^\s]+)\s+"
        r"grip-3d=position=(?P<pos>@[^\s]+)\s+"
        r"direction=(?P<dir>\^[^\s]+)\s+"
        r"radius=(?P<r3d>[^\s]+)"
    )
    out = []
    for i, m in enumerate(pattern.finditer(list_body)):
        kind = m.group("kind")
        pos = m.group("pos")
        direction = m.group("dir")
        radius = m.group("r3d")
        out.append(
            f"id=g{i} name={kind} label={kind} grip-kind={kind} point={pos} direction={direction} radius={radius}"
        )
    if not out:
        return list_body  # unchanged if pattern missed
    return " ".join(out)

def transform_catalog_parts_table(block: str) -> str:
    """Rewrite parts table inside kind-catalogs= ..."""
    # Header
    block = re.sub(
        r"parts \[id:TEXT name:TEXT label:TEXT mesh-url:TEXT grips:LIST\]",
        "parts [id:TEXT name:TEXT label:TEXT description:TEXT icon:TEXT image:TEXT unit:TEXT abstract:BOOL base-kinds:LIST representations:LIST grips:LIST attributes:LIST authors:LIST]",
        block,
        count=1,
    )
    # Each data row: ID NAME LABEL MESHURL [grips]
    # Rows are whitespace-separated; grips are in [ ... ]
    def repl_row(m: re.Match) -> str:
        pid, name, label, mesh, grips = m.group(1), m.group(2), m.group(3), m.group(4), m.group(5)
        # strip quotes for id/name if present
        new_grips = transform_grip_list(grips)
        rep = f'[ id=lod0 name=mesh url={mesh} mime=model/gltf-binary tags=[mesh] description="" ]'
        return (
            f'{pid} {name} {label} "" "" "" "" false [] {rep} [ {new_grips} ] [] []'
        )
    # Match a row line that still has mesh path and grips list
    row_re = re.compile(
        r'^(\"[^\"]+\"|\S+)\s+(\"[^\"]+\"|\S+)\s+(\"[^\"]+\"|\S+)\s+(\"[^\"]+\"|/mesh/\S+)\s+\[(.*)\]\s*$',
        re.M,
    )
    block, n = row_re.subn(repl_row, block)
    return block

def transform_grips_table(block: str) -> str:
    block = re.sub(
        r"grips \[id:TEXT name:TEXT label:TEXT color:TEXT default-rope-kind:REF\]",
        "grips [id:TEXT code:TEXT label:TEXT order:NUM compatible-with:LIST description:TEXT icon:TEXT color:TEXT default-rope-kind:REF]",
        block,
        count=1,
    )
    # rows: id name label color rope
    def repl(m: re.Match) -> str:
        gid, name, label, color, rope = m.groups()
        return f'{gid} {name} {label} 0 [] "" "" {color} {rope}'
    block = re.sub(
        r'^(\S+)\s+(\S+)\s+(\S+)\s+(\"[^\"]+\"|\S+)\s+(\S+)\s*$',
        repl,
        block,
        flags=re.M,
    )
    return block

def transform_fasteners_kinds(block: str) -> str:
    # fasteners kinds table inside catalogs — keep similar
    return block

def transform_file(path: Path) -> None:
    text = path.read_text()
    orig = text
    # kind-compatibility header + rows: add important specificity defaults in header; rows get two cols
    text = text.replace(
        "kind-compatibility [source:REF target:REF bidirectional:BOOL]",
        "kind-compatibility [source:REF target:REF bidirectional:BOOL important:BOOL specificity:TEXT]",
    )
    # Add important false specificity general to each kind-compatibility data row (3-field rows)
    def compat_row(m: re.Match) -> str:
        return f"{m.group(1)} {m.group(2)} {m.group(3)} false general"
    # Only within kind-compatibility table — approximate: lines with two ids and bool
    # Do globally for lines matching three tokens ending with true/false after we changed header
    lines = text.splitlines()
    out_lines = []
    in_compat = False
    compat_depth = 0
    for line in lines:
        if "kind-compatibility [" in line:
            in_compat = True
            out_lines.append(line)
            continue
        if in_compat:
            if "{" in line:
                compat_depth += line.count("{")
            if "}" in line:
                compat_depth -= line.count("}")
                out_lines.append(line)
                if compat_depth <= 0:
                    in_compat = False
                continue
            m = re.match(r'^(\s*)(\S+)\s+(\S+)\s+(true|false)\s*$', line)
            if m:
                out_lines.append(f"{m.group(1)}{m.group(2)} {m.group(3)} {m.group(4)} false general")
                continue
        out_lines.append(line)
    text = "\n".join(out_lines) + ("\n" if orig.endswith("\n") else "")

    # parts instance table: add anchor
    text = text.replace(
        "parts [id:TEXT part-kind:REF part-2d:REC part-3d:REC grips:LIST]",
        "parts [id:TEXT part-kind:REF anchor:TEXT part-2d:REC part-3d:REC grips:LIST]",
    )
    # Insert fixed after part-kind on each part row — hard; rely on default if column missing?
    # Header now requires anchor column — insert `fixed` after second field for rows in parts table outside catalogs.
    # Safer: put default via making anchor optional in header? Rust field has serde default; DSL table may require column if in header.
    # Insert fixed into rows for the main parts table only.
    lines = text.splitlines()
    out_lines = []
    in_parts = False
    parts_depth = 0
    seen_kind_catalogs_parts = False
    for i, line in enumerate(lines):
        if line.startswith("kind-catalogs="):
            seen_kind_catalogs_parts = True
        if re.match(r'^parts \[id:TEXT part-kind:REF anchor:TEXT', line):
            in_parts = True
            out_lines.append(line)
            continue
        if in_parts:
            if "{" in line:
                parts_depth += line.count("{")
            if "}" in line:
                parts_depth -= line.count("}")
                out_lines.append(line)
                if parts_depth <= 0:
                    in_parts = False
                continue
            # row starts with id then part-kind then { for part-2d
            m = re.match(r'^(\s*)(\S+)\s+(\"[^\"]+\"|\S+)\s+(\{.*)$', line)
            if m:
                out_lines.append(f"{m.group(1)}{m.group(2)} {m.group(3)} fixed {m.group(4)}")
                continue
        out_lines.append(line)
    text = "\n".join(out_lines) + ("\n" if text.endswith("\n") else "")

    # fasteners instance table
    text = text.replace(
        "fasteners [id:TEXT source:TEXT target:TEXT fastener-kind:REF gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM]",
        "fasteners [id:TEXT source:TEXT target:TEXT fastener-kind:REF gap:NUM shift:NUM rise:NUM rotation:NUM turn:NUM tilt:NUM x:NUM y:NUM]",
    )

    # Transform kind-catalogs section — find from kind-catalogs= to kind-compatibility or parts [
    m = re.search(r'(kind-catalogs=\s*)(.*?)(\nkind-compatibility )', text, re.S)
    if m:
        catalogs = m.group(2)
        # parts table inside catalogs
        catalogs = transform_catalog_parts_table(catalogs)
        # grips table
        gm = re.search(r'(grips \[.*?\].*?\{)(.*?)(\n\})', catalogs, re.S)
        if gm:
            grips_body = gm.group(2)
            # header already in group1 - rewrite header in full catalogs
            catalogs = re.sub(
                r"grips \[id:TEXT name:TEXT label:TEXT color:TEXT default-rope-kind:REF\]",
                "grips [id:TEXT code:TEXT label:TEXT order:NUM compatible-with:LIST description:TEXT icon:TEXT color:TEXT default-rope-kind:REF]",
                catalogs,
                count=1,
            )
            def grip_row(mm: re.Match) -> str:
                gid, name, label, color, rope = mm.groups()
                return f"{mm.group(0).split(gid)[0]}{gid} {name} {label} 0 [] \"\" \"\" {color} {rope}" if False else f"{gid} {name} {label} 0 [] \"\" \"\" {color} {rope}"
            new_body_lines = []
            for gl in grips_body.splitlines():
                mm = re.match(r'^(\s*)(\S+)\s+(\S+)\s+(\S+)\s+(\"[^\"]+\"|\S+)\s+(\S+)\s*$', gl)
                if mm:
                    new_body_lines.append(f"{mm.group(1)}{mm.group(2)} {mm.group(3)} {mm.group(4)} 0 [] \"\" \"\" {mm.group(5)} {mm.group(6)}")
                else:
                    new_body_lines.append(gl)
            catalogs = catalogs[:gm.start(2)] + "\n".join(new_body_lines) + catalogs[gm.end(2):]
        # fasteners kinds / ropes headers — rename nothing critical; CatalogFastenerKind still has id name label
        text = text[:m.start(2)] + catalogs + text[m.end(2):]

    if text != orig:
        path.write_text(text)
        print("updated", path, "delta", len(text) - len(orig))
    else:
        print("no change", path)

for dsl in examples.rglob("🗣️*.dsl.semio"):
    transform_file(dsl)
print("examples done")
