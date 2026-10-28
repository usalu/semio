#!/bin/sh
# usage: ft.sh <path relative to the process module> [bun test args]  (graph-free official envelope)
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
cd /Users/ueli/Documents/semio || exit 1
VF_WALL=${VF_WALL:-420} exec bun "$C/ver-f-tools/proc-run.ts" file "$@"
