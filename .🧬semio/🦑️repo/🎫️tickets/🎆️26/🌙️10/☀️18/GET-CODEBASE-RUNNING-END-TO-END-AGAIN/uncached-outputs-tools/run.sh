#!/bin/bash
# usage: run.sh [--write]   (decide -> apply [dry run unless --write] -> validate -> report; outputs under C/🗑️generated/uncached-outputs)
set -e
ROOT=/Users/ueli/Documents/semio
C="$ROOT/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd "$C/uncached-outputs-tools"
bun decide.ts
bun apply.ts "$@"
[ "$1" = "--write" ] && bun validate.ts && bun report.ts
exit 0
