#!/usr/bin/env python3
"""🔍️ Interactive item-level queries over the E1 owner graph, loaded once (wave E1b).

Builds `🪜️layer-closure.py`'s item graph (impls folded into their self type) plus each item's target crate from
`🎯️target-crates.json`, pickles it next to the owners JSON, and answers several queries per run.
Usage: item-graph.py OWNERS_JSON users ITEM [ITEM ...] | refs ITEM [ITEM ...] | crate ITEM [ITEM ...]
"""
import importlib.util, json, os, pickle, re, sys
from collections import defaultdict

E1 = os.path.join(os.path.dirname(os.path.dirname(os.path.abspath(__file__))), "e1-os-kernel")
spec = importlib.util.spec_from_file_location("lc", os.path.join(E1, "🪜️layer-closure.py"))
lc = importlib.util.module_from_spec(spec); spec.loader.exec_module(lc)


def load(owners_path):
    cache = owners_path + ".items.pickle"
    if os.path.exists(cache) and os.path.getmtime(cache) > os.path.getmtime(owners_path):
        return pickle.load(open(cache, "rb"))
    owners, modinfo, node_of, facet_of, refs, item_node, item_lines = lc.build(owners_path)
    crates = json.load(open(os.path.join(E1, "🎯️target-crates.json")))["crates"]
    crate_of = {}
    for k, n in item_node.items():
        hits = [c["name"] for c in crates if any(re.search(r, n) for r in c["nodes"])]
        crate_of[k] = hits[0] if hits else "?"
    data = (dict(refs), item_node, dict(item_lines), crate_of)
    pickle.dump(data, open(cache, "wb"))
    return data


def main():
    refs, item_node, item_lines, crate_of = load(sys.argv[1])
    cmd, items = sys.argv[2], sys.argv[3:]
    short = lambda k: crate_of.get(k, "?").replace("semio-framework-os-", "")
    for it in items:
        hit = [k for k in item_node if k == it or k.endswith("::" + it)]
        for k in hit:
            print(f"== {k} [{short(k)}] ~{item_lines.get(k, 0)} lines")
            if cmd == "users":
                for u, ts in sorted(refs.items()):
                    if k in ts: print(f"   <- {u} [{short(u)}]")
            elif cmd == "refs":
                for t in sorted(refs.get(k, ())): print(f"   -> {t} [{short(t)}]")


if __name__ == "__main__":
    main()
