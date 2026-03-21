#!/usr/bin/env python3
"""Consolidate all sketchpad source files into Sketchpad.tsx.

Strategy:
1. Use regex to find all import/export-from statements in each file
2. Classify as internal (to be stripped) vs external (to be kept)
3. Collect code sections (everything that's not header or import)
4. Output consolidated file with proper region structure
"""

import re
import os

BASE = "/workspaces/semio/semio/sketchpad/sketchpad"

# Internal module references to strip (these become local within the consolidated file)
INTERNAL_MODULES = {
    "./shared", "./portColor", "./kitSelectionHelper", "./kitSelectionHelpers",
    "./Sketchpad", "./Design", "./Type", "./Kit", "./Docs", "./Feedback",
    "./Home", "./Quality", "./Tutorials", "../shared",
}

# Regex to match import/export-from statements (including multi-line)
# Matches: import ... from "..."; or export { ... } from "...";
IMPORT_RE = re.compile(
    r'^(?:import\s|export\s+\{|export\s+type\s+\{).*?(?:from\s+["\'].*?["\'];?\s*$)',
    re.MULTILINE | re.DOTALL
)

def find_import_statements(content):
    """Find all import and export-from statements, returning (start, end, text, is_internal)."""
    results = []
    lines = content.split('\n')
    i = 0
    
    while i < len(lines):
        line = lines[i]
        stripped = line.strip()
        
        # Check if line starts an import or export-from statement
        is_import_start = (
            stripped.startswith('import ') or 
            stripped.startswith('import{') or
            stripped.startswith('import\t') or
            (stripped.startswith('export {') and 'from' in stripped) or
            (stripped.startswith('export type {') and 'from' in stripped) or
            (stripped.startswith('export type{') and 'from' in stripped)
        )
        
        if not is_import_start:
            i += 1
            continue
        
        # Collect the full statement (may span multiple lines)
        start = i
        statement_lines = [line]
        
        # Check if it's complete on one line
        if stripped.endswith(';'):
            end = i
        else:
            # Multi-line import - find the end
            i += 1
            while i < len(lines):
                statement_lines.append(lines[i])
                current = lines[i].strip()
                if current.endswith(';'):
                    break
                # Also check for patterns like: } from "module"  (without semicolon at very end)
                if re.search(r'from\s+["\'].*?["\']', current):
                    break
                i += 1
            end = i
        
        full_statement = '\n'.join(statement_lines)
        
        # Determine if internal
        is_internal = False
        for mod in INTERNAL_MODULES:
            if f'from "{mod}"' in full_statement or f"from '{mod}'" in full_statement:
                is_internal = True
                break
        
        results.append((start, end, full_statement, is_internal))
        i += 1
    
    return results

def process_file(filepath, keep_header=False):
    """Process a file and return (header, external_imports, code)."""
    with open(filepath, 'r') as f:
        content = f.read()
    
    lines = content.split('\n')
    
    # Find header bounds
    header_start = None
    header_end = None
    for i, line in enumerate(lines):
        stripped = line.strip()
        if stripped in ('// #region Header', '// #region 🔖Header'):
            header_start = i
        if header_start is not None and stripped.startswith('// #endregion'):
            if 'Header' in stripped or header_end is None:
                header_end = i
                break
    
    header = '\n'.join(lines[header_start:header_end+1]) if header_start is not None and keep_header else ''
    
    # Find all import statements
    imports = find_import_statements(content)
    
    # Build set of line numbers that are imports or header
    skip_lines = set()
    if header_start is not None:
        for j in range(header_start, (header_end or header_start) + 1):
            skip_lines.add(j)
    
    external_imports = []
    for start, end, text, is_internal in imports:
        for j in range(start, end + 1):
            skip_lines.add(j)
        if not is_internal:
            external_imports.append(text)
    
    # Collect code lines (everything not header or import)
    code_lines = []
    for i, line in enumerate(lines):
        if i not in skip_lines:
            code_lines.append(line)
    
    code = '\n'.join(code_lines)
    
    # Clean up excessive blank lines at start
    code = code.lstrip('\n')
    
    return header, external_imports, code

