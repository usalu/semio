#!/bin/sh
# 🍱 custody verify on a root (mirror, or live after apply): `sh verify.sh <root>`. Laws: the staged batch is applied; the resolution-input suite (chunking vs an independent Python decoder, plus a real closure of 260 import edges beyond the 64 KiB text limit verified through the actual resolver child) and the custody control model pass. Red on base: `Bounded own-data UTF-8 child input required` (os-hub:test, hubreds-2) and the missing export.
ROOT="$1"; H=$(cd "$(dirname "$0")" && pwd); L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody"
python3 -I "$H/stage.py" check --root "$ROOT" | grep -v '^applied' && exit 1
sh "$H/../../vchain.sh" "$ROOT" "$L/🧪️tests/🧪️resolution-inputs/🟦️.ts" "$L/🧪️tests/🎛️control/🟦️.ts"
