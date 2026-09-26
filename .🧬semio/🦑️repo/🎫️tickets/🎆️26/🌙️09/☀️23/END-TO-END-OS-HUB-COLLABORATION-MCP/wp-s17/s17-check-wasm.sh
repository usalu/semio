#!/bin/zsh
# 🧪️ S17 wasm32 gate of the VersionPin landing set (guest-linked crates): ONE `cargo check --target wasm32-wasip2 --lib`
# over the SDK, the kernel, procedural and every extension. Run through the fleet wasm mutex.
# usage: zsh .tmp-ticket/📜️fleet-mutex.sh wasm s17 -- zsh s17-check-wasm.sh <capture>
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0
crates=(semio-framework semio-framework-os-kernel semio-framework-plugin semio-s-plugin-procedural)
for c in ✏️s/🔌️plugins/*/🧩️extensions/*/📦️packages/🦀️rust/Cargo.toml(N); do crates+=($(/usr/bin/grep -m1 '^name' "$c" | sed -e 's/.*"\(.*\)".*/\1/')); done
args=(); for c in $crates; do args+=(-p $c); done
echo "START $(date '+%T') crates=${#crates}" > "$1"
nice -n 10 cargo check $args --lib --target wasm32-wasip2 --message-format short >> "$1" 2>&1
echo "EXIT=$? $(date '+%T')" >> "$1"
