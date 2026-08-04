#!/usr/bin/env python3
"""Fix-up pass 2: the first migration's `impl semio_framework_plugin::LabelAxes for XConfig` blocks
violate Rust's orphan rule wherever XConfig isn't defined in the same crate (true for every one of
the 24 _ui crates — Config types live in the app's base/op crate). Removes those impl blocks and
rewrites call sites from `resolve_labels::<L>(cfg)` to
`semio_framework_plugin::resolve_labels_for_locale::<L>(&cfg.locale)`, which needs no trait impl."""
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

IMPL_RE = re.compile(r'impl semio_framework_plugin::LabelAxes for \w+ \{\n(?:.*\n)*?\}\n')
CALL_RE = re.compile(r'\bresolve_labels::<(\w+)>\((\w+)\)')
IMPORT_LIST_RE = re.compile(r'\bresolve_labels,\n')
IMPORT_SINGLE_RE = re.compile(r'use semio_framework_plugin::resolve_labels;\n')


def process(rel):
    path = ROOT + rel
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    changed = False

    m = IMPL_RE.search(text)
    if m:
        text = text[: m.start()] + text[m.end() :]
        changed = True

    def replace_call(m):
        label_ty, cfg_var = m.group(1), m.group(2)
        return f"semio_framework_plugin::resolve_labels_for_locale::<{label_ty}>(&{cfg_var}.locale)"

    new_text, n = CALL_RE.subn(replace_call, text)
    if n:
        text = new_text
        changed = True

    # The old fix-up added a plain `resolve_labels,`/`use ...resolve_labels;` import that's now
    # unused (calls are fully-qualified) — remove it to avoid an unused-import warning-as-error.
    text2 = IMPORT_LIST_RE.sub("", text)
    text2 = IMPORT_SINGLE_RE.sub("", text2)
    if text2 != text:
        text = text2
        changed = True

    if changed:
        with open(path, "w", encoding="utf-8") as f:
            f.write(text)
        print(f"  FIXED ({n} call site(s)): {rel}")
    else:
        print(f"  NOOP: {rel}")


for rel in FILES:
    process(rel)
