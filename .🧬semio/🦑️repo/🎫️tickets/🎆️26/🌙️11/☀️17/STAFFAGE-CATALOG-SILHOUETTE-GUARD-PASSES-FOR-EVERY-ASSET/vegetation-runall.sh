#!/bin/bash
HERE="$(cd "$(dirname "$0")" && pwd)"
n=0
for s in "$@"; do
  "$HERE/vegetation-all.sh" "$s" &
  n=$((n+1))
  if [ $((n % ${PAR:-3})) -eq 0 ]; then wait; fi
done
wait
