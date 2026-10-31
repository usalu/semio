#!/bin/bash
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1029"
run(){ n=$1; shift; rm -f "$G/witclean-$n.rc"; SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --target wasm32-wasip2 --keep-going "$@" > "$G/witclean-$n.txt" 2>&1; echo $? > "$G/witclean-$n.rc"; }
run 3d --manifest-path "✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/Cargo.toml" --workspace
run hub --manifest-path "🌎️hub/Cargo.toml" -p semio-hub-cad-component -p semio-hub-demonstrator -p semio-hub-demonstrator-component -p semio-hub-energy-component -p semio-hub-fem-component -p semio-hub-flow-component -p semio-hub-gis-component -p semio-hub-playbook-component -p semio-hub-procedural-component -p semio-hub-process-component -p semio-hub-puzzle -p semio-hub-puzzle-component -p semio-hub-sourcing-component
echo done > "$G/witclean-all.rc"
