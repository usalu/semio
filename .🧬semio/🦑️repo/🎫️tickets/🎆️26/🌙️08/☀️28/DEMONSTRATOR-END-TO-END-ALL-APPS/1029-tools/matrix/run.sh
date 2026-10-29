#!/bin/bash
# One batched wasm32-wasip2 check per Cargo workspace for the 32-component activation closure.
# The 3d puzzle workspace is not part of the closure and is never listed here.
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1029"
mkdir -p "$G"
cd "$ROOT" || exit 1
EXT="✏️s/🔌️plugins"

chk(){
  name=$1; ws=$2; shift 2
  rm -f "$G/matrix-$name.rc"
  if [ ! -f "$ROOT/$ws" ]; then
    echo "missing manifest: $ROOT/$ws" > "$G/matrix-$name.txt"
    echo 2 > "$G/matrix-$name.rc"
    return
  fi
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$ws" --target wasm32-wasip2 --keep-going "$@" > "$G/matrix-$name.txt" 2>&1
  echo $? > "$G/matrix-$name.rc"
}

chk hub "🌎️hub/Cargo.toml" \
  -p semio-hub-cad-component -p semio-hub-demonstrator -p semio-hub-demonstrator-component \
  -p semio-hub-energy-component -p semio-hub-fem-component -p semio-hub-flow-component \
  -p semio-hub-gis-component -p semio-hub-playbook-component -p semio-hub-procedural-component \
  -p semio-hub-process-component -p semio-hub-puzzle -p semio-hub-puzzle-component \
  -p semio-hub-sourcing-component

chk framework "🧰️framework/Cargo.toml" \
  -p semio-framework-surface -p semio-framework-surface-component \
  -p semio-framework-editor -p semio-framework-editor-component

chk root-flow "Cargo.toml" -p semio-framework-os-flow -p semio-framework-os-flow-component -p semio-framework-os-node-graph -p semio-framework-os-node-graph-component

chk cad-aec-building "$EXT/📐️cad/🧩️extensions/🏢️aec-building/Cargo.toml" -p semio-s-plugin-cad-aec-building-component
chk cad-aec-energy "$EXT/📐️cad/🧩️extensions/🔥️aec-building-energy/Cargo.toml" -p semio-s-plugin-cad-aec-energy-component
chk cad-aec-structure "$EXT/📐️cad/🧩️extensions/🏛️aec-building-structure/Cargo.toml" -p semio-s-plugin-cad-aec-structure-component
chk cad-spatial-shape "$EXT/📐️cad/🧩️extensions/📐️spatial-shape/Cargo.toml" -p semio-s-plugin-cad-spatial-shape-component

chk flow-bim "$EXT/🌊️flow/🧩️extensions/🏗️bim/Cargo.toml" -p semio-s-plugin-flow-bim-component
chk flow-brep "$EXT/🌊️flow/🧩️extensions/📐️brep/Cargo.toml" -p semio-s-plugin-flow-brep-component
chk flow-dictionary "$EXT/🌊️flow/🧩️extensions/📖️dictionary/Cargo.toml" -p semio-s-plugin-flow-dictionary-component
chk flow-draw "$EXT/🌊️flow/🧩️extensions/🖍️draw/Cargo.toml" -p semio-s-plugin-flow-draw-component
chk flow-list "$EXT/🌊️flow/🧩️extensions/📃️list/Cargo.toml" -p semio-s-plugin-flow-list-component
chk flow-logic "$EXT/🌊️flow/🧩️extensions/🧠️logic/Cargo.toml" -p semio-s-plugin-flow-logic-component
chk flow-math "$EXT/🌊️flow/🧩️extensions/🧮️math/Cargo.toml" -p semio-s-plugin-flow-math-component
chk flow-primitive "$EXT/🌊️flow/🧩️extensions/🔤️primitive/Cargo.toml" -p semio-s-plugin-flow-primitive-component
chk flow-text "$EXT/🌊️flow/🧩️extensions/📝️text/Cargo.toml" -p semio-s-plugin-flow-text-component

chk playbook-procedural "$EXT/📖️playbook/🧩️extensions/🌀️procedural/Cargo.toml" -p semio-s-plugin-playbook-procedural-component

chk process-concrete "$EXT/🏭️process/🧩️extensions/🧱️concrete/Cargo.toml" -p semio-s-plugin-process-concrete-component
chk process-metal "$EXT/🏭️process/🧩️extensions/🔩️metal/Cargo.toml" -p semio-s-plugin-process-metal-component
chk process-robotic "$EXT/🏭️process/🧩️extensions/🤖️robotic/Cargo.toml" -p semio-s-plugin-process-robotic-component
chk process-wood "$EXT/🏭️process/🧩️extensions/🪵️wood/Cargo.toml" -p semio-s-plugin-process-wood-component

chk sourcing-beams "$EXT/🪵️sourcing/🧩️extensions/🪵️beams/Cargo.toml" -p semio-s-plugin-sourcing-beams-component
chk sourcing-slabs "$EXT/🪵️sourcing/🧩️extensions/🧱️slabs/Cargo.toml" -p semio-s-plugin-sourcing-slabs-component
chk sourcing-windows "$EXT/🪵️sourcing/🧩️extensions/🪟️windows/Cargo.toml" -p semio-s-plugin-sourcing-windows-component

echo done > "$G/matrix-all.rc"
