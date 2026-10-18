import sys,os,subprocess,shutil,json,re
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
E="/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine"
label=sys.argv[1]; names=sys.argv[2:]
files=[]
for n in names:
    p=f"{E}/🧪️tests/{n}/🟦️.ts"
    if not os.path.exists(p): p=f"{E}/🧪️tests/{n}/🟦️.tsx"
    files.append(p)
backups={}
for p in files:
    b=f"{F}/tsr-tools/orig/{os.path.basename(os.path.dirname(p))}.bak"
    shutil.copy2(p,b); backups[p]=b
try:
    for p in files:
        t=open(p,encoding="utf8").read()
        t=re.sub(r'\bexpect\((?=[^\n]*\.(?:toContain|toMatch|not\.toContain|not\.toMatch)\()','expect.soft(',t)
        open(p,"w",encoding="utf8").write(t)
    env=dict(os.environ,TARGET="🧊️wgpu")
    subprocess.run([f"{F}/tsr-tools/vitest.sh",label]+names,env=env)
finally:
    for p,b in backups.items(): shutil.copy2(b,p)
d=json.load(open(f"{F}/🗑️generated/tsr-vt-{label}.json",encoding="utf8"))
print(d["numPassedTests"],d["numFailedTests"])
for f in d["testResults"]:
    for a in f["assertionResults"]:
        if a["status"]=="failed":
            print("##",f["name"].split("🧪️tests/")[-1][:40],"|",a["title"][:80])
            for m in a["failureMessages"]:
                for line in m.split("\n"):
                    if line.startswith(("AssertionError","Error","TypeError")): print("    ",line[:300])
