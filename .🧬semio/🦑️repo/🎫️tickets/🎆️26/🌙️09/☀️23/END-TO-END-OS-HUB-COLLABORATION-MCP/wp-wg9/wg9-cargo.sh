#!/bin/zsh
# 🦀️ WG9 detached cargo step: waits until ≤ $WG9_RUSTC_BOUND (default 14) rustc run, then runs ONE command in the repo root with
# CARGO_INCREMENTAL=0, `nice -n 10` and the fleet's second build-dir (preamble 13 rule 26), and writes `[wg9] rc=<n>` as the capture's last line. usage:
#   setopt no_bg_nice; nohup zsh wg9-cargo.sh <capture> <cmd…> > /dev/null 2>&1 & disown
set -u
capture="$1"; shift
bound="${WG9_RUSTC_BOUND:-14}"
cd /Users/ueli/Documents/semio || exit 1
export CARGO_INCREMENTAL=0 CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/cargo/build-fleet-b"
echo "[wg9] queued $(date '+%F %T') pid $$: $*" > "$capture"
until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -le "$bound" ]; do sleep 30; done
echo "[wg9] start $(date '+%F %T')" >> "$capture"
nice -n 10 "$@" >> "$capture" 2>&1
rc=$?
echo "[wg9] rc=$rc $(date '+%F %T')" >> "$capture"
