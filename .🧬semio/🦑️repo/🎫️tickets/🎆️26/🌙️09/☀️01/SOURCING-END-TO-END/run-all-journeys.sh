#!/bin/sh
# 🧭️ Runs every sourcing journey (core, editing, history, surfaces) in English and German one after another against the host on 6081 and writes one log per journey to `🗑️generated/all-journeys/`.
# @see ./journey-kit.ts
set -u
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
out="$here/🗑️generated/all-journeys"; mkdir -p "$out"
BUN=$(ls -d ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for pair in journey:sourcing-journey editing:journey-editing history:journey-history surfaces:journey-surfaces; do
  name=${pair%%:*}; file=${pair#*:}
  JOURNEY=$name "$BUN" "$here/$file.ts" ${LANGS:-en de} > "$out/$name.txt" 2>&1; echo "$name exit $?" >> "$out/summary.txt"
done
