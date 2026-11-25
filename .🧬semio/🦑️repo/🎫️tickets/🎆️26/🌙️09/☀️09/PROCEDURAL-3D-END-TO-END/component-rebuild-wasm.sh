#!/bin/sh
# 🕸️ Flow browser-package wasm with the real exit status of repo recorded.
cd /Users/ueli/Documents/semio || exit 1
G=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️09/PROCEDURAL-3D-END-TO-END/🗑️generated/component-rebuild"
n=$1
repo run semio-framework-os-flow-browser-package:wasm -c prototype --exclude-dependencies > "$G/flow-wasm-run-$n.txt" 2>&1
code=$?
echo "$(date +%T) flow-wasm-run-$n repo-exit $code" >> "$G/run.txt"
