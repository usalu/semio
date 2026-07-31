import re
import sys

SECTION_SUMMARIES = {
    "Constants": "Shared constants, enums, and configuration values used across compose.",
    "Utility": "General-purpose utility functions for encoding, serialization, and conversion.",
    "Expressions": "Expression tree types for building and evaluating symbolic computations.",
    "Entitying": "Base entity classes with equality, validation, and diff support.",
    "ComposeValidation": "Validation result types and kit-level validation logic.",
    "Attribute": "Key-value metadata attributes for annotating entities.",
    "Coord": "Coordinate system types shared by Point, Vector, and Plane.",
    "Point": "3D point representation with X, Y, Z coordinates.",
    "Vector": "3D vector representation with X, Y, Z components.",
    "Plane": "3D plane defined by origin point and X/Y direction vectors.",
    "Location": "Spatial location combining a plane with rotation and elevation.",
    "Author": "Author identity with name and contact information.",
    "File": "File reference with URI, MIME type, and optional content.",
    "Folder": "Folder reference with name and optional parent document.",
    "Benchmark": "Benchmark metadata for performance measurement.",
    "QualityKind": "Quality categorization kinds for validation rules.",
    "Quality": "Quality metric combining a kind, name, value, and unit.",
    "Tag": "Lightweight label for categorizing entities.",
    "Concept": "Semantic concept linking a name to a description and icon.",
    "Port": "Connection port defining a typed interface on a type.",
    "Prop": "Property definition binding a name to an expression value.",
    "Model": "3D model reference with URI, MIME type, and local plane.",
    "Connector": "Connector defining a located interface point on a type.",
    "Type": "Parametric type with ports, connectors, and models.",
    "Layer": "Named layer for organizing pieces in a design.",
    "Group": "Named grouping of pieces within a design.",
    "Piece": "Instantiated type placed within a design document.",
    "Side": "Endpoint of a connection referencing a piece and connector.",
    "Connection": "Link between two sides connecting pieces in a design.",
    "Stat": "Statistical metric associated with a design.",
    "Design": "Spatial layout composing pieces, connections, and metadata.",
    "Kit": "Collection of types and designs forming a reusable library.",
    "Design Family Helpers": "Helpers for traversing design parent-child hierarchies.",
    "Type Family Helpers": "Helpers for traversing type parent-child hierarchies.",
    "Api": "REST API client for communicating with the compose engine.",
    "ZipRoundtrip": "Import and export of kits as ZIP archives.",
    "KitImporter": "High-level kit import from ZIP files.",
    "KitExporter": "High-level kit export to ZIP files.",
    "ComposeDiff": "Diff computation, application, and comparison for kits.",
    "Imports": "External namespace imports required by the library.",
    "Namespace": "Root namespace declaration for the Compose library.",
    "Converters": "Grasshopper data type converters between compose and GH types.",
    "Bases": "Abstract base classes for Grasshopper Goo, Param, and Component types.",
    "Scripting": "Grasshopper scripting helpers for C# script components.",
    "Engine": "Engine communication components for kit and design operations.",
    "Persistence": "Grasshopper document persistence for saving and loading kits.",
}

DEF_SUMMARIES = {
    "Symbol": "Abstract base for all expression tree nodes.",
    "Term": "Abstract base for expression terms that can be evaluated.",
    "Constant": "Abstract base for constant value terms.",
    "Entity": "Abstract generic base class providing equality, hashing, cloning, and validation.",
    "EntityValidator": "FluentValidation validator base for Entity subclasses.",
    "Goo": "Generic Grasshopper data wrapper for compose entity types.",
    "Param": "Generic Grasshopper parameter for compose entity types.",
    "EnumGoo": "Generic Grasshopper data wrapper for enum values.",
    "EnumParam": "Generic Grasshopper parameter for enum values.",
    "PassthroughComponent": "Abstract Grasshopper component that passes input through transformation.",
    "IdGoo": "Generic Grasshopper data wrapper for entity ID types.",
    "IdParam": "Generic Grasshopper parameter for entity ID types.",
    "IdComponent": "Abstract Grasshopper component for constructing entity IDs.",
    "DiffGoo": "Generic Grasshopper data wrapper for entity diff types.",
    "DiffParam": "Generic Grasshopper parameter for entity diff types.",
    "DiffComponent": "Abstract Grasshopper component for constructing entity diffs.",
    "SerializeComponent": "Abstract Grasshopper component for serializing entities to JSON.",
    "DeserializeComponent": "Abstract Grasshopper component for deserializing entities from JSON.",
    "SerializeDiffComponent": "Abstract Grasshopper component for serializing diffs to JSON.",
    "DeserializeDiffComponent": "Abstract Grasshopper component for deserializing diffs from JSON.",
    "SerializeIdComponent": "Abstract Grasshopper component for serializing entity IDs to JSON.",
    "DeserializeIdComponent": "Abstract Grasshopper component for deserializing entity IDs from JSON.",
    "EntityGoo": "Generic Grasshopper data wrapper with built-in entity validation.",
    "EntityParam": "Generic Grasshopper parameter with entity validation support.",
    "EntityComponent": "Abstract Grasshopper component for constructing validated entities.",
    "EntityIdGoo": "Generic Grasshopper data wrapper for validated entity ID types.",
    "EntityIdParam": "Generic Grasshopper parameter for validated entity ID types.",
    "EntityIdComponent": "Abstract Grasshopper component for constructing validated entity IDs.",
    "EntityDiffGoo": "Generic Grasshopper data wrapper for validated entity diff types.",
    "EntityDiffParam": "Generic Grasshopper parameter for validated entity diff types.",
    "EntityDiffComponent": "Abstract Grasshopper component for constructing validated entity diffs.",
}

