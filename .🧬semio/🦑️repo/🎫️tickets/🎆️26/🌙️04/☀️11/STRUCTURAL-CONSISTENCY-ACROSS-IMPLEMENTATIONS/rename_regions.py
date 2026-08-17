#!/usr/bin/env python3
"""Rename all region markers to canonical emojis/names across compose implementations."""

import re
import os
import sys

CANONICAL = {
    "header": ("\U0001f9f2", "Header"),
    "imports": ("\u26e9\ufe0f", "Imports"),
    "type hints": ("\U0001f4dd", "Type Hints"),
    "constants": ("\U0001f39e\ufe0f", "Constants"),
    "utilities": ("\U0001f4e6", "Utilities"),
    "utils": ("\U0001f4e6", "Utilities"),
    "utility": ("\U0001f4e6", "Utilities"),
    "logging": ("\U0001f4f0", "Logging"),
    "exceptions": ("\u26a0\ufe0f", "Exceptions"),
    "namespace": ("\U0001f3e0", "Namespace"),
    "expressions": ("\u2744\ufe0f", "Expressions"),
    "entitying": ("\U0001f513", "Entitying"),
    "composevalidation": ("\u2728", "ComposeValidation"),
    "modeling": ("\U0001f3b2", "Modeling"),
    "primitives": ("\U0001f43b", "Primitives"),
    "graphql": ("\U0001f3ac", "Graphql"),
    "domain": ("\U0001f9e9", "Domain"),
    "entity ids": ("\U0001f40d", "Entity IDs"),
    "weak entities": ("\U0001f5a5\ufe0f", "Weak Entities"),
    "attribute": ("\U0001f48e", "Attribute"),
    "coord": ("\U0001f4fa", "Coord"),
    "coord (weak entity)": ("\U0001f4fa", "Coord"),
    "vec": ("\u27a1\ufe0f", "Vec"),
    "vec (weak entity)": ("\u27a1\ufe0f", "Vec"),
    "point": ("\u2716\ufe0f", "Point"),
    "point (weak entity)": ("\u2716\ufe0f", "Point"),
    "vector": ("\u2197\ufe0f", "Vector"),
    "vector (weak entity)": ("\u2197\ufe0f", "Vector"),
    "plane": ("\u25fb\ufe0f", "Plane"),
    "plane (weak entity)": ("\u25fb\ufe0f", "Plane"),
    "camera": ("\U0001f3a5", "Camera"),
    "camera (weak entity)": ("\U0001f3a5", "Camera"),
    "location": ("\U0001f4cd", "Location"),
    "author": ("\u270d\ufe0f", "Author"),
    "artifactauthor": ("\U0001f525", "ArtifactAuthor"),
    "file": ("\U0001f4c4", "File"),
    "folder": ("\U0001f4c1", "Folder"),
    "benchmark": ("\U0001f4cf", "Benchmark"),
    "qualitykind": ("\U0001f5a8\ufe0f", "QualityKind"),
    "quality": ("\U0001f52c", "Quality"),
    "port": ("\u2693", "Port"),
    "prop": ("\U0001f4ca", "Prop"),
    "tag": ("\U0001f3f7\ufe0f", "Tag"),
    "concept": ("\U0001f4a1", "Concept"),
    "model": ("\U0001f5ff", "Model"),
    "connector": ("\U0001f50c", "Connector"),
    "compatibleport": ("\U0001fa99", "CompatiblePort"),
    "type": ("\U0001f9f1", "Type"),
    "layer": ("\U0001f3a8", "Layer"),
    "piece": ("\U0001f9e9", "Piece"),
    "group": ("\U0001f465", "Group"),
    "side": ("\u2194\ufe0f", "Side"),
    "connection": ("\U0001f517", "Connection"),
    "stat": ("\U0001f4c8", "Stat"),
    "design": ("\U0001f4d0", "Design"),
    "kit": ("\u23f1\ufe0f", "Kit"),
    "kitkind": ("\U0001f9ec", "KitKind"),
    "kit kind": ("\U0001f9ec", "KitKind"),
    "serialization": ("\u23f0", "Serialization"),
    "meta and shallow types": ("\U0001f511", "Meta And Shallow"),
    "meta and shallow": ("\U0001f511", "Meta And Shallow"),
    "metashallow": ("\U0001f511", "Meta And Shallow"),
    "meta/shallow conversions": ("\U0001f511", "Meta And Shallow"),
    "meta and shallow conversion functions": (
        "\U0001f4ce",
        "Meta And Shallow Conversions",
    ),
    "metashallowconversions": ("\U0001f4ce", "Meta And Shallow Conversions"),
    "sub-entity meta types": ("\U0001f3bc", "Sub-entity Meta"),
    "sub-entity meta": ("\U0001f3bc", "Sub-entity Meta"),
    "subentitymeta": ("\U0001f3bc", "Sub-entity Meta"),
    "main entity meta types": ("\U0001f570\ufe0f", "Main Entity Meta"),
    "main entity meta": ("\U0001f570\ufe0f", "Main Entity Meta"),
    "shallow types": ("\U0001f43b", "Shallow"),
    "shallow": ("\U0001f43b", "Shallow"),
    "typemetashallow": ("\U0001fa81", "TypeMetaShallow"),
    "designmetashallow": ("\u2728", "DesignMetaShallow"),
    "kitmetashallow": ("\U0001f3d7\ufe0f", "KitMetaShallow"),
    "hash": ("\U0001f5a5\ufe0f", "Hash"),
    "sha-256": ("\U0001f537", "SHA-256"),
    "hashwriter": ("\U0001f329\ufe0f", "HashWriter"),
    "hash value types": ("\U0001f3b5", "Hash Value Types"),
    "hash entities": ("\U0001f3a9", "Hash Entities"),
    "hash diffs": ("\U0001f517", "Hash Diffs"),
    "hash diff value types": ("\U0001f439", "Hash Diff Value Types"),
    "hash diff entities": ("\u2697\ufe0f", "Hash Diff Entities"),
    "design family helpers": ("\U0001f4fb", "Design Family Helpers"),
    "type family helpers": ("\U0001f9ca", "Type Family Helpers"),
    "kit finders": ("\U0001f50d", "Kit Finders"),
    "kit query helpers": ("\U0001f50d", "Kit Finders"),
    "helpers": ("\U0001f50d", "Helpers"),
    "factories": ("\U0001f5e1\ufe0f", "Factories"),
    "filter": ("\U0001f3a0", "Filter"),
    "kit operations": ("\U0001f3aa", "Kit Operations"),
    "kit change helpers": ("\U0001f30a", "Kit Change Helpers"),
    "kit diff validation": ("\U0001f4e6", "Kit Diff Validation"),
    "kitdiffvalidation": ("\U0001f4e6", "Kit Diff Validation"),
    "operationresult": ("\U0001f3af", "OperationResult"),
    "flatten design": ("\U0001f324\ufe0f", "Flatten Design"),
    "flattendesign": ("\U0001f324\ufe0f", "Flatten Design"),
    "copy paste design": ("\U0001f4cb", "Copy Paste Design"),
    "validation": ("\U0001f6e1\ufe0f", "Validation"),
    "validation serialization": ("\U0001f327\ufe0f", "Validation Serialization"),
    "validation core types": ("\U0001f5e1\ufe0f", "Validation Core Types"),
    "validation context & engine": ("\U0001f50d", "Validation Context And Engine"),
    "fix helper": ("\U0001f4e1", "Fix Helper"),
    "guid update helper": ("\U0001f511", "GUID Update Helper"),
    "kit import/export": ("\U0001f9ff", "Kit Import/Export"),
    "kit model export": ("\U0001f529", "Kit Model Export"),
    "exportdesignmodel": ("\U0001f529", "Kit Model Export"),
    "kit model export helpers": ("\U0001f527", "Kit Model Export Helpers"),
    "exportdesignmodel/helpers": ("\U0001f527", "Kit Model Export Helpers"),
    "exportdesignmodel/bfs": ("\U0001f326\ufe0f", "Kit Model Export BFS"),
    "exportdesignmodel/meshdata": ("\u2699\ufe0f", "Kit Model Export MeshData"),
    "exportdesignmodel/buildgltf": ("\U0001f4bb", "Kit Model Export BuildGLTF"),
    "ifc export": ("\U0001f4fb", "IFC Export"),
    "geometric insights": ("\u2744\ufe0f", "Geometric Insights"),
    "spatial math": ("\U0001f50d", "Spatial Math"),
    "sqlite kit operations": ("\U0001f4e1", "SQLite"),
    "sqlite": ("\U0001f4e1", "SQLite"),
    "kit workflow operations": ("\U0001f504", "Kit Workflow"),
    "kit workflow": ("\U0001f504", "Kit Workflow"),
    "kit workflow helpers": ("\U0001f504", "Kit Workflow Helpers"),
    "moved graphene nodes": ("\U0001f9ed", "Moved Graphene Nodes"),
    "dict-based validation": ("\U0001f4e7", "Dict-based Validation"),
    "graph operations": ("\U0001f54c", "Graph Operations"),
    "clustering": ("\U0001f3a1", "Clustering"),
    "kit query helpers dict": ("\U0001f4cd", "Kit Query Helpers Dict"),
    "kit diff operations": ("\U0001f397\ufe0f", "Kit Diff Operations"),
    "api": ("\U0001f3aa", "Api"),
    "kitsqliteload": ("\U0001f948", "KitSqliteLoad"),
    "kitsqlitesave": ("\U0001f3d7\ufe0f", "KitSqliteSave"),
    "kitsqlitechange": ("\U0001f941", "KitSqliteChange"),
    "kitsqlite": ("\U0001f513", "KitSqlite"),
    "ziproundtrip": ("\U0001f3aa", "ZipRoundtrip"),
    "filekit": ("\U0001f4f7", "FileKit"),
    "folderkit": ("\U0001f3f0", "FolderKit"),
    "archivekit": ("\U0001f4d0", "ArchiveKit"),
    "remotekit": ("\U0001f386", "RemoteKit"),
    "temporarykit": ("\U0001f524", "TemporaryKit"),
    "kitimporter": ("\U0001f4e6", "KitImporter"),
    "kitexporter": ("\U0001fa81", "KitExporter"),
    "composediff": ("\u2744\ufe0f", "ComposeDiff"),
    "file tree utilities": ("\U0001f54c", "File Tree Utilities"),
    "kitstore": ("\U0001f3f0", "KitStore"),
    "inmemorykitstore": ("\U0001f5a5\ufe0f", "InMemoryKitStore"),
    "tests": ("\U0001f9ea", "Tests"),
    "test": ("\U0001f9ea", "Tests"),
    "benchmarks": ("\U0001f3cb\ufe0f", "Benchmarks"),
    "benchmark": ("\U0001f4cf", "Benchmark"),
    "benchmarks": ("\U0001f3cb\ufe0f", "Benchmarks"),
    "load or create meshes per type": ("\U0001f381", "Load Or Create Meshes Per Type"),
    "build scene graph with connection document": (
        "\U0001f9ed",
        "Build Scene Graph With Connection Document",
    ),
    "kit filter tests": ("\U0001f3f0", "Kit Filter Tests"),
    "kitkind tests": ("\U0001f6e1\ufe0f", "KitKind Tests"),
    "copy & paste tests": ("\U0001f4cb", "Copy And Paste Tests"),
    "inmemorykitstore tests": ("\U0001f30a", "InMemoryKitStore Tests"),
    "jsonfilekitstore tests": ("\u26c5", "JsonFileKitStore Tests"),
    "folderkitstore tests": ("\U0001f50a", "FolderKitStore Tests"),
    "meta/shallow tests": ("\U0001f380", "Meta And Shallow Tests"),
    "hash tests": ("\U0001f5dd\ufe0f", "Hash Tests"),
    "maxchildren tests": ("\U0001f4ca", "MaxChildren Tests"),
    "constraint: guid uniqueness": ("\U0001f511", "Constraint: GUID Uniqueness"),
    "constraint: type name uniqueness": (
        "\U0001f9f1",
        "Constraint: Type Name Uniqueness",
    ),
    "constraint: design name uniqueness": (
        "\U0001f4d0",
        "Constraint: Design Name Uniqueness",
    ),
    "constraint: piece name uniqueness": (
        "\U0001f9e9",
        "Constraint: Piece Name Uniqueness",
    ),
    "constraint: quality name uniqueness": (
        "\U0001f52c",
        "Constraint: Quality Name Uniqueness",
    ),
    "constraint: port name uniqueness": ("\u2693", "Constraint: Port Name Uniqueness"),
    "constraint: file name uniqueness": (
        "\U0001f4c4",
        "Constraint: File Name Uniqueness",
    ),
    "constraint: folder name uniqueness": (
        "\U0001f4c1",
        "Constraint: Folder Name Uniqueness",
    ),
    "constraint: connector name uniqueness within type": (
        "\U0001f50c",
        "Constraint: Connector Name Uniqueness Within Type",
    ),
    "constraint: model name uniqueness within type": (
        "\U0001f5ff",
        "Constraint: Model Name Uniqueness Within Type",
    ),
    "constraint: layer path uniqueness within design": (
        "\U0001f3a8",
        "Constraint: Layer Path Uniqueness Within Design",
    ),
    "constraint: design piece same family constraint": (
        "\U0001f4d0",
        "Constraint: Design Piece Same Family Constraint",
    ),
    "constraint registration": ("\u2705", "Constraint Registration"),
    # IFC Export steps (Python only)
    "step 1: ifc file, project, units, context, spatial tree from layers": (
        "\U0001f5a8\ufe0f",
        "Step 1: IFC File Project Units Context Spatial Tree From Layers",
    ),
    "step 1: ifc file project units context spatial tree from layers": (
        "\U0001f5a8\ufe0f",
        "Step 1: IFC File Project Units Context Spatial Tree From Layers",
    ),
    "step 1": ("\U0001f5a8\ufe0f", "Step 1"),
    "step 2: piece-to-storey mapping from piece names": (
        "\U0001f4cb",
        "Step 2: Piece-to-storey Mapping From Piece Names",
    ),
    "step 2: piece-to-storey mapping": (
        "\U0001f4cb",
        "Step 2: Piece-to-storey Mapping",
    ),
    "step 2": ("\U0001f4cb", "Step 2"),
    "step 3: types with geometry": ("\U0001f6d5", "Step 3: Types With Geometry"),
    "step 3": ("\U0001f6d5", "Step 3"),
    "step 4: pieces as occurrences": ("\U0001f388", "Step 4: Pieces As Occurrences"),
    "step 4": ("\U0001f388", "Step 4"),
    "step 5: connections as port relationships": (
        "\U0001f32a\ufe0f",
        "Step 5: Connections As Port Relationships",
    ),
    "step 5": ("\U0001f32a\ufe0f", "Step 5"),
    "step 6: kit-level metadata": ("\U0001f3c6", "Step 6: Kit-level Metadata"),
    "step 6": ("\U0001f3c6", "Step 6"),
    # Already-canonical forms that need to be recognized on re-runs
    "kit model export bfs": ("\U0001f326\ufe0f", "Kit Model Export BFS"),
    "kit model export meshdata": ("\u2699\ufe0f", "Kit Model Export MeshData"),
    "kit model export buildgltf": ("\U0001f4bb", "Kit Model Export BuildGLTF"),
    "validation context and engine": ("\U0001f50d", "Validation Context And Engine"),
    "copy and paste tests": ("\U0001f4cb", "Copy And Paste Tests"),
    "meta and shallow tests": ("\U0001f380", "Meta And Shallow Tests"),
    "meta and shallow conversions": ("\U0001f4ce", "Meta And Shallow Conversions"),
}


