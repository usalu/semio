#!/bin/sh
# usage: run-tests.sh real|<mirror-dir> <path relative to the library> ...   runs explicit test files under a minted caller envelope; a mirror is realized on demand
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../../../../../../.." && pwd)"
MODE="$1"; shift
if [ "$MODE" = real ]; then BASE="$ROOT"; else BASE="$(cd "$MODE" && pwd)"; fi
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
ART="${SEMIO_TEST_ARTIFACT_DIR:-$HERE/../🗑️generated/inputs-closure-artifacts}"; mkdir -p "$ART"; ART="$(cd "$ART" && pwd)"; export SEMIO_TEST_ARTIFACT_DIR="$ART"
MINT="$ART/mint.ts"
cat > "$MINT" <<TS
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "$BASE/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "$BASE/$LIB/🏗️build/📥️caller/🟦️.ts";
process.stdout.write(scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"inputs-closure-test",maximumElapsedMilliseconds:3000000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","fixture:build"],"nx",{}),Date.now()),{}).SEMIO_SCRIPT_PROCESS_INVOCATION!);
TS
cd "$BASE" || exit 2
for file in "$@"; do
  echo "== $file"
  if [ "$MODE" = real ]; then
    SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$MINT")" perl -e 'alarm 1700; exec @ARGV' -- bun test --timeout 300000 "./$LIB/$file" ${EXTRA:-} 2>&1 | command grep -E '^ *[0-9]+ (pass|fail|skip)|^\(fail\)|error:' | sed 's/ \[[0-9.]*ms\]//' | sort | uniq -c | sort -rn | head -${LINES_MAX:-30}
  else
    SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$MINT")" perl -e 'alarm 1700; exec @ARGV' -- python3 "$HERE/realize.py" "$BASE" -- bun test --timeout 300000 "./$LIB/$file" ${EXTRA:-} 2>&1 | command grep -E '^ *[0-9]+ (pass|fail|skip)|^\(fail\)|error:|realize' | sed 's/ \[[0-9.]*ms\]//' | sort | uniq -c | sort -rn | head -${LINES_MAX:-30}
  fi
done
