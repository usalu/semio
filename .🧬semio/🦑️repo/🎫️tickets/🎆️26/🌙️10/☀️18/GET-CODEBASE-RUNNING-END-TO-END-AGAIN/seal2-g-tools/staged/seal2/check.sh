#!/bin/sh
# usage: sh check.sh [--root DIR]  (anchored edits + new files; see apply.py)
exec python3 -I "$(dirname "$0")/apply.py" check "$@"
