#!/bin/sh
# 🏖️ Laws of batch `scratch` after apply: sh verify.sh <repository or mirror root> [artifact parent inside that checkout]
# 1. stage check; parse check (Bun transpile + node --check) and tsc --strict over the 8 staged files;
# 2. the new law `🧪️tests/🏖️scratch/🧪️tests` (Cargo and rustup are the oracle);
# 3. every unit-cache test file that starts Cargo, twice: bare and with SEMIO_TEST_ARTIFACT_DIR INSIDE the checkout (the gate's mode: 14 failures on base, 0 with the batch).
set -e
here=$(cd "$(dirname "$0")" && pwd)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
root=${1:?root required}; art=${2:-$root/.tmp/td-ucache-verify}
U="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache"
python3 -I "$here/stage.py" check --root "$root"
list=$(mktemp); python3 -I -c 'import json,sys;[print(sys.argv[2]+"/"+r) for r in json.load(open(sys.argv[1])) if r.endswith(".ts")]' "$here/manifest.json" "$root" > "$list"
bun "$C/gate-g-tools/scan.ts" "$list" "$(mktemp -d)"
cd "$root"
bun "$root/node_modules/typescript/bin/tsc" --noEmit --strict --skipLibCheck --allowImportingTsExtensions --resolveJsonModule --esModuleInterop --target ESNext --module ESNext --moduleResolution bundler --types bun $(tr '\n' ' ' < "$list") | /usr/bin/grep -E "🗃️unit-cache/(🧪️tests/(🟦️|🌱️|🏖️)|🎰️jobserver/🧪️tests/(🟦️|🚦️)|🧵️lane/📤️retention/🧪️tests/🧾️)" && { echo "tsc: errors in staged files"; exit 1; } || echo "tsc: staged files clean"
rm -f "$list"
fail=0
for f in "🧪️tests/🏖️scratch/🧪️tests/🟦️.ts" "🧪️tests/🟦️.ts" "🧪️tests/🌱️incremental/🟦️.ts" "🎰️jobserver/🧪️tests/🟦️.ts" "🎰️jobserver/🧪️tests/🚦️pool/🟦️.ts" "🧵️lane/📤️retention/🧪️tests/🧾️provenance-equality/🟦️.ts"; do
 for mode in bare inside; do
  if [ "$mode" = inside ]; then mkdir -p "$art"; out=$(SEMIO_TEST_ARTIFACT_DIR="$art" bun test "./$U/$f" 2>&1) || true; else out=$(env -u SEMIO_TEST_ARTIFACT_DIR bun test "./$U/$f" 2>&1) || true; fi
  summary=$(printf '%s\n' "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail|skip)$' | tr -s ' \n' ' ')
  printf '%-6s %s :: %s\n' "$mode" "$f" "$summary"
  printf '%s\n' "$out" | /usr/bin/grep -E '^\(fail\)' | cut -c1-200 | sed 's/^/       /'
  printf '%s\n' "$summary" | /usr/bin/grep -Eq ' 0 fail' || fail=1
 done
done
[ "$fail" = 0 ] && echo "verify: GREEN" || { echo "verify: RED"; exit 1; }
