#!/bin/sh
# usage: verify-plugin-descriptors.sh <tag> <crate…>  (the strict native `descriptor_is_fresh` of each component crate through the memory gate, no emission; one `crate exit N` row per crate in 🗑️generated/<tag>.summary)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
tag=${1:?tag}; shift
mkdir -p "$here/🗑️generated/$tag"
for crate in "$@"; do
  dir=$(grep "^$crate|" "$here/plugin-descriptors-components.txt" | cut -d'|' -f2 | sed 's|^\./||')
  NATIVE_CARGO_DIR="$root" sh "$here/native-cargo.sh" test --manifest-path "$root/$dir/📦️packages/🦀️rust/Cargo.toml" --lib descriptor_is_fresh > "$here/🗑️generated/$tag/$crate.log" 2>&1
  echo "$crate exit $?" >> "$here/🗑️generated/$tag.summary"
done
echo done >> "$here/🗑️generated/$tag.summary"
