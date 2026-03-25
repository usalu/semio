import os
import re

files_to_consolidate = [
    "elements/ui/index.ts",
    "elements/ui/elements.tsx",
    "elements/ui/tailwind.config.ts",
    "elements/ui/postcss.config.ts",
    "elements/ui/eslint.config.ts",
    "elements/ui/vitest.config.ts",
    "elements/ui/stories/elements/Button.stories.tsx"
]

storybook_stories_dir = "elements/ui/.storybook/stories/"
if os.path.exists(storybook_stories_dir):
    for f in os.listdir(storybook_stories_dir):
        if f.endswith(".stories.tsx"):
            files_to_consolidate.append(os.path.join(storybook_stories_dir, f))

storybook_dir = "elements/ui/.storybook/"
if os.path.exists(storybook_dir):
    for f in os.listdir(storybook_dir):
        if f.endswith(".ts") or f.endswith(".tsx"):
            if f not in ["main.ts", "preview.ts"]:
                files_to_consolidate.append(os.path.join(storybook_dir, f))

all_imports = set()
file_contents = []

def extract_imports(content):
    pattern = r'^import\s+[^;]+;'
    found_imports = re.findall(pattern, content, re.MULTILINE | re.DOTALL)
    new_content = re.sub(pattern, '', content, flags=re.MULTILINE | re.DOTALL)
    return found_imports, new_content

def process_file(file_path):
    print(f"Processing {file_path}")
    with open(file_path, 'r') as f:
        content = f.read()
    
    content = re.sub(r'// #region 🔖Header.*?// #endregion 🔖Header', '', content, flags=re.DOTALL)
    content = re.sub(r'// #region Header.*?// #endregion Header', '', content, flags=re.DOTALL)
    
    imports, body = extract_imports(content)
    
    cleaned_imports = []
    for imp in imports:
        if '"@elements/ui"' in imp or "'@elements/ui'" in imp:
            continue
        if '"./' in imp or "'./" in imp:
            match = re.search(r'from\s+["\'](\.\/[^"\']+)["\']', imp)
            if match:
                rel_path = match.group(1).rsplit('.', 1)[0]
                is_consolidated = False
                for cf in files_to_consolidate:
                    cf_base = os.path.basename(cf).rsplit('.', 1)[0]
                    if rel_path == f"./{cf_base}" or rel_path == f"../{cf_base}":
                        is_consolidated = True
                        break
                if is_consolidated:
                    continue
        cleaned_imports.append(imp)
    
    all_imports.update(cleaned_imports)
    
    filename_base = os.path.basename(file_path).rsplit('.', 1)[0].replace('.', '_').replace('-', '_')
    
    # Prefix top-level declarations to avoid conflicts
    # This is a bit risky but necessary for stories.
    # We only prefix if it's a story file or if it's not the main elements.tsx
    if "stories" in file_path or file_path.endswith(".config.ts") or "storybook" in file_path:
        # Match 'export const Default', 'const meta', 'type Story', etc.
        # Use a regex that looks for keywords at the start of a line or after an export
        
        declarations = [
            (r'^(export\s+)?const\s+([a-zA-Z0-9_]+)', r'\1const ' + filename_base + r'_\2'),
            (r'^(export\s+)?let\s+([a-zA-Z0-9_]+)', r'\1let ' + filename_base + r'_\2'),
            (r'^(export\s+)?var\s+([a-zA-Z0-9_]+)', r'\1var ' + filename_base + r'_\2'),
            (r'^(export\s+)?type\s+([a-zA-Z0-9_]+)', r'\1type ' + filename_base + r'_\2'),
            (r'^(export\s+)?interface\s+([a-zA-Z0-9_]+)', r'\1interface ' + filename_base + r'_\2'),
            (r'^(export\s+)?function\s+([a-zA-Z0-9_]+)', r'\1function ' + filename_base + r'_\2'),
            (r'^(export\s+)?enum\s+([a-zA-Z0-9_]+)', r'\1enum ' + filename_base + r'_\2'),
            (r'^(export\s+)?class\s+([a-zA-Z0-9_]+)', r'\1class ' + filename_base + r'_\2'),
            (r'^(export\s+)?default\s+([a-zA-Z0-9_]+)', r'export const ' + filename_base + r'_default = \2'),
            # Handle default exports that are expressions
            (r'^export\s+default\s+([^;]+);', r'export const ' + filename_base + r'_default = \1;')
        ]
        
        # Also need to replace occurrences of these variables within the SAME file
        # This is very hard with regex. 
        # But we can at least try to find common ones like 'meta', 'Story', 'Default'.
        
        to_rename = ["meta", "Story", "Default", "Base", "Primary", "Secondary", "Large", "Small", "tailwindConfig", "postcssConfig"]
        for name in to_rename:
             # Look for the name as a whole word, but not preceded by a dot (to avoid property access)
             # and not followed by a colon in an object (though that might be okay).
             # This is still dangerous.
             pass

        # Better approach: if it's a story file, we wrap it in a function or something? No.
        # Let's just do a simple prefixing for the declarations and pray.
        # Actually, for stories, the exports ARE the most important part.
        
        for pattern, replacement in declarations:
            body = re.sub(pattern, replacement, body, flags=re.MULTILINE)
            
        # Specific fix for story meta references
        body = body.replace('StoryObj<typeof meta>', f'StoryObj<typeof {filename_base}_meta>')
        body = body.replace('StoryObj<typeof cycleMeta>', f'StoryObj<typeof {filename_base}_cycleMeta>')

    region_name = os.path.basename(file_path).replace('.', '_')
    return f"// #region 🔖{region_name}\n{body.strip()}\n// #endregion 🔖{region_name}\n"

for fp in files_to_consolidate:
    if os.path.exists(fp):
        file_contents.append(process_file(fp))

header = """// #region 🔖Header

// 💻 elements/ui/index.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Shared export surface for elements ui primitives.

// #endregion 🔖Header

"""

# Sort and join imports
sorted_imports = sorted(list(all_imports))
imports_content = "// #region 🔖Imports\n\n" + "\n".join(sorted_imports) + "\n\n// #endregion 🔖Imports\n\n"

with open("elements/ui/index.tsx", "w") as f:
    f.write(header)
    f.write(imports_content)
    f.write("\n".join(file_contents))

print("Created index.tsx")
