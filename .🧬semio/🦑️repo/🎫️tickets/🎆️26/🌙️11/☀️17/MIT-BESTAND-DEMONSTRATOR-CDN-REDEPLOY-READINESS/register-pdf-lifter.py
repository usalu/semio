"""🧱️ Registers the carved PDF lifter crate in the pdf workspace, the pdf script, the build owners, the path fixture and every lock file that locks the lift crate."""
import os, re, json, subprocess
os.chdir("/Users/ueli/Documents/semio")
PDF="✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf"
MOD="🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🔨️modules"
NEW="semio-s-artifact-stdio-pdf-lifter"
LIFTER=f"{PDF}/{MOD}/⬇️lift/🧱️lifter/📦️packages/🦀️rust"
read=lambda p:open(p,encoding="utf8").read()
def write(p,t): open(p,"w",encoding="utf8").write(t)
def swap(p,old,new):
    t=read(p); assert t.count(old)==1,(p,old,t.count(old)); write(p,t.replace(old,new))
a=f'    "{MOD}/⬇️lift/🧩️lifter-and-document/📜️page-content/📦️packages/🦀️rust",\n'
swap(f"{PDF}/Cargo.toml",a,a+f'    "{MOD}/⬇️lift/🧱️lifter/📦️packages/🦀️rust",\n')
swap(f"{PDF}/📦️packages/🦀️rust/📜️script.ts",'"semio-s-artifact-stdio-pdf-lift", ','"semio-s-artifact-stdio-pdf-lift", "semio-s-artifact-stdio-pdf-lifter", ')
p="📋️build-owners.json"; t=read(p)
anchor='    {\n      "name": "semio-s-artifact-stdio-pdf-lower",'; assert t.count(anchor)==1
write(p,t.replace(anchor,f'    {{\n      "name": "{NEW}",\n      "root": "{LIFTER}",\n      "manifest": "{LIFTER}/Cargo.toml",\n      "packages": []\n    }},\n'+anchor))
p="🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📏️paths/🧪️tests/🧫️fixtures/🔣️.json"; t=read(p)
anchor='    {\n      "packageName": "semio-s-artifact-stdio-pdf-lower",'; assert t.count(anchor)==1
t=t.replace(anchor,f'    {{\n      "packageName": "{NEW}",\n      "crateName": "{NEW.replace("-","_")}",\n      "platform": "wasm32-wasip2",\n      "profile": "prototype"\n    }},\n'+anchor); json.loads(t); write(p,t)
DEPS=["semio-s-artifact-stdio-pdf-conformance-support","semio-s-artifact-stdio-pdf-diff","semio-s-artifact-stdio-pdf-fonts","semio-s-artifact-stdio-pdf-inferences","semio-s-artifact-stdio-pdf-model","semio-s-artifact-stdio-pdf-mut-base-4","semio-s-artifact-stdio-pdf-mut-family-5","semio-s-artifact-stdio-pdf-page-content","semio-s-artifact-stdio-pdf-page-tree","semio-s-artifact-stdio-pdf-snapshot-2"]
changed=[];skipped=[]
files=subprocess.run("find . -name Cargo.lock -not -path '*/target/*' -not -path '*/node_modules/*' -not -path './.🧬semio/*'",shell=True,capture_output=True,text=True).stdout.split("\n")
for f in files:
    if not f: continue
    s=read(f)
    if 'name = "semio-s-artifact-stdio-pdf-lift"\n' not in s or f'name = "{NEW}"\n' in s: continue
    blocks=s.split("\n[[package]]\n"); head,pk=blocks[0],blocks[1:]
    name=lambda b:re.search(r'name = "([^"]+)"',b).group(1)
    names=[name(b) for b in pk]; i=names.index("semio-s-artifact-stdio-pdf-lift")
    m=re.search(r'dependencies = \[\n(.*?)\n\]',pk[i],re.S); deps=set(d.strip().strip(',"') for d in m.group(1).split("\n"))
    if not set(DEPS)<=deps: skipped.append((f,sorted(set(DEPS)-deps))); continue
    deps=sorted(deps|{NEW})
    pk[i]=pk[i][:m.start(1)]+"\n".join(f' "{d}",' for d in deps)+pk[i][m.end(1):]
    names=[name(b) for b in pk]; pos=next(k for k,x in enumerate(names) if x>NEW)
    pk.insert(pos,f'name = "{NEW}"\nversion = "0.1.0"\ndependencies = [\n'+"\n".join(f' "{x}",' for x in DEPS)+'\n]\n')
    write(f,head+"".join("\n[[package]]\n"+b for b in pk)); changed.append(f)
print(len(changed),"locks changed");print(skipped)
