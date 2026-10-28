"""🔁 [backlog] 3-way rebase of gram3-g churn onto the live tree after pathmod rust-mounts (cargoSourceSetV1, unescapeRustString): rewrites edits 62, 70, 71 of staged/churn/edits.json (original kept as edits.json.pre-backlog). Idempotent. usage: rebase_churn.py [live-root]"""
import json, os, shutil, sys
C = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
R = sys.argv[1] if len(sys.argv) > 1 else os.path.abspath(os.path.join(C, *[".."] * 7))
D = C + "/gram3-g-tools/staged/churn/edits.json"
if not os.path.exists(D + ".pre-backlog"): shutil.copy(D, D + ".pre-backlog")
spec = json.load(open(D + ".pre-backlog", encoding="utf8"))
LIB = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/📥️inference/"
def live(rel): return open(R + "/" + LIB + rel, encoding="utf8").read()
byname = {e["name"]: e for e in spec["edits"]}
sa = live("🧬️source-analyses/🟨️.mjs")
lines = sa.split("\n"); i = lines.index("function rustInputTokens(source) {")
unesc, tokens = lines[i + 1], lines[i + 2]
assert unesc.startswith("  const unescapeRustString") and tokens.startswith("  const tokens = [], rawPattern = /(?:b|c)?r(#+)?\"/y")
e = byname["analyses rust patterns"]
consts = e["new"].split("\n\n")[0]
e["old"] = "\n".join(lines[i:i + 3])
e["new"] = consts + "\n\n" + "\n".join([lines[i], unesc, "  const tokens = [], rawPattern = RUST_RAW_PATTERN_V1, characterPattern = RUST_CHARACTER_PATTERN_V1, identifierPattern = RUST_IDENTIFIER_PATTERN_V1;"])
e = byname["analyses string literal"]
spec["edits"].remove(e)
nm = "🦀️native-manifests/🟨️.mjs"
e = byname["native source inputs memo"]
head = "export function cargoSourceSetV1(root, workspaceRoot, facts, includeTests = true, compiled = false) {\n  const directory = join(workspaceRoot, root), manifest = join(directory, \"Cargo.toml\"), unresolved = "
assert live(nm).count(head) == 1
e["old"] = head
e["new"] = ("export function cargoSourceSetV1(root, workspaceRoot, facts, includeTests = true, compiled = false) {\n"
  "  if (facts === null || typeof facts !== \"object\") return cargoSourceSetOfV1(root, workspaceRoot, facts, includeTests, compiled);\n"
  "  let memo = CARGO_SOURCE_SETS_V1.get(facts);\n"
  "  if (memo === undefined) CARGO_SOURCE_SETS_V1.set(facts, memo = new Map());\n"
  "  return memoizedObservedComputationV1(memo, JSON.stringify([root, workspaceRoot, includeTests, compiled]), () => cargoSourceSetOfV1(root, workspaceRoot, facts, includeTests, compiled));\n}\n\n"
  "/** 🧠️ The answers of {@link cargoSourceSetV1} per fact table (one per project): the targets of a project ask for the same crates again and again, and a replayed answer hands every asker the rows it rests on without probing the tree again. */\n"
  "const CARGO_SOURCE_SETS_V1 = new WeakMap();\n\n"
  "function cargoSourceSetOfV1(root, workspaceRoot, facts, includeTests, compiled) {\n  const directory = join(workspaceRoot, root), manifest = join(directory, \"Cargo.toml\"), unresolved = ")
spec["edits"].append({"file": e.get("file"), "name": "native source set copies", "old": "  return set.resolved ? set.files : undefined;\n", "new": "  return set.resolved ? [...set.files] : undefined;\n"}) if "file" in e else None
spec["edits"].append({"file": e.get("file"), "name": "native sources or refuse copies", "old": "  if (set.resolved) return set.files;\n", "new": "  if (set.resolved) return [...set.files];\n"}) if "file" in e else None
if "file" not in e:
    f = spec.get("file")
    raise SystemExit("edits carry no file key; set it: " + str(f))
json.dump(spec, open(D, "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("rebased: edits", len(spec["edits"]))
