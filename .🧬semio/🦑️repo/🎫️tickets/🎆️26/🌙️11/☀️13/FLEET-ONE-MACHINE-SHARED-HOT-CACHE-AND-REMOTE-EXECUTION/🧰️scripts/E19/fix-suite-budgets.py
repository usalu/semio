import json,re,subprocess,sys
L="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/"
P=L+"🗂️suites/🔣️.json"
for _ in range(25):
    out=subprocess.run([sys.path[0]+"/t.sh","⚡️caching/🗂️suites/🧪️tests/🟦️.ts","-t","budget classes"],capture_output=True,text=True).stdout
    m=re.search(r"^error: (\S+) starts processes in (.+)$",out,re.M)
    if not m: print("no more violation lines"); print(out[-600:]); break
    sid,files=m.group(1),[f.strip() for f in m.group(2).split(", ")]
    t=open(P).read(); d=json.loads(t)
    target=sid+"-processes"
    row=next((s for s in d["suites"] if s["id"]==target),None)
    if row is None: print("NO PROCESSES ROW for",sid,files); break
    a='      "id": "%s",\n      "budgetMs": %d,\n      "typecheck": "none",\n      "files": [\n'%(target,row["budgetMs"])
    if t.count(a)!=1: print("anchor missing",target); break
    t=t.replace(a,a+"".join('        "%s",\n'%f for f in files if f not in row["files"]))
    open(P,"w").write(t); json.loads(t); print("moved",len(files),"files into",target)
