import os,re,json,sys
root="/Users/ueli/Documents/semio/🧰️framework"
skip={"node_modules","target",".cache","🗑️generated","dist"}
pat=re.compile(r"evaluateExtractedFunctionV1|new Function\(")
out={}
for d,ds,fs in os.walk(root):
    ds[:]=[x for x in ds if x not in skip]
    rel=os.path.relpath(d,root)
    if "🧩️extracted-function" in rel: continue
    for f in fs:
        if not f.endswith((".ts",".mjs",".js")): continue
        p=os.path.join(rel,f)
        if not re.search(r"🧪️tests|🧪️\.ts|\.test\.",p): continue
        n=len(pat.findall(open(os.path.join(d,f),encoding="utf8",errors="replace").read()))
        if n: out[p]=n
json.dump({"version":1,"ceilings":dict(sorted(out.items()))},open(sys.argv[1],"w"),ensure_ascii=False,indent=1)
print(len(out),sum(out.values()))
