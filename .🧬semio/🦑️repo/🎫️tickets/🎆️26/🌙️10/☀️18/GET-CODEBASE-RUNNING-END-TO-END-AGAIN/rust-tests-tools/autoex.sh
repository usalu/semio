#!/bin/sh
# usage: autoex.sh <log-name> <first> <last>: batch.sh, and on a "Rust crate Source refused ... : <crate>" gate refusal exclude that crate and rerun (max 12), recording the excluded crates in <log>.excluded
cd /Users/ueli/Documents/semio || exit 2
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
L="$C/🗑️generated/rust-tests-logs"
name=$1; first=$2; last=$3; ex="$EXCLUDE"
for round in 1 2 3 4 5 6 7 8 9 10 11 12; do
  EXCLUDE="$ex" sh "$C/rust-tests-tools/batch.sh" "$name" "$first" "$last"
  bad=$(/usr/bin/grep -o 'Rust crate Source refused before compiler: [a-z0-9-]*' "$L/$name.log" | head -1 | sed 's/.*: //')
  [ -z "$bad" ] && break
  ex="$ex $bad"; echo "$bad" >> "$L/$name.excluded"; mv "$L/$name.log" "$L/prev/$name.src$round.log"
done
