#!/bin/sh
# 🐍️ Laws of batch `fleetpy` after apply: sh verify.sh <MIRROR root>. The fleet-status law runs with the toolchain's Python; the suite has an explicit verdict; the independent Python reading of the platform evidence finds no host-bound fact; negative control: the base table leaves the suite unjudged.
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../../.." && pwd); XPLAT5="$C/xplat5-tools"; . "$XPLAT5/verify-common.sh"
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" >/dev/null || { echo "batch not applied in $ROOT"; exit 2; }
P="🧰️framework/🔨️modules/🏃️process/🧪️testing/🗂️suites/🖥️platforms"
law "fleet-status laws through the toolchain's Python" bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/🧪️tests/🟦️.ts"
law "every suite has an explicit and explained platform verdict" bun test "./$P/🧪️tests/🟦️.ts" -t "explicit and explained"
causes() { python3 -I -c "
import importlib.util, json, glob
spec=importlib.util.spec_from_file_location('o','$P/🧪️tests/🐍️oracle/🐍️.py'); o=importlib.util.module_from_spec(spec); spec.loader.exec_module(o)
found=o.suite_causes(sorted(glob.glob('🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🚢️fleet-status/**/🧪️tests/**/🟦️.ts',recursive=True)),o.compile_policy(json.load(open('$P/🔣️.json',encoding='utf-8'))))
print('causes', found); raise SystemExit(1 if found else 0)"; }
law "independent Python reading: fleet-status has no host-bound fact" causes
TABLE="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🗂️suites/🔣️.json"; FILE=$(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1],encoding='utf8'))[sys.argv[2]]['file'])" "$HERE/manifest.json" "$TABLE")
cp "$HERE/base/$FILE" "$ROOT/$TABLE"; out=$(bun test "./$P/🧪️tests/🟦️.ts" -t "explicit and explained" 2>&1); code=$?; cp "$HERE/files/$FILE" "$ROOT/$TABLE"
if [ $code -eq 0 ]; then echo "RED   negative control DID NOT BITE with the base table"; FAILED=1; else echo "GREEN negative control: unjudged with the base table ($(printf '%s\n' "$out" | grep -c 'build-fleet-status') mention)"; fi
exit $FAILED
