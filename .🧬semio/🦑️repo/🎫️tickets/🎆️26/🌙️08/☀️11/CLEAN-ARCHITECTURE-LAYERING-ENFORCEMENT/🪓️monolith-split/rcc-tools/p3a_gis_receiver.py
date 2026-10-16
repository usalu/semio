"""🌍️ Moves the gis receiver-configuration assertion from the hub io-boundary test to the s dev ownership test."""
import re
R = "/Users/ueli/Documents/semio/"
hub = R + "🌎️hub/🧩️compositions/🌍️gis/🧪️tests/🧱️io-boundary/🟦️.ts"
own = R + "✏️s/🧑‍💻dev/🧩️service-composition/🧪️tests/🏷️ownership/🟦️.ts"
h = open(hub, encoding="utf-8").read()
start = h.index('test("GIS receiving configuration selects its owner')
end = h.index('test("GIS receiving schema compilers')
first = h[start:end]
assert first.count("readFileSync") == 3 and first.count("[DEBUG]") == 1
literal = re.search(r'toContain\("([^"]+)"\)', first).group(1)
assert literal.endswith("🧩️service-composition/👷️worker"), literal
config = re.search(r'new URL\("\.\./\.\./\.\./\.\./\.\./✏️s/([^"]+🎚️config/🟦️\.ts)"', first).group(1)
new_hub = h[:start] + h[end:]
new_hub = new_hub.replace('import {readdirSync,readFileSync} from "node:fs";', 'import {readFileSync} from "node:fs";')
assert new_hub.count("readdirSync") == 0
o = open(own, encoding="utf-8").read()
anchor = "  assert(receivingFixtures > 8);\n"
assert o.count(anchor) == 1
addition = anchor + f'  const receiverConfiguration = readFileSync(join(repoRoot, "✏️s/{config}"), "utf8");\n  assert(receiverConfiguration.includes("{literal}"));\n'
o = o.replace(anchor, addition)
open(own, "w", encoding="utf-8").write(o)
open(hub, "w", encoding="utf-8").write(new_hub)
print("ok", literal, config)
