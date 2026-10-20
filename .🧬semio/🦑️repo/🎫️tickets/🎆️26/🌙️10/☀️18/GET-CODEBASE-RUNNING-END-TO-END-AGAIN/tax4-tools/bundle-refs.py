# 🔬 For each remaining mutation-fixture-unpaired bundle, lists the files outside the fixtures that name `<mutation>/<case>` (FE0F-insensitive). usage: python3 bundle-refs.py <findings-s.json> <out.json>
import json,os,re,subprocess,sys,collections,unicodedata
fold=lambda s:unicodedata.normalize("NFC",s).replace("️","")
rows=[v for v in json.load(open(sys.argv[1],encoding="utf8")) if v["code"]=="mutation-fixture-unpaired"]
M="/🧪️tests/🧫️fixtures/🧬️mutations/"
bysub=collections.defaultdict(list)
for v in rows:
    sub,rel=v["path"].split(M,1); bysub[sub].append(rel)
out=[]
for sub,rels in bysub.items():
    files=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",sub],capture_output=True,text=True).stdout.split("\n") if f and os.path.isfile(f) and f.endswith((".rs",".ts",".py",".feature",".json",".md")) and "/target/" not in f and "node_modules" not in f and "/🧫️fixtures/" not in f]
    texts={}
    for f in files:
        try: texts[f]=fold(open(f,encoding="utf8").read())
        except Exception: pass
    for rel in rels:
        key=fold(rel)
        hits=[f.replace(sub+"/","") for f,t in texts.items() if key in t]
        out.append({"subset":sub,"bundle":rel,"refs":hits})
json.dump(out,open(sys.argv[2],"w",encoding="utf8"),ensure_ascii=False,indent=1)
c=collections.Counter((o["subset"].split("/")[2], bool(o["refs"])) for o in out)
for k,v in sorted(c.items()): print(k,v)
