#!/usr/bin/env python3
"""Consolidation script v4: proper multi-line import handling + deduplication."""

import re, os, collections

BASE = "compose/sketchpad/sketchpad"
SKETCHPAD = os.path.join(BASE, "Sketchpad.tsx")

FILES = [
    ("Kit", os.path.join(BASE, "Kit.tsx")),
    ("kitSelectionHelper", os.path.join(BASE, "kitSelectionHelper.ts")),
    ("Design", os.path.join(BASE, "Design.tsx")),
    ("Type", os.path.join(BASE, "Type.tsx")),
    ("Quality", os.path.join(BASE, "Quality.tsx")),
    ("Docs", os.path.join(BASE, "Docs.tsx")),
    ("Home", os.path.join(BASE, "Home.tsx")),
    ("Feedback", os.path.join(BASE, "Feedback.tsx")),
]

RENAMES = {
    "Kit": {
        "DesignSection": "KitDesignSection",
        "ConnectorSection": "KitConnectorSection",
        "defaultDiagramForceSettings": "kitDefaultDiagramForceSettings",
        "App": "KitAppView",
        "DiagramWindow": "KitDiagramWindow",
        "TableWindow": "KitTableWindow",
        "ConnectorSectionForm": "KitConnectorSectionForm",
    },
    "Design": {
        "DesignSection": "DesignDesignSection",
        "ConnectorSection": "DesignConnectorSection",
        "SelectionNormalTool": "DesignSelectionNormalTool",
        "SelectionAdditiveTool": "DesignSelectionAdditiveTool",
        "SelectionSubtractiveTool": "DesignSelectionSubtractiveTool",
        "HandTool": "DesignHandTool",
        "AppProps": "DesignAppProps",
        "App": "DesignAppInner",
        "DiagramWindow": "DesignDiagramWindow",
        "GLTFMesh": "DesignGLTFMesh",
        "FBXMesh": "DesignFBXMesh",
        "OBJMesh": "DesignOBJMesh",
        "ConnectorSectionForm": "DesignConnectorSectionForm",
        "KitSectionLazy": "DesignKitSectionLazy",
    },
    "Type": {
        "ConnectorSection": "TypeConnectorSection",
        "SelectionNormalTool": "TypeSelectionNormalTool",
        "SelectionAdditiveTool": "TypeSelectionAdditiveTool",
        "SelectionSubtractiveTool": "TypeSelectionSubtractiveTool",
        "HandTool": "TypeHandTool",
        "App": "TypeAppInner",
        "GLTFMesh": "TypeGLTFMesh",
        "FBXMesh": "TypeFBXMesh",
        "OBJMesh": "TypeOBJMesh",
        "KitSectionLazy": "TypeKitSectionLazy",
    },
    "Quality": {
        "AppProps": "QualityAppProps",
        "App": "QualityAppView",
        "DiagramWindow": "QualityDiagramWindow",
    },
    "Docs": {
        "App": "DocsAppView",
    },
    "Home": {
        "KitSection": "HomeKitSection",
    },
}

COMMANDS_RENAME = {
    "Kit": "kitAppCommands",
    "Design": "designAppCommands",
    "Type": "typeAppCommands",
}

STRIP_FROM_APP = {
    "Design": ["DesignAppFullscreenWindow"],
    "Type": ["TypeAppHover", "TypeAppSelection", "TypeAppFullscreenWindow"],
    "Kit": ["DiagramForceSettings"],
}

REMOVE_FROM_SKETCHPAD = [
    "DesignAppHover",
    "DesignAppSelection",
    "DesignAppState",
    "KitAppSelection",
    "KitAppState",
    "QualityAppSelection",
    "QualityAppState",
    "TypeAppState",
]

# Function declarations to remove from Sketchpad (redefined in app files)
REMOVE_FUNCTIONS_FROM_SKETCHPAD = [
    "areSameDesignApp",
    "hasSameDesignApp",
    "areSameKitApp",
    "hasSameKitApp",
]

# Lines to strip from app files (regex patterns matched against full line)
STRIP_LINES_FROM_APP = {
    "Design": [r"^let designAppCommands\b"],
}

STRIP_IMPORTS_FROM = {"./Sketchpad", "./Kit", "./Docs"}


def find_import_blocks(content):
    """Parse content into import blocks and non-import sections.
    Returns list of (is_import, text, module_path_or_None) tuples.
    """
    lines = content.split("\n")
    result = []
    i = 0
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        # Check if this is the start of an import statement
        if stripped.startswith("import "):
            # Collect the entire import statement (might span multiple lines)
            import_lines = [line]
            # Check if the import is complete (has 'from' and ends with ';')
            while i < len(lines) - 1:
                full = "\n".join(import_lines)
                if re.search(r"""from\s+['"][^'"]+['"]\s*;?\s*$""", full):
                    break
                if re.search(r"""['"][^'"]+['"]\s*;?\s*$""", full) and "{" not in full:
                    # Side-effect import like: import "./globals.css";
                    break
                i += 1
                import_lines.append(lines[i])

            full_import = "\n".join(import_lines)
            # Extract module path
            mod_match = re.search(r"""from\s+['"]([^'"]+)['"]""", full_import)
            mod_path = mod_match.group(1) if mod_match else None
            result.append((True, full_import, mod_path))
        else:
            result.append((False, line, None))
        i += 1
    return result


