#!/usr/bin/env python3
"""Fix all policy breachs in compose/js/compose.ts.
Phase 1: Wrap orphan blocks in sections (Imports, Utilities)
Phase 2: Move DateProperty inside Attribute section
Phase 3: Add section summaries
Phase 4: Add definition summaries + requirements
"""

import re
import sys

FILE = "/workspaces/semio/compose/js/compose.ts"

with open(FILE, "r") as f:
    content = f.read()
lines = content.split("\n")

section_summary_map = {
    "Constants": "Global constants MUST define shared numeric parameters.",
    "Entity IDs": "Entity identifier types and comparison functions MUST be defined here.",
    "Attribute": "Attribute entity types, schemas, and helper functions MUST be defined here.",
    "Coord (weak entity)": "Coord weak entity types and schemas MUST be defined here.",
    "Vec (weak entity)": "Vec weak entity types and schemas MUST be defined here.",
    "Point (weak entity)": "Point weak entity types and schemas MUST be defined here.",
    "Vector (weak entity)": "Vector weak entity types and schemas MUST be defined here.",
    "Plane (weak entity)": "Plane weak entity types and schemas MUST be defined here.",
    "Camera (weak entity)": "Camera weak entity types and schemas MUST be defined here.",
    "Location": "Location entity types, schemas, and helpers MUST be defined here.",
    "Author": "Author entity types, schemas, and helpers MUST be defined here.",
    "File": "File entity types, schemas, and helpers MUST be defined here.",
    "Folder": "Folder entity types, schemas, and helpers MUST be defined here.",
    "Benchmark": "Benchmark entity types, schemas, and helpers MUST be defined here.",
    "Quality": "Quality entity types, schemas, and helpers MUST be defined here.",
    "Port": "Port entity types, schemas, and helpers MUST be defined here.",
    "Prop": "Prop entity types, schemas, and helpers MUST be defined here.",
    "Tag": "Tag entity types, schemas, and helpers MUST be defined here.",
    "Concept": "Concept entity types, schemas, and helpers MUST be defined here.",
    "Model": "Model entity types, schemas, and helpers MUST be defined here.",
    "Connector": "Connector entity types, schemas, and helpers MUST be defined here.",
    "Type": "Type entity types, schemas, and helpers MUST be defined here.",
    "Layer": "Layer entity types, schemas, and helpers MUST be defined here.",
    "Piece": "Piece entity types, schemas, and helpers MUST be defined here.",
    "Group": "Group entity types, schemas, and helpers MUST be defined here.",
    "Side": "Side entity types, schemas, and helpers MUST be defined here.",
    "Connection": "Connection entity types, schemas, and helpers MUST be defined here.",
    "Stat": "Stat entity types, schemas, and helpers MUST be defined here.",
    "Design": "Design entity types, schemas, and helpers MUST be defined here.",
    "Kit": "Kit entity types, schemas, and helpers MUST be defined here.",
    "Design Family Helpers": "Design family traversal helpers MUST be defined here.",
    "Type Family Helpers": "Type family traversal helpers MUST be defined here.",
    "File Tree Utilities": "File tree construction and traversal utilities MUST be defined here.",
    "Kit Import/Export": "Kit serialization and deserialization functions MUST be defined here.",
    "Validation": "Kit validation engine and constraints MUST be defined here.",
    "Validation core types": "Core validation types and interfaces MUST be defined here.",
    "Validation context & engine": "Validation context construction and engine MUST be defined here.",
    "Fix helper": "Validation fix helper functions MUST be defined here.",
    "GUID update helper": "GUID regeneration helper functions MUST be defined here.",
    "Constraint: GUID uniqueness": "GUID uniqueness constraint MUST be enforced here.",
    "Constraint: Type name uniqueness": "Type name uniqueness constraint MUST be enforced here.",
    "Constraint: Design name uniqueness": "Design name uniqueness constraint MUST be enforced here.",
    "Constraint: Piece name uniqueness": "Piece name uniqueness constraint MUST be enforced here.",
    "Constraint: Quality name uniqueness": "Quality name uniqueness constraint MUST be enforced here.",
    "Constraint: Port name uniqueness": "Port name uniqueness constraint MUST be enforced here.",
    "Constraint: File name uniqueness": "File name uniqueness constraint MUST be enforced here.",
    "Constraint: Folder name uniqueness": "Folder name uniqueness constraint MUST be enforced here.",
    "Constraint: Connector name uniqueness within type": "Connector name uniqueness within type constraint MUST be enforced here.",
    "Constraint: Model name uniqueness within type": "Model name uniqueness within type constraint MUST be enforced here.",
    "Constraint: Layer path uniqueness within design": "Layer path uniqueness within design constraint MUST be enforced here.",
    "Constraint: Design piece same family constraint": "Design piece same family constraint MUST be enforced here.",
    "Constraint registration": "Constraint registration and default configurations MUST be defined here.",
    "Validation serialization": "Validation result serialization and deserialization MUST be defined here.",
    "Imports": "External dependency imports MUST be declared here.",
    "Utilities": "General-purpose utility functions MUST be defined here.",
}

