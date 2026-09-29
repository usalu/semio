#!/bin/zsh
# 🧪️ WG11 session 14d: runs each named test of one lib test binary in its OWN process (nextest's isolation, the canonical runner's
# `RUST_MIN_STACK=134217728`), one TSV row per test (`rc<TAB>name<TAB>first panic line`) plus the full output per failing test.
# usage: zsh wg11-isolated.sh <test binary> <names file> <out.tsv> <details.txt> [RUST_MIN_STACK override]
set -u
bin="$1"; names="$2"; tsv="$3"; details="$4"; stack="${5:-134217728}"
: > "$tsv"; : > "$details"
while IFS= read -r name; do
  [ -z "$name" ] && continue
  out=$(RUST_MIN_STACK="$stack" RUST_BACKTRACE=1 "$bin" --exact "$name" --test-threads=1 2>&1)
  rc=$?
  if [ $rc -eq 0 ] && print -r -- "$out" | /usr/bin/grep -q "running 0 tests"; then rc=404; fi
  line=$(print -r -- "$out" | /usr/bin/grep -A1 "panicked at\|has overflowed\|fatal runtime" | /usr/bin/grep -v "panicked at\|^--" | head -1)
  printf '%s\t%s\t%s\n' "$rc" "$name" "${line:0:400}" >> "$tsv"
  if [ $rc -ne 0 ]; then
    { print -r -- "######## $name"; print -r -- "$out" | /usr/bin/grep -v "^test \|^running \|^$" | head -60; } >> "$details"
  fi
done < "$names"
echo "[wg11-iso] $(awk -F'\t' '$1==0' "$tsv" | wc -l | tr -d ' ') ok / $(awk -F'\t' '$1!=0' "$tsv" | wc -l | tr -d ' ') red of $(wc -l < "$tsv" | tr -d ' ')"
