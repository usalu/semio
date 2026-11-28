#!/bin/bash
cd "/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🪵️en1995" || exit 1
D="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END/🗑️generated/en1995-unit-ceiling"
for p in semio-s-artifact-norm-en1995-mutations-edit-rules semio-s-artifact-norm-en1995-schema semio-s-artifact-norm-en1995-mutations-bridges; do
  for a in 1 2 3 4; do
    repo capacity run --memory=2663 --cpu=1 -- repo cargo --offload=never --store=shared --configuration=prototype --parallelism=1 -- test --offline --locked -p "$p" > "$D/test-$p.log" 2>&1
    rc=$?
    echo "$p try$a rc=$rc" >> "$D/test.status"
    grep -a -q 'host warden cancelled' "$D/test-$p.log" || break
  done
done
echo "exit" >> "$D/test.status"
