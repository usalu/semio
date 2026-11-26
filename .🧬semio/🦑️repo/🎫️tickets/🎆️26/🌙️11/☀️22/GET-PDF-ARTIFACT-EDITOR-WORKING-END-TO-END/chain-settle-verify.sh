#!/bin/sh
# usage: chain-settle-verify.sh <tag> <emit crate…> -- <verify crate…>  (emits the listed crates, settles every emitted pair, then runs the strict freshness check of the verify list, one after the other through the memory gate)
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
tag=${1:?tag}; shift
emit=""; while [ "$#" -gt 0 ] && [ "$1" != "--" ]; do emit="$emit $1"; shift; done; [ "$1" = "--" ] && shift
rm -f "$here/🗑️generated/em-$tag.summary" "$here/🗑️generated/vf-$tag.summary"
[ -n "$emit" ] && sh "$here/refresh-plugin-descriptors.sh" "em-$tag" $emit
bun=$(ls "$root"/.cache/b/t/m/bun/*/bun | head -1)
(cd "$root" && "$bun" "$here/settle-plugin-descriptors.ts" "$here" write > "$here/🗑️generated/settle-$tag.log" 2>&1)
[ "$#" -gt 0 ] && sh "$here/verify-plugin-descriptors.sh" "vf-$tag" "$@"
