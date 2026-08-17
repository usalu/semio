"""Fix remaining broken doc comments in compose/rs/lib.rs.
Specifically targets lines with 🔧️ (wrench) that were incorrectly generated
by the first fix script, and fixes 'represents identifies' → 'identifies' for Id types.
"""

import re

file_path = r"c:\git\compose\compose\rs\lib.rs"

with open(file_path, "r", encoding="utf-8") as f:
    content = f.read()

fix_count = 0

# ================================================================
# FIX 1: "represents identifies" → just "identifies" for Id types
# ================================================================
old = " represents identifies "
new = " "
occurrences = content.count(old)
content = content.replace(old, new)
fix_count += occurrences
print(f"Fixed {occurrences} 'represents identifies' → 'identifies'")

# ================================================================
# FIX 2: Read all lines and fix broken 🔧️ patterns using context
# ================================================================
lines = content.split("\n")
new_lines = []

ENTITY_EMOJI = {
    "Attribute": "💎️",
    "AttributeId": "💎️",
    "Location": "📍️",
    "LocationId": "📍️",
    "Author": "✍️",
    "AuthorId": "✍️",
    "File": "📄️",
    "FileId": "📄️",
    "Folder": "📁️",
    "FolderId": "📁️",
    "Benchmark": "📏️",
    "BenchmarkId": "📏️",
    "Quality": "🔬️",
    "QualityId": "🔬️",
    "QualityKind": "🔬️",
    "Port": "⚓️",
    "PortId": "⚓️",
    "Prop": "📊️",
    "PropId": "📊️",
    "Tag": "🏷️",
    "TagId": "🏷️",
    "Concept": "💡️",
    "ConceptId": "💡️",
    "Model": "🗿️",
    "ModelId": "🗿️",
    "Connector": "🔌️",
    "ConnectorId": "🔌️",
    "Type": "🧱️",
    "TypeId": "🧱️",
    "Layer": "🎨️",
    "LayerId": "🎨️",
    "Piece": "🧩️",
    "PieceId": "🧩️",
    "Group": "👥️",
    "GroupId": "👥️",
    "Side": "↔",
    "Connection": "🔗️",
    "ConnectionId": "🔗️",
    "Stat": "📈️",
    "StatId": "📈️",
    "Design": "📐️",
    "DesignId": "📐️",
    "Kit": "📦️",
    "Coord": "📺️",
    "Vec": "➡️",
    "Point": "✖️",
    "Vector": "↗️",
    "Plane": "◻",
    "Camera": "🎥️",
}

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


def get_emoji(name):
    if name in ENTITY_EMOJI:
        return ENTITY_EMOJI[name]
    for e in ENTITY_EMOJI:
        if name.startswith(e):
            return ENTITY_EMOJI[e]
    return "🔧️"


def find_next_definition(lines, start_idx, max_look=10):
    """Look at the next few lines to find a struct/fn/type/enum/impl definition."""
    for j in range(start_idx + 1, min(start_idx + max_look, len(lines))):
        l = lines[j].strip()
        # pub struct Name / pub enum Name / pub type Name
        m = re.match(r"pub\s+(?:struct|enum|type)\s+(\w+)", l)
        if m:
            return m.group(1), "struct"
        # pub fn name
        m = re.match(r"pub\s+fn\s+(\w+)", l)
        if m:
            return m.group(1), "fn"
        # impl Trait for Name
        m = re.match(r"impl\s+(\w+)\s+for\s+(\w+)", l)
        if m:
            return f"{m.group(1)}_{m.group(2)}", "impl_trait"
        # impl Name {
        m = re.match(r"impl\s+(\w+)\s*\{", l)
        if m:
            return m.group(1), "impl"
        # pub const NAME
        m = re.match(r"pub\s+const\s+(\w+)", l)
        if m:
            return m.group(1), "const"
    return None, None


