#!/bin/sh
# usage: stage.sh <repo-relative path>... -> copies the LIVE file into files/ (once) and records its sha256 as the base in bases.tsv; a path that does not exist live is recorded as NEW
cd /Users/ueli/Documents/semio || exit 1
H="$(cd "$(dirname "$0")" && pwd)"
for p in "$@"; do
  d="$H/files/$p"
  if [ -e "$d" ]; then echo "already staged: $p"; continue; fi
  mkdir -p "$(dirname "$d")"
  if [ -e "$p" ]; then cp "$p" "$d"; printf '%s\t%s\n' "$(shasum -a 256 "$p" | cut -d' ' -f1)" "$p" >> "$H/bases.tsv"; else : > "$d"; printf 'NEW\t%s\n' "$p" >> "$H/bases.tsv"; fi
  echo "staged: $p"
done
