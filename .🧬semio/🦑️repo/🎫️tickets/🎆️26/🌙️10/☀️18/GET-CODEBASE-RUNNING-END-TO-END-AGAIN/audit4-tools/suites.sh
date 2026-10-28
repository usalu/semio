#!/bin/sh
# [DEBUG] audit4: runs every host-capacity law file singly in <root>, prints "<file> pass fail" rows (failure sets compared before/after).
root="$1"; out="$2"; cd "$root/🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity" || exit 2
: > "$out"
for f in $(find 🧪️tests -name '🟦️.ts' ! -path '*📜️child*' | sort); do
  r=$(bun test "./$f" --timeout 120000 2>&1 | tail -40)
  p=$(printf '%s\n' "$r" | sed -n 's/^ *\([0-9][0-9]*\) pass.*/\1/p' | tail -1); x=$(printf '%s\n' "$r" | sed -n 's/^ *\([0-9][0-9]*\) fail.*/\1/p' | tail -1)
  fails=$(printf '%s\n' "$r" | grep '^(fail)' | sed 's/ \[[0-9.]*m\{0,1\}s\]$//' | tr '\n' '|')
  echo "$f ${p:-?} ${x:-?} $fails" >> "$out"
done
cat "$out"
