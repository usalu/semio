#!/bin/sh
# usage: sh verify.sh <root>  - laws of batch stale on that root (a mirror with the batch applied); compare FAILURE SETS with the same run on the base.
exec python3 -I "$(dirname "$0")/../../verify.py" stale "$1"
