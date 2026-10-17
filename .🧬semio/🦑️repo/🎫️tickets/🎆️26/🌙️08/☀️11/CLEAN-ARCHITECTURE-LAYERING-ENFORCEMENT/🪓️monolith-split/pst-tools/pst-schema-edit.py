import json,sys,os
# library: load a schema json, check that json.dumps round-trips, apply an edit fn, write back with snapshot
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ROOT="/Users/ueli/Documents/semio/"
def snap(label,rels):
    import subprocess
    lst=ROOT+F+"/🗑️generated/pst-snaplist.txt"
    open(lst,"w",encoding="utf8").write("\n".join(rels)+"\n")
    subprocess.run(["python3",ROOT+F+"/pst-tools/pst-snap.py",label,lst],check=True)
def edit(rel,fn,label):
    path=ROOT+rel
    raw=open(path,encoding="utf8").read()
    d=json.loads(raw)
    for indent in (2,4,1):
        if json.dumps(d,indent=indent,ensure_ascii=False)+"\n"==raw: break
        if json.dumps(d,indent=indent,ensure_ascii=False)==raw.rstrip("\n"): break
    else:
        raise SystemExit("format does not round-trip: "+rel)
    snap(label,[rel])
    fn(d)
    out=json.dumps(d,indent=indent,ensure_ascii=False)+("\n" if raw.endswith("\n") else "")
    open(path,"w",encoding="utf8").write(out)
