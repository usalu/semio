#!/bin/zsh
HERE=${0:A:h}; export R5_OVERLAY=${R5_OVERLAY:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R20/overlay}
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
for d in "$@"; do for f in $(cd /Users/Shared/semio/$L/$d && ls | /usr/bin/grep '\.ts$'); do out=$($HERE/run-overlay.sh "$L/$d" "$f" 2>&1); echo "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)$' | tr -s ' \n' ' ') :: $d/$f"; echo "$out" | /usr/bin/grep '^(fail)' | head -6; done; done
