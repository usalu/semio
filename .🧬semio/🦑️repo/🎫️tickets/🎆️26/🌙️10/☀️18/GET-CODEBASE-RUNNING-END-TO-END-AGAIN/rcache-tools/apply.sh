#!/bin/bash
# usage: apply.sh [--dry-run] [--allow-drift]   (see apply.py; run only after the coordinator's "PAUSE LIFTED")
cd /Users/ueli/Documents/semio || exit 2
exec python3 "$(dirname "$0")/apply.py" "$@"
