#!/bin/sh
# oracle-target.sh <tag> <allow-list> <cwd-rel> <rounds> [--env K=V]... -- <command...> : one target's action command under the read oracle (the live module CLI, Seatbelt on macOS) with exactly the declared+inferred files of the target as the allow list; report in the generated folder.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/undeclared-g"; R="🧰️framework/🛍️products/🦑️repo"
TAG="$1"; LIST="$2"; CWD="$3"; ROUNDS="$4"; shift 4
EXTRA=""
nice -n 10 bun "$R/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts" --language en --root "$PWD" --allow-list "$LIST" --allow-name package.json --allow-tree "$C/undeclared-g-tools" --allow-tree "$G" --allow-tree "$R/🔨️modules/🚀️launcher" --cwd "$CWD" --rounds "$ROUNDS" --timeout 1500 --backend auto --compare exit --out "$G/reports/$TAG.json" "$@" > "$G/reports/$TAG.log" 2>&1
echo $? > "$G/reports/$TAG.done"
