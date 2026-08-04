#!/usr/bin/env python3
"""Fix-up pass 3: broadens the call-site rewrite to handle `resolve_labels::<L>(cfg.projection)`,
`resolve_labels::<L>(&config)`, etc. (dotted-path + optional leading `&`), which the first regex
(bare identifier only) missed. Skips the zero-arg `resolve_labels::<L>()` call in the playbook
procedural extension (different, deliberately out of scope) and any occurrence inside a `///` doc
comment line."""
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

# Bare `resolve_labels::<L>(&?expr)` not already qualified with `semio_framework_plugin::` and not
# a zero-arg call (handled separately, out of scope) — dotted-path arg, optional leading `&`.
CALL_RE = re.compile(r'(?<!plugin::)\bresolve_labels::<(\w+)>\(&?([\w.]+)\)')


def process(rel):
    path = ROOT + rel
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    n = 0
    for i, line in enumerate(lines):
        stripped = line.lstrip()
        if stripped.startswith("///") or stripped.startswith("//"):
            continue

        def repl(m):
            nonlocal n
            n += 1
            return f"semio_framework_plugin::resolve_labels_for_locale::<{m.group(1)}>(&{m.group(2)}.locale)"

        lines[i] = CALL_RE.sub(repl, line)
    if n:
        with open(path, "w", encoding="utf-8") as f:
            f.writelines(lines)
        print(f"  FIXED ({n} call site(s)): {rel}")
    else:
        print(f"  NOOP: {rel}")


for rel in FILES:
    process(rel)
