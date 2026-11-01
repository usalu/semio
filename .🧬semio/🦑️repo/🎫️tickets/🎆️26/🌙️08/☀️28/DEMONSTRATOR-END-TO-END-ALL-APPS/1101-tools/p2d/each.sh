#!/bin/bash
# usage: MAN=<manifest> each.sh <tag> <crate>...  -- one cargo test per crate so a compile fault does not mask the others
D=$(dirname "$0"); TAG=$1; shift
for c in "$@"; do "$D/t.sh" "$TAG-${c#semio-framework-}" -p "$c"; done
echo done > "$D/../../🗑️generated/1101/p2d-$TAG.done"
