#!/usr/bin/env bash
# Runs the scene tests of the given "family-dir|package..." entries one at a time, retrying on in-flight compile breaks.
# Usage: run-norm-scene-chain.sh <log-prefix> <entry>...   entry = "<family>|<pkg> <pkg>"
set -u
T="$(cd "$(dirname "$0")" && pwd)"
ROOT="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts"
prefix="$1"; shift
for entry in "$@"; do
  family="${entry%%|*}"; pkgs="${entry#*|}"
  dir="$ROOT/$family/🏅️standards/🔖️1/🪆️subsets/✳️any/🎬️scene/📦️packages/🦀️rust"
  args=(); for p in $pkgs; do args+=(-p "$p"); done
  log="$T/🗑️generated/$prefix-${pkgs%% *}.log"
  for attempt in 1 2 3; do
    (cd "$dir" && repo cargo --store=shared --configuration=prototype -- test "${args[@]}" --offline > "$log" 2>&1)
    code=$?
    if [ $code -eq 0 ] || ! grep -q "^error\(\[E[0-9]*\]\)\?: \(could not compile\|cannot find\|unresolved\|expected\)" "$log"; then break; fi
    sleep 30
  done
  echo "$family $pkgs exit=$code" >> "$T/🗑️generated/$prefix-summary.txt"
done
echo done >> "$T/🗑️generated/$prefix-summary.txt"
