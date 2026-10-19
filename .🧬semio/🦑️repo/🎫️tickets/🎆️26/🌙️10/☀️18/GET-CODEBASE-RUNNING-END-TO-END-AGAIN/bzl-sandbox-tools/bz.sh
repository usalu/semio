#!/bin/bash
# usage: bz.sh <wsdir> <outputbase> [bazel args...] ; env EXTRA_STARTUP
W="$1"; OB="$2"; shift 2
BZ=/Users/ueli/Documents/semio/.cache/b/t/m/bazel/bin/45e9388abf21d110/bazel
cd "$W"
export LANG=en_US.UTF-8 LC_ALL=en_US.UTF-8
exec "$BZ" --output_base="$OB" --nosystem_rc --nohome_rc --bazelrc="$W/.bazelrc" --max_idle_secs=60 $EXTRA_STARTUP "$@"