def strip_relative_imports(content, modules_to_strip):
    """Remove entire import blocks that reference given relative modules."""
    blocks = find_import_blocks(content)
    result_parts = []
    for is_import, text, mod_path in blocks:
        if is_import and mod_path in modules_to_strip:
            continue  # Skip this import entirely
        result_parts.append(text)
    return "\n".join(result_parts)


def deduplicate_imports(content, existing_imports):
    """Remove imports from app file that already exist in Sketchpad.tsx.
    existing_imports: dict of {module_path: set(imported_names)}
    """
    blocks = find_import_blocks(content)
    result_parts = []
    for is_import, text, mod_path in blocks:
        if is_import and mod_path and mod_path in existing_imports:
            # Parse imported names from this import
            brace_match = re.search(r"\{([^}]+)\}", text)
            if brace_match:
                names_str = brace_match.group(1)
                # Parse individual names (handling "as" aliases and "type" prefixes)
                import_items = []
                for item in re.split(r",", names_str):
                    item = item.strip()
                    if not item:
                        continue
                    # Remove 'type ' prefix
                    clean = re.sub(r"^type\s+", "", item)
                    # Handle 'as' alias - use the alias name
                    as_match = re.match(r"(\w+)\s+as\s+(\w+)", clean)
                    if as_match:
                        local_name = as_match.group(2)
                        original_name = as_match.group(1)
                    else:
                        local_name = clean.strip()
                        original_name = clean.strip()
                    import_items.append((item.strip(), local_name, original_name))

                # Filter out names already imported
                existing = existing_imports[mod_path]
                new_items = []
                for item_text, local_name, original_name in import_items:
                    if local_name not in existing:
                        new_items.append(item_text)

                if not new_items:
                    continue  # All names already imported, skip entirely
                elif len(new_items) < len(import_items):
                    # Reconstruct import with only new names
                    is_type_import = text.strip().startswith("import type")
                    prefix = "import type" if is_type_import else "import"
                    new_text = (
                        f"{prefix} {{\n  "
                        + ",\n  ".join(new_items)
                        + f',\n}} from "{mod_path}";'
                    )
                    result_parts.append(new_text)
                    continue
            # Default: keep import as-is
            result_parts.append(text)
        else:
            result_parts.append(text)
    return "\n".join(result_parts)


def collect_imports(content):
    """Collect all imported names from content, keyed by module path."""
    blocks = find_import_blocks(content)
    imports = {}
    for is_import, text, mod_path in blocks:
        if is_import and mod_path:
            brace_match = re.search(r"\{([^}]+)\}", text)
            if brace_match:
                names_str = brace_match.group(1)
                names = set()
                for item in re.split(r",", names_str):
                    item = item.strip()
                    if not item:
                        continue
                    clean = re.sub(r"^type\s+", "", item)
                    as_match = re.match(r"(\w+)\s+as\s+(\w+)", clean)
                    if as_match:
                        names.add(as_match.group(2))
                    else:
                        names.add(clean.strip())
                imports.setdefault(mod_path, set()).update(names)
    return imports


def find_declaration_range(content, name):
    lines = content.split("\n")
    decl_pattern = re.compile(
        rf"^export\s+(?:type\s+)?(?:interface|enum)\s+{re.escape(name)}\b"
    )
    decl_line = None
    for i, line in enumerate(lines):
        if decl_pattern.match(line):
            decl_line = i
            break
    if decl_line is None:
        return None, None

    start = decl_line
    for i in range(decl_line - 1, -1, -1):
        stripped = lines[i].strip()
        if (
            stripped.startswith("*")
            or stripped.startswith("/**")
            or stripped.startswith("*/")
            or stripped.startswith("//")
        ):
            start = i
        elif stripped == "":
            start = i
        else:
            break

    depth = 0
    end = decl_line
    for i in range(decl_line, len(lines)):
        depth += lines[i].count("{") - lines[i].count("}")
        if depth <= 0:
            end = i
            break

    return start, end


def remove_declarations(content, names):
    for name in names:
        start, end = find_declaration_range(content, name)
        if start is not None:
            lines = content.split("\n")
            lines[start : end + 1] = []
            content = "\n".join(lines)
    return content


