import os
import re

index_path = "elements/ui/index.tsx"
stories_dir = "elements/ui/.storybook/stories/"

# Collect all story files
story_files = []
if os.path.exists(stories_dir):
    for f in os.listdir(stories_dir):
        if f.endswith(".stories.tsx"):
            story_files.append(os.path.join(stories_dir, f))

# Also elements/ui/stories/elements/Button.stories.tsx
button_stories = "elements/ui/stories/elements/Button.stories.tsx"
if os.path.exists(button_stories):
    story_files.append(button_stories)

def process_story(file_path):
    print(f"Adding {file_path}")
    with open(file_path, 'r') as f:
        content = f.read()
    
    # Strip headers and imports
    content = re.sub(r'// #region 🔖Header.*?// #endregion 🔖Header', '', content, flags=re.DOTALL)
    content = re.sub(r'// #region Header.*?// #endregion Header', '', content, flags=re.DOTALL)
    
    # We'll extract imports to append to top later, but for now just strip them
    pattern = r'^import\s+[^;]+;'
    imports = re.findall(pattern, content, re.MULTILINE | re.DOTALL)
    body = re.sub(pattern, '', content, flags=re.MULTILINE | re.DOTALL)
    
    # Prefix local symbols in stories to avoid conflicts
    filename_base = os.path.basename(file_path).rsplit('.', 1)[0].replace('.', '_').replace('-', '_')
    
    # Prefix common symbols: meta, Story, Default, Base, Primary, Secondary, etc.
    # We do a whole-word replace on the body for these specific ones.
    # Note: this is still a bit dangerous but better than nothing.
    to_prefix = ["meta", "Story", "Default", "Base", "Primary", "Secondary", "Large", "Small", "Create", "Edit", "Delete", "View", "List", "Tree", "Table", "Card", "Orb", "Ring", "Panel", "Window", "Overlay", "Temporary", "Layout", "Section", "Item", "Node", "Link", "Edge", "Handle", "Port", "Tag", "Concept", "Author", "File", "Folder", "Design", "Type", "Quality", "Level", "Expertise", "Mode", "Device", "Theme", "Language", "Layout", "Config", "Props", "Args", "StoryObj", "Meta"]
    
    # Filter only those that are likely declared as top-level in stories
    # We use a heuristic: if 'const NAME =' or 'type NAME =' or 'export const NAME ='
    declared = re.findall(r'^(?:export\s+)?(?:const|let|var|type|interface|function|enum|class)\s+([a-zA-Z0-9_]+)', body, re.MULTILINE)
    
    for name in declared:
        # Avoid prefixing already prefixed or very unique names
        if name in ["React", "useEffect", "useState", "useMemo", "useCallback", "useRef", "useTranslation"]:
            continue
            
        new_name = f"{filename_base}_{name}"
        # Whole word replace but avoid property access or key: value
        body = re.sub(r'\b' + name + r'\b', new_name, body)
        
    region_name = os.path.basename(file_path).replace('.', '_')
    return f"\n// #region 🔖Stories_{region_name}\n{body.strip()}\n// #endregion 🔖Stories_{region_name}\n", imports

all_story_imports = []
all_story_bodies = []

for sf in story_files:
    body, imports = process_story(sf)
    all_story_bodies.append(body)
    all_story_imports.extend(imports)

# Read index.tsx
with open(index_path, 'r') as f:
    index_content = f.read()

# Append imports to the top and bodies to the end
# We'll use another script to deduplicate imports
with open(index_path, 'w') as f:
    f.write("\n".join(all_story_imports) + "\n\n" + index_content + "\n\n" + "\n".join(all_story_bodies))

print("Added stories to index.tsx")
