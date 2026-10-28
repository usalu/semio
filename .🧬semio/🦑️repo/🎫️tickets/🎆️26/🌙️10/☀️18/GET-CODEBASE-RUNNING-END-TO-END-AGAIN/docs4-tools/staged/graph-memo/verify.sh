#!/bin/sh
# usage: sh verify.sh <mirror root>   (laws of the whole docs4 set on a MIRROR; never the live tree)
exec python3 -I "$(dirname "$0")/../../verify.py" "$1"
