#!/bin/zsh
# [DEBUG] retries the guarded law run while another agent's in-flight edit keeps the kernel crate from compiling
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️20/COMPLETE-THE-WELL-KNOWN-XLSX-ARTIFACT-AND-EDITOR"; L=$T/🗑️generated/claude-cleanup-pump/test.log
for attempt in 1 2 3 4 5 6 7 8; do
  rm -f $L; MIN_DISK_GIB=20 $T/claude-resource-guard.sh $T/claude-cleanup-pump-test.sh > $T/🗑️generated/claude-cleanup-pump/guard.log 2>&1
  if ! grep -q 'could not compile' $L; then echo "attempt $attempt final"; exit 0; fi
  echo "attempt $attempt blocked by kernel compile"; sleep 300
done
