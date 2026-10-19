#!/bin/bash
# usage: prodbz.sh <scratchdir> [bazel args] ; env EXTRA_STARTUP
S="$1"; shift
BZ=/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel
cd "$S/ws"
export LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8
exec "$BZ" --output_base="$S/ob" --nosystem_rc --nohome_rc --bazelrc="$S/ws/.bazelrc" --max_idle_secs=60 $EXTRA_STARTUP "$@"
