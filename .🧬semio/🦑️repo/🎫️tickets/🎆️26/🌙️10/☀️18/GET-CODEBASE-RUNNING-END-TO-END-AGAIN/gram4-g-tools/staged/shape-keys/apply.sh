#!/bin/sh
# [gram4-g] usage: apply.sh check|apply|revert|status  (BATCH_ROOT=<mirror root> for a mirror). Anchored edit + law dir, base hash in bases.json.
H="$(cd "$(dirname "$0")" && pwd)"
exec python3 -I "$H/../../stage.py" "$H" "$1"
