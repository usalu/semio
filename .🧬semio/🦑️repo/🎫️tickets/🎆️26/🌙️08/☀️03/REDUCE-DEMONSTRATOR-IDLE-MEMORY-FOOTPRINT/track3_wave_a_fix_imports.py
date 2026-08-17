#!/usr/bin/env python3
"""Fix-up pass: the first migration script's "is resolve_labels already imported" check was buggy
(it matched the bare string, which is always true once a *call site* remains) — ensures every file
that calls `resolve_labels::<...>` but doesn't define or import it gets a proper import added."""
import re

FILES = [
    "✏️s/🔌️plugin/🔱️trinity/🎛️app/🔌️jack/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🔱️trinity/🎛️app/✏️rewrite/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/📸️remodel/🎛️app/📸️remodel/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🖨️raster/🎛️app/🖨️raster/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🌊️flow/🎛️app/🌊️flow/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🏭️process/🎛️app/🧊️3d/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🧱️block/🎛️app/🖐️5d/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🧱️block/🎛️app/◻2d/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/💡️reasoning/🎛️app/🔌️wires/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/✒️writer/🎛️app/✒️writer/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🎬️sequence/🎛️app/🎬️sequence/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🪐️space/🎛️app/🏠️home/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🪐️space/🎛️app/🪐️space/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🌀️procedural/🎛️app/🧊️3d/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🌀️procedural/🎛️app/◻2d/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🌿️vcs/🎛️app/🌿️vcs/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🌍️gis/🎛️app/◻2d/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🪵️sourcing/🎛️app/🗂️curate/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🗒️note/🎛️app/🗒️note/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/📋️forms/🎛️app/📋️forms/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🎥️shooting/🎛️app/🎥️shooting/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/📏️layout/🎛️app/📏️layout/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/🖍️draw/🎛️app/🖍️draw/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
    "✏️s/🔌️plugin/💠️lowpoly/🎛️app/💠️lowpoly/🔨️module/🖱️ui/⚡️implementation/🦀️rust/📦️lib.rs",
]

ROOT = "/Users/ueli/Documents/semio/"

IMPORTED_RE = re.compile(r'use semio_framework_plugin::(?:\{[^}]*\bresolve_labels\b[^}]*\}|resolve_labels;)', re.DOTALL)
DEFINES_RE = re.compile(r'fn resolve_labels\b')
CALLS_RE = re.compile(r'\bresolve_labels::<')
USE_BLOCK_OPEN_RE = re.compile(r'use semio_framework_plugin::\{\n')


def process(rel):
    path = ROOT + rel
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    if not CALLS_RE.search(text):
        print(f"  SKIP (no call site): {rel}")
        return
    if DEFINES_RE.search(text) or IMPORTED_RE.search(text):
        print(f"  OK (already defined/imported): {rel}")
        return
    m = USE_BLOCK_OPEN_RE.search(text)
    if m:
        new_text = text[: m.end()] + "    resolve_labels,\n" + text[m.end() :]
    else:
        new_text = "use semio_framework_plugin::resolve_labels;\n" + text
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_text)
    print(f"  FIXED: {rel}")


for rel in FILES:
    process(rel)
