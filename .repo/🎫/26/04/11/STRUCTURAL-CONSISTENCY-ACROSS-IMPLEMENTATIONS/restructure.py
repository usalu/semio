"""
Restructure script for semio implementations.
Handles: section reordering, nesting fixes, block moves.
"""

import re
import sys
import shutil
from pathlib import Path

BASE = Path(r"c:\git\semio")

# File configs: (path, start_pattern, end_pattern, comment_prefix)
FILES = {
    "go": (
        BASE / "semio/go/main.go",
        r"^// #region (.+)$",
        r"^// #endregion (.+)$",
        "// ",
    ),
    "ts": (
        BASE / "semio/js/index.ts",
        r"^// #region (.+)$",
        r"^// #endregion (.+)$",
        "// ",
    ),
    "py": (
        BASE / "semio/py/main.py",
        r"^# #region (.+)$",
        r"^# #endregion (.+)$",
        "# ",
    ),
    "cs": (
        BASE / "semio/net/Semio/Semio.cs",
        r"^#region (.+)$",
        r"^#endregion (.+)$",
        "",
    ),
    "rs": (BASE / "semio/rs/lib.rs", r"^mod (\w+) \{ // (.+)$", r"^\} // (.+)$", ""),
}

# Canonical top-level section order (shared sections in required relative order)
CANONICAL_ORDER = [
    "Header",
    "Imports",
    "Namespace",
    "Type Hints",
    "Constants",
    "Utilities",
    "Logging",
    "Exceptions",
    "Modeling",
    "Entitying",
    "SemioValidation",
    "Entity IDs",
    "Weak Entities",
    "Attribute",
    "Coord",
    "MoveVector",
    "Vec",
    "Point",
    "Vector",
    "Plane",
    "Camera",
    "Location",
    "Author",
    "ArtifactAuthor",
    "File",
    "Folder",
    "Benchmark",
    "QualityKind",
    "Quality",
    "Port",
    "Prop",
    "Tag",
    "Concept",
    "Model",
    "Connector",
    "Type",
    "Layer",
    "Piece",
    "Group",
    "Side",
    "Connection",
    "Stat",
    "Design",
    "Kit",
    "Serialization",
    "Diff Types",
    "Meta And Shallow",
    "Hash",
    "Design Family Helpers",
    "Type Family Helpers",
    "Helpers",
    "Factories",
    "Kit Finders",
    "Kit Operations",
    "Api",
    "OperationResult",
    "Filter",
    "Kit Diff Validation",
    "Kit Change Helpers",
    "Kit Diff Operations",
    "Copy Paste Design",
    "Moved Graphene Nodes",
    "Validation",
    "Flatten Design",
    "Kit Import/Export",
    "Kit Model Export",
    "Geometric Insights",
    "Spatial Math",
    "SQLite",
    "ZipRoundtrip",
    "KitSqlite",
    "Kit Workflow",
    "FileKit",
    "FolderKit",
    "ArchiveKit",
    "RemoteKit",
    "TemporaryKit",
    "KitImporter",
    "KitExporter",
    "KitStore",
    "File Tree Utilities",
    "SemioDiff",
    "WASM Bindings",
    "Tests",
    "Benchmarks",
]


def strip_emoji(name):
    """Strip leading emoji from section name to get the canonical key."""
    # Remove all leading non-ASCII chars and whitespace
    i = 0
    while i < len(name):
        c = name[i]
        if ord(c) > 127 or c in "\ufe0f\u200d":
            i += 1
        elif c == " ":
            i += 1
            break
        else:
            break
    return name[i:].strip() if i > 0 else name.strip()


def get_order_key(section_name):
    """Get canonical order index for a section name."""
    key = strip_emoji(section_name)
    try:
        return CANONICAL_ORDER.index(key)
    except ValueError:
        # Unknown section - put it at the end
        print(f"  WARNING: Unknown section '{key}' (from '{section_name}')")
        return 9999


def parse_top_level_sections_rs(lines):
    """Parse Rust mod blocks as top-level sections."""
    sections = []
    i = 0
    gap_start = 0

    while i < len(lines):
        line = lines[i].rstrip()
        m = re.match(r"^mod (\w+) \{ // (.+)$", line)
        if m:
            mod_name = m.group(1)
            section_name = m.group(2)
            # Find matching close
            depth = 1
            start = i
            j = i + 1
            while j < len(lines) and depth > 0:
                l = lines[j].rstrip()
                # Count nested mod opens
                if re.match(r"^\s*mod \w+ \{", l):
                    depth += 1
                if l.startswith("} //") and depth == 1:
                    depth = 0
                elif l == "}" and depth > 0:
                    # Could be end of nested block - check next line
                    pass
                # Simple brace counting for mod blocks
                for ch in l:
                    if ch == "{":
                        pass  # already counted mod open
                    elif ch == "}":
                        pass
                j += 1

            # Find the closing } // Name
            j = i + 1
            brace_depth = 1
            while j < len(lines):
                l = lines[j]
                # Count braces
                for ch in l:
                    if ch == "{":
                        brace_depth += 1
                    elif ch == "}":
                        brace_depth -= 1
                if brace_depth == 0:
                    break
                j += 1

            # Gap before this section
            if i > gap_start:
                sections.append(("gap", lines[gap_start:i]))

            sections.append(("section", section_name, lines[i : j + 1]))
            # pub use after mod
            k = j + 1
            while k < len(lines) and lines[k].strip().startswith("pub use " + mod_name):
                k += 1
            if k > j + 1:
                sections[-1] = ("section", section_name, lines[i:k])
                gap_start = k
            else:
                gap_start = j + 1
            i = gap_start
        else:
            i += 1

    if gap_start < len(lines):
        sections.append(("gap", lines[gap_start:]))

    return sections


