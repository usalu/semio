#!/bin/sh
# 🪚️ Waits for the compiler process of one crate to start, writes its argument vector (UTF-8) to <out>, then replays it with each variant through the admitted capacity gate (`unit-variant.sh`), so the variants run while the dependency files of that very command line still exist.
# Usage: capture-unit-argv.sh <crate-name> <package> <version> <out-file> <variant-label> <flags...> [-- <variant-label> <flags...>]...
set -eu
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
crate=$1; package=$2; version=$3; out=$4; shift 4
export LC_ALL=en_US.UTF-8
while :; do
  pid=$(pgrep -f -- "^rustc --crate-name $crate " | head -1 || true)
  if [ -n "$pid" ]; then ps -ww -o args= -p "$pid" > "$out" 2>/dev/null && break; fi
  sleep 0.1
done
while [ $# -gt 0 ]; do
  label=$1; shift; flags=""
  while [ $# -gt 0 ] && [ "$1" != "--" ]; do flags="$flags $1"; shift; done
  [ $# -gt 0 ] && shift
  repo capacity run --memory=6144 -- sh "$here/🪚️unit-variant.sh" "$out" "$package" "$version" "$label" $flags || true
done
