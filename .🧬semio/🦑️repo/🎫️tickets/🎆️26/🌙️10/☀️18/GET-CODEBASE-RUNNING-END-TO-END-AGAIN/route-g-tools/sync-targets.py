"""Adds a live `test-<id>` target to the `repo` project for every suite of the build route table that has none; removes none."""
import json
L = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
p = L + "/⚡️caching/📋️project.json"
d = json.load(open(p, encoding="utf8"))
ids = [s["id"] for s in json.load(open(L + "/🏗️build/🗂️suites/🔣️.json", encoding="utf8"))["suites"]]
cwd = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites"
lines = []
for sid in ids:
    name = "test-" + sid
    if name in d["targets"]:
        print("EXISTS", name); continue
    target = {"executor": "repo:script", "cache": False, "dependsOn": [], "inputs": ["{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/**/*"], "outputs": [], "options": {"cwd": cwd, "command": "repo ./📜️script.ts test " + sid, "forwardAllArgs": False}, "metadata": {"semio": {"execution": {"version": 1, "kind": "source"}, "cache": {"version": 1, "class": "live", "reason": "exercises-host-resources"}}}}
    lines.append("    " + json.dumps(name) + ": " + json.dumps(target, ensure_ascii=False, separators=(",", ":")))
text = open(p, encoding="utf8").read().split("\n")
close = text.index("  },")
if lines:
    text[close - 1] = text[close - 1].rstrip(",") + ","
    text[close:close] = [l + ("," if i < len(lines) - 1 else "") for i, l in enumerate(lines)]
    open(p, "w", encoding="utf8").write("\n".join(text))
    json.load(open(p, encoding="utf8"))
print("added", len(lines))
