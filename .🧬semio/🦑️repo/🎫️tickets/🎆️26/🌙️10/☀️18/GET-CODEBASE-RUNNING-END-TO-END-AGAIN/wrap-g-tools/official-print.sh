#!/bin/sh
# 🧪 wrap-g: ONE official run, print-rs:build on Bazel without cache
cd /Users/ueli/Documents/semio || exit 1
OUT="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/wrap-g/official-print.log"
date > "$OUT"; S=$(date +%s)
/usr/bin/time -l bun repo --tool=bazel run @semio-tech/print-rs:build --skip-cache >> "$OUT" 2>&1
echo "exit $? wall $(($(date +%s)-S)) s" >> "$OUT"; date >> "$OUT"
