#!/bin/zsh
# usage: census.sh <label>   lib and cfg(test) closure census (canonical readRustSourceClosureV1) of the six stdio artifact crates for native, wasm32-unknown-unknown and wasm32-wasip2
export PATH="/usr/bin:/bin:/usr/sbin:/sbin:/opt/homebrew/bin:/usr/local/bin:$HOME/.bun/bin:$HOME/.cargo/bin"
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
A='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts'
label=$1
out="$F/🗑️generated/c4p34-census-$label.jsonl"
: > "$out"
typeset -A W
W=("$A/📷️png" "semio-s-artifact-stdio-png-mutations,semio-s-artifact-stdio-png" "$A/📸️jpg" "semio-s-artifact-stdio-jpg" "$A/📼️avi" "semio-s-artifact-stdio-avi-tests,semio-s-artifact-stdio-avi" "$A/🖋️dxf" "semio-s-artifact-stdio-dxf" "$A/🗜️deflate" "semio-s-artifact-stdio-deflate" "$A/📽️pptx" "semio-s-artifact-stdio-pptx" "$A/📽️pptx/🔮️oracles/📦️packages/🦀️rust" "semio-s-artifact-stdio-pptx-test-oracle")
for ws in "${(@k)W}"; do
  for triple in "" wasm32-unknown-unknown wasm32-wasip2; do
    for ct in "" 1; do
      echo "## ws=${ws:t} triple=${triple:-native} test=${ct:-0}" >> "$out"
      TRIPLE=$triple CFG_TEST=$ct WS_DIR="$ws" ONLY="${W[$ws]}" ulimit -n 65536; TRIPLE=$triple CFG_TEST=$ct WS_DIR="$ws" ONLY="${W[$ws]}" perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p34-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
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
