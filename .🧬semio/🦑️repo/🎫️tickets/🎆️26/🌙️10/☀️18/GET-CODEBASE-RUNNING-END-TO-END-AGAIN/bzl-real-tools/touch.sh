#!/bin/bash
# usage: touch.sh add|revert <file>   (appends a harmless trailing comment line to a checkout file, keeps the original bytes for an exact revert)
cd /Users/ueli/Documents/semio || exit 2
mode=$1; file=$2; keep=/private/tmp/bzlreal-keep/$(printf '%s' "$file" | shasum -a 256 | cut -c1-16)
mkdir -p /private/tmp/bzlreal-keep
case "$mode" in
 add) [ -e "$keep" ] && { echo "already touched: $file"; exit 3; }; cp -p "$file" "$keep" && printf '\n// [DEBUG] bzl-real touch probe\n' >> "$file" && echo "touched $file";;
 revert) [ -e "$keep" ] || { echo "not touched: $file"; exit 3; }; cat "$keep" > "$file" && rm "$keep" && echo "reverted $file";;
esac
