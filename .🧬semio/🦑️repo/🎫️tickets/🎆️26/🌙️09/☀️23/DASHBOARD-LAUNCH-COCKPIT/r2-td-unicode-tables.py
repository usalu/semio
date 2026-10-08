"""Assembles the committed Unicode 17 table module of the TUI text model (slice T-D).

Inputs : unicode-segmentation 1.13.2 `src/tables.rs` (grapheme break + InCB data) and the output of the
         width probe (`r2-td-unicode-width-probe.rs`, built against unicode-width 0.2.2).
Output : `🧰️framework/🔨️modules/🖱️ui/⌨️tui/📝️text/🔤️tables/🦀️.rs`.
The committed module is validated by the oracle tests in `⌨️tui/🧪️tests/📏️text-width/🦀️.rs`.
"""
import glob
import os
import re
import sys

registry = glob.glob(os.path.expanduser("~/.cargo/registry/src/*/unicode-segmentation-1.13.2/src/tables.rs"))[0]
source = open(registry, encoding="utf-8").read()
width_tables = open(sys.argv[1], encoding="utf-8").read()
target = sys.argv[2]


def block(name):
    start = source.index(name)
    start = source.index("= &[", start) + 4
    end = source.index("];", start)
    return source[start:end]


def cp(token):
    return int(token, 16)


grapheme = [(cp(a), cp(b), c) for a, b, c in re.findall(r"'\\u\{([0-9a-f]+)\}',\s*'\\u\{([0-9a-f]+)\}',\s*GC_(\w+)", block("const grapheme_cat_table"))]
incb_extend = [(cp(a), cp(b)) for a, b in re.findall(r"\('\\u\{([0-9a-f]+)\}',\s*'\\u\{([0-9a-f]+)\}'\)", block("const InCB_Extend_table"))]
linkers = [cp(x) for x in re.findall(r"'\\u\{([0-9A-F]+)\}'", source[source.index("fn is_incb_linker"):source.index("\n}", source.index("fn is_incb_linker"))])]

categories = ["Any", "CR", "LF", "Control", "Extend", "ZWJ", "RegionalIndicator", "Prepend", "SpacingMark", "L", "V", "T", "LV", "LVT", "ExtendedPictographic", "InCBConsonant"]
renames = {"Regional_Indicator": "RegionalIndicator", "Extended_Pictographic": "ExtendedPictographic", "InCB_Consonant": "InCBConsonant"}


def wrap(items, width=110):
    lines, line = [], "   "
    for item in items:
        if len(line) + len(item) + 1 > width:
            lines.append(line)
            line = "   "
        line += " " + item
    if line.strip():
        lines.append(line)
    return "\n".join(lines)


out = []
out.append("//! 🔤️ Unicode 17.0 data behind the terminal text model: grapheme cluster break classes (UAX #29),")
out.append("//! `InCB` conjunct data, scalar cell widths (UAX #11 plus emoji presentation) and emoji presentation bases.")
out.append("//! Extracted once from unicode-segmentation 1.13.2 and unicode-width 0.2.2; the oracle tests in")
out.append("//! `🧪️tests/📏️text-width` re-derive every entry from those crates, so a Unicode upgrade fails loudly.")
out.append("")
out.append("/// 🔤️ Grapheme_Cluster_Break class of a scalar (`InCBConsonant` and `ExtendedPictographic` join the UAX #29 classes).")
out.append("#[derive(Clone, Copy, Debug, PartialEq, Eq)]")
out.append("pub(super) enum Break {")
for name in categories:
    out.append(f"    {name},")
out.append("}")
out.append("")
out.append("use Break::*;")
out.append("")
out.append("/// 🔤️ Non-`Any` break classes as sorted inclusive `(first, last, class)` ranges.")
out.append("pub(super) const BREAKS: &[(u32, u32, Break)] = &[")
out.append(wrap([f"(0x{a:04x}, 0x{b:04x}, {renames.get(c, c)})," for a, b, c in grapheme]))
out.append("];")
out.append("")
out.append("/// 🔤️ Scalars with `InCB=Extend` as sorted inclusive ranges.")
out.append("pub(super) const INCB_EXTEND: &[(u32, u32)] = &[")
out.append(wrap([f"(0x{a:04x}, 0x{b:04x})," for a, b in incb_extend]))
out.append("];")
out.append("")
out.append("/// 🔤️ Scalars with `InCB=Linker`.")
out.append("pub(super) const INCB_LINKER: &[u32] = &[")
out.append(wrap([f"0x{x:04x}," for x in sorted(linkers)]))
out.append("];")
out.append("")
out.append(width_tables.strip())
out.append("")
text = "\n".join(out)
with open(target, "w", encoding="utf-8", newline="\n") as handle:
    handle.write(text)
print(f"breaks={len(grapheme)} incb_extend={len(incb_extend)} linkers={len(linkers)} bytes={len(text)}")
