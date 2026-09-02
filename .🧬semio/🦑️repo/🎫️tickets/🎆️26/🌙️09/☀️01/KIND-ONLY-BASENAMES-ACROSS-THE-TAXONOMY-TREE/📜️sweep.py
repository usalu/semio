"""🌳️ Sweep every remaining non-fixture leaf breach to its registered kind-only basename."""
import json,os,collections,subprocess,sys
APPLY="--apply" in sys.argv
TAX="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"
d=json.load(open(TAX,encoding='utf8'))
canon={f"{v['emoji']}{e}" for v in d["fileKinds"].values() for e in v["extensionChains"]}
byemoji={}
for v in d["fileKinds"].values(): byemoji.setdefault(v['emoji'],set()).update(v["extensionChains"])
SKIP_SUB=("target","node_modules",".git","dist","build","storybook-static","pkg","__pycache__","coverage",".nx")
tracked=set(subprocess.run(["git","ls-files"],capture_output=True,text=True).stdout.splitlines())
plan=[]
for tree in ["✏️s","🧰️framework","🌎️hub"]:
    for dp,dn,fn in os.walk(tree):
        dn[:]=[x for x in dn if not any(s in x for s in SKIP_SUB)]
        segs=set(dp.split(os.sep))
        if segs & {"🖼️assets","🔤️fonts","🔣️icons"} or any(s.startswith("🔤️") or s.startswith("😀️") for s in segs): continue
        if segs & {"🧫️fixtures","📚️examples"}: continue          # specimen corpora: separate remedy
        for n in fn:
            rel=os.path.join(dp,n)
            if n in canon or rel not in tracked: continue
            for em,exts in byemoji.items():
                if not n.startswith(em): continue
                rest=n[len(em):]
                for e in sorted(exts,key=len,reverse=True):
                    if rest.endswith(e) and rest!=e:
                        tgt=f"{em}{e}"
                        sib=[x for x in fn if x!=n and x.startswith(em) and x.endswith(e) and x!=tgt]
                        if not sib and tgt in canon and not os.path.exists(os.path.join(dp,tgt)):
                            plan.append((rel,os.path.join(dp,tgt),n,tgt))
                        break
                break
print(f"{'APPLY' if APPLY else 'DRY-RUN'}: {len(plan)} files -> kind-only basenames")
if not APPLY:
    for s,_,n,t in plan[:6]: print(f"   {n} -> {t}   ({os.path.dirname(s)[:70]})")
    sys.exit()
for s,dst,_,_ in plan: subprocess.run(["mv",s,dst],check=True)
# repoint references repo-wide, keyed on the unique emoji token
TEXT={".rs",".ts",".tsx",".js",".mjs",".cjs",".json",".toml",".md",".py",".yaml",".yml",".graphql",".proto",".semio",".wit",".html",".css",".cs",".go",".code-workspace"}
FROZEN={"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json",
        "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🧫️fixtures/🧪️cad-draw-path-projection/🔣️.json"}
pairs=collections.OrderedDict()
for _,_,old,new in plan: pairs[old]=new
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
print(f"renamed {len(plan)}; repointed {subs} refs in {files} files")
