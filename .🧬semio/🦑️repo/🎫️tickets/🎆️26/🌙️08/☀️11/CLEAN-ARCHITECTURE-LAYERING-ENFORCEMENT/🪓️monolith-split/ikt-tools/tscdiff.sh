#!/bin/zsh
# usage: tscdiff.sh <log>  -> normalized error lines (no line numbers), sorted
. "$(dirname "$0")/env.sh"
grep 'error TS' "$1" | sed -E 's#\([0-9]+,[0-9]+\)##' | sed -E "s#^$R/##" | sort
