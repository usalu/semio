"""Helpers for the furniture, objects and vegetation silhouette repairs: id-addressed in-place value edits on top of sil-span.py."""
import importlib.util, json, os
_spec = importlib.util.spec_from_file_location('s', os.path.join(os.path.dirname(os.path.abspath(__file__)), 'sil-span.py'))
s = importlib.util.module_from_spec(_spec); _spec.loader.exec_module(s)

def byid(d, path, sid):
    for it in s.walk(d.root, path)["items"]:
        for k, v in it["pairs"]:
            if k == "id" and json.loads(d.t[v["s"]:v["e"]]) == sid: return it
    raise KeyError((path, sid))

def vol(d, scale, sid, *sub, list_="body"):
    n = byid(d, ["asset", "volumes", str(scale), list_], sid)
    return s.walk(n, list(sub)) if sub else n

def shape(d, scale, view, sid, *sub):
    n = byid(d, ["asset", "representations", str(scale), "geometry", view], sid)
    return s.walk(n, list(sub)) if sub else n

def setv(d, node, value): d.setv(node, value)

def setfloat(d, node, value):
    d.rep(node, repr(round(value, 6)) if isinstance(value, float) else json.dumps(value))
