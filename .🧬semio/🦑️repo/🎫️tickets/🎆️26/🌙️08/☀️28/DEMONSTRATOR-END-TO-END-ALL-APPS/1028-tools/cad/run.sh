#!/bin/bash
# usage: run.sh N  -> launches 5 detached checks tagged N and waits (<=570s)
cd /Users/ueli/Documents/semio || exit 1
N=$1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS/🗑️generated/1028"
E="✏️s/🔌️plugins/📐️cad/🧩️extensions"
if [ ! -f "$G/cad-r$N.started" ]; then
touch "$G/cad-r$N.started"
launch(){ nohup bash -c "SEMIO_LANGUAGE=en repo cargo --store=shared --configuration=prototype -- check --manifest-path '$2' --target wasm32-wasip2 --keep-going -p $3 > '$G/cad-r$N-$1.txt' 2>&1; echo \$? > '$G/cad-r$N-$1.rc'" >/dev/null 2>&1 & }
launch hub "🌎️hub/Cargo.toml" semio-hub-cad-component
launch b "$E/🏢️aec-building/Cargo.toml" semio-s-plugin-cad-aec-building-component
launch e "$E/🔥️aec-building-energy/Cargo.toml" semio-s-plugin-cad-aec-energy-component
launch s "$E/🏛️aec-building-structure/Cargo.toml" semio-s-plugin-cad-aec-structure-component
launch p "$E/📐️spatial-shape/Cargo.toml" semio-s-plugin-cad-spatial-shape-component
fi
for k in $(seq 1 55); do n=0; for t in hub b e s p; do [ -f "$G/cad-r$N-$t.rc" ] && n=$((n+1)); done; [ $n = 5 ] && break; sleep 10; done
for t in hub b e s p; do echo "-- $t rc=$(cat "$G/cad-r$N-$t.rc" 2>/dev/null)"; grep -n "^error" "$G/cad-r$N-$t.txt" | cut -c1-230 | head -6; done
