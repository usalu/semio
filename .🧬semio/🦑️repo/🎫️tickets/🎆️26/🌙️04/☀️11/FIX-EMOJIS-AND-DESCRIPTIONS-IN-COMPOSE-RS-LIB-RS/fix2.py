"""Fix all doc comment emojis and descriptions in compose/rs/lib.rs."""

import re
import sys

file_path = r"c:\git\compose\compose\rs\lib.rs"

with open(file_path, "r", encoding="utf-8") as f:
    lines = f.readlines()

# Canonical emoji mapping
ENTITY_EMOJI = {
    "Attribute": "\U0001f48e",  # 💎️
    "AttributeId": "\U0001f48e",
    "Location": "\U0001f4cd",  # 📍️
    "LocationId": "\U0001f4cd",
    "Author": "\u270d\ufe0f",  # ✍️
    "AuthorId": "\u270d\ufe0f",
    "File": "\U0001f4c4",  # 📄️
    "FileId": "\U0001f4c4",
    "Folder": "\U0001f4c1",  # 📁️
    "FolderId": "\U0001f4c1",
    "Benchmark": "\U0001f4cf",  # 📏️
    "BenchmarkId": "\U0001f4cf",
    "Quality": "\U0001f52c",  # 🔬️
    "QualityId": "\U0001f52c",
    "QualityKind": "\U0001f52c",
    "Port": "\u2693",  # ⚓️
    "PortId": "\u2693",
    "Prop": "\U0001f4ca",  # 📊️
    "PropId": "\U0001f4ca",
    "Tag": "\U0001f3f7\ufe0f",  # 🏷️
    "TagId": "\U0001f3f7\ufe0f",
    "Concept": "\U0001f4a1",  # 💡️
    "ConceptId": "\U0001f4a1",
    "Model": "\U0001f5ff",  # 🗿️
    "ModelId": "\U0001f5ff",
    "Connector": "\U0001f50c",  # 🔌️
    "ConnectorId": "\U0001f50c",
    "Type": "\U0001f9f1",  # 🧱️
    "TypeId": "\U0001f9f1",
    "Layer": "\U0001f3a8",  # 🎨️
    "LayerId": "\U0001f3a8",
    "Piece": "\U0001f9e9",  # 🧩️
    "PieceId": "\U0001f9e9",
    "Group": "\U0001f465",  # 👥️
    "GroupId": "\U0001f465",
    "Side": "\u2194\ufe0f",  # ↔
    "Connection": "\U0001f517",  # 🔗️
    "ConnectionId": "\U0001f517",
    "Stat": "\U0001f4c8",  # 📈️
    "StatId": "\U0001f4c8",
    "Design": "\U0001f4d0",  # 📐️
    "DesignId": "\U0001f4d0",
    "Kit": "\U0001f4e6",  # 📦️
    "Coord": "\U0001f4fa",  # 📺️
    "Vec": "\u27a1\ufe0f",  # ➡️
    "Point": "\u2716\ufe0f",  # ✖️
    "Vector": "\u2197\ufe0f",  # ↗️
    "Plane": "\u25fb\ufe0f",  # ◻
    "Camera": "\U0001f3a5",  # 🎥️
}

