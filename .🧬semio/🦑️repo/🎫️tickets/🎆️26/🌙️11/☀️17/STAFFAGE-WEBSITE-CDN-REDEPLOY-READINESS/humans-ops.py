"""Applies an ops file to a humans asset file by span edits. python3 humans-ops.py <slug> <ops.json>
ops: {"op":"shape","scale":"50","view":"plan","id":"waist","shape":{...}}   replaces the shape object (re-dumped at its indent, two-space style)
     {"op":"shape-set","scale","view","id","set":{k:v}}                       sets keys (replaces in place, appends when new)
     {"op":"shape-add","scale","view","after":id,"shape":{...}}               inserts a shape after another
     {"op":"solid","scale","id","solid":{...}} replaces a body solid ; {"op":"solid-set","scale","id","set":{k:v}} ; {"op":"solid-add","scale","after","solid":{...}}"""
import importlib.util, json, re, sys
spec = importlib.util.spec_from_file_location('s', '/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS/sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
def dump(v, indent):
    return json.dumps(v, indent=2, ensure_ascii=False).replace("\n", "\n" + " " * indent)
def indent_of(d, node):
    m = re.search(r"\n([ ]*)$", d.t[:node["s"]]); return len(m.group(1)) if m else 0
def find(d, path, sid):
    for it in s.walk(d.root, path)["items"]:
        for k, v in it["pairs"]:
            if k == "id" and json.loads(d.t[v["s"]:v["e"]]) == sid: return it
    raise KeyError((path, sid))
def apply(slug, ops):
    d = s.Doc(slug)
    for o in ops:
        kind = o["op"]
        if kind.startswith("shape"): path = ["asset", "representations", o["scale"], "geometry", o["view"]]
        else: path = ["asset", "volumes", o["scale"], "body"]
        if kind in ("shape", "solid"):
            n = find(d, path, o["id"]); d.rep(n, dump(o["shape" if kind == "shape" else "solid"], indent_of(d, n)))
        elif kind in ("shape-set", "solid-set"):
            n = find(d, path, o["id"]); cur = json.loads(d.t[n["s"]:n["e"]]); cur.update(o["set"]); d.rep(n, dump(cur, indent_of(d, n)))
        elif kind in ("shape-add", "solid-add"):
            n = find(d, path, o["after"]); ind = indent_of(d, n)
            d.rep({"s": n["e"], "e": n["e"]}, ",\n" + " " * ind + dump(o["shape" if kind == "shape-add" else "solid"], ind))
        else: raise ValueError(kind)
    d.commit()
if __name__ == "__main__":
    apply(sys.argv[1], json.load(open(sys.argv[2])))
