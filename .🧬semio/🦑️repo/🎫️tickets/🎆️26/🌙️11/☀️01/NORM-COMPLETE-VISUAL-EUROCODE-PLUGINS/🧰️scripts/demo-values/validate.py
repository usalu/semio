"""Replicates the design-case `issues` check (unknown input, wrong kind, not a choice, out of range) on the committed demo document of a code. Run with python3 -I validate.py <code>."""
import re, sys
sys.path.insert(0, "."); from dv_lib import *
code = sys.argv[1]
v = vocab(code)
doc = json.loads(demo_doc_path(code).read_text(encoding="utf-8"))
bad = 0
def find(entry, key):
    r = spec(v, entry, key)
    if r is None:
        m = re.fullmatch(r"(.+?)\.\d+", key) or re.fullmatch(r"(.+?)\d+", key)
        if m:
            r = spec(v, entry, m.group(1)) or spec(v, entry, m.group(1) + "1")
    return r
for c in doc["cases"]:
    for n in c["numbers"]:
        r = find(c["entryId"], n["id"])
        if r is None: print(c["entryId"], "UNKNOWN", n["id"]); bad += 1
        elif r["kind"] in ("choice", "text"): print(c["entryId"], "WRONGKIND number for", n["id"], r["kind"]); bad += 1
        elif "min" in r and not (r["min"] <= n["value"] <= r["max"]): print(c["entryId"], "RANGE", n["id"], n["value"], r["min"], r["max"]); bad += 1
    for t in c["texts"]:
        r = find(c["entryId"], t["id"])
        if r is None: print(c["entryId"], "UNKNOWN", t["id"]); bad += 1
        elif r["kind"] in ("number", "vector", "series"): print(c["entryId"], "WRONGKIND text for", t["id"], r["kind"]); bad += 1
        elif r["kind"] == "choice" and t["value"] not in [x["value"] for x in r.get("choices", [])]: print(c["entryId"], "NOTCHOICE", t["id"], t["value"]); bad += 1
print(code, "issues:", bad)
