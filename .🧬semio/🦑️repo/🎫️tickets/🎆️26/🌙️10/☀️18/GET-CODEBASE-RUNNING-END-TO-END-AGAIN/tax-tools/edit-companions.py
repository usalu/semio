# 🔬 Anchored text edit of taxonomy.json adding fixed filename contracts for declaration companions of non-TypeScript leaves and nested Go command modules. Idempotent; aborts when an anchor is missing or not unique.
import sys
T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
s = open(T, encoding="utf8").read()
if '"javascript-module-declaration"' in s:
    sys.exit("already applied")
anchor = '''    "root-bun-config": {'''
if s.count(anchor) != 1:
    sys.exit(f"anchor count {s.count(anchor)}")
def contract(id_, pattern, authority, reason, verification):
    return f'''    "{id_}": {{
      "pathPattern": "{pattern}",
      "authority": "{authority}",
      "reason": "{reason}",
      "configurability": "unconfigurable",
      "scope": {{
        "kind": "path-pattern"
      }},
      "verification": "{verification}",
      "expires": null
    }},
'''
rows = [
    ("javascript-module-declaration", "**/🟨️.d.mts", "TypeScript", "Declaration companion of the sibling JavaScript module leaf, named by that leaf's own kind emoji", "tsc --noEmit"),
    ("sql-module-declaration", "**/🗄️.d.ts", "TypeScript", "Declaration companion of the sibling SQL module leaf, named by that leaf's own kind emoji", "tsc --noEmit"),
    ("sql-module-extension-declaration", "**/🗄️.sql.d.ts", "TypeScript", "Declaration companion of the sibling SQL module leaf imported with its extension, named by that leaf's own kind emoji", "tsc --noEmit"),
    ("go-command-module-manifest", "**/📦️packages/🐹️go/*/go.mod", "Go", "Nested Go command module manifest below its package module root", "go list -m"),
]
s = s.replace(anchor, "".join(contract(*r) for r in rows) + anchor)
open(T, "w", encoding="utf8").write(s)
print("ok")
