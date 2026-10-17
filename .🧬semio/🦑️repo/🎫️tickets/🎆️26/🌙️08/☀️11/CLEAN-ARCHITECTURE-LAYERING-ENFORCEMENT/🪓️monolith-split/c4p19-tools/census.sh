#!/bin/zsh
# usage: census.sh <label>   lib and cfg(test) closure census of the eight c4p19 crates for native, wasm32-unknown-unknown and wasm32-wasip2
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
label=$1
out="$F/🗑️generated/c4p19-census-$label.jsonl"
: > "$out"
N='✏️s/🔌️plugins/📕️norm/🗿️artifacts'
for spec in "$N/📇️iso16757:semio-s-artifact-norm-iso16757-diff" "$N/🧩️en1994:semio-s-artifact-norm-en1994-diff" "$N/🧱️din4108:semio-s-artifact-norm-din4108-checks" "$N/🪨️en1996:semio-s-artifact-norm-en1996-schema" "$N/🪵️en1995:semio-s-artifact-norm-en1995-schema" "$N/🪶️en1999:semio-s-artifact-norm-en1999,semio-s-artifact-norm-en1999-schema" "$N/🫨️en1998:semio-s-artifact-norm-en1998-inferences" "✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural:semio-s-plugin-playbook-procedural"; do
  ws="${spec%%:*}"; only="${spec#*:}"
  for triple in "" wasm32-unknown-unknown wasm32-wasip2; do
    for ct in "" 1; do
      echo "## $ws triple=${triple:-native} test=${ct:-0}" >> "$out"
      TRIPLE=$triple CFG_TEST=$ct WS_DIR="$ws" ONLY="$only" perl -e 'alarm 900; exec @ARGV' -- bun "$F/c4p19-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
    done
  done
done
python3 - "$out" <<'PY'
import json,sys
cur=None
for line in open(sys.argv[1],encoding='utf-8'):
    line=line.strip()
    if line.startswith('##'):
        cur=line[3:]; print(cur.split('/')[-1] if False else cur[-60:]); continue
    d=json.loads(line)
    print('  %-40s files %3d bytes %7d lines %5d ratio %.3f binding %-5s over %s' % (d['name'][-40:], d['files'], d['bytes'], d['activeLines'], d['worstRatio'], d['binding'], d['overLimit']))
PY
