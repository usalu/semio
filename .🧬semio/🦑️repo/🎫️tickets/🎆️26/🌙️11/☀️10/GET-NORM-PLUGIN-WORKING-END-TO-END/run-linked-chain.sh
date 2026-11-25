#!/bin/bash
# 🔗 Linked default rows: waits for the empty-insert chain, then builds the en1997 mutations tests through the lane and runs the linking tests.
T="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️10/GET-NORM-PLUGIN-WORKING-END-TO-END"
G="$T/🗑️generated"; M="/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/🌍️en1997/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust"
while [ ! -f "$G/ei.done" ]; do sleep 20; done
cd "$M" && python3 "$T/lane.py" repo cargo --store=shared --configuration=prototype -- test -p semio-s-artifact-norm-en1997-mutations --offline -- insert_without_a_row an_insert_without a_field_without > "$G/linked-en1997.log" 2>&1
echo "exit $?" >> "$G/linked-en1997.log"; echo done > "$G/linked.done"