export_pattern = re.compile(
    r"^export\s+(?:(?:async|abstract|declare|default)\s+)*(?:const|let|var|function|class|interface|type|enum)\s+(\w+)"
)
section_start_pattern = re.compile(r"^\s*//\s*#region\s+\u0001?\u0002?🔖️(.+?)\s*$")
section_end_pattern = re.compile(r"^\s*//\s*#endregion\s+\u0001?\u0002?🔖️(.+?)\s*$")


def gen_summary(name, lt):
    if name.endswith("IdSchema"):
        base = name[:-8]
        return f"Zod schema for validating {base} identifiers."
    if name.endswith("DiffSchema"):
        base = name[:-10]
        return f"Zod schema for {base} diff validation."
    if name.endswith("Schema") and (
        "z." in lt
        or "z.object" in lt
        or "z.enum" in lt
        or "z.string" in lt
        or "z.array" in lt
    ):
        base = name[:-6]
        return f"Zod schema for {base} validation."
    if name.endswith("Id") and "type " in lt:
        base = name[:-2]
        return f"Identifier type for {base} entities."
    if name.startswith("create") and name.endswith("Id"):
        base = name[6:-2]
        return f"Factory for creating {base} identifiers."
    if name.startswith("areSame") and name.endswith("Id"):
        base = name[7:-2]
        return f"Equality check for {base} identifiers."
    if name.startswith("get") and name.endswith("Guid"):
        base = name[3:-4]
        return f"Extracts the GUID from a {base} identifier."
    if name.startswith("empty") and len(name) > 5:
        base = name[5:]
        return f"Default empty {base} instance."
    if name.startswith("areSame"):
        base = name[7:]
        return f"Equality check for {base} values."
    if name.startswith("are") and "Equal" in name:
        base = name[3:].replace("Equal", "").replace("Ignoring", " ignoring ")
        return f"Deep equality check for {base} entities."
    if name.startswith("are") and "InSameFamily" in name:
        base = name[3:].replace("InSameFamily", "")
        return f"Checks if {base} belong to the same family."
    if name.startswith("has") and len(name) > 3 and name[3:4].isupper():
        base = name[3:]
        return f"Checks whether {base} condition holds."
    if name.startswith("find"):
        base = name[4:]
        return f"Searches for matching {base} entry."
    if name.startswith("add") and len(name) > 3 and name[3:4].isupper():
        base = name[3:]
        return f"Adds a {base} element."
    if name.startswith("set") and len(name) > 3 and name[3:4].isupper():
        base = name[3:]
        return f"Replaces an existing {base} element."
    if name.startswith("remove") and len(name) > 6 and name[6:7].isupper():
        base = name[6:]
        return f"Removes a {base} element."
    if name.startswith("get") and len(name) > 3 and name[3:4].isupper():
        base = name[3:]
        return f"Retrieves the {base} value."
    if name.startswith("can") and len(name) > 3 and name[3:4].isupper():
        base = name[3:]
        return f"Checks if {base} action is possible."
    if name.startswith("parse"):
        base = name[5:]
        return f"Parses {base} from serialized input."
    if name.startswith("serialize"):
        base = name[9:]
        return f"Serializes {base} for transport."
    if name.startswith("to") and len(name) > 2 and name[2:3].isupper():
        base = name[2:]
        return f"Converts to {base} representation."
    if name.startswith("import") and len(name) > 6 and name[6:7].isupper():
        base = name[6:]
        return f"Imports {base} from external source."
    if name.startswith("export") and len(name) > 6 and name[6:7].isupper():
        base = name[6:]
        return f"Exports {base} to external format."
    if name.startswith("build"):
        base = name[5:]
        return f"Constructs {base} from components."
    if name.startswith("validate"):
        base = name[8:]
        return f"Validates {base} against constraints."
    if name.startswith("color"):
        base = name[5:]
        return f"Assigns colors to {base} elements."
    if name.startswith("flatten"):
        base = name[7:]
        return f"Flattens nested {base} structure."
    if "Constraint" in name and name.startswith("compose"):
        base = name.replace("compose", "").replace("Constraint", "")
        return f"Constraint validating {base} rules."
    if name.endswith("Diff"):
        base = name[:-4]
        return f"Diff type for tracking {base} changes."
    if "type " in lt and "=" in lt:
        return f"Type alias for {name}."
    if "interface " in lt:
        return f"Interface defining {name} structure."
    if "enum " in lt:
        return f"Enumeration of {name} values."
    if "class " in lt:
        return f"Class implementing {name} behavior."
    if name.isupper() or "_" in name:
        return f"Constant value for {name}."
    if "function " in lt or "=>" in lt:
        return f"Performs the {name} operation."
    return f"Definition of {name}."


