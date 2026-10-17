#!/bin/zsh
# usage: census.sh <label> [root]   lib and cfg(test) closure census of the c4p32 crates for native, wasm32-unknown-unknown and wasm32-wasip2; root defaults to the repo, a sandbox root measures the restored original
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label=$1; R=${2:-/Users/ueli/Documents/semio}
out="$F/🗑️generated/c4p32-census-$label.jsonl"
: > "$out"
for spec in "$R/✏️s/🔌️plugins/🎥️shooting/🗿️artifacts/🎥️shooting:semio-s-artifact-shooting-shooting" "$R/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d:semio-s-artifact-fem-3d-editor,semio-s-artifact-fem-3d-editor-session" "$R/✏️s/🔌️plugins/🏙️bim/🗿️artifacts/🏢️model:semio-s-artifact-bim-model"; do
  ws="${spec%%:*}"; only="${spec#*:}"
  for triple in "" wasm32-unknown-unknown wasm32-wasip2; do
    for ct in "" 1; do
      echo "## $ws triple=${triple:-native} test=${ct:-0}" >> "$out"
      TRIPLE=$triple CFG_TEST=$ct WS_DIR="$ws" ONLY="$only" perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p32-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
    done
  done
done
python3 - "$out" <<'PY'
import json,sys
cur=None
for line in open(sys.argv[1],encoding='utf-8'):
    line=line.strip()
    if line.startswith('##'):
        cur=line[3:]; print(cur[-70:]); continue
    d=json.loads(line)
    print('  %-44s files %3d bytes %7d lines %5d ratio %.3f binding %-5s over %s' % (d['name'][-44:], d['files'], d['bytes'], d['activeLines'], d['worstRatio'], d['binding'], d['overLimit']))
PY
