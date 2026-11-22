#!/bin/sh
# ♻️ Re-publishes every reclaimed asset whose catalog folder exists but whose manifest row or collection membership was lost to a concurrent manifest writer.
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
CATALOG="../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog"
for dir in assets/*/; do
  slug=$(basename "$dir")
  [ -f "$CATALOG/🧱️components/$slug/🔣️.json" ] || continue
  count=$(grep -c "\"$slug\"" "$CATALOG/🔣️.json")
  [ "$count" -ge 2 ] && continue
  "$BUN" publish.ts "$slug"
done