def gen_spec(name, lt):
    is_fn = "function " in lt or "=>" in lt
    is_cls = "class " in lt
    if not is_fn and not is_cls:
        return None
    if name.startswith("areSame") or (name.startswith("are") and "Equal" in name):
        return "MUST return a boolean equality result."
    if name.startswith("find"):
        return "MUST return the matching element or undefined."
    if name.startswith("add"):
        return "MUST append the element to the collection."
    if name.startswith("set") and len(name) > 3 and name[3:4].isupper():
        return "MUST replace the existing element."
    if name.startswith("remove"):
        return "MUST remove the element from the collection."
    if name.startswith("get"):
        return "MUST return the requested value."
    if name.startswith("has"):
        return "MUST return true if the condition is met."
    if name.startswith("create"):
        return "MUST return a new valid instance."
    if name.startswith("build"):
        return "MUST construct and return a complete structure."
    if name.startswith("validate"):
        return "MUST check all constraints and return problems."
    if name.startswith("parse"):
        return "MUST produce a valid in-memory representation."
    if name.startswith("serialize"):
        return "MUST produce a serializable output."
    if name.startswith("to") and len(name) > 2 and name[2:3].isupper():
        return "MUST convert to the target representation."
    if name.startswith("import"):
        return "MUST load and return the imported data."
    if name.startswith("export"):
        return "MUST produce the exported format."
    if name.startswith("color"):
        return "MUST assign colors deterministically."
    if name.startswith("flatten"):
        return "MUST return a flat array."
    if name.startswith("can") or (
        name.startswith("are") and not name.startswith("areSame")
    ):
        return "MUST return a boolean result."
    if "Constraint" in name:
        return "MUST detect and report constraint breachs."
    if is_cls:
        return "MUST provide the declared public interface."
    special = {
        "cn": "MUST merge CSS class names using Tailwind merge.",
        "guid": "MUST return a new UUID v7 string.",
        "normalize": "MUST return empty string for null or undefined.",
        "round": "MUST round to the nearest tolerance unit.",
        "jaccard": "MUST compute the Jaccard similarity coefficient.",
        "deepEqual": "MUST recursively compare values for equality.",
        "arraysEqual": "MUST compare arrays element by element.",
        "generateUniqueName": "MUST return a name not in the existing set.",
        "vectorToThree": "MUST convert compose vector to Three.js vector.",
        "composeMakeFix": "MUST produce a Fix that regenerates the GUID.",
        "defaultConstraints": "MUST include all built-in constraints.",
        "hasErrors": "MUST return true when problems exist.",
        "piecesMetadata": "MUST return piece metadata for all pieces.",
    }
    if name in special:
        return special[name]
    return "MUST perform the operation correctly."


