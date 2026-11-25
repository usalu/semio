#!/bin/sh
# 🔗️ Runs link-only replays through the admitted capacity gate, one at a time, retrying while the gate gave up waiting, and appends each measurement line to <log>.
# Usage: [MEMORY=<MB>] link-only-run.sh <log> <variant-dir> <label> <cc args...> [-- <label> <cc args...>]...
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
log=$1; variant=$2; shift 2
while [ $# -gt 0 ]; do
  label=$1; shift; flags=""
  while [ $# -gt 0 ] && [ "$1" != "--" ]; do flags="$flags $1"; shift; done
  [ $# -gt 0 ] && shift
  until line=$(repo capacity run --memory="${MEMORY:-4300}" -- python3 -I "$here/🔗️link-only.py" "$variant" "$label" $flags 2>/dev/null | grep "wall="); do sleep 20; done
  echo "$line" >> "$log"
done
