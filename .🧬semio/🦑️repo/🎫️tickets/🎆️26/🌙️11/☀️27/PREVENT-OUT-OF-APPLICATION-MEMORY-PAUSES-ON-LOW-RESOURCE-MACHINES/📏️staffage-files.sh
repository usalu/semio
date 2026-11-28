#!/usr/bin/env bash
# 📏️ Peak resident memory of every staffage test case in a process of its own: run inside one capacity grant; rows go to 🗑️generated/staffage-files.tsv — usage: [PASS=after] 📏️staffage-files.sh [case-substring...]
set -uo pipefail
HERE="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../.." && pwd)"
S="$ROOT/🎓️teaching/🏛️architecture/🧍️staffage"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
OUT="$HERE/🗑️generated"
NAME="staffage-files${PASS:+-$PASS}"
mkdir -p "$OUT/$NAME"
cd "$ROOT"
for dir in "$S"/🧪️tests/*/; do
  name="$(basename "$dir")"
  [ -f "$dir/🟦️.ts" ] || continue
  [ "$name" = "🎚️config" ] && continue; [ "$name" = "🧊️model-catalog" ] && continue
  if [ "$#" -gt 0 ]; then ok=0; for p in "$@"; do case "$name" in *"$p"*) ok=1;; esac; done; [ $ok = 1 ] || continue; fi
  [ -f "$OUT/$NAME/$name.log" ] && continue
  /usr/bin/time -l timeout "${TIMEOUT:-1500}" "$BUN" node_modules/vitest/vitest.mjs run --configLoader native --maxWorkers=1 --config "$S/🧪️tests/🎚️config/🟦️.ts" "/$name/" --testTimeout=3600000 --hookTimeout=3600000 > "$OUT/$NAME/$name.log" 2>&1
  code=$?
  rss=$(grep "maximum resident" "$OUT/$NAME/$name.log" | awk '{print int($1/1048576)}')
  secs=$(grep " real " "$OUT/$NAME/$name.log" | awk '{print $1}')
  echo -e "$name\t$code\t${rss:-?}\t${secs:-?}" | tee -a "$OUT/$NAME.tsv"
done
