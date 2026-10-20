#!/usr/bin/env python3
"""Builds staged/followup.json: the presentation-resource exclusion of the command closure (anchors extracted from the LIVE files; run again right before applying).

usage: followup_ops_gen.py [--root REPO_ROOT]     apply with: apply.py apply --spec followup --write"""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
if "--root" in sys.argv:
    ROOT = sys.argv[sys.argv.index("--root") + 1]
E = lambda c: chr(c) + "️"
L = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(L, E(0x1F3D7) + "build")
INF = os.path.join(BUILD, E(0x1F578) + "graph", E(0x1F4E5) + "inference")
SI = os.path.join(INF, E(0x1F9ED) + "script-inputs", E(0x1F7E8) + ".mjs")
POLICY = os.path.join(BUILD, E(0x1F523) + "policy.json")
TEST = os.path.join(INF, E(0x1F9EA) + "tests", E(0x1F648) + "presentation-resources")
NEW = {"test.ts": os.path.join(TEST, E(0x1F7E6) + ".ts"), "fixture.json": os.path.join(TEST, E(0x1F523) + ".json"), "fixture-schema.json": os.path.join(TEST, E(0x1F9EC) + "schema", E(0x1F523) + ".json"), "oracle.py": os.path.join(TEST, E(0x1F40D) + "oracle.py")}
PROGRESS = "/".join([E(0x1F9F0) + "framework", E(0x1F528) + "modules", E(0x1F3C3) + "process", E(0x1F9ED) + "routing", E(0x1F4E5) + "invocation", E(0x1F3C3) + "process", E(0x1F9EC) + "schema", E(0x1F4C8) + "progress.json"])
ops = [
    {"path": POLICY, "anchor": '  "defaultTool": "nx",\n',
     "replacement": '  "defaultTool": "nx",\n  "presentationResources": [\n    "' + PROGRESS + '"\n  ],\n'},
    {"path": SI, "anchor": "/** \U0001F527\ufe0f Native leaves hash their compiler contract and exact command implementation, independently of UI selection. */\n",
     "replacement": "/** \U0001F648\ufe0f The resources the build policy declares presentation-only (progress labels): their edits never change a command's behavior, so they are not command inputs. */\nfunction presentationResourcesV1(cache) {\n  return cache.presentation ??= new Set(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, \"\U0001F3D7\ufe0fbuild\", \"\U0001F523\ufe0fpolicy.json\")).toString(\"utf8\")).presentationResources);\n}\n\n/** \U0001F527\ufe0f Native leaves hash their compiler contract and exact command implementation, independently of UI selection. */\n"},
    {"path": SI, "anchor": "    const seen=files.has(path);\n    files.add(path);\n",
     "replacement": "    if(presentationResourcesV1(cache).has(buildPath(relative(workspaceRoot,path))))return;\n    const seen=files.has(path);\n    files.add(path);\n"},
]
texts = {}
for row in ops:
    texts.setdefault(row["path"], open(os.path.join(ROOT, row["path"]), encoding="utf8").read())
for row in ops:
    count = texts[row["path"]].count(row["anchor"])
    if count != 1 and row["replacement"] not in texts[row["path"]]:
        raise SystemExit("anchor occurs %d times in %s: %r" % (count, row["path"], row["anchor"][:70]))
bases = {path: hashlib.sha256(text.encode("utf8")).hexdigest() for path, text in texts.items()}
json.dump({"ops": ops, "bases": bases, "new": NEW}, open(os.path.join(HERE, "staged", "followup.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("followup ops:", len(ops), "new:", len(NEW))
