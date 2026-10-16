"""📊️ Read-only: per alias, count in-crate and external consumers."""
import json, re, os, subprocess, sys
ROOT="/Users/ueli/Documents/semio"; os.chdir(ROOT)
F=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"; G=F+"/🗑️generated"
reach=json.load(open(G+"/dac-reach.json"))
rows=json.load(open(G+"/dac-alias-rows.json"))
files=[f for f in subprocess.run(["git","-c","core.quotepath=false","-c","core.fsmonitor=false","ls-files","-z","--","*.rs",":!.🧬semio"],capture_output=True,text=True).stdout.split("\0") if f and os.path.exists(f)]
text={f:open(f,encoding="utf8").read() for f in files}
bylib={}
for f,v in reach.items():
    for o in v: bylib.setdefault(o["lib"],set()).add(f)
res=[]
for r in rows:
    f=r["file"]
    if f not in text: continue
    line=r["line"]; t=text[f]
    # find actual current line in file starting with prefix
    cur=[l for l in t.split("\n") if l.strip().startswith(line[:60].strip()) or line[:80] in l]
    for a in r.get("aliases",[]):
        libs=r.get("crates") or []
        inside=set()
        for lib in libs: inside|=bylib.get(lib,set())
        pat_in=re.compile(r"(?<![A-Za-z0-9_])"+re.escape(a)+r"(?![A-Za-z0-9_])")
        n_in=0; fi=set()
        for g in inside:
            if g not in text: continue
            for l in text[g].split("\n"):
                if pat_in.search(l) and not re.search(r"\bas\s+"+re.escape(a)+r"\b",l):
                    n_in+=1; fi.add(g)
        n_ext=0; fe=set()
        for lib in libs:
            pe=re.compile(r"\b"+re.escape(lib)+r"::"+re.escape(a)+r"\b")
            pb=re.compile(r"\b"+re.escape(lib)+r"::\{[^;]*?(?<![A-Za-z0-9_])"+re.escape(a)+r"(?![A-Za-z0-9_])",re.S)
            for g,tx in text.items():
                if g in inside: continue
                if lib in tx:
                    m=len(pe.findall(tx))+len(pb.findall(tx))
                    if m: n_ext+=m; fe.add(g)
        res.append(dict(file=f,alias=a,vis=r.get("vis"),crates=libs,line=line[:160],in_refs=n_in,in_files=len(fi),ext_refs=n_ext,ext_files=len(fe),cur=bool(cur)))
json.dump(res,open(G+"/dac-alias-consumers.json","w"),ensure_ascii=False)
for x in res:
    print(f"{x['vis'] or '-':11}|{x['alias']:28}|in {x['in_refs']:4}/{x['in_files']:3}|ext {x['ext_refs']:4}/{x['ext_files']:3}|{','.join(c.replace('semio_framework_','sf_') for c in x['crates'])[:50]}|{x['file'][-60:]}")
