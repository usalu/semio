#!/bin/sh
# [graphlat-g] batch plan-once2 (REPLACES plan-once, rev4-g MID 2 fixed): sh apply.sh apply|revert|status. apply first reverts plan-once when it is applied (its law file is replaced by the extended one), revert leaves the tree without either.
cd /Users/ueli/Documents/semio || exit 1
HERE='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/graphlat-g-tools/staged'
case "${1:-status}" in
 apply) python3 "$HERE/batch_generic.py" "$HERE/plan-once" revert && python3 "$HERE/batch_generic.py" "$HERE/plan-once2" apply ;;
 *) exec python3 "$HERE/batch_generic.py" "$HERE/plan-once2" "${1:-status}" ;;
esac
