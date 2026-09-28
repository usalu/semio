#!/bin/zsh
# 🧪️ ST2 overlay job 2: play pane coverage (in-source vitest law), the stdio census laws (shipped_fleet) and CX1's laws
# (stdio registry unit laws, native_openable_provider, txt/tsv/html codecs).
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-cx1-overlay
echo "== play pane coverage $(date '+%T')"; (cd "$O/🏢️semio-tech/🎡️play" && bun ./📜️script.ts test 🔨️modules/🧩️runtime/🟦️.ts -t "play pane coverage"); echo "== play rc=$?"
cd "$O" || exit 1
echo "== stdio shipped_fleet + native_openable_provider $(date '+%T')"; cargo test -p semio-s-plugin-stdio --test shipped_fleet --test native_openable_provider --no-fail-fast --message-format=short; echo "== stdio integration rc=$? $(date '+%T')"
echo "== txt/tsv/html $(date '+%T')"; cargo test -p semio-s-artifact-stdio-txt -p semio-s-artifact-stdio-tsv -p semio-s-artifact-stdio-html --lib --tests --no-fail-fast --message-format=short; echo "== txt/tsv/html rc=$? $(date '+%T')"
echo "== stdio lib $(date '+%T')"; cargo test -p semio-s-plugin-stdio --lib --no-fail-fast --message-format=short; echo "== stdio lib rc=$? $(date '+%T')"
