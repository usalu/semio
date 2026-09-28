#!/bin/zsh
# 🧪️ C13: P1 overlay proof under ONE overlay-lane hold — the replication crate's fold laws (Rust, private build-dir inside the
# overlay) and the replication package's in-source laws (the TS fold twin replaying the same fixture).
OV="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-c13-overlay"
cd "$OV" || exit 1
export CARGO_BUILD_BUILD_DIR="$OV/.c13-build" CARGO_TARGET_DIR="$OV/.c13-target" CARGO_INCREMENTAL=0 NX_DAEMON=false
echo "START $(date '+%F %T')"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh overlay c13 -- zsh -c '
  nice -n 15 cargo test -p semio-framework-replication --lib --no-fail-fast -- transition; echo "RUST rc=$? $(date "+%F %T")"
  cd "$0/🧰️framework/🔨️modules/📡️replication/📦️packages/🟦️typescript" && nice -n 15 bunx vitest run --config "../../🧪️tests/🎚️config/🟦️.ts"; echo "TS rc=$? $(date "+%F %T")"
' "$OV"
echo "EXIT rc=$? $(date '+%F %T')"
