#!/usr/bin/env python3
"""🧮️ A file lives in exactly one batch: removes from every emoji batch the files the lang batch owns (the lang batch runs the emoji pass on them): python3 -I dedupe_batches.py"""
import json, os
HERE = os.path.dirname(os.path.abspath(__file__))
lang = json.load(open(os.path.join(HERE, "batches", "lang.json"), encoding="utf8"))
owned = set(lang["files"])
for name in ("fwprocess", "fwmodules", "libother", "libtools", "libbuild", "libgraph"):
    path = os.path.join(HERE, "batches", name + ".json")
    spec = json.load(open(path, encoding="utf8"))
    for key in ("files", "emoji"): spec[key] = [f for f in spec[key] if f not in owned]
    spec["manual"] = [m for m in spec["manual"] if m["rel"] not in owned]
    json.dump(spec, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