for i, line in enumerate(lines):
    stripped = line.rstrip()

    # Only process lines with 🔧️ in a <summary> tag
    if "🔧️" not in stripped or "<summary>" not in stripped:
        new_lines.append(line)
        continue

    # Extract the indent
    indent_match = re.match(r"^(\s*)", stripped)
    indent = indent_match.group(1) if indent_match else ""

    # Get the broken content after 🔧️
    m = re.match(r".*<summary>🔧️(.*?)</summary>", stripped)
    if not m:
        new_lines.append(line)
        continue

    broken_content = m.group(1).rstrip(".")

    # Try to find what this line documents by looking ahead
    next_def, def_kind = find_next_definition(lines, i)

    # Build the correct doc comment
    # Strategy: use the definition name to look up the correct emoji and description

    new_line = None

    if next_def:
        # Handle impl_trait (e.g., HasGuid_Attribute or DiffHasGuid_TypeDiff)
        if def_kind == "impl_trait":
            parts = next_def.split("_", 1)
            trait_name = parts[0]
            entity_name = parts[1] if len(parts) > 1 else ""

            if trait_name == "HasGuid":
                emoji = get_emoji(entity_name)
                new_line = f"{indent}/// <summary>{emoji}HasGuid implementation for {entity_name}.</summary>"
            elif trait_name == "DiffHasGuid":
                base = entity_name.replace("Diff", "")
                emoji = get_emoji(base)
                new_line = f"{indent}/// <summary>{emoji}DiffHasGuid implementation for {entity_name}.</summary>"
            elif trait_name == "Default":
                emoji = get_emoji(entity_name)
                new_line = f"{indent}/// <summary>{emoji}Default implementation for {entity_name}.</summary>"
            else:
                emoji = get_emoji(entity_name)
                new_line = f"{indent}/// <summary>{emoji}{trait_name} implementation for {entity_name}.</summary>"

        # Handle struct/enum/type
        elif def_kind == "struct":
            name = next_def
            if name in ENTITY_DESC:
                emoji = get_emoji(name)
                new_line = f"{indent}/// <summary>{emoji}{name} represents {ENTITY_DESC[name]}.</summary>"
            elif name.endswith("Diff"):
                base = name[:-4]
                emoji = get_emoji(base)
                new_line = f"{indent}/// <summary>{emoji}{name} represents a partial update to {base.lower()}'s fields.</summary>"
            elif name.endswith("Change"):
                base = name[:-6]
                emoji = get_emoji(base)
                if base == "Kit":
                    new_line = f"{indent}/// <summary>{emoji}{name} tracks kit-level modifications.</summary>"
                else:
                    new_line = f"{indent}/// <summary>{emoji}{name} tracks {base.lower()} modifications in a kit change.</summary>"
            elif name.endswith("Meta"):
                base = name[:-4]
                emoji = get_emoji(base)
                new_line = f"{indent}/// <summary>{emoji}{name} represents scalar-only view of {base.lower()} excluding nested arrays.</summary>"
            elif name == "RemovedItem":
                new_line = f"{indent}/// <summary>🗑️RemovedItem represents an entity marked for removal by GUID.</summary>"
            elif name == "DiffUpdate":
                new_line = f"{indent}/// <summary>🔄️DiffUpdate represents a before-after pair for entity updates.</summary>"
            elif name == "Change":
                new_line = f"{indent}/// <summary>🔄️Change represents a tracked modification with timestamp and author.</summary>"
            elif name == "CollectionDiff":
                new_line = f"{indent}/// <summary>🔄️CollectionDiff represents batched entity additions, removals and updates.</summary>"
            elif name == "ValidationProblem":
                new_line = f"{indent}/// <summary>⚠️ValidationProblem represents a validation issue with severity and location.</summary>"
            elif name == "ValidationFix":
                new_line = f"{indent}/// <summary>🔧️ValidationFix represents a suggested fix for a validation problem.</summary>"
            elif name == "ValidationResult":
                new_line = f"{indent}/// <summary>✅️ValidationResult represents the outcome of a kit validation.</summary>"
            elif name == "ComposeError":
                new_line = f"{indent}/// <summary>❌️ComposeError represents a domain error with context message.</summary>"
            elif name == "FlattenedPiece":
                new_line = f"{indent}/// <summary>🧩️FlattenedPiece represents a piece with fully resolved world-space transform.</summary>"
            elif name == "WasmResult":
                new_line = f"{indent}/// <summary>🌐️WasmResult represents a WebAssembly-compatible result wrapper.</summary>"
            else:
                new_line = f"{indent}/// <summary>🔧️{name}.</summary>"

        # Handle fn
        elif def_kind == "fn":
            name = next_def
            readable = name.replace("_", " ")

            # Finder functions
            if name.startswith("find_"):
                entity = None
                fm = re.match(r"find_(\w+?)_in_(\w+?)(_mut)?$", name)
                if fm:
                    entity = fm.group(1)
                    container = fm.group(2)
                    is_mut = fm.group(3)
                    emoji = get_emoji(entity.capitalize())
                    mut_str = " mutably" if is_mut else ""
                    new_line = f"{indent}/// <summary>🔍️finds a {entity} in a {container}{mut_str} by GUID.</summary>"
                else:
                    new_line = f"{indent}/// <summary>🔍️{readable}.</summary>"

            # Serialization
            elif name.startswith("serialize_") or name.startswith("deserialize_"):
                new_line = f"{indent}/// <summary>💾️{readable}.</summary>"

            # Apply diff
            elif name.startswith("apply_") and name.endswith("_diff"):
                target = name.replace("apply_", "").replace("_diff", "")
                emoji = get_emoji(target.capitalize())
                new_line = f"{indent}/// <summary>🔄️applies a diff to update {target}.</summary>"

            # Check/validation
            elif name.startswith("check_"):
                new_line = f"{indent}/// <summary>✅️{readable}.</summary>"

            # Equality
            elif name.startswith("are_") and name.endswith("_equal"):
                target = name.replace("are_", "").replace("_equal", "")
                new_line = f"{indent}/// <summary>🔄️compares two {target} entities for deep equality.</summary>"

            # Import/export
            elif name.startswith("import_"):
                new_line = f"{indent}/// <summary>📥️{readable}.</summary>"
            elif name.startswith("export_"):
                new_line = f"{indent}/// <summary>📤️{readable}.</summary>"

            # Edit functions
            elif name.startswith("edit_"):
                new_line = f"{indent}/// <summary>✏️{readable}.</summary>"

            # Compute
            elif name.startswith("compute_"):
                new_line = f"{indent}/// <summary>💻️{readable}.</summary>"

            # Get
            elif name.startswith("get_"):
                new_line = f"{indent}/// <summary>🔍️{readable}.</summary>"

            # Make
            elif name.startswith("make_"):
                new_line = f"{indent}/// <summary>🔧️{readable}.</summary>"

            # Validate
            elif name == "validate_kit":
                new_line = f"{indent}/// <summary>✅️validates a kit for structural and referential integrity.</summary>"

            # Flatten
            elif name == "flatten_design":
                new_line = f"{indent}/// <summary>📐️flattens nested design references into a single design.</summary>"

            # Planes equal
            elif name == "planes_equal_approx":
                new_line = f"{indent}/// <summary>◻compares two planes for approximate equality.</summary>"

            # Quat
            elif name == "quat_to_matrix4":
                new_line = f"{indent}/// <summary>🔄️converts quaternion to a 4x4 rotation matrix.</summary>"

            # Apply matrix
            elif name.startswith("apply_matrix"):
                new_line = f"{indent}/// <summary>💻️applies a 4x4 matrix to a 3D vector.</summary>"

            # Connector to plane
            elif name == "connector_to_plane":
                new_line = f"{indent}/// <summary>🔌️converts a connector to a plane representation.</summary>"

            # Mime
            elif name.startswith("mime_") or "mime" in name:
                new_line = f"{indent}/// <summary>📄️{readable}.</summary>"

            # Default
            else:
                new_line = f"{indent}/// <summary>🔧️{readable}.</summary>"

        # Handle const
        elif def_kind == "const":
            name = next_def
            if name == "SUPPORTED_MODEL_EXTENSIONS":
                new_line = f"{indent}/// <summary>🗿️the list of supported 3D model file extensions.</summary>"
            else:
                new_line = f"{indent}/// <summary>🔧️{name}.</summary>"

        # Handle impl
        elif def_kind == "impl":
            name = next_def
            if name in ENTITY_DESC:
                emoji = get_emoji(name)
                new_line = f"{indent}/// <summary>{emoji}{name} represents {ENTITY_DESC[name]}.</summary>"
            else:
                emoji = get_emoji(name)
                new_line = (
                    f"{indent}/// <summary>{emoji}{name} implementation.</summary>"
                )

    # Fallback: handle narrative docs (Converts, Preserves, Assigns, Selects)
    if new_line is None:
        if (
            "Converts" in broken_content
            or "Preserves" in broken_content
            or "Assigns" in broken_content
            or "Selects" in broken_content
        ):
            # Keep the narrative but with proper emoji
            new_line = f"{indent}/// <summary>🔧️{broken_content}.</summary>"
        else:
            # Can't determine - keep broken with a comment
            new_lines.append(line)
            continue

    new_lines.append(new_line + "\n")
    fix_count += 1

result = "\n".join(l.rstrip("\n") for l in new_lines)
with open(file_path, "w", encoding="utf-8") as f:
    f.write(result)

print(f"Fixed {fix_count} total remaining issues")
