import sys,os,json,subprocess,collections,re,shutil
HERE=os.path.dirname(os.path.abspath(__file__))
WORK=os.environ.get("HO_WORK",HERE)
MS=os.path.abspath(os.path.join(HERE,".."))
sys.path.insert(0,HERE)
import mksbx, neutral
REPO="/Users/ueli/Documents/semio"
SPEC=json.loads(open(HERE+"/spec.json").read().replace("{TOOLS}",HERE).replace("{MS}",MS))
CFG={"native":[], "test":["--test"], "wasm":["--target","wasm32-unknown-unknown"], "wasip2":["--target","wasm32-wasip2"], "testwasip2":["--test","--target","wasm32-wasip2"], "testwasm":["--test","--target","wasm32-unknown-unknown"],"bin":[],
     "feat":["--cfg",'feature="integration-fixtures"'], "testfeat":["--test","--cfg",'feature="integration-fixtures"']}
def prep(t):
    s=SPEC[t]
    raw=WORK+"/raw-"+t
    ov={s["root"]:REPO+"/"+s["root"]}
    ov.update(s["baseline"])
    mksbx.mk(raw,ov)
    neutral.run(t+"-base",raw,s["root"],WORK+"/nsbx-"+t+"-base")
    neutral.run(t+"-split",REPO,s["root"],WORK+"/nsbx-"+t+"-split")
def rustc(root,crate,ct,extra,out,fam):
    cmd=["rustc","+nightly-2026-07-20","--edition","2021","--crate-type",ct,"--crate-name",crate,"--emit=metadata","-o","/dev/null","--error-format=json","--cap-lints","allow"]+extra+[root]
    p=subprocess.run(cmd,capture_output=True,text=True)
    open(out+".raw","w").write(p.stderr)
    keys=collections.Counter(); rows=[]
    fams=[re.compile(x) for x in (fam if isinstance(fam,list) else [fam])] if fam else []
    for line in p.stderr.splitlines():
        if not line.startswith("{"): continue
        try: d=json.loads(line)
        except: continue
        if not str(d.get("level","")).startswith("error"): continue
        code=(d.get("code") or {}).get("code") or ""
        msg=d.get("message","")
        ps=[x for x in d.get("spans",[]) if x.get("is_primary")]
        if not ps:
            k=(code,msg,"","",""); keys[k]+=1; rows.append(k+(0,"")); continue
        x=ps[0]; txt=""
        if x.get("text"):
            tt=x["text"][0]; txt=tt["text"][tt["highlight_start"]-1:tt["highlight_end"]-1]
        f=x["file_name"]
        f=re.sub(r"^.*?/(nsbx-[^/]+|raw-[^/]+)/","",f)
        f=f.replace(REPO+"/","")
        f=os.path.normpath(f)
        fr=f
        for i,fr_ in enumerate(fams):
            if fr_.search(f): f="FAMILY%d"%i; break
        msg=re.sub(r"`[^`]*/(nsbx-[^/]+|raw-[^/]+)/[^`]*`","`<sandbox-path>`",msg)
        k=(code,msg,txt,"",f)
        keys[k]+=1; rows.append(k+(x["line_start"],fr))
    json.dump(rows,open(out+".rows.json","w"),ensure_ascii=False)
    return keys,p.returncode,rows
EXT=re.compile(r"cannot find module or crate `|unresolved imports? |use of unresolved module or unlinked crate|aborting due|cannot find attribute|could not find `\w+` in|failed to resolve: use of undeclared crate")
def run(t,cfg,which=("base","split")):
    s=SPEC[t]; extra=CFG[cfg]+s.get("extra",[])
    res={}
    for w in which:
        if w=="rawbase": root=WORK+"/raw-"+t+"/"+s["root"]
        elif w=="rawsplit": root=REPO+"/"+s["root"]
        else: root=WORK+"/nsbx-"+t+"-"+w+"/"+s["root"]
        res[w]=rustc(root,s["crate"],s["ct"],extra,WORK+"/out-"+t+"-"+cfg+"-"+w,s["fam"])
    return res
def total(c): return sum(c.values())
def nonext(c): return collections.Counter({k:v for k,v in c.items() if not EXT.search(k[1])})
if __name__=="__main__":
    cmd=sys.argv[1]
    if cmd=="prep": prep(sys.argv[2]); sys.exit()
    t=sys.argv[2]; cfgs=sys.argv[3].split(",")
    mode=sys.argv[4] if len(sys.argv)>4 else "patched"
    for cfg in cfgs:
        if mode=="patched":
            r=run(t,cfg); A,rc0,_=r["base"]; B,rc1,_=r["split"]
        else:
            r=run(t,cfg,("rawbase","rawsplit")); A,rc0,_=r["rawbase"]; B,rc1,_=r["rawsplit"]
        more=B-A; less=A-B
        print(f"[{t} {cfg} {mode}] base={total(A)} split={total(B)} nonext base={total(nonext(A))} split={total(nonext(B))} distinct base={len(A)} split={len(B)} | split-only={total(more)} base-only={total(less)}")
        for k,n in sorted(more.items()): print("  +SPLIT",n,tuple(str(x)[:140] for x in k))
        for k,n in sorted(less.items()): print("  -BASE ",n,tuple(str(x)[:140] for x in k))
