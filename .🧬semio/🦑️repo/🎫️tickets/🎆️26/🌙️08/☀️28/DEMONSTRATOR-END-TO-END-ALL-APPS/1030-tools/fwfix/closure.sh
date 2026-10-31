#!/bin/bash
# usage: closure.sh <n>  -- full framework closure wasm --lib probe + red list in fwfix-c<n>.red
D=$(dirname "$0"); N=$1
ROOT=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1030"
rm -f "$ROOT/fwfix-c$N.red"
"$D/wasm2.sh" "c$N" --lib -p semio-framework-os-kernel -p semio-framework-plugin -p semio-framework-surface -p semio-framework-os-node-graph -p semio-framework-os-flow-drawing -p semio-framework-2d-booleans -p semio-framework-2d-regions -p semio-framework-pixels -p semio-framework-os-store-presence -p semio-framework-job -p semio-framework-value-retirement -p semio-framework-value -p semio-framework-os-kernel-neural-engine-model -p semio-framework-os-vcs -p semio-framework-tool-run -p semio-framework-tool-machine
python3 - "$ROOT/fwfix-wc$N.txt" > "$ROOT/fwfix-c$N.red" <<'P'
import re,sys
s=open(sys.argv[1],errors='replace').read()
seen={}
for m in re.finditer(r'could not compile `([^`]+)` \(lib[^)]*\) due to (\d+) previous error',s):seen[m.group(1)]=int(m.group(2))
for k,v in sorted(seen.items()):print(v,k)
P
