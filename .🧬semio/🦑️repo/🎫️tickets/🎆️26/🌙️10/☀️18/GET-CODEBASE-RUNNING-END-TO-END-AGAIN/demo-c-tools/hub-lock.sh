#!/bin/sh
# aligns the hub workspace lock to the installed wasm-bindgen CLI (0.2.129) offline
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
for spec in "wasm-bindgen 0.2.129" "js-sys 0.3.106" "web-sys 0.3.106" "wasm-bindgen-futures 0.4.79"; do
  set -- $spec
  nice -n 10 bun repo cargo --store=shared --configuration=prototype -- update --manifest-path 🌎️hub/Cargo.toml --offline -p "$1" --precise "$2" 2>&1 | tail -5
  echo "rc=$? $1"
done
