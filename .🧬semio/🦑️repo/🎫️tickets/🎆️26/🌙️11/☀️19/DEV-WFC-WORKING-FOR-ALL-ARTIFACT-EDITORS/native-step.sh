#!/bin/bash
export SEMIO_OFFLOAD=never
# usage: native-step.sh <logname> <cwd relative to repo> <script relative to cwd> [args...]  (env: SEMIO_BUILD_MODE etc. passed through)
# Mints the Script process envelope the graph executor would issue, so repo scripts run without the (blocked) graph load.
ROOT="$HOME/Documents/semio"; T="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️19/DEV-WFC-WORKING-FOR-ALL-ARTIFACT-EDITORS"
log="$1"; cwd="$2"; shift 2
export SEMIO_SCRIPT_PROCESS_INVOCATION="$(cat "$T/🗑️generated/envelope.json")"
export SEMIO_SCRIPT_PROVIDERS="$(cat "$T/🗑️generated/providers.json")"
cd "$ROOT/$cwd" && repo "$@" > "$T/🗑️generated/$log.log" 2>&1; echo "rc=$?" >> "$T/🗑️generated/$log.log"
