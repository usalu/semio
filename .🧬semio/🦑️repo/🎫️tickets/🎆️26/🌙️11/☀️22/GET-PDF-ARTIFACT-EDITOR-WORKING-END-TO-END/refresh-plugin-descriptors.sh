#!/bin/sh
# usage: refresh-plugin-descriptors.sh <tag> [crate…]  (each component crate's native `descriptor_is_fresh` through the memory gate, emitting the assembled pair to 🗑️generated/described/<plugin-id>/; no crates given = every committed component except stdio-pdf)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
tag=${1:?tag}; shift
if [ "$#" -eq 0 ]; then set -- $(cut -d'|' -f1 "$here/plugin-descriptors-components.txt" | grep -v '^semio-hub-stdio-pdf-component$'); fi
mkdir -p "$here/🗑️generated/described" "$here/🗑️generated/$tag"
guard() {
  while :; do
    free=$(df -k /System/Volumes/Data | awk 'NR==2 {print int($4/1048576)}')
    [ "$free" -ge 25 ] && return 0
    find "$here"/🗑️generated/native-target/lane-*/b/debug/build/*/* -mindepth 0 -maxdepth 0 -type d -mmin +60 -exec rm -rf {} + 2>/dev/null
    free=$(df -k /System/Volumes/Data | awk 'NR==2 {print int($4/1048576)}')
    [ "$free" -ge 25 ] && return 0
    echo "disk guard: ${free} GB free, waiting" >&2
    sleep 60
  done
}
for crate in "$@"; do
  guard
  SEMIO_DESCRIBE_EMIT_DIR="$here/🗑️generated/described" NATIVE_CARGO_DIR="$root" sh "$here/native-cargo.sh" test --manifest-path "$root/$(grep "^$crate|" "$here/plugin-descriptors-components.txt" | cut -d"|" -f2 | sed "s|^\./||")/📦️packages/🦀️rust/Cargo.toml" --lib descriptor_is_fresh > "$here/🗑️generated/$tag/$crate.log" 2>&1
  echo "$crate exit $?" >> "$here/🗑️generated/$tag.summary"
done
echo done >> "$here/🗑️generated/$tag.summary"
