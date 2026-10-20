# 🔧 Sets every energy leaf-test `directory: "…"` literal to the fixture bundle mirroring the test's own schema leaf path. usage: python3 energy-directory-literals.py [--apply]
import re,os,subprocess,sys
S="✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any"
F=f"{S}/🧪️tests/🧫️fixtures/🧬️mutations"; M=f"{S}/🧬️schema/🧬️mutations/"
fs=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",S],capture_output=True,text=True).stdout.split("\n") if f.endswith(".rs") and f.startswith(M) and "/🧪️tests/" in f]
rows=[]
for f in fs:
    rel=f[len(M):]
    if "/🧪️tests/" not in rel: continue
    leaf,rest=rel.split("/🧪️tests/",1)
    case=rest.split("/")[0]
    t=open(f,encoding="utf8").read()
    for m in re.finditer(r'directory: "([^"]+)"',t):
        want=f"{leaf}/{case}"
        if m.group(1)!=want:
            ok=os.path.isdir(f"{F}/{want}")
            rows.append((f,m.group(1),want,ok))
print(len(rows),"stale;",sum(1 for r in rows if not r[3]),"with no bundle at the mirrored path")
for r in rows:
    if not r[3]: print("  NO BUNDLE",r[2])
if "--apply" in sys.argv:
    for f,old,want,ok in rows:
        if not ok: continue
        t=open(f,encoding="utf8").read()
        open(f,"w",encoding="utf8").write(t.replace(f'directory: "{old}"',f'directory: "{want}"'))
    print("applied")
