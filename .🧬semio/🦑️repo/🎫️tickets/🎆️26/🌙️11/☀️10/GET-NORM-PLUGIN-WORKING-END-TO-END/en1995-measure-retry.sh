#!/bin/bash
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995" || exit 1
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/en1995-unit-ceiling"
for attempt in 1 2 3 4 5 6; do
  repo capacity run --memory=2663 --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- build --offline --locked -p "$1" --target wasm32-wasip2 > "$T/$2.attempt$attempt.log" 2>&1
  rc=$?
  echo "attempt $attempt exit $rc" >> "$T/$2.status"
  if [ $rc -eq 0 ]; then echo "exit 0" >> "$T/$2.status"; exit 0; fi
  if grep -a -q 'exceeds its source boundary\|could not compile' "$T/$2.attempt$attempt.log"; then echo "exit $rc" >> "$T/$2.status"; exit $rc; fi
done
echo "exit 1" >> "$T/$2.status"
