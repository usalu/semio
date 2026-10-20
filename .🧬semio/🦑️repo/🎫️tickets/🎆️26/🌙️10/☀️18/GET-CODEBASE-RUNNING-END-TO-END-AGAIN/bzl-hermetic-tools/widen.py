#!/usr/bin/env python3
"""Widens the hermetic class table from probe results (AFTER GO): an `ok` row admits its project to the class that selects the target and marks the class `proven`; a `fail:<reason>` row adds a declared exception; `plain` rows change nothing.
usage: widen.py <results.tsv> [--write]   (dry run prints the table diff summary)"""
import json, os, re, sys
sys.dont_write_bytecode = True
HERE = os.path.dirname(os.path.abspath(__file__))
sys.path.insert(0, HERE)
from targets import NEW
ROOT = "/Users/ueli/Documents/semio"
TABLE = os.path.join(ROOT, NEW["classes-table.json"])


def rx(pattern):
    out = ""
    i = 0
    while i < len(pattern):
        c = pattern[i]
        if c == "*":
            if pattern[i + 1:i + 2] == "*":
                out += ".*"
                i += 1
            else:
                out += "[^/]*"
        else:
            out += re.escape(c)
        i += 1
    return re.compile("^" + out + "$")


def main():
    table = json.load(open(TABLE, encoding="utf8"))
    rows = [line.rstrip("\n").split("\t") for line in open(sys.argv[1], encoding="utf8") if line.strip()]
    for project, target, status, *_ in rows:
        klass = next((c for c in table["classes"] if any(rx(p).match(target) for p in c["select"]["targets"])), None)
        if klass is None:
            continue
        if status == "ok":
            if project not in klass["select"]["projects"]:
                klass["select"]["projects"].append(project)
            klass["evidence"] = "proven"
        elif status.startswith("fail:"):
            entry = {"project": project, "target": target, "reason": status[5:]}
            if entry not in table["exceptions"]:
                table["exceptions"].append(entry)
    for klass in table["classes"]:
        klass["select"]["projects"] = sorted(set(klass["select"]["projects"]))
    table["exceptions"].sort(key=lambda e: (e["project"], e["target"]))
    text = json.dumps(table, ensure_ascii=False, indent=2) + "\n"
    if "--write" in sys.argv:
        open(TABLE, "w", encoding="utf8").write(text)
        print("table written")
    else:
        print({c["id"]: (c["evidence"], len(c["select"]["projects"])) for c in table["classes"]}, "exceptions", len(table["exceptions"]))


main()
