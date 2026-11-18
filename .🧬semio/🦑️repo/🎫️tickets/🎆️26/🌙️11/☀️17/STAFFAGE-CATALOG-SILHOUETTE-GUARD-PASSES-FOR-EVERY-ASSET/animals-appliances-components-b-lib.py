"""Span edits for the animals: set keys of drawn shapes and volume solids across scales, value-only."""
import sys, json, importlib
sys.path.insert(0, "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️17/STAFFAGE-WEBSITE-CDN-REDEPLOY-READINESS")
mv = importlib.import_module("mvlib")
CAT = "🐕️animals/"
def doc(slug): return mv.D(CAT + slug)
def pretty(d, node, v):
    i = d.t.rfind("\n", 0, node["s"]) + 1
    line = d.t[i:node["s"]]
    ind = line[:len(line) - len(line.lstrip())]
    return json.dumps(v, ensure_ascii=False, indent=2).replace("\n", "\n" + ind)
def put(d, node, key, v, inline=False):
    c = d.field(node, key)
    d.rep(c, pretty(d, c, v) if isinstance(v, (list, dict)) and not inline else mv.sp.flat(v))
def shape(d, scale, view, sid, **kv):
    n = d.shape(scale, view, sid)
    for k, v in kv.items(): put(d, n, k, v)
def shapes(d, scales, view, sid, **kv):
    for sc in scales: shape(d, sc, view, sid, **kv)
def solid(d, scale, sid, **kv):
    n = d.solid(scale, sid)
    for k, v in kv.items(): put(d, n, k, v, True)
def getshape(d, scale, view, sid): return d.obj(d.shape(scale, view, sid))
def getsolid(d, scale, sid): return d.obj(d.solid(scale, sid))
