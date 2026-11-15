#!/usr/bin/env python3
"""Rebases the working copy onto the live module: files I changed are three-way merged (base from the content-addressed blob store), files I did not change are copied from live, then base.sha is rewritten from live. Prints conflicts and stops on them."""
import hashlib,os,shutil,subprocess,sys,tempfile
T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
LIVE=os.path.join("/Users/Shared/semio",M); WORK=os.path.join(T,"🧰️scripts/1113-entry/work",M); BASE=os.path.join(T,"🧰️scripts/1113-entry/base.sha"); STORE="/Users/Shared/semio-store/v1/blobs"
sha=lambda p:hashlib.sha256(open(p,"rb").read()).hexdigest()
base={}
for line in open(BASE,encoding="utf8"):
    d,p=line.rstrip("\n").split("  ",1); base[p[2:]]=d
def walk(root):
    out=[]
    for dp,dn,fn in os.walk(root):
        dn[:]=[n for n in dn if n!="node_modules" and not os.path.islink(os.path.join(dp,n))]
        for n in fn:
            f=os.path.join(dp,n)
            if not os.path.islink(f): out.append(os.path.relpath(f,root))
    return sorted(out)
live_files=walk(LIVE); work_files=set(walk(WORK)); conflicts=[]
for rel in live_files:
    lp=os.path.join(LIVE,rel); wp=os.path.join(WORK,rel)
    if rel in base and rel in work_files:
        if rel=="🔁️replica/🟦️.ts": continue
        mine=sha(wp)!=base[rel]; theirs=sha(lp)!=base[rel]
        if not theirs: continue
        if not mine:
            shutil.copyfile(lp,wp); print("copied",rel); continue
        blob=os.path.join(STORE,base[rel][:2],base[rel])
        if not os.path.exists(blob): print("NO BLOB",rel); conflicts.append(rel); continue
        merged=subprocess.run(["git","merge-file","-p",lp,blob,wp],capture_output=True)
        if merged.returncode!=0: print("CONFLICT",rel,merged.returncode); conflicts.append(rel); continue
        open(wp,"wb").write(merged.stdout); print("merged",rel)
    elif rel not in work_files:
        os.makedirs(os.path.dirname(wp),exist_ok=True); shutil.copyfile(lp,wp); print("added from live",rel)
    elif rel not in base:
        print("live has a new file that I also have",rel)
if conflicts: print("conflicts:",conflicts); sys.exit(1)
with open(BASE,"w",encoding="utf8") as handle:
    for rel in live_files: handle.write(f"{sha(os.path.join(LIVE,rel))}  ./{rel}\n")
print("base.sha rewritten from live")
