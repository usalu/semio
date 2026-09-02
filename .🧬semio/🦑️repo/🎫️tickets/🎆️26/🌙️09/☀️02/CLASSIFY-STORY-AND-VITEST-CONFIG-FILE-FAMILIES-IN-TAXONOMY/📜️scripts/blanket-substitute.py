import os

FILES = [
"📜️script.ts",
"🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust/📜️script.ts",
"🧰️framework/🔨️modules/🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
"🧰️framework/🛍️products/🖥️server/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🟦️.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏪️店/📜️script.ts".replace("🏪️店","🏪️store"),
"🧰️framework/🛍️products/💻️os/🔨️模块/🔌️plugin/🪟️window-kits/📜️script.ts".replace("🔨️模块","🔨️modules"),
"🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🧑️‍💻️dev/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/🧱️elements/TiledMapHost/🧪️component.test.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🦀️rust/🎯️targets/🧊️wgpu/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑️‍🎨️engine/📦️packages/🟦️typescript/🎯️targets/⚛️react/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust/📜️script.ts",
"🧰️framework/🛍️products/💻️os/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🟦️typescript/📜️script.ts",
"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json",
"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️run-vitest-config-argument-tokens/🟦️.ts",
"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts",
"🧰️framework/🛍️products/🦑️repo/🔨️模块/📚️library/🔍️discovery/🟦️.ts".replace("🔨️模块","🔨️modules"),
"🧰️framework/📦️packages/🦀️rust/📜️script.ts",
"✏️s/🔌️plugins/🔱️trinity/🔨️modules/🔌️jack/🧠️lsp/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/🌊️flow/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/📐️cad/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/🏢️aec-building/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/📐️spatial-shape/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/🏛️aec-building-structure/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/📐️cad/🧩️extensions/🔥️aec-building-energy/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/📜️script.ts",
"✏️s/🔌️plugins/🧩️puzzle/📦️packages/🟦️typescript/🎯️targets/⚛️5d-react/📜️script.ts",
"🌎️hub/🔨️modules/🛡️admin/📦️packages/🟦️typescript/📜️script.ts",
".vscode/settings.json",
]

OLD = "🧪️vitest.config.ts"
NEW = "🧪️tests/🟦️.ts"

total = 0
for rel in FILES:
    if not os.path.exists(rel):
        print("MISSING", rel)
        continue
    with open(rel, "r", encoding="utf-8") as f:
        content = f.read()
    count = content.count(OLD)
    if count == 0:
        print("NO OCCURRENCE (unexpected)", rel)
        continue
    content = content.replace(OLD, NEW)
    with open(rel, "w", encoding="utf-8") as f:
        f.write(content)
    print(f"UPDATED {rel} ({count} occurrence(s))")
    total += count
print("TOTAL substitutions:", total)
