#!/bin/zsh
# 🎚️ H13 hold 12 (session 14c, items 3/5): the all-driver os-hub (sqlite + postgres + neo4j) from today's tree incl. the kernel-db
# SQLite readers / retirement wake / cleanup-fault routing, copied (rm + cp + codesign) to s14-h13-bin for the live gates on ALL.
# usage: h13-hold-12.sh <label>
W=/Users/ueli/Documents/semio/.tmp-ticket/wp-h13
BIN="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-h13-bin/os-hub-all-drivers-$(date +%H%M)"
zsh $W/h13-cargo.sh "$1-os-hub" build -p semio-hub --bin os-hub --features postgres,neo4j || exit 1
rm -f "$BIN" && cp "$W/target/debug/os-hub" "$BIN" && codesign -f -s - "$BIN" && echo "binary $BIN"
