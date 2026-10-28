"""python3 runtime-state.py [--check] : the command store (captures, leases, cargo target, vite cache of every `repo … command` run) leaves the source tree.
Found 10-26 09:20: `⚙️configuration/🏃️process/🔣️.json` named `../../🤖️generated/process`, i.e. `🧰️framework/🤖️generated/process`: every framework-rs verify target wrote a receipt JSON, stdout/stderr and lease databases INTO the owner tree of the framework umbrella, where the owner glob (`**/*.{json,…}`) hashed them: a test run changed the key of the next run of every framework-owner task (193 MB, 535 captures, 178 JSON files today).
Fix 1: the store moves to `.cache/command` (ignored, never observed, never hashed). Fix 2 (schema-first, for leftovers and old processes): policy `runtimeStateDirectories` negates the listed paths in every project's inputs."""
import json, sys
ROOT = "/Users/ueli/Documents/semio/"
L = ROOT + "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
FILES = {"config": ROOT + "🧰️framework/⚙️configuration/🏃️process/🔣️.json", "policy": L + "⚡️caching/🔣️policy.json", "schema": L + "⚡️caching/🧬️schema/🔣️.json", "inputs": L + "🏗️build/📥️inputs/🟨️.mjs", "inferTest": L + "🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🟦️.ts", "corpus": L + "🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🔣️.json", "corpusSchema": L + "🏗️build/📥️inputs/🌳️owner-view/🧪️tests/🧬️inference/🧬️schema/🔣️.json"}
text = {k: open(v, encoding="utf-8").read() for k, v in FILES.items()}
def sub(key, anchor, new, marker):
    if marker in text[key]: return
    if text[key].count(anchor) != 1: sys.exit(f"{key}: anchor not unique ({text[key].count(anchor)}): {anchor[:80]}")
    text[key] = text[key].replace(anchor, new)
sub("config", '"store": "../../🤖️generated/process",', '"store": "../../../.cache/command",', '"../../../.cache/command"')
policy = json.loads(text["policy"])
if "runtimeStateDirectories" not in policy:
    items = list(policy.items()); at = [k for k, _ in items].index("ownerViews")
    items.insert(at + 1, ("runtimeStateDirectories", ["🧰️framework/🤖️generated/process"])); policy = dict(items)
text["policy"] = json.dumps(policy, ensure_ascii=False, indent=2) + "\n"
sub("schema", '    "ownerViews": {', '''    "runtimeStateDirectories": {
      "description": "Workspace-relative directories that hold runtime state of earlier or older processes (captures, leases, build targets), never an input: every project negates them. They must be ignored by version control and never tracked (a law checks it with git); the command store itself lives in `.cache/command`.",
      "type": "array",
      "uniqueItems": true,
      "items": { "type": "string", "pattern": "^[^/].*[^/]$" }
    },
    "ownerViews": {''', '"runtimeStateDirectories"')
sub("inputs", "  for (const directory of POLICY.generatedDirectories) {\n    inputs.push(`!{projectRoot}/**/${directory}/**/*`);", "  for (const path of POLICY.runtimeStateDirectories ?? []) inputs.push(`!{workspaceRoot}/${path}/**/*`);\n  for (const directory of POLICY.generatedDirectories) {\n    inputs.push(`!{projectRoot}/**/${directory}/**/*`);", "POLICY.runtimeStateDirectories ??")
sub("inferTest", "POLICY: {generatedDirectories: [\"node_modules\", \".cache\"], toolchains: corpus.toolchains, ownerViews: corpus.views}", "POLICY: {generatedDirectories: [\"node_modules\", \".cache\"], toolchains: corpus.toolchains, ownerViews: corpus.views, runtimeStateDirectories: corpus.runtimeState}", "runtimeStateDirectories: corpus.runtimeState")
sub("inferTest", "    expect(result.ownerPrivate ?? null, row.id).toEqual(row.ownerPrivate);", "    expect(result.ownerPrivate ?? null, row.id).toEqual(row.ownerPrivate);\n    for (const path of corpus.runtimeState) expect((result.default ?? []).includes(`!{workspaceRoot}/${path}/**/*`), `${row.id} negates ${path}`).toBe(true);", "negates ${path}")
corpus = json.loads(text["corpus"]); corpus.setdefault("runtimeState", ["fw/🤖️generated/process"])
text["corpus"] = json.dumps(corpus, ensure_ascii=False, indent=2) + "\n"
cs = json.loads(text["corpusSchema"]); cs["required"] = sorted(set(cs["required"]) | {"runtimeState"}); cs["properties"].setdefault("runtimeState", {"type": "array", "items": {"type": "string"}})
text["corpusSchema"] = json.dumps(cs, ensure_ascii=False, indent=2) + "\n"
if "--check" in sys.argv: print("anchors ok"); sys.exit(0)
for k, v in FILES.items(): open(v, "w", encoding="utf-8").write(text[k])
print("applied", len(FILES))
