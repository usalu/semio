#!/bin/sh
# Runs single test files graph-free: sh runtests.sh <mirror name|live> <list file: one directory per line, relative to the library module or starting with 🧰️framework/>
# prints pass/fail totals and the names of failing tests per directory.
C="$(cd "$(dirname "$0")/.." && pwd)"; LIVE="$(cd "$C/../../../../../../.." && pwd)"
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
WHERE="$1"
while IFS= read -r D; do
  [ -z "$D" ] && continue
  case "$D" in 🧰️framework/*) REL="$D";; *) REL="$L/$D";; esac
  if [ "$WHERE" = live ]; then
    OUT="$(cd "$LIVE/$REL" && nice -n 10 bun test ./🟦️.ts 2>&1)"
  else
    OUT="$(python3 -I "$C/mirror-tools/mirror.py" run "$WHERE" --cwd "$REL" -- nice -n 10 bun test ./🟦️.ts 2>&1)"
  fi
  printf '== %s: %s pass, %s fail\n' "$D" "$(printf '%s\n' "$OUT" | /usr/bin/grep -E '^ *[0-9]+ pass$' | tr -d ' ' | sed 's/pass//')" "$(printf '%s\n' "$OUT" | /usr/bin/grep -E '^ *[0-9]+ fail$' | tr -d ' ' | sed 's/fail//')"
  printf '%s\n' "$OUT" | /usr/bin/grep -E '^\(fail\)' | cut -c1-150
done < "$2"
