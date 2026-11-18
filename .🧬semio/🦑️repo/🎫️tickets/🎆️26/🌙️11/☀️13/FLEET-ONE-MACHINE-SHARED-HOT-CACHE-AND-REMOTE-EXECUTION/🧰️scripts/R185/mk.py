import json,os,sys,shutil
S=os.path.dirname(os.path.abspath(__file__));mode=sys.argv[1] if len(sys.argv)>1 else "staged"
B="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
R=B+"/🌐️remote-execution/🛰️sidecar/🚦️ready";D=B+"/🌡️host-capacity/🧹️disk-recovery"
X="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset"
mod=[("f1.ts",R+"/🟦️.ts"),("f2.json",R+"/🧬️schema/🔣️.json"),("f3.ts",D+"/🟦️.ts"),("f4.json",D+"/🧬️schema/🔣️.json"),("f5.ts",D+"/🔌️real/🟦️.ts"),("f6.ts",D+"/🖼️image-slack/🟦️.ts"),("f7.ts",X+"/📜️script.ts"),("f8.json",X+"/📋️project.json"),("f9.ts",D+"/🗃️replica-store/🟦️.ts")]
new=[("ready-law.ts",R+"/🧪️tests/💽️disk-recovery/🟦️.ts"),("ready-oracle.py",R+"/🧪️tests/💽️disk-recovery/🐍️.py"),("need-law.ts",D+"/🧪️tests/📥️need/🟦️.ts"),("reclaim-law.ts",D+"/🖼️image-slack/🧪️tests/♻️reclaim/🟦️.ts")]
run=S+"/run";shutil.rmtree(run,ignore_errors=True);os.makedirs(run)
m={};rows=[]
for f,live in mod:
    src=S+("/stage/" if mode=="staged" else "/base/")+f;dst=run+"/"+f;shutil.copy(src,dst);m[live]=dst;rows.append((live,S+"/stage/"+f,S+"/base/"+f))
for f,live in new:
    dst=run+"/"+("🐍️.py" if f.endswith(".py") else f);shutil.copy(S+"/new/"+f,dst)
    m[live]=dst
    rows.append((live,S+"/new/"+f,""))
json.dump(m,open(S+"/map.json","w"),ensure_ascii=False,indent=1)
open(S+"/land.tsv","w").write("\n".join("\t".join(r) for r in rows)+"\n")
