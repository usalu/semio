#!/bin/sh
# usage: chainrun.sh <mirror> <repo-relative test file> [bun test args]  -> runs the file inside the mirror under the library test envelope (repo-lib:test capabilities) with a short realpath artifact dir; prints summary + failing names + first errors
name="$1"; f="$2"; shift 2
R="$(cd -P "${TMPDIR%/}/sm/$name" && pwd -P)"
ART="/private/tmp/claude-501/tca/$name-$$"; rm -rf "$ART"; mkdir -p "$ART"
T="$R/.tc-run"; mkdir -p "$T"
C="$(cd "$(dirname "$0")/.." && pwd)"
sed "s#\"\.\./\.\./\.\./\.\./\.\./\.\./\.\./\.\./🧰️#\"$R/🧰️#g" "$C/chain-g-tools/wire.ts" > "$T/wire.ts"
cd "$R" || exit 1
wire=$(bun "$T/wire.ts" 2>/dev/null)
out=$(SEMIO_TEST_ARTIFACT_DIR="$ART" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" nice -n 10 bun test --timeout 170000 "./$f" "$@" 2>&1)
printf '%s\n' "$out" > "$ART/out.txt"
printf '%s chain: %s\n' "$name" "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' \n' ' ')"
echo "$out" | /usr/bin/grep -E '^\(fail\)' | sort -u | cut -c1-240
echo "$out" | /usr/bin/grep -E '^error:' | sort -u | head -4 | cut -c1-300
cp "$ART/out.txt" "$C/🗑️generated/tc/chain-$(echo "$f" | md5 -q | cut -c1-8).txt"; rm -rf "$ART"
