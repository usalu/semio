#!/usr/bin/env bash
T="/c/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️22/GET-FEM-PLUGIN-WORKING-END-TO-END"
L="$T/🗑️generated/pub-describe7.log"
cd /c/semio || exit 2
for attempt in $(seq 1 60); do
  echo "=== attempt $attempt $(date +%H:%M)" >> "$L"
  repo run workspace:components -- describe fem --profile prototype > "$L.run" 2>&1
  rc=$?
  cat "$L.run" >> "$L"; echo "rc=$rc" >> "$L"
  if [ $rc -eq 0 ]; then echo "DONE ok" >> "$L"; exit 0; fi
  if ! grep -qE "not found in|Cannot find module|does not provide an export|Stale source facts|ENOENT|graph service could not answer|stamps moved" "$L.run"; then echo "DONE fail" >> "$L"; exit 1; fi
  sleep 300
done
echo "DONE gave-up" >> "$L"
