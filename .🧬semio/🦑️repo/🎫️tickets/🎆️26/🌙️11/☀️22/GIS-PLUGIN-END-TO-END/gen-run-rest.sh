#!/bin/bash
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GIS-PLUGIN-END-TO-END"
cd /Users/ueli/Documents/semio
for t in framework-actor-rs:typegen framework-rs:generate ui-contract-rs:generate; do
  sleep 120
  until mkdir "$T/🗑️generated/compile.lock" 2>/dev/null; do if [ $(( $(date +%s) - $(stat -f %m "$T/🗑️generated/compile.lock") )) -gt 7200 ]; then rmdir "$T/🗑️generated/compile.lock"; fi; sleep 20; done; echo gen-lane > "$T/🗑️generated/compile.lock/owner"
  repo run @semio-tech/$t --lang=en > "$T/🗑️generated/gen-${t%%:*}.log" 2>&1; echo "$t rc=$?" >> "$T/🗑️generated/gen-results.txt"
  rm -rf "$T/🗑️generated/compile.lock"
done
echo done >> "$T/🗑️generated/gen-results.txt"
