#!/usr/bin/env bash
# 🧾️ Regenerates every plugin's committed descriptor (`describe`), then activates play's 28 lanes — ONE mutex job.
set -u
cd /Users/ueli/Documents/semio || exit 1
export DEVELOPER_DIR=/Library/Developer/CommandLineTools CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432
P=$(bun nx show projects --projects '@semio-tech/*-plugin' --with-target describe 2>/dev/null | python3 -c "import json,sys;print(','.join(p for p in json.load(sys.stdin) if p!='@semio-tech/framework-plugin'))")
echo "== $(date '+%H:%M:%S') describe: $P"
bun nx run-many -t describe --projects "$P" --parallel=2 --nx-bail=false; rc1=$?
echo "== $(date '+%H:%M:%S') describe rc=$rc1"
echo "== $(date '+%H:%M:%S') activate-dev"
bun nx run @semio-tech/semio-tech-play:activate-dev; rc2=$?
echo "== $(date '+%H:%M:%S') activate-dev rc=$rc2"
exit $rc2
