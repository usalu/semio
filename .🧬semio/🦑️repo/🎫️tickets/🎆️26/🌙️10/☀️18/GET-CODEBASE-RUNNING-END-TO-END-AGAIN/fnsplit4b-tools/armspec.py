"""🧱 Builds an inline arms spec for rsfn.py. usage (import): build(file, function, match_line, rows, ret, method=None, types=None, nth=0, out=path, **extra)
rows: [(armStartLine, helperName, doc text) | (line, name, doc, {"binds": {...}, "derive": {...}, "allow_returns": bool})]
Every helper gets a unique doc emoji (unused in the file), lands right after the function (inline) and keeps `allow_returns` true unless the row says otherwise."""
import json, os, sys
sys.path.insert(0, os.path.dirname(os.path.abspath(__file__)))
import emojipool

def build(file, function, match_line, rows, ret, method=None, types=None, nth=0, out=None, **extra):
    text = open(file, encoding="utf-8").read()
    em = emojipool.pick(text, len(rows))
    arms = []
    for r, e in zip(rows, em):
        line, name, doc = r[:3]
        opt = r[3] if len(r) > 3 else {}
        a = {"line": line, "name": name, "doc": "%s %s" % (e, doc), "child": {"dir": "inline", "doc": ""}, "allow_returns": opt.get("allow_returns", True), "binds": opt.get("binds", {})}
        if "derive" in opt: a["derive"] = opt["derive"]
        arms.append(a)
    spec = {"file": file, "function": function, "nth": nth, "match_line": match_line, "types": types or {}, "ret": ret, "inline": True, "arms": arms}
    if method: spec["method"] = method
    spec.update(extra)
    if out: json.dump(spec, open(out, "w", encoding="utf-8"), ensure_ascii=False, indent=1)
    return spec
