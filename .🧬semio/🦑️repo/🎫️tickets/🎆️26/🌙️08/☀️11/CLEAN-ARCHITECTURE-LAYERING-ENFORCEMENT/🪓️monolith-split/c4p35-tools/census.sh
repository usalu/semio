#!/bin/zsh
# usage: census.sh <label>   canonical closure census (F/🗑️generated/census.ts) of the c4p35 crates (framework crates, lib closure)
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
cd "$F/🗑️generated" || exit 2
label=$1
raw="$F/🗑️generated/c4p35-census-$label.json.log"
ONLY=semio-framework-pack-format,semio-framework-pack-json,semio-framework-pack-json-tests,semio-framework-onboarding,semio-framework-replication-mutation,semio-framework-replication-command-ingress,semio-framework-replication-presence,semio-framework-replication-frames,semio-framework-machine-derive perl -e 'alarm 900; exec @ARGV' -- bun census.ts > "$raw" 2>&1
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
