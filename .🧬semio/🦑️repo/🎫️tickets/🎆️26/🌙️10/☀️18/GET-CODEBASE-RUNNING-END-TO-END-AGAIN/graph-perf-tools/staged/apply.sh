#!/bin/sh
# usage: apply.sh check | diff | apply [--dry-run] [--ignore-running]   (refuses on drift and while a graph/dependency command runs; idempotent)
exec python3 "$(dirname "$0")/apply.py" "$@"
