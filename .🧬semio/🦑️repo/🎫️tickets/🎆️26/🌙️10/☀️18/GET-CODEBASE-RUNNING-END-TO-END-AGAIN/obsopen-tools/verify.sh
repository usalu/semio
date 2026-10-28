#!/bin/sh
# 🔬️ [obsopen] verify.sh <root>: runs the laws of the batch this file sits in INSIDE <root> (a mirror with the batch applied, never live) with the PINNED Bun (package.json packageManager) from the checkout root as the suite runner does, no parent and no artifact directory (the declared library envelope births the files). nestedfix: observation blocks (all laws but the retirement law: 16 pass, that one needs the official loader), presence (35/0), reuse (12/0), suite route law (9/0). lostbirth: settlement (4/0) and boot (4/0) laws. Exit 0 only when every count matches.
R="$(cd "${1:?usage: verify.sh <root>}" && pwd -P)"; HERE="$(cd "$(dirname "$0")" && pwd)"; C="$(cd "$HERE/../../.." && pwd)"; batch="$(basename "$HERE")"
want=$(sed -n 's/.*"packageManager": "bun@\([^"]*\)".*/\1/p' "$R/package.json"); B="${BUN:-}"
[ -n "$B" ] || for b in "$HOME"/Library/Caches/semio/runtimes/bun/*/bun "$HOME"/.cache/semio/runtimes/bun/*/bun; do [ -x "$b" ] && [ "$("$b" --version)" = "$want" ] && B="$b" && break; done
[ -n "$B" ] || { echo "pinned bun $want not found (set BUN)"; exit 2; }
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; O="$L/🏗️build/👁️observation/🧪️tests"; P="🧰️framework/🔨️modules/🏃️process"
G="$C/🗑️generated/obsopen/verify-$batch"; mkdir -p "$G"; rc=0
python3 -I -c 'import json,sys;[print(sys.argv[2]+"/"+k) for k in json.load(open(sys.argv[1]+"/manifest.json")) if k.endswith(".ts")]' "$HERE" "$R" > "$G/scan.txt"
"$B" "$C/gate-g-tools/scan.ts" "$G/scan.txt" "$G/scan" | tail -1 | grep -q "bad 0" || { echo "PARSE FAIL"; rc=1; }
count() { printf '%s: %s (want %s)\n' "$1" "$(grep -E '^ *[0-9]+ (pass|fail)$' "$2" | tr -s ' \n' ' ')" "$3"; [ "$(grep -E '^ *[0-9]+ pass$' "$2" | tr -dc 0-9)" = "$4" ] && [ "$(grep -E '^ *[0-9]+ fail$' "$2" | tr -dc 0-9)" = "$5" ] || rc=1; }
detach() { for d in $(mount | grep "$R/.cache/b/o/m/tmp/gf-" | awk '{print $1}'); do hdiutil detach "$d" >/dev/null 2>&1; done; }
root_test() { (cd "$R" && env -u SEMIO_TEST_ARTIFACT_DIR nice -n 10 "$B" test --timeout 170000 "./$1" > "$2" 2>&1); detach; }
case "$batch" in
 nestedfix)
  root_test "$O/🧱️blocks/🟦️.ts" "$G/blocks.txt"; count "blocks (retirement law needs the official loader)" "$G/blocks.txt" "16/1 + unnamed" 16 2
  grep -q "^(fail) observation block publisher rebuilds" "$G/blocks.txt" && [ "$(grep -c '^(fail) observation block' "$G/blocks.txt")" = 1 ] || { echo "blocks: unexpected failing law set"; rc=1; }
  root_test "$O/🪪️presence/🟦️.ts" "$G/presence.txt"; count presence "$G/presence.txt" "35/0" 35 0
  root_test "$O/📥️reuse/🟦️.ts" "$G/reuse.txt"; count reuse "$G/reuse.txt" "12/0" 12 0
  root_test "$L/🏗️build/🗂️suites/🧪️tests/🟦️.ts" "$G/suites.txt"; count "suite route law" "$G/suites.txt" "9/0" 9 0;;
 lostbirth)
  (cd "$R/$P/🎛️owned-execution/🏁️settlement/🧪️tests" && env -u SEMIO_TEST_ARTIFACT_DIR "$B" test ./🟦️.ts > "$G/settlement.txt" 2>&1); count settlement "$G/settlement.txt" "4/0" 4 0
  mkdir -p "$R/.tmp/oo"; (cd "$R/$P/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🧪️tests" && SEMIO_TEST_ARTIFACT_DIR="$R/.tmp/oo" "$B" test ./🟦️.ts > "$G/boot.txt" 2>&1); count boot "$G/boot.txt" "4/0" 4 0;;
 *) echo "unknown batch $batch"; exit 2;;
esac
echo "verify $batch rc=$rc (logs $G)"; exit $rc
