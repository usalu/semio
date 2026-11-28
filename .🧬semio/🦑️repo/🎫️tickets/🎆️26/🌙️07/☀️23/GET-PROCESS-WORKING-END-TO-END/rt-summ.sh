#!/bin/bash
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️07/☀️23/GET-PROCESS-WORKING-END-TO-END/🗑️generated"
for f in $G/rust-tests-p-*.txt; do echo "== $(basename $f) $(cat ${f%.txt}.done 2>/dev/null)"; grep -E '^test result:|^test .* FAILED$|^error(\[E[0-9]+\])?: (test failed|could not compile)|Running .*' $f | sed -E 's/\(\/Users.*//' | cut -c1-160; done
