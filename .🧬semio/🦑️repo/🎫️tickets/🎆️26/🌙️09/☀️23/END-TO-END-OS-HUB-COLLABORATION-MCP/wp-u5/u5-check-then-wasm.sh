#!/bin/zsh
# 🧪️ U5: resume the label-pass sweep, then the node-graph binding's wasm32 check through the fleet wasm mutex.
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-u5 || exit 1
zsh u5-check-sweep.sh "$1" "$2"
cd /Users/ueli/Documents/semio || exit 1
echo "### wasm32 surface check queued $(date +%T)" > .tmp-ticket/wp-u5/generated/u5-check-surface-wasm32.txt
CARGO_INCREMENTAL=0 zsh .tmp-ticket/📜️fleet-mutex.sh wasm u5 -- cargo check -p semio-framework-surface --lib --target wasm32-unknown-unknown --no-default-features --features session-bindgen --message-format short >> .tmp-ticket/wp-u5/generated/u5-check-surface-wasm32.txt 2>&1
echo "EXIT=$? $(date +%T)" >> .tmp-ticket/wp-u5/generated/u5-check-surface-wasm32.txt
