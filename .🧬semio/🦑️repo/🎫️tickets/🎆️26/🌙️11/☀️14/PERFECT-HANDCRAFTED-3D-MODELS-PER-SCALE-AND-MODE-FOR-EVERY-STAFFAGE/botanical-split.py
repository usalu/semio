"""🌿 Run the native print and scale rules in independent memory-bounded jobs."""
from hashlib import sha256
from pathlib import Path
from runpy import run_path
import json
import os
import shutil
import subprocess
import sys

ticket = Path(__file__).resolve().parent
path, which = Path(sys.argv[1]), sys.argv[2]
raw = path.read_bytes()
asset = json.loads(raw)["asset"]
proof = sha256(raw).hexdigest()
out = ticket/"🗑️generated"/"botanical-split"
out.mkdir(exist_ok=True)
bun = shutil.which("bun")
modules = next(ticket.parents[6].glob("🎓*/*architecture/*staffage/*modules"))
signature = sha256()
for p in sorted(modules.rglob("*.ts")):
    signature.update(p.read_bytes())
for name in ["digital-paper-candidate.ts","digital-scale-proof.ts","r3-validate.ts","living-silhouette-view.ts","botanical-split.py","botanical-paper-slice.ts","digital-bounded-proof.py"]:
    signature.update((ticket/name).read_bytes())
code = signature.hexdigest()
if which == "document":
    problems = run_path(str(ticket/"digital-bounded-proof.py"))["measure"](bun,path,"document",code)
    if problems is None:
        problems=[]
        semantic=subprocess.run([sys.executable,ticket/"equipment-guard.py",bun,"--smol",ticket/"r3-validate.ts","--file",path,"--compact","--semantics"],env=dict(os.environ,EQUIPMENT_RSS_MIB="1050",EQUIPMENT_HEADROOM_PERCENT="42"),capture_output=True,text=True)
        if semantic.returncode not in (0,1):sys.exit(semantic.returncode)
        problems.extend(line for line in semantic.stdout.splitlines() if line.startswith(("ASSET ","SCHEMA ","PRINT ")))
        assert any(line.startswith(("VALID ","INVALID ")) for line in semantic.stdout.splitlines())
        for k,rep in asset["representations"].items():
            if rep["status"]!="authored":continue
            ends=sorted({int(k),rep["validRange"]["minDenominator"],rep["validRange"]["maxDenominator"]})
            for end in ends:
                cached=ticket/"🗑️generated"/"split-rules"/f"{asset['id']}-{proof[:12]}-{code[:12]}-print-{k}-{end}.json"
                if cached.exists():
                    row=json.loads(json.loads(cached.read_text())["output"])
                    if not row["suitable"]:problems.extend(row["violations"])
                    continue
                for pose in [p["id"] for p in asset["poses"] if not p.get("key")]:
                    for view in ["front","plan","elevation"]:
                        sub=out/f"{asset['id']}-{proof[:12]}-{code[:12]}-{k}-{end}-{pose}-{view}.json"
                        if not sub.exists():
                            run=subprocess.run([sys.executable,ticket/"equipment-guard.py",bun,"--smol",ticket/"botanical-paper-slice.ts",path,k,str(end),pose,view],env=dict(os.environ,EQUIPMENT_RSS_MIB="830",EQUIPMENT_HEADROOM_PERCENT="42"),stdout=subprocess.PIPE,text=True)
                            if run.returncode not in (0,1):sys.exit(run.returncode)
                            sub.write_text(run.stdout.strip())
                        row=json.loads(sub.read_text())
                        if not row["suitable"]:problems.extend(row["violations"])
    for p in problems:print(p)
    print(("INVALID" if problems else "VALID")+" file/"+asset["id"]+" (native schema, semantics, print nominal and band ends): schema and validateAsset "+str(len(problems)))
    sys.exit(1 if problems else 0)
groups = [
    ("source","availability,feature,bones,solids,copied,slender,monotonic,views-disagree",645),
    ("bounds","bounds",680),
    ("pair","cross-scale",1000),
    ("heat","thermal-coverage",760),
    ("structure","structure-coverage",675),
    ("elevation","silhouette",1000),
    ("front","silhouette",1000),
    ("plan","silhouette",1000),
]
rules, issues = set(), []
for tag, names, cap in groups:
    target = out/("-".join([asset["id"],proof[:12],code[:12],which,tag])+".json")
    selected = ["volumes-"+key for key in names.split(",")]
    rules.update(selected)
    if not target.exists():
        view = names == "silhouette"
        prog = "living-silhouette-view.ts" if view else "digital-scale-proof.ts"
        args = [asset["id"],which,tag,path,target] if view else [path,which,",".join(selected)]
        env = dict(os.environ,EQUIPMENT_RSS_MIB=str(cap),EQUIPMENT_HEADROOM_PERCENT="42")
        command = [sys.executable,ticket/"equipment-guard.py",bun,"--smol",ticket/prog,*args]
        run = subprocess.run(command,env=env,stdout=subprocess.PIPE,text=True)
        if run.returncode==75:
            run = subprocess.run(command,env=dict(env,EQUIPMENT_RSS_MIB="1100"),stdout=subprocess.PIPE,text=True)
        if run.returncode not in (0,1):sys.exit(run.returncode)
        if not view:
            body=json.loads(run.stdout.strip().splitlines()[-1])
            assert set(body["rules"])==set(selected)
            target.write_text(json.dumps(body))
    result=json.loads(target.read_bytes())
    if names=="silhouette":assert result["hash"]==proof
    issues.extend(result.get("issues",result.get("findings",[])))
    print("[DEBUG]",which,tag,len(issues),flush=True)
assert len(rules)==13
print(json.dumps({"asset":asset["id"],"scale":int(which),"rules":sorted(rules),"issues":issues}))
sys.exit(1 if issues else 0)
