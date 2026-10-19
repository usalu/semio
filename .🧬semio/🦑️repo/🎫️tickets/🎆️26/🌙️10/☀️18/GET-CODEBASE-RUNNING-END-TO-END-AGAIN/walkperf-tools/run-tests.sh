#!/bin/sh
# usage: run-tests.sh mirror <mirror-dir> | real
# Runs the explicit-path suites that cover the walkperf change (never a directory, never ./): sequentially, with the minted caller envelope.
set -u
HERE="$(cd "$(dirname "$0")" && pwd)"
MODE="${1:-real}"
if [ "$MODE" = mirror ]; then BASE="$(cd "$2" && pwd)"; else BASE="$(cd "$HERE/../../../../../../../.." && pwd)"; fi
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
DEPS="$LIB/⚡️caching/🚀️bootstrap/📦️dependencies"
ART="${SEMIO_TEST_ARTIFACT_DIR:-${TMPDIR:-/tmp}/walkperf-artifacts}"; mkdir -p "$ART"; export SEMIO_TEST_ARTIFACT_DIR="$ART"
MINT="$ART/mint.ts"
cat > "$MINT" <<TS
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "$BASE/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "$BASE/$LIB/🏗️build/📥️caller/🟦️.ts";
process.stdout.write(scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"walkperf-test",maximumElapsedMilliseconds:3000000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["show","projects"],"nx",process.env),Date.now()),{}).SEMIO_SCRIPT_PROCESS_INVOCATION!);
TS
cd "$BASE" || exit 2
run() { SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$MINT")" perl -e 'alarm 560; exec @ARGV' -- bun test "$BASE/$1" "${@:2}" 2>&1 | grep -E '^ *[0-9]+ (pass|fail|skip)|^\(fail\)|error:' | sed 's/ \[[0-9.]*ms\]//' | head -${LINES_MAX:-12}; }
echo "== generation receipt (new)"; run "$DEPS/🧾️generation/🧪️tests/🟦️.ts"
echo "== workspaces bun (cjs patch)"; run "$LIB/🗂️workspaces/🟦️bun/🧪️tests/🟦️.ts"
echo "== bootstrap parser AST law (patched ensureBuildEnvironmentV1)"; run "$LIB/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts" -t "Bootstrap publishes missing parser receipts"
echo "== bootstrap parser receipt schema"; run "$LIB/🏗️build/🚀️bootstrap/📦️parsers/🧪️tests/🟦️.ts" -t "neutral bootstrap parser receipt"
echo "== bootstrap phase law"; run "$LIB/🏗️build/📥️inputs/💾️storage/🧪️tests/🎛️phase/🟦️.ts" -t "graph bootstrap authenticates its explicit phase"
echo "== storage source laws (reader/producer text laws)"; run "$DEPS/💾️storage/🧪️tests/👓️reading/🟦️.ts" -t "actual owner separates readonly inspection|all actual installed receipt consumers|neutral capacity bounds|private finite policy"
echo "== storage suite (baseline fails 20 without a full bun repo run)"; LINES_MAX=3 run "$DEPS/💾️storage/🧪️tests/🟦️.ts"
