#!/usr/bin/env python3
"""🐍️ Independent oracle of the row table: rows are equal exactly when kind, path and the text of the fact are equal (a primitive fact by its type and value, a flat file fact by its stamp and digest, any other object by its compact JSON text with the key order kept). Prints, per case, the number of the first row that equals each row (first occurrence order), which is what the table's numbers must say.

usage: 🐍️.py < {"cases":[...]}   prints {"cases":[{"rows":[...],"groups":[n,...]},...]}   (generated cases are expanded, so the test replays the very rows the model saw)"""
import json, sys


def fact_key(fact):
    if fact is None:
        return "objectnull"
    if isinstance(fact, bool):
        return "boolean" + ("true" if fact else "false")
    if isinstance(fact, (int, float)):
        return "number" + (str(int(fact)) if float(fact).is_integer() else repr(fact))
    if isinstance(fact, str):
        return "string" + fact
    if isinstance(fact.get("sha256"), str) and "provider" not in fact and isinstance(fact.get("size"), (int, float)) and not isinstance(fact.get("size"), bool):
        return "f" + ",".join(str(fact.get(name)) for name in ("size", "mtimeMs", "ctimeMs", "ino", "sha256"))
    return json.dumps(fact, separators=(",", ":"), ensure_ascii=False)


def generate(spec):
    state = spec["seed"]

    def draw(limit):
        nonlocal state
        state = (state * 1103515245 + 12345) % (2 ** 31)
        return (state >> 8) % limit

    rows = []
    for _ in range(spec["rows"]):
        kind = spec["kinds"][draw(len(spec["kinds"]))]
        path = "/w/" + "d" * (draw(40) + 100) + str(draw(spec["paths"]))
        rows.append([kind, path, draw(3) == 0 if kind == "exists" else "v" + str(draw(4))])
    return rows


def main():
    cases = json.load(sys.stdin)["cases"]
    out = []
    for case in cases:
        rows = case["rows"] if "rows" in case else generate(case["generate"])
        seen, groups = {}, []
        for kind, path, fact in rows:
            key = (kind, path, fact_key(fact))
            seen.setdefault(key, len(seen) + 1)
            groups.append(seen[key])
        out.append({"rows": rows, "groups": groups})
    print(json.dumps({"cases": out}, ensure_ascii=False))


main()
