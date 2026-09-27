#!/bin/zsh
# 🦀️ AV1 overlay cargo lane: one cargo at a time, private build/target dirs, nice 10, no incremental.
# Usage: zsh av1-cargo.sh <log-name> <cargo args...>
setopt no_bg_nice
OVERLAY="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-overlay"
LOG="/Users/ueli/Documents/semio/.tmp-ticket/wp-av1/generated/$1.txt"
shift
export CARGO_BUILD_BUILD_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-build"
export CARGO_TARGET_DIR="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s13-av1-target"
export CARGO_INCREMENTAL=0
cd "$OVERLAY" || exit 2
until [ "$(ps -axo command | /usr/bin/grep -c '^[^ ]*rustc ')" -lt 10 ]; do sleep 30; done
echo "[av1-cargo] start $(date +%H:%M:%S) cargo $*" > "$LOG"
nice -n 10 cargo "$@" >> "$LOG" 2>&1
echo "[av1-cargo] exit=$? end $(date +%H:%M:%S)" >> "$LOG"
