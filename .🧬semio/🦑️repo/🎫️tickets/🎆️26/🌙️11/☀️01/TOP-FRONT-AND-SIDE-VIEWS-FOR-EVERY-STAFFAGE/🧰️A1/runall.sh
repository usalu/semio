#!/usr/bin/env bash
# [DEBUG] build all A1 assets, verify in one bun process (dumps rings), then check the animals category in one bun process
export PYTHONIOENCODING=utf-8
HERE="$(cd "$(dirname "$0")" && pwd)"
cd "$HERE"
MODS="${1:-horse dairy_cow domestic_cat goat hedgehog herring_gull labrador_dog}"
python build.py $MODS
IDS=$(echo "$MODS" | sed 's/_/-/g; s/ /,/g')
cd "$HERE/../../../../../../../.."
repo tools exec bun -- "$HERE/verify.ts" "$IDS" 50,100,200,500 > "$HERE/../🗑️generated/A1/verify-final.log" 2>&1
echo VERIFY-DONE
repo tools exec bun -- 🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/🗂️catalog/🩺️check/🟦️.ts --category animals
