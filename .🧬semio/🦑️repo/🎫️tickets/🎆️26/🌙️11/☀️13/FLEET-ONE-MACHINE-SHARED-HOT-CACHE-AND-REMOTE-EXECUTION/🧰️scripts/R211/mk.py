import json,os,shutil,sys
R="/Users/ueli/Documents/semio"; S="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R211"
B=R+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
OBS=B+"/👁️observation"
mode=sys.argv[1]  # new | base
def rel(p): return p[len(R)+1:]
mx=S+"/mx"+mode
shutil.rmtree(mx,ignore_errors=True)
def put(live,src):
    d=os.path.join(mx,rel(live)); os.makedirs(os.path.dirname(d),exist_ok=True); shutil.copyfile(src,d); return d
m={}
m[OBS+"/🟨️.mjs"]=put(OBS+"/🟨️.mjs",S+"/stage/obs."+mode+".mjs")
put(OBS+"/🧬️schema/🔣️.json",OBS+"/🧬️schema/🔣️.json")
put(R+"/🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts",R+"/🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts")
m[OBS+"/📂️source-names/🟨️.mjs"]=put(OBS+"/📂️source-names/🟨️.mjs",S+"/stage/sn."+mode+".mjs")
m[OBS+"/📂️source-names/🧬️schema/🔣️.json"]=S+"/stage/sch."+mode+".json"
RD=B+"/🌐️remote-execution/🛰️sidecar/🕸️graph"
m[RD+"/🔭️read/🟦️.ts"]=put(RD+"/🔭️read/🟦️.ts",S+"/stage/read."+mode+".ts")
put(RD+"/🏃️runner/🟦️.ts",RD+"/🏃️runner/🟦️.ts")
json.dump(m,open(S+"/map."+mode+".json","w"),ensure_ascii=False,indent=0)
# run copies
for name,src,subs in [("parser-identity","law/parser-identity/test.ts",[("\"../../🟨️.mjs\"",json.dumps(OBS+"/🟨️.mjs",ensure_ascii=False)),("new URL(\"../../🟨️.mjs\",import.meta.url).pathname",json.dumps(OBS+"/🟨️.mjs",ensure_ascii=False))]),
 ("machine-local-paths","law/machine-local-paths/test.ts",[("\"../../../🟨️.mjs\",import.meta.url","\"file://"+mx+"/"+rel(OBS)+"/🟨️.mjs\""),("\"../../🧬️schema/🔣️.json\"",json.dumps(OBS+"/📂️source-names/🧬️schema/🔣️.json",ensure_ascii=False)),("import(\"../../../🟨️.mjs\")",  "import("+json.dumps(OBS+"/🟨️.mjs",ensure_ascii=False)+")"),("import(\"../../🟨️.mjs\")","import("+json.dumps(OBS+"/📂️source-names/🟨️.mjs",ensure_ascii=False)+")")])]:
    t=open(S+"/"+src,encoding="utf8").read()
    if name=="parser-identity":
        t=t.replace("fileURLToPath(new URL(\"../../🟨️.mjs\",import.meta.url)),observation=await import(\"../../🟨️.mjs\")","OBSPATH,observation=await import(OBSPATH)")
        t="const OBSPATH="+json.dumps(OBS+"/🟨️.mjs",ensure_ascii=False)+";\n"+t.replace("const observationPath=OBSPATH,","const observationPath=OBSPATH,")
    else:
        for a,b in subs:
            t=t.replace(a,b)
        t=t.replace("new URL(\"file://"+mx,"new URL(\"file://"+mx)
    d=S+"/run-"+mode+"/"+name; os.makedirs(d,exist_ok=True); open(d+"/🟦️.ts","w",encoding="utf8").write(t)
