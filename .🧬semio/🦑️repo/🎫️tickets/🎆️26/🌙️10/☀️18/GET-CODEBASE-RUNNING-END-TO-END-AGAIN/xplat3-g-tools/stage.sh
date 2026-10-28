#!/bin/sh
# usage: stage.sh <repo-relative path>... -> copies the LIVE file into staged/files (once) and records its sha256 as the base in bases.tsv; a path that does not exist live is recorded as NEW
cd /Users/ueli/Documents/semio || exit 1
T=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/xplat3-g-tools"
for p in "$@"; do
  d="$T/staged/files/$p"
  if [ -e "$d" ]; then echo "already staged: $p"; continue; fi
  mkdir -p "$(dirname "$d")"
  if [ -e "$p" ]; then cp "$p" "$d"; printf '%s\t%s\n' "$(shasum -a 256 "$p" | cut -d' ' -f1)" "$p" >> "$T/bases.tsv"; else : > "$d"; printf 'NEW\t%s\n' "$p" >> "$T/bases.tsv"; fi
  echo "staged: $p"
done
