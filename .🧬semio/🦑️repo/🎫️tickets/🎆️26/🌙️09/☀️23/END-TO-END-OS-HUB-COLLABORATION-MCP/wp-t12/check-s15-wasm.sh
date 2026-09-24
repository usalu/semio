#!/bin/zsh
# 🧱️ T12 / S15: one wasm32-wasip2 check of every guest plugin T12 touched for the S15 defects, through the fleet mutex.
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0
P=()
for plugin in stdio sourcing demonstrator norm procedural raster process reasoning writer trinity gis vcs draw architect imperative; do P+=(-p "semio-s-plugin-$plugin"); done
start=$(date +%s)
zsh .tmp-ticket/📜️fleet-mutex.sh wasm T12 -- cargo check --target wasm32-wasip2 "${P[@]}" --lib --keep-going --message-format=short 2>&1 | /usr/bin/grep -E '^error|error\[|: error|Finished|could not compile|warning: unused'
echo "WASM_EXIT=${pipestatus[1]} wall=$(( $(date +%s) - start ))s"
echo ALL_DONE
