#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S=/private/tmp/claude-501/-Users-ueli-Documents-semio/f251ca6a-3613-4a20-b3b7-6e7765237e44/scratchpad
export REPO_ROOT=$PWD SEMIO_TEST_ARTIFACT_DIR=$S/ta NO_COLOR=1 SEMIO_REPO_ROOT=$PWD SEMIO_TICKET_DIR=$C SEMIO_BUILD_MODE=prototype
eval "$(bun $C/tst3b-tools/envwire.ts | python3 -c 'import json,sys,shlex;[print("export %s=%s"%(k,shlex.quote(v))) for k,v in json.load(sys.stdin).items()]')"
mkdir -p $S/ta
perl -e 'alarm 280; exec @ARGV' -- bun test --tsconfig-override $S/np2/tsconfig.json --timeout 120000 "$@" 2>&1
