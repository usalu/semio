"""🧭️ For every editor-layer vocabulary in `editor-survey-*.json`: the crate-rooted module path of its aggregate and of its
snapshot type, and whether each is publicly reachable (T5's `module_files` walker from `wp-t5/bridges.py`).

Usage: editor-paths.py <editor-survey.json>"""
import json, os, re, sys
root = "/Users/ueli/Documents/semio/"
source = open(root + ".tmp-ticket/wp-t5/bridges.py", encoding="utf-8").read()
exec(source[source.index("def module_files"):source.index("def leaf_kinds")])
survey = json.load(open(sys.argv[1]))
libs = {}
def crate_lib(owner):
    d = root + owner
    while d.startswith(root):
        m = os.path.join(d, "📦️packages/🦀️rust/Cargo.toml")
        if os.path.exists(m):
            t = open(m).read()
            lib = re.search(r'(?ms)^\[lib\].*?^path\s*=\s*"([^"]+)"', t)
            return re.search(r'(?m)^name\s*=\s*"([^"]+)"', t).group(1).replace("-", "_"), os.path.normpath(os.path.join(d, "📦️packages/🦀️rust", lib.group(1) if lib else "src/lib.rs"))
        d = os.path.dirname(d)
out = []
for entry in survey:
    if not entry["aggregate"]: continue
    crate, lib = crate_lib(entry["owner"])
    files = libs.setdefault(lib, module_files(lib))
    vocab = os.path.normpath(root + entry["owner"] + "/🧬️schema/🧬️mutations/🦀️.rs")
    agg = files.get(vocab)
    snap_name = entry["snapshot"].split("::")[-1]
    snaps = [(mp, pub, p) for p, (mp, pub) in files.items() if p.startswith(os.path.normpath(root + entry["owner"])) and re.search(rf"(?m)^pub struct {snap_name}\b", open(p, encoding="utf-8").read())]
    out.append({**entry, "crate_ident": crate, "aggregate_path": "::".join([crate] + agg[0] + [entry["aggregate"]]) if agg else None, "aggregate_public": agg[1] if agg else None,
                "snapshot_path": "::".join([crate] + snaps[0][0] + [snap_name]) if snaps else None, "snapshot_public": snaps[0][1] if snaps else None})
json.dump(out, sys.stdout, indent=1, ensure_ascii=False)
