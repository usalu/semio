#!/bin/zsh
# 🧪️ ST2 overlay job 4 (session 14c): after the classification (wav, semio mesh/brep), editor-catalog details and CX1 genesis-ops
# fixes — wasm32 check of the two touched families, wav unit laws, semio artifact tests compile, then the stdio census,
# catalogue and provider laws again. Disk gate 45 GiB.
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-cx1-overlay
gate() { local free=$(df -g / | awk 'NR==2{print $4}'); echo "== disk ${free} GiB free $(date '+%T')"; [ "$free" -ge 45 ]; }
cd "$O" || exit 1
gate || exit 3
echo "== wasm32 check media+semio $(date '+%T')"; cargo check --target wasm32-wasip2 -p semio-s-plugin-stdio-media -p semio-s-plugin-stdio-semio --lib --message-format=short; echo "== wasm32 check rc=$? $(date '+%T')"
echo "== native check wav+semio artifacts --tests $(date '+%T')"; cargo check -p semio-s-artifact-stdio-wav -p semio-s-artifact-stdio-semio --lib --tests --message-format=short; echo "== native artifacts rc=$? $(date '+%T')"
gate || exit 3
echo "== wav lib laws $(date '+%T')"; cargo test -p semio-s-artifact-stdio-wav --lib --no-fail-fast --message-format=short; echo "== wav rc=$? $(date '+%T')"
gate || exit 3
echo "== stdio shipped_fleet + editor_catalog $(date '+%T')"; cargo test -p semio-s-plugin-stdio --test shipped_fleet --test editor_catalog --no-fail-fast --message-format=short; echo "== stdio census rc=$? $(date '+%T')"
gate || exit 3
echo "== stdio native_openable_provider $(date '+%T')"; cargo test -p semio-s-plugin-stdio --features full-artifact-catalog --test native_openable_provider --no-fail-fast --message-format=short; echo "== stdio provider rc=$? $(date '+%T')"