# Entity descriptions
ENTITY_DESC = {
    "Attribute": "a key-value metadata entry with optional definition",
    "AttributeId": "identifies an attribute entity by GUID",
    "Location": "a geographic point with longitude, latitude and optional altitude",
    "LocationId": "identifies a location entity by GUID",
    "Author": "a named contributor with email and custom attributes",
    "AuthorId": "identifies an author entity by GUID",
    "File": "a named binary resource with optional remote URL and folder",
    "FileId": "identifies a file entity by GUID",
    "Folder": "a named directory for organizing files",
    "FolderId": "identifies a folder entity by GUID",
    "Benchmark": "a named metric range with min/max bounds and optional icon",
    "BenchmarkId": "identifies a benchmark entity by GUID",
    "Quality": "a measurable property with formula, units and benchmarks",
    "QualityId": "identifies a quality entity by GUID",
    "QualityKind": "the numeric kind of a quality (integer, float or boolean)",
    "Port": "a named connection interface with compatible ports",
    "PortId": "identifies a port entity by GUID",
    "Prop": "a quality measurement value with optional unit",
    "PropId": "identifies a prop entity by GUID",
    "Tag": "a named categorization label with optional description and icon",
    "TagId": "identifies a tag entity by GUID",
    "Concept": "a named categorization concept with optional description and icon",
    "ConceptId": "identifies a concept entity by GUID",
    "Model": "a 3D model reference linking a file with tags and description",
    "ModelId": "identifies a model entity by GUID",
    "Connector": "a connection point on a type with position, direction and parameter",
    "ConnectorId": "identifies a connector entity by GUID",
    "Type": "a reusable element blueprint with connectors, models and props",
    "TypeId": "identifies a type entity by GUID",
    "Layer": "a named visibility and color layer within a design",
    "LayerId": "identifies a layer entity by GUID",
    "Piece": "a positioned instance of a type within a design",
    "PieceId": "identifies a piece entity by GUID",
    "Group": "a named collection of pieces within a design",
    "GroupId": "identifies a group entity by GUID",
    "Side": "one side of a connection identifying a piece and optional connector",
    "Connection": "a spatial relationship between two pieces with gap, shift and rotation",
    "ConnectionId": "identifies a connection entity by GUID",
    "Stat": "a statistical quality measurement with min/max bounds and unit",
    "StatId": "identifies a stat entity by GUID",
    "Design": "an assembly of pieces, connections, layers and groups",
    "DesignId": "identifies a design entity by GUID",
    "Kit": "the root container for all domain entities",
    "Coord": "a 2D coordinate with U and V components",
    "Vec": "a 2D vector with U and V components",
    "Point": "a 3D point with X, Y and Z components",
    "Vector": "a 3D vector with X, Y and Z components",
    "Plane": "a plane defined by origin point and two axis vectors",
    "Camera": "a camera defined by position, forward and up vectors",
}

# Diff descriptions
DIFF_DESC = {}
for entity in [
    "Attribute",
    "Prop",
    "Connector",
    "Model",
    "Type",
    "Side",
    "Connection",
    "Piece",
    "Layer",
    "Group",
    "Stat",
    "Design",
    "Tag",
    "Concept",
    "Port",
    "Quality",
    "File",
    "Folder",
    "Author",
    "Kit",
    "Benchmark",
    "Location",
]:
    base = entity.lower()
    DIFF_DESC[f"{entity}Diff"] = f"a partial update to {base}'s fields"

# Change descriptions
CHANGE_DESC = {}
for entity in [
    "Attribute",
    "Author",
    "File",
    "Folder",
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
]:
    CHANGE_DESC[f"{entity}Change"] = (
        f"tracks {entity.lower()} modifications in a kit change"
    )
CHANGE_DESC["KitChange"] = "tracks kit-level modifications"

# Meta descriptions
META_DESC = {}
for entity in [
    "Attribute",
    "Stat",
    "Tag",
    "Concept",
    "Prop",
    "Author",
    "File",
    "Folder",
    "Quality",
    "Port",
    "Connector",
    "Model",
    "Type",
    "Layer",
    "Piece",
    "Group",
    "Side",
    "Connection",
    "Design",
    "Kit",
    "Benchmark",
    "Location",
]:
    META_DESC[f"{entity}Meta"] = (
        f"scalar-only view of {entity.lower()} excluding nested arrays"
    )

