#!/bin/zsh
# ⚖️ H14 14b: db credit fix compile proof through the native lane (build-fleet-b, private target): `cargo check` of
# `semio-framework-os-kernel-db` (lib + tests) then `semio-hub` (lib + bins + tests); capture → <capture>.
# FLEET_TICKET_STAMP (optional) orders the ticket in the FIFO queue (coordinator-approved priority only).
OUT=$1
cd /Users/ueli/Documents/semio
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b" CARGO_TARGET_DIR=/Users/ueli/Documents/semio/.tmp-ticket/wp-h14/target
echo "=== queued $(date +%T)" > "$OUT"
zsh /Users/ueli/Documents/semio/.tmp-ticket/📜️fleet-mutex.sh native h14 -- zsh -c 'echo "=== start $(date +%T)"; nice -n 5 cargo check -p semio-framework-os-kernel-db --lib --tests --message-format short; echo "=== DB EXIT $? $(date +%T)"; nice -n 5 cargo check -p semio-hub --lib --bins --tests --message-format short; echo "=== HUB EXIT $? $(date +%T)"' >> "$OUT" 2>&1
echo "=== EXIT $? $(date +%T)" >> "$OUT"
