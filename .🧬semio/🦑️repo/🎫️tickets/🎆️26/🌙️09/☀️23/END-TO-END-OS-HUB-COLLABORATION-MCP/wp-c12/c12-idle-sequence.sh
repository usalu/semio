#!/bin/zsh
# 🧾️ C12: waits until no native/wasm/overlay lane job runs (preamble rule 25), then — one after the other — the rule-20 boot to Home
# (one `serve s react dev` via S18's `ensureDevServe`, local-only) and the c12-splice scratch proof in the overlay lane.
# usage: zsh c12-idle-sequence.sh <capture dir>
DIR="$1"; mkdir -p "$DIR"
cd /Users/ueli/Documents/semio/.tmp-ticket/wp-c12 || exit 1
echo "WAIT $(date +%T)" > "$DIR/sequence.txt"
while [ -d /tmp/semio-native-build.lock ] || [ -d /tmp/semio-wasm-build.lock ] || [ -d /tmp/semio-overlay-build.lock ]; do sleep 60; done
echo "IDLE $(date +%T)" >> "$DIR/sequence.txt"
NX_DAEMON=false bun c12-boot.ts 6520 > "$DIR/boot-14c-3.txt" 2>&1
echo "BOOT rc=$? $(date +%T)" >> "$DIR/sequence.txt"
zsh splice/scratch-proof.sh "$DIR/splice-scratch-3.txt"
echo "SCRATCH rc=$? $(date +%T)" >> "$DIR/sequence.txt"
