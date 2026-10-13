#!/bin/bash
# 🏗️ Runs one foreground cargo command for wave E1b through the fleet cargo slot, with the private E1b target and build dirs.
# usage: 🏗️cargo.sh <log> <cargo args...>   (prints error/warning counts and the first errors; exit code is cargo's)
REPO=/Users/ueli/Documents/semio; cd "$REPO" || exit 2
E1B="$REPO/.🧬semio/🦑️repo/⚡️cache/e1b"; LOG=$1; shift
env CARGO_TARGET_DIR="$E1B/t" CARGO_BUILD_BUILD_DIR="$E1B/b" CARGO_TERM_COLOR=never \
  "$REPO/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️23/BUILD-SEMIO-TECH-PLAY-FOR-CDN-DEPLOYMENT/🚦️cargo-slot.sh" e1b-kernel -- cargo "$@" > "$LOG" 2>&1
RC=$?
echo "rc=$RC errors=$(grep -cE '^error' "$LOG") warnings=$(grep -cE '^warning' "$LOG")"
grep -E '^error' -A6 "$LOG" | head -${HEADN:-40}
exit $RC
