import os, tempfile
C = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
def patch(rel, subs):
    P = C + rel
    s = open(P, encoding="utf8").read()
    for old, new in subs:
        assert s.count(old) == 1, (rel, s.count(old), old[:80])
        s = s.replace(old, new)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(P), prefix=".tmp-F107-")
    with os.fdopen(fd, "w", encoding="utf8") as f: f.write(s)
    os.replace(tmp, P)

patch("🗂️suites/🔣️.json", [(
'''    {
      "id": "import-edges",
''', '''    {
      "id": "schema-catalog-fresh",
      "budgetMs": 600000,
      "typecheck": "none",
      "files": [
        "../🔍️discovery/🧾️schema-scope-catalog/🧪️tests/🧮️catalog-freshness/🟦️.ts"
      ],
      "platforms": [
        "darwin",
        "linux",
        "win32"
      ]
    },
    {
      "id": "import-edges",
''')])
patch("📋️project.json", [
('''        "test-sdk-material",''', '''        "test-schema-catalog-fresh",
        "test-sdk-material",'''),
('''    "test-selected-graph": {''', '''    "test-schema-catalog-fresh": {
      "executor": "repo:script",
      "inputs": [
        "default",
        "^default",
        "testEnv",
        "reads",
        "schemaSources",
        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json",
        "{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📓️schema-catalog.md"
      ],
      "cache": true,
      "outputs": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching",
        "command": "repo ./📜️script.ts test schema-catalog-fresh",
        "forwardAllArgs": false
      }
    },
    "test-selected-graph": {'''),
])
print("ok")
