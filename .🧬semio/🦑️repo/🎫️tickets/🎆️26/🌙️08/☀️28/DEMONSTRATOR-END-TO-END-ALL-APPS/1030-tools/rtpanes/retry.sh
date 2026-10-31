#!/bin/bash
# usage: retry.sh <tag> <pane...>  : like seq.sh, but waits for >=10GiB on the rust cache image before each pane and retries (max 4) while the log shows ENOSPC
D=$(cd "$(dirname "$0")" && pwd); C=$(cd "$D/../.." && pwd)
IMG="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/⚡️cache/rust/fixed/m"
tag=$1; shift
for pane in "$@"; do
  for try in $(seq 1 16); do
    while [ "$(df -k "$IMG" | tail -1 | awk '{print $4}')" -lt 10000000 ]; do sleep 20; done
    "$D/seq.sh" "$tag-$pane" "$pane"
    if grep -aq 'failed to write\|No space left\|Rust crate Source extent' "$C/🗑️generated/1030/rtpanes-$pane.txt"; then cp "$C/🗑️generated/1030/rtpanes-$pane.txt" "$C/🗑️generated/1030/rtpanes-$pane.enospc$try.txt"; sleep 300; continue; fi
    break
  done
done
echo done > "$C/🗑️generated/1030/rtpanes-retry-$tag.rc"
