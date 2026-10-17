#!/bin/zsh
# usage: census.sh <label>   lib and cfg(test) closure census of the four touched artifact crates for native and wasm32-wasip2
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label=$1
out="$F/🗑️generated/c4p18-census-$label.jsonl"
: > "$out"
P='✏️s/🔌️plugins'
for pair in "🌀️procedural/🗿️artifacts/🧊️generation3d|semio-s-artifact-procedural-3d-envelope" "🌀️procedural/🗿️artifacts/🧊️generation3d|semio-s-artifact-procedural-generation3d" "🌍️gis/🗿️artifacts/🗺️gismap|semio-s-artifact-gis-gismap" "🌿️vcs/🗿️artifacts/🌿️vcs|semio-s-artifact-vcs-vcs" "🎞️animate/🗿️artifacts/🎬️presentation|semio-s-artifact-animate-presentation"; do
  ws=${pair%%|*}; crate=${pair##*|}
  for triple in "" wasm32-wasip2; do
    for ct in "" 1; do
      echo "## $crate triple=${triple:-native} test=${ct:-0}" >> "$out"
      TRIPLE=$triple CFG_TEST=$ct WS_DIR="$P/$ws" ONLY="$crate" perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p18-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
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
    print('  files %3d bytes %7d lines %5d ratio %s binding %-5s over %s' % (d['files'], d['bytes'], d['activeLines'], d['worstRatio'], d['binding'], d['overLimit']))
PY