def find_function_range(content, name):
    """Find the range of a function declaration including preceding comments."""
    lines = content.split("\n")
    decl_pattern = re.compile(rf"^(?:export\s+)?function\s+{re.escape(name)}\b")
    decl_line = None
    for i, line in enumerate(lines):
        if decl_pattern.match(line):
            decl_line = i
            break
    if decl_line is None:
        return None, None

    # Walk backwards to include comments/blank lines
    start = decl_line
    for i in range(decl_line - 1, -1, -1):
        stripped = lines[i].strip()
        if (
            stripped.startswith("*")
            or stripped.startswith("/**")
            or stripped.startswith("*/")
            or stripped.startswith("//")
        ):
            start = i
        elif stripped == "":
            start = i
        else:
            break

    # Walk forward to find the closing brace
    depth = 0
    end = decl_line
    for i in range(decl_line, len(lines)):
        depth += lines[i].count("{") - lines[i].count("}")
        if depth <= 0:
            end = i
            break

    return start, end


def remove_function_declarations(content, names):
    """Remove function declarations from content by name."""
    for name in names:
        start, end = find_function_range(content, name)
        if start is not None:
            lines = content.split("\n")
            lines[start : end + 1] = []
            content = "\n".join(lines)
    return content


def strip_matching_lines(content, patterns):
    """Remove lines matching any of the given regex patterns."""
    if not patterns:
        return content
    compiled = [re.compile(p) for p in patterns]
    lines = content.split("\n")
    result = []
    for line in lines:
        if any(p.search(line) for p in compiled):
            continue
        result.append(line)
    return "\n".join(result)


def strip_header(content):
    match = re.search(r"// #endregion\s+🔖?Header\b", content)
    if match:
        return content[match.end() :].lstrip("\n")
    match2 = re.search(r"// #endregion\s+Header\b", content)
    if match2:
        return content[match2.end() :].lstrip("\n")
    return content


def rename_symbol(content, old_name, new_name):
    return re.sub(r"\b" + re.escape(old_name) + r"\b", new_name, content)


def fix_lazy_kit_import(content, label):
    pattern = r'const\s+\w+\s*=\s*React\.lazy\(\s*\(\)\s*=>\s*import\(\s*["\']\.\/Kit["\']\s*\)\.then\(\s*\(module\)\s*=>\s*\(\{\s*default:\s*module\.KitSection\s*\}\)\s*\)\s*\);'
    if label == "Design":
        return re.sub(
            pattern,
            "const DesignKitSectionLazy = React.lazy(async () => ({ default: KitSection }));",
            content,
        )
    elif label == "Type":
        return re.sub(
            pattern,
            "const TypeKitSectionLazy = React.lazy(async () => ({ default: KitSection }));",
            content,
        )
    return content


def strip_default_export(content):
    return re.sub(r"^export\s+default\s+\w+;\s*$", "", content, flags=re.MULTILINE)


def rename_commands_targeted(content, new_name):
    content = re.sub(
        r"^(export\s+const\s+)commands(\s*[=:{])",
        rf"\1{new_name}\2",
        content,
        flags=re.MULTILINE,
    )
    content = re.sub(
        r"Object\.entries\(commands\)", f"Object.entries({new_name})", content
    )
    content = re.sub(r"useRef\(commands\)", f"useRef({new_name})", content)
    content = re.sub(r"\.current\s*=\s*commands;", f".current = {new_name};", content)
    return content


def rename_config(content, label):
    prefix = label[0].lower() + label[1:]
    new_name = f"{prefix}Config"
    content = re.sub(
        r"^(export\s+const\s+)config(\s*[:=])",
        rf"\1{new_name}\2",
        content,
        flags=re.MULTILINE,
    )
    return content


def process_file(label, filepath, sketchpad_imports):
    with open(filepath, "r") as f:
        content = f.read()

    content = strip_header(content)
    content = strip_relative_imports(content, STRIP_IMPORTS_FROM)

    # Deduplicate imports already in Sketchpad
    content = deduplicate_imports(content, sketchpad_imports)

    # Strip identical type declarations
    strip_list = STRIP_FROM_APP.get(label, [])
    if strip_list:
        content = remove_declarations(content, strip_list)

    # Strip matching lines (e.g., forward variable declarations)
    line_patterns = STRIP_LINES_FROM_APP.get(label, [])
    if line_patterns:
        content = strip_matching_lines(content, line_patterns)

    if label in ("Design", "Type"):
        content = fix_lazy_kit_import(content, label)

    renames = RENAMES.get(label, {})
    for old, new in renames.items():
        content = rename_symbol(content, old, new)

    if label in COMMANDS_RENAME:
        content = rename_commands_targeted(content, COMMANDS_RENAME[label])

    content = rename_config(content, label)
    content = strip_default_export(content)

    wrapped = f"\n// #region 🔖{label}\n"
    wrapped += f"// Consolidated from {os.path.basename(filepath)}\n\n"
    wrapped += content.strip()
    wrapped += f"\n// #endregion 🔖{label}\n"

    return wrapped


