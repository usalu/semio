#!/bin/sh
# [gram4-g] S0 stack applied to a mirror root ($1): walk-g walks, gram2-g row-table, gram3-g index-chunks, gram3-g churn, in this order (the order gram3-g measured). usage: s0-batch.sh <mirror root>
set -e
R="$1"; C='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
G="/Users/ueli/Documents/semio/$C"
for B in walk-g-tools/staged/walks gram2-g-tools/staged/row-table gram3-g-tools/staged/index-chunks gram3-g-tools/staged/churn; do
  BATCH_ROOT="$R" python3 -I "$G/walk-g-tools/staged/batch_generic.py" "$G/$B" apply
done
