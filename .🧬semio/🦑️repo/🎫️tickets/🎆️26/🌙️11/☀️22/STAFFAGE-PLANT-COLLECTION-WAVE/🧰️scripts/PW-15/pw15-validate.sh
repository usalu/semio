#!/bin/sh
# ✅ Validates PW-15 catalog assets with the round-3 validator (schema, validateAsset, print ranges, checkVolumes) from its place; one report per asset in 🗑️generated/PW-15/<id>-validate.txt. Usage: sh pw15-validate.sh <id> [<id> ...]
W="$(cd "$(dirname "$0")/../.." && pwd)"
R3="$W/../../☀️14/PERFECT-HANDCRAFTED-3D-MODELS-PER-SCALE-AND-MODE-FOR-EVERY-STAFFAGE"
CAT="$(cd "$W/../../../../../../.." && pwd)/🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🌳️vegetation"
cd "$R3" || exit 1
for ID in "$@"; do
  repo tools exec bun -- r3-validate.ts --file "$CAT/$ID/🔣️.json" --volumes 2>&1 | grep -v "capacity\|still working" > "$W/🗑️generated/PW-15/$ID-validate.txt"
  echo "== $ID"; cut -c1-400 "$W/🗑️generated/PW-15/$ID-validate.txt"
done
