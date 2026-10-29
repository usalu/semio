#!/bin/bash
# usage: act.sh <label> [repo args...]  (default: activate-prototype on bazel)
T="$(cd "$(dirname "$0")/.." && pwd)"; L="$1"; shift
[ $# -eq 0 ] && set -- --tool=bazel run @semio-tech/mit-bestand-demonstrator:activate-prototype
cd /Users/ueli/Documents/semio
O="$T/🗑️generated/1029/$L"
echo "start $(date '+%F %T') repo $*" > "$O.status"
SEMIO_LANGUAGE=en repo "$@" > "$O.log" 2>&1
echo "exit $? $(date '+%F %T')" >> "$O.status"
