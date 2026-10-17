#!/bin/zsh
# usage: census.sh <label>   lib and cfg(test) closure census of the c4p20 crates (artifact workspace) for native, wasm32-unknown-unknown and wasm32-wasip2
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
P=semio-s-artifact-remodel-remodeling
label=$1
out="$F/🗑️generated/c4p20-census-$label.jsonl"
: > "$out"
for triple in "" wasm32-unknown-unknown wasm32-wasip2; do
  for ct in "" 1; do
    echo "## triple=${triple:-native} test=${ct:-0}" >> "$out"
    TRIPLE=$triple CFG_TEST=$ct WS_DIR='✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling' ONLY="$P-sfm,$P-mesh,$P-mesh-surface,$P-mesh-repair,$P-editor,$P-session,$P-algebra" perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p20-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
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
    print('  %-12s files %3d bytes %7d lines %5d ratio %.3f binding %-5s over %s' % (d['name'].replace('semio-s-artifact-remodel-remodeling-','') or 'main', d['files'], d['bytes'], d['activeLines'], d['worstRatio'], d['binding'], d['overLimit']))
PY
