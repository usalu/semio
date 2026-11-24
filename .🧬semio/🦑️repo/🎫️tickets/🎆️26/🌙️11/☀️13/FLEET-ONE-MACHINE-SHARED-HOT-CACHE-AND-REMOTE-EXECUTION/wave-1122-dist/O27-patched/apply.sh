#!/bin/zsh
# O27 patch set. usage: apply.sh <snapshot-root>
# job-slots (🟦️.ts + schema) is DRIVER code: loaded once by the native-build driver, takes effect only after the play chain restarts. units (capacity planner of `repo cargo` and the component build's -j count) is read per build/launcher process start.
# Stage 1 verifies every target (pre-image, or already the post-image = skipped); any mismatch aborts before anything changes. Stage 2 writes through a temporary name + rename, schemas first. Idempotent.
set -e
root=${1:?snapshot root}
here=$(cd "$(dirname "$0")" && pwd)
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }
todo=()
while IFS=$'\t' read -r name target pre post; do
 [[ $(sha "$here/$name") == "$post" ]] || { echo "BROKEN SOURCE: $name" >&2; exit 1; }
 file="$root/$target"
 [[ -f $file ]] || { echo "REFUSED: $target is missing" >&2; exit 1; }
 current=$(sha "$file")
 if [[ $current == "$post" ]]; then echo "skip (already patched): $name"; continue; fi
 [[ $current == "$pre" ]] || { echo "REFUSED: $target is $current, expected $pre" >&2; exit 1; }
 todo+=("$name"$'\t'"$target")
done < "$here/list.tsv"
echo "stage 1 ok: ${#todo[@]} file(s) to write"
for row in $todo; do
 name=${row%%$'\t'*}; target=${row#*$'\t'}
 cp "$here/$name" "$root/$target.o27tmp" && mv "$root/$target.o27tmp" "$root/$target"
 echo "wrote: $target"
done
echo "stage 2 ok"
