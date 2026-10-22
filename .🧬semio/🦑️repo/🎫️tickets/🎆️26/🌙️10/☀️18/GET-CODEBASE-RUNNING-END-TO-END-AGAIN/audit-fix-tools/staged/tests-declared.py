#!/usr/bin/env python3
"""Rewrites the cache corpus of the inference tests for the declared classification: test-scope rows carry only the declaration, the closure-driver rows and their test are gone, the artifact row declares its command targets. Dry run by default; --write patches live; --overlay <dir> writes copies."""
import json, os, re, sys
REPO = os.environ.get("AUDIT_FIX_REPO", "/Users/ueli/Documents/semio")
D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/🧪️tests/💾️cache/"
def rd(rel): return open(os.path.join(REPO, D + rel), encoding="utf8").read()
def sub(t, old, new):
    if t.count(old) != 1: sys.exit("refused: anchor drift (%d): %s" % (t.count(old), old[:90]))
    return t.replace(old, new)
def between(t, start, end):
    i = t.index(start); j = t.index(end, i)
    if t.count(start) != 1: sys.exit("refused: anchor drift: " + start)
    return i, j
out = {}
corpus = rd("🔣️.json")
i, k = between(corpus, '  "nativeRunsTests": [', '  "nativeInputs": [')
rows = [
 {"id": "test-contract", "execution": {"kind": "native", "scope": "test"}, "expect": True},
 {"id": "production-contract", "execution": {"kind": "native", "scope": "production"}, "expect": False},
 {"id": "production-contract-with-preparation-override", "execution": {"kind": "native", "scope": "production", "preparation": "none"}, "expect": False},
 {"id": "test-contract-with-preparation-override", "execution": {"kind": "native", "scope": "test", "preparation": "production"}, "expect": True}]
corpus = corpus[:i] + '  "nativeRunsTests": ' + json.dumps(rows, indent=2, ensure_ascii=False).replace("\n", "\n  ") + ",\n" + corpus[k:]
a, b = between(corpus, '"artifact": {', '"expect": {')
region = re.sub(r'("command": "bun \./📜️script\.ts [\w-]+"\n        \})\n      \}', r'\1,\n        "metadata": {\n          "semio": {\n            "execution": { "version": 1, "kind": "source" }\n          }\n        }\n      }', corpus[a:b])
if region.count('"execution": { "version": 1, "kind": "source" }') != 4: sys.exit("refused: artifact targets drifted")
corpus = corpus[:a] + region + corpus[b:]
json.loads(corpus); out["🔣️.json"] = corpus
schema = rd("🧬️schema/🔣️.json")
i, k = between(schema, '    "nativeDrivers": {', '    "version": {'); schema = schema[:i] + schema[k:]
schema = sub(schema, '    "nativeRunsTests",\n    "nativeDrivers",\n', '    "nativeRunsTests",\n')
i, k = between(schema, '    "nativeRunsTests": {', '    "nativeInputs": {')
schema = schema[:i] + '''    "nativeRunsTests": {
      "type": "array",
      "minItems": 1,
      "items": {
        "type": "object",
        "additionalProperties": false,
        "required": ["id", "execution", "expect"],
        "properties": {
          "id": { "type": "string" },
          "execution": {
            "type": "object",
            "additionalProperties": false,
            "required": ["kind", "scope"],
            "properties": {
              "kind": { "const": "native" },
              "scope": { "enum": ["production", "test"] },
              "preparation": { "enum": ["none", "production", "test"] }
            }
          },
          "expect": { "type": "boolean" }
        }
      }
    },
''' + schema[k:]
json.loads(schema); out["🧬️schema/🔣️.json"] = schema
test = rd("🟦️.ts")
test = sub(test, 'test("native targets run tests by contract, by name or through the Cargo test driver", () => {\n  for (const row of corpus.nativeRunsTests) expect(api.nativeTargetRunsTestsV1(row.name, row.execution ?? undefined, row.commandSources), row.id).toBe(row.expect);\n});', 'test("native targets run tests exactly when their declared scope says so", () => {\n  for (const row of corpus.nativeRunsTests) expect(api.nativeTargetRunsTestsV1(row.execution), row.id).toBe(row.expect);\n});')
a, b = between(test, 'test("a command closure is native only when it reaches a declared native driver"', 'test("generated shared command inputs never count')
out["🟦️.ts"] = test[:a] + test[b:]
mode = "overlay" if "--overlay" in sys.argv else "write" if "--write" in sys.argv else "dry-run"
for rel, text in out.items():
    if mode == "write": open(os.path.join(REPO, D + rel), "w", encoding="utf8").write(text)
    elif mode == "overlay":
        dest = os.path.join(sys.argv[sys.argv.index("--overlay") + 1], D + rel); os.makedirs(os.path.dirname(dest), exist_ok=True); open(dest, "w", encoding="utf8").write(text)
print(mode, "ok", list(out))
