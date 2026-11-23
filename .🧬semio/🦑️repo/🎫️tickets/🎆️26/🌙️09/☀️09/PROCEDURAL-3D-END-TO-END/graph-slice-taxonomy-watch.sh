#!/bin/sh
# Logs every change of the taxonomy projection input (mtime and size of the taxonomy file) while a read runs.
R="$(git rev-parse --show-toplevel)"; F="$R/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"; L="$(dirname "$0")/🗑️generated/taxonomy-watch.txt"; last=""
while [ "$(date +%s)" -lt "$1" ]; do cur="$(stat -f '%m %z' "$F")"; [ "$cur" != "$last" ] && echo "$(date +%T) $cur" >> "$L"; last="$cur"; sleep 5; done