DEF_SPECS = {
    "Symbol": "Implementations MUST be immutable value types within expression trees.",
    "Term": "Implementations MUST support evaluation via the Evaluate method.",
    "Constant": "Implementations MUST provide a fixed value independent of context.",
    "Entity": "Implementations MUST override equality based on serialized representation.",
    "EntityValidator": "Implementations MUST define validation rules in the constructor.",
    "Goo": "Implementations MUST override CastFrom and CastTo for type conversion.",
    "Param": "Implementations MUST provide component exposure and icon metadata.",
    "EnumGoo": "Implementations MUST convert between string names and enum values.",
    "EnumParam": "Implementations MUST restrict input to valid enum members.",
    "PassthroughComponent": "Implementations MUST transform input data and output the result.",
    "IdGoo": "Implementations MUST wrap entity ID types for Grasshopper data flow.",
    "IdParam": "Implementations MUST provide type-safe parameter access for IDs.",
    "IdComponent": "Implementations MUST register input parameters matching ID fields.",
    "DiffGoo": "Implementations MUST wrap entity diff types for Grasshopper data flow.",
    "DiffParam": "Implementations MUST provide type-safe parameter access for diffs.",
    "DiffComponent": "Implementations MUST register input parameters matching diff fields.",
    "SerializeComponent": "Implementations MUST convert entities to valid JSON strings.",
    "DeserializeComponent": "Implementations MUST parse JSON strings into entity instances.",
    "SerializeDiffComponent": "Implementations MUST convert diffs to valid JSON strings.",
    "DeserializeDiffComponent": "Implementations MUST parse JSON strings into diff instances.",
    "SerializeIdComponent": "Implementations MUST convert entity IDs to valid JSON strings.",
    "DeserializeIdComponent": "Implementations MUST parse JSON strings into entity ID instances.",
    "EntityGoo": "Implementations MUST validate entities before exposing them downstream.",
    "EntityParam": "Implementations MUST enforce entity validation on parameter access.",
    "EntityComponent": "Implementations MUST validate constructed entities before output.",
    "EntityIdGoo": "Implementations MUST validate entity IDs before exposing them downstream.",
    "EntityIdParam": "Implementations MUST enforce entity ID validation on parameter access.",
    "EntityIdComponent": "Implementations MUST validate constructed entity IDs before output.",
    "EntityDiffGoo": "Implementations MUST validate entity diffs before exposing them downstream.",
    "EntityDiffParam": "Implementations MUST enforce entity diff validation on parameter access.",
    "EntityDiffComponent": "Implementations MUST validate constructed entity diffs before output.",
}


def process_file(filepath):
    with open(filepath, "r") as f:
        lines = f.readlines()

    insertions = []

    for i, line in enumerate(lines):
        stripped = line.strip()
        lineno = i + 1

        # Check for section region
        m = re.match(r"\s*#region 🔖️(.+)", stripped)
        if m:
            section_name = m.group(1).strip()
            # Check if next non-blank line is already a comment
            j = i + 1
            while j < len(lines) and lines[j].strip() == "":
                j += 1
            if j < len(lines) and lines[j].strip().startswith("//"):
                continue  # already has a comment
            # Check if next non-blank line is #endregion (empty section)
            if j < len(lines) and lines[j].strip().startswith("#endregion"):
                continue  # empty section, skip
            summary = SECTION_SUMMARIES.get(section_name)
            if summary:
                indent = line[: len(line) - len(line.lstrip())]
                insertions.append((i + 1, f"{indent}// {summary}\n"))

    # Sort insertions in reverse order so line numbers don't shift
    insertions.sort(key=lambda x: x[0], reverse=True)
    for pos, text in insertions:
        lines.insert(pos, text)

    with open(filepath, "w") as f:
        f.writelines(lines)
    print(f"Inserted {len(insertions)} section summaries in {filepath}")


