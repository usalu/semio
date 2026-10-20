#!/bin/bash
# usage: run-suites.sh <live|staged> [real-bazel] <suite>...   runs copied adapter suites sequentially under a minted caller envelope (explicit file paths only)
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"; ROOT=/Users/ueli/Documents/semio
MODE=$1; shift
if [ "${1:-}" = real-bazel ]; then export SEMIO_TEST_BAZEL_BINARY="$ROOT/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"; shift; fi
ART=/private/tmp/bzlazy/artifacts; mkdir -p "$ART"; export SEMIO_TEST_ARTIFACT_DIR="$ART"
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
MINT="$ART/mint.ts"
cat > "$MINT" <<TS
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "$ROOT/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "$ROOT/$LIB/🏗️build/📥️caller/🟦️.ts";
process.stdout.write(scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"bzl-lazy-test",maximumElapsedMilliseconds:3000000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","fixture:build"],"bazel",{}),Date.now()),{}).SEMIO_SCRIPT_PROCESS_INVOCATION!);
TS
cd "$ROOT" || exit 2
for suite in "$@"; do
  file="/private/tmp/bzlazy/suites-$MODE/bz/🧪️tests/$suite/🟦️.ts"
  echo "== $MODE $suite"
  SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$MINT")" perl -e 'alarm 580; exec @ARGV' -- bun test --timeout 300000 "$file" 2>&1 | command grep -E '^ *[0-9]+ (pass|fail|skip)|^\(fail\)|error:|Expected|Received|Cannot find|ENOENT' | sed 's/ \[[0-9.]*ms\]//' | head -${LINES_MAX:-25}
done
