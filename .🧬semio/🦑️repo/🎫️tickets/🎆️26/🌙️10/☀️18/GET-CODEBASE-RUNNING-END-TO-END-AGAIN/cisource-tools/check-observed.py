#!/usr/bin/env python3
"""🔭 Independent oracle for the dashboard CI Source gate: the files the test OPENS (recorded by `trace-reads.ts`, a `node:fs` wrapper, over the tests that existed before the law; not by the law's own regular expressions) must all be declared inputs of `test-ci-source`, and must equal the set the law extracts statically.
usage: python3 -I check-observed.py MIRROR_ROOT TRACE_FILE"""
import json, os, re, sys
root, trace = os.path.realpath(sys.argv[1]), sys.argv[2]
OWNER = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust"
TEST = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🏗️ci/🟦️.ts"
target = json.load(open(os.path.join(root, OWNER, "📋️project.json"), encoding="utf8"))["targets"]["test-ci-source"]
def expand(item): return item.replace("{workspaceRoot}/", "").replace("{projectRoot}", OWNER)
declared = {expand(item) for item in target["inputs"]}
observed = set()
for line in open(trace, encoding="utf8"):
    kind, rel = line.rstrip("\n").split("\t", 1)
    if kind in ("readFileSync", "existsSync") and "node_modules" not in rel and os.path.isfile(os.path.join(root, rel)) and rel.endswith((".yml", ".yaml", ".json")): observed.add(rel)
source = open(os.path.join(root, TEST), encoding="utf8").read()
static = {m.group(1) for m in re.finditer(r'"(\.github/[\w./-]+\.ya?ml)"', source)} | {OWNER + "/" + m.group(1) for m in re.finditer(r'gate\.owner,"([^"/]+\.json)"', source)}
print("observed data files:", *sorted(observed), sep="\n  ")
bad = []
if observed - declared: bad.append(("read but not declared", sorted(observed - declared)))
if observed != static: bad.append(("observed != static law set", sorted(observed ^ static)))
if len(observed) < 5: bad.append(("too few observed reads", len(observed)))
for kind, value in bad: print("FAIL", kind, value)
sys.exit(1 if bad else 0)
