"""🔎️ Read-only scan: classify each added `pub use X as Y` alias with its owning crate(s) and consumer counts."""
import json, re, os, subprocess, sys
ROOT="/Users/ueli/Documents/semio"
os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
G=F+"/🗑️generated"
reach=json.load(open(G+"/dac-reach.json"))
rows=[l.rstrip("\n").split("\t") for l in open(G+"/dac-alias-hits.tsv",encoding="utf8")]
files=subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-z","--","*.rs",":!.🧬semio"],capture_output=True,text=True).stdout.split("\0")
files=[f for f in files if f and os.path.exists(f)]
text={}
for f in files:
    try: text[f]=open(f,encoding="utf8").read()
    except Exception: pass
print("loaded",len(text),file=sys.stderr)
out=[]
ALIAS=re.compile(r"\bas\s+([A-Za-z_][A-Za-z0-9_]*)")
for path,line in rows:
    if path not in text:
        out.append(dict(file=path,line=line,status="missing"));continue
    t=text[path]
    exists=line in t
    vis=re.match(r"pub(\([^)]*\))?",line)
    vis=vis.group(0) if vis else ""
    aliases=ALIAS.findall(line)
    crates=sorted({o["lib"] for o in reach.get(path,[])})
    out.append(dict(file=path,line=line,exists=exists,vis=vis,aliases=aliases,crates=crates))
json.dump(out,open(G+"/dac-alias-rows.json","w"),ensure_ascii=False,indent=0)
