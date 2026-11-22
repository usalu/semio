#!/usr/bin/env bash
# usage: l1d-driver.sh pkg... ; sequential per-package tests through the l1d session
G="$(cd "$(dirname "$0")" && pwd)/🗑️generated"
for p in "$@"; do
  log="l1d-test-${p#semio-s-artifact-puzzle-}.log"
  printf '%s\ntest\n--no-fail-fast\n-p\n%s\n' "$log" "$p" > "$G/l1d-req"
  until grep -q '^done$' "$G/$log" 2>/dev/null; do sleep 5; done
  echo "$p $(grep -E '^exit=' "$G/$log")" >> "$G/l1d-summary.txt"
done
