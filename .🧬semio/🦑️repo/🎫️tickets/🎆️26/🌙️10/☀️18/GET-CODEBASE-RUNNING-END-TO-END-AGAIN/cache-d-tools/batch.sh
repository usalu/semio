#!/bin/sh
# usage: batch.sh <tag> <tool> <suite>... : waits for o1-route-table, then runs official `repo:test-<suite>` targets sequentially
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
tag="$1"; tool="$2"; shift 2
while [ ! -f "$C/🗑️generated/cache-d/o1-route-table.rc" ]; do sleep 20; done
for t in "$@"; do sh "$C/cache-d-tools/one.sh" "$tag-$t" "$tool" "test-$t"; done
echo done > "$C/🗑️generated/cache-d/$tag.batch-done"
