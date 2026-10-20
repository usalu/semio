#!/bin/bash
# usage: apply.sh [--dry-run] [--rebase] [--root R]   applies the staged bzl-coverage set (edits.py + staged/) to the checkout; refuses on drift of any edited target (bases.json) and on an anchor that does not occur exactly once
exec python3 -I "$(cd "$(dirname "$0")" && pwd)/apply.py" apply "$@"
