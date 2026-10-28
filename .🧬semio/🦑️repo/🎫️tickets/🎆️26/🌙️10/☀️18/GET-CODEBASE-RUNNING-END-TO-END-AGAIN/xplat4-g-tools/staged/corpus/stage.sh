#!/bin/sh
# usage: stage.sh <repo-relative path>... -> copies the base of each file into files/ (once) and records the base digest in bases.tsv. The base is the xplat3-g STAGED file when xplat3-g staged that path (this batch builds on top of it: `apply.py check` reports DRIFT until xplat3-g landed), else the LIVE file; a path that does not exist is NEW.
cd /Users/ueli/Documents/semio || exit 1
H="$(cd "$(dirname "$0")" && pwd)"
X="$H/../../../xplat3-g-tools/staged/files"
for p in "$@"; do
  d="$H/files/$p"
  if [ -e "$d" ]; then echo "already staged: $p"; continue; fi
  mkdir -p "$(dirname "$d")"
  if [ -e "$X/$p" ]; then src="$X/$p"; elif [ -e "$p" ]; then src="$p"; else src=""; fi
  if [ -n "$src" ]; then cp "$src" "$d"; printf '%s\t%s\n' "$(shasum -a 256 "$src" | cut -d' ' -f1)" "$p" >> "$H/bases.tsv"; echo "staged: $p (base: $src)" | cut -c1-200; else : > "$d"; printf 'NEW\t%s\n' "$p" >> "$H/bases.tsv"; echo "staged NEW: $p"; fi
done
