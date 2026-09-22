#!/usr/bin/env bash
# 🧾️ Regenerates the committed descriptor of the given plugin projects ONE nx invocation at a time (peer rule: one wasm cargo per mutex hold). Usage: describe-plugins.sh stdio mathematical …
set -u
cd /Users/ueli/Documents/semio || exit 1
export DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 NX_DAEMON=false
rc=0
for p in "$@"; do
  echo "== $(date '+%H:%M:%S') describe $p"
  bun nx run "@semio-tech/$p-plugin:describe" --parallel=1 || { rc=1; echo "== $(date '+%H:%M:%S') describe $p FAILED"; }
done
echo "== $(date '+%H:%M:%S') describe-plugins rc=$rc"; exit $rc
