"""🧩 Check one equipment document and its four digital scales within a bounded slot."""
import json
import os
from pathlib import Path
import subprocess
import sys
from runpy import run_path
import time

ticket=Path(__file__).resolve().parent
bun,version,digest,source,seconds=sys.argv[1:];path=Path(source)
cache=ticket/"🗑️generated/equipment-fleet"/(path.parent.name+"-shards.json")
stored=json.loads(cache.read_bytes()) if cache.exists() else {}
if stored.get("code")!=version or stored.get("input")!=digest:stored={"code":version,"input":digest,"parts":{},"split":stored.get("split",[]) if stored.get("input")==digest else []}
parts=stored["parts"];until=time.monotonic()+float(seconds) if float(seconds)>0 else float("inf")
read_view=run_path(str(ticket/"digital-view-results.py"))["issue_lines"]
def save():
    lockfile=cache.with_suffix(".lock")
    with lockfile.open("a+b") as lock:
        if os.name=="nt":
            import msvcrt
            lock.seek(0);msvcrt.locking(lock.fileno(),msvcrt.LK_LOCK,1)
        else:
            import fcntl
            fcntl.flock(lock,fcntl.LOCK_EX)
        prev=json.loads(cache.read_bytes()) if cache.exists() else {}
        if prev.get("code")==version and prev.get("input")==digest:
            for key,lines in prev["parts"].items():parts.setdefault(key,lines)
            stored["split"]=list(dict.fromkeys([*stored["split"],*prev["split"]]))
        tmp=cache.with_suffix("."+str(os.getpid())+".partial")
        tmp.write_text(json.dumps(stored,ensure_ascii=False));os.replace(tmp,cache)
def measure(name,command,scale=None,view=None):
    if name in parts:return True
    if until-time.monotonic()<1:return False
    child=subprocess.Popen([sys.executable,str(ticket/"equipment-guard.py"),bun,"--smol",*map(str,command)],stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
    try:out,err=child.communicate(timeout=None if until==float("inf") else until-time.monotonic())
    except subprocess.TimeoutExpired:
        child.terminate();out,err=child.communicate()
        if scale is not None and name==str(scale) and float(seconds)>=15 and name not in stored["split"]:
            stored["split"].append(name);save()
        print("[DEBUG] tranche ended during "+name)
        return False
    if view is not None:
        dest=Path(command[-1])
        if child.returncode or not dest.exists():return False
        row=json.loads(dest.read_bytes());dest.unlink()
        lines=read_view(row,path.parent.name,digest,scale,view)
    elif name.startswith("paper-"):
        row=next((json.loads(l) for l in out.splitlines() if l.startswith("{")),None)
        if row is None or "suitable" not in row:return False
        lines=["PRINT "+name+" "+json.dumps(v,ensure_ascii=False) for v in row["violations"]]
        if not row["suitable"] and not lines:lines=["PRINT unsuitable "+name]
    elif scale is None:
        if not any(l.startswith(("VALID ","INVALID ")) for l in out.splitlines()):
            if name=="document" and child.returncode==75 and name not in stored["split"]:stored["split"].append(name);save()
            return False
        lines=[l for l in out.splitlines() if l.startswith(("SCHEMA ","ASSET ","PRINT "))]
    else:
        row=next((json.loads(l) for l in out.splitlines() if l.startswith("{")),None)
        if row is None:
            if "paused for headroom" in err and name==str(scale) and name not in stored["split"]:stored["split"].append(name);save()
            return False
        lines=["VOLUMES 1:"+str(scale)+" "+code+" "+loc+" "+detail for code,loc,detail in row["issues"]]
    parts[name]=lines;save()
    return True

name=path.parent.name
category=path.parent.parent.name.lstrip(''.join(c for c in path.parent.parent.name if not c.isascii()))
doc_cmd=[ticket/"r3-validate.ts",category+"/"+name,"--catalog-only","--compact"]
ready="document" not in stored["split"] and measure("document",doc_cmd)
if not ready and "document" in stored["split"]:
    ready=measure("semantics",[*doc_cmd,"--semantics"])
    drawings=json.loads(path.read_bytes())["asset"]["representations"]
    samples=[(d,q) for d,r in drawings.items() if r["status"]=="authored" for q in sorted({int(d),r["validRange"]["minDenominator"],r["validRange"]["maxDenominator"]})]
    del drawings
    for d,q in samples:
        if not ready:break
        ready=measure("paper-"+d+"-"+str(q),[ticket/"digital-paper-candidate.ts",path,d+"/"+str(q)])
rules=None
for scale in (500,200,100,50):
    if not ready:break
    key=str(scale);cmd=[ticket/"digital-scale-proof.ts",path,key]
    if key not in stored["split"] and measure(key,cmd,scale):continue
    if key not in stored["split"]:ready=False;break
    if rules is None:rules=json.loads(subprocess.check_output([bun,"--smol",str(ticket/"digital-scale-proof.ts"),"--rules"],text=True))
    for rule in rules:
        label=key+"-"+rule
        if rule=="volumes-silhouette" and label not in parts:
            for view in ("elevation","front","plan"):
                dest=cache.parent/("view-"+str(os.getpid())+"-"+key+"-"+view+".json")
                if not measure(label+"-"+view,[ticket/"living-silhouette-view.ts",name,key,view,path,dest],scale,view):
                    ready=False;break
            if not ready:break
        elif not measure(label,[*cmd,rule],scale):ready=False;break
if not ready:
    print("[DEBUG] completed shards",",".join(parts))
    raise SystemExit(124)
lines=[line for block in parts.values() for line in block]
print(*lines,sep="\n");print(("INVALID " if lines else "VALID ")+category+"/"+name+" (all live catalog shards): "+str(len(lines))+" findings")
raise SystemExit(int(bool(lines)))