def strip_emoji(text):
    """Strip leading non-ASCII characters (emojis) from text."""
    result = []
    for ch in text:
        if ord(ch) < 128:
            result.append(ch)
    return "".join(result).strip()


def process_file(filepath):
    """Process a single file: rename all region markers to canonical form."""
    with open(filepath, "r", encoding="utf-8") as f:
        lines = f.readlines()

    changes = 0
    warnings = []
    new_lines = []

    for i, line in enumerate(lines):
        stripped = line.strip()

        # Fix malformed Go marker: ✏️#region 🌩️HashWriter
        if (
            "#region" in stripped
            and "HashWriter" in stripped
            and stripped.count("#region") > 1
        ):
            indent = line[: len(line) - len(line.lstrip())]
            new_lines.append(indent + "// #region \U0001f329\ufe0fHashWriter\n")
            changes += 1
            continue

        # Fix malformed Go marker: //#endregion 🎬️Hash (no space)
        if (
            stripped.startswith("//#endregion")
            and "Hash" in stripped
            and "Hash " not in stripped
        ):
            indent = line[: len(line) - len(line.lstrip())]
            new_lines.append(indent + "// #endregion \U0001f5a5\ufe0fHash\n")
            changes += 1
            continue

        # Try matching region patterns
        m = None
        style = None
        for pattern, s in [
            (r"^(\s*)(//\s*#(end)?region\s+)(.*?)(\s*)$", "slash"),
            (r"^(\s*)(#\s*#(end)?region\s+)(.*?)(\s*)$", "hash"),
            (r"^(\s*)(#(end)?region\s+)(.*?)(\s*)$", "csharp"),
        ]:
            m = re.match(pattern, line.rstrip("\n"))
            if m:
                style = s
                break

        if not m:
            new_lines.append(line)
            continue

        indent = m.group(1)
        prefix = m.group(2)
        is_end = m.group(3) is not None
        rest = m.group(4)

        name = strip_emoji(rest)
        if not name:
            new_lines.append(line)
            continue

        key = name.lower().strip()
        if key not in CANONICAL:
            warnings.append(f"  WARN L{i + 1}: no mapping for '{name}' (key='{key}')")
            new_lines.append(line)
            continue

        c_emoji, c_name = CANONICAL[key]
        tag = "endregion" if is_end else "region"

        if filepath.endswith(".go") or filepath.endswith(".ts"):
            new_line = f"{indent}// #{tag} {c_emoji}{c_name}\n"
        elif filepath.endswith(".py"):
            new_line = f"{indent}# #{tag} {c_emoji}{c_name}\n"
        elif filepath.endswith(".cs"):
            if prefix.strip().startswith("//"):
                new_line = f"{indent}// #{tag} {c_emoji}{c_name}\n"
            else:
                new_line = f"{indent}#{tag} {c_emoji}{c_name}\n"
        else:
            new_line = line

        if new_line.rstrip() != line.rstrip():
            changes += 1
        new_lines.append(new_line)

    for w in warnings:
        print(w)

    with open(filepath, "w", encoding="utf-8") as f:
        f.writelines(new_lines)

    return changes


