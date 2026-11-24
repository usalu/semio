#!/bin/sh
# 🏗️ Builds the candidate of one person from the untouched copy: the hand-authored ops of `ops-3/` (where the person has a script there), then the restating of `prism-3.ts`; leaves `cand.json`, `pre.json` and `ops.json` in `🗑️generated/tv3/<id>/`. `build-3.sh <id> …`
HERE="$(cd "$(dirname "$0")" && pwd)"; U="$(dirname "$HERE")"
K="$U/../STAFFAGE-HUMAN-COLLECTION-WAVE"
S="$U/../../../../../../../🎓️teaching/🏛️architecture/🧍️staffage"
BUN=$(ls ~/Library/Caches/semio/runtimes/bun/*/bun | head -1)
for id in "$@"; do
  G="$U/🗑️generated/tv3/$id"
  case "$id" in
    parent-with-child-on-shoulders) PRE=parent-toe.py; SIDE= ;;
    person-carrying-ladder) PRE=ladder.py; SIDE= ;;
    person-hanging-laundry) PRE=; SIDE=plan:torso ;;
    person-with-arm-sling) PRE=sling-hand.py; SIDE=plan:hand-l ;;
    *) PRE=; SIDE= ;;
  esac
  if [ -n "$PRE" ]; then
    python3 "$HERE/ops-3/$PRE" "$G/before.json" "$G/pre.json" || exit 1
    (cd "$K/🧰️scripts/REP-B" && timeout 120 "$BUN" apply.ts "$id" --ops "$G/pre.json" --in "$G/before.json" --out "$G/cand.json") || exit 1
  else
    rm -f "$G/pre.json"; cp "$G/before.json" "$G/cand.json"
  fi
  (cd "$S" && timeout 600 "$BUN" "$HERE/prism-3.ts" "$id" --file "$G/cand.json" --ops "$G/ops.json" ${SIDE:+--side "$SIDE"} | grep -v "nothing to restate" | cut -c1-260)
done
