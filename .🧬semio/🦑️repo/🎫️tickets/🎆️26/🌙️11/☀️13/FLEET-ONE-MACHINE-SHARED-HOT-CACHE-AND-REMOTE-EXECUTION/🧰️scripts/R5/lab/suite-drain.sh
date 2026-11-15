#!/bin/zsh
HERE=${0:A:h}; OV=$1
L="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"; RX="$L/🏗️build/🌐️remote-execution"
run(){ out=$(R5_OVERLAY=$OV $HERE/run-overlay.sh "$1" "$2" 2>&1); echo "$(echo "$out" | /usr/bin/grep -E '^ *[0-9]+ (pass|fail)$' | tr -s ' \n' ' ') :: ${1#$RX/}/$2"; echo "$out" | /usr/bin/grep '^(fail)' | head -4; }
for d in $(cd /Users/Shared/semio/$RX && find . -path '*/🧪️tests/*' -name '🟦️.ts' -not -path '*/node_modules/*' | sed 's|/🟦️.ts$||; s|^\./||' | sort); do run "$RX/$d" '🟦️.ts'; done
