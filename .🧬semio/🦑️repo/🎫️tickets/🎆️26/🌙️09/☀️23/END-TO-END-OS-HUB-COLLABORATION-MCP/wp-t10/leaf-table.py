"""📋️ Builds wp-t10/leaf-outcomes.json (leaf → protocol outcome classes) from the reviewed evidence plus the hand
overrides in wp-t10/leaf-overrides.json, and prints how the table differs from each leaf's current declaration."""
import json, collections
root = "/Users/ueli/Documents/semio/.tmp-ticket/wp-t10/"
ev = json.load(open(root + "generated/leaf-evidence.json"))
over = json.load(open(root + "leaf-overrides.json"))
table, delta, unresolved = {}, collections.Counter(), []
NAIVE = {"applied": "applied", "info": "applied", "warning": "applied", "error": "rejected", "fatal": "rejected"}
keyed = {(k, a) for c, k, _, a in json.load(open(root + "generated/stdio-review.json")) if c in ("keyed", "keyed-insert")}
for rel, r in sorted(ev.items()):
    classes = over.get(rel, {}).get("classes") or r["evidence"]
    if "🗄️stdio" in rel and "🗿️artifacts" in rel and (r["kind"], rel.split("/🗿️artifacts/")[1].split("/")[0]) in keyed and classes:
        classes = sorted(set(classes) | {"rejected"})
    if not classes: unresolved.append(rel); continue
    table[rel] = [c for c in ["applied", "no-op", "empty", "disjoint", "rejected"] if c in classes]
    old = sorted({NAIVE.get(c, c) for c in r["declared"]})
    delta[("same" if old == sorted(table[rel]) else "changed", "/".join(rel.split("/")[:3]))] += 1
json.dump(table, open(root + "leaf-outcomes.json", "w"), ensure_ascii=False, indent=1)
print(len(table), "leaves;", sum(v for k, v in delta.items() if k[0] == "changed"), "differ from the old info→applied projection")
for k, v in sorted(delta.items()):
    if k[0] == "changed": print(f"  {v:4} {k[1]}")
print("unresolved:", unresolved)
