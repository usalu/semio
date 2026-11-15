import json,re,subprocess,sys,os
os.chdir(os.path.dirname(os.path.abspath(__file__)))
bun=subprocess.check_output("ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1",shell=True,text=True).strip()
S="/Users/ueli/Documents/semio/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation/"
slug=sys.argv[1]; apply="--apply" in sys.argv
out=subprocess.run([bun,"volumes-check.ts","vegetation/"+slug,"--scale","50","--rules","volumes-thermal-coverage","--json"],capture_output=True,text=True).stdout
d=json.loads(out)
fixes={}
for m in d.get("measurements",[]):
    if m.get("scale")==50 and str(m.get("name","")).startswith("areaDeviation:") and not m.get("ok",True):
        mm=re.search(r"exposed ([0-9.]+) m2",m.get("note","") or "")
        if mm: fixes[m["name"].split(":")[1]]=float(mm.group(1))
path=S+slug+"/🔣️.json"
raw=open(path).read()
doc=json.loads(raw); regs=doc['asset']['thermal']['regions']
rep=[]
for r in regs:
    if r['id'] in fixes:
        new=float('%.3g'%fixes[r['id']]); rep.append((r['id'],r['areaM2'],new))
        if apply:
            i=raw.index('"id": "%s"'%r['id'], raw.index('"regions"'))
            j=raw.index('"areaM2":',i)
            k=raw.index(',',j)
            raw=raw[:j]+'"areaM2": %s'%repr(new)+raw[k:]
print(json.dumps(rep))
if apply and rep: open(path,"w").write(raw)
