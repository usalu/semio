#!/bin/sh
# Ticket input script: the final verification of the course end to end on the final code: the full dev topology, then the full rehearsal topology (each through retry.sh).
#   final.sh <run id prefix>
here=$(cd "$(dirname "$0")" && pwd)
prefix=$1
sh "$here/retry.sh" "${prefix}d" dev; dev=$?
echo "[final] dev rc=$dev"
sh "$here/retry.sh" "${prefix}r" rehearsal; rehearsal=$?
echo "[final] rehearsal rc=$rehearsal"
[ "$dev" = 0 ] && [ "$rehearsal" = 0 ]
