#!/bin/zsh
# 🧩️ S15: builds the os-hub binary from the tree on a private target dir and installs it into S15's durable hub data root.
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-s15/generated/target
echo "[s15] build os-hub $(date '+%T')"
cargo build -p semio-hub --bin os-hub 2>&1 | tail -5
rc=${pipestatus[1]}
[ $rc -ne 0 ] && { echo "[s15] build failed rc=$rc"; exit $rc; }
BIN="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s11-s15-hub-8040/bin/os-hub.next"
rm -f "$BIN" && cp "$CARGO_TARGET_DIR/debug/os-hub" "$BIN" && codesign --force --sign - "$BIN" 2>&1 | tail -1
echo "[s15] built $(date '+%T') $(shasum -a 256 "$BIN" | cut -c1-16)"
