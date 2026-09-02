"""🧪️ Move named fixture specimens into registered 🧪️<slug>/ dirs with kind-only basenames."""
import json,os,re,subprocess,sys,collections
APPLY="--apply" in sys.argv
TAX="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
d=json.load(open(TAX,encoding='utf8'))
canon={f"{v['emoji']}{e}" for v in d["fileKinds"].values() for e in v["extensionChains"]}
byemoji={}
for v in d["fileKinds"].values(): byemoji.setdefault(v['emoji'],set()).update(v["extensionChains"])
SKIP_SUB=("target","node_modules",".git","dist","build","storybook-static","pkg","__pycache__","coverage",".nx")
tracked=set(subprocess.run(["git","ls-files"],capture_output=True,text=True).stdout.splitlines())
SLUG=re.compile(r"^[^\W_]+(?:-[^\W_]+)*$",re.UNICODE)
plan=[]
for tree in ["✏️s","🧰️framework","🌎️hub"]:
    for dp,dn,fn in os.walk(tree):
        dn[:]=[x for x in dn if not any(s in x for s in SKIP_SUB)]
        segs=set(dp.split(os.sep))
        if not (segs & {"🧫️fixtures","📚️examples"}): continue
        for n in fn:
            rel=os.path.join(dp,n)
            if n in canon or rel not in tracked: continue
            for em,exts in byemoji.items():
                if not n.startswith(em): continue
                rest=n[len(em):]
                for e in sorted(exts,key=len,reverse=True):
                    if rest.endswith(e) and rest!=e:
                        slug=rest[:-len(e)]
                        if SLUG.match(slug): plan.append((rel,dp,f"🧪️{slug}",f"{em}{e}",n))
                        break
                break
print(f"{'APPLY' if APPLY else 'DRY-RUN'}: {len(plan)} specimens")
if not APPLY:
    for rel,dp,sd,tgt,n in plan[:5]: print(f"   {n} -> {sd}/{tgt}")
    sys.exit()
pairs=collections.OrderedDict()
for rel,dp,sd,tgt,n in plan:
    os.makedirs(os.path.join(dp,sd),exist_ok=True)
    subprocess.run(["mv",rel,os.path.join(dp,sd,tgt)],check=True)
    pairs[n]=f"{sd}/{tgt}"
TEXT={".rs",".ts",".tsx",".js",".mjs",".cjs",".json",".toml",".md",".py",".yaml",".yml",".graphql",".proto",".semio",".html",".cs",".go"}
FROZEN={"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json"}
subs=files=0
for dp,dn,fn in os.walk("."):
    dn[:]=[x for x in dn if not any(s in x for s in SKIP_SUB) and not x.startswith(".git")]
    if ".🧬semio" in dp.split(os.sep) or "♻️mit-bestand" in dp or ".cursor" in dp.split(os.sep): continue
    for n in fn:
        if os.path.splitext(n)[1] not in TEXT: continue
        p=os.path.join(dp,n)
        if os.path.relpath(p,".") in FROZEN: continue
        try: s=open(p,encoding="utf8").read()
        except Exception: continue
        o=s; c=0
        for old,new in pairs.items():
            if old in s: c+=s.count(old); s=s.replace(old,new)
        if s!=o: open(p,"w",encoding="utf8").write(s); subs+=c; files+=1
print(f"moved {len(plan)}; repointed {subs} refs in {files} files")