# ===============================================================
# CANONICAL SECTION ORDER
# ===============================================================
CANONICAL_ORDER = [
    # Infrastructure
    "Header",
    "Imports",
    "Namespace",
    "Type Hints",
    "Constants",
    "Utilities",
    "Logging",
    "Exceptions",
    "Expressions",
    "Entitying",
    "ComposeValidation",
    "Modeling",
    # Entity IDs
    "Entity IDs",
    "Weak Entities",
    # Domain Entities
    "Attribute",
    "Coord",
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
    # Operations
    "Serialization",
    "Meta And Shallow",
    "Hash",
    "Helpers",
    "Design Family Helpers",
    "Type Family Helpers",
    "Kit Finders",
    "Factories",
    "Filter",
    "Kit Operations",
    "Kit Change Helpers",
    "Kit Diff Validation",
    "OperationResult",
    "Moved Graphene Nodes",
    "Validation",
    "Flatten Design",
    "Copy Paste Design",
    "Kit Diff Operations",
    "Kit Import/Export",
    "Kit Model Export",
    "Geometric Insights",
    "Spatial Math",
    "SQLite",
    "Kit Workflow",
    "Api",
    "KitSqlite",
    "ZipRoundtrip",
    "FileKit",
    "FolderKit",
    "ArchiveKit",
    "RemoteKit",
    "TemporaryKit",
    "KitImporter",
    "KitExporter",
    "Kit Diff Validation",
    "ComposeDiff",
    "KitStore",
    "File Tree Utilities",
    "InMemoryKitStore",
    # Testing
    "Tests",
    "Benchmarks",
]


