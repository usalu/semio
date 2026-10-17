#!/bin/zsh
# usage: census.sh <label>   lib and cfg(test) closure census of the c4p16 crates for native, wasm32-unknown-unknown and wasm32-wasip2
cd /Users/ueli/Documents/semio || exit 2
F="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
S='✏️s/🔌️plugins/🗄️stdio'
A="$S/🗿️artifacts"
label=$1
out="$F/🗑️generated/c4p16-census-$label.jsonl"
: > "$out"
# group: WS_DIR | crate dirs | names
typeset -a G
G=(
"$S|$S/📇️registry/🧬️contract/📐️part21/📦️packages/🦀️rust,$S/🔮️oracles/📃️document/📦️packages/🦀️rust|semio-s-artifact-stdio-contract-part21,semio-s-plugin-stdio-document-test-oracle"
"$A/☁️las|$A/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📸️snapshot/📦️packages/🦀️rust,$A/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/📦️packages/🦀️rust,$A/☁️las/🏅️standards/🔖️1.0/🪆️subsets/🎩️header/🧬️schema/🔺️diff/🧪️tests/🔬️unit/📦️packages/🦀️rust|semio-s-artifact-stdio-las-snapshot,semio-s-artifact-stdio-las-schema,semio-s-artifact-stdio-las-tests"
"$A/🎒️zip|$A/🎒️zip/📦️packages/🦀️rust,$A/🎒️zip/🔮️oracles/📦️packages/🦀️rust|semio-s-artifact-stdio-zip,semio-s-artifact-stdio-zip-test-oracle"
"$A/🎥️mp4|$A/🎥️mp4/🏅️standards/🔖️isobmff/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🔬️unit/📦️packages/🦀️rust,$A/🎥️mp4/📦️packages/🦀️rust,$A/🎥️mp4/🔮️oracles/📦️packages/🦀️rust|semio-s-artifact-stdio-mp4-tests,semio-s-artifact-stdio-mp4,semio-s-artifact-stdio-mp4-test-oracle"
"$A/💬️bcf|$A/💬️bcf/🔮️oracles/📦️packages/🦀️rust|semio-s-artifact-stdio-bcf-test-oracle"
"$A/📐️step|$A/📐️step/🏅️standards/🔖️ap214/🪆️subsets/🧱️base/🚪️io/📐️geometry/📦️packages/🦀️rust,$A/📐️step/📦️packages/🦀️rust,$A/📐️step/🔮️oracles/📦️packages/🦀️rust|semio-s-artifact-stdio-step-geometry,semio-s-artifact-stdio-step,semio-s-artifact-stdio-step-test-oracle"
"$A/📕️xlsx|$A/📕️xlsx/🔮️oracles/📦️packages/🦀️rust|semio-s-artifact-stdio-xlsx-test-oracle"
)
for triple in "" wasm32-unknown-unknown wasm32-wasip2; do
  for ct in "" 1; do
    echo "## triple=${triple:-native} test=${ct:-0}" >> "$out"
    for g in "${G[@]}"; do
      ws=${g%%|*}; rest=${g#*|}; dirs=${rest%%|*}; names=${rest##*|}
      TRIPLE=$triple CFG_TEST=$ct WS_DIR=$ws CRATE_DIRS=$dirs ONLY=$names perl -e 'alarm 600; exec @ARGV' -- bun "$F/c4p16-tools/census.ts" 2>&1 | /usr/bin/grep '^{"name"' >> "$out"
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
    print('  %-48s files %3d bytes %7d lines %5d ratio %.3f binding %-5s over %s' % (d['name'].replace('semio-s-artifact-stdio-','').replace('semio-s-plugin-stdio-',''), d['files'], d['bytes'], d['activeLines'], d['worstRatio'], d['binding'], d['overLimit']))
PY