def process_orphans(filepath):
    """Wrap orphan code blocks in sections."""
    with open(filepath, "r") as f:
        content = f.read()

    if filepath.endswith("Compose.cs"):
        # Wrap imports (line 27-50) in Imports section and namespace (line 51) in Namespace section
        content = content.replace(
            "#endregion 🔖️Header\n\nusing",
            "#endregion 🔖️Header\n\n#region 🔖️Imports\n// External namespace imports required by the library.\nusing",
            1,
        )
        # Find where namespace starts
        content = content.replace(
            "\nnamespace Compose;\n\n#region 🔖️Constants",
            "\n#endregion 🔖️Imports\n\n#region 🔖️Namespace\n// Root namespace declaration for the Compose library.\nnamespace Compose;\n#endregion 🔖️Namespace\n\n#region 🔖️Constants",
            1,
        )
    elif filepath.endswith("Grasshopper.cs"):
        content = content.replace(
            "#endregion 🔖️Header\n\nusing",
            "#endregion 🔖️Header\n\n#region 🔖️Imports\n// External namespace imports required by the Grasshopper plugin.\nusing",
            1,
        )
        content = content.replace(
            "\nnamespace Compose.Grasshopper;\n\n#region 🔖️Constants",
            "\n#endregion 🔖️Imports\n\n#region 🔖️Namespace\n// Root namespace declaration for the Grasshopper plugin.\nnamespace Compose.Grasshopper;\n#endregion 🔖️Namespace\n\n#region 🔖️Constants",
            1,
        )

    with open(filepath, "w") as f:
        f.write(content)
    print(f"Fixed orphan definitions in {filepath}")


def process_definitions(filepath):
    """Add summary and spec comments before definitions missing them."""
    with open(filepath, "r") as f:
        lines = f.readlines()

    # Get breachs from analyze
    import json
    import subprocess

    result = subprocess.run(
        ["./repo/cli/cli", "analyze", filepath, "--json"],
        capture_output=True,
        text=True,
    )
    data = json.loads(result.stdout)
    breachs = data["analyze"]["breachs"]

    # Get definitions needing summaries
    missing_summary = {}
    missing_requirements = {}
    for v in breachs:
        if v["kind"]["id"] == "🚫️Code#Definition#Missing Summary":
            missing_summary[v["line"]] = (
                v["excerpt"] if v.get("excerpt") else v["summary"].split('"')[1]
            )
        if v["kind"]["id"] == "🚫️Code#Definition#Missing Requirements":
            missing_requirements[v["line"]] = (
                v["excerpt"] if v.get("excerpt") else v["summary"].split('"')[1]
            )

    if not missing_summary and not missing_requirements:
        print(f"No definition breachs found in {filepath}")
        return

    insertions = []
    for lineno in sorted(
        set(list(missing_summary.keys()) + list(missing_requirements.keys()))
    ):
        idx = lineno - 1
        if idx >= len(lines):
            continue
        line = lines[idx]
        indent = line[: len(line) - len(line.lstrip())]

        # Extract the class/struct name from the line
        m = re.search(
            r"(?:public|internal)\s+(?:abstract\s+|static\s+|sealed\s+)*(?:class|struct|interface|enum|record)\s+(\w+)",
            line,
        )
        if m:
            name = m.group(1)
        else:
            name = missing_summary.get(lineno, missing_requirements.get(lineno, ""))

        is_class = bool(re.search(r"(?:class|struct|record)\s+", line))
        is_interface = bool(re.search(r"interface\s+", line))
        is_enum = bool(re.search(r"enum\s+", line))

        texts = []
        if lineno in missing_summary:
            summary = DEF_SUMMARIES.get(name, f"{name} definition.")
            texts.append(f"{indent}// {summary}")

        if (
            lineno in missing_requirements
            and is_class
            and not is_interface
            and not is_enum
        ):
            spec = DEF_SPECS.get(
                name, f"Implementations MUST conform to the {name} contract."
            )
            texts.append(f"{indent}// {spec}")

        if texts:
            insert_text = "\n".join(texts) + "\n"
            insertions.append((idx, insert_text))

    # Sort insertions in reverse order so line numbers don't shift
    insertions.sort(key=lambda x: x[0], reverse=True)
    for pos, text in insertions:
        lines.insert(pos, text)

    with open(filepath, "w") as f:
        f.writelines(lines)
    print(f"Inserted {len(insertions)} definition comments in {filepath}")


if __name__ == "__main__":
    filepath = sys.argv[1]
    process_orphans(filepath)
    process_file(filepath)
    process_definitions(filepath)
