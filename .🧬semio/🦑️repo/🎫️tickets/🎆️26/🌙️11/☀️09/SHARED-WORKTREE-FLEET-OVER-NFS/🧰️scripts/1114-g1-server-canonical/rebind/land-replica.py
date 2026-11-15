import hashlib,os,sys,subprocess,shutil,tempfile
S=os.path.dirname(os.path.abspath(__file__));ROOT=sys.argv[1]
R=ROOT+"/🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica/📍️relocate"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
for live,base,mine in[(R+"/🟦️.ts","relocate.base.ts.txt","relocate.mine.ts.txt"),(R+"/🧪️tests/🟦️.ts","relocate-test.base.ts.txt","relocate-test.mine.ts.txt")]:
    before=sha(live);tmp=live+".land-tmp";shutil.copy(live,tmp)
    r=subprocess.run(["git","merge-file",tmp,S+"/"+base,S+"/"+mine])
    if r.returncode!=0:os.unlink(tmp);raise SystemExit(f"merge conflict in {live}: nothing written for it")
    os.replace(tmp,live);print(live.replace(ROOT+"/",""),before[:12],"->",sha(live)[:12],"(live was base)" if before==sha(S+"/"+base) else "(three-way merged)")
