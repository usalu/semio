import re

index_path = "elements/ui/index.tsx"

with open(index_path, 'r') as f:
    content = f.read()

# Pattern for imports (handle single and multiline)
pattern = r'^import\s+[^;]+;'
all_imports = re.findall(pattern, content, re.MULTILINE | re.DOTALL)
body = re.sub(pattern, '', content, flags=re.MULTILINE | re.DOTALL)

# Clean up imports
# 1. Remove relative imports that point to consolidated files (e.g. from "@elements/ui" or "./elements")
cleaned_imports = set()
for imp in all_imports:
    imp = imp.strip()
    if '"@elements/ui"' in imp or "'@elements/ui'" in imp:
        continue
    # Remove duplicates
    cleaned_imports.add(imp)

# Sort imports
sorted_imports = sorted(list(cleaned_imports))

header = """// #region 🔖Header

// 💻 elements/ui/index.tsx

// 2026 Ueli Saluz <ueli@semio-tech.com>

// Shared export surface for elements ui primitives.

// #endregion 🔖Header

"""

new_content = header + "// #region 🔖Imports\n\n" + "\n".join(sorted_imports) + "\n\n// #endregion 🔖Imports\n\n" + body.strip()

with open(index_path, 'w') as f:
    f.write(new_content)

print("Deduplicated imports in index.tsx")
