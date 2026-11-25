#!/bin/sh
# 🔗️ Replays the captured argument vector with each variant through the admitted capacity gate, one at a time, retrying a variant while the gate gave up waiting (the host was busy), and appends the measurement line of each to <log>.
# Usage: [MEMORY=<MB reserved at the gate>] link-run.sh <log> <label> <flags...> [-- <label> <flags...>]...
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
log=$1; shift
out=$here/🗑️generated/link/argv.txt
dir="✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation"
while [ $# -gt 0 ]; do
  label=$1; shift; flags=""
  while [ $# -gt 0 ] && [ "$1" != "--" ]; do flags="$flags $1"; shift; done
  [ $# -gt 0 ] && shift
  until line=$(repo capacity run --memory="${MEMORY:-6144}" -- sh "$here/🔗️link-variant.sh" "$out" "$dir" semio-s-artifact-sourcing-curation 0.1.0 "$label" $flags 2>/dev/null | grep "wall="); do sleep 20; done
  echo "$line" >> "$log"
done
