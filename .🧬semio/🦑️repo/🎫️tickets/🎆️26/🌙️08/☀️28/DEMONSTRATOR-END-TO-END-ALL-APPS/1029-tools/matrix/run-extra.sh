#!/bin/bash
# Re-runs the framework batch split by workspace: the flow family and flow component declare the repo root as their workspace, so they are checked from the root manifest.
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1029"
T="$C/1029-tools/matrix"
while kill -0 "$(cat "$T/pid")" 2>/dev/null; do sleep 15; done
cd "$ROOT" || exit 1
chk(){
  name=$1; ws=$2; shift 2
  rm -f "$G/matrix-$name.rc"
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$ws" --target wasm32-wasip2 --keep-going "$@" > "$G/matrix-$name.txt" 2>&1
  echo $? > "$G/matrix-$name.rc"
}
chk framework-retry "🧰️framework/Cargo.toml" \
  -p semio-framework-surface -p semio-framework-surface-component \
  -p semio-framework-editor -p semio-framework-editor-component \
  -p semio-framework-os-node-graph -p semio-framework-os-node-graph-component
chk root-flow "Cargo.toml" -p semio-framework-os-flow -p semio-framework-os-flow-component
echo done > "$G/matrix-extra.rc"
