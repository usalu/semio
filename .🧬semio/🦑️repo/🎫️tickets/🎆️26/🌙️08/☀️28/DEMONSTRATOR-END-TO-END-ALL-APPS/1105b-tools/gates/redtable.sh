#!/bin/bash
# usage: redtable.sh <log> — per-crate error counts and first error line
L="$1"
/usr/bin/grep -E "^error: could not compile|^error\[|^error: " "$L" | /usr/bin/grep "could not compile" | sed -E 's/error: could not compile `([^`]*)` \(([a-z ]*)\) due to ([0-9]+) previous error.*/\1 \3/' | sort -u
