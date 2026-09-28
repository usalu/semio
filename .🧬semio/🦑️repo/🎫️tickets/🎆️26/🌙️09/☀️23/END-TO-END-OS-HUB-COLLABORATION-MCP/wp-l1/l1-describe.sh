#!/bin/zsh
# 🧾️ L1: regenerate every plugin descriptor + registry through the chain's own rebuild-all steps (components = describe +
# materialize-dev, then generate), in the fleet wasm lane, default build-dir (= the chain's). usage: zsh l1-describe.sh <capture>
setopt no_bg_nice
R=/Users/ueli/Documents/semio; out="${1:A}"
cd $R || exit 2
export NX_DAEMON=false CARGO_INCREMENTAL=0
unset CARGO_TARGET_DIR CARGO_BUILD_TARGET_DIR CARGO_BUILD_BUILD_DIR
echo "QUEUED $(date '+%F %T')" > "$out"
zsh "$R/.tmp-ticket/📜️fleet-mutex.sh" wasm l1 -- zsh -c 'echo "START $(date "+%F %T")"; s=$(date +%s); nice -n 5 bun nx run @semio-tech/plugin-registry:rebuild-all --from components --to generate --outputStyle=stream; rc=$?; echo "END rc=$rc wall=$(( $(date +%s) - s ))s $(date "+%F %T")"; exit $rc' >> "$out" 2>&1
echo "LANE-EXIT rc=$? $(date '+%F %T')" >> "$out"
