#!/bin/sh
# usage: s17-descriptor.sh  (slice 17: regenerate the committed stdio-pdf descriptor pair with the real emitter after the Save-declined fault catalog grew: committed_descriptor run, then the settle step in write mode)
export PATH=$HOME/Library/Caches/semio/runtimes/bun/90987a3a16d7db556d886ac3d551e7b6d3edf0a1cf43acaed622e8676be1d12f:$PATH
here=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
root=$(CDPATH= cd -- "$here/../../../../../../.." && pwd)
sh "$here/committed-descriptor-with-retries.sh" s17-desc > "$here/🗑️generated/s17-desc.chain" 2>&1
dir=$(find "$here/🗑️generated/native-target" -type d -name stdio-pdf-described -print0 | xargs -0 ls -dt | head -1)
echo "described: $dir" >> "$here/🗑️generated/s17-desc.chain"
[ -n "$dir" ] && (cd "$root" && bun "$here/settle-pdf-descriptor.ts" "$dir") >> "$here/🗑️generated/s17-desc.chain" 2>&1
echo "exit $?" >> "$here/🗑️generated/s17-desc.chain"
