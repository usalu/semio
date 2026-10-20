# 🔬 For every descriptor whose semanticKind differs from its directory slug, lists the text files that name the directory (bounded by / or quotes). usage: python3 identity-refs.py <findings-s.json> <out.json>
import json,os,re,subprocess,sys,collections,unicodedata
rows=[v for v in json.load(open(sys.argv[1],encoding="utf8")) if v["code"]=="mutation-payload-schema-authority-invalid" and "Descriptor version" in v["message"]]
plugins=collections.defaultdict(list)
for v in rows:
    owner=os.path.dirname(v["path"]); sk=json.load(open(v["path"]))["semanticKind"]
    plugins["/".join(v["path"].split("/")[:3])].append((owner,sk))
out=[]
for plugin,items in plugins.items():
    files=[f for f in subprocess.run(["git","ls-files","-co","--exclude-standard","--",plugin],capture_output=True,text=True).stdout.split("\n") if f.endswith((".rs",".ts",".toml",".py",".json",".feature",".md",".proto",".graphql")) and "node_modules" not in f and "/target/" not in f and os.path.isfile(f)]
    texts={}
    for f in files:
        try: texts[f]=open(f,encoding="utf8").read()
        except Exception: pass
    for owner,sk in items:
        name=os.path.basename(owner)
        pat=re.compile(r'(?<![^/"\s(])'+re.escape(name)+r'(?=[/"\s)]|$)')
        hits=[f for f,t in texts.items() if pat.search(t)]
        out.append({"owner":owner,"name":name,"semanticKind":sk,"refs":hits})
        print(len(hits),owner.split("🪆️subsets/")[-1][:80],"->",sk)
json.dump(out,open(sys.argv[2],"w",encoding="utf8"),ensure_ascii=False,indent=1)
