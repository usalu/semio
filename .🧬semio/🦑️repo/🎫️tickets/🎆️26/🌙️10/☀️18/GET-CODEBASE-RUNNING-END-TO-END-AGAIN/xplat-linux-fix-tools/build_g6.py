import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), "groups"))
from _gen import emit, live
HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
TABLE, CLASSES = B + "/🏃️execution/🔒️hermetic/🔣️.json", B + "/🏃️execution/🧬️schema/🔒️hermetic-classes.json"
UNIT, ADAPTER = B + "/🏃️execution/🔒️hermetic/🧪️tests/🧫️fixtures/🔣️.json", B + "/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/🧫️fixtures/🔣️.json"
edits = []
edits.append((TABLE, '''    "darwin": [
      "remote",
      "darwin-sandbox",
      "isolated-root"
    ],
    "linux": [
      "remote",
      "linux-sandbox",
      "processwrapper-sandbox",
      "isolated-root"
    ],''', '''    "darwin": [
      "remote",
      "sandboxed",
      "isolated-root"
    ],
    "linux": [
      "remote",
      "sandboxed",
      "isolated-root"
    ],'''))
edits.append((CLASSES, '''          "remote",
          "linux-sandbox",
          "darwin-sandbox",
          "processwrapper-sandbox",
          "isolated-root"''', '''          "remote",
          "sandboxed",
          "isolated-root"'''))
edits.append((UNIT, '''    {"platform": "darwin", "expected": ["remote", "darwin-sandbox", "local"]},
    {"platform": "linux", "expected": ["remote", "linux-sandbox", "processwrapper-sandbox", "local"]},''', '''    {"platform": "darwin", "expected": ["remote", "sandboxed", "local"]},
    {"platform": "linux", "expected": ["remote", "sandboxed", "local"]},'''))
text = live(ADAPTER)
for old, new in [('"--strategy=RepoHermetic=darwin-sandbox,local"', '"--strategy=RepoHermetic=sandboxed,local"'), ('"--strategy=RepoHermetic=remote,darwin-sandbox,local"', '"--strategy=RepoHermetic=remote,sandboxed,local"'), ('"--strategy=RepoHermetic=remote,linux-sandbox,processwrapper-sandbox,local"', '"--strategy=RepoHermetic=remote,sandboxed,local"')]:
    edits.append((ADAPTER, old, new, text.count(old)))
new_files = [B + "/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/📥️strategies/🟦️.ts", B + "/🔧️tools/🏗️bazel/🏃️execution/🧪️tests/📥️strategies/🧫️fixtures/🔣️.json"]
emit("g6", "hermetic spawn strategies name only strategies every Bazel host registers: sandboxed resolves to the best sandbox the host has", [], edits, new_files, os.path.join(HERE, "groups"))
print("g6 written")
