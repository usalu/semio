#!/usr/bin/env python3
"""oracle2-g: rewrites the staged read-oracle corpus (policy rows with their own tree and the exception cover, one split run) and its schema. The expected `readable` sets come from the policy in words (computed here), the `rules` are frozen from the staged policy module and checked by the law tests against the independent evaluators.
usage: gen-corpus.py <read-oracle dir>"""
import json, os, subprocess, sys
base = sys.argv[1]
here = os.path.dirname(os.path.abspath(__file__))
corpus_path = os.path.join(base, "🧪️tests/🔣️.json")
schema_path = os.path.join(base, "🧪️tests/🧬️schema/🔣️.json")
corpus = json.load(open(corpus_path, encoding="utf-8"))
AMBIENT = ["node_modules", ".cache", ".git", "target", ".🧬semio/🦑️repo/⚡️cache"]

BASE = list(corpus["tree"].keys()) if "many/m1.txt" not in corpus["tree"] else [k for k in corpus["tree"] if not k.startswith("many/")]
for i in range(1, 9):
    corpus["tree"][f"many/m{i}.txt"] = f"M{i}"
LIB = ["README.md", "docs/a.md", "docs/b.md", "scripts/s.sh", "tools/x.sh", "app/main.ts", "app/test/t1.ts", "app/test/t2.ts", "lib/a.ts", "lib/b.ts", "lib/c.ts", "lib/d.ts", "lib/e.ts", "lib/package.json", "lib/sub/f.ts", "lib/sub/g.ts", "lib/vendor/h.ts", "lib/vendor/i.ts", "lib/vendor/j.ts", "lib/vendor/k.ts", "node_modules/x/y.js"]
LIB_ALL_BUT_C = [p for p in LIB if p.startswith("lib/") and p != "lib/c.ts"]
rows = [
    ("single-file", BASE, ["a.txt"], [], [], True),
    ("whole-directory-collapses", BASE, ["dir/c.txt", "dir/d.txt"], [], [], True),
    ("partial-directory-stays-literal", BASE, ["dir/c.txt", "a.txt"], [], [], True),
    ("names-are-readable-anywhere", BASE, ["a.txt"], [], ["package.json"], True),
    ("extra-ambient-tree", BASE, [], ["other"], [], True),
    ("nested-complete-directory", BASE, ["pkg/package.json", "pkg/deep/f.txt"], [], [], True),
    ("big-tree-is-granted-and-its-exception-refused", LIB, LIB_ALL_BUT_C + ["lib/gone.ts"], [], [], True),
    ("grants-only-plans-list-every-partial-directory", LIB, LIB_ALL_BUT_C, [], [], False),
    ("refused-subtree-keeps-its-declared-file", LIB, [p for p in LIB_ALL_BUT_C if not p.startswith("lib/vendor/") or p == "lib/vendor/h.ts"], [], [], True),
    ("everything-but-one-file-grants-the-root", LIB, [p for p in LIB if p not in ("README.md",) and not p.startswith("node_modules/")], [], [], True),
    ("ambient-trees-and-names-need-no-rule", LIB, ["lib/a.ts", "node_modules/x/y.js"], [], ["package.json"], True),
    ("a-whole-tree-and-a-lone-file", LIB, ["app/main.ts"] + [p for p in LIB if p.startswith("lib/")], [], [], True),
]
policy = []
for id_, tree, allowed, ambient, names, refusals in rows:
    trees = AMBIENT + ambient
    readable = [p for p in tree if p in allowed or any(p == t or p.startswith(t + "/") for t in trees) or p.rsplit("/", 1)[-1] in names]
    policy.append({"id": id_, "tree": tree, "allowed": allowed, "ambient": ambient, "names": names, "refusals": refusals, "rules": [], "readable": readable})
done = subprocess.run(["bun", os.path.join(here, "rules-of.ts")], input=json.dumps(policy), capture_output=True, text=True)
if done.returncode:
    sys.exit(done.stderr)
rules = json.loads(done.stdout)
for row in policy:
    row["rules"] = rules[row["id"]]
corpus["policy"] = policy
runs = [r for r in corpus["runs"] if r["id"] != "split-profiles-learn-undeclared"]
runs.append({"id": "split-profiles-learn-undeclared", "allowed": ["many/m1.txt", "many/m3.txt", "many/m4.txt", "many/m6.txt", "many/m7.txt", "many/m8.txt"], "ambient": ["tools"], "names": [], "reads": ["many/m1.txt", "many/m2.txt", "many/m5.txt"], "mode": "report", "rounds": 4, "capacity": 2,
             "expect": {"seatbelt": {"clean": True, "passed": True, "learned": ["many/m2.txt", "many/m5.txt"], "firstDenied": ["many/m2.txt", "many/m5.txt"], "firstExit": 1},
                        "landlock": {"clean": True, "passed": True, "learned": ["many/m2.txt", "many/m5.txt"], "needsTrace": True},
                        "mirror": {"clean": True, "passed": True, "learned": ["many/m2.txt", "many/m5.txt"]}}})
corpus["runs"] = runs

def leaf(v):
    return not isinstance(v, (dict, list)) or (isinstance(v, list) and all(not isinstance(x, (dict, list)) for x in v)) or (isinstance(v, list) and all(isinstance(x, dict) and all(not isinstance(y, (dict, list)) for y in x.values()) for x in v))

def fmt(v, level=0):
    pad = "  " * level
    if leaf(v):
        return json.dumps(v, ensure_ascii=False)
    if isinstance(v, dict):
        return "{\n" + ",\n".join(f'{pad}  {json.dumps(k, ensure_ascii=False)}: {fmt(x, level + 1)}' for k, x in v.items()) + f"\n{pad}}}"
    return "[\n" + ",\n".join(f"{pad}  {fmt(x, level + 1)}" for x in v) + f"\n{pad}]"

open(corpus_path, "w", encoding="utf-8").write(fmt(corpus) + "\n")

schema = json.load(open(schema_path, encoding="utf-8"))
policy_item = schema["properties"]["policy"]["items"]
policy_item["required"] = ["id", "tree", "allowed", "ambient", "names", "refusals", "rules", "readable"]
policy_item["properties"] = {k: policy_item["properties"][k] for k in ["id"]} | {"tree": {"type": "array", "items": {"type": "string"}}} | {k: policy_item["properties"][k] for k in ["allowed", "ambient", "names"]} | {"refusals": {"type": "boolean"}} | {k: policy_item["properties"][k] for k in ["rules", "readable"]}
rule = policy_item["properties"]["rules"]["items"]
rule["required"] = ["effect", "kind", "path"]
rule["properties"] = {"effect": {"enum": ["allow", "deny"]}, **rule["properties"]}
schema["properties"]["runs"]["items"]["properties"]["capacity"] = {"type": "integer", "minimum": 1}
open(schema_path, "w", encoding="utf-8").write(json.dumps(schema, indent=2, ensure_ascii=False) + "\n")
print("policy rows:", len(policy), "runs:", len(runs))
for row in policy:
    print(row["id"], [f'{r["effect"]} {r["kind"]} {r["path"]}' for r in row["rules"]])
