#!/bin/sh
# usage: veg-finish.sh <slug> [insert]   runs author script, check, optional splice + validation
T="$(cd "$(dirname "$0")" && pwd)"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun)
S="$T/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
G="$T/🗑️generated/authoring-vegetation"
slug="$1"
$BUN "$T/volumes/vegetation/$slug.author.ts" || exit 1
if [ "$2" = "insert" ]; then
  python3 "$T/volume-insert.py" "$T/volumes/vegetation/$slug.json" "$S/🗂️catalog/🌳️vegetation/$slug/🔣️.json" || exit 1
  $BUN "$T/volume-check.ts" "$slug" --json > "$G/$slug.check.json"
  $BUN "$T/volume-check.ts" "$slug" | grep -E "IoU|x \(|y \(|z \(|PASS|FAIL"
  $BUN "$T/veg-validate.ts" "$slug" --poses > "$G/$slug.validate.txt" 2>&1
  head -3 "$G/$slug.validate.txt" | cut -c1-200
  echo "poses/clips evaluated: $(grep -c 'pose:\|clip:' "$G/$slug.validate.txt"); worst joint gap: $(grep -o 'joint gap +[0-9.]*' "$G/$slug.validate.txt" | sort -k3 | tail -1); failed evals: $(grep -c 'evaluate failed' "$G/$slug.validate.txt")"
else
  $BUN "$T/volume-check.ts" "$slug" --volume "$T/volumes/vegetation/$slug.json" | grep -E "IoU|x \(|y \(|z \(|PASS|FAIL|outside|misses"
fi
