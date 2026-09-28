#!/bin/zsh
# 🧪️ ST2 overlay job 1: play pane coverage (vitest), 1b receipt law, then the native check of stdio + the nine families.
O=/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-cx1-overlay
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-st2
echo "== play pane coverage $(date '+%T')"; (cd "$O/🏢️semio-tech/🎡️play" && bun ./📜️script.ts test 🧪️tests/🧪️playpanecoverage); echo "== play rc=$?"
echo "== nx1b law $(date '+%T')"; bun "$W/nx1b-law.ts" "$O"; echo "== nx1b rc=$?"
echo "== cargo check $(date '+%T')"
cd "$O" && cargo check -p semio-s-plugin-stdio -p semio-s-plugin-stdio-image -p semio-s-plugin-stdio-media -p semio-s-plugin-stdio-cad -p semio-s-plugin-stdio-bim -p semio-s-plugin-stdio-mesh -p semio-s-plugin-stdio-pdf -p semio-s-plugin-stdio-office -p semio-s-plugin-stdio-semio -p semio-s-plugin-stdio-binary --lib --tests --message-format=short
echo "== cargo check rc=$? $(date '+%T')"
