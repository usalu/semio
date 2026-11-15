#!/usr/bin/env python3
"""Lands the working copy of the service entry onto the live module: only files that changed against base.sha, each replaced only while the live file is byte-identical to the base (else the landing stops before writing anything), originals saved to backup/ first, writes through tmp + rename, scripts last.
usage: land.py plan | apply | revert   (apply and revert expect the landing lock to be held by the caller)"""
import hashlib,json,os,sys

T="/Users/Shared/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️09/SHARED-WORKTREE-FLEET-OVER-NFS"
M="🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree"
LIVE=os.path.join("/Users/Shared/semio",M)
WORK=os.path.join(T,"🧰️scripts/1113-entry/work",M)
BASE=os.path.join(T,"🧰️scripts/1113-entry/base.sha")
BACKUP=os.path.join(T,"🧰️scripts/1113-entry/backup")
RECORD=os.path.join(T,"🧰️scripts/1113-entry/landing.json")
LAST=["🔁️replica/🟦️.ts","🔁️replica/⌨️command/🟦️.ts","🔁️replica/📜️script.ts"]

def sha(path):
    return hashlib.sha256(open(path,"rb").read()).hexdigest()

def base_hashes():
    result={}
    for line in open(BASE,encoding="utf8"):
        digest,path=line.rstrip("\n").split("  ",1)
        result[path[2:]]=digest
    return result

def walk(root):
    found=[]
    for directory,names,files in os.walk(root):
        names[:]=[n for n in names if n!="node_modules" and not os.path.islink(os.path.join(directory,n))]
        for name in files:
            full=os.path.join(directory,name)
            if os.path.islink(full): continue
            found.append(os.path.relpath(full,root))
    return sorted(found)

def plan():
    base=base_hashes(); changes=[]
    for rel in walk(WORK):
        digest=sha(os.path.join(WORK,rel))
        if rel not in base: changes.append(("new",rel,None,digest))
        elif base[rel]!=digest: changes.append(("modified",rel,base[rel],digest))
    order=lambda c:(c[0]=="modified",c[1] in LAST,LAST.index(c[1]) if c[1] in LAST else 0,c[1])
    return sorted(changes,key=order)

def write_atomic(path,data):
    os.makedirs(os.path.dirname(path),exist_ok=True)
    temporary=path+".entry-landing.tmp"
    with open(temporary,"wb") as handle: handle.write(data)
    os.replace(temporary,path)

def apply():
    changes=plan(); problems=[]
    for kind,rel,before,after in changes:
        live=os.path.join(LIVE,rel)
        if kind=="modified":
            if not os.path.exists(live): problems.append("missing live "+rel)
            elif sha(live)[:len(before)]!=before[:len(before)] and sha(live)!=before: problems.append("live changed "+rel)
        elif os.path.exists(live) and sha(live)!=after: problems.append("exists live "+rel)
    if problems:
        print("STOP, nothing written:"); print("\n".join(problems)); sys.exit(1)
    record=[]
    for kind,rel,before,after in changes:
        live=os.path.join(LIVE,rel)
        if os.path.exists(live):
            write_atomic(os.path.join(BACKUP,rel),open(live,"rb").read())
    for kind,rel,before,after in changes:
        live=os.path.join(LIVE,rel)
        old=sha(live) if os.path.exists(live) else None
        write_atomic(live,open(os.path.join(WORK,rel),"rb").read())
        if os.path.exists(os.path.join(WORK,rel)) and os.access(os.path.join(WORK,rel),os.X_OK): os.chmod(live,os.stat(os.path.join(WORK,rel)).st_mode&0o777)
        record.append({"kind":kind,"path":rel,"before":old,"after":sha(live)})
        print(kind,rel,(old or "-")[:16],sha(live)[:16])
    json.dump(record,open(RECORD,"w",encoding="utf8"),ensure_ascii=False,indent=1)

def revert():
    record=json.load(open(RECORD,encoding="utf8"))
    for item in reversed(record):
        live=os.path.join(LIVE,item["path"])
        if item["kind"]=="new":
            if os.path.exists(live): os.remove(live)
        else:
            write_atomic(live,open(os.path.join(BACKUP,item["path"]),"rb").read())
        print("reverted",item["path"])

if __name__=="__main__":
    command=sys.argv[1] if len(sys.argv)>1 else "plan"
    if command=="plan":
        for kind,rel,before,after in plan(): print(kind,rel,(before or "-")[:16],after[:16])
    elif command=="apply": apply()
    elif command=="revert": revert()
