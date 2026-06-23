#!/usr/bin/env python3
"""Transform Rust lib.rs: split grouped mods, rename comments, reorder."""

import re
import copy

# Canonical emoji+name for each mod
CANONICAL = {
    "header": ("\U0001f9f2", "Header"),
    "imports": ("\u26e9\ufe0f", "Imports"),
    "error_types": ("\u26a0\ufe0f", "Exceptions"),
    "utility_functions": ("\U0001f4e6", "Utilities"),
    "model_types_attribute": ("\U0001f48e", "Attribute"),
    "attribute": ("\U0001f48e", "Attribute"),
    "model_types_coord": ("\U0001f4fa", "Coord"),
    "coord": ("\U0001f4fa", "Coord"),
    "model_types_vector": ("\u2197\ufe0f", "Vector"),
    "vector": ("\u2197\ufe0f", "Vector"),
    "model_types_plane": ("\u25fb\ufe0f", "Plane"),
    "plane": ("\u25fb\ufe0f", "Plane"),
    "model_types_camera": ("\U0001f3a5", "Camera"),
    "camera": ("\U0001f3a5", "Camera"),
    "location": ("\U0001f4cd", "Location"),
    "author": ("\u270d\ufe0f", "Author"),
    "file_entity": ("\U0001f4c4", "File"),
    "folder_entity": ("\U0001f4c1", "Folder"),
    "benchmark_entity": ("\U0001f4cf", "Benchmark"),
    "benchmark": ("\U0001f3cb\ufe0f", "Benchmarks"),
    "quality": ("\U0001f52c", "Quality"),
    "port": ("\u2693", "Port"),
    "tag": ("\U0001f3f7\ufe0f", "Tag"),
    "concept": ("\U0001f4a1", "Concept"),
    "prop": ("\U0001f4ca", "Prop"),
    "model_entity": ("\U0001f5ff", "Model"),
    "connector": ("\U0001f50c", "Connector"),
    "model_types_type": ("\U0001f9f1", "Type"),
    "type_entity": ("\U0001f9f1", "Type"),
    "model_types_layer_piece_group_side_connection_stat": (
        "\U0001f3a8",
        "Layer+Piece+Group+Side+Connection+Stat",
    ),
    "layer": ("\U0001f3a8", "Layer"),
    "piece": ("\U0001f9e9", "Piece"),
    "design_id": ("\U0001f4d0", "DesignId"),
    "group": ("\U0001f465", "Group"),
    "side": ("\u2194\ufe0f", "Side"),
    "connection": ("\U0001f517", "Connection"),
    "stat": ("\U0001f4c8", "Stat"),
    "model_types_design": ("\U0001f4d0", "Design"),
    "design": ("\U0001f4d0", "Design"),
    "model_types_kit": ("\u23f1\ufe0f", "Kit"),
    "kit": ("\u23f1\ufe0f", "Kit"),
    "finder_functions": ("\U0001f50d", "Helpers"),
    "serialization": ("\u23f0", "Serialization"),
    "diff_types": ("\u2702\ufe0f", "Diff Types"),
    "meta_and_shallow_types": ("\U0001f511", "Meta And Shallow"),
    "has_guid_trait": ("\U0001f40d", "Entity IDs"),
    "apply_diff": ("\U0001f3aa", "Kit Operations"),
    "kit_diff_validation": ("\U0001f4e6", "Kit Diff Validation"),
    "kit_change_helpers": ("\U0001f30a", "Kit Change Helpers"),
    "filter": ("\U0001f50d", "Filter"),
    "flatten_design": ("\U0001f324\ufe0f", "Flatten Design"),
    "copy_paste_design": ("\U0001f4cb", "Copy Paste Design"),
    "kit_model_export": ("\U0001f529", "Kit Model Export"),
    "geometric_insights": ("\u2744\ufe0f", "Geometric Insights"),
    "validation_types": ("\U0001f6e1\ufe0f", "Validation"),
    "sqlite_import_export": ("\U0001f4e1", "SQLite"),
    "zip_import_export": ("\U0001f4e6", "ZipRoundtrip"),
    "kit_workflow": ("\U0001f3d7\ufe0f", "Kit Workflow"),
    "wasm_bindings": ("\U0001f947", "WASM Bindings"),
    "hash": ("\U0001f5a5\ufe0f", "Hash"),
    "tests": ("\U0001f9ea", "Tests"),
}

