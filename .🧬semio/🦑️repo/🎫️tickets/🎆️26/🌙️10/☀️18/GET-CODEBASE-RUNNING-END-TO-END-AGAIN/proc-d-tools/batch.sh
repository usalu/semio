#!/bin/sh
# usage: batch.sh <tag> <target>... : runs official targets sequentially through the semio tool
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; shift
for t in "$@"; do sh "$C/proc-d-tools/one.sh" "$tag-$t" semio @semio-tech/framework-process "$t"; done
echo done > "$C/🗑️generated/proc-d/$tag.batch-done"
