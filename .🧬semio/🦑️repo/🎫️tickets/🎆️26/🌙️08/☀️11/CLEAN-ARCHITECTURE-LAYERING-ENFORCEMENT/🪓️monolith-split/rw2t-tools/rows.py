"""📋 rw2t: list the oracle rows of a pos result by crate-module and law file with the unresolved names."""
import json, collections, re, sys
def load(path, cfg="test"):
    d = json.load(open(path))
    return d["summary"][cfg]["new_list"]
def pk_of(r):
    m = re.match(r"__m_semio_framework_os_renderer_wgpu_?([a-z0-9_]*)/", r["file"])
    return m.group(1) if m else "?"
if __name__ == "__main__":
    path, cfg = sys.argv[1], sys.argv[2]
    cats = sys.argv[3].split(",")
    pks = sys.argv[4].split(",") if len(sys.argv) > 4 else None
    L = load(path, cfg)
    g = collections.defaultdict(list)
    for r in L:
        if r["cat"] not in cats: continue
        pk = pk_of(r)
        if pks and pk not in pks: continue
        g[(pk, r["file"].split("/")[-2])].append(r)
    for k, rows in sorted(g.items()):
        print("==", k, len(rows))
        names = collections.Counter()
        for r in rows:
            names[(r["code"], r["msg"][:110])] += 1
        for (code, msg), n in names.most_common(60):
            print("   ", n, code, msg)
