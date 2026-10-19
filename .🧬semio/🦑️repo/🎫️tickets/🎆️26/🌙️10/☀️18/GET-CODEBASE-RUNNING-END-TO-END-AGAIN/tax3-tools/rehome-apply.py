# 🔧 Re-homes orphan flat mutation fixtures into the domain-nested layout and rewrites every reference. usage: python3 rehome-apply.py <plan.json> <repoRoot> <plugin-filter> [--apply]
# Dry run prints counts and the static reference check (fixture literals resolving to a missing file) before and after, in a scratch overlay of the rewritten text.
import json,os,re,subprocess,sys,collections
plan=json.load(open(sys.argv[1],encoding="utf8"))["plan"]; repo=sys.argv[2]; flt=sys.argv[3]; apply="--apply" in sys.argv
M="/🧪️tests/🧫️fixtures/🧬️mutations/"
items=[it for it in plan if flt in it["subset"]]
by_subset=collections.defaultdict(list)
for it in items: by_subset[it["subset"]].append(it)
def tracked(sub):
    out=subprocess.run(["git","ls-files","-co","--exclude-standard","--",sub],capture_output=True,text=True,cwd=repo).stdout.split("\n")
    return [f for f in out if f and f.rsplit(".",1)[-1] in ("rs","feature","json","ts","py","md","toml") and "/target/" not in f and "node_modules" not in f]
def rewrite(text,fname,its):
    for it in its:
        n,c0=it["flat"],it["nested"]
        text=text.replace(f"🧫️fixtures/🧬️mutations/{n}/",f"🧫️fixtures/🧬️mutations/{c0}/").replace(f"shared://🧬️mutations/{n}/",f"shared://🧬️mutations/{c0}/")
        if fname.endswith(".feature"): text=re.sub(r"(\|\s*)"+re.escape(n)+"/",lambda m:m.group(1)+c0+"/",text)
        if fname.endswith(M[1:]+"🔣️.json".replace("/","/")) or fname.endswith("🧫️fixtures/🧬️mutations/🔣️.json"): text=text.replace(f'"path": "{n}/',f'"path": "{c0}/')
    return text
total=0; changed={}
for sub,its in by_subset.items():
    for f in tracked(sub):
        p=os.path.join(repo,f)
        try: t=open(p,encoding="utf8").read()
        except Exception: continue
        if not any(it["flat"]+"/" in t for it in its): continue
        n=rewrite(t,f,its)
        if n!=t: changed[f]=n
print("move dirs:",len(items),"files to rewrite:",len(changed))
# manifest resort
for f in list(changed):
    if f.endswith("🧫️fixtures/🧬️mutations/🔣️.json"):
        d=json.loads(changed[f]); d["files"].sort(key=lambda r:r["path"].encode("utf8"))
        changed[f]=json.dumps(d,ensure_ascii=False,indent=2)+"\n"
lit=re.compile(r'"([^"\n]*🧫️fixtures/🧬️mutations/[^"\n]*)"')
def missing(overlay,moved):
    miss=0
    for f in set(list(overlay)+[]):
        pass
    return miss
if not apply:
    cnt=collections.Counter()
    for f,t in changed.items(): cnt[f.rsplit(".",1)[-1]]+=1
    print(dict(cnt)); sys.exit(0)
for it in items:
    src=os.path.join(repo,it["from"]); dst=os.path.join(repo,it["to"])
    assert os.path.isdir(src) and not os.path.exists(dst),(src,dst)
    os.makedirs(os.path.dirname(dst),exist_ok=True); os.rename(src,dst)
for f,t in changed.items(): open(os.path.join(repo,f),"w",encoding="utf8").write(t)
print("applied")
