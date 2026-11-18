"""Applies the output lines of vegetation-extent.ts: python3 vegetation-extent-apply.py <slug> <lines.jsonl>"""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib"); sp = mv.sp
d = mv.D("🌳️vegetation/" + sys.argv[1])
for line in open(sys.argv[2]):
    if not line.startswith("{"): continue
    e = json.loads(line)
    if "error" in e: print("SKIP", e); continue
    node = d.solid(e["scale"], e["solid"])
    if "translate" in e:
        tr = e["translate"]
        if e["kind"] == "loft":
            secs = sp.walk(node, ["sections"])["items"]
            for k in range(len(secs)):
                at = sp.walk(node, ["sections", str(k), "at"])
                cur = json.loads(d.t[at["s"]:at["e"]])
                d.setv(at, [round(cur[i] + tr[i], 5) for i in range(3)])
        else:
            try:
                tn = sp.walk(node, ["transform", "translation"])
                cur = json.loads(d.t[tn["s"]:tn["e"]]); d.setv(tn, [round(cur[i] + tr[i], 5) for i in range(3)])
            except KeyError:
                try:
                    tnode = sp.walk(node, ["transform"]); cur = json.loads(d.t[tnode["s"]:tnode["e"]]); cur["translation"] = [round(v, 5) for v in tr]
                    d.replace_obj(tnode, cur)
                except KeyError:
                    last = node["pairs"][-1][1]
                    ind = d.t[d.t.rfind("\n", 0, node["s"]) + 1:node["s"]]
                    d.rep({"s": last["e"], "e": last["e"]}, ',\n' + ind + '  "transform": {"translation": ' + json.dumps([round(v, 5) for v in tr]) + '}')
        print("translate", e["scale"], e["solid"], tr); continue
    sec = sp.walk(node, ["sections", str(e["section"]), "at"])
    d.setv(sec, [round(v, 5) for v in e["at"]])
    print("set", e["scale"], e["solid"], e["section"], e["at"])
d.commit()
