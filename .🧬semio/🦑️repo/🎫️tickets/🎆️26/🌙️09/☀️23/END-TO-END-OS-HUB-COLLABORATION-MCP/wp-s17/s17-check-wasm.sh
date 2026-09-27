#!/bin/zsh
# 🧪️ S17 wasm32 gate of the VersionPin set (guest-linked crates): SDK, kernel, procedural, demonstrator and every extension,
# `--lib --target wasm32-wasip2`, build-fleet-b, nice 10. Starts its cargo only after the native gate capture says ALL_DONE.
# usage: zsh .tmp-ticket/📜️fleet-mutex.sh wasm s17 -- zsh s17-check-wasm.sh <capture> <native-capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b
crates=(semio-framework semio-framework-os-kernel semio-framework-plugin semio-s-plugin-procedural semio-s-plugin-demonstrator)
for c in ✏️s/🔌️plugins/*/🧩️extensions/*/📦️packages/🦀️rust/Cargo.toml(N); do crates+=($(/usr/bin/grep -m1 '^name' "$c" | sed -e 's/.*"\(.*\)".*/\1/')); done
args=(); for c in $crates; do args+=(-p $c); done
until /usr/bin/grep -q '^ALL_DONE' "$2" 2>/dev/null; do sleep 15; done
echo "START $(date '+%T') crates=${#crates}" > "$1"
nice -n 10 cargo check $args --lib --target wasm32-wasip2 --keep-going --message-format short >> "$1" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1"
