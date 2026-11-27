"""🔒️ Syncs every Cargo.lock that locks semio-framework-job-context with the manifests of the facade and its step custody core."""
import re,subprocess,sys
from pathlib import Path
root=Path(__file__).resolve().parents[8] if (Path(__file__).resolve().parents[8]/"Cargo.toml").is_file() else Path.cwd()
ctx=root/"🧰️framework/🔨️modules/🧵️job/⏱️context"
def deps(manifest):
    names=set();section=None
    for line in manifest.read_text(encoding="utf8").splitlines():
        m=re.match(r'\[(.+)\]',line)
        if m:section=m.group(1);continue
        if section in("dependencies","dev-dependencies"):
            k=re.match(r'([A-Za-z0-9_-]+)\s*=',line)
            if k:names.add(k.group(1))
    return sorted(names)
spec={"semio-framework-job-context":deps(ctx/"📦️packages/🦀️rust/Cargo.toml"),"semio-framework-job-context-step":deps(ctx/"🧱️step/📦️packages/🦀️rust/Cargo.toml")}
def block(name):
    return '[[package]]\nname = "%s"\nversion = "0.1.0"\ndependencies = [\n%s]\n'%(name,"".join(' "%s",\n'%d for d in spec[name]))
locks=[Path(p) for p in subprocess.run(["grep","-rl","--include=Cargo.lock",'name = "semio-framework-job-context"',str(root)],capture_output=True,text=True).stdout.split()]
for lock in locks:
    if "node_modules" in lock.parts:continue
    text=lock.read_text(encoding="utf8")
    parts=re.split(r'(?=^\[\[package\]\]\n)',text,flags=re.M)
    head,pkgs=parts[0],[p for p in parts[1:]]
    out=[]
    for p in pkgs:
        n=re.search(r'^name = "([^"]+)"',p,re.M).group(1)
        if n=="semio-framework-job-context-step":continue
        if n=="semio-framework-job-context":
            tail="\n" if p.endswith("\n\n") else ""
            p=block(n)+tail
        out.append((n,p))
    names=[n for n,_ in out]
    step=block("semio-framework-job-context-step")+"\n"
    i=names.index("semio-framework-job-context")
    out.insert(i+1,("semio-framework-job-context-step",step))
    if out[-1][1].endswith("\n\n") and len(out)>0 and out[-1][0]!="semio-framework-job-context-step": pass
    new=head+"".join(p for _,p in out)
    if new!=text:
        lock.write_text(new,encoding="utf8");print("updated",lock.relative_to(root))
