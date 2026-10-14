#!/bin/bash
# 🔁️ Runs one P8 cargo check, applies `🩺️fix-deps.py` and repeats while it adds dependencies (max 6 rounds).
# usage: 🔁️check-loop.sh <tag> <workspace Cargo.toml> <cargo check args...>
TAG=$1; WS=$2; shift 2
HERE="$(cd "$(dirname "$0")" && pwd)"; LOG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/p8/logs/$TAG.log"
for i in 1 2 3 4 5 6; do
  "$HERE/🏗️cargo.sh" "$TAG" check --manifest-path "$WS" "$@" --message-format short && exit 0
  added=$(python3 "$HERE/🩺️fix-deps.py" "$LOG" "$WS" apply)
  echo "$added"
  [ -z "$added" ] && break
done
grep -E "error(\[E[0-9]+\])?:" "$LOG" | grep -v "^error: could not compile" | sed -E 's/^.*\/(🦀️\.rs|[^\/]*\.rs):/\1:/' | sort | uniq -c | sort -rn | head -40
exit 1
