#!/usr/bin/env python3
"""A revision reader whose own files were edited while it ran must say so: the self-check treats a size-bound refusal of its own implementation or schema as a stale reader. Anchored, dry run by default; adds the stale-reader law test.

usage: revision-fix.py [--root REPO_ROOT] [--write]
WARNING: the revision module is part of every graph module authority: a running graph build fails its authority check and restarts when this is written."""
import os, sys
HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
if "--root" in sys.argv:
    ROOT = os.path.abspath(sys.argv[sys.argv.index("--root") + 1])
E = lambda c: chr(c) + "️"
REVISION = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library", E(0x1F3D7) + "build", E(0x1F4E5) + "inputs", E(0x1F50F) + "revision")
MODULE = os.path.join(REVISION, E(0x1F7E8) + ".mjs")
TEST = os.path.join(REVISION, E(0x1F9EA) + "tests", E(0x1F9D3) + "stale-reader", E(0x1F7E6) + ".ts")
OLD = 'function current(){if(implementationPinnedV1())return;if(capture(ownPath,loaded.byteLength,false).sha256!==loaded.sha256||loadedSchema&&capture(ownSchemaPath,loadedSchema.byteLength,false).sha256!==loadedSchema.sha256)throw Error("Stale loaded Source revision reader; restart required");}'
NEW = 'function unchanged(path,row){try{return capture(path,row.byteLength,false).sha256===row.sha256;}catch(error){if(error?.message==="Bounded physical Source revision required"||error?.code==="ENOENT")return false;throw error;}}\nfunction current(){if(implementationPinnedV1())return;if(!unchanged(ownPath,loaded)||loadedSchema&&!unchanged(ownSchemaPath,loadedSchema))throw Error("Stale loaded Source revision reader; restart required");}'
text = open(os.path.join(ROOT, MODULE), encoding="utf8").read()
state = "applied" if NEW in text else "pending" if text.count(OLD) == 1 else None
if state is None:
    raise SystemExit("DRIFT: the self-check line moved")
test = open(os.path.join(HERE, "staged", "new", "revision", "test.ts"), encoding="utf8").read()
tpath = os.path.join(ROOT, TEST)
tstate = "applied" if os.path.exists(tpath) and open(tpath, encoding="utf8").read() == test else "pending"
print(state, MODULE[-40:]); print(tstate, TEST[-60:])
if "--write" in sys.argv:
    if state == "pending":
        open(os.path.join(ROOT, MODULE), "w", encoding="utf8", newline="").write(text.replace(OLD, NEW))
    if tstate == "pending":
        os.makedirs(os.path.dirname(tpath), exist_ok=True)
        open(tpath, "w", encoding="utf8", newline="").write(test)
    print("WRITTEN")
