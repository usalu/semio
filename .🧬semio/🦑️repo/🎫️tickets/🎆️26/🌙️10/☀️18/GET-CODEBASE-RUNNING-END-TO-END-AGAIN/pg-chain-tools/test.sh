#!/bin/bash
# usage: test.sh [real-bazel] <relative test path in the overlay> ...   runs explicit-path bun tests inside the overlay copy under a minted caller envelope
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
OV=/private/tmp/pgc-ov
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"
:
ART="$(cd "$C/🗑️generated" && pwd)/pg-chain-artifacts"; mkdir -p "$ART"; export SEMIO_TEST_ARTIFACT_DIR="$ART"
if [ "${1:-}" = real-bazel ]; then export SEMIO_TEST_BAZEL_BINARY="/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel"; shift; fi
MINT="$ART/mint.ts"
cat > "$MINT" <<TS
import {createScriptProcessEnvelope,scriptProcessEnvironment} from "$OV/🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {createNativeBuildCallerCapabilitiesV1} from "$OV/$LIB/🏗️build/📥️caller/🟦️.ts";
process.stdout.write(scriptProcessEnvironment(createScriptProcessEnvelope({version:1,owner:"pg-chain-test",maximumElapsedMilliseconds:3000000},createNativeBuildCallerCapabilitiesV1(process.cwd(),["run","fixture:build"],"bazel",{}),Date.now()),{}).SEMIO_SCRIPT_PROCESS_INVOCATION!);
TS
cd "$OV" || exit 2
for file in "$@"; do echo "== $file"; SEMIO_WORKSPACE_ROOT="$OV" SEMIO_SCRIPT_PROCESS_INVOCATION="$(bun "$MINT")" perl -e 'alarm 580; exec @ARGV' -- bun test --timeout 300000 "./$file" 2>&1 | command grep -E '^ *[0-9]+ (pass|fail|skip)|^\(fail\)|^\(pass\)|error:|Expected|Received|expect\(|^[-+] ' | head -${LINES_MAX:-60}; done