# Canonical section order (by canonical name)
CANONICAL_ORDER = [
    "Header",
    "Imports",
    "Exceptions",
    "Utilities",
    "Entity IDs",
    "Attribute",
    "Coord",
    "Vector",
    "Plane",
    "Camera",
    "Location",
    "Author",
    "File",
    "Folder",
    "Benchmark",
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
    "Kit Operations",
    "Kit Diff Validation",
    "Kit Change Helpers",
    "Filter",
    "Helpers",
    "Flatten Design",
    "Copy Paste Design",
    "Kit Model Export",
    "Geometric Insights",
    "Validation",
    "SQLite",
    "ZipRoundtrip",
    "Kit Workflow",
    "WASM Bindings",
    "Hash",
    "Tests",
    "Benchmarks",  # benchmark tests at end
]


def get_order_key(canonical_name):
    if canonical_name in CANONICAL_ORDER:
        return CANONICAL_ORDER.index(canonical_name)
    return 9999


def make_mod_block(mod_name, canonical_name, emoji, inner_lines):
    """Create a mod block with canonical comment labeling."""
    result = []
    result.append(f"mod {mod_name} {{ // {emoji}{canonical_name}")
    result.append(f"    // {emoji}{canonical_name}")
    result.append(
        f"    // {emoji}{canonical_name} MUST provide the {canonical_name.lower()} functionality."
    )
    result.append("")
    result.append("    use super::*;")
    result.append("")
    result.extend(inner_lines)
    result.append(f"}} // {emoji}{canonical_name}")
    return result


def extract_inner_content(mod_lines):
    """Extract inner content from a mod block (skip boilerplate header)."""
    # Skip: mod line, comment lines, use super::*, blank lines at start
    inner = mod_lines[1:-1]  # Remove mod open and close brace
    content_start = 0
    for j, line in enumerate(inner):
        s = line.strip()
        if (
            s.startswith("//")
            or s.startswith("use super")
            or s == ""
            or s.startswith("#[cfg")
        ):
            content_start = j + 1
        else:
            break
    return inner[content_start:]


def split_entity_lines(content_lines, entity_groups):
    """Split content lines into entity groups based on struct/enum names.
    entity_groups: list of (label, [struct_names])
    Returns: {label: [lines]}
    """
    result = {label: [] for label, _ in entity_groups}
    name_to_label = {}
    for label, struct_names in entity_groups:
        for sn in struct_names:
            name_to_label[sn] = label

    current_label = None
    pending = []

    for line in content_lines:
        s = line.strip()
        # Check for struct/enum definition - use stripped line for matching
        struct_m = re.match(r"pub struct (\w+)", s)
        enum_m = re.match(r"pub enum (\w+)", s)

        matched_name = None
        if struct_m and struct_m.group(1) in name_to_label:
            matched_name = struct_m.group(1)
        elif enum_m and enum_m.group(1) in name_to_label:
            matched_name = enum_m.group(1)

        if matched_name:
            new_label = name_to_label[matched_name]
            if new_label != current_label:
                if pending and current_label:
                    deriv_start = None
                    for pi in range(len(pending) - 1, -1, -1):
                        ps = pending[pi].strip()
                        if ps.startswith("#[") or ps.startswith("///"):
                            deriv_start = pi
                        elif ps == "":
                            continue
                        else:
                            break
                    if deriv_start is not None:
                        result[current_label].extend(pending[:deriv_start])
                        result[new_label].extend(pending[deriv_start:])
                    else:
                        result[current_label].extend(pending)
                elif pending:
                    result[new_label].extend(pending)
                pending = []
                current_label = new_label

        pending.append(line)

    # Flush remaining
    if pending and current_label:
        result[current_label].extend(pending)

    return result


