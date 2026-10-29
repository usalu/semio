#!/bin/bash
# Corrected framework batch: the node-graph crates declare the repo root as their workspace (like the flow crates), so they move to the root manifest.
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1029"
while [ ! -f "$G/matrix-rerun.rc" ]; do sleep 15; done
cd "$ROOT" || exit 1
chk2(){
  name=$1-r2; ws=$2; shift 2
  rm -f "$G/matrix-$name.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$ws" --target wasm32-wasip2 --keep-going "$@" > "$G/matrix-$name.txt" 2>&1
  echo $? > "$G/matrix-$name.rc"
}
chk2 framework "🧰️framework/Cargo.toml" -p semio-framework-surface -p semio-framework-surface-component -p semio-framework-editor -p semio-framework-editor-component
chk2 root-nodegraph "Cargo.toml" -p semio-framework-os-node-graph -p semio-framework-os-node-graph-component
echo done > "$G/matrix-extra2.rc"
