#!/bin/zsh
# usage: smoke.sh <script> [args...]   runs one lifted script under the minted launcher envelope; proves argv reaches the program
ROOT=/Users/ueli/Documents/semio
TICKET=$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN
cd $ROOT
export REPO_ROOT=$ROOT SEMIO_WORKSPACE_ROOT=$ROOT SEMIO_REPO_ROOT=$ROOT SEMIO_TICKET_DIR=$TICKET NO_COLOR=1 SEMIO_BUILD_MODE=prototype
eval "$(bun $TICKET/lib-defects-tools/envwire.ts $ROOT | python3 -c 'import json,sys,shlex;[print("export %s=%s"%(k,shlex.quote(v))) for k,v in json.load(sys.stdin).items()]')"
S=$1; shift
perl -e 'alarm 60; exec @ARGV' -- bun "$S" "$@"
