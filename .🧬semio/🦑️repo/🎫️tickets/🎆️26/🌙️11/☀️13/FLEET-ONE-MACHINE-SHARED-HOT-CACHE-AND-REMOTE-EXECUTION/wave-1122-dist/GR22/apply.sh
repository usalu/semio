#!/bin/zsh
# 🚀️ Lands the staged GR22 files in one burst: every target is first checked against its pre-image sha256 (sha-before.txt; nothing changes on a mismatch), then every staged file is copied beside its target and all are renamed into place, then the superseded paths of deletions.txt are removed; sha-after.txt records the result.
set -e
here=${0:A:h}; lib="/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/"
target_of(){ [[ $1 == /* ]] && print -r -- "$1" || print -r -- "$lib$1"; }
while IFS=$'\t' read -r staged target; do
  dest=$(target_of "$target"); expected=$(grep -F "  $target" "$here/sha-before.txt" | tail -1 | cut -c1-64)
  now=$(shasum -a 256 "$dest" | cut -c1-64)
  if [[ $now != $expected ]]; then print -r -- "pre-image mismatch: $target"; exit 1; fi
done < "$here/list.tsv"
while IFS=$'\t' read -r staged target; do dest=$(target_of "$target"); cp "$here/staged/$staged" "$dest.gr22tmp"; done < "$here/list.tsv"
while IFS=$'\t' read -r staged target; do dest=$(target_of "$target"); mv -f "$dest.gr22tmp" "$dest"; done < "$here/list.tsv"
while read -r gone; do [[ -n $gone ]] && rm -rf "$lib$gone"; done < "$here/deletions.txt"
: > "$here/sha-after.txt"
while IFS=$'\t' read -r staged target; do dest=$(target_of "$target"); printf '%s  %s\n' "$(shasum -a 256 "$dest" | cut -c1-64)" "$target" >> "$here/sha-after.txt"; done < "$here/list.tsv"
date '+landed %Y-%m-%d %H:%M:%S' >> "$here/sha-after.txt"
