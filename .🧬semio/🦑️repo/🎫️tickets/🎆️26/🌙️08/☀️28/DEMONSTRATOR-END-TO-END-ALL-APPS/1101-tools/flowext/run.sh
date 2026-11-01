#!/bin/bash
# usage: run.sh <tag> <batch>...
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1101"
P="✏️s/🔌️plugins"
spec(){
  case "$1" in
    flow-bim) echo "$P/🌊️flow/🧩️extensions/🏗️bim/Cargo.toml -p semio-s-plugin-flow-bim-component";;
    flow-brep) echo "$P/🌊️flow/🧩️extensions/📐️brep/Cargo.toml -p semio-s-plugin-flow-brep-component";;
    flow-dictionary) echo "$P/🌊️flow/🧩️extensions/📖️dictionary/Cargo.toml -p semio-s-plugin-flow-dictionary-component";;
    flow-draw) echo "$P/🌊️flow/🧩️extensions/🖍️draw/Cargo.toml -p semio-s-plugin-flow-draw-component";;
    flow-list) echo "$P/🌊️flow/🧩️extensions/📃️list/Cargo.toml -p semio-s-plugin-flow-list-component";;
    flow-logic) echo "$P/🌊️flow/🧩️extensions/🧠️logic/Cargo.toml -p semio-s-plugin-flow-logic-component";;
    flow-math) echo "$P/🌊️flow/🧩️extensions/🧮️math/Cargo.toml -p semio-s-plugin-flow-math-component";;
    flow-primitive) echo "$P/🌊️flow/🧩️extensions/🔤️primitive/Cargo.toml -p semio-s-plugin-flow-primitive-component";;
    flow-text) echo "$P/🌊️flow/🧩️extensions/📝️text/Cargo.toml -p semio-s-plugin-flow-text-component";;
    flow-model) echo "$P/🌊️flow/🧩️extensions/🔤️primitive/Cargo.toml -p semio-framework-artifact-flow-flow-model";;
    flow-plugin) echo "$P/🌊️flow/🗿️artifacts/🌊️flow/Cargo.toml --workspace";;
    hub-flowplaybook) echo "🌎️hub/Cargo.toml -p semio-hub-flow-component -p semio-hub-playbook-component";;
  esac
}
tag=$1; shift
rm -f "$G/flowext-run-$tag.rc"
cd "$ROOT" || exit 1
for b in "$@"; do
  read -r m args <<< "$(spec "$b")"
  out="$G/flowext-$b-$tag"
  for attempt in 1 2 3 4 5 6; do
    rm -f "$out.rc"
    SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path "$ROOT/$m" --target wasm32-wasip2 --keep-going $args > "$out.txt" 2>&1
    echo $? > "$out.rc"
    if /usr/bin/grep -q "No space left on device\|ENOSPC\|Original resource loan deadline exhausted\|ProgressDeadlineExpired" "$out.txt"; then sleep 240; else break; fi
  done
done
echo done > "$G/flowext-run-$tag.rc"
