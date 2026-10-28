#!/bin/sh
# 🧷️ verify.sh <root>: the staged batch is applied, both shell scripts parse, the three TS files parse (gate scan), the JSON is valid, both PowerShell files keep their BOM and the cross-script literals agree, the installation and install-contract suites pass (the two envelope-only tests of the installation suite are accepted bare and proven through the envelope runner when given: INSTALLOWN_MRUN="sh <mrun.sh> <mirror-name>"), and the stage tool keeps file modes.
set -u
ROOT=${1:?usage: verify.sh <root>}
HERE=$(cd "$(dirname "$0")" && pwd)
C=$(cd "$HERE/../../.." && pwd)
M="🧰️framework/🛍️products/🦑️repo/🔨️modules"
fail=0
python3 -I "$HERE/stage.py" check "$ROOT" >/dev/null || { echo "RED stage check"; fail=1; }
cd "$ROOT" || exit 1
for script in "$M/🚀️launcher/📥️install/🐚️.sh" "$M/🔩️native/🥾️bootstrap/🐚️.sh"; do bash -n "$script" || { echo "RED bash -n $script"; fail=1; }; done
for json in "$M/💻️client/🔌️mcp/📥️installation/🧬️schema/🔣️.json" "$M/💻️client/🔌️mcp/📥️installation/🧫️fixtures/🔣️.json"; do python3 -I -c 'import json,sys;json.load(open(sys.argv[1],encoding="utf8"))' "$json" || { echo "RED json $json"; fail=1; }; done
for ps in "$M/🚀️launcher/📥️install/🔵️.ps1" "$M/🔩️native/🥾️bootstrap/🔵️.ps1"; do [ "$(head -c 3 "$ps" | od -An -tx1 | tr -d ' \n')" = efbbbf ] || { echo "RED missing BOM $ps"; fail=1; }; done
scratch="$C/🗑️generated/installown"; mkdir -p "$scratch"
printf '%s\n' "$M/💻️client/🔌️mcp/📜️script.ts" "$M/💻️client/🔌️mcp/📥️installation/🧪️tests/🟦️.ts" "$M/🚀️launcher/🧪️tests/📥️install-contract/🟦️.ts" >"$scratch/verify-scan.list"
bun "$C/gate-g-tools/scan.ts" "$scratch/verify-scan.list" "$scratch/verify-scan" | /usr/bin/grep -q "bad 0" || { echo "RED scan"; fail=1; }
run() { out=$(bun test "./$1" 2>&1); echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)$' | tr -s ' \n' ' '; echo "$1" | sed 's#.*/\(.*\)/.*#\1#'; echo "$out" | /usr/bin/grep -E '^\(fail\)' | cut -c1-160 >"$scratch/verify-fails.txt"; }
run "$M/🚀️launcher/🧪️tests/📥️install-contract/🟦️.ts"
[ ! -s "$scratch/verify-fails.txt" ] || { echo "RED install-contract:"; cat "$scratch/verify-fails.txt"; fail=1; }
run "$M/💻️client/🔌️mcp/📥️installation/🧪️tests/🟦️.ts"
unexpected=$(/usr/bin/grep -v -e "managed POSIX profiles deduplicate" -e "the actual installation, in a child process" "$scratch/verify-fails.txt")
[ -z "$unexpected" ] || { echo "RED installation:"; echo "$unexpected"; fail=1; }
if [ -n "${INSTALLOWN_MRUN:-}" ]; then $INSTALLOWN_MRUN env "$M/💻️client/🔌️mcp/📥️installation/🧪️tests/🟦️.ts" | /usr/bin/grep -q " 0 fail" || { echo "RED installation through the envelope"; fail=1; }; fi
python3 -I "$C/xplat5-tools/tests/test_stage_mode.py" "$HERE/stage.py" >/dev/null 2>&1 || { echo "RED stage mode law"; fail=1; }
echo "bootown verify exit $fail"
exit $fail
