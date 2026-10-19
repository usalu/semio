#!/bin/zsh
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
A=$PWD/.cache/tst6/t$$
mkdir -p $A
export SEMIO_TEST_ARTIFACT_DIR=$A NO_COLOR=1 SEMIO_REPO_ROOT=$PWD REPO_ROOT=$PWD SEMIO_WORKSPACE_ROOT=$PWD SEMIO_TICKET_DIR=$C SEMIO_BUILD_MODE=prototype SEMIO_BUILD_TOOL=nx
unset NODE_PATH
eval "$(SEMIO_REPO_ROOT=$PWD bun $C/tst6-tools/envwire.ts | python3 -c 'import json,sys,shlex;d=json.load(sys.stdin);print("export SEMIO_SCRIPT_PROCESS_INVOCATION=%s SEMIO_SCRIPT_PROVIDERS=%s"%(shlex.quote(d["invocation"]),shlex.quote(d["providers"])))')"
perl -e 'alarm 280; exec @ARGV' -- bun test --timeout 120000 "$@" 2>&1
rm -rf $A
