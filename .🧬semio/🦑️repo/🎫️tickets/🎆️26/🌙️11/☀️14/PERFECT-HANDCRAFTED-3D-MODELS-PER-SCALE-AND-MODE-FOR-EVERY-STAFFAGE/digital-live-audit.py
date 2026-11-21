"""🧭 Verify the shipped staffage documents and retain the input digests for each result."""
from hashlib import sha256
from pathlib import Path
from shutil import which
import json
import os
import re
import subprocess
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
staffage = root / "🎓️teaching/🏛️architecture/🧍️staffage"
folder = ticket / "🗑️generated" / "digital-live"
folder.mkdir(parents=True, exist_ok=True)
bun = which("bun") or next((p for p in (root / ".cache/b/t/m/bun").glob("*/*") if p.name in ("bun","bun.exe")),None)
if not bun:
    raise RuntimeError("Bun is required to load the staffage validation modules.")
by_scale="--by-scale" in sys.argv[1:]
args = [a for a in sys.argv[1:] if a!="--by-scale"]
assets = []
for p in sorted((staffage / "🗂️catalog").glob("*/*/🔣️.json")):
    category = re.sub(r"^[^a-z]+", "", p.parent.parent.name)
    slug = p.parent.name
    if not args or category in args or slug in args:
        assets.append((category,slug,p))
def digest(path):
    return sha256(path.read_bytes()).hexdigest()

from runpy import run_path
validator=run_path(str(ticket/"digital-proof-digest.py"))["module_digest"](root, staffage/"🧬️schema/🔣️.json", ticket/"digital-scale-proof.ts", ticket/"r3-validate.ts")

def run(*args):
    return subprocess.run([sys.executable,str(ticket / "equipment-guard.py"),str(bun),"--smol",*[str(a) for a in args]],
                          cwd=root,stdout=subprocess.PIPE,stderr=subprocess.STDOUT,text=True)


def guarded_check(category,name,path):
    base=[ticket / "r3-validate.ts",category+"/"+name,"--catalog-only","--json"]
    if not by_scale:
        full=run(*base,"--volumes")
        if "paused for headroom" not in full.stdout:return full.returncode,full.stdout.splitlines()
    base_result=run(*base);code=base_result.returncode;lines=base_result.stdout.splitlines()
    rule_names=json.loads(run(ticket / "digital-scale-proof.ts","--rules").stdout)
    for scale in [50,100,200,500]:
        command=[ticket / "digital-scale-proof.ts",path,scale]
        shard=run(*command)
        if "paused for headroom" not in shard.stdout:
            code=code or shard.returncode;lines.extend(shard.stdout.splitlines());continue
        for rule in rule_names:
            part=run(*command,rule);code=code or part.returncode;lines.extend(part.stdout.splitlines())
    return code,lines

def check(entry):
    category, name, path = entry
    target = folder / (name + ".json")
    before = digest(path)
    if target.is_file():
        prior = json.loads(target.read_bytes())
        if prior.get("exit")==0 and prior.get("current") and prior.get("digest")==before and prior.get("validator")==validator:
            print(name, prior["exit"], "cached", flush=True)
            return
    code,lines=guarded_check(category,name,path)
    report = {"validator": validator, "asset": name, "digest": before, "current": before==digest(path), "exit": code,
              "output": lines}
    temp = target.with_suffix(".partial")
    temp.write_text(json.dumps(report,ensure_ascii=False))
    os.replace(temp, target)
    print(name, code, report["current"], flush=True)
for entry in assets:
    check(entry)
