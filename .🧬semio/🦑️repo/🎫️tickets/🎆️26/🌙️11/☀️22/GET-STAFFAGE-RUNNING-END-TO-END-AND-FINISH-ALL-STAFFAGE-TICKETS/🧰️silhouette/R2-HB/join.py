"""🧵️ Joins the parts of the ops of a person (`ops/<slug>.<part>.json`, in the order given) into `ops/<slug>.json`: `python3 join.py <slug> <part>…`"""
import json
import os
import sys

import lib

slug, parts = sys.argv[1], sys.argv[2:]
ops = []
for part in parts:
    ops += json.load(open(os.path.join(lib.HERE, "ops", f"{slug}.{part}.json"), encoding="utf8"))
lib.write_ops(slug, ops)
