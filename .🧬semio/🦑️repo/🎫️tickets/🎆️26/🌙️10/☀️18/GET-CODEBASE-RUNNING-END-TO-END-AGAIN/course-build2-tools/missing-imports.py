import re,sys,os,json,collections
root="/Users/ueli/Documents/semio/"
start=root+sys.argv[1]
rx=re.compile(r'''(?:\bfrom|\bimport)\s*\(?\s*["']([^"'\n]+)["']|\bexport\s+\*\s+from\s+["']([^"']+)["']''')
cfg=open(root+"🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts",encoding="utf8").read()
aliases=[(re.compile(m.group(1).replace("\\/","/")),root+m.group(2)) for m in re.finditer(r'find: /(\^[^/]+(?:\\/[^/]+)*\$)/,\s*replacement: resolve\(repoRoot,\s*"([^"]+)"\)',cfg)]
exts=["",".ts",".tsx",".mjs",".js",".json",".css","/🟦️.ts","/🟦️.tsx"]
def try_file(p):
    for e in exts:
        if os.path.isfile(p+e):return p+e
def pkg(spec):
    parts=spec.split("/");name="/".join(parts[:2]) if spec.startswith("@") else parts[0];sub="/".join(parts[2:] if spec.startswith("@") else parts[1:])
    d=root+"node_modules/"+name
    if not os.path.isdir(d):return None
    try:j=json.load(open(d+"/package.json",encoding="utf8"))
    except Exception:return d
    key="."+("/"+sub if sub else "")
    ex=j.get("exports")
    if isinstance(ex,dict) and key in ex:
        t=ex[key]
        while isinstance(t,dict):t=t.get("import") or t.get("default") or t.get("types") or next(iter(t.values()))
        return os.path.join(d,t) if isinstance(t,str) else d
    if not sub:return d
    return d+"/"+sub
seen=set();missing=collections.defaultdict(list);q=[start]
while q:
    f=q.pop()
    if f in seen:continue
    seen.add(f)
    if not f.endswith((".ts",".tsx",".mjs",".js")) :continue
    try:s=open(f,encoding="utf8").read()
    except Exception:continue
    for m in rx.finditer(s):
        spec=m.group(1) or m.group(2)
        if spec.startswith(("node:","http")) :continue
        t=None
        if spec.startswith("."):
            base=os.path.normpath(os.path.join(os.path.dirname(f),spec)); t=try_file(base)
            if not t:missing[f.replace(root,"")].append(spec);continue
        else:
            hit=next((r for rg,r in aliases if rg.search(spec)),None)
            if hit:
                t=try_file(hit)
                if not t:missing[f.replace(root,"")].append(spec+" (alias -> "+hit.replace(root,"")+")");continue
            elif spec.startswith("@semio-tech/"):
                p=pkg(spec)
                if p is None:missing[f.replace(root,"")].append(spec+" (package not installed)");continue
                t=try_file(p) or (p if os.path.isdir(p) else None)
                if not t:missing[f.replace(root,"")].append(spec+" (-> "+p.replace(root,"")+")");continue
            else:continue
        if t and os.path.isfile(t):q.append(os.path.realpath(t))
print("visited",len(seen))
for f,l in missing.items():
    for x in l:print("MISSING",f[-90:],"->",x[-150:])
