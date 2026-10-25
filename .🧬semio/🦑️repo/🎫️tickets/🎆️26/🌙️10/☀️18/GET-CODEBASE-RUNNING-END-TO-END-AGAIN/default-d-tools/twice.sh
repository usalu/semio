#!/bin/sh
# twice.sh <name> <repo args...>: two back-to-back runs <name>a, <name>b (cache-hit proof)
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
n=$1; shift
python3 "$C/default-d-tools/drive.py" "${n}a" "$@"
python3 "$C/default-d-tools/drive.py" "${n}b" "$@"
