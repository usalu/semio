#!/usr/bin/env python3
"""Builds staged/speed2.json: (1) fact-table blocks are compressed at brotli quality 5 and a quota refusal is remembered for 30 s instead of recompressing every block of every project at quality 11, (the producer publications stay mandatory by the optional-cache law; free their space with reclaim-publications.py).
RESTART REQUIRED: the observation module is part of the loaded authority; apply only between builds.

usage: speed2_ops_gen.py ; apply.py apply --spec speed2 --write"""
import hashlib, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
ROOT = os.path.abspath(os.path.join(HERE, *([".."] * 8)))
E = lambda c: chr(c) + "️"
L = os.path.join(E(0x1F9F0) + "framework", E(0x1F6CD) + "products", E(0x1F991) + "repo", E(0x1F528) + "modules", E(0x1F4DA) + "library")
BUILD = os.path.join(L, E(0x1F3D7) + "build")
OB = os.path.join(BUILD, E(0x1F441) + "observation", E(0x1F7E8) + ".mjs")
TEST = os.path.join(BUILD, E(0x1F441) + "observation", E(0x1F9EA) + "tests", E(0x1F9F1) + "block-cost")
NEW = {"test.ts": os.path.join(TEST, E(0x1F7E6) + ".ts"), "fixture.json": os.path.join(TEST, E(0x1F523) + ".json"), "fixture-schema.json": os.path.join(TEST, E(0x1F9EC) + "schema", E(0x1F523) + ".json")}
ops = [
    {"path": OB, "anchor": "let blockRowCounter=0;\n", "replacement": "let blockRowCounter=0;\nconst blocksRefused=new Map(),BLOCK_REFUSAL_MILLISECONDS=30000;\n"},
    {"path": OB, "anchor": "const path=inferenceObservationBlockPathV1(directory,known.digest);\n if(fs.existsSync(path)){blocksWritten.add(stamp);return true;}\n",
     "replacement": "const path=inferenceObservationBlockPathV1(directory,known.digest);\n if(fs.existsSync(path)){blocksWritten.add(stamp);return true;}\n if((blocksRefused.get(directory)??0)>Date.now())return false;\n"},
    {"path": OB, "anchor": "const packed=brotliCompressSync(Buffer.from(observationRowsJsonV1(block.rows))),temporary=",
     "replacement": "const packed=brotliCompressSync(Buffer.from(observationRowsJsonV1(block.rows)),{params:{[zlibConstants.BROTLI_PARAM_QUALITY]:5,[zlibConstants.BROTLI_PARAM_LGWIN]:22}}),temporary="},
    {"path": OB, "anchor": "fs.writeFileSync(temporary,packed,{flag:\"wx\"});fs.renameSync(temporary,path);}))return false;\n",
     "replacement": "fs.writeFileSync(temporary,packed,{flag:\"wx\"});fs.renameSync(temporary,path);})){blocksRefused.set(directory,Date.now()+BLOCK_REFUSAL_MILLISECONDS);return false;}\n"},
]
texts = {}
for row in ops:
    texts.setdefault(row["path"], open(os.path.join(ROOT, row["path"]), encoding="utf8").read())
for row in ops:
    count = texts[row["path"]].count(row["anchor"])
    if count != 1 and row["replacement"] not in texts[row["path"]]:
        raise SystemExit("anchor occurs %d times in %s: %r" % (count, row["path"], row["anchor"][:70]))
json.dump({"ops": ops, "bases": {p: hashlib.sha256(t.encode("utf8")).hexdigest() for p, t in texts.items()}, "new": NEW}, open(os.path.join(HERE, "staged", "speed2.json"), "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("speed2 ops:", len(ops), "new:", len(NEW))
