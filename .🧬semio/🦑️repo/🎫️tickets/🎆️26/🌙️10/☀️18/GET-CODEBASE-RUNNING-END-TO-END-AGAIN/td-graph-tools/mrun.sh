#!/bin/sh
# 🪞 [td-graph] runs a command inside mirror <name> sealed, with the lane's generated folder writable; usage: mrun.sh <name> <cmd...>
C="$(cd "$(dirname "$0")/.." && pwd)"; n="$1"; shift
exec python3 -I "$C/mirror-tools/mirror.py" run "$n" --peers-ok --seal-allow ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/🗑️generated/td-graph" -- "$@"
