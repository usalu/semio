import os, re, sys

FILES_WITH_DIRNAME = [
"✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/🧪️vitest.config.ts",
"🌎️hub/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/🧪️vitest.config.ts",
"🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️store/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window-kits/🧪️vitest.config.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/🧪️vitest.config.ts",
"🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/🧪️vitest.config.ts",
]

changed = []
for rel in FILES_WITH_DIRNAME:
    if not os.path.exists(rel):
        print("MISSING", rel); sys.exit(1)
    with open(rel, "r", encoding="utf-8") as f:
        content = f.read()
    original = content
    needle = "dirname(fileURLToPath(import.meta.url))"
    replacement = 'resolve(dirname(fileURLToPath(import.meta.url)), "..")'
    count = content.count(needle)
    if count != 1:
        print("UNEXPECTED COUNT", count, rel); sys.exit(1)
    content = content.replace(needle, replacement)
    # ensure resolve is imported alongside dirname from node:path
    m = re.search(r'import \{ dirname \} from "node:path";', content)
    if m:
        content = content.replace('import { dirname } from "node:path";', 'import { dirname, resolve } from "node:path";')
    if 'resolve' not in re.search(r'import \{[^}]*\} from "node:path";', content).group(0):
        print("resolve still missing import in", rel); sys.exit(1)
    new_dir = os.path.dirname(rel) + "/🧪️tests"
    new_path = new_dir + "/🟦️.ts"
    os.makedirs(new_dir, exist_ok=True)
    with open(new_path, "w", encoding="utf-8") as f:
        f.write(content)
    os.remove(rel)
    changed.append((rel, new_path))

for old, new in changed:
    print("MOVED", old, "->", new)
print(f"Total: {len(changed)}")
