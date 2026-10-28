#!/bin/sh
# 🪟️ Laws of batch `winlaunch` after apply: sh verify.sh <MIRROR root>   (never the live checkout: the negative control swaps base files in and out). Needs go, bun, python3; nothing here runs on Windows, the Windows-only Go case compiles for windows/amd64 and windows/arm64 and runs on the first Windows host.
ROOT=${1:?mirror root}; HERE=$(cd "$(dirname "$0")" && pwd); C=$(cd "$HERE/../../.." && pwd); XPLAT5="$C/xplat5-tools"; . "$XPLAT5/verify-common.sh"
cd "$ROOT" || exit 2
[ -e "$ROOT/.git" ] && { echo "refusing the live checkout"; exit 2; }
python3 -I "$HERE/stage.py" check --root "$ROOT" >/dev/null || { echo "batch not applied in $ROOT"; exit 2; }
WORK="$C/🗑️generated/windows/verify-winlaunch"
gotests() { out=$(cd "$WORK" && SEMIO_WORKSPACE_ROOT="$ROOT" GOWORK=off GOTOOLCHAIN=local CGO_ENABLED=0 go test -count=1 -v . 2>&1); code=$?; printf '%s\n' "$out" | grep -E '^--- FAIL'; echo "$(printf '%s\n' "$out" | grep -c '^--- PASS') pass"; return $code; }
cross() { out=$(bash "$C/windows-tools/go-win.sh" "$ROOT" "$WORK" 2>&1); printf '%s\n' "$out" | grep -vE '^==|^-rw' | head -20; ! printf '%s\n' "$out" | grep -vE '^==|^-rw' | grep -q .; }
law "launcher vet + build + test binary for windows/amd64, windows/arm64, linux/amd64, darwin/arm64" cross
law "launcher Go tests on this host (job, pipe and final-path laws compile for Windows only)" gotests
law "service session: Windows worktree root is the kernel's final path" bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service/🧪️tests/🟦️.ts"
SESSION="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service/🔑️session/🟨️.mjs"; FILE=$(python3 -I -c "import json,sys;print(json.load(open(sys.argv[1],encoding='utf8'))[sys.argv[2]]['file'])" "$HERE/manifest.json" "$SESSION")
cp "$HERE/base/$FILE" "$ROOT/$SESSION"; out=$(bun test "./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️caller/🛰️service/🧪️tests/🟦️.ts" -t "final path" 2>&1); code=$?; cp "$HERE/files/$FILE" "$ROOT/$SESSION"
if [ $code -eq 0 ]; then echo "RED   negative control DID NOT BITE with the base session module"; FAILED=1; else echo "GREEN negative control: the final-path law is red with the base session module"; fi
echo "negative control of the Go laws: the base sources have no renameRetrying / FinalPathSpelling / containChild, so the new cases do not compile there; the Windows job law is red by behaviour on base (run it there with the base windows-host)"
rm -rf "$WORK"
exit $FAILED
