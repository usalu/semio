#!/bin/sh
# usage: run-tests.sh mirror <mirror-dir> [real-bazel] | real [real-bazel]   -- runs the explicit-path Bazel suites sequentially under a minted caller envelope
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
MODE="${1:-real}"
if [ "$MODE" = mirror ]; then BASE="$(cd "$2" && pwd)"; shift; else BASE="$(cd "$HERE/../../../../../../../.." && pwd)"; fi
shift
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
BZ="$LIB/🏗️build/🔧️tools/🏗️bazel"
ART="${SEMIO_TEST_ARTIFACT_DIR:-$HERE/../🗑️generated/bzl-stage-artifacts}"; mkdir -p "$ART"; ART="$(cd "$ART" && pwd)"; export SEMIO_TEST_ARTIFACT_DIR="$ART"
if [ "${1:-}" = real-bazel ]; then export SEMIO_TEST_BAZEL_BINARY="$(cd "$HERE/../../../../../../../.." && pwd)/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"; shift; fi
MINT="$ART/mint.ts"
cat > "$MINT" <<TS
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "$BASE/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "$BASE/$LIB/🏗️build/📥️caller/🟦️.ts";
process.stdout.write(scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"bzl-stage-test",maximumElapsedMilliseconds:3000000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","fixture:build"],"bazel",{}),Date.now()),{}).SEMIO_SCRIPT_PROCESS_INVOCATION!);
TS
cd "$BASE" || exit 2
for file in "$@"; do
  echo "== $file"
  SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$MINT")" perl -e 'alarm 580; exec @ARGV' -- bun test --timeout 300000 "$BASE/$BZ/$file" 2>&1 | command grep -E '^ *[0-9]+ (pass|fail|skip)|^\(fail\)|^\(pass\)|error:|Expected|Received|expect\(' | sed 's/ \[[0-9.]*ms\]//' | head -${LINES_MAX:-40}
done
