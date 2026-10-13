#!/bin/bash
# ⏳️ Re-checks the upstream framework crates (os-kernel and plugin, native lib, private E4 dirs) until both compile or
# <seconds> elapse; prints GREEN or the first upstream error. usage: ⏳️await-upstream.sh <seconds>
HERE="$(cd "$(dirname "$0")" && pwd)"; E4=/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/e4; END=$(( $(date +%s) + $1 ))
while :; do
  rm -f "$E4/logs/upstream.log"
  bash "$HERE/🏗️cargo.sh" upstream 300 check --offline --manifest-path Cargo.toml -p semio-framework-os-kernel -p semio-framework-plugin --lib --message-format short >/dev/null 2>&1
  if grep -q 'Finished' "$E4/logs/upstream.log" && ! grep -q ': error' "$E4/logs/upstream.log"; then echo GREEN; exit 0; fi
  grep -m1 ': error' "$E4/logs/upstream.log" | cut -c1-220
  [ $(date +%s) -ge $END ] && { echo "still red"; exit 1; }
  sleep 60
done
