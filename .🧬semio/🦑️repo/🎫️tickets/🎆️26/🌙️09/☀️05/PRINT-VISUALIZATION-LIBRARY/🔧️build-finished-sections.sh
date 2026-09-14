#!/usr/bin/env bash
# 🏗️ Builds every gallery section owned by a finished agent and reports the first LaTeX error of each.
set -u
TS="C:/git/semio/🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript"
SECTIONS="${*:-1 2 3 4 5 6 11 12 17 18 19 20 23 24 42 48 49 50 51 52 60 62 65 68 69 70 72 73 74 77 78}"
cd "$TS" || exit 1
for s in $SECTIONS; do
  printf '%s: ' "$s"
  if bun ./📜️script.ts build viz "$s" > /dev/null 2>&1; then
    echo OK
  else
    grep -m 1 -A 4 '^!' "dist/viz-$s.log" 2>/dev/null | tr '\n' ' ' | cut -c1-200
    echo
  fi
done
