#!/bin/sh
# 🔁️ Laws of batch `wininstall` after apply: sh verify.sh <MIRROR root>. The install contract (POSIX runs of the shell installer plus the text laws of the PowerShell installer) is green; negative control: the base PowerShell installer fails the replacement-order law. The PowerShell itself runs only on Windows (no pwsh here).
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../../.." && pwd); XPLAT5="$C/xplat5-tools"; . "$XPLAT5/verify-common.sh"
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" >/dev/null || { echo "batch not applied in $ROOT"; exit 2; }
T="./🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/🧪️tests/📥️install-contract/🟦️.ts"
law "install contract" bun test "$T"
PS="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📥️install/🔵️.ps1"; FILE=$(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1],encoding='utf8'))[sys.argv[2]]['file'])" "$HERE/manifest.json" "$PS")
cp "$HERE/base/$FILE" "$ROOT/$PS"; out=$(bun test "$T" -t "replaces a running launcher" 2>&1); code=$?; cp "$HERE/files/$FILE" "$ROOT/$PS"
if [ $code -eq 0 ]; then echo "RED   negative control DID NOT BITE with the base installer"; FAILED=1; else echo "GREEN negative control: the replacement law is red with the base installer"; fi
exit $FAILED
