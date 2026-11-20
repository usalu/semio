#!/usr/bin/env bash
# Builds and stages each listed plugin or extension module for the browser, one module per process, with bounded parallelism. Usage: native-modules.sh <parallelism> <plugin-id>...
set -u
T="$(cd "$(dirname "$0")" && pwd)"; G="$T/🗑️generated"; P="$1"; shift
one() { local id="$1"; [ -f "$G/done/browser-$id" ] && { echo "[skip] browser-$id"; return 0; }; "$T/native-lanes.sh" step "browser-$id" "" repo ./📜️script.ts components describe "$id" --facet browser || { grep -q " 0 failure(s) in " "$G/browser-$id.log" && touch "$G/done/browser-$id" && echo "[ok] browser-$id (exit 75 after success)"; }; }
export -f one; export T G
printf '%s\n' "$@" | xargs -P "$P" -I{} bash -c 'one "$@"' _ {}
for id in "$@"; do [ -f "$G/done/browser-$id" ] || { echo "MISSING $id"; exit 1; }; done
