#!/bin/zsh
# usage: census.sh <label>   canonical closure census (F/🗑️generated/census.ts) of the c4p43 crates (framework crates, lib closure)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/🗑️generated" || exit 2
label=$1
raw="$F/🗑️generated/c4p43-census-$label.json.log"
ulimit -n 65536
ONLY=semio-framework-os-inference,semio-framework-os-directory,semio-framework-os-history-codec,semio-framework-os-spr,semio-framework-os-channel,semio-framework-os-channel-tests perl -e 'alarm 900; exec @ARGV' -- nice -n 10 bun census.ts > "$raw" 2>&1
python3 - "$raw" <<'PY'
import json,re,sys
t=open(sys.argv[1],encoding='utf-8').read()
dec=json.JSONDecoder(); i=0
while True:
    j=t.find('{',i)
    if j<0: break
    try: d,e=dec.raw_decode(t[j:])
    except Exception: i=j+1; continue
    i=j+e
    if isinstance(d,dict) and 'name' in d and 'activeLines' in d:
        print('%-48s files %3d bytes %7d lines %5d ratio %.3f binding %-6s over %s err %s'%(d['name'],d['files'],d['bytes'],d['activeLines'],d['worstRatio'],d['binding'],d['overLimit'],d['error']))
PY
