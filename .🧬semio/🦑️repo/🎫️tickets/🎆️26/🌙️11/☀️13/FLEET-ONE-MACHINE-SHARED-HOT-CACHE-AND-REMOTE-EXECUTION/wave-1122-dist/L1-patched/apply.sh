#!/bin/zsh
# L1: lane pool keeps the lanes of a starting build. usage: apply.sh <snapshot-root>   (try on a copy of the files first; nothing is written unless every pre-image hash matches and every patch applies)
set -e
root=${1:?snapshot root}
here=$(cd "$(dirname "$0")" && pwd)
tmp=$(mktemp -d); trap 'rm -rf "$tmp"' EXIT
tail -n +2 "$here/list.tsv" | while IFS=$'\t' read -r order rel pre patchname read; do
 target="$root/$rel"
 [ -f "$target" ] || { echo "MISSING: $rel" >&2; exit 1; }
 [ "$(shasum -a 256 "$target" | cut -d' ' -f1)" = "$pre" ] || { echo "PRE-IMAGE MISMATCH: $rel" >&2; exit 1; }
 mkdir -p "$tmp/$(dirname "$rel")"; cp "$target" "$tmp/$rel"
 patch -s "$tmp/$rel" < "$here/patches/$patchname" || { echo "FAILED: $patchname on $rel" >&2; exit 1; }
done
echo "stage 1 ok: every pre-image matches and every patch applies"
tail -n +2 "$here/list.tsv" | while IFS=$'\t' read -r order rel pre patchname read; do cp "$tmp/$rel" "$root/$rel.l1tmp"; done
tail -n +2 "$here/list.tsv" | while IFS=$'\t' read -r order rel pre patchname read; do mv "$root/$rel.l1tmp" "$root/$rel"; done
(cd "$here/new-files" && find . -type f | while read -r file; do mkdir -p "$root/$(dirname "$file")"; cp "$file" "$root/$file"; done)
echo "stage 2 ok: patched files moved in place, flavour-need test directory added"
