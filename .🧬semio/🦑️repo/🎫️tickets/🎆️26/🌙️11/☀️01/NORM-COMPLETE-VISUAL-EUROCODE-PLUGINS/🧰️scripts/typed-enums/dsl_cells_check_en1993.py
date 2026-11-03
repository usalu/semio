#!/usr/bin/env python3
"""🧪 Typed enums v6: dry run of the en1993 DSL assets: every enum cell is the kebab tag of a schema variant and equals the Pascal wire token of the sibling `snapshot.json`.
usage: dsl_cells_check_en1993.py"""
import glob, json, os, re, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import convert as C
spec = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "specs", "en1993.json"), encoding="utf-8"))
kebab = lambda n: "-".join(w.lower() for w in re.findall(r"[A-Z][a-z0-9]*|[a-z0-9]+", n))
enums = {c["enum"]: [v[0] for v in c["variants"]] for c in spec["conversions"]}
col = {}
for c in spec["conversions"]:
    for h in c["hosts"]:
        col[(h.split(".")[0], C.camel(h.split(".")[1]))] = c["enum"]
TOK = re.compile(r'"(?:[^"\\]|\\.)*"|\[[^\]]*\]|\{[^}]*\}|\S+')
bad = checked = 0
for d in glob.glob(C.artifact_root("en1993") + "/🖼️assets/*/"):
    snap = json.load(open(d + "snapshot.json", encoding="utf-8"))
    dsl = open(glob.glob(d + "*/*.dsl.semio")[0], encoding="utf-8").read().split("\n")
    head = None
    rows = {}
    for line in dsl:
        m = re.match(r"^(\S+) \[(.*)\] \{$", line)
        if m:
            head = (m.group(1), [x.split(":")[0] for x in m.group(2).split(" ")])
            rows[head[0]] = []
        elif head and line.startswith("  ") and not line.startswith("  ["):
            t = [x.group(0).strip('"') for x in TOK.finditer(line)]
            if len(t) == len(head[1]):
                rows[head[0]].append(dict(zip(head[1], t)))
    for table, rs in rows.items():
        key = re.sub(r"-(\w)", lambda m: m.group(1).upper(), table)
        for i, r in enumerate(rs):
            srow = (snap.get(key) or [])
            if i >= len(srow):
                continue
            for cname, val in r.items():
                k2 = re.sub(r"-(\w)", lambda m: m.group(1).upper(), cname)
                if not isinstance(srow[i].get(k2), str):
                    continue
                for (struct, f), e in col.items():
                    if f == k2 and srow[i][k2] in enums[e]:
                        checked += 1
                        if kebab(srow[i][k2]) != val:
                            bad += 1
                            print("MISMATCH", d, table, i, cname, val, srow[i][k2])
                        break
print(f"checked {checked} enum cells, {bad} mismatches")
sys.exit(1 if bad else 0)
