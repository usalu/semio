"""[walk-g] oracle for the project values of two harness runs (before/after the derived walks): the projects must be equal field by field; streams the two JSON documents, reports the first differences. usage: cmp-projects.py <before.json> <after.json>"""
import json
import sys


def walk(a, b, path, out, limit=12):
    if len(out) >= limit:
        return
    if type(a) != type(b):
        out.append((path, "type", str(type(a)), str(type(b))))
    elif isinstance(a, dict):
        for key in sorted(set(a) | set(b)):
            if key not in a:
                out.append((path + "/" + key, "only after", "", ""))
            elif key not in b:
                out.append((path + "/" + key, "only before", "", ""))
            else:
                walk(a[key], b[key], path + "/" + key, out, limit)
    elif isinstance(a, list):
        if len(a) != len(b):
            out.append((path, "length", len(a), len(b)))
        for index, (x, y) in enumerate(zip(a, b)):
            walk(x, y, path + "[%d]" % index, out, limit)
    elif a != b:
        out.append((path, "value", str(a)[:80], str(b)[:80]))


before = json.load(open(sys.argv[1], encoding="utf8"))
after = json.load(open(sys.argv[2], encoding="utf8"))
differences = []
walk(before, after, "", differences)
print("projects", sorted(before), "equal" if not differences else "DIFFERENT")
for row in differences:
    print(row)
sys.exit(1 if differences else 0)
