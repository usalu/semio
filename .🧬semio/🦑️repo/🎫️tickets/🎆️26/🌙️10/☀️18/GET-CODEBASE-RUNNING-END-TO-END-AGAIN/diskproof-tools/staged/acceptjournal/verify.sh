#!/bin/sh
# usage: sh verify.sh <root> [before]   (laws of the batch acceptjournal on a mirror root; `before` = laws on base, must be RED)
HERE=$(cd "$(dirname "$0")" && pwd)
exec python3 -I "$HERE/../../verify_batch.py" "$HERE" "$@"
