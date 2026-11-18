import importlib.util, json, os
here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location('s', here + '/sil-span.py'); s = importlib.util.module_from_spec(spec); spec.loader.exec_module(s)
import glob
def doc(slug):
    p = glob.glob(s.CAT + '*/' + slug + '/🔣️.json')[0]
    cat = p[len(s.CAT):-len('/🔣️.json')]
    return s.Doc(cat)
def byid(d, path, sid):
    for it in s.walk(d.root, path)["items"]:
        for k, v in it["pairs"]:
            if k == "id" and json.loads(d.t[v["s"]:v["e"]]) == sid: return it
    raise KeyError(sid)
def V(d, scale, sid): return byid(d, ["asset", "volumes", scale, "body"], sid)
def R(d, scale, view, sid): return byid(d, ["asset", "representations", scale, "geometry", view], sid)
def sp(d, node, *path, val):
    n = node
    for p in path: n = s.child(n, p)
    d.setv(n, val)
def get(d, node, *path):
    n = node
    for p in path: n = s.child(n, p)
    return json.loads(d.t[n["s"]:n["e"]])
import re
def ins_after(d, node, obj):
    before = d.t[:node["s"]]
    indent = re.search(r'\n([ ]*)$', before).group(1)
    txt = json.dumps(obj, ensure_ascii=False, indent=2)
    txt = txt.replace('\n', '\n' + indent)
    d.rep({"s": node["e"], "e": node["e"]}, ',\n' + indent + txt)
def rm(d, node):
    m = re.match(r'\s*,\s*', d.t[node["e"]:])
    if m: d.rep({"s": node["s"], "e": node["e"] + m.end()}, '')
    else:
        before = d.t[:node["s"]]
        m2 = re.search(r',\s*$', before)
        d.rep({"s": m2.start(), "e": node["e"]}, '')
def ins_before(d, node, obj):
    before = d.t[:node["s"]]
    indent = re.search(r'\n([ ]*)$', before).group(1)
    txt = json.dumps(obj, ensure_ascii=False, indent=2).replace('\n', '\n' + indent)
    d.rep({"s": node["s"], "e": node["s"]}, txt + ',\n' + indent)
def head_obj(slug, scale, view, sid):
    import subprocess
    p = glob.glob(s.CAT + '*/' + slug + '/🔣️.json')[0]
    t = subprocess.run(["git", "-C", s.ROOT, "show", "HEAD:" + p[len(s.ROOT) + 1:]], capture_output=True, text=True, encoding="utf8").stdout
    a = json.loads(t)['asset']
    for x in a['representations'][scale]['geometry'][view]:
        if x['id'] == sid: return x
def replace_obj(d, node, fn):
    obj = json.loads(d.t[node["s"]:node["e"]])
    new = fn(obj) or obj
    before = d.t[:node["s"]]
    indent = re.search(r'\n([ ]*)$', before).group(1)
    txt = json.dumps(new, ensure_ascii=False, indent=2).replace('\n', '\n' + indent)
    d.rep(node, txt)
