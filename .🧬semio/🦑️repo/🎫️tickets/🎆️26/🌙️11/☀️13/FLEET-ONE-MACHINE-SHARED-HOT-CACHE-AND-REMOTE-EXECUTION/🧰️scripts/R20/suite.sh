#!/bin/zsh
HERE=${0:A:h}; export R5_OVERLAY=${R5_OVERLAY:-/private/tmp/claude-501/-Users-ueli-Documents-semio/9f6d3d77-cfd2-486b-942c-2101ee51a1fb/scratchpad/R20/overlay}
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
run(){ out=$(R5_RUN_IN=$R5_OVERLAY $HERE/run-overlay.sh "$1" "$2" 2>&1); echo "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)$' | tr -s ' \n' ' ') :: ${1#$L/}"; echo "$out" | /usr/bin/grep '^(fail)' | head -6; }
for d in "$@"; do for f in $(cd $R5_OVERLAY/$L/$d && ls | /usr/bin/grep -v '^node_modules$' | /usr/bin/grep '\.ts$'); do run "$L/$d" "$f"; done; done
