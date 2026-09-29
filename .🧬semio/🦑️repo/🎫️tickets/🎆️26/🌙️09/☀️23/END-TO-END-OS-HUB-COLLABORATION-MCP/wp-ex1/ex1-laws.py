#!/usr/bin/env python3
"""📚️ EX1: per non-stdio plugin — the example-catalog fixture rows (from `generated/editors.json`) and the editor/create-fn
paths the plugin's `🧪️tests/🔬️surface` law names them by. Writes `laws/<plugin-slug>.json`; unresolved paths are printed."""
import collections, json, os, re, sys
R = "/Users/ueli/Documents/semio/✏️s/🔌️plugins"
HERE = os.path.dirname(os.path.abspath(__file__))
editors = json.load(open(os.path.join(HERE, "generated", "editors.json"), encoding="utf-8"))
os.makedirs(os.path.join(HERE, "laws"), exist_ok=True)
by = collections.defaultdict(list)
for e in editors:
    by[e["plugin"]].append(e)
for plugin, rows in sorted(by.items()):
    sources = {}
    for dp, dns, fns in os.walk(os.path.join(R, plugin)):
        dns[:] = [d for d in dns if d not in ("target", "node_modules")]
        for fn in fns:
            if fn.endswith(".rs"):
                sources[os.path.join(dp, fn)] = open(os.path.join(dp, fn), encoding="utf-8").read()
    blob = "\n".join(sources.values())
    slug = re.sub(r"^[^a-z]+", "", plugin)
    out = {"plugin": slug, "dir": plugin, "apps": [], "tests": []}
    for e in rows:
        paths = collections.Counter(re.findall(rf"((?:crate|semio_s_[a-z0-9_]+)::editor::[a-z0-9_]+)::{e['editor']}\b", blob))
        module = paths.most_common(1)[0][0] if paths else None
        create = [c for c in e["create"] if module and re.search(rf"{re.escape(module)}::{c}\b", blob)] or e["create"]
        if module is None or not create or e["app"] is None:
            print(f"UNRESOLVED {plugin} {e['editor']} module={module} create={create} app={e['app']}", file=sys.stderr)
        ids = [x.lstrip("=~") for x in e["examples"] if not x.startswith("?")]
        out["apps"].append({"app": e["app"] or f"UNRESOLVED:{e['editor']}", "examples": [{"id": i, "document": "distinct"} for i in ids]})
        out["tests"].append({"editor": f"{module}::{e['editor']}" if module else e["editor"], "create": f"{module}::{create[0]}" if module and create else None, "app": e["app"]})
    json.dump(out, open(os.path.join(HERE, "laws", f"{slug}.json"), "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    print(slug, len(rows))
