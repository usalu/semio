import re, sys

D = "/Users/ueli/Documents/semio/framework/renderer/react"

FILES_IN_ORDER = [
    ("ui-interpreter.tsx", "UiInterpreter"),
    ("os-shell.tsx", "OsShell"),
    ("components/world-terrain-layer.tsx", "WorldTerrainLayerHost"),
    ("components/canvas-2d-host.tsx", "Canvas2dHost"),
    ("components/world-3d-host.tsx", "World3dHost"),
    ("components/node-graph-host.tsx", "NodeGraphHost"),
    ("components/text-editor-host.tsx", "TextEditorHost"),
    ("components/table-host.tsx", "TableHost"),
    ("components/paint-2d-host.tsx", "Paint2dHost"),
    ("components/tiled-map-host.tsx", "TiledMapHost"),
    ("components/board-2d-host.tsx", "Board2dHost"),
    ("components/icon-render-host.tsx", "IconRenderHost"),
    ("components/ink-canvas-host.tsx", "InkCanvasHost"),
    ("components/graph-timeline-host.tsx", "GraphTimelineHost"),
    ("components/block-list-host.tsx", "BlockListHost"),
    ("components/diff-view-host.tsx", "DiffViewHost"),
    ("components/event-feed-host.tsx", "EventFeedHost"),
]

RELATIVE_SPECIFIERS = {
    "./os-shell.tsx", "../os-shell.tsx",
    "./ui-interpreter.tsx", "../ui-interpreter.tsx",
    "./canvas-2d-host.tsx", "../canvas-2d-host.tsx", "./components/canvas-2d-host.tsx",
    "./world-terrain-layer.tsx", "../world-terrain-layer.tsx", "./components/world-terrain-layer.tsx",
}

IMPORT_RE = re.compile(
    r'^import(?:\s+type)?\s+(?:[A-Za-z0-9_$]+\s*,\s*)?(?:\{[^}]*\}|\*\s+as\s+[A-Za-z0-9_$]+|[A-Za-z0-9_$]+)?\s*from\s*["\'][^"\']+["\'];\s*$',
    re.MULTILINE,
)

# collected imports: specifier -> {"default": name_or_None, "namespace": name_or_None, "names": OrderedDict(entry_text -> True)}
from collections import OrderedDict
collected = OrderedDict()

def parse_import_block(text):
    """Split leading import statements (each possibly multi-line) from the rest of the file."""
    lines = text.split('\n')
    i = 0
    import_lines = []
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        if stripped.startswith('import ') or stripped.startswith('import{') or stripped == 'import':
            # gather until we hit a line ending in ';' that also contains 'from "'
            block = [line]
            j = i
            while 'from "' not in block[-1] and "from '" not in block[-1]:
                j += 1
                block.append(lines[j])
            # block[-1] should end with ; possibly
            i = j + 1
            import_lines.append('\n'.join(block))
            continue
        elif stripped == '':
            i += 1
            continue
        else:
            break
    rest = '\n'.join(lines[i:])
    return import_lines, rest

def register_import(stmt):
    m = re.match(r'^import(\s+type)?\s+(.*)\s+from\s+["\']([^"\']+)["\'];?\s*$', stmt.strip(), re.DOTALL)
    if not m:
        raise ValueError(f"Could not parse import: {stmt!r}")
    is_type_only = bool(m.group(1))
    clause = m.group(2).strip()
    spec = m.group(3)

    if spec in RELATIVE_SPECIFIERS:
        return  # dropped: symbols become locally available after merge

    entry = collected.setdefault(spec, {"default": None, "namespace": None, "names": OrderedDict()})

    # clause forms: "Default", "Default, { a, b }", "{ a, b }", "* as NS", "Default, * as NS"
    ns_match = re.search(r'\*\s+as\s+([A-Za-z0-9_$]+)', clause)
    if ns_match:
        entry["namespace"] = ns_match.group(1)
        clause = clause[:ns_match.start()].rstrip(', ').strip()

    brace_match = re.search(r'\{(.*)\}', clause, re.DOTALL)
    if brace_match:
        names_blob = brace_match.group(1)
        default_part = clause[:brace_match.start()].rstrip(', ').strip()
        for raw in names_blob.split(','):
            raw = raw.strip()
            if not raw:
                continue
            if is_type_only and not raw.startswith('type '):
                raw = 'type ' + raw
            entry["names"][raw] = True
    else:
        default_part = clause.strip()

    if default_part:
        entry["default"] = default_part

def emit_imports():
    out = []
    for spec, entry in collected.items():
        parts = []
        if entry["default"]:
            parts.append(entry["default"])
        if entry["namespace"]:
            parts.append(f'* as {entry["namespace"]}')
        if entry["names"]:
            # dedupe: prefer non-"type " form if both exist for same base name
            names = list(entry["names"].keys())
            bare_to_full = {}
            for n in names:
                bare = n[5:].strip() if n.startswith('type ') else n
                base_id = bare.split(' as ')[0].strip()
                is_type = n.startswith('type ')
                if base_id not in bare_to_full or (bare_to_full[base_id][1] and not is_type):
                    bare_to_full[base_id] = (n, is_type)
            final_names = [v[0] for v in bare_to_full.values()]
            parts.append('{ ' + ', '.join(final_names) + ' }')
        if parts:
            out.append(f'import {", ".join(parts)} from "{spec}";')
    return '\n'.join(out)

bodies = []
for relpath, label in FILES_IN_ORDER:
    with open(f"{D}/{relpath}") as f:
        text = f.read()
    import_lines, rest = parse_import_block(text)
    for stmt in import_lines:
        register_import(stmt)
    bodies.append((label, rest.strip('\n')))

header = '''// #region 🧱️Header
/** @emoji 🎨️ `@semio-tech/framework-renderer-react` — trusted React renderer for declarative Rust plugin UI trees. */
// #endregion 🧱️Header
'''

import_block = emit_imports()

sections = []
for label, body in bodies:
    sections.append(f"//#region 🔖️{label}\n{body}\n//#endregion 🔖️{label}")

output = header + "\n" + import_block + "\n\n" + "\n\n".join(sections) + "\n"

with open(f"{D}/index.tsx", "w") as f:
    f.write(output)

print("Wrote index.tsx:", len(output.splitlines()), "lines")
print("Import specifiers merged:", len(collected))
