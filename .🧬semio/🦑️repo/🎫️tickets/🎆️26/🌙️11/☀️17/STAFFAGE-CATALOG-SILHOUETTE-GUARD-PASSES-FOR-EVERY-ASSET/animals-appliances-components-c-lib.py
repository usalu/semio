"""Helpers of the components silhouette repairs: duplicate a drawn shape with changes, set fields."""
import sys
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
from mvlib import *
def dup(d, scale, view, sid, newid, **ch):
    n = d.shape(scale, view, sid); o = d.obj(n); o["id"] = newid; o.update(ch); d.insert_after(n, o)
def setf(d, scale, view, sid, **ch):
    n = d.shape(scale, view, sid)
    for k, v in ch.items(): d.set(n, k, v)
import importlib
_sp = importlib.import_module("sil-span")
def sn(d, node, *path, value):
    n = node
    for p in path: n = _sp.child(n, p)
    d.setv(n, value)
def replace_value(d, node, obj_):
    i = d.t.rfind("\n", 0, node["s"]) + 1
    line = d.t[i:node["s"]]
    ind = line[: len(line) - len(line.lstrip())]
    d.edits.append((node["s"], node["e"], json.dumps(obj_, ensure_ascii=False, indent=2).replace("\n", "\n" + ind)))
def rm_key(d, node, key):
    pairs = node["pairs"]
    for i, (k, v) in enumerate(pairs):
        if k != key: continue
        ks = d.t.rfind('"' + key + '"', 0, v["s"])
        if i == len(pairs) - 1:
            start = d.t.rfind(",", 0, ks)
            d.edits.append((start, v["e"], ""))
        else:
            nxt = pairs[i + 1][1]
            nks = d.t.rfind('"', 0, nxt["s"])
            nks = d.t.rfind('"', 0, nks)
            d.edits.append((ks, nks, ""))
        return
    raise KeyError(key)
