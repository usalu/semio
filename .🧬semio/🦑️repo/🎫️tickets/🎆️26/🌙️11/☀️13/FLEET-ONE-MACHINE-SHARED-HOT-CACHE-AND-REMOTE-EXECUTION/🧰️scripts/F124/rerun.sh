#!/bin/zsh
cd /Users/ueli/Documents/semio
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚖️laws/👓️image-lease-readers"
SC=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F124
python3 -I "$L/🧪️tests/🐍️oracle/🐍️.py" repository /Users/ueli/Documents/semio "$L/📒️allowlist/🔣️.json" > $SC/oracle-repo.json
repo test-file "$L/🧪️tests/🟦️.ts" > $SC/law-run.txt 2>&1
python3 - <<'E'
import re,json
sc="/private/tmp/claude-501/-Users-ueli--claude-spares-spare-1dOdZf/35823786-c219-418d-81dc-737e033e7539/scratchpad/F124/"
t=open(sc+"law-run.txt",encoding="utf8").read()
print(re.findall(r"\(fail\)[^\n]*",t), re.findall(r"\n (\d+) (?:pass|fail)",t))
m=re.search(r"error: Calls that open a path[^\n]*",t)
o=json.load(open(sc+"oracle-repo.json"));print("oracle sites",len(o["sites"]),"stale",o["stale"])
for s in o["sites"]:print("  ",s.replace("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/","L/"))
if m:
    items=re.findall(r"(🧰️framework/[^,]*?\.(?:ts|mjs)):(\d+) (\S+?)(?:,|$)",m.group(0).split(" / ")[0]);T=sorted(f"{p}:{l}:{op}" for p,l,op in items);O=sorted(o["sites"])
    print("TS",len(T),"PY",len(O));print("only TS",[x.split('/')[-3:] for x in set(T)-set(O)][:20]);print("only PY",[x.split('/')[-3:] for x in set(O)-set(T)][:20])
E
