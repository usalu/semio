#!/bin/sh
# 🔗 Runs one test file inside a mirror under the runner envelope like red-g-tools/mrun.sh chain, but with the scratch TMPDIR inside the mirror root (as an official Bazel test sees a TMPDIR below the checkout): `sh mchain.sh <mirror> <repo-relative test file>`
name="$1"; f="$2"; C=$(cd "$(dirname "$0")/.." && pwd)
R="${TMPDIR%/}/sm/$name"; [ -d "$R" ] || { echo "no mirror $R"; exit 2; }
T="$R/.red-g-run"; mkdir -p "$T/art" "$R/.rn/tmp"
sed "s#/Users/ueli/Documents/semio#$R#g; s#\"\.\./\.\./\.\./\.\./\.\./\.\./\.\./\.\./🧰️#\"$R/🧰️#g" "$C/chain-g-tools/wire.ts" > "$T/wire.ts"
cd "$R" || exit 2
wire=$(bun "$T/wire.ts" 2>/dev/null)
out=$(TMPDIR="$R/.rn/tmp" SEMIO_TEST_ARTIFACT_DIR="$T/art" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" bun test "./$f" 2>&1)
printf '%s chain(tmp in mirror): %s\n' "$name" "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' \n' ' ')"
echo "$out" | /usr/bin/grep -E '^\(fail\)' | cut -c1-200