def get_section_order_key(name):
    """Get sort key for a section based on canonical order."""
    # Strip emoji prefix
    clean = strip_emoji(name)
    if clean in CANONICAL_ORDER:
        return CANONICAL_ORDER.index(clean)
    # Unknown sections go at the end
    return 9999


def parse_top_level_sections(lines, filepath):
    """Parse a file into top-level sections delimited by region markers.

    For C#, only matches non-indented #region/#endregion (file-level markers),
    not indented ones inside class bodies.
    """

    # Detect region pattern - applied to STRIPPED lines for Go/TS/Py
    # For C#, we match against the ORIGINAL line to exclude indented markers
    if filepath.endswith(".go") or filepath.endswith(".ts"):

        def is_region(line):
            m = re.match(r"^// #region\s+(.+)$", line.strip())
            return m.group(1) if m else None

        def is_endregion(line):
            return bool(re.match(r"^// #endregion\s+(.+)$", line.strip()))
    elif filepath.endswith(".py"):

        def is_region(line):
            m = re.match(r"^# #region\s+(.+)$", line.strip())
            return m.group(1) if m else None

        def is_endregion(line):
            return bool(re.match(r"^# #endregion\s+(.+)$", line.strip()))
    elif filepath.endswith(".cs"):

        def is_region(line):
            # Only match file-level (non-indented) #region markers
            m = re.match(r"^#region\s+(.+)$", line.rstrip())
            return m.group(1) if m else None

        def is_endregion(line):
            return bool(re.match(r"^#endregion\s+(.+)$", line.rstrip()))
    else:
        return None

    sections = []
    depth = 0
    current_start = None
    current_name = None
    preamble_start = 0

    for i, line in enumerate(lines):
        region_name = is_region(line)
        endregion = is_endregion(line)

        if region_name and depth == 0:
            # Start of a new top-level section
            # Save any preamble (code before this section)
            if preamble_start < i:
                gap_lines = lines[preamble_start:i]
                if sections or any(l.strip() for l in gap_lines):
                    sections.append(
                        {
                            "name": "__gap__",
                            "start": preamble_start,
                            "end": i,
                            "lines": gap_lines,
                        }
                    )
            current_start = i
            current_name = strip_emoji(region_name)
            depth = 1
        elif region_name and depth > 0:
            depth += 1
        elif endregion and depth > 0:
            depth -= 1
            if depth == 0:
                # End of top-level section
                sections.append(
                    {
                        "name": current_name,
                        "start": current_start,
                        "end": i + 1,
                        "lines": lines[current_start : i + 1],
                    }
                )
                preamble_start = i + 1
                current_start = None
                current_name = None
        elif endregion and depth == 0:
            # Stray endregion - treat as part of gap
            pass

    # Remaining lines after last section
    if preamble_start < len(lines):
        remaining = lines[preamble_start:]
        if any(l.strip() for l in remaining):
            sections.append(
                {
                    "name": "__trailing__",
                    "start": preamble_start,
                    "end": len(lines),
                    "lines": remaining,
                }
            )

    return sections


