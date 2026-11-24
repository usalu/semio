#!/bin/zsh
# O24 driver side: component jobs on fleet slots and local slots. usage: apply.sh <snapshot-root>
# DRIVER files: the running native-build driver loads them once; they take effect only after the driver (play chain) restarts.

# Stage 1 verifies every target (pre-image, or already the post-image = skipped, NEW = absent); any mismatch aborts before anything changes.
# Stage 2 copies each file to a temporary name and moves it in place, in list order (schemas and policy first, entry file last). Idempotent.
set -e
root=${1:?snapshot root}
here=$(cd "$(dirname "$0")" && pwd)
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }
todo=()
while IFS=$'\t' read -r name target pre post; do
 [[ $(sha "$here/$name") == "$post" ]] || { echo "BROKEN SOURCE: $name" >&2; exit 1; }
 file="$root/$target"
 if [[ -f $file ]]; then
  current=$(sha "$file")
  if [[ $current == "$post" ]]; then echo "skip (already patched): $name"; continue; fi
  [[ $pre != NEW && $current == "$pre" ]] || { echo "REFUSED: $target is $current, expected $pre" >&2; exit 1; }
 else
  [[ $pre == NEW ]] || { echo "REFUSED: $target is missing" >&2; exit 1; }
 fi
 todo+=("$name"$'\t'"$target")
done < "$here/list.tsv"
echo "stage 1 ok: ${#todo[@]} file(s) to write"
for row in $todo; do
 name=${row%%$'\t'*}; target=${row#*$'\t'}
 mkdir -p "$(dirname "$root/$target")"
 cp "$here/$name" "$root/$target.o24tmp" && mv "$root/$target.o24tmp" "$root/$target"
 echo "wrote: $target"
done
echo "stage 2 ok"
