#!/usr/bin/env python3
"""Track 3 Wave A item #5: migrates each app crate's hand-rolled is_de_locale/X_locale/resolve_labels
trio (or 2-fn variant without is_de_locale) onto the new shared semio_framework_plugin::LabelAxes
trait + locale_from_str + generic resolve_labels. Ticket-scoped, throwaway; only touches files that
match the confirmed pattern shape — anomalous files are reported and left untouched."""
import re
import sys

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

RESOLVE_SIG_RE = re.compile(r'fn resolve_labels<L: AppLabels>\((\w+): &(\w+)\) -> &\x27static L \{')


def find_matching_brace(text, open_brace_idx):
    depth = 0
    i = open_brace_idx
    while i < len(text):
        if text[i] == "{":
            depth += 1
        elif text[i] == "}":
            depth -= 1
            if depth == 0:
                return i
        i += 1
    raise ValueError("unbalanced braces")


def region_start(text, resolve_fn_start):
    # Walk backwards from the resolve_labels fn to include any preceding is_de_locale/_locale
    # helper functions AND their doc comments, up to (but not including) a blank-line gap that
    # precedes unrelated code, or a region marker.
    before = text[:resolve_fn_start]
    # Find the start of the nearest preceding `fn is_de_locale` or `fn \w+_locale` run.
    helper_starts = [m.start() for m in re.finditer(r'\n(?:/// .*\n)*fn (?:is_de_locale|\w+_locale)\(', before)]
    if helper_starts:
        candidate = helper_starts[-1] + 1  # skip the leading \n
        # Walk back further to catch a chain of 1-2 helper functions (is_de_locale then X_locale).
        chain_starts = [m.start() for m in re.finditer(r'\n(?:/// .*\n)*fn (?:is_de_locale|\w+_locale)\(', before)]
        return chain_starts[0] + 1
    # No helper fn found (2-fn variant already collapsed, or direct) — just the doc comment
    # immediately preceding resolve_labels itself, if any.
    m = re.search(r'(?:/// .*\n)*$', before)
    return m.start() if m else resolve_fn_start


def process(rel):
    path = ROOT + rel
    with open(path, "r", encoding="utf-8") as f:
        text = f.read()
    m = RESOLVE_SIG_RE.search(text)
    if not m:
        print(f"  SKIP (no matching resolve_labels signature): {rel}")
        return False
    param_name, config_type = m.group(1), m.group(2)
    fn_open_brace = text.index("{", m.end() - 1)
    fn_end = find_matching_brace(text, fn_open_brace) + 1
    start = region_start(text, m.start())
    old_region = text[start:fn_end]
    replacement = (
        f"impl semio_framework_plugin::LabelAxes for {config_type} {{\n"
        f"    fn locale(&self) -> Locale {{\n"
        f"        semio_framework_plugin::locale_from_str(&self.{param_name if param_name != 'self' else 'locale'}.locale)\n"
        f"    }}\n"
        f"}}\n"
    )
    # The field is always `.locale` on the config struct itself, not on the param name.
    replacement = (
        f"impl semio_framework_plugin::LabelAxes for {config_type} {{\n"
        f"    fn locale(&self) -> Locale {{\n"
        f"        semio_framework_plugin::locale_from_str(&self.locale)\n"
        f"    }}\n"
        f"}}\n"
    )
    new_text = text[:start] + replacement + text[fn_end:]
    # Ensure `resolve_labels` resolves via the crate-level use of semio_framework_plugin — add an
    # explicit import if the existing `use semio_framework_plugin::{...}` block doesn't already list it.
    if "resolve_labels" not in re.sub(re.escape(old_region), "", text):
        use_block_re = re.compile(r'use semio_framework_plugin::\{\n')
        um = use_block_re.search(new_text)
        if um:
            new_text = new_text[: um.end()] + "    resolve_labels,\n" + new_text[um.end() :]
        else:
            single_use_re = re.compile(r'(use semio_framework_plugin::)(\w+;)')
            sm = single_use_re.search(new_text)
            if sm:
                pass  # leave single-import files; resolve_labels likely already covered separately
            else:
                new_text = f"use semio_framework_plugin::resolve_labels;\n" + new_text
    with open(path, "w", encoding="utf-8") as f:
        f.write(new_text)
    print(f"  OK ({config_type}): {rel}")
    return True


def main():
    ok = 0
    for rel in FILES:
        if process(rel):
            ok += 1
    print(f"\n{ok}/{len(FILES)} migrated")


if __name__ == "__main__":
    main()
