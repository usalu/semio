#!/bin/sh
# 🔗 Runs repo-relative test files under the runner envelope inside ROOT (a mirror, or live after apply): `sh vchain.sh <root> <rel-test-file>...`; prints pass/fail counts per file and exits non-zero on any failure.
R="$1"; shift; C=$(cd "$(dirname "$0")/.." && pwd); T="$R/.hubreds-run"; mkdir -p "$T/art"
sed "s#/Users/ueli/Documents/semio#$R#g; s#\"\.\./\.\./\.\./\.\./\.\./\.\./\.\./\.\./🧰️#\"$R/🧰️#g" "$C/chain-g-tools/wire.ts" > "$T/wire.ts"
cd "$R" || exit 2; status=0
for f in "$@"; do
  wire=$(bun "$T/wire.ts" 2>/dev/null)
  out=$(SEMIO_TEST_ARTIFACT_DIR="$T/art" SEMIO_SCRIPT_PROCESS_INVOCATION="$wire" bun test "./$f" 2>&1)
  fail=$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ fail$' | tr -dc '0-9'); pass=$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ pass$' | tr -dc '0-9')
  printf '%s: pass %s fail %s\n' "$(basename "$(dirname "$f")")" "${pass:-0}" "${fail:-?}"
  echo "$out" | /usr/bin/grep -E '^\(fail\)|^error:' | head -3 | cut -c1-200
  [ "${fail:-1}" = 0 ] || status=1
done
rm -rf "$T"; exit $status
