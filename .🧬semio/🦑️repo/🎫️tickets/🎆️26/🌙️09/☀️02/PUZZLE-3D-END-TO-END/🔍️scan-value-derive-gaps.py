#!/usr/bin/env python3
"""🔍️ Scans the repo for the half-applied value_derive migration pattern.

Ticket 26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS converts serde derives to
`value_derive::ToValue`/`FromValue`. Some sites got the item-level `#[value(...)]` container attribute
without the derives that register it. This finds every remaining occurrence: an item-level
`#[value(...)]` (immediately preceding a struct/enum declaration, not a field) whose preceding
`#[derive(...)]` lacks `value_derive::ToValue`.
"""
import re
import sys
from pathlib import Path

ROOTS = ["✏️s", "🧰️framework", "🌎️hub"]
EXCLUDE_DIRS = {"target", "node_modules", ".🧬semio"}
ITEM_DECL_RE = re.compile(r"^\s*(pub(\(\w+\))?\s+)?(struct|enum)\s+([A-Za-z_][A-Za-z0-9_]*)")
DERIVE_RE = re.compile(r"^\s*#\[derive\(")
ATTR_OR_COMMENT_RE = re.compile(r"^\s*(#\[|///|//!|//|$)")


def find_rs_files(root: Path):
    for p in root.rglob("*.rs"):
        if any(part in EXCLUDE_DIRS for part in p.parts):
            continue
        yield p


def scan_file(path: Path):
    hits = []
    try:
        text = path.read_text(encoding="utf-8")
    except UnicodeDecodeError:
        return hits
    lines = text.split("\n")
    for index, line in enumerate(lines):
        if not line.lstrip().startswith("#[value("):
            continue
        # walk forward to find next non-attribute, non-comment, non-blank line
        cursor = index + 1
        # value(...) attribute may span multiple lines if unbalanced parens; handle simple single-line case,
        # but also skip continuation lines that don't start a new attribute (rare) by tracking paren balance.
        paren_depth = line.count("(") - line.count(")")
        while paren_depth > 0 and cursor < len(lines):
            paren_depth += lines[cursor].count("(") - lines[cursor].count(")")
            cursor += 1
        while cursor < len(lines):
            l = lines[cursor]
            if not l.strip():
                cursor += 1
                continue
            if l.lstrip().startswith("#["):
                # skip this attribute, accounting for multi-line attrs
                depth = l.count("(") - l.count(")")
                cursor += 1
                while depth > 0 and cursor < len(lines):
                    depth += lines[cursor].count("(") - lines[cursor].count(")")
                    cursor += 1
                continue
            if l.lstrip().startswith("///") or l.lstrip().startswith("//!") or l.lstrip().startswith("//"):
                cursor += 1
                continue
            break
        if cursor >= len(lines):
            continue
        decl_line = lines[cursor]
        m = ITEM_DECL_RE.match(decl_line)
        if not m:
            continue  # not an item-level #[value(...)] (likely field-level)
        type_name = m.group(4)

        # walk backward through the WHOLE contiguous attribute/comment block above
        # #[value(...)] (there may be multiple stacked #[derive(...)] attributes --
        # Rust merges them -- so any one of them carrying ToValue/FromValue is enough).
        b = index - 1
        derive_positions = []  # (start_idx, end_idx) nearest-first
        while b >= 0:
            bl = lines[b]
            if not bl.strip():
                b -= 1
                continue
            if bl.lstrip().startswith("///") or bl.lstrip().startswith("//!") or bl.lstrip().startswith("//"):
                b -= 1
                continue
            if bl.lstrip().startswith("#["):
                # this line is the END of an attribute (walking backward); find its start
                end = b
                depth = bl.count(")") - bl.count("(")
                start = b
                while depth > 0 and start - 1 >= 0:
                    start -= 1
                    depth += lines[start].count(")") - lines[start].count("(")
                if DERIVE_RE.match(lines[start]):
                    derive_positions.append((start, end))
                b = start - 1
                continue
            break  # hit non-attribute content: end of the attribute block
        if not derive_positions:
            hits.append((path, index + 1, type_name, "NO_DERIVE_FOUND", None))
            continue
        any_fixed = False
        for start, end in derive_positions:
            full_derive = "\n".join(lines[start:end + 1])
            if "ToValue" in full_derive or "FromValue" in full_derive:
                any_fixed = True
                break
        if any_fixed:
            continue  # already fixed (has at least one of the two, on any stacked derive)
        # target the derive nearest to #[value(...)] for the fix
        derive_idx, derive_end = derive_positions[0]
        hits.append((path, index + 1, type_name, "MISSING_BOTH", (derive_idx, derive_end)))
    return hits


def main():
    all_hits = []
    for root_name in ROOTS:
        root = Path(root_name)
        if not root.exists():
            print(f"WARN: root not found: {root_name}", file=sys.stderr)
            continue
        for f in find_rs_files(root):
            hits = scan_file(f)
            all_hits.extend(hits)
    for path, lineno, type_name, status, derive_range in all_hits:
        print(f"{path}:{lineno}\t{type_name}\t{status}\t{derive_range}")
    print(f"TOTAL: {len(all_hits)}", file=sys.stderr)


if __name__ == "__main__":
    main()
