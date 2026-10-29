#!/bin/bash
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1029"
EXT="✏️s/🔌️plugins"
mkdir -p "$G"

spec(){
  case "$1" in
    cad-aec-building) echo "$EXT/📐️cad/🧩️extensions/🏢️aec-building semio-s-plugin-cad-aec-building-component";;
    cad-aec-energy) echo "$EXT/📐️cad/🧩️extensions/🔥️aec-building-energy semio-s-plugin-cad-aec-energy-component";;
    cad-aec-structure) echo "$EXT/📐️cad/🧩️extensions/🏛️aec-building-structure semio-s-plugin-cad-aec-structure-component";;
    cad-spatial-shape) echo "$EXT/📐️cad/🧩️extensions/📐️spatial-shape semio-s-plugin-cad-spatial-shape-component";;
    flow-bim) echo "$EXT/🌊️flow/🧩️extensions/🏗️bim semio-s-plugin-flow-bim-component";;
    flow-brep) echo "$EXT/🌊️flow/🧩️extensions/📐️brep semio-s-plugin-flow-brep-component";;
    flow-dictionary) echo "$EXT/🌊️flow/🧩️extensions/📖️dictionary semio-s-plugin-flow-dictionary-component";;
    flow-draw) echo "$EXT/🌊️flow/🧩️extensions/🖍️draw semio-s-plugin-flow-draw-component";;
    flow-list) echo "$EXT/🌊️flow/🧩️extensions/📃️list semio-s-plugin-flow-list-component";;
    flow-logic) echo "$EXT/🌊️flow/🧩️extensions/🧠️logic semio-s-plugin-flow-logic-component";;
    flow-math) echo "$EXT/🌊️flow/🧩️extensions/🧮️math semio-s-plugin-flow-math-component";;
    flow-primitive) echo "$EXT/🌊️flow/🧩️extensions/🔤️primitive semio-s-plugin-flow-primitive-component";;
    flow-text) echo "$EXT/🌊️flow/🧩️extensions/📝️text semio-s-plugin-flow-text-component";;
    playbook-procedural) echo "$EXT/📖️playbook/🧩️extensions/🌀️procedural semio-s-plugin-playbook-procedural-component";;
    process-concrete) echo "$EXT/🏭️process/🧩️extensions/🧱️concrete semio-s-plugin-process-concrete-component";;
    process-metal) echo "$EXT/🏭️process/🧩️extensions/🔩️metal semio-s-plugin-process-metal-component";;
    process-robotic) echo "$EXT/🏭️process/🧩️extensions/🤖️robotic semio-s-plugin-process-robotic-component";;
    process-wood) echo "$EXT/🏭️process/🧩️extensions/🪵️wood semio-s-plugin-process-wood-component";;
    sourcing-beams) echo "$EXT/🪵️sourcing/🧩️extensions/🪵️beams semio-s-plugin-sourcing-beams-component";;
    sourcing-slabs) echo "$EXT/🪵️sourcing/🧩️extensions/🧱️slabs semio-s-plugin-sourcing-slabs-component";;
    sourcing-windows) echo "$EXT/🪵️sourcing/🧩️extensions/🪟️windows semio-s-plugin-sourcing-windows-component";;
  esac
}
ALL="cad-aec-building cad-aec-energy cad-aec-structure cad-spatial-shape flow-bim flow-brep flow-dictionary flow-draw flow-list flow-logic flow-math flow-primitive flow-text playbook-procedural process-concrete process-metal process-robotic process-wood sourcing-beams sourcing-slabs sourcing-windows"

chk(){
  b=$1; tag=$2
  read -r dir pkg <<< "$(spec "$b")"
  out="$G/extfix-$b-$tag"
  rm -f "$out.rc"
  cd "$ROOT" || return
  SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$dir/Cargo.toml" --target wasm32-wasip2 --keep-going -p "$pkg" > "$out.txt" 2>&1
  echo $? > "$out.rc"
}

chk_retry(){
  b=$1; tag=$2
  for attempt in 1 2 3 4 5 6; do
    chk "$b" "$tag"
    if /usr/bin/grep -q "No space left on device\|ENOSPC\|Original resource loan deadline exhausted" "$G/extfix-$b-$tag.txt"; then
      echo "[retry] $b attempt $attempt hit infrastructure ENOSPC/deadline" >> "$G/extfix-retry-$tag.log"
      sleep 240
    else
      break
    fi
  done
}
