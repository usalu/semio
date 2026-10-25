#!/bin/sh
cd "/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m" || exit 1
for d in b/p/debug b/p/prototype b/p/release b/p/wasm32-unknown-unknown b/p/wasm32-wasip2 b/p/t b/p/tmp b/p/semio-cargo-logs b/p/semio-cargo-provenance b/p/preparation b/r b/isolated u/o u/l u/a u/s u/k u/x u/w u/q u/r u/e cargo nx t tmp components published configuration; do printf '%s %s\n' "$d" "$(du -sk "$d" 2>/dev/null | cut -f1)"; done
for p in debug prototype release wasm32-unknown-unknown wasm32-wasip2; do for s in b/p/$p/*; do [ -d "$s" ] && printf '%s %s\n' "$s" "$(du -sk "$s" 2>/dev/null | cut -f1)"; done; done
echo DONE
