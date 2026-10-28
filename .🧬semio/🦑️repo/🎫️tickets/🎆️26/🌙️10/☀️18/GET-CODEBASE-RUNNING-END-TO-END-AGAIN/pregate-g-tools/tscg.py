"""🔎 pregate-g: tsc --strict (root tsconfig options) over the .ts files that differ between BASE root and ROOT; prints diagnostics located in touched files, compared with the same files in BASE.
usage: tscg.py ROOT BASE OUT.json"""
import filecmp,json,os,re,subprocess,sys,time
root,base,out=[os.path.abspath(a) for a in sys.argv[1:4]]
F="🧰️framework"
subs=[F+"/🛍️products/🦑️repo/🔨️modules/📚️library",F+"/🔨️modules/🏃️process",F+"/🚀️launcher"]
touched=[]
for s in subs:
    for dp,dn,fn in os.walk(os.path.join(root,s)):
        dn[:]=[d for d in dn if d!="node_modules"]
        for f in fn:
            if not f.endswith((".ts",".mts")) or f.endswith(".d.ts"):continue
            p=os.path.join(dp,f);q=os.path.join(base,os.path.relpath(p,root))
            if not os.path.exists(q) or not filecmp.cmp(p,q,shallow=False):touched.append(os.path.relpath(p,root))
print("touched ts files:",len(touched))
def tsc(r,files):
    cfg={"extends":os.path.join(r,"tsconfig.json"),"compilerOptions":{"noEmit":True,"incremental":False,"skipLibCheck":True,"strict":True},"include":[],"exclude":[],"files":[os.path.join(r,f) for f in files if os.path.exists(os.path.join(r,f))]}
    p=os.path.join(r,".pgt/tsconfig.pg.json");os.makedirs(os.path.dirname(p),exist_ok=True);json.dump(cfg,open(p,"w"))
    t=time.time();res=subprocess.run(["node","--max-old-space-size=6144",os.path.join(r,"node_modules/typescript/bin/tsc"),"-p",p,"--pretty","false"],cwd=r,capture_output=True,text=True)
    return res.stdout+res.stderr,time.time()-t
def diag(text,r,files):
    keep=set(files);rows=[]
    for l in text.splitlines():
        m=re.match(r"^(.*?)\((\d+),\d+\): error (TS\d+): (.*)$",l)
        if m:
            f=os.path.relpath(os.path.abspath(os.path.join(r,m.group(1))),r) if not os.path.isabs(m.group(1)) else os.path.relpath(m.group(1),r)
            if f in keep:rows.append((f,m.group(3),m.group(4)[:160]))
    return rows
a,ta=tsc(root,touched);ra=diag(a,root,touched)
b,tb=tsc(base,touched);rb=diag(b,base,touched)
nb={(f,c,m) for f,c,m in rb};new=[r for r in ra if r not in nb]
json.dump({"touched":len(touched),"errors":len(ra),"baseErrors":len(rb),"new":new,"secs":[round(ta),round(tb)],"raw_first":a[:1500]},open(out,"w"),ensure_ascii=False,indent=1)
print(f"tsc strict: touched {len(touched)} files, errors in touched {len(ra)} (base {len(rb)}), NEW {len(new)} [{ta:.0f}s/{tb:.0f}s]")
for r in new[:15]:print("NEW",r)
