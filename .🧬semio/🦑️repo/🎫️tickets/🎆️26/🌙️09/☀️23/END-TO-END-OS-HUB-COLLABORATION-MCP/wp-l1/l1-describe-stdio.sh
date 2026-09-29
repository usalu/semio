#!/bin/zsh
# 🧾️ L1 T2: describe + materialize-dev of stdio and its 9 family packages, then plugin-registry generate (wasm lane, default build-dir).
# usage: zsh l1-describe-stdio.sh <capture>
setopt no_bg_nice
R=/Users/ueli/Documents/semio; out="${1:A}"
cd $R || exit 2
export NX_DAEMON=false CARGO_INCREMENTAL=0
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
B="./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"
echo "QUEUED $(date '+%F %T')" > "$out"
zsh "$R/.tmp-ticket/📜️fleet-mutex.sh" wasm l1 -- zsh -c 'echo "START $(date "+%F %T")"; s=$(date +%s); nice -n 5 bun "$0" nx run-many -t describe materialize-dev -p "@semio-tech/stdio-plugin" "@semio-tech/stdio-*-plugin" --parallel=2 --outputStyle=stream; rc=$?; echo "DESCRIBE rc=$rc $(date "+%T")"; [ $rc -eq 0 ] && { nice -n 5 bun nx run @semio-tech/plugin-registry:generate --outputStyle=stream; rc=$?; echo "GENERATE rc=$rc $(date "+%T")"; }; echo "END rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc' "$B" >> "$out" 2>&1
echo "LANE-EXIT rc=$? $(date '+%F %T')" >> "$out"