def main():
    # 1. Process Sketchpad.tsx (the target file - keep header)
    header, sketchpad_imports, sketchpad_code = process_file(
        os.path.join(BASE, "Sketchpad.tsx"), keep_header=True
    )
    
    # 2. Process all other files
    files_to_merge = {
        "shared.ts": "Shared",
        "portColor.ts": "PortColor", 
        "Tutorials.tsx": "Tutorials",
        "Kit.tsx": "Kit",
        "kitSelectionHelper.ts": "KitSelectionHelper",
        "kitSelectionHelpers.ts": "KitSelectionHelpers",
        "Docs.tsx": "Docs",
        "Design.tsx": "Design",
        "Type.tsx": "Type",
        "Quality.tsx": "Quality",
        "Home.tsx": "Home",
        "Feedback.tsx": "Feedback",
    }
    
    all_imports = list(sketchpad_imports)
    sections = {}
    
    for filename, section_name in files_to_merge.items():
        filepath = os.path.join(BASE, filename)
        if not os.path.exists(filepath):
            print(f"WARNING: {filepath} not found, skipping")
            continue
        
        _, file_imports, code = process_file(filepath)
        all_imports.extend(file_imports)
        sections[filename] = code
        code_lines = len(code.split('\n'))
        print(f"  {filename}: {code_lines} code lines, {len(file_imports)} import groups")
    
    # 3. Deduplicate imports
    seen = set()
    unique_imports = []
    for imp in all_imports:
        key = imp.strip()
        if key and key not in seen:
            seen.add(key)
            unique_imports.append(imp)
    
    print(f"\n  Total unique imports: {len(unique_imports)}")
    
    # 4. Build consolidated file
    output_parts = []
    
    # Header
    output_parts.append(header)
    output_parts.append('')
    
    # Imports
    output_parts.append('// #region 🔖Imports')
    output_parts.append('// Consolidated imports from all sketchpad source files.')
    output_parts.append('')
    output_parts.append('\n'.join(unique_imports))
    output_parts.append('')
    output_parts.append('// #endregion 🔖Imports')
    output_parts.append('')
    
    # Shared utilities (base - no internal deps)
    output_parts.append('// #region 🔖Shared')
    output_parts.append('// Shared types, interfaces, utilities, and registries.')
    output_parts.append(sections.get("shared.ts", ""))
    output_parts.append('// #endregion 🔖Shared')
    output_parts.append('')
    
    # Port color utilities
    output_parts.append('// #region 🔖PortColor')
    output_parts.append('// Port color utilities.')
    output_parts.append(sections.get("portColor.ts", ""))
    output_parts.append('// #endregion 🔖PortColor')
    output_parts.append('')
    
    # Tutorials (needed by Sketchpad core before it references Tutorial stuff)
    output_parts.append('// #region 🔖Tutorials')  
    output_parts.append('// Tutorial system.')
    output_parts.append(sections.get("Tutorials.tsx", ""))
    output_parts.append('// #endregion 🔖Tutorials')
    output_parts.append('')
    
    # Sketchpad core
    output_parts.append('// #region 🔖SketchpadCore')
    output_parts.append('// Core sketchpad store, hooks, machine, providers, and components.')
    output_parts.append(sketchpad_code)
    output_parts.append('// #endregion 🔖SketchpadCore')
    output_parts.append('')
    
    # Apps section with sub-regions
    output_parts.append('// #region 🔖Apps')
    output_parts.append('// App implementations.')
    output_parts.append('')
    
    app_order = [
        ("Design.tsx", "Design"),
        ("Type.tsx", "Type"),
        ("Kit.tsx", "Kit"),
        ("kitSelectionHelper.ts", "KitSelectionHelper"),
        ("kitSelectionHelpers.ts", "KitSelectionHelpers"),
        ("Quality.tsx", "Quality"),
        ("Docs.tsx", "Docs"),
        ("Home.tsx", "Home"),
        ("Feedback.tsx", "Feedback"),
    ]
    
    for filename, name in app_order:
        if filename in sections:
            output_parts.append(f'// #region 🔖{name}')
            output_parts.append(f'// {name} app.')
            output_parts.append(sections[filename])
            output_parts.append(f'// #endregion 🔖{name}')
            output_parts.append('')
    
    output_parts.append('// #endregion 🔖Apps')
    
    # Write output
    outpath = os.path.join(BASE, "Sketchpad.tsx")
    final = '\n'.join(output_parts)
    with open(outpath, 'w') as f:
        f.write(final)
    
    total_lines = len(final.split('\n'))
    print(f"\n  Consolidated file: {total_lines} lines")

if __name__ == '__main__':
    main()
