#!/bin/sh
# ✅ Finisher A check: the staffage check CLI on candidate documents (`--file`), one process at a time; the full output goes to 🗑️generated/finish-a/check-<folder name>.txt, the verdict line is printed.
# usage: check.sh <asset document json>...
HERE="$(cd "$(dirname "$0")" && pwd)"
S="$HERE/../../../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
G="$HERE/../../🗑️generated/finish-a"
BUN="$(ls "$HOME"/Library/Caches/semio/runtimes/bun/*/bun | head -1)"
mkdir -p "$G"
for file in "$@"; do
  name="$(basename "$(dirname "$file")")"
  (cd "$S" && "$BUN" 🔨️modules/🗂️catalog/🩺️check/🟦️.ts --file "$file" > "$G/check-$name.txt" 2>&1)
  echo "[DEBUG] $name: $(grep -m1 -e '^✓' -e '^✗' "$G/check-$name.txt" | cut -c1-80) ($(grep -c . "$G/check-$name.txt") lines)"
done
