#!/bin/sh
# 🧹 Validate the 152 original catalog assets as published (schema + validateAsset + print, no measured rules), one process at a time, resumable; it waits while the file `🗑️generated/finisher-sweep.pause` exists and writes `🗑️generated/finisher-sweep.busy` while a validation runs.
cd "$(dirname "$0")"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
OUT=🗑️generated/finisher-sweep.tsv
touch "$OUT"
cat rework/[0-9][0-9].txt | sort | while read -r a; do
  [ -z "$a" ] && continue
  grep -q "^$a	" "$OUT" && continue
  while [ -f 🗑️generated/finisher-sweep.pause ]; do sleep 3; done
  touch 🗑️generated/finisher-sweep.busy
  out=$(nice -n 10 "$BUN" r3-validate.ts "$a" --catalog-only 2>&1)
  rm -f 🗑️generated/finisher-sweep.busy
  last=$(printf '%s\n' "$out" | tail -1 | cut -c1-220)
  printf '%s\t%s\n' "$a" "$last" >> "$OUT"
  case "$last" in VALID*) ;; *) printf '%s\n' "$out" | head -40 | sed "s|^|$a	  |" >> "$OUT.issues";; esac
done
echo "[DEBUG] sweep done $OUT"