def main():
    filepath = "compose/rs/lib.rs"
    with open(filepath, "r", encoding="utf-8") as f:
        content = f.read()
    original_lines = content.split("\n")
    print(f"Original: {len(original_lines)} lines")

    # ================================================================
    # Step 1: Parse file into segments
    # ================================================================
    segments = []
    i = 0
    in_mod = False
    mod_start = 0
    mod_name = ""
    depth = 0

    while i < len(original_lines):
        line = original_lines[i]
        s = line.rstrip()
        if not in_mod:
            m = re.match(r"^(pub\s+)?mod\s+(\w+)\s*\{", s)
            if m:
                mod_name = m.group(2)
                mod_start = i
                in_mod = True
                depth = s.count("{") - s.count("}")
                if depth == 0:
                    segments.append(
                        {"kind": "mod", "name": mod_name, "lines": [original_lines[i]]}
                    )
                    in_mod = False
                i += 1
                continue
            pu = re.match(r"^pub use (\w+)::\*;", s)
            if pu:
                segments.append(
                    {
                        "kind": "pub_use",
                        "name": pu.group(1),
                        "lines": [original_lines[i]],
                    }
                )
                i += 1
                continue
            segments.append({"kind": "other", "name": "", "lines": [original_lines[i]]})
            i += 1
        else:
            depth += s.count("{") - s.count("}")
            if depth <= 0:
                mod_lines = original_lines[mod_start : i + 1]
                segments.append({"kind": "mod", "name": mod_name, "lines": mod_lines})
                in_mod = False
                depth = 0
            i += 1

    print(f"Parsed {len(segments)} segments")

    # ================================================================
    # Step 2: Split grouped mods
    # ================================================================
    SPLIT_CONFIGS = {
        "model_types_location_author_file_folder": [
            ("location", ["LocationId", "Location"]),
            ("author", ["AuthorId", "Author"]),
            ("folder_entity", ["FolderId", "Folder"]),
            ("file_entity", ["FileId", "File"]),
        ],
        "model_types_quality_port_tag_concept": [
            ("benchmark_entity", ["BenchmarkId", "Benchmark"]),
            ("quality", ["QualityId", "QualityKind", "Quality"]),
            ("port", ["PortId", "Port"]),
            ("tag", ["TagId", "Tag"]),
            ("concept", ["ConceptId", "Concept"]),
        ],
        "model_types_prop_model_connector": [
            ("prop", ["PropId", "Prop"]),
            ("model_entity", ["ModelId", "Model"]),
            ("connector", ["ConnectorId", "Connector"]),
        ],
        "model_types_layer_piece_group_side_connection_stat": [
            ("layer", ["LayerId", "Layer"]),
            ("piece", ["PieceId", "DesignId", "Piece"]),
            ("group", ["GroupId", "Group"]),
            ("side", ["Side"]),
            ("connection", ["ConnectionId", "Connection"]),
            ("stat", ["StatId", "Stat"]),
        ],
    }

    new_segments = []
    for seg in segments:
        if seg["kind"] == "mod" and seg["name"] in SPLIT_CONFIGS:
            config = SPLIT_CONFIGS[seg["name"]]
            inner = extract_inner_content(seg["lines"])
            splits = split_entity_lines(inner, config)

            for new_mod_name, struct_names in config:
                entity_lines = splits.get(new_mod_name, [])
                if not entity_lines:
                    continue
                emoji, canonical_name = CANONICAL.get(new_mod_name, ("", new_mod_name))
                mod_block = make_mod_block(
                    new_mod_name, canonical_name, emoji, entity_lines
                )
                new_segments.append(
                    {"kind": "mod", "name": new_mod_name, "lines": mod_block}
                )
                new_segments.append(
                    {
                        "kind": "pub_use",
                        "name": new_mod_name,
                        "lines": [f"pub use {new_mod_name}::*;"],
                    }
                )
                new_segments.append({"kind": "other", "name": "", "lines": [""]})

            print(f"  Split mod {seg['name']} into {len(config)} mods")
        elif seg["kind"] == "pub_use" and seg["name"] in SPLIT_CONFIGS:
            # Skip old pub use for grouped mod (replaced by individual pub uses)
            pass
        else:
            new_segments.append(seg)

    segments = new_segments

    # ================================================================
    # Step 3: Rename mod comments to canonical format
    # ================================================================
    for seg in segments:
        if seg["kind"] != "mod":
            continue
        if seg["name"] not in CANONICAL:
            continue
        emoji, canonical_name = CANONICAL[seg["name"]]
        label = f"{emoji}{canonical_name}"

        # Update opening line
        old_open = seg["lines"][0]
        m = re.match(r"^((?:pub\s+)?mod\s+\w+\s*\{)(.*)", old_open)
        if m:
            seg["lines"][0] = f"{m.group(1)} // {label}"

        # Update closing line
        old_close = seg["lines"][-1]
        cm = re.match(r"^(\})(.*)", old_close)
        if cm:
            seg["lines"][-1] = f"}} // {label}"

    # ================================================================
    # Step 4: Reorder - group mod+pub_use+trailing_other as units
    # ================================================================
    # First, collect preamble (everything before first mod)
    preamble = []
    first_mod_idx = None
    for idx, seg in enumerate(segments):
        if seg["kind"] == "mod":
            first_mod_idx = idx
            break
        preamble.append(seg)

    if first_mod_idx is None:
        print("ERROR: No mods found")
        return

    # Collect use statements (between imports mod and error_types mod)
    # These are root-level use statements that should stay after Imports
    use_statements = []
    mod_units = []  # List of (canonical_name, [segments])

    idx = first_mod_idx
    while idx < len(segments):
        seg = segments[idx]
        if seg["kind"] == "mod":
            canonical_name = CANONICAL.get(seg["name"], ("", seg["name"]))[1]
            unit = [seg]
            # Collect pub_use and blank lines after mod
            j = idx + 1
            while j < len(segments) and segments[j]["kind"] in ("pub_use", "other"):
                if segments[j]["kind"] == "pub_use":
                    unit.append(segments[j])
                    j += 1
                elif segments[j]["kind"] == "other":
                    s = segments[j]["lines"][0].strip() if segments[j]["lines"] else ""
                    if s == "" or s.startswith("//"):
                        unit.append(segments[j])
                        j += 1
                    else:
                        break
                else:
                    break
            mod_units.append((canonical_name, unit))
            idx = j
        elif seg["kind"] == "other":
            s = seg["lines"][0].strip() if seg["lines"] else ""
            if s.startswith("use ") or s.startswith("#[cfg"):
                use_statements.append(seg)
            elif s == "":
                pass  # skip stray blanks
            else:
                use_statements.append(seg)
            idx += 1
        elif seg["kind"] == "pub_use":
            # Stray pub_use not attached to mod
            idx += 1
        else:
            idx += 1

    # Sort mod_units by canonical order
    mod_units.sort(key=lambda x: get_order_key(x[0]))

    # ================================================================
    # Step 5: Reassemble
    # ================================================================
    output_lines = []

    # Preamble
    for seg in preamble:
        output_lines.extend(seg["lines"])

    # Mods in order
    for canonical_name, unit in mod_units:
        # Special: use statements go right after Imports mod
        for seg in unit:
            output_lines.extend(seg["lines"])
        if canonical_name == "Imports":
            for seg in use_statements:
                output_lines.extend(seg["lines"])
        # Ensure blank line between sections
        if output_lines and output_lines[-1].strip():
            output_lines.append("")

    # Write result
    result = "\n".join(output_lines)
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(result)

    new_line_count = len(output_lines)
    print(f"Result: {new_line_count} lines (was {len(original_lines)})")

    # Print new mod order
    print("\nNew mod order:")
    for canonical_name, unit in mod_units:
        mod_seg = unit[0]
        print(
            f"  [{get_order_key(canonical_name)}] {canonical_name} (mod {mod_seg['name']}, {len(mod_seg['lines'])} lines)"
        )


if __name__ == "__main__":
    main()
