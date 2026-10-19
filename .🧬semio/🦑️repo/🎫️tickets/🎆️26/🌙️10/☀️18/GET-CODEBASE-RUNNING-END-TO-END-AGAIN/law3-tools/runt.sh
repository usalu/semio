#!/bin/zsh
# usage: runt.sh file pattern [width]  -> full bun output (url-decoded), trimmed. Env: LAW3_MAX_MS, LAW3_CAPS, LAW3_ART_DIR (absolute), LAW3_NO_MODE=1 drops SEMIO_BUILD_MODE
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
SP=/private/tmp/claude-501/-Users-ueli-Documents-semio/214979e9-1d0c-472f-865a-73fad9fbbf95/scratchpad
WIRE=$(LAW3_MAX_MS=${LAW3_MAX_MS:-3600000} bun $C/law3-tools/envwire.ts)
export SEMIO_TEST_ARTIFACT_DIR=${LAW3_ART_DIR:-$SP/ta} SEMIO_REPO_ROOT=$PWD SEMIO_SCRIPT_PROCESS_INVOCATION="$WIRE" SEMIO_TICKET_DIR=$C NO_COLOR=1
if [ -n "$LAW3_NO_MODE" ]; then unset SEMIO_BUILD_MODE; else export SEMIO_BUILD_MODE=prototype; fi
bun test --tsconfig-override $SP/hoist/tsconfig.json --timeout 120000 "./$1" -t "$2" 2>&1 | python3 -c "
import sys,urllib.parse
t=urllib.parse.unquote(sys.stdin.read())
for l in t.split('\n'):
    if l.startswith('Internal error: directory'): continue
    print(l[:${3:-600}])
"
