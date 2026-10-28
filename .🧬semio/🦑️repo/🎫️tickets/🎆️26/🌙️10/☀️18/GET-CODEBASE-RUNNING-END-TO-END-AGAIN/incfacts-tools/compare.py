"""⚖️ [incfacts] Compares two replay outputs node for node and the library file facts their rows hold; for a node pair that differs in `SEMIO_SOURCE_CLOSURE_<key>` tokens it prints the file-list delta of the two closure records (published in the graph store as {version, files}). usage: python3 -I compare.py <base.json> <new.json> <closures dir>"""
import json, re, sys
base, new, closures = json.load(open(sys.argv[1], encoding="utf8")), json.load(open(sys.argv[2], encoding="utf8")), sys.argv[3]
TOKEN = re.compile(r"SEMIO_SOURCE_CLOSURE_([0-9a-f]{64})")
files = lambda key: json.load(open(f"{closures}/{key}.json", encoding="utf8"))["files"]
assert base.keys() == new.keys(), "different project sets"
differ, deltas = [], {}
for project in base:
    a, b = base[project]["node"], new[project]["node"]
    if a == b: continue
    differ.append(project)
    for x, y in zip(TOKEN.findall(a), TOKEN.findall(b)):
        if x != y:
            fx, fy = files(x), files(y)
            for path in set(fy) - set(fx): deltas[("+", path)] = deltas.get(("+", path), 0) + 1
            for path in set(fx) - set(fy): deltas[("-", path)] = deltas.get(("-", path), 0) + 1
facts = lambda rows: sum(1 for row in rows.values() if row["libraryFacts"])
print(json.dumps({"projects": len(base), "refused": sum(1 for row in base.values() if row["node"].startswith("refused")), "nodes_identical": len(base) - len(differ), "nodes_differ": differ[:8],
                  "closure_deltas": {f"{sign} {path}": count for (sign, path), count in deltas.items()},
                  "projects_with_library_facts": {"base": facts(base), "new": facts(new)}}, ensure_ascii=False, indent=1))
