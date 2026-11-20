#!/usr/bin/env bash
# Builds each listed plugin as its own browser component in parallel processes, so a failure of one does not discard the others. Usage: native-sep.sh <plugin-id>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"
for id in "$@"; do
  ( rm -f "$G/done/browser-$id"; [ -f "$G/browser-$id.log" ] && mv "$G/browser-$id.log" "$G/browser-$id.log.prev"
    "$T/native-lanes.sh" step "browser-$id" "" repo ./📜️script.ts components describe "$id" --facet browser || { grep -q " 0 failure(s) in " "$G/browser-$id.log" && echo "[ok] browser-$id (exit 75 after success)"; } ) &
done
wait
