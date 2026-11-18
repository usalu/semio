"""Helpers of the machinery and vehicles silhouette repairs: span edits on top of sil-span, shape lookup in representations and volumes, inserting a copy of a shape with changes."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
sp = importlib.import_module("sil-span")
class D(sp.Doc):
    def shape(s, scale, view, sid):
        for it in s.at("asset", "representations", str(scale), "geometry", view)["items"]:
            for k, v in it["pairs"]:
                if k == "id" and json.loads(s.t[v["s"]:v["e"]]) == sid: return it
        raise KeyError(sid)
    def solid(s, scale, sid, lst="body"):
        return s.find(str(scale), sid, lst)
    def field(s, node, key): return sp.child(node, key)
    def set(s, node, key, value): s.setv(sp.child(node, key), value)
    def obj(s, node): return json.loads(s.t[node["s"]:node["e"]])
    def insert_after(s, node, obj_):
        i = s.t.rfind("\n", 0, node["s"]) + 1
        ind = s.t[i:node["s"]]
        txt = json.dumps(obj_, ensure_ascii=False, indent=2).replace("\n", "\n" + ind)
        s.edits.append((node["e"], node["e"], ",\n" + ind + txt))
    def pretty(s, node, obj_):
        i = s.t.rfind("\n", 0, node["s"]) + 1
        ind = s.t[i:node["s"]]
        return json.dumps(obj_, ensure_ascii=False, indent=2).replace("\n", "\n" + ind)
    def replace_obj(s, node, obj_): s.edits.append((node["s"], node["e"], s.pretty(node, obj_)))
    def delete_item(s, node):
        j = s.t.rfind(",", 0, node["s"]); k = s.t.find(",", node["e"])
        i = s.t.rfind("\n", 0, node["s"]) + 1
        nxt = s.t[node["e"]:].lstrip()
        if nxt.startswith(","): s.edits.append((i, s.t.index(",", node["e"]) + 1 + (1 if s.t[s.t.index(",", node["e"]) + 1] == "\n" else 0), ""))
        else: s.edits.append((j, node["e"], ""))
    def sub(s, node, old, new):
        key = (node["s"], node["e"])
        cur = s.pend.get(key, s.t[node["s"]:node["e"]]) if hasattr(s, "pend") else s.t[node["s"]:node["e"]]
        assert old in cur, old
        if not hasattr(s, "pend"): s.pend = {}
        s.pend[key] = cur.replace(old, new)
    def commit(s):
        for (a, b), x in getattr(s, "pend", {}).items(): s.edits.append((a, b, x))
        s.pend = {}
        sp.Doc.commit(s)
