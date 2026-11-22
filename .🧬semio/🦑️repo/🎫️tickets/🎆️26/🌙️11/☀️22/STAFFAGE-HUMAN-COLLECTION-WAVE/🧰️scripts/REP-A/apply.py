"""🩹️ Last stage of the generator chain of a person of HW-01 … HW-09: applies the authored repair ops `ops/<slug>*.json` (sorted by
name) to the generator output and writes the catalog file. The ops are design inputs: a drawn shape replaced, added, changed or
removed (`shape`, `shape-add`, `shape-set`, `shape-delete` with `scale`, `view`), a volume solid changed, added or removed
(`solid-set`, `solid-add`, `solid-delete` with `scale`, `list`) and a value set by path (`set`, path segments, `id=<x>` picks a
list item by id). The files are staged by the number after the slug (`<slug>.<stage>-<name>.json`); `--upto n` applies the stages below n.
python3 apply.py <slug> <generator-output.json> <catalog-file.json> [--upto n]"""
import glob, json, os, sys


def pick(node, seg):
    if isinstance(node, list):
        return next(i for i, x in enumerate(node) if isinstance(x, dict) and x.get("id") == seg[3:]) if seg.startswith("id=") else int(seg)
    return seg


def apply(a, o):
    kind = o["op"]
    if kind == "set":
        node = a
        for seg in o["path"][:-1]:
            node = node[pick(node, seg)]
        node[pick(node, o["path"][-1])] = o["value"]
    elif kind.startswith("shape"):
        g = a["representations"][o["scale"]]["geometry"][o["view"]]
        if kind == "shape-add":
            g.insert(next(i for i, x in enumerate(g) if x["id"] == o["after"]) + 1 if o.get("after") else len(g), o["shape"])
        elif kind == "shape":
            g[next(i for i, x in enumerate(g) if x["id"] == o["id"])] = o["shape"]
        elif kind == "shape-set":
            next(x for x in g if x["id"] == o["id"]).update(o["set"])
        elif kind == "shape-delete":
            n = len(g)
            g[:] = [x for x in g if x["id"] != o["id"]]
            assert len(g) < n, o
    elif kind.startswith("solid"):
        l = a["volumes"][o["scale"]][o["list"]]
        if kind == "solid-add":
            l.insert(next(i for i, x in enumerate(l) if x["id"] == o["after"]) + 1 if o.get("after") else len(l), o["solid"])
        elif kind == "solid-set":
            s = next(x for x in l if x["id"] == o["id"])
            for k, v in o["set"].items():
                if v is None:
                    s.pop(k, None)
                else:
                    s[k] = v
        elif kind == "solid-delete":
            n = len(l)
            l[:] = [x for x in l if x["id"] != o["id"]]
            assert len(l) < n, o
    else:
        raise ValueError(kind)


if __name__ == "__main__":
    slug, src, out = sys.argv[1:4]
    doc = json.load(open(src, encoding="utf-8"))
    upto = int(sys.argv[sys.argv.index("--upto") + 1]) if "--upto" in sys.argv else 10 ** 9
    files = [f for f in sorted(glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "ops", f"{slug}.*.json"))) if int(os.path.basename(f)[len(slug) + 1:].split("-")[0]) < upto]
    n = 0
    for f in files:
        for o in json.load(open(f, encoding="utf-8")):
            apply(doc["asset"], o)
            n += 1
    with open(out, "w", encoding="utf-8") as f:
        f.write(json.dumps(doc, indent=1, ensure_ascii=False) + "\n")
    print(f"{slug}: {n} ops of {len(files)} files")
