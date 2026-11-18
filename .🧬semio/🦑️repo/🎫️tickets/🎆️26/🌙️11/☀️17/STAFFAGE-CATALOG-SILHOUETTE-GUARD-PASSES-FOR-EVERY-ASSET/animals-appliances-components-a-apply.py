import sys, json, subprocess, os
exec(open(os.path.dirname(os.path.abspath(__file__)) + "/animals-appliances-components-a-lib.py").read())
B = "/Users/ueli/Documents/semio/.cache/b/t/m/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f/bun"
T = os.path.dirname(os.path.abspath(__file__))
slug, ops = sys.argv[1], sys.argv[2]
full = json.load(open(ops))
adds = [o for o in full if "add" in o]
replaces = [o for o in full if "replace" in o]
rest = [o for o in full if "add" not in o and "replace" not in o]
ops = T + "/🗑️generated/aac-a-rest-" + slug + ".json"
json.dump(rest, open(ops, "w"), ensure_ascii=False)
out = T + "/🗑️generated/aac-a-patched-" + slug + ".json"
env = dict(os.environ, OPS=ops)
subprocess.run([B, T + "/animals-appliances-components-a-dump.ts", slug, out], check=True, env=env, cwd="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/📦️packages/🟦️typescript")
d = e.doc(slug)
new = json.load(open(out)); old = json.loads(d.t)["asset"]
n = [0]
def walk(o, nw, path):
    if isinstance(o, dict):
        assert o.keys() == nw.keys(), path
        for k in o: walk(o[k], nw[k], path + [k])
    elif isinstance(o, list):
        assert len(o) == len(nw), path
        for i in range(len(o)): walk(o[i], nw[i], path + [str(i)])
    elif o != nw:
        d.setv(e.s.walk(d.root, ["asset"] + path), nw); n[0] += 1
walk(old, new, [])
anchors = sys.argv[3].split(",") if len(sys.argv) > 3 else []
for o, anchor in zip(adds, anchors):
    node = e.R(d, o["scale"], o["view"], anchor)
    e.ins_after(d, node, o["add"])
for o in replaces:
    node = e.R(d, o["scale"], o["view"], o["id"])
    e.replace_obj(d, node, lambda old, n=o["replace"]: n)
d.commit(); print("changed values", n[0])
