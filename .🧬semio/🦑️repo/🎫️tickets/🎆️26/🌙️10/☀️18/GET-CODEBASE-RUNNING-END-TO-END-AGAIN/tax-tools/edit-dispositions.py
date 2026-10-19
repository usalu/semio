# 🔬 Anchored text edit of taxonomy.json adding the package source dispositions for the declaration-companion fixed contracts. Idempotent; aborts when the anchor is missing or not unique.
import sys
T = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
s = open(T, encoding="utf8").read()
if '    "javascript-module-declaration": {\n      "contractKind"' in s:
    sys.exit("already applied")
anchor = '''      "authority": "Storybook preview adapter identity without package purity exemption",
      "verification": "storybook build configuration loading"
    }
  },'''
if s.count(anchor) != 1:
    sys.exit(f"anchor count {s.count(anchor)}")
def row(id_, authority):
    return f''',
    "{id_}": {{
      "contractKind": "fixed",
      "disposition": "adapter-source",
      "validator": "package-glue",
      "authority": "{authority}",
      "verification": "tsc --noEmit"
    }}'''
rows = [("javascript-module-declaration", "Declaration companion of a JavaScript module leaf without purity exemption"),
        ("sql-module-declaration", "Declaration companion of an SQL module leaf without purity exemption"),
        ("sql-module-extension-declaration", "Extension-qualified declaration companion of an SQL module leaf without purity exemption")]
new = '''      "authority": "Storybook preview adapter identity without package purity exemption",
      "verification": "storybook build configuration loading"
    }''' + "".join(row(*r) for r in rows) + '''
  },'''
s = s.replace(anchor, new)
open(T, "w", encoding="utf8").write(s)
print("ok")
