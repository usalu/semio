import os, tempfile
C = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
def patch(rel, subs):
    P = C + rel
    s = open(P, encoding="utf8").read()
    for old, new in subs:
        assert s.count(old) == 1, (rel, s.count(old), old[:80])
        s = s.replace(old, new)
    fd, tmp = tempfile.mkstemp(dir=os.path.dirname(P), prefix=".tmp-F112-")
    with os.fdopen(fd, "w", encoding="utf8") as f: f.write(s)
    os.replace(tmp, P)

patch("🗂️suites/🔣️.json", [(
'''    {
      "id": "import-edges",
''', '''    {
      "id": "schema-twins-fresh",
      "budgetMs": 300000,
      "typecheck": "none",
      "files": [
        "../🎛️commands/🧬️schema/🧬️twin-projection/🧪️tests/🧮️twins-freshness/🟦️.ts"
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
('''        "test-schema-catalog-fresh",
''', '''        "test-schema-catalog-fresh",
        "test-schema-twins-fresh",
'''),
('''    "test-selected-graph": {''', '''    "test-schema-twins-fresh": {
      "executor": "repo:script",
      "inputs": [
        "default",
        "^default",
        "testEnv",
        "reads",
        "schemaSources"
      ],
      "cache": true,
      "outputs": [],
      "options": {
        "cwd": "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching",
        "command": "repo ./📜️script.ts test schema-twins-fresh",
        "forwardAllArgs": false
      }
    },
    "test-selected-graph": {'''),
])
print("ok")
