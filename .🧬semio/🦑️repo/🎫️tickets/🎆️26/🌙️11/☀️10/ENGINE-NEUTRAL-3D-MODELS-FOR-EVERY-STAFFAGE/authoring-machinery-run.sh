#!/bin/sh
# 🏃️ Builds, checks, splices and validates one machinery asset: sh authoring-machinery-run.sh <slug> [--splice]
cd "$(dirname "$0")" || exit 1
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
SLUG="$1"
CAT=$(ls -d ../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/*machinery)
python3 "volumes/machinery/$SLUG.py" || exit 1
if [ "$2" = "--splice" ]; then
  python3 volume-insert.py "volumes/machinery/$SLUG.json" "$CAT/$SLUG/🔣️.json" || exit 1
  $BUN volume-check.ts "$SLUG" | tee "🗑️generated/authoring-machinery/$SLUG.check.txt"
  $BUN authoring-machinery-validate.ts "$SLUG"
  $BUN authoring-machinery-poses.ts "$SLUG" | tail -40
else
  $BUN volume-check.ts "$SLUG" --volume "volumes/machinery/$SLUG.json" | tee "🗑️generated/authoring-machinery/$SLUG.check.txt"
fi
