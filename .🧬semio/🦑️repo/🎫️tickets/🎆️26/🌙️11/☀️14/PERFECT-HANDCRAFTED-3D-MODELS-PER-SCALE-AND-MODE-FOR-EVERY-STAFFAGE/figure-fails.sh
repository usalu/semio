#!/bin/bash
# prints only failing measurements: figure-fails.sh <asset> [scale]
source "$(dirname "$0")/author-33-env.sh"
cd "$(dirname "$0")"
$BUN volumes-check.ts "$1" ${2:+--scale $2} 2>&1 | grep -v "^\[check" | grep -E "^FAIL" | while read -r l; do echo "$l" | tr ' ' '\n' | grep -E "^(FAIL|1:|!)|volumes-" | tr '\n' ' '; echo; done
