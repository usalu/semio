#!/usr/bin/env python3
"""usage: errs2.py LOG [--missing]  Summarises the filtered JSON error log of run2.sh per package; --missing prints unresolved-crate rows (pkg, target kind, crate) for the dependency fixer."""
import json,sys,re,collections
pk=collections.OrderedDict(); miss=collections.Counter()
for line in open(sys.argv[1],errors="replace"):
    if not line.startswith('{"E"'): continue
    j=json.loads(line)
    m=re.match(r"(?:path\+file://)?.*[#/]([^#/@ ]+?)(?:@[\d.]+)?$",j["pkg"]) or None
    name=j["pkg"]
    mm=re.search(r"#([\w\-]+)@",j["pkg"]) or re.search(r"^([\w\-]+) ",j["pkg"])
    if mm: name=mm.group(1)
    kind=",".join(j["kind"])
    e=pk.setdefault(name,collections.OrderedDict())
    msg=re.sub(r"`[^`]*`","`_`",j["msg"])[:100]
    prim=[s for s in j["spans"] if s[3]]
    loc=(prim[0][0][-60:]+":"+str(prim[0][1])) if prim else ""
    row=e.setdefault((j["code"],msg),[0,loc,kind,j["msg"]])
    row[0]+=1
    um=re.search(r"unresolved module or unlinked crate `(\w+)`|unresolved import `(\w+)`|cannot find .* in crate `(\w+)`",j["msg"])
    if um: miss[(name,kind,[g for g in um.groups() if g][0])]+=1
if "--missing" in sys.argv:
    for (n,k,c),v in sorted(miss.items()): print(n,k,c,v)
    sys.exit()
for n,e in pk.items():
    print(f"{n}: {sum(r[0] for r in e.values())} errors")
    for (code,msg),(c,loc,kind,full) in list(e.items())[:6]: print(f"   {c:3d} {code or '-'} [{kind}] {full[:130]} | {loc}")
