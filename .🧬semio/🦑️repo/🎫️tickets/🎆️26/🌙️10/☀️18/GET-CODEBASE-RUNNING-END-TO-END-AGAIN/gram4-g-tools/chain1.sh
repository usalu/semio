#!/bin/sh
# [gram4-g] sequential: S0 baseline then S0 + gc-exp (collection checkpoint), root project only (LIMIT=1), rows dumped for the identity check.
T='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN'
S0='/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/walk-g-tools/staged/walks,/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram2-g-tools/staged/row-table,/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram3-g-tools/staged/index-chunks,/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/gram3-g-tools/staged/churn'
sh $T/gram4-g-tools/run.sh b0 LIMIT=1 PROBES=0 ROWS_DUMP=/private/tmp/gram4-g/rows-b0 DERIVED_DUMP=/private/tmp/gram4-g/derived-b0 BATCHES=$S0 > $T/🗑️generated/gram4-g/b0.out 2>&1
sh $T/gram4-g-tools/run.sh g0 LIMIT=1 PROBES=0 ROWS_DUMP=/private/tmp/gram4-g/rows-g0 BATCHES=$S0,$T/gram3-g-tools/staged/gc-exp > $T/🗑️generated/gram4-g/g0.out 2>&1
