#!/usr/bin/env python3
# compares the bun totals of a local and an offloaded run of one suite target: proof.py local.txt remote.txt
import re,sys,json
def totals(path):
    t={"pass":0,"fail":0,"expect":0,"tests":0,"files":0,"skip":0}; origins={}; lines=0
    for line in open(path,errors="replace"):
        line=re.sub(r"^\[\d+/\d+ [^\]]+\] ","",line.rstrip("\n")); lines+=1
        m=re.match(r"^\s*(\d+) (pass|fail|skip)\b",line)
        if m: t[m.group(2)]+=int(m.group(1))
        m=re.match(r"^\s*(\d+) expect\(\) calls",line)
        if m: t["expect"]+=int(m.group(1))
        m=re.match(r"^Ran (\d+) tests? across (\d+) files?",line)
        if m: t["tests"]+=int(m.group(1)); t["files"]+=int(m.group(2))
        m=re.search(r"ran on (g1-[a-z0-9]+)|lief auf (g1-[a-z0-9]+)|was run on (g1-[a-z0-9]+)|ran on (g1-[a-z0-9]+)",line)
        if m:
            o=[g for g in m.groups() if g][0]; origins[o]=origins.get(o,0)+1
    return t,origins
a,_=totals(sys.argv[1]); b,o=totals(sys.argv[2])
print(json.dumps({"local":a,"remote":b,"identical":a==b,"origins":o,"distinctMachines":len(o)},indent=1))
sys.exit(0 if a==b and len(o)>=2 else 1)
