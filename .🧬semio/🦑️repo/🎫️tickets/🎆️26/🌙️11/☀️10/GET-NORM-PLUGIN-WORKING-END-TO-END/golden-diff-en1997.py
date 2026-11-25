import json, re, sys
log = open("🗑️generated/cp-scene-en1997.log", errors="replace").read()
def grab(tag):
    m = re.search(r"^ " + tag + r": (\".*\")$", log, re.M)
    return json.loads(json.loads(m.group(1).replace('\\"', '"').join(['"', '"'])) if False else None) if False else m.group(1)
def unq(s):
    return json.loads('"' + s[1:-1].replace('\\"','\\\\"') + '"') if False else eval(s.replace('\\"', '"').replace('"', '\\"').join(['"','"'])) if False else None
import ast
def dec(s):
    return ast.literal_eval("'" + s[1:-1].replace("'", "\\'") + "'") if False else s[1:-1].encode().decode("unicode_escape").encode("latin1").decode("utf8")
l = re.search(r"^  left: (\".*\")$", log, re.M).group(1); r = re.search(r"^ right: (\".*\")$", log, re.M).group(1)
a = json.loads(dec(l)); b = json.loads(dec(r))
def keyed(s):
    out = {}
    for e in s["entities"]: out["entity:" + e["entity"]["id"]] = e
    for x in s["annotations"]: out["annotation:" + x["data"]["id"]] = x
    return out
ka, kb = keyed(a), keyed(b)
print("only in run:", sorted(set(ka) - set(kb))); print("only in golden:", sorted(set(kb) - set(ka)))
for k in sorted(set(ka) & set(kb)):
    if ka[k] != kb[k]:
        fa, fb = ka[k], kb[k]
        def walk(x, y, p=""):
            if isinstance(x, dict) and isinstance(y, dict):
                for kk in sorted(set(x) | set(y)): walk(x.get(kk), y.get(kk), p + "/" + kk)
            elif isinstance(x, list) and isinstance(y, list) and len(x) == len(y):
                for i, (u, v) in enumerate(zip(x, y)): walk(u, v, p + f"[{i}]")
            elif x != y: print(k, p, "run=", str(x)[:80], "golden=", str(y)[:80])
        walk(fa, fb)
for kk in ("bounds","layers","legends","owners","tracks","description","revision"):
    if a.get(kk) != b.get(kk): print("top-level differs:", kk)
