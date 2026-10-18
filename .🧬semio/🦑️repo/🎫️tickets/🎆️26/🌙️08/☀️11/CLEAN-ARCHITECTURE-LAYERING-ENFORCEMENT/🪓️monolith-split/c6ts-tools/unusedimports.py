#!/usr/bin/env python3
"""c6ts: report (and with --fix remove) unused import specifiers in the given files via tsc noUnusedLocals."""
import json,subprocess,sys,os,re
sys.path.insert(0,os.path.dirname(__file__))
from tscdiag import ROOT,TSC,pkgpaths
F=ROOT+"/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
ONLY=[a.split("=",1)[1] for a in sys.argv if a.startswith("--only=")]
def wanted(line):
    return not ONLY or any(re.search(o,line) for o in ONLY)
def main():
    fix="--fix" in sys.argv
    files=[a for a in sys.argv[1:] if not a.startswith("--")]
    files=[l.strip() for l in open(files[0]) if l.strip()]
    absf=[f if f.startswith("/") else os.path.join(ROOT,f) for f in files]
    cfg={"extends":ROOT+"/tsconfig.json","compilerOptions":{"noEmit":True,"types":[],"skipLibCheck":True,"noUnusedLocals":True,"baseUrl":ROOT,"paths":pkgpaths()},"include":[],"files":absf}
    p=F+"/🗑️generated/c6ts-tsconfig.unused.json"
    json.dump(cfg,open(p,"w"))
    r=subprocess.run(["node",TSC,"-p",p,"--pretty","false"],cwd=ROOT,capture_output=True,text=True,timeout=1500)
    os.remove(p)
    targets=set(absf)
    hits={}
    for l in (r.stdout+r.stderr).splitlines():
        m=re.match(r"(.+?)\((\d+),(\d+)\): error (TS6133|TS6192|TS6196|TS6198): (.*)",l)
        if not m: continue
        f=m.group(1); f=f if f.startswith("/") else os.path.join(ROOT,f)
        if f not in targets: continue
        hits.setdefault(f,[]).append((int(m.group(2)),int(m.group(3)),m.group(4),m.group(5)))
    for f,hs in hits.items():
        lines=open(f,encoding="utf-8").read().split("\n")
        for ln,col,code,msg in hs:
            if lines[ln-1].startswith("import") and wanted(lines[ln-1]):
                print(os.path.relpath(f,ROOT)[-70:],ln,code,msg[:100])
    if fix:
        for f,hs in hits.items():
            lines=open(f,encoding="utf-8").read().split("\n")
            byline={}
            for ln,col,code,msg in hs:
                if lines[ln-1].startswith("import") and wanted(lines[ln-1]):
                    byline.setdefault(ln,[]).append((code,msg))
            for ln,items in byline.items():
                line=lines[ln-1]
                m=re.match(r'import (type )?\{(.*)\} from (".*");',line)
                if not m: continue
                names=[x.strip() for x in m.group(2).split(",") if x.strip()]
                drop=set()
                allused=False
                for code,msg in items:
                    if code in("TS6192","TS6133") and "All imports in import declaration" in msg or code=="TS6192":
                        allused=True
                    mm=re.match(r"'(\w+)' is declared but its value is never read",msg)
                    if mm: drop.add(mm.group(1))
                keep=[] if allused else [n for n in names if n.replace("type ","") not in drop]
                lines[ln-1]=("import %s{ %s } from %s;"%(m.group(1) or "",", ".join(keep),m.group(3))) if keep else None
            lines=[l for l in lines if l is not None]
            open(f,"w",encoding="utf-8").write("\n".join(lines))
main()