# Function descriptions
FUNC_DESC = {
    "ComposeError": (
        "\u274c",
        "ComposeError represents a domain error with context message",
    ),
    "Result": ("\u2705", "Result represents a success or ComposeError outcome"),
    "Guid": ("\U0001f511", "Guid represents a UUID string identifier"),
    "guid": ("\U0001f511", "generates a new v7 UUID string"),
    "normalize": ("\U0001f4d0", "rounds a float to the given number of decimal places"),
    "round": ("\U0001f4d0", "rounds a float to 3 decimal places"),
    "jaccard": ("\U0001f4ca", "computes Jaccard similarity between two sets"),
    "deep_equal": ("\U0001f504", "compares two serializable values for deep equality"),
    "generate_unique_name": (
        "\U0001f4dd",
        "generates a unique name avoiding collisions with existing names",
    ),
    "RemovedItem": (
        "\U0001f5d1\ufe0f",
        "RemovedItem represents an entity marked for removal by GUID",
    ),
    "DiffUpdate": (
        "\U0001f504",
        "DiffUpdate represents a before-after pair for entity updates",
    ),
    "Change": (
        "\U0001f504",
        "Change represents a tracked modification with timestamp and author",
    ),
    "CollectionDiff": (
        "\U0001f504",
        "CollectionDiff represents batched entity additions, removals and updates",
    ),
    "ValidationProblem": (
        "\u26a0\ufe0f",
        "ValidationProblem represents a validation issue with severity and location",
    ),
    "ValidationFix": (
        "\U0001f527",
        "ValidationFix represents a suggested fix for a validation problem",
    ),
    "ValidationResult": (
        "\u2705",
        "ValidationResult represents the outcome of a kit validation",
    ),
    "SUPPORTED_MODEL_EXTENSIONS": (
        "\U0001f5ff",
        "the list of supported 3D model file extensions",
    ),
    "validate_kit": (
        "\u2705",
        "validates a kit for structural and referential integrity",
    ),
    "sqlite": ("\U0001f4be", "SQLite database import and export operations"),
    "zip_roundtrip": ("\U0001f4e6", "ZIP archive round-trip import and export"),
    "wasm": ("\U0001f310", "WebAssembly bindings for the compose library"),
    "tests": ("\U0001f9ea", "unit and integration tests for the compose library"),
    "flatten_design": (
        "\U0001f4d0",
        "flattens nested design references into a single design",
    ),
    "planes_equal_approx": (
        "\u25fb\ufe0f",
        "compares two planes for approximate equality",
    ),
    "quat_to_matrix4": ("\U0001f504", "converts quaternion to a 4x4 rotation matrix"),
}

# Regex to strip any emoji prefix (catches multi-char emojis and variation selectors)
EMOJI_RE = re.compile(
    r"[\U0001F000-\U0001FFFF\u2000-\u3300\u200d\uFE00-\uFE0F\u20E3\u2600-\u27BF"
    r"\u2300-\u23FF\u2B05-\u2B55\u25A0-\u25FF\u2702-\u27B0\u2764\uFE0F"
    r"\U0001FA00-\U0001FA6F\U0001FA70-\U0001FAFF]*"
)


def strip_emojis(s):
    """Remove all emojis/variation selectors from start of string."""
    result = s
    while result:
        m = EMOJI_RE.match(result)
        if m and m.end() > 0:
            result = result[m.end() :]
        else:
            break
    return result


def get_entity_emoji(name):
    """Get canonical emoji for an entity name."""
    if name in ENTITY_EMOJI:
        return ENTITY_EMOJI[name]
    # For Diff types
    base = name.replace("Diff", "").replace("Change", "").replace("Meta", "")
    if base in ENTITY_EMOJI:
        return ENTITY_EMOJI[base]
    return None


def get_description(name):
    """Get canonical description for any entity/func name."""
    if name in ENTITY_DESC:
        return f"{name} represents {ENTITY_DESC[name]}"
    if name in DIFF_DESC:
        emoji = get_entity_emoji(name) or "\U0001f504"
        return f"{name} represents {DIFF_DESC[name]}"
    if name in CHANGE_DESC:
        return f"{name} represents {CHANGE_DESC[name]}"
    if name in META_DESC:
        return f"{name} represents {META_DESC[name]}"
    if name in FUNC_DESC:
        return FUNC_DESC[name][1]
    return None


def get_emoji_for(name):
    """Get the emoji to use for a given name."""
    if name in ENTITY_EMOJI:
        return ENTITY_EMOJI[name]
    if name in FUNC_DESC:
        return FUNC_DESC[name][0]
    e = get_entity_emoji(name)
    if e:
        return e
    return "\U0001f527"  # 🔧️ default


# Pattern: "/// <anything><summary><anything>WORD holds/represents..." or similar broken patterns
SUMMARY_RE = re.compile(r"^(\s*///\s*)(.*?)(<summary>)(.*?)(</summary>)(.*?)$")
# Pattern for the specific broken lines: "/// <summary>🔧️xyz.</summary>"
BROKEN_RE = re.compile(r"^(\s*///\s*)<summary>(.+?)</summary>\s*$")

fix_count = 0
new_lines = []
skip_next = False