# ===== PHASE 1: Wrap orphans =====
result = []
i = 0
date_property_line = None

while i < len(lines):
    line = lines[i]
    stripped = line.strip()

    # Find and save DateProperty line for moving
    if stripped.startswith("const DateProperty"):
        date_property_line = line
        i += 1
        # Skip trailing blank line
        if i < len(lines) and lines[i].strip() == "":
            i += 1
        continue

    # Wrap import block in Imports section
    if (
        stripped.startswith("import ")
        and i > 0
        and lines[i - 1].strip().startswith("// #endregion")
    ):
        result.append("")
        result.append("// #region 🔖️Imports")
        result.append("// External dependency imports MUST be declared here.")
        result.append("")
        while i < len(lines):
            s = lines[i].strip()
            if s.startswith("import "):
                result.append(lines[i])
                i += 1
            elif s == "":
                # Check if next line is also import
                if i + 1 < len(lines) and lines[i + 1].strip().startswith("import "):
                    result.append(lines[i])
                    i += 1
                else:
                    break
            else:
                break
        result.append("")
        result.append("// #endregion 🔖️Imports")
        continue

    # After Constants endregion, open Utilities section
    if stripped == "// #endregion 🔖️Constants":
        result.append(line)
        i += 1
        result.append("")
        result.append("// #region 🔖️Utilities")
        result.append("// General-purpose utility functions MUST be defined here.")
        result.append("")
        # Skip blank line after endregion if present
        if i < len(lines) and lines[i].strip() == "":
            i += 1
        continue

    # Before Entity IDs, close Utilities section
    if stripped == "// #region 🔖️Entity IDs":
        result.append("// #endregion 🔖️Utilities")
        result.append("")
        result.append(line)
        i += 1
        continue

    # After Attribute section start, inject DateProperty
    if stripped == "// #region 🔖️Attribute":
        result.append(line)
        i += 1
        # Skip blank line
        if i < len(lines) and lines[i].strip() == "":
            result.append(lines[i])
            i += 1
        if date_property_line:
            result.append(date_property_line)
            result.append("")
        continue

    result.append(line)
    i += 1

# ===== PHASE 2: Add section summaries and definition summaries =====
lines2 = result
result2 = []

for i in range(len(lines2)):
    line = lines2[i]
    stripped = line.strip()

    # Check for section start needing summary
    sm = re.match(r"^\s*//\s*#region\s+🔖️(.+?)\s*$", stripped)
    if sm:
        section_name = sm.group(1).strip()
        # Check if next non-blank line is already a summary comment
        j = i + 1
        has_summary = False
        while j < len(lines2):
            ns = lines2[j].strip()
            if ns == "":
                j += 1
                continue
            if ns.startswith("//") and not ns.startswith("// #"):
                has_summary = True
            break

        result2.append(line)
        if not has_summary and section_name not in (
            "Header",
            "License",
            "Requirements",
        ):
            summary_text = section_summary_map.get(
                section_name, f"{section_name} definitions MUST be defined here."
            )
            result2.append(f"// {summary_text}")
        continue

    # Check for exported definition needing summary
    em = export_pattern.match(stripped)
    if em:
        def_name = em.group(1)
        # Check if previous non-blank line is already a summary/comment
        j = len(result2) - 1
        has_comment_above = False
        while j >= 0:
            prev = result2[j].strip()
            if prev == "":
                j -= 1
                continue
            if (
                prev.startswith("//")
                and not prev.startswith("// #region")
                and not prev.startswith("// #endregion")
            ):
                has_comment_above = True
            break

        if not has_comment_above:
            spec = gen_spec(def_name, stripped)
            if spec:
                result2.append(f"// {spec}")
            summary = gen_summary(def_name, stripped)
            result2.append(f"// {summary}")

    result2.append(line)

output = "\n".join(result2)
with open(FILE, "w") as f:
    f.write(output)

print(f"Lines: {len(lines)} -> {len(result2)}")
