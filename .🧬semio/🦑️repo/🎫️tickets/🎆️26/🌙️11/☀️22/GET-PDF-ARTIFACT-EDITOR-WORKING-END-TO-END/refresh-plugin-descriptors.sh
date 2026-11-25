#!/bin/sh
# usage: refresh-plugin-descriptors.sh <tag> [crate…]  (each component crate's native `descriptor_is_fresh` through the memory gate, emitting the assembled pair to 🗑️generated/described/<plugin-id>/; no crates given = every committed component except stdio-pdf)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
tag=${1:?tag}; shift
if [ "$#" -eq 0 ]; then set -- $(cut -d'|' -f1 "$here/plugin-descriptors-components.txt" | grep -v '^semio-hub-stdio-pdf-component$'); fi
mkdir -p "$here/🗑️generated/described" "$here/🗑️generated/$tag"
for crate in "$@"; do
  SEMIO_DESCRIBE_EMIT_DIR="$here/🗑️generated/described" NATIVE_CARGO_DIR="$root" sh "$here/native-cargo.sh" test -p "$crate" --lib descriptor_is_fresh > "$here/🗑️generated/$tag/$crate.log" 2>&1
  echo "$crate exit $?" >> "$here/🗑️generated/$tag.summary"
done
echo done >> "$here/🗑️generated/$tag.summary"