for i, line in enumerate(lines):
    if skip_next:
        skip_next = False
        continue

    stripped = line.rstrip("\n").rstrip("\r")

    # ================================================================
    # PATTERN A: Lines with <summary> tag
    # ================================================================
    m = SUMMARY_RE.match(stripped)
    if m:
        prefix = m.group(1)  # "    /// "
        before_tag = m.group(2)  # stuff before <summary> (emoji junk)
        tag_open = m.group(3)  # "<summary>"
        content = m.group(4)  # content inside summary
        tag_close = m.group(5)  # "</summary>"
        after = m.group(6)  # stuff after </summary>

        # Strip all emojis from content to find the real word
        clean_content = strip_emojis(content)

        # Check if this is a known entity/function pattern
        # Pattern: "EntityName holds the data fields..."
        # Or: "EntityName represents ..."  (already partially fixed)
        # Or broken: just "d." or "ror." etc.

        name_match = re.match(
            r"(\w+)\s+(?:holds the data fields|represents)", clean_content
        )
        if name_match:
            name = name_match.group(1)
            desc = get_description(name)
            emoji = get_emoji_for(name)
            if desc:
                new_line = f"{prefix}<summary>{emoji}{desc}.</summary>"
                new_lines.append(new_line + "\n")
                fix_count += 1
                # Check for duplicate summary on next line
                if i + 1 < len(lines):
                    next_stripped = lines[i + 1].rstrip("\n").rstrip("\r")
                    next_m = SUMMARY_RE.match(next_stripped)
                    if next_m:
                        next_content = strip_emojis(next_m.group(4))
                        next_name_match = re.match(
                            r"(\w+)\s+(?:holds|represents)", next_content
                        )
                        if next_name_match and next_name_match.group(1) == name:
                            skip_next = True
                            fix_count += 1
                continue

        # Pattern: "Default holds..." or "Default implementation..."
        default_match = re.match(r"Default\s+(?:holds|implementation)", clean_content)
        if default_match:
            # Look ahead to find what entity
            entity = None
            for j in range(i + 1, min(i + 5, len(lines))):
                dm = re.search(r"impl Default for (\w+)", lines[j])
                if dm:
                    entity = dm.group(1)
                    break
            if entity:
                emoji = get_emoji_for(entity)
                new_line = f"{prefix}<summary>{emoji}Default implementation for {entity}.</summary>"
                new_lines.append(new_line + "\n")
                fix_count += 1
                continue

        # Pattern: function descriptions like "find_type_in_kit..."
        func_match = re.match(r"(\w+)\s", clean_content)
        if not func_match:
            func_match = re.match(r"(\w+)\.", clean_content)
        if func_match:
            func_name = func_match.group(1)
            # Check known functions
            if func_name in FUNC_DESC:
                emoji, desc = FUNC_DESC[func_name]
                new_line = f"{prefix}<summary>{emoji}{desc}.</summary>"
                new_lines.append(new_line + "\n")
                fix_count += 1
                continue
            # Check entity types
            if func_name in ENTITY_DESC:
                emoji = get_emoji_for(func_name)
                desc = get_description(func_name)
                new_line = f"{prefix}<summary>{emoji}{desc}.</summary>"
                new_lines.append(new_line + "\n")
                fix_count += 1
                continue
            if func_name in DIFF_DESC:
                emoji = get_emoji_for(func_name)
                desc = get_description(func_name)
                new_line = f"{prefix}<summary>{emoji}{desc}.</summary>"
                new_lines.append(new_line + "\n")
                fix_count += 1
                continue
            if func_name in CHANGE_DESC:
                emoji = get_emoji_for(func_name)
                desc = get_description(func_name)
                new_line = f"{prefix}<summary>{emoji}{desc}.</summary>"
                new_lines.append(new_line + "\n")
                fix_count += 1
                continue

        # Now handle the broken truncated patterns
        # These are lines like: "/// <summary>🔧️ror.</summary>" where the first script ate the name
        # We need to identify what this line is FOR by looking at context
        bm = BROKEN_RE.match(stripped)
        if bm:
            content_inner = bm.group(2)
            clean = strip_emojis(content_inner)

            # Already good patterns (already fixed correctly by first script)
            # Check if this has a proper description
            if re.match(r"\w{3,}.*\w{3,}", clean):
                # Looks like it has real content, might be OK
                # But check for the specific "ror.", "t.", "d.", "ize.", "qual." etc. broken patterns
                if re.match(r"^[a-z]{1,10}\.$", clean):
                    # This is a truncated/broken description - need to figure out what it was
                    pass  # Fall through to context-based fix below
                else:
                    new_lines.append(line)
                    continue

            # Broken - figure out from context what this should be
            # Look ahead for the next struct/fn/type/enum/impl/pub line
            entity_name = None
            for j in range(i + 1, min(i + 8, len(lines))):
                nl = lines[j].strip()
                # pub struct Name
                sm = re.match(r"pub\s+(?:struct|enum|type)\s+(\w+)", nl)
                if sm:
                    entity_name = sm.group(1)
                    break
                # pub fn name
                fm = re.match(r"pub\s+fn\s+(\w+)", nl)
                if fm:
                    entity_name = fm.group(1)
                    break
                # impl HasGuid for Name
                im = re.match(r"impl\s+(?:HasGuid|DiffHasGuid)\s+for\s+(\w+)", nl)
                if im:
                    entity_name = "HasGuid_" + im.group(1)
                    break
                # impl Default for Name
                dm2 = re.match(r"impl\s+Default\s+for\s+(\w+)", nl)
                if dm2:
                    entity_name = "Default_" + dm2.group(1)
                    break
                # impl Name
                im2 = re.match(r"impl\s+(\w+)\s*\{", nl)
                if im2:
                    entity_name = im2.group(1)
                    break
                # pub use or }
                if nl.startswith("pub use") or nl == "}" or nl.startswith("use"):
                    # Check for fn/type further
                    em = re.match(r"pub\s+type\s+(\w+)", nl)
                    if em:
                        entity_name = em.group(1)
                        break
                    continue
                # pub const NAME
                cm = re.match(r"pub\s+const\s+(\w+)", nl)
                if cm:
                    entity_name = cm.group(1)
                    break

            if entity_name:
                # Handle HasGuid impl
                if entity_name.startswith("HasGuid_"):
                    impl_entity = entity_name[8:]
                    emoji = get_entity_emoji(impl_entity)
                    if not emoji:
                        emoji = "\U0001f511"
                    new_line = f"{bm.group(1)}<summary>{emoji}HasGuid implementation for {impl_entity}.</summary>"
                    new_lines.append(new_line + "\n")
                    fix_count += 1
                    continue
                # Handle DiffHasGuid impl
                elif entity_name.startswith("DiffHasGuid_"):
                    impl_entity = entity_name[12:]
                    base_entity = impl_entity.replace("Diff", "")
                    emoji = get_entity_emoji(base_entity) or "\U0001f511"
                    new_line = f"{bm.group(1)}<summary>{emoji}DiffHasGuid implementation for {impl_entity}.</summary>"
                    new_lines.append(new_line + "\n")
                    fix_count += 1
                    continue
                # Handle Default impl
                elif entity_name.startswith("Default_"):
                    impl_entity = entity_name[8:]
                    emoji = get_emoji_for(impl_entity)
                    new_line = f"{bm.group(1)}<summary>{emoji}Default implementation for {impl_entity}.</summary>"
                    new_lines.append(new_line + "\n")
                    fix_count += 1
                    continue
                else:
                    desc = get_description(entity_name)
                    emoji = get_emoji_for(entity_name)
                    if desc:
                        new_line = f"{bm.group(1)}<summary>{emoji}{desc}.</summary>"
                    else:
                        # For functions, generate a description
                        readable = entity_name.replace("_", " ")
                        if entity_name.startswith("find_"):
                            new_line = (
                                f"{bm.group(1)}<summary>\U0001f50d{readable}.</summary>"
                            )
                        elif entity_name.startswith("apply_"):
                            target = entity_name.replace("apply_", "").replace(
                                "_diff", ""
                            )
                            new_line = f"{bm.group(1)}<summary>\U0001f504applies a diff to update {target}.</summary>"
                        elif entity_name.startswith("check_"):
                            new_line = (
                                f"{bm.group(1)}<summary>\u2705{readable}.</summary>"
                            )
                        elif entity_name.startswith(
                            "serialize"
                        ) or entity_name.startswith("deserialize"):
                            new_line = (
                                f"{bm.group(1)}<summary>\U0001f4be{readable}.</summary>"
                            )
                        elif entity_name.startswith("are_"):
                            target = entity_name.replace("are_", "").replace(
                                "_equal", ""
                            )
                            new_line = f"{bm.group(1)}<summary>\U0001f504compares two {target} entities for deep equality.</summary>"
                        elif entity_name.startswith("import_"):
                            new_line = (
                                f"{bm.group(1)}<summary>\U0001f4e5{readable}.</summary>"
                            )
                        elif entity_name.startswith("export_"):
                            new_line = (
                                f"{bm.group(1)}<summary>\U0001f4e4{readable}.</summary>"
                            )
                        elif entity_name.startswith("edit_"):
                            new_line = f"{bm.group(1)}<summary>\u270f\ufe0f{readable}.</summary>"
                        elif entity_name.startswith("compute_"):
                            new_line = (
                                f"{bm.group(1)}<summary>\U0001f4bb{readable}.</summary>"
                            )
                        elif entity_name.startswith("make_"):
                            new_line = (
                                f"{bm.group(1)}<summary>\U0001f527{readable}.</summary>"
                            )
                        elif entity_name.startswith("get_"):
                            new_line = (
                                f"{bm.group(1)}<summary>\U0001f50d{readable}.</summary>"
                            )
                        elif entity_name == "Result":
                            new_line = f"{bm.group(1)}<summary>\u2705Result represents a success or ComposeError outcome.</summary>"
                        else:
                            new_line = (
                                f"{bm.group(1)}<summary>{emoji}{readable}.</summary>"
                            )
                    new_lines.append(new_line + "\n")
                    fix_count += 1
                    continue

    # ================================================================
    # PATTERN B: doc lines with emojis before <remarks>
    # e.g. "/// 🔻️<remarks>"
    # ================================================================
    remarks_m = re.match(r"^(\s*///\s+)\S.*?(<remarks>)", stripped)
    if remarks_m and "<summary>" not in stripped:
        prefix = remarks_m.group(1)
        new_lines.append(f"{prefix}<remarks>\n")
        fix_count += 1
        continue

    # ================================================================
    # PATTERN C: Section markers with wrong emojis
    # Opening: "// EMOJIModel Types - Entity"
    # Closing: "} // EMOJIModel Types - Entity"
    # ================================================================
    section_m = re.match(r"^(\s*)(}?\s*//\s*)\S+?(Model Types - )(\w+)(.*?)$", stripped)
    if section_m and "///" not in stripped:
        indent = section_m.group(1)
        prefix_part = section_m.group(2)  # "} // " or "// "
        model_types = section_m.group(3)  # "Model Types - "
        entity = section_m.group(4)
        rest = section_m.group(5)
        if entity in ENTITY_EMOJI:
            emoji = ENTITY_EMOJI[entity]
            new_line = f"{indent}{prefix_part}{emoji}{model_types}{entity}{rest}"
            new_lines.append(new_line + "\n")
            fix_count += 1
            continue

    # ================================================================
    # PATTERN D: Meta type doc comments
    # "/// 🔖️StatMeta is identical to Stat"
    # "/// 🔖️PropMeta is Prop without attributes."
    # ================================================================
    meta_m = re.match(r"^(\s*///\s+)\S+?((\w+)Meta\s+is\s+.+)$", stripped)
    if meta_m:
        prefix = meta_m.group(1)
        content = meta_m.group(2)
        meta_name = meta_m.group(3) + "Meta"
        if meta_name in META_DESC:
            emoji = get_emoji_for(meta_name)
            new_lines.append(
                f"{prefix}{emoji}{meta_name} represents {META_DESC[meta_name]}.\n"
            )
            fix_count += 1
            continue

    # ================================================================
    # PATTERN E: Random emoji before "All valid KitKind values."
    # ================================================================
    if "All valid KitKind" in stripped:
        m = re.match(r"^(\s*///\s+)\S+(All valid.*)", stripped)
        if m:
            new_lines.append(f"{m.group(1)}\U0001f4e6{m.group(2)}\n")
            fix_count += 1
            continue

    # ================================================================
    # PATTERN F: Various narrative doc comments with wrong leading emojis
    # "/// 📚️<summary>🔖️Converts a nalgebra..."
    # Already partially handled, but check for remaining
    # ================================================================
    narrative_m = re.match(
        r"^(\s*///\s+)\S+?<summary>\S+?((?:Converts|Preserves|Assigns|Selects)\s.+)</summary>",
        stripped,
    )
    if narrative_m:
        prefix = narrative_m.group(1)
        desc = narrative_m.group(2)
        new_lines.append(f"{prefix}<summary>\U0001f527{desc}</summary>\n")
        fix_count += 1
        continue

    # No fix needed
    new_lines.append(line)

with open(file_path, "w", encoding="utf-8") as f:
    f.writelines(new_lines)

print(f"Fixed {fix_count} additional doc comment issues in {file_path}")
