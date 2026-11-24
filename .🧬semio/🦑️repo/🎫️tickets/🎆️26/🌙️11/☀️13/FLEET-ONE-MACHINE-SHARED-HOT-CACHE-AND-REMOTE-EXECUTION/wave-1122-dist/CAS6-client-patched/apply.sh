#!/bin/zsh
# CAS7 requester side: the lease release per action (DELETE on action end) and the 507/5xx/unreachable retry of input uploads, onto the clone files as they are. usage: apply.sh <snapshot-root> [sealed]
# usage: apply.sh <snapshot-root> [sealed]   (with `sealed`, <snapshot-root> is a sealed wrapper directory .cache/b/r/m/u/sealed/<digest>: the leading framework folder is dropped and only files that exist there are patched: the wrapper closure, no tests, no driver)

# Stage 1 verifies every target (pre-image, or already the post-image = skipped, NEW = absent); any mismatch aborts before anything changes.
# Stage 2 copies each file to a temporary name and moves it in place, in list order (schemas and policy first, entry file last). Idempotent.
set -e
root=${1:?snapshot root}
mode=${2:-tree}
here=$(cd "$(dirname "$0")" && pwd)
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }
todo=()
while IFS=$'\t' read -r name target pre post; do
 [[ $(sha "$here/$name") == "$post" ]] || { echo "BROKEN SOURCE: $name" >&2; exit 1; }
 [[ $mode == sealed ]] && target=${target#🧰️framework/}
 file="$root/$target"
 [[ $mode == sealed && ! -f $file ]] && { echo "skip (not in a sealed wrapper): $name"; continue; }
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
 cp "$here/$name" "$root/$target.cas7tmp" && mv "$root/$target.cas7tmp" "$root/$target"
 echo "wrote: $target"
done
echo "stage 2 ok"
