#!/bin/zsh
# 🧪️ ST2 overlay job 3 (session 14c): the stdio family proof native + wasm32 (stdio + the nine family packages), then the
# stdio census/catalogue/provider laws and CX1's txt/tsv/html codec laws. A disk gate skips every later step below 45 GiB free.
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-cx1-overlay
P=(-p semio-s-plugin-stdio -p semio-s-plugin-stdio-image -p semio-s-plugin-stdio-media -p semio-s-plugin-stdio-cad -p semio-s-plugin-stdio-bim -p semio-s-plugin-stdio-mesh -p semio-s-plugin-stdio-pdf -p semio-s-plugin-stdio-office -p semio-s-plugin-stdio-semio -p semio-s-plugin-stdio-binary)
gate() { local free=$(df -g / | awk 'NR==2{print $4}'); echo "== disk ${free} GiB free $(date '+%T')"; [ "$free" -ge 45 ]; }
echo "== play pane coverage $(date '+%T')"; (cd "$O/🏢️semio-tech/🎡️play" && NX_DAEMON=false bun ./📜️script.ts test 🔨️modules/🧩️runtime/🟦️.ts -t "play pane coverage"); echo "== play rc=$? $(date '+%T')"
cd "$O" || exit 1
gate || exit 3
echo "== native check $(date '+%T')"; cargo check $P --lib --tests --message-format=short; echo "== native check rc=$? $(date '+%T')"
gate || exit 3
echo "== wasm32 check $(date '+%T')"; cargo check --target wasm32-wasip2 $P --lib --message-format=short; echo "== wasm32 check rc=$? $(date '+%T')"
gate || exit 3
echo "== stdio shipped_fleet + editor_catalog $(date '+%T')"; cargo test -p semio-s-plugin-stdio --test shipped_fleet --test editor_catalog --no-fail-fast --message-format=short; echo "== stdio census rc=$? $(date '+%T')"
gate || exit 3
echo "== stdio native_openable_provider $(date '+%T')"; cargo test -p semio-s-plugin-stdio --features full-artifact-catalog --test native_openable_provider --no-fail-fast --message-format=short; echo "== stdio provider rc=$? $(date '+%T')"
gate || exit 3
echo "== txt/tsv/html $(date '+%T')"; cargo test -p semio-s-artifact-stdio-txt -p semio-s-artifact-stdio-tsv -p semio-s-artifact-stdio-html --lib --tests --no-fail-fast --message-format=short; echo "== txt/tsv/html rc=$? $(date '+%T')"
