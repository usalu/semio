#!/usr/bin/env bash
# usage: l1d-retry.sh <log> <pkg> ; retries while a non-puzzle crate fails to compile
G="$(cd "$(dirname "$0")" && pwd)/🗑️generated"; log=$1; pkg=$2
for i in $(seq 1 12); do
  rm -f "$G/$log"; printf '%s\ntest\n--no-fail-fast\n-p\n%s\n' "$log" "$pkg" > "$G/l1d-req"
  until grep -q '^done$' "$G/$log" 2>/dev/null; do sleep 10; done
  grep -q "could not compile" "$G/$log" || break
  sleep 240
done
echo finished
