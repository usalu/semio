#!/bin/bash
# usage: owned3-sum.sh <label>  -> per-test verdicts and first assertion line of each panic
F="$(dirname "$0")/../🗑️generated/1105b/$1.txt"
grep -aE "^(test [A-Za-z_:0-9]+ \.\.\. (ok|FAILED)|test result|error(\[E[0-9]+\])?: (could not|test failed)|\[slot\] HANG)" "$F" | cut -c1-230
grep -a -A2 "^thread '.*panicked at" "$F" | grep -av "^--$" | cut -c1-330
