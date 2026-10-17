#!/bin/zsh
# usage: census.sh <label>   lib and cfg(test) closure census of the c4p17 crates (one artifact workspace each) for native, wasm32-unknown-unknown and wasm32-wasip2
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label=$1
out="$F/🗑️generated/c4p17-census-$label.jsonl"
: > "$out"
PL='✏️s/🔌️plugins'
typeset -A WS
WS=(
 "$PL/➗️mathematical/🗿️artifacts/➗️equation" "semio-s-artifact-mathematical-equation"
 "$PL/🀄️wfc/🗿️artifacts/🖼️bitmap" "semio-s-artifact-wfc-bitmap"
 "$PL/🀄️wfc/🗿️artifacts/🧊️3d" "semio-s-artifact-wfc-3d"
 "$PL/🌀️procedural/🗿️artifacts/🌀️generation2d" "semio-s-artifact-procedural-2d-editor,semio-s-artifact-procedural-generation2d"
 "$PL/🌀️procedural/🗿️artifacts/🧊️generation3d" "semio-s-artifact-procedural-3d-io-text,semio-s-artifact-procedural-3d-io,semio-s-artifact-procedural-3d-preview-eval,semio-s-artifact-procedural-generation3d"
)
for ws in "${(@k)WS}"; do
  for triple in "" wasm32-unknown-unknown wasm32-wasip2; do
    for ct in "" 1; do
      echo "## ws=${ws:t} triple=${triple:-native} test=${ct:-0}" >> "$out"
      TRIPLE=$triple CFG_TEST=$ct WS_DIR="$ws" ONLY="${WS[$ws]}" perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p17-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
    done
  done
done
python3 - "$out" <<'PY'
import json,sys
cur=None
for line in open(sys.argv[1],encoding='utf-8'):
    line=line.strip()
    if line.startswith('##'):
        cur=line[3:]; print(cur); continue
    d=json.loads(line)
    print('  %-52s files %3d bytes %7d lines %5d ratio %.3f binding %-5s over %s err %s' % (d['name'].replace('semio-s-artifact-',''), d['files'], d['bytes'], d['activeLines'], d['worstRatio'], d['binding'], d['overLimit'], d['error'][:60]))
PY
