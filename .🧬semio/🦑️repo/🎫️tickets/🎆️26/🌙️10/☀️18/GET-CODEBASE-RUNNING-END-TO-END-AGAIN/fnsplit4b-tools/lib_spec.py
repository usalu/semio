"""🧩 Shared helpers for the statement-run phase specs of fnsplit2."""
import json
V = "️"
ROOT = "/Users/ueli/Documents/semio/"
def child(e, name, text):
    return {"dir": e + V + name, "doc": "//! %s%s %s See [`super`]." % (e, V, text)}
def ph(name, doc, a, b, ch, struct=None, struct_doc=None, **kw):
    d = {"name": name, "doc": doc, "from": a, "to": b, "child": ch}
    if struct:
        d.update(struct=struct, struct_doc=struct_doc)
    d.update(kw)
    return d
def dump(spec, path):
    json.dump(spec, open(path, "w", encoding="utf8"), ensure_ascii=False, indent=1)