def fix_sketchpad(content):
    # Rename commands
    content = re.sub(
        r"^(export\s+const\s+)commands(\s*=\s*\{)",
        r"\1sketchpadCommands\2",
        content,
        count=1,
        flags=re.MULTILINE,
    )
    content = content.replace(
        "Object.entries(commands).forEach(([commandId, command]) => {\n      this.registerCommand(commandId, command);\n    });\n    Object.entries(devCommands)",
        "Object.entries(sketchpadCommands).forEach(([commandId, command]) => {\n      this.registerCommand(commandId, command);\n    });\n    Object.entries(devCommands)",
    )

    # Update TypeAppFullscreenWindow
    old_enum = """export enum TypeAppFullscreenWindow {
  None = "none",
  Scene = "scene",
}"""
    new_enum = """export enum TypeAppFullscreenWindow {
  None = "none",
  Connectors = "connectors",
  Models = "models",
}"""
    content = content.replace(old_enum, new_enum)

    # Remove stubs
    content = remove_declarations(content, REMOVE_FROM_SKETCHPAD)

    # Remove function declarations that are redefined in app files
    content = remove_function_declarations(content, REMOVE_FUNCTIONS_FROM_SKETCHPAD)

    return content


def main():
    os.chdir("/workspaces/semio")

    with open(SKETCHPAD, "r") as f:
        sketchpad_content = f.read()

    print(f"Original Sketchpad.tsx: {len(sketchpad_content.splitlines())} lines")

    # Collect existing imports before modification
    sketchpad_imports = collect_imports(sketchpad_content)
    print(f"Found imports from {len(sketchpad_imports)} modules in Sketchpad")

    sketchpad_content = fix_sketchpad(sketchpad_content)

    if "export const sketchpadCommands" not in sketchpad_content:
        print("ERROR: Failed to rename sketchpad commands!")
        return
    print("✓ Sketchpad pre-processing done")

    consolidated_parts = []
    for label, filepath in FILES:
        print(f"Processing {label}...")
        part = process_file(label, filepath, sketchpad_imports)
        consolidated_parts.append(part)
        print(f"  → {len(part.splitlines())} lines")

    full_content = sketchpad_content.rstrip()
    full_content += "\n\n// #region 🔖ConsolidatedApps\n"
    full_content += "// All app modules consolidated from individual files.\n"
    for part in consolidated_parts:
        full_content += part
    full_content += "\n// #endregion 🔖ConsolidatedApps\n"

    total = len(full_content.splitlines())
    print(f"\nTotal: {total} lines")

    # Verify
    const_pat = re.compile(r"^export\s+const\s+(\w+)\s*[=:{]", re.MULTILINE)
    enum_pat = re.compile(r"^export\s+enum\s+(\w+)\b", re.MULTILINE)
    func_pat = re.compile(r"^(?:export\s+)?function\s+(\w+)\s*[(<]", re.MULTILINE)
    let_pat = re.compile(r"^(?:export\s+)?(?:let|var)\s+(\w+)\s*[=;:]", re.MULTILINE)

    const_dupes = [
        n
        for n, c in collections.Counter(const_pat.findall(full_content)).items()
        if c > 1
    ]
    enum_dupes = [
        n
        for n, c in collections.Counter(enum_pat.findall(full_content)).items()
        if c > 1
    ]
    func_dupes = [
        n
        for n, c in collections.Counter(func_pat.findall(full_content)).items()
        if c > 1
    ]
    let_dupes = [
        n
        for n, c in collections.Counter(let_pat.findall(full_content)).items()
        if c > 1
    ]

    if const_dupes:
        print(f"⚠ Duplicate const: {const_dupes}")
    else:
        print("✓ No duplicate const declarations")
    if enum_dupes:
        print(f"⚠ Duplicate enum: {enum_dupes}")
    else:
        print("✓ No duplicate enum declarations")
    if func_dupes:
        print(f"⚠ Duplicate function: {func_dupes}")
    else:
        print("✓ No duplicate function declarations")
    if let_dupes:
        print(f"⚠ Duplicate let/var: {let_dupes}")
    else:
        print("✓ No duplicate let/var declarations")

    iface_pat = re.compile(r"^export\s+(?:type\s+)?interface\s+(\w+)\b", re.MULTILINE)
    iface_dupes = [
        n
        for n, c in collections.Counter(iface_pat.findall(full_content)).items()
        if c > 1
    ]
    if iface_dupes:
        print(f"ℹ Merged interfaces: {iface_dupes}")

    with open(SKETCHPAD, "w") as f:
        f.write(full_content)

    print(f"✓ Written to {SKETCHPAD}")


if __name__ == "__main__":
    main()
