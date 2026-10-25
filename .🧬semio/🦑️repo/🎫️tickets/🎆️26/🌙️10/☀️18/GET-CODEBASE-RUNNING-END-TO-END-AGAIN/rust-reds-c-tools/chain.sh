#!/bin/sh
# usage: chain.sh <tag> <manifest> <crate>... ; runs `cargo test -p <crate> --no-fail-fast -- --nocapture` for each crate one after another, logs <tag>-<crate>.log
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; L="$C/🗑️generated/rust-reds-c-logs"
tag=$1; ws=$2; shift 2
for crate in "$@"; do
  rm -f "$L/$tag-$crate.log.status"
  sh "$C/rust-reds-c-tools/run.sh" "$ws" "$L/$tag-$crate.log" -p "$crate" -- --nocapture
done
echo done > "$L/$tag.chain.status"
