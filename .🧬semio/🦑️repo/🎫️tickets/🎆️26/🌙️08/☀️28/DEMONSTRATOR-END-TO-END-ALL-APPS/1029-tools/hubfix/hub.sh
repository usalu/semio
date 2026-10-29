#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
N=$1
cd "$ROOT" || exit 1
rm -f "$C/🗑️generated/1029/hubfix-chk$N.rc"
SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/🌎️hub/Cargo.toml" --target wasm32-wasip2 --keep-going \
  -p semio-hub-cad-component -p semio-hub-demonstrator -p semio-hub-demonstrator-component \
  -p semio-hub-energy-component -p semio-hub-fem-component -p semio-hub-flow-component \
  -p semio-hub-gis-component -p semio-hub-playbook-component -p semio-hub-procedural-component \
  -p semio-hub-process-component -p semio-hub-puzzle -p semio-hub-puzzle-component \
  -p semio-hub-sourcing-component > "$C/🗑️generated/1029/hubfix-chk$N.txt" 2>&1
echo $? > "$C/🗑️generated/1029/hubfix-chk$N.rc"
