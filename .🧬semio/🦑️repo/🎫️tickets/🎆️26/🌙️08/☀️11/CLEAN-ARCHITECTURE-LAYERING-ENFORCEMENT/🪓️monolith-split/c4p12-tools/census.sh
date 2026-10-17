#!/bin/zsh
# usage: census.sh <label>   lib and cfg(test) closure census of the c4p12 crates (four workspaces) for native, wasm32-unknown-unknown and wasm32-wasip2
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label=$1
out="$F/🗑️generated/c4p12-census-$label.jsonl"
: > "$out"
typeset -A WSMAP
WSMAP[✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout]="semio-s-artifact-layout-layout-diff"
WSMAP[✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad]="semio-s-artifact-cad-cad-picking,semio-s-artifact-cad-cad-editor,semio-s-artifact-cad-cad-inferences,semio-s-artifact-cad-cad"
WSMAP[✏️s/🔌️plugins/📕️norm]="semio-s-plugin-norm-compliance,semio-s-plugin-norm-app-surface"
WSMAP[✏️s/🔌️plugins/📕️norm/🗿️artifacts/⚖️en1990]="semio-s-artifact-norm-en1990-inferences"
for triple in "" wasm32-unknown-unknown wasm32-wasip2; do
  for ct in "" 1; do
    for ws in ${(k)WSMAP}; do
      echo "## triple=${triple:-native} test=${ct:-0} ws=${ws:t}" >> "$out"
      TRIPLE=$triple CFG_TEST=$ct WS_DIR="$ws" ONLY="${WSMAP[$ws]}" perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p12-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
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
    print('  %-44s files %3d bytes %7d lines %5d ratio %.3f binding %-5s over %s' % (d['name'], d['files'], d['bytes'], d['activeLines'], d['worstRatio'], d['binding'], d['overLimit']))
PY
