#!/bin/bash
# usage: apply.sh [--dry-run] [--allow-drift]   (see apply.py; after the coordinator's GO)
cd /Users/ueli/Documents/semio || exit 2
exec python3 "$(dirname "$0")/apply.py" "$@"
