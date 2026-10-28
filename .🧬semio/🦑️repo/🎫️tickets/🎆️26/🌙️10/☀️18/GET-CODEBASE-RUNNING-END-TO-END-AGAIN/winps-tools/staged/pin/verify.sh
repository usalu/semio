#!/bin/sh
# 🪟️ Laws of batch `pin` after apply: sh verify.sh <MIRROR root>   (never the live checkout). Needs go. The Windows-only pin law compiles for windows/amd64 and windows/arm64 and runs on the first Windows host; negative control: with the base provisioning source the order and adoption laws are red by behaviour.
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../../.." && pwd); XPLAT5="$C/xplat5-tools"; . "$XPLAT5/verify-common.sh"
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" >/dev/null || { echo "batch not applied in $ROOT"; exit 2; }
WORK="$C/🗑️generated/winps/verify-pin"
gotests() { out=$(cd "$WORK" && SEMIO_WORKSPACE_ROOT="$ROOT" GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 go test -count=1 -v . 2>&1); code=$?; printf '%s\n' "$out" | grep -E '^--- FAIL'; echo "$(printf '%s\n' "$out" | grep -c '^--- PASS') pass"; return $code; }
cross() { out=$(bash "$C/windows-tools/go-win.sh" "$ROOT" "$WORK" 2>&1); printf '%s\n' "$out" | grep -vE '^==|^-rw' | head -20; ! printf '%s\n' "$out" | grep -vE '^==|^-rw' | grep -q .; }
law "launcher vet + build + test binary for windows/amd64, windows/arm64, linux/amd64, darwin/arm64" cross
law "launcher Go tests on this host (order, adoption, race, refused adoption; the pin law is Windows-only)" gotests
(cd "$WORK" && SEMIO_WORKSPACE_ROOT="$ROOT" GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 go test -count=1 -race -run 'TestWindows' . 2>&1 | tail -3 | sed 's/^/      race: /')
PROV="🧰️framework/🛍️products/🦑️repo/🔨️modules/🚀️launcher/📦️packages/🐹️go/📥️runtime-provision/🐹️.go"; FILE=$(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1],encoding='utf8'))[sys.argv[2]]['file'])" "$HERE/manifest.json" "$PROV")
cp "$HERE/base/$FILE" "$ROOT/$PROV"; bash "$C/windows-tools/go-win.sh" "$ROOT" "$WORK" >/dev/null 2>&1; out=$(gotests); code=$?; cp "$HERE/files/$FILE" "$ROOT/$PROV"
if [ $code -eq 0 ]; then echo "RED   negative control DID NOT BITE with the base provisioning source"; FAILED=1; else echo "GREEN negative control: red with the base provisioning source ($(printf '%s\n' "$out" | grep -c '^--- FAIL') failing cases)"; fi
rm -rf "$WORK"
exit $FAILED
