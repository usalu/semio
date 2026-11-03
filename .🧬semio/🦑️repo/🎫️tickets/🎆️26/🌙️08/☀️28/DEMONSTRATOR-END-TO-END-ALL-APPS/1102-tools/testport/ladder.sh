#!/bin/bash
# usage: ladder.sh [first-batch-index]  — sequential slot-wrapped test batches
D="$(cd "$(dirname "$0")" && pwd)"; S=${1:-0}
for b in "$D"/batch-?; do n=$(basename $b); [ "${n#batch-}" -lt "$S" ] && continue; "$D/t.sh" r2-$n $n -- --skip bounded_history_read_cursors_obey_the_neutral_law; done
