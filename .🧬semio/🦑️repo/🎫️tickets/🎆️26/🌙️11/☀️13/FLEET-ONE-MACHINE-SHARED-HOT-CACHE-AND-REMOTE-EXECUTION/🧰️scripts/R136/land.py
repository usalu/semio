import hashlib,importlib.util,json,os,shutil,subprocess,sys
D=os.path.dirname(os.path.abspath(__file__))
SC="/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R136"
W="/Users/ueli/Documents/semio"
LIB=W+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
LAB=LIB+"/🧪️testing/🔬️slice-lab"
ASM=LIB+"/🏗️build/🕸️graph/📥️inference/🧱️project-assembly/🟨️.mjs"
RENDER=W+"/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🏗️ifc/📚️catalog/🛠️extract/🦀️render.ts"
OBS=LIB+"/🏗️build/👁️observation/🟨️.mjs"
WIDE=LIB+"/🏗️build/👁️observation/🧪️tests/📚️wide-failure/🟦️.ts"
REQ=LIB+"/🏗️build/🕸️graph/📥️inference/🧪️tests/🔌️providers/🎯️requested-group/🟦️.ts"
def load(n,f):
    s=importlib.util.spec_from_file_location(n,os.path.join(D,f));m=importlib.util.module_from_spec(s);s.loader.exec_module(m);return m
lab=load("lab","lab-patch.py");asm=load("asm","patch-assembly.py");obs=load("obs","patch-observation.py")
sha=lambda b:hashlib.sha256(b).hexdigest()[:12]
BK=SC+"/backup";os.makedirs(BK,exist_ok=True)
manifest=[]
def edit(path,fn):
    b=open(path,"rb").read();t=fn(b.decode("utf8"));cp=os.path.join(BK,sha(path.encode()));open(cp,"wb").write(b)
    open(path,"w",encoding="utf8").write(t);manifest.append({"file":path,"backup":cp,"before":sha(b),"after":sha(t.encode())})
def new(path,content):
    assert not os.path.exists(path),path
    os.makedirs(os.path.dirname(path),exist_ok=True);open(path,"wb").write(content);manifest.append({"file":path,"backup":None,"before":None,"after":sha(content)})
try:
    new(RENDER,open(SC+"/render.ts","rb").read())
    edit(ASM,asm.patch);edit(OBS,obs.patch)
    new(WIDE,open(D+"/new/wide-failure.tmpl.ts",encoding="utf8").read().replace("{{OBS}}","../../🟨️.mjs").encode("utf8"))
    edit(LAB+"/🟦️.ts",lab.patch_ts);edit(LAB+"/🧬️schema/🔣️.json",lab.patch_schema);edit(LAB+"/🧪️tests/🟦️.ts",lab.patch_test)
    tmp=SC+"/req-render/"+"🟦️.ts";os.makedirs(os.path.dirname(tmp),exist_ok=True)
    t=open(D+"/new/requested-group.tmpl.ts",encoding="utf8").read()
    paths={"ROUTING":W+"/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts","PRIVATE":LIB+"/🧪️testing/🧫️private-repository/🟦️.ts","BIRTH":LIB+"/🧪️testing/🪪️native-birth/🟦️.ts","INFER":LIB+"/🏗️build/🕸️graph/📥️inference","CORPUS":LIB+"/🏗️build/🕸️graph/📥️inference/🧪️tests/🔌️providers/🔣️.json"}
    for k,v in paths.items():t=t.replace("{{"+k+"}}",os.path.relpath(v,os.path.dirname(REQ)))
    t=t.replace("{{SCHEMA}}",LIB+"/🏃️process/🔌️providers/🧬️schema/🔣️.json")
    new(REQ,t.encode("utf8"))
except Exception as e:
    json.dump(manifest,open(BK+"/manifest.json","w"),indent=1);print("LAND FAILED",repr(e));sys.exit(1)
json.dump(manifest,open(BK+"/manifest.json","w"),indent=1);print("landed",len(manifest),"files")