def parse_top_level_sections_generic(lines, start_re, end_re):
    """Parse region-based top-level sections."""
    sections = []
    i = 0
    gap_start = 0

    while i < len(lines):
        line = lines[i].rstrip()
        m = re.match(start_re, line)
        if m:
            section_name = m.group(1)
            # Find matching endregion at same depth
            depth = 1
            j = i + 1
            while j < len(lines) and depth > 0:
                l = lines[j].rstrip()
                if re.match(start_re, l):
                    depth += 1
                if re.match(end_re, l):
                    depth -= 1
                if depth > 0:
                    j += 1
                    continue
                break

            # Gap before section
            if i > gap_start:
                sections.append(("gap", lines[gap_start:i]))

            sections.append(("section", section_name, lines[i : j + 1]))
            gap_start = j + 1
            i = gap_start
        else:
            i += 1

    if gap_start < len(lines):
        sections.append(("gap", lines[gap_start:]))

    return sections


def reorder_sections(sections):
    """Reorder sections by canonical order. Gaps attach to PRECEDING section."""
    # Build pairs: (section, trailing_gap)
    # First collect leading gap (before any section)
    leading_gap = None
    pairs = []  # list of (section, trailing_gap)
    trailing_gap = None

    for item in sections:
        if item[0] == "gap":
            if not pairs:
                # Before any section - this is the leading gap
                if leading_gap is None:
                    leading_gap = item
                else:
                    leading_gap = ("gap", leading_gap[1] + item[1])
            else:
                # After a section - attach to preceding section
                if trailing_gap is not None:
                    trailing_gap = ("gap", trailing_gap[1] + item[1])
                else:
                    trailing_gap = item
        else:
            # Flush any trailing gap to previous section
            if pairs and trailing_gap:
                sec, _ = pairs[-1]
                pairs[-1] = (sec, trailing_gap)
                trailing_gap = None
            elif trailing_gap:
                # trailing gap before this section but after no section - merge to leading
                if leading_gap:
                    leading_gap = ("gap", leading_gap[1] + trailing_gap[1])
                else:
                    leading_gap = trailing_gap
                trailing_gap = None
            pairs.append((item, None))

    # Flush final trailing gap
    if pairs and trailing_gap:
        sec, _ = pairs[-1]
        pairs[-1] = (sec, trailing_gap)
    final_trailing = None
    if not pairs and trailing_gap:
        final_trailing = trailing_gap

    # Sort by canonical order
    def sort_key(pair):
        section, _ = pair
        return get_order_key(section[1])

    sorted_pairs = sorted(pairs, key=sort_key)

    # Flatten back to lines
    output = []
    if leading_gap:
        output.extend(leading_gap[1])
    for section, gap in sorted_pairs:
        output.extend(section[2])
        if gap:
            output.extend(gap[1])
    if final_trailing:
        output.extend(final_trailing[1])

    return output


def analyze(lang):
    """Show current section order for a language."""
    path, start_re, end_re, _ = FILES[lang]
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    if lang == "rs":
        sections = parse_top_level_sections_rs(lines)
    else:
        sections = parse_top_level_sections_generic(lines, start_re, end_re)

    print(f"\n=== {lang.upper()} ({len(lines)} lines) ===")
    for i, item in enumerate(sections):
        if item[0] == "section":
            name = item[1]
            key = strip_emoji(name)
            order = get_order_key(name)
            nlines = len(item[2])
            print(f"  [{order:3d}] {name} ({nlines} lines)")
        else:
            nlines = len(item[1])
            if nlines > 2:
                print(f"  [gap] ({nlines} lines)")


def reorder(lang, dry_run=False):
    """Reorder sections in a language file."""
    path, start_re, end_re, _ = FILES[lang]
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()

    if lang == "rs":
        sections = parse_top_level_sections_rs(lines)
    else:
        sections = parse_top_level_sections_generic(lines, start_re, end_re)

    print(f"\n=== {lang.upper()} REORDER ===")
    print("Before:")
    for item in sections:
        if item[0] == "section":
            print(f"  [{get_order_key(item[1]):3d}] {item[1]}")

    output = reorder_sections(sections)

    print(f"\nLines: {len(lines)} -> {len(output)}")

    if not dry_run:
        tmp = str(path) + ".tmp"
        with open(tmp, "w", encoding="utf-8", newline="") as f:
            f.writelines(output)
        shutil.move(tmp, str(path))
        print(f"Written {path}")
    else:
        print("(dry run - no changes)")


if __name__ == "__main__":
    cmd = sys.argv[1] if len(sys.argv) > 1 else "analyze"
    lang = sys.argv[2] if len(sys.argv) > 2 else "all"
    dry_run = "--dry" in sys.argv

    langs = list(FILES.keys()) if lang == "all" else [lang]

    if cmd == "analyze":
        for l in langs:
            analyze(l)
    elif cmd == "reorder":
        for l in langs:
            reorder(l, dry_run=dry_run)
    else:
        print(f"Unknown command: {cmd}")