def reorder_file(filepath):
    """Reorder top-level sections according to canonical order."""
    with open(filepath, "r", encoding="utf-8") as f:
        lines = f.readlines()

    sections = parse_top_level_sections(lines, filepath)
    if sections is None:
        print("  Unsupported file format")
        return

    # Print current order
    print("  Current section order:")
    for s in sections:
        if s["name"] not in ("__gap__", "__trailing__"):
            idx = (
                CANONICAL_ORDER.index(s["name"])
                if s["name"] in CANONICAL_ORDER
                else "?"
            )
            print(f"    [{idx}] {s['name']} (L{s['start'] + 1}-L{s['end']})")

    # Separate gaps, named sections, and trailing
    # Attach each gap to the FOLLOWING named section so gaps move with their section
    named = []
    pending_gap = None
    for s in sections:
        if s["name"] == "__gap__":
            pending_gap = s
        elif s["name"] == "__trailing__":
            continue  # handled separately
        else:
            if pending_gap:
                # Prepend gap lines to this section
                s = dict(s)
                s["lines"] = pending_gap["lines"] + s["lines"]
                pending_gap = None
            named.append(s)
    trailing = [s for s in sections if s["name"] == "__trailing__"]

    # Sort named sections by canonical order
    named.sort(key=lambda s: get_section_order_key(s["name"]))

    # Reassemble
    new_lines = []
    for s in named:
        # Add blank line separator between sections
        if new_lines and new_lines[-1].strip():
            new_lines.append("\n")
        new_lines.extend(s["lines"])

    for s in trailing:
        new_lines.extend(s["lines"])

    with open(filepath, "w", encoding="utf-8") as f:
        f.writelines(new_lines)

    print("  Reordered section order:")
    reordered = parse_top_level_sections(new_lines, filepath)
    for s in reordered:
        if s["name"] not in ("__gap__", "__trailing__"):
            idx = (
                CANONICAL_ORDER.index(s["name"])
                if s["name"] in CANONICAL_ORDER
                else "?"
            )
            print(f"    [{idx}] {s['name']} (L{s['start'] + 1}-L{s['end']})")


if __name__ == "__main__":
    base = os.path.dirname(os.path.abspath(__file__))
    while not os.path.exists(os.path.join(base, "compose")):
        parent = os.path.dirname(base)
        if parent == base:
            break
        base = parent

    files = [
        os.path.join(base, "compose", "go", "main.go"),
        os.path.join(base, "compose", "js", "index.ts"),
        os.path.join(base, "compose", "py", "main.py"),
        os.path.join(base, "compose", "net", "Compose", "Compose.cs"),
    ]

    mode = "rename"
    if len(sys.argv) > 1:
        mode = sys.argv[1]

    if mode == "rename":
        for fp in files:
            print(f"\n=== {os.path.basename(fp)} ===")
            n = process_file(fp)
            print(f"  {n} changes")
    elif mode == "reorder":
        for fp in files:
            print(f"\n=== {os.path.basename(fp)} ===")
            reorder_file(fp)
